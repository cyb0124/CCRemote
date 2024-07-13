use crate::factory::{Factory, FactoryConfig};
use crate::{access::*, config_util::*, process::*, recipe::*, storage::*};
use crate::{detail_cache::DetailCache, server::Server, Tui};
use std::{cell::RefCell, rc::Rc, time::Duration};

pub fn build_factory(tui: Rc<Tui>) -> Rc<RefCell<Factory>> {
    const BUS: &str = "minecraft:barrel_1";
    let acc = |inv_addr| vec![BusAccess { client: s("1a"), inv_addr, bus_addr: s(BUS) }];
    FactoryConfig {
        tui: tui.clone(),
        detail_cache: DetailCache::new(&tui, s("detail_cache.txt")),
        server: Server::new(tui, 1853),
        min_cycle_time: Duration::from_secs(1),
        log_clients: vec![s("1a")],
        bus_accesses: vec![BasicAccess { client: s("1a"), addr: s(BUS) }],
        fluid_bus_accesses: vec![],
        fluid_bus_capacity: 0,
        backups: vec![],
        fluid_backups: vec![],
    }
    .build(|factory| {
        for inv_addr in ["quark:variant_chest_3"] {
            factory.add_storage(ChestConfig { accesses: acc(s(inv_addr)), override_max_stack_size: None })
        }
        factory.add_process(ManualUiConfig { accesses: vec![] });
        for inv_addr in ["create:depot_0"] {
            factory.add_process(BufferedConfig {
                name: s("lavaFan"),
                accesses: acc(s(inv_addr)),
                slot_filter: todo!(),
                to_extract: todo!(),
                recipes: todo!(),
                max_recipe_inputs: todo!(),
                stocks: todo!(),
            })
        }
        for (inv_addr, item) in [("projectexpansion:emc_link_1", "Raw Copper")] {
            factory.add_process(BlockingOutputConfig {
                accesses: acc(s(inv_addr)),
                slot_filter: None,
                outputs: vec![Output { item: label(item), n_wanted: 64 }],
            })
        }
    })
}
