#![feature(arc_new_cyclic)]

#[macro_use]
pub mod util;
pub mod access;
pub mod action;
pub mod config;
pub mod detail_cache;
pub mod factory;
pub mod inventory;
pub mod item;
pub mod lua_value;
pub mod process;
pub mod recipe;
pub mod server;
pub mod storage;

use config::build_factory;
use tokio::{signal::ctrl_c, task::LocalSet};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let tasks = LocalSet::new();
    tasks.spawn_local(async {
        let _factory = build_factory();
        ctrl_c().await.unwrap()
    });
    tasks.await
}
