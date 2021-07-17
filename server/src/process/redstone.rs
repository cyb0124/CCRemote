use super::super::access::RedstoneAccess;
use super::super::action::{ActionFuture, Log, RedstoneInput, RedstoneOutput};
use super::super::factory::Factory;
use super::super::inventory::Inventory;
use super::super::item::Filter;
use super::super::util::{alive, spawn, AbortOnDrop};
use super::{IntoProcess, Process};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub type RedstoneFn = Box<dyn Fn(&Factory) -> u8>;
pub fn emit_when_want_item(name: &'static str, item: Filter, n_wanted: i32) -> RedstoneFn {
    Box::new(move |factory| {
        if factory.search_n_stored(&item) < n_wanted {
            factory.log(Log { text: format!("{}: on", name), color: 10 });
            15
        } else {
            0
        }
    })
}

pub struct RedstoneEmitterConfig {
    pub accesses: Vec<RedstoneAccess>,
    pub output: RedstoneFn,
}

pub struct RedstoneEmitterProcess {
    weak: Weak<RefCell<RedstoneEmitterProcess>>,
    config: RedstoneEmitterConfig,
    prev_value: Option<u8>,
}

impl IntoProcess for RedstoneEmitterConfig {
    type Output = RedstoneEmitterProcess;
    fn into_process(self, _factory: &Factory) -> Rc<RefCell<Self::Output>> {
        Rc::new_cyclic(|weak| RefCell::new(Self::Output { weak: weak.clone(), config: self, prev_value: None }))
    }
}

impl Process for RedstoneEmitterProcess {
    fn run(&self, factory: &Factory) -> AbortOnDrop<Result<(), String>> {
        let value = (self.config.output)(factory);
        if Some(value) == self.prev_value {
            spawn(async { Ok(()) })
        } else {
            let server = factory.get_server().borrow();
            let access = server.load_balance(&self.config.accesses);
            let action = ActionFuture::from(RedstoneOutput { side: access.side, addr: access.addr, value });
            server.enqueue_request_group(access.client, vec![action.clone().into()]);
            let weak = self.weak.clone();
            spawn(async move {
                action.await?;
                alive_mut!(weak, this);
                this.prev_value = Some(value);
                Ok(())
            })
        }
    }
}

pub struct RedstoneConditionalConfig<T: IntoProcess> {
    pub name: Option<&'static str>,
    pub accesses: Vec<RedstoneAccess>,
    pub condition: Box<dyn Fn(u8) -> bool>,
    pub child: T,
}

pub struct RedstoneConditionalProcess<T: Process> {
    weak: Weak<RefCell<RedstoneConditionalProcess<T>>>,
    name: Option<&'static str>,
    accesses: Vec<RedstoneAccess>,
    condition: Box<dyn Fn(u8) -> bool>,
    child: Rc<RefCell<T>>,
}

impl<T: IntoProcess> IntoProcess for RedstoneConditionalConfig<T> {
    type Output = RedstoneConditionalProcess<T::Output>;
    fn into_process(self, factory: &Factory) -> Rc<RefCell<Self::Output>> {
        Rc::new_cyclic(|weak| {
            RefCell::new(Self::Output {
                weak: weak.clone(),
                name: self.name,
                accesses: self.accesses,
                condition: self.condition,
                child: self.child.into_process(factory),
            })
        })
    }
}

impl<T: Process> Process for RedstoneConditionalProcess<T> {
    fn run(&self, factory: &Factory) -> AbortOnDrop<Result<(), String>> {
        let server = factory.get_server().borrow();
        let access = server.load_balance(&self.accesses);
        let action = ActionFuture::from(RedstoneInput { side: access.side, addr: access.addr });
        server.enqueue_request_group(access.client, vec![action.clone().into()]);
        let weak = self.weak.clone();
        let factory = factory.get_weak().clone();
        spawn(async move {
            let value = action.await?;
            let task = {
                alive!(weak, this);
                upgrade!(factory, factory);
                if (this.condition)(value) {
                    this.child.borrow().run(factory)
                } else {
                    if let Some(name) = this.name {
                        factory.log(Log { text: format!("{}: skipped", name), color: 14 })
                    }
                    return Ok(());
                }
            };
            task.into_future().await
        })
    }
}
