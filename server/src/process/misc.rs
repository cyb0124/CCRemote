use super::super::access::{BusAccess, RedstoneAccess};
use super::super::action::{ActionFuture, Log, RedstoneInput, RedstoneOutput};
use super::super::detail_cache::DetailCache;
use super::super::factory::Factory;
use super::super::inventory::{list_inventory, Inventory};
use super::super::item::{insert_into_inventory, jammer, InsertPlan};
use super::super::recipe::Input;
use super::super::server::Server;
use super::super::util::{alive, join_tasks, spawn, AbortOnDrop};
use super::{extract_output, scattering_insert, BufferedInput, IntoProcess, Process};
use std::{
    cell::RefCell,
    future::Future,
    rc::{Rc, Weak},
};

pub struct ConditionalConfig<T: IntoProcess> {
    pub condition: Box<dyn Fn(&Factory) -> bool>,
    pub child: T,
}

pub struct ConditionalProcess<T: Process> {
    condition: Box<dyn Fn(&Factory) -> bool>,
    child: Rc<RefCell<T>>,
}

impl<T: IntoProcess> IntoProcess for ConditionalConfig<T> {
    type Output = ConditionalProcess<T::Output>;
    fn into_process(self, factory: &Factory) -> Rc<RefCell<Self::Output>> {
        Rc::new(RefCell::new(Self::Output { condition: self.condition, child: self.child.into_process(factory) }))
    }
}

impl<T: Process> Process for ConditionalProcess<T> {
    fn run(&self, factory: &Factory) -> AbortOnDrop<Result<(), String>> {
        if (self.condition)(factory) {
            self.child.borrow().run(factory)
        } else {
            spawn(async { Ok(()) })
        }
    }
}

pub struct SyncAndRestockConfig {
    pub name: &'static str,
    pub accesses: Vec<BusAccess>,
    pub accesses_in: Vec<RedstoneAccess>,
    pub accesses_out: Vec<RedstoneAccess>,
    pub stocks: Box<dyn Fn(&Factory) -> Vec<BufferedInput>>,
    pub hold_if_not_stocked: bool,
}

pub struct SyncAndRestockProcess {
    weak: Weak<RefCell<SyncAndRestockProcess>>,
    config: SyncAndRestockConfig,
    detail_cache: Rc<RefCell<DetailCache>>,
    factory: Weak<RefCell<Factory>>,
    server: Rc<RefCell<Server>>,
    size: Option<usize>,
    initialized: bool,
    waiting_for_low: bool,
}

impl_inventory!(SyncAndRestockProcess, BusAccess);

impl IntoProcess for SyncAndRestockConfig {
    type Output = SyncAndRestockProcess;
    fn into_process(self, factory: &Factory) -> Rc<RefCell<Self::Output>> {
        Rc::new_cyclic(|weak| {
            RefCell::new(Self::Output {
                weak: weak.clone(),
                config: self,
                detail_cache: factory.get_detail_cache().clone(),
                factory: factory.get_weak().clone(),
                server: factory.get_server().clone(),
                size: None,
                initialized: false,
                waiting_for_low: false,
            })
        })
    }
}

impl SyncAndRestockProcess {
    fn output(&self, server: &Server, is_high: bool) -> impl Future<Output = Result<(), String>> {
        let access = server.load_balance(&self.config.accesses_out);
        let value = if is_high { 15 } else { 0 };
        let action =
            ActionFuture::from(RedstoneOutput { side: access.side, addr: access.addr, bit: access.bit, value });
        server.enqueue_request_group(access.client, vec![action.clone().into()]);
        let weak = self.weak.clone();
        async move {
            action.await?;
            alive_mut!(weak, this);
            this.initialized = true;
            this.waiting_for_low = is_high;
            Ok(())
        }
    }

    fn restock(&self, weak: Weak<RefCell<Self>>) -> impl Future<Output = Result<bool, String>> {
        let stacks = list_inventory(self);
        async move {
            let mut stacks = stacks.await?;
            let mut tasks = Vec::new();
            let mut unfilled = false;
            {
                alive!(weak, this);
                upgrade_mut!(this.factory, factory);
                let stocks = (this.config.stocks)(factory);
                let mut remaining_stocks: Vec<_> = stocks.iter().map(|x| x.get_size()).collect();
                for (slot, stack) in stacks.iter_mut().enumerate() {
                    if let Some(some_stack) = stack {
                        for (stock, remaining) in stocks.iter().zip(&mut remaining_stocks) {
                            if stock.get_item().apply(&some_stack.item, &some_stack.detail) {
                                let to_keep = some_stack.size.min(*remaining);
                                *remaining -= to_keep;
                                let to_extract = some_stack.size - to_keep;
                                if to_extract > 0 {
                                    tasks.push(extract_output(this, factory, slot, to_extract))
                                }
                                some_stack.size -= to_extract;
                                if some_stack.size <= 0 {
                                    *stack = Some(jammer())
                                }
                                break;
                            }
                        }
                    }
                }
                for (stock, remaining) in stocks.iter().zip(&mut remaining_stocks) {
                    if let Some((item, info)) = factory.search_item(&stock.get_item()) {
                        let info = info.borrow();
                        let to_insert =
                            info.get_availability(stock.get_allow_backup(), stock.get_extra_backup()).min(*remaining);
                        if to_insert <= 0 {
                            continue;
                        }
                        let InsertPlan { n_inserted, insertions } =
                            insert_into_inventory(&mut stacks, item, &info.detail, to_insert);
                        drop(info);
                        if n_inserted <= 0 {
                            continue;
                        }
                        *remaining -= n_inserted;
                        let reservation = factory.reserve_item(this.config.name, item, n_inserted);
                        tasks.push(scattering_insert(this, factory, reservation, insertions))
                    }
                    if *remaining > 0 {
                        unfilled = true
                    }
                }
                unfilled = unfilled && this.config.hold_if_not_stocked;
            }
            join_tasks(tasks).await?;
            Ok(unfilled)
        }
    }

    fn run_initialized(&self, server: &Server) -> AbortOnDrop<Result<(), String>> {
        let access = server.load_balance(&self.config.accesses_in);
        let action = ActionFuture::from(RedstoneInput { side: access.side, addr: access.addr, bit: access.bit });
        server.enqueue_request_group(access.client, vec![action.clone().into()]);
        let weak = self.weak.clone();
        spawn(async move {
            let is_high = action.await? > 0;
            let task = {
                alive!(weak, this);
                if this.waiting_for_low {
                    if is_high {
                        return Ok(());
                    } else {
                        upgrade!(this.factory, factory);
                        factory.log(Log { text: format!("{}: leave", this.config.name), color: 10 });
                        spawn(this.output(&*this.server.borrow(), false))
                    }
                } else {
                    if is_high {
                        let task = this.restock(weak.clone());
                        spawn(async move {
                            let skip = task.await?;
                            if skip {
                                alive!(weak, this);
                                upgrade!(this.factory, factory);
                                factory.log(Log { text: format!("{}: unfilled", this.config.name), color: 10 });
                                Ok(())
                            } else {
                                let task = {
                                    alive!(weak, this);
                                    upgrade!(this.factory, factory);
                                    factory.log(Log { text: format!("{}: enter", this.config.name), color: 10 });
                                    let server = this.server.borrow();
                                    this.output(&*server, true)
                                };
                                task.await
                            }
                        })
                    } else {
                        return Ok(());
                    }
                }
            };
            task.into_future().await
        })
    }
}

impl Process for SyncAndRestockProcess {
    fn run(&self, factory: &Factory) -> AbortOnDrop<Result<(), String>> {
        let server = factory.get_server().borrow();
        if self.initialized {
            self.run_initialized(&*server)
        } else {
            spawn(self.output(&*server, false))
        }
    }
}
