use super::super::access::BusAccess;
use super::super::detail_cache::DetailCache;
use super::super::factory::Factory;
use super::super::inventory::{list_inventory, Inventory};
use super::super::item::{DetailStack, Filter};
use super::super::recipe::{compute_demands, resolve_inputs, Demand, Input, Outputs, Recipe};
use super::super::server::Server;
use super::super::util::{alive, join_tasks, spawn};
use super::{extract_output, scattering_insert, ExtractFilter, IntoProcess, Process};
use abort_on_drop::ChildTask;
use flexstr::{local_fmt, LocalStr};
use fnv::FnvHashMap;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Clone)]
pub struct ScatteringInput {
    item: Filter,
    size: i32,
    allow_backup: bool,
    extra_backup: i32,
}

impl ScatteringInput {
    pub fn new(item: Filter) -> Self { ScatteringInput { item, size: 1, allow_backup: false, extra_backup: 0 } }
}

impl_input!(ScatteringInput);

#[derive(Clone)]
pub struct ScatteringRecipe {
    outputs: Rc<dyn Outputs>,
    inputs: Vec<ScatteringInput>,
}

impl ScatteringRecipe {
    pub fn new(outputs: Rc<dyn Outputs>, input: ScatteringInput) -> Self {
        ScatteringRecipe { outputs, inputs: vec![input] }
    }
}

impl_recipe!(ScatteringRecipe, ScatteringInput);

pub struct ScatteringConfig {
    pub name: LocalStr,
    pub accesses: Vec<BusAccess>,
    // plant_sower: 6, 7, .., 14
    pub input_slots: Vec<usize>,
    pub to_extract: Option<ExtractFilter>,
    pub recipes: Vec<ScatteringRecipe>,
    pub max_per_slot: i32,
}

pub struct ScatteringProcess {
    weak: Weak<RefCell<ScatteringProcess>>,
    config: ScatteringConfig,
    detail_cache: Rc<RefCell<DetailCache>>,
    factory: Weak<RefCell<Factory>>,
    server: Rc<RefCell<Server>>,
    size: Option<usize>,
}

impl_inventory!(ScatteringProcess, BusAccess);
impl_into_process!(ScatteringConfig, ScatteringProcess);

impl Process for ScatteringProcess {
    fn run(&self, factory: &Factory) -> ChildTask<Result<(), LocalStr>> {
        if self.config.to_extract.is_none() && compute_demands(factory, &self.config.recipes).is_empty() {
            return spawn(async { Ok(()) });
        }
        let stacks = list_inventory(self);
        let weak = self.weak.clone();
        spawn(async move {
            let mut stacks = stacks.await?;
            let mut tasks = Vec::new();
            {
                alive!(weak, this);
                upgrade_mut!(this.factory, factory);
                let mut is_input_slot = vec![false; stacks.len()];
                for slot in &this.config.input_slots {
                    if *slot >= stacks.len() {
                        return Err(local_fmt!("{}: invalid slot", this.config.name));
                    }
                    is_input_slot[*slot] = true
                }
                if let Some(ref to_extract) = this.config.to_extract {
                    for (slot, stack) in stacks.iter().enumerate() {
                        if let Some(stack) = stack {
                            if !is_input_slot[slot] && to_extract(factory, slot, stack) {
                                tasks.push(extract_output(this, factory, slot, stack.detail.max_size))
                            }
                        }
                    }
                }
                for Demand { i_recipe, .. } in compute_demands(factory, &this.config.recipes) {
                    if let Some(mut inputs) = resolve_inputs(factory, &this.config.recipes[i_recipe]) {
                        let mut insertions = FnvHashMap::<usize, i32>::default();
                        let mut n_inserted = 0;
                        while inputs.n_sets > 0 {
                            let mut best = None;
                            for slot in &this.config.input_slots {
                                if let Some(ref stack) = stacks[*slot] {
                                    if stack.item == inputs.items[0].0 {
                                        if let Some((_, best_size)) = best {
                                            if stack.size >= best_size {
                                                continue;
                                            }
                                        }
                                        best = Some((*slot, stack.size))
                                    }
                                } else {
                                    best = Some((*slot, 0));
                                    break;
                                }
                            }
                            let Some((slot, size)) = best else { break };
                            if size >= this.config.max_per_slot.min(inputs.items[0].1.max_size) {
                                break;
                            }
                            inputs.n_sets -= 1;
                            n_inserted += 1;
                            *insertions.entry(slot).or_default() += 1;
                            let stack = &mut stacks[slot];
                            if let Some(ref mut stack) = stack {
                                stack.size += 1
                            } else {
                                let (item, detail) = inputs.items[0].clone();
                                *stack = Some(DetailStack { item, detail, size: 1 })
                            }
                        }
                        if n_inserted > 0 {
                            let reservation = factory.reserve_item(&this.config.name, &inputs.items[0].0, n_inserted);
                            tasks.push(scattering_insert(this, factory, reservation, insertions))
                        }
                    }
                }
            }
            join_tasks(tasks).await
        })
    }
}
