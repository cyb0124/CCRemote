use super::access::BusAccess;
use super::action::{ActionFuture, Call};
use super::factory::Factory;
use super::inventory::Inventory;
use super::item::DetailStack;
use super::util::{alive, spawn, AbortOnDrop};
use std::{cell::RefCell, iter::once, rc::Rc};

pub trait Process: 'static {
    fn run(&self, factory: &Factory) -> AbortOnDrop<Result<(), String>>;
}

pub trait IntoProcess {
    type Output: Process;
    fn into_process(self, factory: &Factory) -> Rc<RefCell<Self::Output>>;
}

pub type SlotFilter = Box<dyn Fn(usize) -> bool>;
pub type ExtractFilter = Box<dyn Fn(usize, &DetailStack) -> bool>;
pub fn extract_all() -> Option<ExtractFilter> { Some(Box::new(|_, _| true)) }

fn extract_output<T>(this: &T, factory: &mut Factory, slot: usize, size: i32) -> AbortOnDrop<Result<(), String>>
where
    T: Inventory<Access = BusAccess>,
{
    let bus_slot = factory.bus_allocate();
    let weak = this.get_weak().clone();
    let factory = factory.get_weak().clone();
    spawn(async move {
        let bus_slot = bus_slot.await?;
        let action;
        {
            alive!(weak, this);
            let server = this.get_server().borrow();
            let access = server.load_balance(this.get_accesses());
            action = ActionFuture::from(Call {
                addr: access.inv_addr,
                args: vec![
                    "pushItems".into(),
                    access.bus_addr.into(),
                    (slot + 1).into(),
                    size.into(),
                    (bus_slot + 1).into(),
                ],
            });
            server.enqueue_request_group(access.client, vec![action.clone().into()])
        }
        let result = action.await.map(|_| ());
        alive(&factory)?.borrow_mut().bus_deposit(once(bus_slot));
        result
    })
}

mod inputless;
mod slotted;
pub use inputless::*;
pub use slotted::*;
