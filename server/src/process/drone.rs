use super::super::access::BasicAccess;
use super::super::action::Log;
use super::super::action::{ActionFuture, Call};
use super::super::factory::Factory;
use super::super::inventory::Inventory;
use super::super::lua_value::{call_result, Value};
use super::super::util::{alive, make_local_one_shot, spawn};
use super::{IntoProcess, Process};
use abort_on_drop::ChildTask;
use flexstr::{local_fmt, LocalStr};
use futures_util::future::pending;
use serde::{Deserialize, Serialize};
use std::io::{BufReader, BufWriter};
use std::rc::{Rc, Weak};
use std::{cell::RefCell, convert::TryFrom, fs::File, future::Future, marker::PhantomData};

pub struct DroneContext<State: Serialize> {
    _phantom: PhantomData<dyn Fn(State) -> ()>,
    weak: Weak<RefCell<DroneProcess>>,
    file_name: LocalStr,
}

impl<State: Serialize> DroneContext<State> {
    pub fn log(&self, args: std::fmt::Arguments, color: u8) {
        if let Some(this) = self.weak.upgrade() {
            this.borrow().log(args, color);
        }
    }

    pub fn save(&self, state: &State) {
        if let Some(this) = self.weak.upgrade() {
            let result = File::create(&*self.file_name)
                .map_err(|e| local_fmt!("{}", e))
                .and_then(|x| serde_json::to_writer(BufWriter::new(x), state).map_err(|e| local_fmt!("{}", e)));
            if let Err(e) = result {
                this.borrow().log(format_args!("{}", e), 14);
            }
        }
    }

    pub fn sync<T: 'static>(&self, f: impl FnOnce(&Factory) -> T + 'static) -> ChildTask<T> {
        if let Some(this) = self.weak.upgrade() {
            this.borrow_mut().sync(f)
        } else {
            spawn(pending())
        }
    }

    pub fn call_raw<T: 'static>(
        &self,
        args: Vec<Value>,
        parse: impl Fn(Value) -> Result<T, LocalStr> + 'static,
    ) -> ChildTask<T> {
        let weak = self.weak.clone();
        spawn(async move {
            loop {
                let task = if let Some(this) = weak.upgrade() {
                    let this = this.borrow();
                    upgrade!(this.factory, factory);
                    let server = factory.get_server().borrow();
                    let access = server.load_balance(&this.accesses);
                    let action = ActionFuture::from(Call { addr: access.addr.clone(), args: args.clone() });
                    server.enqueue_request_group(&access.client, vec![action.clone().into()]);
                    action
                } else {
                    pending().await
                };
                match task.await.and_then(&parse) {
                    Ok(x) => return x,
                    Err(e) => {
                        if let Some(this) = weak.upgrade() {
                            let task = {
                                let mut this = this.borrow_mut();
                                this.log(format_args!("{}", e), 14);
                                this.sync(|_| ())
                            };
                            task.await.unwrap()
                        }
                    }
                }
            }
        })
    }

    pub fn call_void(&self, args: Vec<Value>) -> ChildTask<()> { self.call_raw(args, |_| Ok(())) }

    pub fn call_result<T: TryFrom<Value, Error = LocalStr> + 'static>(&self, args: Vec<Value>) -> ChildTask<T> {
        self.call_raw(args, call_result)
    }

    pub async fn wait_for_done(&self) {
        while !self.call_result::<bool>(vec!["isActionDone".into()]).await.unwrap() {
            self.sync(|_| ()).await.unwrap()
        }
    }
}

pub struct DroneConfig<State: Serialize + for<'a> Deserialize<'a>, Task: Future<Output = ()> + 'static> {
    pub name: LocalStr,
    pub file_name: LocalStr,
    pub accesses: Vec<BasicAccess>,
    pub program: Box<dyn FnOnce(DroneContext<State>, Option<State>) -> Task>,
}

pub struct DroneProcess {
    weak: Weak<RefCell<DroneProcess>>,
    factory: Weak<RefCell<Factory>>,
    name: LocalStr,
    accesses: Vec<BasicAccess>,
    sync_task: Option<Box<dyn FnOnce(&Factory)>>,
    _task: ChildTask<()>,
}

impl<State: Serialize + for<'a> Deserialize<'a>, Task: Future<Output = ()> + 'static> IntoProcess
    for DroneConfig<State, Task>
{
    type Output = DroneProcess;
    fn into_process(self, factory: &Factory) -> Rc<RefCell<Self::Output>> {
        Rc::new_cyclic(|weak| {
            let state = File::open(&*self.file_name).ok().and_then(|x| serde_json::from_reader(BufReader::new(x)).ok());
            let context =
                DroneContext { _phantom: PhantomData::default(), weak: weak.clone(), file_name: self.file_name };
            RefCell::new(DroneProcess {
                weak: weak.clone(),
                factory: factory.get_weak().clone(),
                name: self.name,
                accesses: self.accesses,
                sync_task: None,
                _task: spawn((self.program)(context, state)),
            })
        })
    }
}

impl Process for DroneProcess {
    fn run(&self, _: &Factory) -> ChildTask<Result<(), LocalStr>> {
        let weak = self.weak.clone();
        spawn(async move {
            alive_mut!(weak, this);
            upgrade!(this.factory, factory);
            for sync_task in this.sync_task.take() {
                sync_task(factory)
            }
            Ok(())
        })
    }
}

impl DroneProcess {
    fn log(&self, args: std::fmt::Arguments, color: u8) {
        upgrade!(self.factory, factory);
        factory.log(Log { text: local_fmt!("{}: {}", self.name, args), color })
    }

    fn sync<T: 'static>(&mut self, f: impl FnOnce(&Factory) -> T + 'static) -> ChildTask<T> {
        let (sender, receiver) = make_local_one_shot();
        self.sync_task = Some(Box::new(move |factory| sender.send(Ok(f(factory)))));
        spawn(async move {
            if let Ok(x) = receiver.await {
                x
            } else {
                pending().await
            }
        })
    }
}
