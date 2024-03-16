use super::super::factory::Factory;
use super::super::inventory::Inventory;
use super::super::util::{alive, join_tasks, spawn};
use super::{IntoProcess, Process};
use crate::access::TankAccess;
use crate::action::{ActionFuture, Call};
use crate::recipe::FluidOutput;
use abort_on_drop::ChildTask;
use flexstr::LocalStr;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct BlockingFluidOutputConfig {
    pub accesses: Vec<TankAccess>,
    pub outputs: Vec<FluidOutput>,
}

pub struct BlockingFluidOutputProcess {
    weak: Weak<RefCell<BlockingFluidOutputProcess>>,
    config: BlockingFluidOutputConfig,
    factory: Weak<RefCell<Factory>>,
}

impl IntoProcess for BlockingFluidOutputConfig {
    type Output = BlockingFluidOutputProcess;
    fn into_process(self, factory: &Factory) -> Rc<RefCell<Self::Output>> {
        Rc::new_cyclic(|weak| {
            RefCell::new(BlockingFluidOutputProcess {
                weak: weak.clone(),
                config: self,
                factory: factory.get_weak().clone(),
            })
        })
    }
}

impl Process for BlockingFluidOutputProcess {
    fn run(&self, factory: &Factory) -> ChildTask<Result<(), LocalStr>> {
        let mut tasks = Vec::new();
        for output in &self.config.outputs {
            let n_stored = factory.search_n_fluid(&output.fluid);
            let qty = output.n_wanted - n_stored;
            let fluid = output.fluid.clone();
            if qty > 0 {
                let weak = self.weak.clone();
                tasks.push(spawn(async move {
                    let bus = {
                        alive!(weak, this);
                        upgrade_mut!(this.factory, factory);
                        factory.fluid_bus_allocate()
                    };
                    let bus = bus.await?;
                    let task;
                    {
                        alive!(weak, this);
                        upgrade!(this.factory, factory);
                        let server = factory.get_server().borrow();
                        let access = server.load_balance(&this.config.accesses);
                        task = ActionFuture::from(Call {
                            addr: access.fluid_bus_addrs[bus].clone(),
                            args: vec!["pullFluid".into(), access.tank_addr.clone().into(), qty.into(), fluid.into()],
                        });
                        server.enqueue_request_group(&access.client, vec![task.clone().into()])
                    }
                    let result = task.await.map(|_| ());
                    alive(&weak)?.borrow().factory.upgrade().unwrap().borrow_mut().fluid_bus_deposit([bus]);
                    result
                }));
            }
        }
        spawn(join_tasks(tasks))
    }
}
