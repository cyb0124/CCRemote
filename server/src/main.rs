#![feature(arc_new_cyclic)]

#[macro_use]
pub mod util;
pub mod access;
pub mod action;
pub mod detail_cache;
pub mod item;
pub mod lua_value;
pub mod server;

use server::Server;
use tokio::{signal::ctrl_c, task::LocalSet};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let tasks = LocalSet::new();
    tasks.spawn_local(async {
        let _server = Server::new(1847);
        ctrl_c().await.unwrap()
    });
    tasks.await
}
