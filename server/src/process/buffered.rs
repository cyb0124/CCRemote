use super::super::access::BusAccess;
use super::super::action::{ActionFuture, Call};
use super::super::detail_cache::DetailCache;
use super::super::factory::Factory;
use super::super::inventory::{list_inventory, Inventory};
use super::super::item::{insert_into_inventory, jammer, Detail, Filter, InsertPlan, Item};
use super::super::recipe::{compute_demands, resolve_inputs, Demand, Input, Output, Recipe};
use super::super::server::Server;
use super::super::util::{alive, join_outputs, join_tasks, spawn, AbortOnDrop};
use super::{extract_output, scattering_insert, ExtractFilter, IntoProcess, Process, SlotFilter};
use fnv::FnvHashMap;
use std::{
    cell::RefCell,
    cmp::min,
    mem::drop,
    rc::{Rc, Weak},
};

pub struct BufferedInput {
    item: Filter,
    size: i32,
    allow_backup: bool,
    extra_backup: i32,
}

impl BufferedInput {
    pub fn new(item: Filter, size: i32) -> Self { BufferedInput { item, size, allow_backup: false, extra_backup: 0 } }
}

impl_input!(BufferedInput);

pub struct BufferedRecipe {
    pub outputs: Vec<Output>,
    pub inputs: Vec<BufferedInput>,
    pub max_inputs: i32,
}

impl_recipe!(BufferedRecipe, BufferedInput);

pub struct BufferedConfig {
    pub name: &'static str,
    pub accesses: Vec<BusAccess>,
    pub slot_filter: Option<SlotFilter>,
    pub to_extract: Option<ExtractFilter>,
    pub recipes: Vec<BufferedRecipe>,
    pub max_recipe_inputs: i32,
    pub stocks: Vec<BufferedInput>,
}

pub struct BufferedProcess {
    weak: Weak<RefCell<BufferedProcess>>,
    config: BufferedConfig,
    detail_cache: Rc<RefCell<DetailCache>>,
    factory: Weak<RefCell<Factory>>,
    server: Rc<RefCell<Server>>,
    size: Option<usize>,
}

impl_inventory!(BufferedProcess, BusAccess);
impl_into_process!(BufferedConfig, BufferedProcess);

impl Process for BufferedProcess {
    fn run(&self, factory: &Factory) -> AbortOnDrop<Result<(), String>> {
        if self.config.to_extract.is_none() && self.config.stocks.is_empty() {
            if compute_demands(factory, &self.config.recipes).is_empty() {
                return spawn(async { Ok(()) });
            }
        }
        let stacks = list_inventory(self);
        let weak = self.weak.clone();
        spawn(async move {
            let mut stacks = stacks.await?;
            let mut tasks = Vec::new();
            {
                alive!(weak, this);
                upgrade_mut!(this.factory, factory);
                let mut remaining_size = this.config.max_recipe_inputs;
                let mut existing_size = FnvHashMap::<Rc<Item>, i32>::default();
                'slot: for (slot, stack) in stacks.iter_mut().enumerate() {
                    if let Some(ref to_extract) = this.config.to_extract {
                        if let Some(some_stack) = stack {
                            if to_extract(slot, some_stack) {
                                tasks.push(extract_output(this, factory, slot, some_stack.detail.max_size));
                                *stack = Some(jammer());
                                continue 'slot;
                            }
                        }
                    }
                    if let Some(ref slot_filter) = this.config.slot_filter {
                        if !slot_filter(slot) {
                            *stack = Some(jammer());
                            continue 'slot;
                        }
                    }
                    if let Some(stack) = stack {
                        *existing_size.entry(stack.item.clone()).or_default() += stack.size;
                        for stock in &this.config.stocks {
                            if stock.item.apply(&stack.item, &stack.detail) {
                                continue 'slot;
                            }
                        }
                        remaining_size -= stack.size
                    }
                }
                for stock in &this.config.stocks {
                    if let Some((item, info)) = factory.search_item(&stock.item) {
                        let info = info.borrow();
                        let existing = existing_size.entry(item.clone()).or_default();
                        let to_insert =
                            min(stock.size - *existing, info.get_availability(stock.allow_backup, stock.extra_backup));
                        if to_insert <= 0 {
                            continue;
                        }
                        let InsertPlan { n_inserted, insertions } =
                            insert_into_inventory(&mut stacks, item, &info.detail, to_insert);
                        drop(info);
                        if n_inserted <= 0 {
                            continue;
                        }
                        *existing += n_inserted;
                        let reservation = factory.reserve_item(this.config.name, item, n_inserted);
                        tasks.push(scattering_insert(this, factory, reservation, insertions))
                    }
                }
                if remaining_size > 0 {
                    'recipe: for Demand { i_recipe, .. } in compute_demands(factory, &this.config.recipes) {
                        let recipe = &this.config.recipes[i_recipe];
                        if let Some(mut inputs) = resolve_inputs(factory, recipe) {
                            let size_per_set: i32 = recipe.inputs.iter().map(|x| x.size).sum();
                            inputs.n_sets = min(inputs.n_sets, remaining_size / size_per_set);
                            if inputs.n_sets <= 0 {
                                continue 'recipe;
                            }
                            let existing_total: i32 =
                                inputs.items.iter().map(|(item, _)| *existing_size.entry(item.clone()).or_default()).sum();
                            inputs.n_sets = min(inputs.n_sets, (recipe.max_inputs - existing_total) / size_per_set);
                            if inputs.n_sets <= 0 {
                                continue 'recipe;
                            }
                            let backup = stacks.clone();
                            let mut plans = Vec::new();
                            plans.reserve(recipe.inputs.len());
                            'retry: loop {
                                for (i_input, (item, detail)) in inputs.items.iter().enumerate() {
                                    let to_insert = inputs.n_sets * recipe.inputs[i_input].size;
                                    let plan = insert_into_inventory(&mut stacks, item, detail, to_insert);
                                    if plan.n_inserted == to_insert {
                                        plans.push(plan)
                                    } else {
                                        inputs.n_sets -= 1;
                                        if inputs.n_sets <= 0 {
                                            continue 'recipe;
                                        }
                                        plans.clear();
                                        stacks = backup.clone();
                                        continue 'retry;
                                    }
                                }
                                break 'retry;
                            }
                            for (i_input, (item, _)) in inputs.items.iter().enumerate() {
                                *existing_size.get_mut(item).unwrap() += plans[i_input].n_inserted
                            }
                            remaining_size -= inputs.n_sets * size_per_set;
                            tasks.push(this.execute_recipe(factory, inputs.items, plans));
                            if remaining_size <= 0 {
                                break 'recipe;
                            }
                        }
                    }
                }
            }
            join_tasks(tasks).await
        })
    }
}

impl BufferedProcess {
    fn execute_recipe(
        &self,
        factory: &mut Factory,
        items: Vec<(Rc<Item>, Rc<Detail>)>,
        plans: Vec<InsertPlan>,
    ) -> AbortOnDrop<Result<(), String>> {
        let mut bus_slots = Vec::new();
        let slots_to_free = Rc::new(RefCell::new(Vec::new()));
        for (i_input, (item, _)) in items.into_iter().enumerate() {
            let reservation = factory.reserve_item(self.config.name, &item, plans[i_input].n_inserted);
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
                    let server = this.get_server().borrow();
                    let access = server.load_balance(&this.config.accesses);
                    let mut group = Vec::new();
                    for (i_input, InsertPlan { insertions, .. }) in plans.into_iter().enumerate() {
                        for (inv_slot, size) in insertions {
                            let action = ActionFuture::from(Call {
                                addr: access.inv_addr,
                                args: vec![
                                    "pullItems".into(),
                                    access.bus_addr.into(),
                                    (bus_slots[i_input] + 1).into(),
                                    size.into(),
                                    (inv_slot + 1).into(),
                                ],
                            });
                            group.push(action.clone().into());
                            tasks.push(spawn(async move { action.await.map(|_| ()) }))
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
