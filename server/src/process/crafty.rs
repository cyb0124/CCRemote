use super::super::access::CraftyAccess;
use super::super::action::{ActionFuture, Call, TurtleCall};
use super::super::factory::Factory;
use super::super::inventory::Inventory;
use super::super::recipe::{compute_demands, resolve_inputs, CraftingGridRecipe, ResolvedInputs};
use super::super::util::{alive, join_outputs, join_tasks, spawn, AbortOnDrop};
use super::{IntoProcess, Process};
use std::{
    cell::RefCell,
    cmp::min,
    collections::VecDeque,
    rc::{Rc, Weak},
};

pub struct CraftyTurtle {
    pub client: &'static str,
    pub accesses: Vec<CraftyAccess>,
}

pub struct CraftyConfig {
    pub name: &'static str,
    pub turtles: Vec<CraftyTurtle>,
    pub recipes: Vec<CraftingGridRecipe>,
}

pub struct CraftyProcess {
    weak: Weak<RefCell<CraftyProcess>>,
    config: CraftyConfig,
    factory: Weak<RefCell<Factory>>,
    job_queue: VecDeque<usize>,
}

struct Job {
    i_recipe: usize,
    n_sets: i32,
    slots_to_free: Rc<RefCell<Vec<usize>>>,
    bus_slots: Vec<AbortOnDrop<Result<usize, String>>>,
}

struct JobRef<'a> {
    i_recipe: usize,
    i_turtle: usize,
    n_sets: i32,
    bus_slots: &'a Vec<usize>,
}

fn map_turtle_grid(slot: usize) -> usize {
    if slot >= 6 {
        slot + 2
    } else if slot >= 3 {
        slot + 1
    } else {
        slot
    }
}

impl CraftyProcess {
    fn next_job(&mut self) -> Option<Job> {
        while let Some(i_recipe) = self.job_queue.pop_front() {
            let recipe = &self.config.recipes[i_recipe];
            if recipe.max_sets <= 0 {
                continue;
            }
            upgrade_mut!(self.factory, factory);
            if let Some(ResolvedInputs { mut n_sets, items }) = resolve_inputs(factory, recipe) {
                n_sets = min(n_sets, recipe.max_sets);
                let mut bus_slots = Vec::new();
                let slots_to_free = Rc::new(RefCell::new(Vec::new()));
                for (i_input, (item, _)) in items.into_iter().enumerate() {
                    let reservation = factory.reserve_item(self.config.name, &item, n_sets * recipe.inputs[i_input].size);
                    let slots_to_free = slots_to_free.clone();
                    let bus_slot = factory.bus_allocate();
                    bus_slots.push(spawn(async move {
                        let bus_slot = bus_slot.await?;
                        slots_to_free.borrow_mut().push(bus_slot);
                        let extraction = reservation.extract(bus_slot);
                        extraction.await.map(|_| bus_slot)
                    }))
                }
                if bus_slots.is_empty() {
                    let slots_to_free = slots_to_free.clone();
                    let bus_slot = factory.bus_allocate();
                    bus_slots.push(spawn(async move {
                        let bus_slot = bus_slot.await?;
                        slots_to_free.borrow_mut().push(bus_slot);
                        Ok(bus_slot)
                    }))
                }
                return Some(Job { i_recipe, n_sets, slots_to_free, bus_slots });
            }
        }
        None
    }

    fn load_inputs(&self, job: &JobRef) -> Vec<AbortOnDrop<Result<(), String>>> {
        upgrade!(self.factory, factory);
        let server = factory.get_server().borrow();
        let access = server.load_balance(&self.config.turtles[job.i_turtle].accesses);
        let mut group = Vec::new();
        let recipe = &self.config.recipes[job.i_recipe];
        for (i_input, input) in recipe.inputs.iter().enumerate() {
            let size_per_slot = input.size / input.slots.len() as i32;
            for inv_slot in &input.slots {
                group.push(Call {
                    addr: access.bus_addr,
                    args: vec![
                        "pushItems".into(),
                        access.turtle_addr.into(),
                        (job.bus_slots[i_input] + 1).into(),
                        (size_per_slot * job.n_sets).into(),
                        (map_turtle_grid(*inv_slot) + 1).into(),
                    ],
                })
            }
        }
        for non_consumable in &recipe.non_consumables {
            group.push(Call {
                addr: access.non_consumable_addr,
                args: vec![
                    "pushItems".into(),
                    access.turtle_addr.into(),
                    (non_consumable.storage_slot + 1).into(),
                    64.into(),
                    (map_turtle_grid(non_consumable.crafting_grid_slot) + 1).into(),
                ],
            })
        }
        let group: Vec<_> = group.into_iter().map(|x| ActionFuture::from(x)).collect();
        server.enqueue_request_group(access.client, group.iter().map(|x| x.clone().into()).collect());
        group.into_iter().map(|x| spawn(async move { x.await.map(|_| ()) })).collect()
    }

    fn craft(&self, job: &JobRef) -> ActionFuture<TurtleCall> {
        let action = ActionFuture::from(TurtleCall { func: "craft", args: vec![] });
        upgrade!(self.factory, factory);
        let server = factory.get_server().borrow();
        server.enqueue_request_group(self.config.turtles[job.i_turtle].client, vec![action.clone().into()]);
        action
    }

    fn store_outputs(&self, job: &JobRef, output_bus_slot: usize) -> Vec<AbortOnDrop<Result<(), String>>> {
        upgrade!(self.factory, factory);
        let server = factory.get_server().borrow();
        let access = server.load_balance(&self.config.turtles[job.i_turtle].accesses);
        let mut group = Vec::new();
        group.push(Call {
            addr: access.bus_addr,
            args: vec!["pullItems".into(), access.turtle_addr.into(), 1.into(), 64.into(), (output_bus_slot + 1).into()],
        });
        for non_consumable in &self.config.recipes[job.i_recipe].non_consumables {
            group.push(Call {
                addr: access.non_consumable_addr,
                args: vec![
                    "pullItems".into(),
                    access.turtle_addr.into(),
                    (map_turtle_grid(non_consumable.crafting_grid_slot) + 1).into(),
                    64.into(),
                    (non_consumable.storage_slot + 1).into(),
                ],
            })
        }
        let group: Vec<_> = group.into_iter().map(|x| ActionFuture::from(x)).collect();
        server.enqueue_request_group(access.client, group.iter().map(|x| x.clone().into()).collect());
        group.into_iter().map(|x| spawn(async move { x.await.map(|_| ()) })).collect()
    }
}

async fn worker_main(weak: Weak<RefCell<CraftyProcess>>, i_turtle: usize) -> Result<(), String> {
    loop {
        let Job { i_recipe, n_sets, slots_to_free, bus_slots } =
            if let Some(job) = alive(&weak)?.borrow_mut().next_job() { job } else { break Ok(()) };
        let bus_slots = join_outputs(bus_slots).await;
        let mut slots_to_free =
            Rc::try_unwrap(slots_to_free).map_err(|_| "slots_to_free should be exclusively owned here").unwrap().into_inner();
        let task = async {
            let bus_slots = bus_slots?;
            let job = JobRef { i_recipe, i_turtle, n_sets, bus_slots: &bus_slots };
            let tasks = alive(&weak)?.borrow().load_inputs(&job);
            join_tasks(tasks).await?;
            let action = alive(&weak)?.borrow().craft(&job);
            action.await?;
            let tasks = alive(&weak)?.borrow().store_outputs(&job, slots_to_free[0]);
            join_tasks(tasks).await?;
            alive!(weak, this);
            upgrade_mut!(this.factory, factory);
            while slots_to_free.len() > 1 {
                factory.bus_free(slots_to_free.pop().unwrap())
            }
            Result::<(), String>::Ok(())
        };
        let result = task.await;
        alive_mut!(weak, this);
        this.factory.upgrade().unwrap().borrow_mut().bus_deposit(slots_to_free);
        if result.is_err() {
            this.job_queue.clear();
            break result;
        }
    }
}

impl IntoProcess for CraftyConfig {
    type Output = CraftyProcess;
    fn into_process(self, factory: &Factory) -> Rc<RefCell<Self::Output>> {
        Rc::new_cyclic(|weak| {
            RefCell::new(Self::Output {
                weak: weak.clone(),
                config: self,
                factory: factory.get_weak().clone(),
                job_queue: VecDeque::new(),
            })
        })
    }
}

impl Process for CraftyProcess {
    fn run(&self, factory: &Factory) -> AbortOnDrop<Result<(), String>> {
        let jobs = compute_demands(factory, &self.config.recipes).into_iter().map(|x| x.i_recipe).collect();
        let weak = self.weak.clone();
        spawn(async move {
            let tasks = {
                alive_mut!(weak, this);
                this.job_queue = jobs;
                (0..this.config.turtles.len()).map(|i| spawn(worker_main(weak.clone(), i))).collect()
            };
            join_tasks(tasks).await
        })
    }
}
