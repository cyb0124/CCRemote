use super::detail_cache::DetailCache;
use super::factory::{Factory, FactoryConfig};
use super::server::Server;
use super::{access::*, config_util::*, process::*, recipe::*, storage::*};
use std::{cell::RefCell, rc::Rc, time::Duration};

pub fn build_factory() -> Rc<RefCell<Factory>> {
    FactoryConfig {
        detail_cache: DetailCache::new(Some(s("detail_cache.txt"))),
        server: Server::new(1847),
        min_cycle_time: Duration::from_secs(1),
        log_clients: vec![s("1a")],
        bus_accesses: vec![BasicAccess { client: s("1a"), addr: s("enderstorage:ender_chest_1") }],
        backups: vec![],
    }
    .build(|factory| {
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: s("1a"),
                inv_addr: s("ironchest:obsidian_chest_1"),
                bus_addr: s("enderstorage:ender_chest_1"),
            }],
            override_max_stack_size: None,
        })
    })
}
