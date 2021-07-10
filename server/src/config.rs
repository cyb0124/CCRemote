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
        bus_accesses: vec![BasicAccess { client: "1a", addr: "immersiveengineering:woodencrate_2" }],
        backups: vec![(Filter::Label("Potato"), 32), (Filter::Label("Flax"), 32), (Filter::Label("Peanut"), 32)],
    }
    .build(|factory| {
        factory.add_storage(DrawerConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "storagedrawers:controller_0",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            filters: vec![
                Filter::Label("Coal"),
                Filter::Label("Flint"),
                Filter::Label("Blaze Powder"),
                Filter::Label("Nether Quartz"),
                Filter::Label("Birch Sapling"),
                Filter::Label("Tin Ore Piece"),
                Filter::Label("Iron Ore Piece"),
                Filter::Label("Gold Ore Piece"),
                Filter::Label("Lead Ore Piece"),
                Filter::Label("Sky Stone Dust"),
                Filter::Label("Osmium Ore Piece"),
                Filter::Label("Nickel Ore Piece"),
                Filter::Label("Silver Ore Piece"),
                Filter::Label("Copper Ore Piece"),
                Filter::Label("Aluminium Ore Piece"),
                Filter::Label("Crushed Black Quartz"),
            ],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchestmedium_3",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchestmedium_2",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchestmedium_1",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchestmedium_0",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
        });
        factory.add_process(BufferedConfig {
            name: "crusherOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "immersiveengineering:woodencrate_3",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "smelterOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "immersiveengineering:woodencrate_0",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "manufactoryOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "immersiveengineering:woodencrate_7",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "atomicOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:rangedcollector_0",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "wirePressOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_2",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "lavaBarrelOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_5",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "plantGatherer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:crop_recolector_tile_2",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: None,
            to_extract: Some(Box::new(|slot, _| slot >= 6)),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(ScatteringConfig {
            name: "plantSower",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:crop_sower_tile_5",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            input_slots: vec![6, 7, 8, 9, 11, 12, 13, 14],
            to_extract: None,
            recipes: vec![
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Potato"), n_wanted: 64 }],
                    ScatteringInput::new(Filter::Label("Potato")).allow_backup(),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Flax"), n_wanted: 64 }],
                    ScatteringInput::new(Filter::Label("Flax")).allow_backup(),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Peanut"), n_wanted: 64 }],
                    ScatteringInput::new(Filter::Label("Peanut")).allow_backup(),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Birch Wood"), n_wanted: 64 }],
                    ScatteringInput::new(Filter::Label("Birch Sapling")),
                ),
            ],
            max_per_slot: 4,
        });
        factory.add_process(BufferedConfig {
            name: "crusher",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_0",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 16,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Gravel"), n_wanted: 64 }],
                inputs: vec![BufferedInput::new(Filter::Label("Cobblestone"), 1)],
                max_inputs: i32::MAX,
            }],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "smelter",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_1",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 16,
            recipes: vec![
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
                    outputs: vec![Output { item: Filter::Label("Baked Potato"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Potato"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Glass"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sand"), 1)],
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
                    outputs: vec![Output { item: Filter::Label("Nickel Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Nickel"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Black Quartz"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Crushed Black Quartz"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Plastic"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dry Rubber"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Uranium Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Uranium Grit"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "manufactory",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "immersiveengineering:woodencrate_6",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 16,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Clay Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Name("minecraft:clay"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Coal"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Coal"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nether Quartz Dust"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Quartz"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Obsidian"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Obsidian"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sand"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cobblestone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Birch Wood Planks"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Birch Wood"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Iron"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Aluminum"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Aluminum Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Nickel"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nickel Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Copper"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Copper Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Gold"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Osmium Dust"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Osmium Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Lead"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lead Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Silver"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Silver Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Tin"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Tin Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Uranium Grit"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Uranium Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Boron Dust"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Boron Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Magnesium Dust"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Magnesium Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lithium Dust"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lithium Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Thorium Dust"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Thorium Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cobalt Ore Dust"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cobalt Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ardite Ore Dust"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Ardite Ore"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            stocks: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "pressurizer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:pressurizer_0",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Diamond"), n_wanted: 64 }],
                inputs: vec![SlottedInput::new(Filter::Label("Graphite Ingot"), 64, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "compressor",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:compressor_0",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            input_slots: vec![6],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Advanced Alloy"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Mixed Metal Ingot"), 1, vec![6])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Carbon Plate"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Raw Carbon Mesh"), 1, vec![6])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "extruder",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:metal_former_0",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            input_slots: vec![6],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Gold Cable"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Gold Ingot"), 1, vec![6])],
                max_per_slot: 8,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "fission",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:fission_controller_new_1",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("TBU Oxide Fuel"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "treeFluidExtractor",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "immersiveengineering:woodencrate_8",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![BufferedInput::new(Filter::Label("Birch Wood"), 64)],
        });
        factory.add_process(BufferedConfig {
            name: "wirePress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_0",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Aluminium Wire"), n_wanted: 64 }],
                inputs: vec![BufferedInput::new(Filter::Label("Aluminum Ingot"), 1)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "waterBarrel",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_4",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Name("minecraft:clay"), n_wanted: 64 }],
                inputs: vec![BufferedInput::new(Filter::Label("Dust"), 1)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "lavaBarrel",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_6",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("End Stone"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Glowstone Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Netherrack"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Redstone"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "platePress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_3",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Large Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Block of Iron"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Item Casing"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lead Plate"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Item Casing"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Copper Plate"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Plate"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Plate"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lead Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Plate"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Copper Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bronze Plate"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Bronze Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Tin Plate"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Tin Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gold Plate"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Plate"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Steel Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lapis Lazuli Plate"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lapis Lazuli"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminum Plate"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Aluminum Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "seedOil",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:squeezer_1",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Peanut"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "impregnatedStick",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:carpenter_1",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            input_slots: vec![12],
            to_extract: Some(Box::new(|slot, _| slot == 10)),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Impregnated Stick"), n_wanted: 64 }],
                inputs: vec![SlottedInput::new(Filter::Label("Birch Wood"), 2, vec![12])],
                max_per_slot: 16,
            }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:cobblestone_generator_1",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Cobblestone"), n_wanted: 64 }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:latex_processing_unit_tile_0",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Tiny Dry Rubber"), n_wanted: 64 }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_1",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Obsidian"), n_wanted: 64 }],
        });
        factory.add_process(BufferedConfig {
            name: "autoCompressor",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "excompressum:auto_compressor_0",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: None,
            to_extract: Some(Box::new(|slot, _| slot >= 12)),
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lapis Lazuli Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lapis Lazuli"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Compressed Nether Gravel"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Crushed Netherrack"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Compressed Netherrack"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Netherrack"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Compressed End Stone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("End Stone"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Compressed Ender Gravel"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Crushed Endstone"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Compressed Cobblestone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cobblestone"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Compressed Gravel"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gravel"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Compressed Sand"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sand"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Compressed Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dust"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Compressed Soul Sand"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Soul Sand"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Graphite Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Graphite Ingot"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Iron"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 9)],
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
                    outputs: vec![Output { item: Filter::Label("Osmium Ore"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Osmium Ore Piece"), 4)],
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
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Uranium Ore"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Uranium Ore Piece"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Boron Ore"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Boron Ore Piece"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Magnesium Ore"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Magnesium Ore Piece"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lithium Ore"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lithium Ore Piece"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cobalt Ore"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cobalt Ore Piece"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ardite Ore"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Ardite Ore Piece"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Thorium Ore"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Thorium Ore Piece"), 4)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "flintSieve",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "excompressum:auto_heavy_sieve_0",
                bus_addr: "immersiveengineering:woodencrate_2",
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
                    Output { item: Filter::Label("Tin Ore Piece"), n_wanted: 64 },
                    Output { item: Filter::Label("Lapis Lazuli"), n_wanted: 64 },
                    Output { item: Filter::Label("Coal"), n_wanted: 64 },
                ],
                inputs: vec![SlottedInput::new(Filter::Label("Compressed Gravel"), 1, vec![0])],
                max_per_slot: 8,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "diamondSieve",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "excompressum:auto_heavy_sieve_1",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot < 21)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Osmium Ore Piece"), n_wanted: 64 },
                        Output { item: Filter::Label("Crushed Black Quartz"), n_wanted: 64 },
                    ],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed Gravel"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed Dust"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aquamarine"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed Sand"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nether Quartz"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed Soul Sand"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Gold Ore Piece"), n_wanted: 64 },
                        Output { item: Filter::Label("Boron Ore Piece"), n_wanted: 64 },
                        Output { item: Filter::Label("Magnesium Ore Piece"), n_wanted: 64 },
                        Output { item: Filter::Label("Lithium Ore Piece"), n_wanted: 64 },
                        Output { item: Filter::Label("Cobalt Ore Piece"), n_wanted: 64 },
                        Output { item: Filter::Label("Ardite Ore Piece"), n_wanted: 64 },
                        Output { item: Filter::Label("Thorium Ore Piece"), n_wanted: 64 },
                    ],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed Nether Gravel"), 1, vec![0])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "ironSieve",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "excompressum:auto_heavy_sieve_2",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot < 21)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Uranium Ore Piece"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed Ender Gravel"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Glowstone Dust"), n_wanted: 64 },
                        Output { item: Filter::Label("Blaze Powder"), n_wanted: 64 },
                    ],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed Soul Sand"), 1, vec![0])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "autoHammer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "excompressum:auto_compressed_hammer_json_0",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot < 21)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dust"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed Sand"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crushed Endstone"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed End Stone"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crushed Netherrack"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed Netherrack"), 1, vec![0])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "isotopeSeparator",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:isotope_separator_0",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            input_slots: vec![0],
            to_extract: extract_all(),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Thorium-232"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Thorium Dust"), 1, vec![0])],
                max_per_slot: 8,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "oxidizer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:infuser_0",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            input_slots: vec![0],
            to_extract: extract_all(),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("TBU Oxide Fuel"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("TBU Fuel"), 1, vec![0])],
                max_per_slot: 8,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "alloyFurnace",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:alloy_furnace_0",
                bus_addr: "immersiveengineering:woodencrate_2",
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
                    outputs: vec![Output { item: Filter::Label("Ferroboron Alloy"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Steel Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Boron Dust"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Tough Alloy"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Ferroboron Alloy"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Lithium Dust"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Manyullyn Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Cobalt Ore Dust"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Ardite Ore Dust"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
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
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Constantan Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Pulverized Copper"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Pulverized Nickel"), 1, vec![1]),
                    ],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterA",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_0",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("minecraft:clay_ball"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Name("minecraft:clay"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("String"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Flax"), 3)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Enori Crystal"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Enori Crystal Block"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Sheetmetal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lead Plate"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basic Machine Casing"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iron Large Plate"), 4),
                        BufferedInput::new(Filter::Label("Aluminum Plate"), 4),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Reinforced Stone"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Clay Dust"), 1),
                        BufferedInput::new(Filter::Label("Stone"), 4),
                        BufferedInput::new(Filter::Label("Grout"), 4),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Mixed Metal Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iron Plate"), 3),
                        BufferedInput::new(Filter::Label("Bronze Plate"), 3),
                        BufferedInput::new(Filter::Label("Tin Plate"), 3),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Raw Carbon Mesh"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Raw Carbon Fibre"), 2)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterB",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_1",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basic Plating"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Lead Sheetmetal"), 4),
                        BufferedInput::new(Filter::Label("Lead Item Casing"), 4),
                        BufferedInput::new(Filter::Label("Graphite Block"), 1),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Grout"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Sand"), 4),
                        BufferedInput::new(Filter::Label("Gravel"), 4),
                        BufferedInput::new(Filter::Name("minecraft:clay"), 1),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Raw Carbon Fibre"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Coal"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Advanced Machine Casing"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Steel Plate"), 4),
                        BufferedInput::new(Filter::Label("Advanced Alloy"), 2),
                        BufferedInput::new(Filter::Label("Carbon Plate"), 2),
                        BufferedInput::new(Filter::Label("Basic Machine Casing"), 1),
                    ],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterC",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_2",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Machine Case"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Advanced Machine Casing"), 1),
                        BufferedInput::new(Filter::Label("Reinforced Stone"), 4),
                        BufferedInput::new(Filter::Label("Plastic"), 4),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("TBU Fuel"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Thorium-232"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basic Coil"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Impregnated Stick"), 3),
                        BufferedInput::new(Filter::Label("Aluminium Wire"), 4),
                        BufferedInput::new(Filter::Label("Enori Crystal"), 2),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Palis Crystal"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Palis Crystal Block"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "atomic",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:dropper_0",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 8,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Enori Crystal Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Block of Iron"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Palis Crystal Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lapis Lazuli Block"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Soul Sand"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sand"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "atomic",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:dropper_1",
                bus_addr: "immersiveengineering:woodencrate_2",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 0,
            recipes: vec![],
            stocks: vec![BufferedInput::new(Filter::Label("Charcoal"), 64)],
        })
    })
}
