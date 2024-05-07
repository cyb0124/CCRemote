use super::super::access::BusAccess;
use super::super::action::{ActionFuture, Call};
use super::super::detail_cache::DetailCache;
use super::super::factory::Factory;
use super::super::inventory::{list_inventory, Inventory};
use super::super::item::{DetailStack, Filter};
use super::super::recipe::{compute_demands, Demand, Input, Outputs, Recipe};
use super::super::server::Server;
use super::super::util::{alive, join_outputs, join_tasks, spawn};
use super::{extract_output, ExtractFilter, IntoProcess, Process};
use abort_on_drop::ChildTask;
use flexstr::LocalStr;
use fnv::{FnvHashMap, FnvHashSet};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Clone)]
pub struct SlottedInput {
    item: Filter,
    pub size: i32,
    pub slots: Vec<(usize, i32)>,
    allow_backup: bool,
    extra_backup: i32,
}

impl SlottedInput {
    pub fn new(item: Filter, slots: Vec<(usize, i32)>) -> Self {
        let size = slots.iter().map(|(_, size)| size).sum();
        SlottedInput { item, size, slots, allow_backup: false, extra_backup: 0 }
    }
}

impl_input!(SlottedInput);

#[derive(Clone)]
pub struct SlottedRecipe {
    pub outputs: Rc<dyn Outputs>,
    pub inputs: Vec<SlottedInput>,
    pub max_sets: i32,
}

impl_recipe!(SlottedRecipe, SlottedInput);

pub struct SlottedConfig {
    pub name: LocalStr,
    pub accesses: Vec<BusAccess>,
    pub input_slots: Vec<usize>,
    pub to_extract: Option<ExtractFilter>,
    pub recipes: Vec<SlottedRecipe>,
    pub strict_priority: bool,
}

pub struct SlottedProcess {
    weak: Weak<RefCell<SlottedProcess>>,
    config: SlottedConfig,
    detail_cache: Rc<RefCell<DetailCache>>,
    factory: Weak<RefCell<Factory>>,
    server: Rc<RefCell<Server>>,
    size: Option<usize>,
}

impl_inventory!(SlottedProcess, BusAccess);
impl_into_process!(SlottedConfig, SlottedProcess);

impl Process for SlottedProcess {
    fn run(&self, factory: &Factory) -> ChildTask<Result<(), LocalStr>> {
        if self.config.to_extract.is_none() && compute_demands(factory, &self.config.recipes).is_empty() {
            return spawn(async { Ok(()) });
        }
        let stacks = list_inventory(self);
        let weak = self.weak.clone();
        spawn(async move {
            let stacks = stacks.await?;
            let mut tasks = Vec::new();
            {
                alive!(weak, this);
                upgrade_mut!(this.factory, factory);
                let mut existing_inputs = FnvHashMap::<usize, Option<DetailStack>>::default();
                for slot in &this.config.input_slots {
                    existing_inputs.insert(*slot, None);
                }
                for (slot, stack) in stacks.into_iter().enumerate() {
                    if let Some(stack) = stack {
                        if let Some(existing_input) = existing_inputs.get_mut(&slot) {
                            *existing_input = Some(stack)
                        } else if let Some(ref to_extract) = this.config.to_extract {
                            if to_extract(factory, slot, &stack) {
                                tasks.push(extract_output(this, factory, slot, stack.detail.max_size))
                            }
                        }
                    }
                }
                let mut demands = compute_demands(factory, &this.config.recipes);
                if this.config.strict_priority {
                    demands.truncate(1)
                }
                'recipe: for mut demand in demands.into_iter() {
                    let recipe = &this.config.recipes[demand.i_recipe];
                    let mut used_slots = FnvHashSet::<usize>::default();
                    for (i_input, input) in recipe.inputs.iter().enumerate() {
                        for (slot, mult) in &input.slots {
                            let existing_input = existing_inputs.get(slot).unwrap();
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

impl SlottedProcess {
    fn execute_recipe(&self, factory: &mut Factory, demand: Demand) -> ChildTask<Result<(), LocalStr>> {
        let mut bus_slots = Vec::new();
        let slots_to_free = Rc::new(RefCell::new(Vec::new()));
        let recipe = &self.config.recipes[demand.i_recipe];
        for (i_input, input) in recipe.inputs.iter().enumerate() {
            let reservation = factory.reserve_item(
                &self.config.name,
                &demand.inputs.items[i_input].0,
                demand.inputs.n_sets * input.size,
            );
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
            let slots_to_free = Rc::into_inner(slots_to_free).unwrap().into_inner();
            let task = async {
                let bus_slots = bus_slots?;
                let mut tasks = Vec::new();
                {
                    alive!(weak, this);
                    let server = this.server.borrow();
                    let access = server.load_balance(&this.config.accesses);
                    let mut group = Vec::new();
                    let recipe = &this.config.recipes[demand.i_recipe];
                    for (i_input, input) in recipe.inputs.iter().enumerate() {
                        for (inv_slot, mult) in &input.slots {
                            let action = ActionFuture::from(Call {
                                addr: access.bus_addr.clone(),
                                args: vec![
                                    "pushItems".into(),
                                    access.inv_addr.clone().into(),
                                    (bus_slots[i_input] + 1).into(),
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
