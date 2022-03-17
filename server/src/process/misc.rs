use super::super::access::{BusAccess, RedstoneAccess};
use super::super::action::{ActionFuture, Log, RedstoneInput, RedstoneOutput};
use super::super::detail_cache::DetailCache;
use super::super::factory::Factory;
use super::super::inventory::{list_inventory, Inventory};
use super::super::item::{insert_into_inventory, jammer, Filter, InsertPlan};
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
    pub hold_if_unfilled: bool,
}

pub struct SyncAndRestockProcess {
    weak: Weak<RefCell<SyncAndRestockProcess>>,
    config: SyncAndRestockConfig,
    detail_cache: Rc<RefCell<DetailCache>>,
    factory: Weak<RefCell<Factory>>,
    server: Rc<RefCell<Server>>,
    size: Option<usize>,
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
                'slot: for (slot, stack) in stacks.iter_mut().enumerate() {
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
                                continue 'slot;
                            }
                        }
                        tasks.push(extract_output(this, factory, slot, some_stack.detail.max_size));
                        *stack = Some(jammer());
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
                unfilled = unfilled && this.config.hold_if_unfilled;
            }
            join_tasks(tasks).await?;
            Ok(unfilled)
        }
    }
}

impl Process for SyncAndRestockProcess {
    fn run(&self, factory: &Factory) -> AbortOnDrop<Result<(), String>> {
        let server = factory.get_server().borrow();
        let access = server.load_balance(&self.config.accesses_in);
        let action = ActionFuture::from(RedstoneInput { side: access.side, addr: access.addr, bit: access.bit });
        server.enqueue_request_group(access.client, vec![action.clone().into()]);
        let weak = self.weak.clone();
        spawn(async move {
            let is_high = action.await? > 0;
            let task = {
                alive!(weak, this);
                if this.waiting_for_low {
                    if !is_high {
                        upgrade!(this.factory, factory);
                        factory.log(Log { text: format!("{}: leave", this.config.name), color: 10 });
                    }
                    spawn(this.output(&*this.server.borrow(), is_high))
                } else {
                    if is_high {
                        let task = this.restock(weak.clone());
                        spawn(async move {
                            let skip = task.await?;
                            let task = {
                                alive!(weak, this);
                                upgrade!(this.factory, factory);
                                if skip {
                                    factory.log(Log { text: format!("{}: unfilled", this.config.name), color: 10 });
                                } else {
                                    factory.log(Log { text: format!("{}: enter", this.config.name), color: 10 });
                                }
                                let server = this.server.borrow();
                                this.output(&*server, !skip)
                            };
                            task.await
                        })
                    } else {
                        spawn(this.output(&*this.server.borrow(), false))
                    }
                }
            };
            task.into_future().await
        })
    }
}

pub struct LowAlert {
    item: Filter,
    n_wanted: i32,
    log: String,
}

impl LowAlert {
    pub fn new(item: Filter, n_wanted: i32, log: Option<String>) -> Self {
        let log = log.unwrap_or_else(|| match &item {
            Filter::Label(x) => (*x).to_owned(),
            Filter::Name(x) => format!("<{}>", x),
            Filter::Both { label, name } => format!("{} <{}>", label, name),
            Filter::Fn(_) => "<fn>".to_owned(),
        });
        Self { item, n_wanted, log }
    }
}

impl Process for LowAlert {
    fn run(&self, factory: &Factory) -> AbortOnDrop<Result<(), String>> {
        let n_stored = factory.search_n_stored(&self.item);
        if n_stored < self.n_wanted {
            factory.log(Log { text: format!("need {}*{}", self.log, self.n_wanted - n_stored), color: 6 })
        }
        spawn(async { Ok(()) })
    }
}
