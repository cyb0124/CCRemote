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

    pub async fn is_action_done(&self) -> bool { self.call_result(vec!["isActionDone".into()]).await.unwrap() }
    pub async fn get_pressure(&self) -> f64 { self.call_result(vec!["getDronePressure".into()]).await.unwrap() }
    pub async fn abort_action(&self) { self.call_void(vec!["abortAction".into()]).await.unwrap() }
    pub async fn clear_area(&self) { self.call_void(vec!["clearArea".into()]).await.unwrap() }
    pub async fn clear_whitelist_text(&self) { self.call_void(vec!["clearWhitelistText".into()]).await.unwrap() }

    pub async fn wait_for_done(&self) {
        while !self.is_action_done().await {
            self.sync(|_| ()).await.unwrap()
        }
    }

    pub async fn set_action(&self, action: LocalStr) {
        self.call_void(vec!["setAction".into(), action.into()]).await.unwrap()
    }

    pub async fn set_side(&self, side: LocalStr, enabled: bool) {
        self.call_void(vec!["setSide".into(), side.into(), enabled.into()]).await.unwrap()
    }

    pub async fn add_point(&self, x: i32, y: i32, z: i32) {
        self.call_void(vec!["addArea".into(), x.into(), y.into(), z.into()]).await.unwrap()
    }

    pub async fn add_area(&self, x1: i32, y1: i32, z1: i32, x2: i32, y2: i32, z2: i32) {
        self.call_void(vec![
            "addArea".into(),
            x1.into(),
            y1.into(),
            z1.into(),
            x2.into(),
            y2.into(),
            z2.into(),
            "filled".into(),
        ])
        .await
        .unwrap()
    }

    pub async fn add_whitelist_text(&self, text: LocalStr) {
        self.call_void(vec!["addWhitelistText".into(), text.into()]).await.unwrap()
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
    sync_queue: Vec<Box<dyn FnOnce(&Factory)>>,
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
                sync_queue: Vec::new(),
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
            for sync_task in std::mem::take(&mut this.sync_queue) {
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
        self.sync_queue.push(Box::new(move |factory| sender.send(Ok(f(factory)))));
        spawn(async move {
            if let Ok(x) = receiver.await {
                x
            } else {
                pending().await
            }
        })
    }
}
