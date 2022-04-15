use super::action::ActionFuture;
use super::action::{Call, TurtleCall};
use super::lua_value::Value;
use super::server::Server;
use super::util::{spawn, AbortOnDrop};
use fnv::FnvHashSet;
use ordered_float::NotNan;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};

const QUEUE_SIZE: usize = 8;
struct Context {
    server: Rc<RefCell<Server>>,
    leaks: FnvHashSet<&'static str>,
    queue: VecDeque<AbortOnDrop<()>>,
}

fn run_command(ctx: Rc<RefCell<Context>>, client: String, args: Vec<String>) -> AbortOnDrop<()> {
    spawn(async move {
        if let Ok(n) = args[0].parse::<usize>() {
            if args.len() < 2 {
                println!("expect delimiter");
                return;
            }
            let commands: Vec<_> = args[2..].split(|arg| *arg == args[1]).collect();
            for args in &commands {
                if args.is_empty() {
                    println!("expect args");
                    return;
                }
            }
            for _ in 0..n {
                for args in &commands {
                    run_command(ctx.clone(), client.clone(), args.to_vec()).into_future().await
                }
            }
        } else {
            let mut ctx = ctx.borrow_mut();
            let first = &args[0];
            let is_call = first.starts_with('-');
            let first = if is_call { &first[1..] } else { first as &str };
            let first = ctx.leaks.get(first).map(|x| *x).unwrap_or_else(|| {
                let leaked = Box::leak(first.to_owned().into_boxed_str());
                ctx.leaks.insert(leaked);
                leaked
            });
            let args = args
                .iter()
                .skip(1)
                .map(|arg| if let Ok(arg) = arg.parse::<NotNan<f64>>() { Value::F(arg) } else { arg.to_owned().into() })
                .collect();
            let task = if is_call {
                let action = ActionFuture::from(Call { addr: first, args });
                ctx.server.borrow().enqueue_request_group(&client, vec![action.clone().into()]);
                spawn(async move { println!("{:?}", action.await) })
            } else {
                let action = ActionFuture::from(TurtleCall { func: first, args });
                ctx.server.borrow().enqueue_request_group(&client, vec![action.clone().into()]);
                spawn(async move { println!("{:?}", action.await) })
            };
            if ctx.queue.len() == QUEUE_SIZE {
                ctx.queue.pop_front().unwrap().into_future().await
            }
            ctx.queue.push_back(task)
        }
    })
}

pub async fn run(server: Rc<RefCell<Server>>) {
    let ctx = Rc::new(RefCell::new(Context { server, leaks: FnvHashSet::default(), queue: VecDeque::new() }));
    let mut stdin = BufReader::new(stdin());
    let mut client = None;
    loop {
        let mut line = String::new();
        if stdin.read_line(&mut line).await.unwrap() == 0 {
            break;
        }
        if line.ends_with('\n') {
            line.pop();
        }
        let args: Vec<_> = line.split_whitespace().collect();
        if args.is_empty() || args[0].is_empty() {
            println!("expect args")
        } else if args[0] == "c" {
            if args.len() != 2 {
                println!("expect 2 args")
            } else {
                client = Some(args[1].to_owned());
                println!("client set")
            }
        } else if let Some(client) = client.as_ref() {
            run_command(ctx.clone(), client.clone(), args.iter().map(|x| (*x).to_owned()).collect()).into_future().await
        } else {
            println!("set client first")
        }
    }
}
