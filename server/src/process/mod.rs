use super::access::BusAccess;
use super::action::{ActionFuture, Call};
use super::factory::{Factory, Reservation};
use super::inventory::Inventory;
use super::item::DetailStack;
use super::util::{alive, join_tasks, spawn, AbortOnDrop};
use std::{cell::RefCell, iter::once, rc::Rc};

pub trait Process: 'static {
    fn run(&self, factory: &Factory) -> AbortOnDrop<Result<(), String>>;
}

pub trait IntoProcess {
    type Output: Process;
    fn into_process(self, factory: &Factory) -> Rc<RefCell<Self::Output>>;
}

macro_rules! impl_into_process {
    ($c:ident, $p:ident) => {
        impl IntoProcess for $c {
            type Output = $p;
            fn into_process(self, factory: &Factory) -> Rc<RefCell<Self::Output>> {
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
    };
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

fn scattering_insert<T, U>(
    this: &T,
    factory: &mut Factory,
    reservation: Reservation,
    insertions: U,
) -> AbortOnDrop<Result<(), String>>
where
    T: Inventory<Access = BusAccess>,
    U: IntoIterator<Item = (usize, i32)> + 'static,
{
    let bus_slot = factory.bus_allocate();
    let weak = this.get_weak().clone();
    let factory = factory.get_weak().clone();
    spawn(async move {
        let bus_slot = bus_slot.await?;
        let task = async {
            reservation.extract(bus_slot).await?;
            let mut tasks = Vec::new();
            {
                alive!(weak, this);
                let server = this.get_server().borrow();
                for (inv_slot, size) in insertions.into_iter() {
                    let access = server.load_balance(this.get_accesses());
                    let action = ActionFuture::from(Call {
                        addr: access.inv_addr,
                        args: vec![
                            "pullItems".into(),
                            access.bus_addr.into(),
                            (bus_slot + 1).into(),
                            size.into(),
                            (inv_slot + 1).into(),
                        ],
                    });
                    server.enqueue_request_group(access.client, vec![action.clone().into()]);
                    tasks.push(spawn(async move { action.await.map(|_| ()) }))
                }
            }
            join_tasks(tasks).await?;
            alive(&factory)?.borrow_mut().bus_free(bus_slot);
            Ok(())
        };
        let result = task.await;
        if result.is_err() {
            alive(&factory)?.borrow_mut().bus_deposit(once(bus_slot));
        }
        result
    })
}

mod buffered;
mod inputless;
mod scattering;
mod slotted;
pub use buffered::*;
pub use inputless::*;
pub use scattering::*;
pub use slotted::*;
