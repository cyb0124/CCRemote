use super::detail_cache::DetailCache;
use super::factory::{Factory, FactoryConfig};
use super::server::Server;
use super::{access::*, item::*, process::*, recipe::*, storage::*};
use std::{cell::RefCell, rc::Rc, time::Duration};

pub fn build_factory() -> Rc<RefCell<Factory>> {
    FactoryConfig {
        detail_cache: DetailCache::new(),
        server: Server::new(1847),
        min_cycle_time: Duration::from_secs(1),
        log_clients: vec!["1a"],
        bus_accesses: vec![BasicAccess { client: "1a", addr: "quark:quark_chest_7" }],
        backups: vec![(Filter::Label("Potato"), 32)],
    }
    .build(|factory| {
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchest_0",
                bus_addr: "quark:quark_chest_7",
            }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchest_1",
                bus_addr: "quark:quark_chest_7",
            }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchest_2",
                bus_addr: "quark:quark_chest_7",
            }],
        });
        factory.add_process(ScatteringConfig {
            name: "plantSower",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:crop_sower_tile_1",
                bus_addr: "quark:quark_chest_7",
            }],
            input_slots: vec![6, 7, 8, 9, 11, 12, 13, 14],
            to_extract: None,
            recipes: vec![
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Potato"), n_wanted: 64 }],
                    ScatteringInput::new(Filter::Label("Potato")).allow_backup(),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Cotton"), n_wanted: 64 }],
                    ScatteringInput::new(Filter::Label("Cotton")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Seeds"), n_wanted: 64 }],
                    ScatteringInput::new(Filter::Label("Seeds")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Birch Wood"), n_wanted: 64 }],
                    ScatteringInput::new(Filter::Label("Birch Sapling")),
                ),
            ],
            max_per_slot: 4,
        });
        factory.add_process(SlottedConfig {
            name: "plantGatherer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:crop_recolector_tile_0",
                bus_addr: "quark:quark_chest_7",
            }],
            input_slots: vec![],
            to_extract: Some(Box::new(|slot, _| slot >= 6)),
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "bioGenerator",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_3",
                bus_addr: "quark:quark_chest_7",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![],
                inputs: vec![BufferedInput::new(Filter::Label("Bio Fuel"), 2)],
                max_inputs: 8,
            }],
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "crusher",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_1",
                bus_addr: "quark:quark_chest_7",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 8,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bio Fuel"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Potato"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gravel"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cobblestone"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "furnace",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_2",
                bus_addr: "quark:quark_chest_7",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 8,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Glass"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sand"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Stone"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cobblestone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Charcoal"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Birch Wood"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Graphite Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Charcoal"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Iron"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Copper"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gold Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Gold"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Osmium Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Osmium Dust"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            stocks: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "outputA",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "immersiveengineering:woodencrate_4",
                bus_addr: "quark:quark_chest_7",
            }],
            input_slots: vec![],
            to_extract: extract_all(),
            recipes: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "manufactory",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:manufactory_1",
                bus_addr: "quark:quark_chest_7",
            }],
            input_slots: vec![0],
            to_extract: extract_all(),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sand"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Cobblestone"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Birch Wood Planks"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Birch Wood"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Iron"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Iron Ore"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Aluminum"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Aluminum Ore"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Copper"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Copper Ore"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Gold"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Gold Ore"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Lead"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lead Ore"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Silver"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Silver Ore"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Tin"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Tin Ore"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cobalt Ore Dust"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Cobalt Ore"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ardite Ore Tin"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Ardite Ore"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Osmium Dust"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Osmium Ore"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Uranium Grit"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Uranium Ore"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lithium Dust"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lithium Ore"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Magnesium Dust"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Magnesium Ore"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Boron Dust"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Boron Ore"), 1, vec![0])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "flintSieve",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "excompressum:auto_heavy_sieve_0",
                bus_addr: "quark:quark_chest_7",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot < 21)),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Iron Ore Piece"), n_wanted: 64 }],
                inputs: vec![SlottedInput::new(Filter::Label("Compressed Gravel"), 1, vec![0])],
                max_per_slot: 8,
            }],
        });
        factory.add_process(CraftyConfig {
            name: "gridCrafting",
            turtles: vec![CraftyTurtle {
                client: "2a",
                accesses: vec![CraftyAccess {
                    client: "1a",
                    non_consumable_addr: "",
                    turtle_addr: "turtle_0",
                    bus_addr: "quark:quark_chest_7",
                }],
            }],
            recipes: vec![
                CraftingGridRecipe {
                    outputs: vec![Output { item: Filter::Label("Compressed Gravel"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Gravel"), 9, vec![0, 1, 2, 3, 4, 5, 6, 7, 8])],
                    max_sets: 7,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Ore"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Iron Ore Piece"), 4, vec![0, 1, 2, 3])],
                    max_sets: 16,
                    non_consumables: vec![],
                },
            ],
        })
    })
}
