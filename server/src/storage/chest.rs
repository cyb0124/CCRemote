use super::super::access::BusAccess;
use super::super::action::{ActionFuture, Call};
use super::super::detail_cache::DetailCache;
use super::super::factory::Factory;
use super::super::inventory::{list_inventory, Inventory};
use super::super::item::{Detail, DetailStack, Item};
use super::super::server::Server;
use super::super::util::{alive, spawn};
use super::{DepositResult, Extractor, IntoStorage, Provider, Storage};
use abort_on_drop::ChildTask;
use flexstr::LocalStr;
use std::{
    cell::RefCell,
    cmp::min,
    rc::{Rc, Weak},
};

pub struct ChestConfig {
    pub accesses: Vec<BusAccess>,
    pub override_max_stack_size: Option<Box<dyn Fn(i32) -> i32>>,
}

impl ChestConfig {
    fn max_size(&self, orig_max_size: i32) -> i32 {
        match &self.override_max_stack_size {
            Some(f) => f(orig_max_size),
            None => orig_max_size,
        }
    }
}

pub struct ChestStorage {
    weak: Weak<RefCell<ChestStorage>>,
    config: ChestConfig,
    detail_cache: Rc<RefCell<DetailCache>>,
    factory: Weak<RefCell<Factory>>,
    server: Rc<RefCell<Server>>,
    size: Option<usize>,
    stacks: Vec<Option<DetailStack>>,
    inv_slot_to_deposit: usize,
}

impl_inventory!(ChestStorage, BusAccess);

struct ChestExtractor {
    weak: Weak<RefCell<ChestStorage>>,
    inv_slot: usize,
}

impl IntoStorage for ChestConfig {
    type Output = ChestStorage;
    fn into_storage(self, factory: &Factory) -> Rc<RefCell<Self::Output>> {
        Rc::new_cyclic(|weak| {
            RefCell::new(Self::Output {
                weak: weak.clone(),
                config: self,
                detail_cache: factory.get_detail_cache().clone(),
                factory: factory.get_weak().clone(),
                server: factory.get_server().clone(),
                size: None,
                stacks: Vec::new(),
                inv_slot_to_deposit: 0,
            })
        })
    }
}

impl Storage for ChestStorage {
    fn update(&self) -> ChildTask<Result<(), LocalStr>> {
        let stacks = list_inventory(self);
        let weak = self.weak.clone();
        spawn(async move {
            let stacks = stacks.await?;
            alive_mut!(weak, this);
            this.stacks = stacks;
            upgrade_mut!(this.factory, factory);
            for (inv_slot, stack) in this.stacks.iter().enumerate() {
                if let Some(stack) = stack {
                    factory.register_stored_item(stack.item.clone(), &stack.detail).provide(Provider {
                        priority: -stack.size,
                        n_provided: stack.size.into(),
                        extractor: Rc::new(ChestExtractor { weak: weak.clone(), inv_slot }),
                    });
                }
            }
            Ok(())
        })
    }

    fn cleanup(&mut self) { self.stacks.clear() }

    fn deposit_priority(&mut self, item: &Rc<Item>, detail: &Rc<Detail>) -> Option<i32> {
        let mut empty_slot = None;
        let mut size_of_best_slot = None;
        for (inv_slot, stack) in self.stacks.iter().enumerate() {
            if let Some(stack) = stack {
                if stack.item == *item && stack.size < self.config.max_size(detail.max_size) {
                    if let Some(best_size) = size_of_best_slot {
                        if stack.size <= best_size {
                            continue;
                        }
                    }
                    size_of_best_slot = Some(stack.size);
                    self.inv_slot_to_deposit = inv_slot
                }
            } else {
                empty_slot = Some(inv_slot)
            }
        }
        size_of_best_slot.or_else(|| {
            empty_slot.map(|x| {
                self.inv_slot_to_deposit = x;
                i32::MIN
            })
        })
    }

    fn deposit(&mut self, stack: &DetailStack, bus_slot: usize) -> DepositResult {
        let inv_slot = self.inv_slot_to_deposit;
        let inv_stack = &mut self.stacks[inv_slot];
        let n_deposited;
        if let Some(inv_stack) = inv_stack {
            n_deposited = min(stack.size, self.config.max_size(inv_stack.detail.max_size) - inv_stack.size);
            inv_stack.size += n_deposited
        } else {
            n_deposited = stack.size;
            *inv_stack = Some(stack.clone())
        }
        let server = self.server.borrow();
        let access = server.load_balance(&self.config.accesses);
        let action = ActionFuture::from(Call {
            addr: access.bus_addr.clone(),
            args: vec![
                "pushItems".into(),
                access.inv_addr.clone().into(),
                (bus_slot + 1).into(),
                n_deposited.into(),
                (inv_slot + 1).into(),
            ],
        });
        server.enqueue_request_group(&access.client, vec![action.clone().into()]);
        let task = spawn(async move { action.await.map(|_| ()) });
        DepositResult { n_deposited, task }
    }
}

impl Extractor for ChestExtractor {
    fn extract(&self, size: i32, bus_slot: usize) -> ChildTask<Result<(), LocalStr>> {
        let inv_slot = self.inv_slot;
        upgrade!(self.weak, this);
        let server = this.server.borrow();
        let access = server.load_balance(&this.config.accesses);
        let action = ActionFuture::from(Call {
            addr: access.bus_addr.clone(),
            args: vec![
                "pullItems".into(),
                access.inv_addr.clone().into(),
                (inv_slot + 1).into(),
                size.into(),
                (bus_slot + 1).into(),
            ],
        });
        server.enqueue_request_group(&access.client, vec![action.clone().into()]);
        let weak = self.weak.clone();
        spawn(async move {
            action.await?;
            alive_mut!(weak, this);
            let inv_stack = &mut this.stacks[inv_slot];
            let inv_size = &mut inv_stack.as_mut().unwrap().size;
            *inv_size -= size;
            if *inv_size <= 0 {
                *inv_stack = None;
            }
            Ok(())
        })
    }
}
