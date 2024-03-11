use super::super::action::Log;
use super::super::action::{ActionFuture, TurtleCall};
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
use std::{cell::RefCell, fs::File, future::Future, marker::PhantomData};

pub struct TurtleContext<State: Serialize> {
    _phantom: PhantomData<dyn Fn(State) -> ()>,
    weak: Weak<RefCell<TurtleProcess>>,
    file_name: LocalStr,
}

impl<State: Serialize> TurtleContext<State> {
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

    pub fn log(&self, args: std::fmt::Arguments, color: u8) {
        if let Some(this) = self.weak.upgrade() {
            this.borrow().log(args, color);
        }
    }

    pub fn sync<T: 'static>(&self, f: impl FnOnce(&Factory) -> T + 'static) -> ChildTask<T> {
        if let Some(this) = self.weak.upgrade() {
            spawn(this.borrow_mut().sync(f))
        } else {
            spawn(pending())
        }
    }

    pub fn call_raw(&self, func: LocalStr, args: Vec<Value>) -> ChildTask<Result<Value, LocalStr>> {
        if let Some(this) = self.weak.upgrade() {
            spawn(this.borrow().call_raw(func, args))
        } else {
            spawn(pending())
        }
    }

    pub fn call_retry<T: 'static>(
        &self,
        func: LocalStr,
        args: Vec<Value>,
        parse: impl Fn(Result<Value, LocalStr>) -> Result<T, LocalStr> + 'static,
    ) -> ChildTask<T> {
        let weak = self.weak.clone();
        spawn(async move {
            loop {
                let task = if let Some(this) = weak.upgrade() {
                    this.borrow().call_raw(func.clone(), args.clone())
                } else {
                    pending().await
                };
                match parse(task.await) {
                    Ok(x) => return x,
                    Err(e) => {
                        if let Some(this) = weak.upgrade() {
                            let task = {
                                let mut this = this.borrow_mut();
                                this.log(format_args!("{}", e), 14);
                                this.sync(|_| ())
                            };
                            task.await
                        }
                    }
                }
            }
        })
    }

    pub fn call_void(&self, func: LocalStr, args: Vec<Value>) -> ChildTask<()> {
        self.call_retry(func, args, |x| x.map(|_| ()))
    }

    pub fn call_result<T: TryFrom<Value, Error = LocalStr> + 'static>(
        &self,
        func: LocalStr,
        args: Vec<Value>,
    ) -> ChildTask<T> {
        self.call_retry(func, args, |x| x.and_then(call_result))
    }
}

pub struct TurtleConfig<State: Serialize + for<'a> Deserialize<'a>, Task: Future<Output = ()> + 'static> {
    pub name: LocalStr,
    pub file_name: LocalStr,
    pub client: LocalStr,
    pub program: Box<dyn FnOnce(TurtleContext<State>, Option<State>) -> Task>,
}

pub struct TurtleProcess {
    weak: Weak<RefCell<TurtleProcess>>,
    factory: Weak<RefCell<Factory>>,
    name: LocalStr,
    client: LocalStr,
    sync_queue: Vec<Box<dyn FnOnce(&Factory)>>,
    _task: ChildTask<()>,
}

impl<State: Serialize + for<'a> Deserialize<'a>, Task: Future<Output = ()> + 'static> IntoProcess
    for TurtleConfig<State, Task>
{
    type Output = TurtleProcess;
    fn into_process(self, factory: &Factory) -> Rc<RefCell<Self::Output>> {
        Rc::new_cyclic(|weak| {
            let state = File::open(&*self.file_name).ok().and_then(|x| serde_json::from_reader(BufReader::new(x)).ok());
            let context =
                TurtleContext { _phantom: PhantomData::default(), weak: weak.clone(), file_name: self.file_name };
            RefCell::new(TurtleProcess {
                weak: weak.clone(),
                factory: factory.get_weak().clone(),
                name: self.name,
                client: self.client,
                sync_queue: Vec::new(),
                _task: spawn((self.program)(context, state)),
            })
        })
    }
}

impl Process for TurtleProcess {
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

impl TurtleProcess {
    fn log(&self, args: std::fmt::Arguments, color: u8) {
        upgrade!(self.factory, factory);
        factory.log(Log { text: local_fmt!("{}: {}", self.name, args), color })
    }

    fn sync<T: 'static>(&mut self, f: impl FnOnce(&Factory) -> T + 'static) -> impl Future<Output = T> {
        let (sender, receiver) = make_local_one_shot();
        self.sync_queue.push(Box::new(move |factory| sender.send(Ok(f(factory)))));
        async move {
            if let Ok(x) = receiver.await {
                x
            } else {
                pending().await
            }
        }
    }

    fn call_raw(&self, func: LocalStr, args: Vec<Value>) -> ActionFuture<TurtleCall> {
        upgrade!(self.factory, factory);
        let server = factory.get_server().borrow();
        let action = ActionFuture::from(TurtleCall { func: func.clone(), args: args.clone() });
        server.enqueue_request_group(&self.client, vec![action.clone().into()]);
        action
    }
}
