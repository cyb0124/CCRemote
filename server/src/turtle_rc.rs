use crate::action::{ActionFuture, Call, TurtleCall};
use crate::{lua_value::Value, server::Server, util::spawn, Tui};
use abort_on_drop::ChildTask;
use flexstr::LocalStr;
use ordered_float::NotNan;
use std::{cell::RefCell, collections::VecDeque, future::Future, mem::take, pin::Pin, rc::Rc};

const QUEUE_SIZE: usize = 8;
struct Context {
    tui: Rc<Tui>,
    server: Rc<RefCell<Server>>,
    queue: RefCell<VecDeque<ChildTask<()>>>,
}

impl Context {
    fn run_command(self: Rc<Self>, client: LocalStr, args: Vec<LocalStr>) -> ChildTask<()> {
        spawn(async move {
            if let Ok(n) = args[0].parse::<usize>() {
                if args.len() < 2 {
                    self.tui.log("expect delimiter".to_owned(), 0);
                    return;
                }
                let commands: Vec<_> = args[2..].split(|arg| *arg == args[1]).collect();
                for args in &commands {
                    if args.is_empty() {
                        self.tui.log("expect args".to_owned(), 0);
                        return;
                    }
                }
                for _ in 0..n {
                    for args in &commands {
                        self.clone().run_command(client.clone(), args.to_vec()).await.unwrap()
                    }
                }
            } else {
                let mut args = args.into_iter();
                let mut first = args.next().unwrap();
                let is_call = first.starts_with('-');
                if is_call {
                    first = (&first[1..]).into()
                }
                let args = args
                    .map(|x| {
                        if let Ok(x) = x.parse::<NotNan<f64>>() {
                            Value::F(x)
                        } else if x == "true" {
                            Value::B(true)
                        } else if x == "false" {
                            Value::B(false)
                        } else {
                            x.into()
                        }
                    })
                    .collect();
                let task: Pin<Box<dyn Future<Output = String>>> = if is_call {
                    let action = ActionFuture::from(Call { addr: first, args });
                    self.server.borrow().enqueue_request_group(&client, vec![action.clone().into()]);
                    Box::pin(async move { format!("{:?}", action.await) })
                } else {
                    let action = ActionFuture::from(TurtleCall { func: first, args });
                    self.server.borrow().enqueue_request_group(&client, vec![action.clone().into()]);
                    Box::pin(async move { format!("{:?}", action.await) })
                };
                let mut queue = self.queue.borrow_mut();
                if queue.len() == QUEUE_SIZE {
                    queue.pop_front().unwrap().await.unwrap()
                }
                let tui = self.tui.clone();
                queue.push_back(spawn(async move { tui.log(task.await, 0) }))
            }
        })
    }
}

pub fn run(server: Rc<RefCell<Server>>) -> ChildTask<()> {
    spawn(async move {
        let tui = server.borrow().tui.clone();
        let ctx = Rc::new(Context { tui: tui.clone(), server, queue: <_>::default() });
        let mut client = None;
        loop {
            tui.on_input.notified().await;
            let lines = take(&mut *tui.input_queue.borrow_mut());
            for line in lines {
                let args: Vec<_> = line.split_whitespace().collect();
                if args.is_empty() || args[0].is_empty() {
                    tui.log("expect args".to_owned(), 0)
                } else if args[0] == "c" {
                    if args.len() != 2 {
                        tui.log("expect 2 args".to_owned(), 0)
                    } else {
                        client = Some(LocalStr::from(args[1]));
                        tui.log("client set".to_owned(), 0)
                    }
                } else if let Some(client) = client.as_ref() {
                    ctx.clone().run_command(client.clone(), args.iter().map(|x| (*x).into()).collect()).await.unwrap()
                } else {
                    tui.log("set client first".to_owned(), 0)
                }
            }
        }
    })
}
