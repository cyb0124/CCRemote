use super::action::ActionFuture;
use super::action::TurtleCall;
use super::lua_value::Value;
use super::server::Server;
use fnv::FnvHashSet;
use ordered_float::NotNan;
use std::cell::RefCell;
use std::rc::Rc;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};

fn run_command(funcs: &mut FnvHashSet<&'static str>, client: &str, args: &[&str], server: &Server) {
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
                run_command(funcs, client, args, server)
            }
        }
    } else {
        let func = funcs.get(args[0]).map(|x| *x);
        let func = func.unwrap_or_else(|| {
            let leaked = Box::leak(args[0].to_owned().into_boxed_str());
            funcs.insert(leaked);
            leaked
        });
        let action = ActionFuture::from(TurtleCall {
            func,
            args: args
                .iter()
                .skip(1)
                .map(|arg| if let Ok(arg) = arg.parse::<NotNan<f64>>() { Value::F(arg) } else { arg.to_owned().into() })
                .collect(),
        });
        server.enqueue_request_group(client, vec![action.clone().into()]);
        tokio::task::spawn_local(async move { println!("{:?}", action.await) });
    }
}

pub async fn run(server: Rc<RefCell<Server>>) {
    let mut funcs = FnvHashSet::<&'static str>::default();
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
            run_command(&mut funcs, client, &args, &*server.borrow())
        } else {
            println!("set client first")
        }
    }
}
