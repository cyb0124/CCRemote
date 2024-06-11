use super::{EachInv, EachInvConfig, IntoProcess, MultiInvExtractFilter, MultiInvSlottedInput, Process};
use crate::{
    access::{BusAccess, InvTankAccess},
    action::{ActionFuture, Call},
    factory::{read_tanks, tanks_to_fluid_map, Factory},
    inventory::{list_inventory, Inventory},
    item::DetailStack,
    process::extract_output,
    recipe::{resolve_inputs, Demand, Outputs, Recipe},
    server::Server,
    util::{alive, join_outputs, join_tasks, spawn},
};
use abort_on_drop::ChildTask;
use flexstr::LocalStr;
use fnv::{FnvHashMap, FnvHashSet};
use std::{
    cell::RefCell,
    collections::{hash_map::Entry, BTreeMap},
    rc::{Rc, Weak},
};

#[derive(Clone)]
pub struct FluidSlottedInput {
    fluid: LocalStr,
    size: i64,
    tanks: Vec<(usize, i64)>,
    allow_backup: bool,
    extra_backup: i64,
}

impl FluidSlottedInput {
    pub fn new(fluid: LocalStr, tanks: Vec<(usize, i64)>) -> Self {
        let size = tanks.iter().map(|(_, size)| size).sum();
        Self { fluid, size, tanks, allow_backup: false, extra_backup: 0 }
    }

    pub fn allow_backup(mut self) -> Self {
        self.allow_backup = true;
        self
    }

    pub fn extra_backup(mut self, size: i64) -> Self {
        self.extra_backup += size;
        self
    }
}

#[derive(Clone)]
pub struct FluidSlottedRecipe {
    pub outputs: Rc<dyn Outputs>,
    pub inputs: Vec<MultiInvSlottedInput>,
    pub fluids: Vec<FluidSlottedInput>,
    pub max_sets: i32,
}
impl_recipe!(FluidSlottedRecipe, MultiInvSlottedInput);

pub type FluidExtractFilter =
    Box<dyn Fn(&Factory, usize, BTreeMap<usize, (LocalStr, i64)>) -> FnvHashMap<LocalStr, i64>>;
pub fn fluid_extract_all() -> Option<FluidExtractFilter> { Some(Box::new(|_, _, tanks| tanks_to_fluid_map(&tanks))) }
pub fn fluid_extract_slots(slots: impl Fn(usize, usize) -> bool + 'static) -> Option<FluidExtractFilter> {
    Some(Box::new(move |_, i, tanks| {
        let mut result = FnvHashMap::default();
        for (slot, (fluid, qty)) in tanks {
            if slots(i, slot) {
                *result.entry(fluid).or_default() += qty
            }
        }
        result
    }))
}

pub struct FluidSlottedConfig {
    pub name: LocalStr,
    pub input_slots: Vec<Vec<usize>>,
    pub input_tanks: Vec<Vec<usize>>,
    pub accesses: Vec<InvTankAccess>,
    pub to_extract: Option<MultiInvExtractFilter>,
    pub fluid_extract: Option<FluidExtractFilter>,
    pub recipes: Vec<FluidSlottedRecipe>,
    pub strict_priority: bool,
}

pub struct FluidSlottedProcess {
    weak: Weak<RefCell<FluidSlottedProcess>>,
    name: LocalStr,
    accesses: Vec<InvTankAccess>,
    factory: Weak<RefCell<Factory>>,
    server: Rc<RefCell<Server>>,
    to_extract: Option<MultiInvExtractFilter>,
    fluid_extract: Option<FluidExtractFilter>,
    recipes: Vec<FluidSlottedRecipe>,
    strict_priority: bool,
    invs: Vec<Rc<RefCell<EachInv>>>,
    input_tanks: Vec<Vec<usize>>,
}

impl IntoProcess for FluidSlottedConfig {
    type Output = FluidSlottedProcess;
    fn into_process(self, factory: &Factory) -> Rc<RefCell<Self::Output>> {
        Rc::new_cyclic(|weak| {
            let accesses = self.accesses;
            let invs = self
                .input_slots
                .into_iter()
                .enumerate()
                .map(|(i, input_slots)| {
                    Rc::new_cyclic(|weak| {
                        RefCell::new(EachInv {
                            weak: weak.clone(),
                            config: EachInvConfig {
                                accesses: accesses
                                    .iter()
                                    .map(|access| BusAccess {
                                        client: access.client.clone(),
                                        inv_addr: access.inv_addrs[i].clone(),
                                        bus_addr: access.bus_addr.clone(),
                                    })
                                    .collect(),
                                input_slots,
                            },
                            detail_cache: factory.get_detail_cache().clone(),
                            server: factory.get_server().clone(),
                            size: None,
                        })
                    })
                })
                .collect();
            RefCell::new(Self::Output {
                weak: weak.clone(),
                name: self.name,
                accesses,
                factory: factory.get_weak().clone(),
                server: factory.get_server().clone(),
                to_extract: self.to_extract,
                fluid_extract: self.fluid_extract,
                recipes: self.recipes,
                strict_priority: self.strict_priority,
                invs,
                input_tanks: self.input_tanks,
            })
        })
    }
}

struct InputInfo {
    n_available: i64,
    n_needed: i64,
}

fn compute_fluid_demands(factory: &Factory, recipes: &[FluidSlottedRecipe]) -> Vec<Demand> {
    let mut result = Vec::new();
    for (i_recipe, recipe) in recipes.iter().enumerate() {
        let Some(mut priority) = recipe.get_outputs().get_priority(factory) else { continue };
        let Some(mut inputs) = resolve_inputs(factory, recipe) else { continue };
        let mut infos = FnvHashMap::<LocalStr, InputInfo>::default();
        let mut bus_bound = i64::MAX;
        for input in &recipe.fluids {
            match infos.entry(input.fluid.clone()) {
                Entry::Occupied(input_info) => input_info.into_mut().n_needed += input.size,
                Entry::Vacant(input_info) => {
                    input_info.insert(InputInfo {
                        // Note: backup params are considered for only the first input of the same fluid.
                        n_available: factory.get_fluid_availability(
                            &*input.fluid,
                            input.allow_backup,
                            input.extra_backup,
                        ),
                        n_needed: input.size,
                    });
                }
            }
            bus_bound = bus_bound.min(factory.config.fluid_bus_capacity / input.size)
        }
        let mut availability_bound = i64::MAX;
        for (_, input_info) in infos {
            availability_bound = availability_bound.min(input_info.n_available / input_info.n_needed)
        }
        inputs.n_sets = (inputs.n_sets as i64).min(bus_bound).min(availability_bound) as _;
        if inputs.n_sets > 0 {
            inputs.priority = (inputs.priority as i64).min(availability_bound) as _;
            priority *= inputs.priority as f64;
            result.push(Demand { i_recipe, inputs, priority })
        }
    }
    result.sort_by(|x: &Demand, y: &Demand| x.priority.partial_cmp(&y.priority).unwrap().reverse());
    result
}

impl Process for FluidSlottedProcess {
    fn run(&self, factory: &Factory) -> ChildTask<Result<(), LocalStr>> {
        if self.to_extract.is_none()
            && self.fluid_extract.is_none()
            && compute_fluid_demands(factory, &self.recipes).is_empty()
        {
            return spawn(async { Ok(()) });
        }
        let stacks = Vec::from_iter(self.invs.iter().map(|inv| spawn(list_inventory(&*inv.borrow()))));
        let tanks = Vec::from_iter((0..self.input_tanks.len()).map(|i| {
            spawn(read_tanks(&*factory.get_server().borrow(), &self.accesses, |access| access.tank_addrs[i].clone()))
        }));
        let weak = self.weak.clone();
        spawn(async move {
            let stacks = join_outputs(stacks).await?;
            let tanks = join_outputs(tanks).await?;
            let mut tasks = Vec::new();
            {
                alive!(weak, this);
                upgrade_mut!(this.factory, factory);
                let mut existing_inputs = FnvHashMap::<(usize, usize), Option<DetailStack>>::default();
                for (i, inv) in this.invs.iter().enumerate() {
                    for slot in &inv.borrow().config.input_slots {
                        existing_inputs.insert((i, *slot), None);
                    }
                }
                for (i, stacks) in stacks.into_iter().enumerate() {
                    for (slot, stack) in stacks.into_iter().enumerate() {
                        if let Some(stack) = stack {
                            if let Some(existing_input) = existing_inputs.get_mut(&(i, slot)) {
                                *existing_input = Some(stack)
                            } else if let Some(to_extract) = &this.to_extract {
                                if to_extract(factory, i, slot, &stack) {
                                    tasks.push(extract_output(
                                        &*this.invs[i].borrow(),
                                        factory,
                                        slot,
                                        stack.detail.max_size,
                                    ))
                                }
                            }
                        }
                    }
                }
                let existing_fluids = Vec::from_iter(tanks.into_iter().enumerate().map(|(i, tanks)| {
                    let mut fluid_map = FnvHashMap::<LocalStr, i64>::default();
                    for slot in &this.input_tanks[i] {
                        let Some((fluid, qty)) = tanks.get(slot) else { continue };
                        *fluid_map.entry(fluid.clone()).or_default() += qty
                    }
                    if let Some(fluid_extract) = &this.fluid_extract {
                        this.extract_fluids(factory, i, fluid_extract(factory, i, tanks), &mut tasks)
                    }
                    fluid_map
                }));
                let mut demands = compute_fluid_demands(factory, &this.recipes);
                if this.strict_priority {
                    demands.truncate(1)
                }
                'recipe: for mut demand in demands.into_iter() {
                    let recipe = &this.recipes[demand.i_recipe];
                    let mut used_slots = FnvHashSet::<(usize, usize)>::default();
                    for (i_input, input) in recipe.inputs.iter().enumerate() {
                        for &(inv, inv_slot, mult) in &input.slots {
                            let slot = (inv, inv_slot);
                            let existing_input = existing_inputs.get(&slot).unwrap();
                            let existing_size = if let Some(existing_input) = existing_input {
                                if existing_input.item != demand.inputs.items[i_input].0 {
                                    continue 'recipe;
                                }
                                existing_input.size
                            } else {
                                0
                            };
                            demand.inputs.n_sets = demand.inputs.n_sets.min(
                                ((recipe.max_sets * mult).min(demand.inputs.items[i_input].1.max_size) - existing_size)
                                    / mult,
                            );
                            if demand.inputs.n_sets <= 0 {
                                continue 'recipe;
                            }
                            used_slots.insert(slot);
                        }
                    }
                    for (slot, existing_input) in &existing_inputs {
                        if existing_input.is_some() && !used_slots.contains(slot) {
                            continue 'recipe;
                        }
                    }
                    let mut mismatched_fluids = (existing_fluids.iter().enumerate())
                        .flat_map(|(i, fluid_map)| fluid_map.keys().map(move |fluid| (i, fluid.clone())))
                        .collect::<FnvHashSet<_>>();
                    for input in &recipe.fluids {
                        for &(i, mult) in &input.tanks {
                            mismatched_fluids.remove(&(i, input.fluid.clone()));
                            let fluid_map = &existing_fluids[i];
                            let existing_size = fluid_map.get(&input.fluid).copied().unwrap_or_default();
                            demand.inputs.n_sets = (demand.inputs.n_sets)
                                .min(((recipe.max_sets as i64 * mult - existing_size) / mult).clamp(0, i32::MAX as _)
                                    as _);
                            if demand.inputs.n_sets <= 0 {
                                continue 'recipe;
                            }
                        }
                    }
                    if !mismatched_fluids.is_empty() {
                        continue;
                    }
                    tasks.push(this.execute_recipe(factory, demand));
                    break;
                }
            }
            join_tasks(tasks).await
        })
    }
}

impl FluidSlottedProcess {
    fn extract_fluids(
        &self,
        factory: &mut Factory,
        i: usize,
        fluids: FnvHashMap<LocalStr, i64>,
        tasks: &mut Vec<ChildTask<Result<(), LocalStr>>>,
    ) {
        for (fluid, mut remain) in fluids {
            while remain > 0 {
                let bus = factory.fluid_bus_allocate();
                let weak = self.weak.clone();
                let fluid = fluid.clone();
                let qty = remain.min(factory.config.fluid_bus_capacity);
                remain -= qty;
                tasks.push(spawn(async move {
                    let bus = bus.await?;
                    let task;
                    {
                        alive!(weak, this);
                        upgrade!(this.factory, factory);
                        let server = factory.get_server().borrow();
                        let access = server.load_balance(&this.accesses);
                        task = ActionFuture::from(Call {
                            addr: access.fluid_bus_addrs[bus].clone(),
                            args: vec![
                                "pullFluid".into(),
                                access.tank_addrs[i].clone().into(),
                                qty.into(),
                                fluid.into(),
                            ],
                        });
                        server.enqueue_request_group(&access.client, vec![task.clone().into()])
                    }
                    let result = task.await.map(|_| ());
                    alive(&weak)?.borrow().factory.upgrade().unwrap().borrow_mut().fluid_bus_deposit([bus]);
                    result
                }))
            }
        }
    }

    fn execute_recipe(&self, factory: &mut Factory, demand: Demand) -> ChildTask<Result<(), LocalStr>> {
        let mut bus_slots = Vec::new();
        let slots_to_free = Rc::new(RefCell::new(Vec::new()));
        let mut fluid_buses = Vec::new();
        let fluid_buses_to_free = Rc::new(RefCell::new(Vec::new()));
        let recipe = &self.recipes[demand.i_recipe];
        for (i_input, input) in recipe.inputs.iter().enumerate() {
            let reservation =
                factory.reserve_item(&self.name, &demand.inputs.items[i_input].0, demand.inputs.n_sets * input.size);
            let bus_slot = factory.bus_allocate();
            let slots_to_free = slots_to_free.clone();
            bus_slots.push(spawn(async move {
                let bus_slot = bus_slot.await?;
                slots_to_free.borrow_mut().push(bus_slot);
                let extraction = reservation.extract(bus_slot);
                extraction.await.map(|_| bus_slot)
            }))
        }
        for input in &recipe.fluids {
            let reservation =
                factory.reserve_fluid(&self.name, &*input.fluid, input.size * demand.inputs.n_sets as i64);
            let fluid_bus = factory.fluid_bus_allocate();
            let fluid_buses_to_free = fluid_buses_to_free.clone();
            fluid_buses.push(spawn(async move {
                let fluid_bus = fluid_bus.await?;
                fluid_buses_to_free.borrow_mut().push(fluid_bus);
                let extraction = reservation.extract(fluid_bus);
                extraction.await.map(|_| fluid_bus)
            }))
        }
        let weak = self.weak.clone();
        let factory = factory.get_weak().clone();
        spawn(async move {
            let bus_slots = join_outputs(bus_slots).await;
            let fluid_buses = join_outputs(fluid_buses).await;
            let slots_to_free = Rc::into_inner(slots_to_free).unwrap().into_inner();
            let fluid_buses_to_free = Rc::into_inner(fluid_buses_to_free).unwrap().into_inner();
            let task = async {
                let bus_slots = bus_slots?;
                let fluid_buses = fluid_buses?;
                let mut tasks = Vec::new();
                {
                    alive!(weak, this);
                    let server = this.server.borrow();
                    let access = server.load_balance(&this.accesses);
                    let mut group = Vec::new();
                    let recipe = &this.recipes[demand.i_recipe];
                    for (input, fluid_bus) in recipe.fluids.iter().zip(fluid_buses) {
                        for &(i, mult) in &input.tanks {
                            let action = ActionFuture::from(Call {
                                addr: access.fluid_bus_addrs[fluid_bus].clone(),
                                args: vec![
                                    "pushFluid".into(),
                                    access.tank_addrs[i].clone().into(),
                                    (demand.inputs.n_sets as i64 * mult).into(),
                                    input.fluid.clone().into(),
                                ],
                            });
                            group.push(action.clone().into());
                            tasks.push(spawn(async move { action.await.map(|_| ()) }));
                        }
                    }
                    for (input, bus_slot) in recipe.inputs.iter().zip(bus_slots) {
                        for (inv, inv_slot, mult) in &input.slots {
                            let action = ActionFuture::from(Call {
                                addr: access.bus_addr.clone(),
                                args: vec![
                                    "pushItems".into(),
                                    access.inv_addrs[*inv].clone().into(),
                                    (bus_slot + 1).into(),
                                    (demand.inputs.n_sets * mult).into(),
                                    (inv_slot + 1).into(),
                                ],
                            });
                            group.push(action.clone().into());
                            tasks.push(spawn(async move { action.await.map(|_| ()) }));
                        }
                    }
                    server.enqueue_request_group(&access.client, group)
                }
                join_tasks(tasks).await?;
                alive_mut!(factory, factory);
                for &slot_to_free in &slots_to_free {
                    factory.bus_free(slot_to_free)
                }
                for &fluid_bus_to_free in &fluid_buses_to_free {
                    factory.fluid_bus_free(fluid_bus_to_free)
                }
                Ok(())
            };
            let result = task.await;
            if result.is_err() {
                alive_mut!(factory, factory);
                factory.bus_deposit(slots_to_free);
                factory.fluid_bus_deposit(fluid_buses_to_free)
            }
            result
        })
    }
}
