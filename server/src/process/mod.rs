use super::access::BusAccess;
use super::action::{ActionFuture, Call};
use super::factory::{Factory, Reservation};
use super::inventory::Inventory;
use super::item::DetailStack;
use super::util::{alive, join_tasks, spawn};
use abort_on_drop::ChildTask;
use flexstr::LocalStr;
use std::{cell::RefCell, iter::once, rc::Rc};

pub trait Process: 'static {
    fn run(&self, factory: &Factory) -> ChildTask<Result<(), LocalStr>>;
}

pub trait IntoProcess {
    type Output: Process;
    fn into_process(self, factory: &Factory) -> Rc<RefCell<Self::Output>>;
}

impl<T: Process> IntoProcess for T {
    type Output = T;
    fn into_process(self, _: &Factory) -> Rc<RefCell<Self::Output>> { Rc::new(RefCell::new(self)) }
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
pub type ExtractFilter = Box<dyn Fn(&Factory, usize, &DetailStack) -> bool>;
pub fn extract_all() -> Option<ExtractFilter> { Some(Box::new(|_, _, _| true)) }

fn extract_output<T>(this: &T, factory: &mut Factory, slot: usize, size: i32) -> ChildTask<Result<(), LocalStr>>
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
                addr: access.bus_addr.clone(),
                args: vec![
                    "pullItems".into(),
                    access.inv_addr.clone().into(),
                    (slot + 1).into(),
                    size.into(),
                    (bus_slot + 1).into(),
                ],
            });
            server.enqueue_request_group(&access.client, vec![action.clone().into()])
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
) -> ChildTask<Result<(), LocalStr>>
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
                        addr: access.bus_addr.clone(),
                        args: vec![
                            "pushItems".into(),
                            access.inv_addr.clone().into(),
                            (bus_slot + 1).into(),
                            size.into(),
                            (inv_slot + 1).into(),
                        ],
                    });
                    server.enqueue_request_group(&access.client, vec![action.clone().into()]);
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

mod blocking_fluid_output;
mod blocking_output;
mod buffered;
mod crafty;
mod drone;
mod fluid_slotted;
mod manual_ui;
mod misc;
mod multi_inv_slotted;
mod redstone;
mod scattering;
mod slotted;
mod turtle;
mod workbench;
pub use blocking_fluid_output::*;
pub use blocking_output::*;
pub use buffered::*;
pub use crafty::*;
pub use drone::*;
pub use fluid_slotted::*;
pub use manual_ui::*;
pub use misc::*;
pub use multi_inv_slotted::*;
pub use redstone::*;
pub use scattering::*;
pub use slotted::*;
pub use turtle::*;
pub use workbench::*;
