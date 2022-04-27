use super::super::access::RedstoneAccess;
use super::super::action::{ActionFuture, Log, RedstoneInput, RedstoneOutput};
use super::super::factory::Factory;
use super::super::inventory::Inventory;
use super::super::recipe::Outputs;
use super::super::util::{alive, spawn};
use super::{IntoProcess, Process};
use abort_on_drop::ChildTask;
use flexstr::{local_fmt, LocalStr};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub type RedstoneFn = Box<dyn Fn(&Factory) -> u8>;
pub fn emit_when_want_item(name: LocalStr, off: u8, on: u8, outputs: Box<dyn Outputs>) -> RedstoneFn {
    Box::new(move |factory| {
        if outputs.get_priority(&factory).is_some() {
            factory.log(Log { text: local_fmt!("{}: on", name), color: 10 });
            return on;
        }
        off
    })
}

pub struct RedstoneEmitterConfig {
    pub accesses: Vec<RedstoneAccess>,
    pub output: RedstoneFn,
}

impl Process for RedstoneEmitterConfig {
    fn run(&self, factory: &Factory) -> ChildTask<Result<(), LocalStr>> {
        let value = (self.output)(factory);
        let server = factory.get_server().borrow();
        let access = server.load_balance(&self.accesses);
        let action = ActionFuture::from(RedstoneOutput {
            side: access.side.clone(),
            addr: access.addr.clone(),
            bit: access.bit,
            value,
        });
        server.enqueue_request_group(&access.client, vec![action.clone().into()]);
        spawn(async move { action.await.map(|_| ()) })
    }
}

pub struct RedstoneConditionalConfig<T: IntoProcess> {
    pub name: Option<LocalStr>,
    pub accesses: Vec<RedstoneAccess>,
    pub condition: Box<dyn Fn(u8) -> bool>,
    pub child: T,
}

pub struct RedstoneConditionalProcess<T: Process> {
    weak: Weak<RefCell<RedstoneConditionalProcess<T>>>,
    name: Option<LocalStr>,
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
    fn run(&self, factory: &Factory) -> ChildTask<Result<(), LocalStr>> {
        let server = factory.get_server().borrow();
        let access = server.load_balance(&self.accesses);
        let action =
            ActionFuture::from(RedstoneInput { side: access.side.clone(), addr: access.addr.clone(), bit: access.bit });
        server.enqueue_request_group(&access.client, vec![action.clone().into()]);
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
                    if let Some(name) = &this.name {
                        factory.log(Log { text: local_fmt!("{}: skipped", name), color: 10 })
                    }
                    return Ok(());
                }
            };
            task.await.unwrap()
        })
    }
}
