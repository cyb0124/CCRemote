use crate::factory::{Factory, FactoryConfig};
use crate::{access::*, config_util::*, process::*, recipe::*, storage::*};
use crate::{detail_cache::DetailCache, server::Server, Tui};
use std::{cell::RefCell, rc::Rc, time::Duration};

pub fn build_factory(tui: Rc<Tui>) -> Rc<RefCell<Factory>> {
    FactoryConfig {
        tui: tui.clone(),
        detail_cache: DetailCache::new(&tui, s("detail_cache.txt")),
        server: Server::new(tui, 1847),
        min_cycle_time: Duration::from_secs(1),
        log_clients: vec![s("1a")],
        bus_accesses: vec![BasicAccess { client: s("1a"), addr: s("enderstorage:ender_chest_1") }],
        fluid_bus_accesses: vec![],
        fluid_bus_capacity: 0,
        backups: vec![],
        fluid_backups: vec![],
    }
    .build(|factory| {
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: s("1a"),
                inv_addr: s("ironchest:obsidian_chest_1"),
                bus_addr: s("enderstorage:ender_chest_1"),
            }],
            override_max_stack_size: None,
        });
        factory.add_process(ManualUiConfig { accesses: vec![] });
    })
}
