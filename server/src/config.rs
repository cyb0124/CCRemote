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
        bus_accesses: vec![BasicAccess { client: "1a", addr: "ic2:steel_storage_box_0" }],
        backups: vec![(Filter::Label("Potato"), 32)],
    }
    .build(|factory| {
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchest_0",
                bus_addr: "ic2:steel_storage_box_0",
            }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchest_1",
                bus_addr: "ic2:steel_storage_box_0",
            }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchest_2",
                bus_addr: "ic2:steel_storage_box_0",
            }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchestmedium_0",
                bus_addr: "ic2:steel_storage_box_0",
            }],
        });
        factory.add_process(ScatteringConfig {
            name: "plantSower",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:crop_sower_tile_1",
                bus_addr: "ic2:steel_storage_box_0",
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
                bus_addr: "ic2:steel_storage_box_0",
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
                bus_addr: "ic2:steel_storage_box_0",
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
                bus_addr: "ic2:steel_storage_box_0",
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
                bus_addr: "ic2:steel_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 24,
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
                    outputs: vec![Output { item: Filter::Label("Silver Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Silver"), 1)],
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
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Tin Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Tin"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Lead"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminum Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Aluminum"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Plastic"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dry Rubber"), 1)],
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
                bus_addr: "ic2:steel_storage_box_0",
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
                bus_addr: "ic2:steel_storage_box_0",
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
                    outputs: vec![Output { item: Filter::Label("Pulverized Nickel"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Nickel Ore"), 1, vec![0])],
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
                bus_addr: "ic2:steel_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot < 21)),
            recipes: vec![SlottedRecipe {
                outputs: vec![
                    Output { item: Filter::Label("Aluminium Ore Piece"), n_wanted: 64 },
                    Output { item: Filter::Label("Nickel Ore Piece"), n_wanted: 64 },
                    Output { item: Filter::Label("Copper Ore Piece"), n_wanted: 64 },
                    Output { item: Filter::Label("Silver Ore Piece"), n_wanted: 64 },
                    Output { item: Filter::Label("Iron Ore Piece"), n_wanted: 64 },
                    Output { item: Filter::Label("Lead Ore Piece"), n_wanted: 64 },
                    Output { item: Filter::Label("Gold Ore Piece"), n_wanted: 64 }, // TODO: diamondSieve nether
                    Output { item: Filter::Label("Tin Ore Piece"), n_wanted: 64 },
                ],
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
                    bus_addr: "ic2:steel_storage_box_0",
                }],
            }],
            recipes: vec![
                CraftingGridRecipe {
                    outputs: vec![Output { item: Filter::Label("String"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Cotton"), 3, vec![0, 1, 3])],
                    max_sets: 21,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: vec![Output { item: Filter::Label("Basic Machine Casing"), n_wanted: 8 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Aluminum Plate"), 4, vec![0, 2, 6, 8]),
                        SlottedInput::new(Filter::Label("Dense Iron Plate"), 4, vec![1, 3, 5, 7]),
                    ],
                    max_sets: 16,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Sheetmetal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Iron Plate"), 4, vec![1, 3, 5, 7])],
                    max_sets: 16,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Sheetmetal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lead Plate"), 4, vec![1, 3, 5, 7])],
                    max_sets: 16,
                    non_consumables: vec![],
                },
            ],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:cobblestone_generator_0",
                bus_addr: "ic2:steel_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Cobblestone"), n_wanted: 64 }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:latex_processing_unit_tile_0",
                bus_addr: "ic2:steel_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Tiny Dry Rubber"), n_wanted: 64 }],
        });
        factory.add_process(SlottedConfig {
            name: "alloyFurnace",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:alloy_furnace_0",
                bus_addr: "ic2:steel_storage_box_0",
            }],
            input_slots: vec![0, 1],
            to_extract: extract_all(),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Pulverized Iron"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Graphite Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bronze Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Pulverized Copper"), 3, vec![0]),
                        SlottedInput::new(Filter::Label("Pulverized Tin"), 1, vec![1]),
                    ],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Invar Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Pulverized Iron"), 2, vec![0]),
                        SlottedInput::new(Filter::Label("Pulverized Nickel"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electrum Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Pulverized Gold"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Pulverized Silver"), 1, vec![1]),
                    ],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "autoCompressor",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "excompressum:auto_compressor_0",
                bus_addr: "ic2:steel_storage_box_0",
            }],
            slot_filter: None,
            to_extract: Some(Box::new(|slot, _| slot >= 12)),
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Compressed Gravel"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gravel"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Graphite Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Graphite Ingot"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dry Rubber"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Tiny Dry Rubber"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Ore"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ore Piece"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Ore"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lead Ore Piece"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Tin Ore"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Tin Ore Piece"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gold Ore"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Ore Piece"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Ore"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Copper Ore Piece"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminum Ore"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Aluminium Ore Piece"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nickel Ore"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nickel Ore Piece"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silver Ore"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Silver Ore Piece"), 4)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "pressurizer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:pressurizer_0",
                bus_addr: "ic2:steel_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: extract_all(),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminum Plate"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Aluminum Ingot"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Plate"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Iron Ingot"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Plate"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Steel Ingot"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Plate"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lead Ingot"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dense Iron Plate"), n_wanted: 8 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Iron Plate"), 9, vec![0])],
                    max_per_slot: 63,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "tinElectronTube",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:fabricator_1",
                bus_addr: "ic2:steel_storage_box_0",
            }],
            input_slots: vec![0, 3, 4],
            to_extract: extract_all(),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Tin Electron Tube"), n_wanted: 16 }],
                inputs: vec![
                    SlottedInput::new(Filter::Label("Sand"), 1, vec![0]),
                    SlottedInput::new(Filter::Label("Tin Ingot"), 10, vec![3]),
                    SlottedInput::new(Filter::Label("Redstone"), 4, vec![4]),
                ],
                max_per_slot: 40,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "rubber",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_5",
                bus_addr: "ic2:steel_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![BufferedInput::new(Filter::Label("Birch Wood"), 8)],
        })
    })
}
