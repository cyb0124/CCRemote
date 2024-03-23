use super::super::access::{BusAccess, RedstoneAccess};
use super::super::action::{ActionFuture, Call, Log, RedstoneInput, RedstoneOutput};
use super::super::detail_cache::DetailCache;
use super::super::factory::Factory;
use super::super::inventory::{list_inventory, Inventory};
use super::super::item::{insert_into_inventory, jammer, Filter, InsertPlan};
use super::super::recipe::Input;
use super::super::server::Server;
use super::super::util::{alive, join_tasks, spawn};
use super::{extract_output, scattering_insert, BufferedInput, IntoProcess, Process, ScatteringInput};
use abort_on_drop::ChildTask;
use flexstr::{local_fmt, LocalStr};
use std::{
    cell::RefCell,
    fs::read_to_string,
    future::Future,
    iter::once,
    rc::{Rc, Weak},
    str::FromStr,
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
    fn run(&self, factory: &Factory) -> ChildTask<Result<(), LocalStr>> {
        if (self.condition)(factory) {
            self.child.borrow().run(factory)
        } else {
            spawn(async { Ok(()) })
        }
    }
}

pub struct SyncAndRestockConfig {
    pub name: LocalStr,
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
    fn output(&self, server: &Server, is_high: bool) -> impl Future<Output = Result<(), LocalStr>> {
        let access = server.load_balance(&self.config.accesses_out);
        let value = if is_high { 15 } else { 0 };
        let action = ActionFuture::from(RedstoneOutput {
            side: access.side.clone(),
            addr: access.addr.clone(),
            bit: access.bit,
            value,
        });
        server.enqueue_request_group(&access.client, vec![action.clone().into()]);
        let weak = self.weak.clone();
        async move {
            action.await?;
            alive_mut!(weak, this);
            this.waiting_for_low = is_high;
            Ok(())
        }
    }

    fn restock(&self, weak: Weak<RefCell<Self>>) -> impl Future<Output = Result<bool, LocalStr>> {
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
                        let reservation = factory.reserve_item(&this.config.name, item, n_inserted);
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
    fn run(&self, factory: &Factory) -> ChildTask<Result<(), LocalStr>> {
        let server = factory.get_server().borrow();
        let access = server.load_balance(&self.config.accesses_in);
        let action =
            ActionFuture::from(RedstoneInput { side: access.side.clone(), addr: access.addr.clone(), bit: access.bit });
        server.enqueue_request_group(&access.client, vec![action.clone().into()]);
        let weak = self.weak.clone();
        spawn(async move {
            let is_high = action.await? > 0;
            let task = {
                alive!(weak, this);
                if this.waiting_for_low {
                    if !is_high {
                        upgrade!(this.factory, factory);
                        factory.log(Log { text: local_fmt!("{}: leave", this.config.name), color: 10 });
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
                                    factory.log(Log { text: local_fmt!("{}: unfilled", this.config.name), color: 10 });
                                } else {
                                    factory.log(Log { text: local_fmt!("{}: enter", this.config.name), color: 10 });
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
            task.await.unwrap()
        })
    }
}

pub struct LowAlert {
    item: Filter,
    n_wanted: i32,
    log: LocalStr,
}

impl LowAlert {
    pub fn new(item: Filter, n_wanted: i32) -> Self {
        let log = match &item {
            Filter::Label(x) => x.clone(),
            Filter::Name(x) => local_fmt!("<{}>", x),
            Filter::Both { label, name } => local_fmt!("{} <{}>", label, name),
            Filter::Custom { desc, .. } => local_fmt!("<{}>", desc),
        };
        Self { item, n_wanted, log }
    }
}

impl Process for LowAlert {
    fn run(&self, factory: &Factory) -> ChildTask<Result<(), LocalStr>> {
        let n_stored = factory.search_n_stored(&self.item);
        if n_stored < self.n_wanted {
            factory.log(Log { text: local_fmt!("need {}*{}", self.log, self.n_wanted - n_stored), color: 6 })
        }
        spawn(async { Ok(()) })
    }
}

pub struct FluidLowAlert(pub LocalStr, pub i64);
impl Process for FluidLowAlert {
    fn run(&self, factory: &Factory) -> ChildTask<Result<(), LocalStr>> {
        let n_stored = factory.search_n_fluid(&self.0);
        if n_stored < self.1 {
            factory.log(Log { text: local_fmt!("need {}*{}", self.0, self.1 - n_stored), color: 6 })
        }
        spawn(async { Ok(()) })
    }
}

pub struct ItemCycleConfig {
    pub name: LocalStr,
    pub file_name: LocalStr,
    pub accesses: Vec<BusAccess>,
    pub slot: usize,
    pub items: Vec<ScatteringInput>,
}

pub struct ItemCycleProcess {
    weak: Weak<RefCell<ItemCycleProcess>>,
    config: ItemCycleConfig,
    detail_cache: Rc<RefCell<DetailCache>>,
    factory: Weak<RefCell<Factory>>,
    server: Rc<RefCell<Server>>,
    size: Option<usize>,
    next_item: usize,
}

impl IntoProcess for ItemCycleConfig {
    type Output = ItemCycleProcess;
    fn into_process(self, factory: &Factory) -> Rc<RefCell<Self::Output>> {
        let next_item =
            read_to_string(&*self.file_name).ok().and_then(|x| usize::from_str(&x).ok()).unwrap_or_default();
        Rc::new_cyclic(|weak| {
            RefCell::new(Self::Output {
                weak: weak.clone(),
                config: self,
                detail_cache: factory.get_detail_cache().clone(),
                factory: factory.get_weak().clone(),
                server: factory.get_server().clone(),
                size: None,
                next_item,
            })
        })
    }
}

impl_inventory!(ItemCycleProcess, BusAccess);

impl Process for ItemCycleProcess {
    fn run(&self, _: &Factory) -> ChildTask<Result<(), LocalStr>> {
        let stacks = list_inventory(self);
        let weak = self.weak.clone();
        spawn(async move {
            let stacks = stacks.await?;
            let mut slot_to_free = None;
            let task = {
                alive!(weak, this);
                if this.config.slot >= stacks.len() {
                    return Err(local_fmt!("{}: invalid slot", this.config.name));
                }
                if stacks[this.config.slot].is_some() {
                    return Ok(());
                }
                upgrade_mut!(this.factory, factory);
                let input = &this.config.items[this.next_item];
                if let Some((item, info)) = factory.search_item(input.get_item()) {
                    if info.borrow().get_availability(input.get_allow_backup(), input.get_extra_backup()) < 1 {
                        return Ok(());
                    }
                    let reservation = factory.reserve_item(&this.config.name, item, 1);
                    let bus_slot = factory.bus_allocate();
                    let weak = weak.clone();
                    let slot_to_free = &mut slot_to_free;
                    async move {
                        let bus_slot = bus_slot.await?;
                        *slot_to_free = Some(bus_slot);
                        reservation.extract(bus_slot).await?;
                        let task = {
                            alive_mut!(weak, this);
                            let server = this.server.borrow();
                            let access = server.load_balance(&this.config.accesses);
                            let action = ActionFuture::from(Call {
                                addr: access.bus_addr.clone(),
                                args: vec![
                                    "pushItems".into(),
                                    access.inv_addr.clone().into(),
                                    (bus_slot + 1).into(),
                                    1.into(),
                                    (this.config.slot + 1).into(),
                                ],
                            });
                            server.enqueue_request_group(&access.client, vec![action.clone().into()]);
                            action
                        };
                        task.await?;
                        alive_mut!(weak, this);
                        upgrade_mut!(this.factory, factory);
                        factory.bus_free(bus_slot);
                        *slot_to_free = None;
                        this.next_item += 1;
                        if this.next_item == this.config.items.len() {
                            this.next_item = 0
                        }
                        std::fs::write(&*this.config.file_name, this.next_item.to_string())
                            .map_err(|e| local_fmt!("{}: {}", this.config.name, e))
                    }
                } else {
                    return Ok(());
                }
            };
            let result = task.await;
            if let Some(slot) = slot_to_free {
                alive(&weak)?.borrow().factory.upgrade().unwrap().borrow_mut().bus_deposit(once(slot))
            }
            result
        })
    }
}
