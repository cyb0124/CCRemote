use super::super::access::BusAccess;
use super::super::action::{ActionFuture, Call};
use super::super::factory::Factory;
use super::super::inventory::Inventory;
use super::super::recipe::{
    compute_demands, resolve_inputs, CraftingGridRecipe, Demand, NonConsumable, ResolvedInputs,
};
use super::super::util::{alive, join_outputs, join_tasks, spawn};
use super::{IntoProcess, Process};
use abort_on_drop::ChildTask;
use flexstr::LocalStr;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct WorkbenchConfig {
    pub name: LocalStr,
    pub accesses: Vec<BusAccess>,
    pub recipes: Vec<CraftingGridRecipe>,
}

pub struct WorkbenchProcess {
    weak: Weak<RefCell<WorkbenchProcess>>,
    config: WorkbenchConfig,
}

impl IntoProcess for WorkbenchConfig {
    type Output = WorkbenchProcess;
    fn into_process(self, _factory: &Factory) -> Rc<RefCell<Self::Output>> {
        Rc::new_cyclic(|weak| RefCell::new(Self::Output { weak: weak.clone(), config: self }))
    }
}

impl Process for WorkbenchProcess {
    fn run(&self, factory: &Factory) -> ChildTask<Result<(), LocalStr>> {
        let mut tasks = Vec::new();
        for Demand { i_recipe, .. } in compute_demands(factory, &self.config.recipes) {
            let recipe = &self.config.recipes[i_recipe];
            if recipe.max_sets <= 0 {
                continue;
            }
            if let Some(ResolvedInputs { mut n_sets, items, .. }) = resolve_inputs(factory, recipe) {
                n_sets = n_sets.min(recipe.max_sets);
                let mut bus_slots = Vec::new();
                let slots_to_free = Rc::new(RefCell::new(Vec::new()));
                for (i_input, (item, _)) in items.into_iter().enumerate() {
                    let reservation =
                        factory.reserve_item(&self.config.name, &item, n_sets * recipe.inputs[i_input].size);
                    let slots_to_free = slots_to_free.clone();
                    let weak = factory.get_weak().clone();
                    bus_slots.push(spawn(async move {
                        let bus_slot = alive(&weak)?.borrow_mut().bus_allocate();
                        let bus_slot = bus_slot.await?;
                        slots_to_free.borrow_mut().push(bus_slot);
                        let extraction = reservation.extract(bus_slot);
                        extraction.await.map(|_| bus_slot)
                    }))
                }
                if bus_slots.is_empty() {
                    let slots_to_free = slots_to_free.clone();
                    let weak = factory.get_weak().clone();
                    bus_slots.push(spawn(async move {
                        let bus_slot = alive(&weak)?.borrow_mut().bus_allocate();
                        let bus_slot = bus_slot.await?;
                        slots_to_free.borrow_mut().push(bus_slot);
                        Ok(bus_slot)
                    }))
                }
                let weak = self.weak.clone();
                let factory = factory.get_weak().clone();
                tasks.push(spawn(async move {
                    let bus_slots = join_outputs(bus_slots).await;
                    let mut slots_to_free = Rc::into_inner(slots_to_free).unwrap().into_inner();
                    let task = async {
                        let bus_slots = bus_slots?;
                        let tasks = {
                            alive!(weak, this);
                            upgrade!(factory, factory);
                            let server = factory.get_server().borrow();
                            let access = server.load_balance(&this.config.accesses);
                            let mut group = Vec::new();
                            let recipe = &this.config.recipes[i_recipe];
                            for (i_input, input) in recipe.inputs.iter().enumerate() {
                                for inv_slot in &input.slots {
                                    load_input(&mut group, access, bus_slots[i_input], *inv_slot, n_sets)
                                }
                            }
                            for non_consumable in &recipe.non_consumables {
                                load_non_consumable(&mut group, access, non_consumable)
                            }
                            store_output(&mut group, access, slots_to_free[0], n_sets);
                            for non_consumable in &recipe.non_consumables {
                                store_non_consumable(&mut group, access, non_consumable)
                            }
                            let group: Vec<_> = group.into_iter().map(|x| ActionFuture::from(x)).collect();
                            server.enqueue_request_group(
                                &access.client,
                                group.iter().map(|x| x.clone().into()).collect(),
                            );
                            group.into_iter().map(|x| spawn(async move { x.await.map(|_| ()) })).collect()
                        };
                        join_tasks(tasks).await?;
                        alive_mut!(factory, factory);
                        while slots_to_free.len() > 1 {
                            factory.bus_free(slots_to_free.pop().unwrap())
                        }
                        Ok(())
                    };
                    let result = task.await;
                    alive(&factory)?.borrow_mut().bus_deposit(slots_to_free);
                    result
                }))
            }
        }
        spawn(async move { join_tasks(tasks).await })
    }
}

fn load_input(group: &mut Vec<Call>, access: &BusAccess, bus_slot: usize, inv_slot: usize, size: i32) {
    group.push(Call {
        addr: access.bus_addr.clone(),
        args: vec![
            "pushItems".into(),
            access.inv_addr.clone().into(),
            (bus_slot + 1).into(),
            size.into(),
            (inv_slot + 1).into(),
        ],
    })
}

fn load_non_consumable(group: &mut Vec<Call>, access: &BusAccess, non_consumable: &NonConsumable) {
    group.push(Call {
        addr: access.inv_addr.clone(),
        args: vec![
            "pushItems".into(),
            access.inv_addr.clone().into(),
            (non_consumable.storage_slot + 11).into(),
            64.into(),
            (non_consumable.crafting_grid_slot + 1).into(),
        ],
    })
}

fn store_output(group: &mut Vec<Call>, access: &BusAccess, bus_slot: usize, n_sets: i32) {
    for _ in 0..n_sets {
        group.push(Call {
            addr: access.bus_addr.clone(),
            args: vec!["pullItems".into(), access.inv_addr.clone().into(), 10.into(), 64.into(), (bus_slot + 1).into()],
        })
    }
}

fn store_non_consumable(group: &mut Vec<Call>, access: &BusAccess, non_consumable: &NonConsumable) {
    group.push(Call {
        addr: access.inv_addr.clone(),
        args: vec![
            "pushItems".into(),
            access.inv_addr.clone().into(),
            (non_consumable.crafting_grid_slot + 1).into(),
            64.into(),
            (non_consumable.storage_slot + 11).into(),
        ],
    })
}
