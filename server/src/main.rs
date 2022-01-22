#![feature(arc_new_cyclic)]

#[macro_use]
pub mod util;
#[macro_use]
pub mod inventory;
#[macro_use]
pub mod recipe;
pub mod access;
pub mod action;
pub mod config;
pub mod detail_cache;
pub mod factory;
pub mod item;
pub mod lua_value;
pub mod process;
pub mod server;
pub mod storage;
pub mod turtle_rc;

use config::build_factory;
use tokio::{signal::ctrl_c, task::LocalSet};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let tasks = LocalSet::new();
    tasks.spawn_local(async {
        let _factory = build_factory();
        ctrl_c().await.unwrap()
        // To run turtle_rc, replace above with:
        // turtle_rc::run(server::Server::new(1848)).await
    });
    tasks.await
}
