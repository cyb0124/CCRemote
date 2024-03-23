use super::super::access::BusAccess;
use super::super::action::{ActionFuture, Call};
use super::super::detail_cache::DetailCache;
use super::super::factory::Factory;
use super::super::inventory::{list_inventory, Inventory};
use super::super::item::{Detail, DetailStack, Filter, Item};
use super::super::server::Server;
use super::super::util::{alive, spawn};
use super::{DepositResult, Extractor, IntoStorage, Provider, Storage};
use abort_on_drop::ChildTask;
use flexstr::LocalStr;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct DrawerConfig {
    pub accesses: Vec<BusAccess>,
    pub filters: Vec<Filter>,
}

pub struct DrawerStorage {
    weak: Weak<RefCell<DrawerStorage>>,
    config: DrawerConfig,
    detail_cache: Rc<RefCell<DetailCache>>,
    factory: Weak<RefCell<Factory>>,
    server: Rc<RefCell<Server>>,
    size: Option<usize>,
}

struct DrawerExtractor {
    weak: Weak<RefCell<DrawerStorage>>,
    inv_slot: usize,
}

impl_inventory!(DrawerStorage, BusAccess);

impl IntoStorage for DrawerConfig {
    type Output = DrawerStorage;
    fn into_storage(self, factory: &Factory) -> Rc<RefCell<Self::Output>> {
        Rc::new_cyclic(|weak| {
            RefCell::new(Self::Output {
                weak: weak.clone(),
                config: self,
                detail_cache: factory.get_detail_cache().clone(),
                factory: factory.get_weak().clone(),
                server: factory.get_server().clone(),
                size: None,
            })
        })
    }
}

impl Storage for DrawerStorage {
    fn update(&self) -> ChildTask<Result<(), LocalStr>> {
        let stacks = list_inventory(self);
        let weak = self.weak.clone();
        spawn(async move {
            let stacks = stacks.await?;
            alive!(weak, this);
            upgrade_mut!(this.factory, factory);
            for (inv_slot, stack) in stacks.into_iter().enumerate() {
                if let Some(stack) = stack {
                    factory.register_stored_item(stack.item, &stack.detail).provide(Provider {
                        priority: i32::MIN,
                        n_provided: stack.size.into(),
                        extractor: Rc::new(DrawerExtractor { weak: weak.clone(), inv_slot }),
                    });
                }
            }
            Ok(())
        })
    }

    fn cleanup(&mut self) {}

    fn deposit_priority(&mut self, item: &Rc<Item>, detail: &Rc<Detail>) -> Option<i32> {
        for filter in &self.config.filters {
            if filter.apply(item, detail) {
                return Some(i32::MAX);
            }
        }
        None
    }

    fn deposit(&mut self, stack: &DetailStack, bus_slot: usize) -> DepositResult {
        let n_deposited = stack.size;
        let server = self.server.borrow();
        let access = server.load_balance(&self.config.accesses);
        let action = ActionFuture::from(Call {
            addr: access.bus_addr.clone(),
            args: vec!["pushItems".into(), access.inv_addr.clone().into(), (bus_slot + 1).into(), n_deposited.into()],
        });
        server.enqueue_request_group(&access.client, vec![action.clone().into()]);
        let task = spawn(async move { action.await.map(|_| ()) });
        DepositResult { n_deposited, task }
    }
}

impl Extractor for DrawerExtractor {
    fn extract(&self, size: i32, bus_slot: usize) -> ChildTask<Result<(), LocalStr>> {
        upgrade!(self.weak, this);
        let server = this.server.borrow();
        let access = server.load_balance(&this.config.accesses);
        let action = ActionFuture::from(Call {
            addr: access.bus_addr.clone(),
            args: vec![
                "pullItems".into(),
                access.inv_addr.clone().into(),
                (self.inv_slot + 1).into(),
                size.into(),
                (bus_slot + 1).into(),
            ],
        });
        server.enqueue_request_group(&access.client, vec![action.clone().into()]);
        spawn(async move { action.await.map(|_| ()) })
    }
}
