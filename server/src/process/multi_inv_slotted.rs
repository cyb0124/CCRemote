use super::super::access::{BusAccess, MultiInvAccess};
use super::super::action::{ActionFuture, Call};
use super::super::detail_cache::DetailCache;
use super::super::factory::Factory;
use super::super::inventory::{list_inventory, Inventory};
use super::super::item::{DetailStack, Filter};
use super::super::process::{IntoProcess, Process};
use super::super::recipe::{compute_demands, Demand, Input, Output, Recipe};
use super::super::server::Server;
use super::super::util::{alive, join_outputs, join_tasks, spawn, AbortOnDrop};
use super::extract_output;
use fnv::{FnvHashMap, FnvHashSet};
use std::cell::RefCell;
use std::cmp::min;
use std::rc::{Rc, Weak};

pub struct MultiInvSlottedInput {
    item: Filter,
    size: i32,
    slots: Vec<(usize, usize)>,
    allow_backup: bool,
    extra_backup: i32,
}

impl MultiInvSlottedInput {
    pub fn new(item: Filter, size: i32, slots: Vec<(usize, usize)>) -> Self {
        MultiInvSlottedInput { item, size, slots, allow_backup: false, extra_backup: 0 }
    }
}

impl_input!(MultiInvSlottedInput);

pub struct MultiInvSlottedRecipe {
    pub outputs: Vec<Output>,
    pub inputs: Vec<MultiInvSlottedInput>,
    pub max_per_slot: i32,
}

impl_recipe!(MultiInvSlottedRecipe, MultiInvSlottedInput);

pub type MultiInvExtractFilter = Box<dyn Fn(usize, usize, &DetailStack) -> bool>;
pub fn multi_inv_extract_all() -> Option<MultiInvExtractFilter> { Some(Box::new(|_, _, _| true)) }

pub struct MultiInvSlottedConfig {
    pub name: &'static str,
    pub input_slots: Vec<Vec<usize>>,
    pub accesses: Vec<MultiInvAccess>,
    pub to_extract: Option<MultiInvExtractFilter>,
    pub recipes: Vec<MultiInvSlottedRecipe>,
}

struct EachInvConfig {
    pub accesses: Vec<BusAccess>,
    pub input_slots: Vec<usize>,
}

struct EachInv {
    weak: Weak<RefCell<EachInv>>,
    config: EachInvConfig,
    detail_cache: Rc<RefCell<DetailCache>>,
    server: Rc<RefCell<Server>>,
    size: Option<usize>,
}

impl_inventory!(EachInv, BusAccess);

pub struct MultiInvSlottedProcess {
    weak: Weak<RefCell<MultiInvSlottedProcess>>,
    name: &'static str,
    accesses: Vec<MultiInvAccess>,
    factory: Weak<RefCell<Factory>>,
    server: Rc<RefCell<Server>>,
    to_extract: Option<MultiInvExtractFilter>,
    recipes: Vec<MultiInvSlottedRecipe>,
    invs: Vec<Rc<RefCell<EachInv>>>,
}

impl IntoProcess for MultiInvSlottedConfig {
    type Output = MultiInvSlottedProcess;
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
                                        client: access.client,
                                        inv_addr: access.inv_addrs[i],
                                        bus_addr: access.bus_addr,
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
                recipes: self.recipes,
                invs,
            })
        })
    }
}

impl Process for MultiInvSlottedProcess {
    fn run(&self, factory: &Factory) -> AbortOnDrop<Result<(), String>> {
        if self.to_extract.is_none() && compute_demands(factory, &self.recipes).is_empty() {
            return spawn(async { Ok(()) });
        }
        let stacks: Vec<_> = self.invs.iter().map(|inv| spawn(list_inventory(&*inv.borrow()))).collect();
        let weak = self.weak.clone();
        spawn(async move {
            let stacks = join_outputs(stacks).await?;
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
                            } else if let Some(ref to_extract) = this.to_extract {
                                if to_extract(i, slot, &stack) {
                                    tasks.push(extract_output(&*this.invs[i].borrow(), factory, slot, stack.detail.max_size))
                                }
                            }
                        }
                    }
                }
                let demands = compute_demands(factory, &this.recipes);
                'recipe: for mut demand in demands.into_iter() {
                    let recipe = &this.recipes[demand.i_recipe];
                    let mut used_slots = FnvHashSet::<(usize, usize)>::default();
                    for (i_input, input) in recipe.inputs.iter().enumerate() {
                        let size_per_slot = input.size / input.slots.len() as i32;
                        for slot in &input.slots {
                            let existing_input = existing_inputs.get(&slot).unwrap();
                            let existing_size = if let Some(existing_input) = existing_input {
                                if existing_input.item != demand.inputs.items[i_input].0 {
                                    continue 'recipe;
                                }
                                existing_input.size
                            } else {
                                0
                            };
                            demand.inputs.n_sets = min(
                                demand.inputs.n_sets,
                                (min(recipe.max_per_slot, demand.inputs.items[i_input].1.max_size) - existing_size)
                                    / size_per_slot,
                            );
                            if demand.inputs.n_sets <= 0 {
                                continue 'recipe;
                            }
                            used_slots.insert(*slot);
                        }
                    }
                    for (slot, existing_input) in &existing_inputs {
                        if existing_input.is_some() && !used_slots.contains(slot) {
                            continue 'recipe;
                        }
                    }
                    tasks.push(this.execute_recipe(factory, demand));
                    break;
                }
            }
            join_tasks(tasks).await
        })
    }
}

impl MultiInvSlottedProcess {
    fn execute_recipe(&self, factory: &mut Factory, demand: Demand) -> AbortOnDrop<Result<(), String>> {
        let mut bus_slots = Vec::new();
        let slots_to_free = Rc::new(RefCell::new(Vec::new()));
        let recipe = &self.recipes[demand.i_recipe];
        for (i_input, input) in recipe.inputs.iter().enumerate() {
            let reservation = factory.reserve_item(self.name, &demand.inputs.items[i_input].0, demand.inputs.n_sets * input.size);
            let bus_slot = factory.bus_allocate();
            let slots_to_free = slots_to_free.clone();
            bus_slots.push(spawn(async move {
                let bus_slot = bus_slot.await?;
                slots_to_free.borrow_mut().push(bus_slot);
                let extraction = reservation.extract(bus_slot);
                extraction.await.map(|_| bus_slot)
            }))
        }
        let weak = self.weak.clone();
        let factory = factory.get_weak().clone();
        spawn(async move {
            let bus_slots = join_outputs(bus_slots).await;
            let slots_to_free =
                Rc::try_unwrap(slots_to_free).map_err(|_| "slots_to_free should be exclusively owned here").unwrap().into_inner();
            let task = async {
                let bus_slots = bus_slots?;
                let mut tasks = Vec::new();
                {
                    alive!(weak, this);
                    let server = this.server.borrow();
                    let access = server.load_balance(&this.accesses);
                    let mut group = Vec::new();
                    let recipe = &this.recipes[demand.i_recipe];
                    for (i_input, input) in recipe.inputs.iter().enumerate() {
                        let size_per_slot = input.size / input.slots.len() as i32;
                        for (inv_i, inv_slot) in &input.slots {
                            let action = ActionFuture::from(Call {
                                addr: access.inv_addrs[*inv_i],
                                args: vec![
                                    "pullItems".into(),
                                    access.bus_addr.into(),
                                    (bus_slots[i_input] + 1).into(),
                                    (demand.inputs.n_sets * size_per_slot).into(),
                                    (inv_slot + 1).into(),
                                ],
                            });
                            group.push(action.clone().into());
                            tasks.push(spawn(async move { action.await.map(|_| ()) }));
                        }
                    }
                    server.enqueue_request_group(access.client, group)
                }
                join_tasks(tasks).await?;
                alive_mut!(factory, factory);
                for slot_to_free in &slots_to_free {
                    factory.bus_free(*slot_to_free)
                }
                Ok(())
            };
            let result = task.await;
            if result.is_err() {
                alive(&factory)?.borrow_mut().bus_deposit(slots_to_free)
            }
            result
        })
    }
}
