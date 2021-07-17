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
        bus_accesses: vec![BasicAccess { client: "1a", addr: "minecraft:ender chest_0" }],
        backups: vec![
            (Filter::Label("Seeds"), 32),
            (Filter::Label("Potato"), 32),
            (Filter::Label("Flax"), 32),
            (Filter::Label("Peanut"), 32),
            (Filter::Label("Sugar Canes"), 32),
            (Filter::Label("Cactus"), 32),
            (Filter::Label("Nether Wart"), 32),
            (Filter::Label("Netherrack"), 32),
            (Filter::Name("minecraft:brown_mushroom"), 32),
            (Filter::Name("minecraft:red_mushroom"), 32),
        ],
    }
    .build(|factory| {
        factory.add_storage(DrawerConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "storagedrawers:controller_0",
                bus_addr: "minecraft:ender chest_0",
            }],
            filters: vec![
                Filter::Label("Flint"),
                Filter::Label("Wheat"),
                Filter::Label("Coal Coke"),
                Filter::Label("Grain Bait"),
                Filter::Label("Lapis Lazuli"),
                Filter::Label("Blaze Powder"),
                Filter::Label("Nickel Ingot"),
                Filter::Label("Nether Quartz"),
                Filter::Label("Birch Sapling"),
                Filter::Label("Tin Ore Piece"),
                Filter::Label("Iron Ore Piece"),
                Filter::Label("Gold Ore Piece"),
                Filter::Label("Lead Ore Piece"),
                Filter::Label("Sky Stone Dust"),
                Filter::Label("Pulverized Gold"),
                Filter::Label("Boron Ore Piece"),
                Filter::Label("Osmium Ore Piece"),
                Filter::Label("Nickel Ore Piece"),
                Filter::Label("Silver Ore Piece"),
                Filter::Label("Copper Ore Piece"),
                Filter::Label("Cobalt Ore Piece"),
                Filter::Label("Ardite Ore Piece"),
                Filter::Label("Lithium Ore Piece"),
                Filter::Label("Aluminium Ore Piece"),
                Filter::Label("Magnesium Ore Piece"),
                Filter::Label("Crushed Black Quartz"),
                Filter::Label("Certus Quartz Crystal"),
            ],
        });
        factory.add_process(BufferedConfig {
            name: "output",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:ender chest_1",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchestlarge_3",
                bus_addr: "minecraft:ender chest_0",
            }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchestlarge_2",
                bus_addr: "minecraft:ender chest_0",
            }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchestlarge_1",
                bus_addr: "minecraft:ender chest_0",
            }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchestlarge_0",
                bus_addr: "minecraft:ender chest_0",
            }],
        });
        factory.add_process(BufferedConfig {
            name: "crusherOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "immersiveengineering:woodencrate_3",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "sandInductionOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "immersiveengineering:woodencrate_10",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "chargerOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "immersiveengineering:woodencrate_11",
                bus_addr: "minecraft:ender chest_0",
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
                bus_addr: "minecraft:ender chest_0",
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
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "phytoOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "immersiveengineering:woodencrate_12",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "gearPressOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "immersiveengineering:woodencrate_4",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "redstoneInfuserOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "immersiveengineering:woodencrate_5",
                bus_addr: "minecraft:ender chest_0",
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
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "blockTransmuteOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:rangedcollector_1",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "empowererOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "immersiveengineering:woodencrate_13",
                bus_addr: "minecraft:ender chest_0",
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
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "redstoneInfuser",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_3",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 16,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Enriched Alloy"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basic Control Circuit"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Osmium Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Reception Coil"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Transmission Coil"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Silver Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "diamondInfuser",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_4",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 16,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Refined Obsidian Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Obsidian"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Reinforced Alloy"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Enriched Alloy"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "enrichment",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_2",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 16,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Compressed Redstone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Redstone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Compressed Diamond"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Diamond"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "crusher",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_0",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 16,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gravel"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cobblestone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Charcoal"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Charcoal"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "smelter",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_1",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 16,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("minecraft:netherbrick"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Netherrack"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Starmetal Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Stardust"), 1)],
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
                    outputs: vec![Output { item: Filter::Label("Baked Potato"), n_wanted: 16 }],
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
                    outputs: vec![Output { item: Filter::Label("Cobalt Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cobalt Ore Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ardite Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Ardite Ore Dust"), 1)],
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
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 16,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Niter"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sandstone"), 1)],
                    max_inputs: i32::MAX,
                },
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
                    outputs: vec![Output { item: Filter::Label("Stardust"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Starmetal Ore"), 1)],
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
                    outputs: vec![Output { item: Filter::Label("Pulverized Copper"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Copper Ore"), 1)],
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
            name: "compactor",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_compactor_0",
                bus_addr: "minecraft:ender chest_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Blaze Rod"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Blaze Powder"), 5, vec![0])],
                max_per_slot: 40,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "centrifugal",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_centrifuge_0",
                bus_addr: "minecraft:ender chest_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sugar"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Sugar Canes"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silken Tofu"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Soybean"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Firm Tofu"), n_wanted: 16 },
                        Output { item: Filter::Label("Soy Milk"), n_wanted: 16 },
                    ],
                    inputs: vec![SlottedInput::new(Filter::Label("Silken Tofu"), 1, vec![0])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "pulverizer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_pulverizer_0",
                bus_addr: "minecraft:ender chest_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Flour"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Wheat"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sulfur"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Blaze Rod"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Pulverized Gold"), n_wanted: 64 },
                        Output { item: Filter::Label("Cinnabar"), n_wanted: 64 },
                    ],
                    inputs: vec![SlottedInput::new(Filter::Label("Gold Ore"), 1, vec![0])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "inductionSmelter",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_smelter_0",
                bus_addr: "minecraft:ender chest_0",
            }],
            input_slots: vec![0, 1],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Black Iron Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Block of Invar"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Tough Alloy"), 1, vec![1]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Nickel Ingot"), n_wanted: 64 },
                        Output { item: Filter::Label("Platinum Ingot"), n_wanted: 64 },
                    ],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Cinnabar"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Nickel Ore"), 1, vec![1]),
                    ],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "pressurizer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:pressurizer_0",
                bus_addr: "minecraft:ender chest_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Diamond"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Graphite Ingot"), 64, vec![0])],
                    max_per_slot: 64,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dense Obsidian Plate"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Obsidian Plate"), 9, vec![0])],
                    max_per_slot: 63,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lapis Lazuli Plate"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lapis Lazuli"), 1, vec![0])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "energizedGlowstone",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:melter_0",
                bus_addr: "minecraft:ender chest_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Glowstone Dust"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "lightwell",
            accesses: vec![BusAccess { client: "1a", inv_addr: "xu2:tileuse_0", bus_addr: "minecraft:ender chest_0" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Aquamarine"), 2, vec![0])],
                max_per_slot: 2,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "cokeOven",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_furnace_0",
                bus_addr: "minecraft:ender chest_0",
            }],
            input_slots: vec![0],
            to_extract: extract_all(),
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Coal"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(RedstoneEmitterConfig {
            accesses: vec![RedstoneAccess { client: "1a", addr: Some("redstone_integrator_2"), side: NORTH }],
            output: emit_when_want_item("cokeOven", Filter::Label("Coal Coke"), 64),
        });
        factory.add_process(SlottedConfig {
            name: "compressor",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:compressor_0",
                bus_addr: "minecraft:ender chest_0",
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
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Obsidian Plate"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Pulverized Obsidian"), 1, vec![6])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "extruder",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:metal_former_0",
                bus_addr: "minecraft:ender chest_0",
            }],
            input_slots: vec![6],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gold Cable"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Gold Ingot"), 1, vec![6])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Cable"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Copper Ingot"), 1, vec![6])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "fission",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:fission_controller_new_3",
                bus_addr: "minecraft:ender chest_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("TBU Oxide Fuel"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "pinkSlime",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "storagedrawers:compdrawers_0",
                bus_addr: "minecraft:ender chest_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|_, item| Filter::Label("Pink Slime").apply(&item.item, &item.detail))),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Pink Slime"), n_wanted: 64 }],
                inputs: vec![SlottedInput::new(Filter::Label("Green Slime Block"), 1, vec![0])],
                max_per_slot: 16,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "treeFluidExtractor",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "immersiveengineering:woodencrate_8",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![BufferedInput::new(Filter::Label("Birch Wood"), 64)],
        });
        factory.add_process(BufferedConfig {
            name: "sandInductionStock",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_16",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![BufferedInput::new(Filter::Label("Sand"), 64)],
        });
        factory.add_process(BufferedConfig {
            name: "phytoStock",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_18",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![BufferedInput::new(Filter::Label("Fluxed Phyto-Gro"), 64)],
        });
        factory.add_process(BufferedConfig {
            name: "wirePress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_0",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Aluminium Wire"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Aluminum Ingot"), 1)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "rodPress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_24",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Ardite Tool Rod"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Ardite Ingot"), 1)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "sandInduction",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_17",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Rich Slag"), n_wanted: 64 }],
                inputs: vec![BufferedInput::new(Filter::Label("Clock"), 1)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 16,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "phyto",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_19",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Potato"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Potato"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Birch Wood"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Birch Sapling"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ender Pearl"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Ender Lilly"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Flax"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Flax Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Peanut"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Peanut Seed"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Soybean"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Soybean Seed"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Seeds"), n_wanted: 64 },
                        Output { item: Filter::Label("Wheat"), n_wanted: 64 },
                    ],
                    inputs: vec![BufferedInput::new(Filter::Label("Seeds"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Rice"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Rice Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sugar Canes"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sugar Canes"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cactus"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cactus"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nether Wart"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Wart"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("minecraft:brown_mushroom"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Name("minecraft:brown_mushroom"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("minecraft:red_mushroom"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Name("minecraft:red_mushroom"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("§eInferium Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Inferium Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dye Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dye Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 16,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "charger",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_15",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Fluxed Phyto-Gro"), n_wanted: 64 }],
                inputs: vec![BufferedInput::new(Filter::Label("Rich Phyto-Gro"), 1)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 16,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "gearPress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_7",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lead Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Copper Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bronze Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Bronze Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gold Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lumium Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lumium Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Invar Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Invar Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silver Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Silver Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 32,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "waterBarrel",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_4",
                bus_addr: "minecraft:ender chest_0",
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
            name: "platePress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_3",
                bus_addr: "minecraft:ender chest_0",
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
                    outputs: vec![Output { item: Filter::Label("Gold Item Casing"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Plate"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Item Casing"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Copper Plate"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Tin Item Casing"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Tin Plate"), 1)],
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
                    outputs: vec![Output { item: Filter::Label("Aluminum Plate"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Aluminum Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Uranium Plate"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Uranium Ingot"), 1)],
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
                bus_addr: "minecraft:ender chest_0",
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
                bus_addr: "minecraft:ender chest_0",
            }],
            input_slots: vec![12],
            to_extract: Some(Box::new(|slot, _| slot == 10)),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Impregnated Stick"), n_wanted: 64 }],
                inputs: vec![SlottedInput::new(Filter::Label("Birch Wood"), 2, vec![12])],
                max_per_slot: 16,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "hardenedCasing",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:carpenter_0",
                bus_addr: "minecraft:ender chest_0",
            }],
            input_slots: vec![12, 13],
            to_extract: Some(Box::new(|slot, _| slot == 10)),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Hardened Casing"), n_wanted: 16 }],
                inputs: vec![
                    SlottedInput::new(Filter::Label("Sturdy Casing"), 1, vec![12]),
                    SlottedInput::new(Filter::Label("Diamond"), 4, vec![13]),
                ],
                max_per_slot: 16,
            }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:cobblestone_generator_1",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Cobblestone"), n_wanted: 64 }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:latex_processing_unit_tile_0",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Tiny Dry Rubber"), n_wanted: 64 }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_23",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Obsidian"), n_wanted: 64 }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_precipitator_1",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Snowball"), n_wanted: 64 }],
        });
        factory.add_process(BufferedConfig {
            name: "autoCompressor",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "excompressum:auto_compressor_0",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 12)),
            to_extract: Some(Box::new(|slot, _| slot >= 12)),
            max_recipe_inputs: 128,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Tiny Clump of Thorium-230"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Green Slime Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Rice Slimeball"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Redstone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Redstone"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Black Quartz"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Black Quartz"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Black Iron"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Black Iron Ingot"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Invar"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Invar Ingot"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Coal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Coal"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lapis Lazuli Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lapis Lazuli"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Quartz"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Quartz"), 4)],
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
                bus_addr: "minecraft:ender chest_0",
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
                bus_addr: "minecraft:ender chest_0",
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
                    outputs: vec![Output { item: Filter::Label("Emerald"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed Gravel"), 1, vec![0])],
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
                SlottedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Ruby"), n_wanted: 64 },
                        Output { item: Filter::Label("Malachite"), n_wanted: 64 },
                    ],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed Ender Gravel"), 1, vec![0])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "ironSieve",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "excompressum:auto_heavy_sieve_2",
                bus_addr: "minecraft:ender chest_0",
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
                bus_addr: "minecraft:ender chest_0",
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
                bus_addr: "minecraft:ender chest_0",
            }],
            input_slots: vec![0],
            to_extract: extract_all(),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Thorium-232"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Thorium Dust"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Uranium-238"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Uranium Grit"), 1, vec![0])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "oxidizer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:infuser_0",
                bus_addr: "minecraft:ender chest_0",
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
            name: "glowstoneInfuser",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:infuser_1",
                bus_addr: "minecraft:ender chest_0",
            }],
            input_slots: vec![0],
            to_extract: extract_all(),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lumium Ingot"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Tin Silver Alloy"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Impulse Itemduct"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Itemduct"), 1, vec![0])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "creosoteInfuser",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:infuser_3",
                bus_addr: "minecraft:ender chest_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Treated Wood Planks"), n_wanted: 64 }],
                inputs: vec![SlottedInput::new(Filter::Label("Birch Wood Planks"), 1, vec![0])],
                max_per_slot: 8,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "alloyFurnace",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:alloy_furnace_0",
                bus_addr: "minecraft:ender chest_0",
            }],
            input_slots: vec![0, 1],
            to_extract: extract_all(),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fused Quartz"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Nether Quartz"), 4, vec![0]),
                        SlottedInput::new(Filter::Label("Block of Quartz"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
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
                    outputs: vec![Output { item: Filter::Label("Tin Silver Alloy"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Pulverized Tin"), 3, vec![0]),
                        SlottedInput::new(Filter::Label("Pulverized Silver"), 1, vec![1]),
                    ],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Invar Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Pulverized Iron"), 2, vec![0]),
                        SlottedInput::new(Filter::Label("Nickel Ingot"), 1, vec![1]),
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
                        SlottedInput::new(Filter::Label("Nickel Ingot"), 1, vec![1]),
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
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("minecraft:clay_ball"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Name("minecraft:clay"), 1)],
                    max_inputs: 16,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("String"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Flax"), 3)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Enori Crystal"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Enori Crystal Block"), 1)],
                    max_inputs: 8,
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
                bus_addr: "minecraft:ender chest_0",
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
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Torch"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                        BufferedInput::new(Filter::Label("Stick"), 1),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Scaffolding"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Steel Ingot"), 3),
                        BufferedInput::new(Filter::Label("Steel Rod"), 3),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sturdy Casing"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Bronze Ingot"), 4),
                        BufferedInput::new(Filter::Label("Bronze Gear"), 2),
                        BufferedInput::new(Filter::Label("Copper Gear"), 2),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Sheetmetal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Plate"), 4)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterC",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_2",
                bus_addr: "minecraft:ender chest_0",
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
                    max_inputs: 8,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Sheetmetal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Steel Plate"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Advanced Control Circuit"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Basic Control Circuit"), 1),
                        BufferedInput::new(Filter::Label("Enriched Alloy"), 4),
                        BufferedInput::new(Filter::Label("Redstone"), 4),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Casing"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iron Sheetmetal"), 4),
                        BufferedInput::new(Filter::Label("Tin Electron Tube"), 4),
                        BufferedInput::new(Filter::Label("Hardened Casing"), 1),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Mechanical Component"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iron Plate"), 4),
                        BufferedInput::new(Filter::Label("Copper Ingot"), 1),
                    ],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "atomic",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:dropper_0",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 8,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Void Crystal Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Block of Coal"), 1)],
                    max_inputs: i32::MAX,
                },
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
                    outputs: vec![Output { item: Filter::Label("Soul Sand"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sand"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Rhodochrosite"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Ruby"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ethetic Green Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Chiseled Quartz Block"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Restonia Crystal Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Block of Redstone"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "throwInLiquid",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:dropper_2",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 8,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Infused Wood"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Birch Wood"), 1)],
                max_inputs: i32::MAX,
            }],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "conjuration",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:dropper_3",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 8,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Netherrack"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Netherrack"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Manasteel Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Mana Diamond"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Diamond"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Mana Pearl"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Ender Pearl"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "elven",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:dropper_4",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 8,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Elementium Ingot"), n_wanted: 64 }],
                inputs: vec![BufferedInput::new(Filter::Label("Manasteel Ingot"), 2)],
                max_inputs: i32::MAX,
            }],
            stocks: vec![],
        });
        factory.add_process(ConditionalConfig {
            condition: Box::new(|factory| factory.search_n_stored(&Filter::Label("Terrasteel Ingot")) < 64),
            child: RedstoneConditionalConfig {
                name: Some("terrasteel"),
                accesses: vec![RedstoneAccess { client: "1a", addr: Some("redstone_integrator_3"), side: NORTH }],
                condition: Box::new(|x| x <= 0),
                child: BufferedConfig {
                    name: "terrasteel",
                    accesses: vec![BusAccess {
                        client: "1a",
                        inv_addr: "actuallyadditions:dropper_5",
                        bus_addr: "minecraft:ender chest_0",
                    }],
                    slot_filter: None,
                    to_extract: None,
                    max_recipe_inputs: i32::MAX,
                    recipes: vec![BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Terrasteel Ingot"), n_wanted: 64 }],
                        inputs: vec![
                            BufferedInput::new(Filter::Label("Manasteel Ingot"), 1),
                            BufferedInput::new(Filter::Label("Mana Diamond"), 1),
                            BufferedInput::new(Filter::Label("Mana Pearl"), 1),
                        ],
                        max_inputs: 3,
                    }],
                    stocks: vec![],
                },
            },
        });
        factory.add_process(BufferedConfig {
            name: "kekimurus",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:placer_1",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 0,
            recipes: vec![],
            stocks: vec![BufferedInput::new(Filter::Label("Cake"), 9)],
        });
        factory.add_process(BufferedConfig {
            name: "infuserStock",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_8",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 0,
            recipes: vec![],
            stocks: vec![
                BufferedInput::new(Filter::Label("Compressed Redstone"), 8),
                BufferedInput::new(Filter::Label("Compressed Diamond"), 8),
            ],
        });
        factory.add_process(BufferedConfig {
            name: "smallPlatePresser",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:placer_0",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 8,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Stick"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Birch Wood Planks"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Rod"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Steel Sheetmetal"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminium Rod"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Aluminium Sheetmetal"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            stocks: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "tinElectronTube",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:fabricator_0",
                bus_addr: "minecraft:ender chest_0",
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
        factory.add_process(SlottedConfig {
            name: "teFrame",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:fabricator_1",
                bus_addr: "minecraft:ender chest_0",
            }],
            input_slots: vec![0, 3, 4, 5, 6, 7, 8],
            to_extract: extract_all(),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output {
                    item: Filter::Both { label: "Machine Frame", name: "thermalexpansion:frame" },
                    n_wanted: 16,
                }],
                inputs: vec![
                    SlottedInput::new(Filter::Label("Sand"), 6, vec![0]),
                    SlottedInput::new(Filter::Label("Enori Crystal"), 4, vec![3]),
                    SlottedInput::new(Filter::Name("rftools:machine_frame"), 1, vec![4]),
                    SlottedInput::new(Filter::Label("Heavy Engineering Block"), 1, vec![5]),
                    SlottedInput::new(Filter::Label("Device Frame"), 1, vec![6]),
                    SlottedInput::new(Filter::Label("Iron Casing"), 1, vec![7]),
                    SlottedInput::new(Filter::Label("Machine Case"), 1, vec![8]),
                ],
                max_per_slot: 24,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "crafterD",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_3",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Heavy Engineering Block"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Steel Scaffolding"), 1),
                        BufferedInput::new(Filter::Label("Uranium Plate"), 4),
                        BufferedInput::new(Filter::Label("Reinforced Alloy"), 2),
                        BufferedInput::new(Filter::Label("Iron Mechanical Component"), 2),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Glass Pane"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Glass"), 6)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Range Addon"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Cobblestone"), 6),
                        BufferedInput::new(Filter::Label("Plastic"), 2),
                        BufferedInput::new(Filter::Label("Glass Pane"), 1),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Coil"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Copper Cable"), 8),
                        BufferedInput::new(Filter::Label("Iron Ingot"), 1),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Heat Vent"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iron Bars"), 4),
                        BufferedInput::new(Filter::Label("Iron Plate"), 4),
                        BufferedInput::new(Filter::Label("Electric Motor"), 1),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Itemduct"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Tin Ingot"), 2),
                        BufferedInput::new(Filter::Label("Fused Quartz"), 1),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sandstone"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sand"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Clock"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Gold Ingot"), 4),
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                    ],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterE",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_4",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Bars"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 6)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Device Frame"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Tin Ingot"), 4),
                        BufferedInput::new(Filter::Label("Glass"), 4),
                        BufferedInput::new(Filter::Label("Copper Gear"), 1),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("rftools:machine_frame"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Heat Vent"), 2),
                        BufferedInput::new(Filter::Label("Dry Rubber"), 2),
                        BufferedInput::new(Filter::Label("Pink Slime"), 2),
                        BufferedInput::new(Filter::Label("Machine Case"), 1),
                        BufferedInput::new(Filter::Label("Range Addon"), 1),
                        BufferedInput::new(Filter::Label("Gold Gear"), 1),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Rich Phyto-Gro"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Pulverized Charcoal"), 1),
                        BufferedInput::new(Filter::Label("Rich Slag"), 1),
                        BufferedInput::new(Filter::Label("Niter"), 1),
                    ],
                    max_inputs: 12,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Rice Dough"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Rice"), 3)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Paper"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sugar Canes"), 3)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Quartz Slab"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Block of Quartz"), 3)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gold Sheetmetal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Plate"), 4)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterF",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_5",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electric Motor"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iron Ingot"), 1),
                        BufferedInput::new(Filter::Label("Tin Item Casing"), 2),
                        BufferedInput::new(Filter::Label("Coil"), 2),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Chiseled Quartz Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Quartz Slab"), 2)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Advanced Coil"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Impregnated Stick"), 2),
                        BufferedInput::new(Filter::Label("Gold Cable"), 4),
                        BufferedInput::new(Filter::Label("Basic Coil"), 1),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Reinforcement"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Dense Obsidian Plate"), 2),
                        BufferedInput::new(Filter::Label("Gold Sheetmetal"), 6),
                        BufferedInput::new(Filter::Label("Gold Item Casing"), 1),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Button"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Stone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Birch Chest"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Button"), 1),
                        BufferedInput::new(Filter::Label("Treated Wood Planks"), 4),
                        BufferedInput::new(Filter::Label("Birch Wood"), 4),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Quartz Sliver"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Quartz"), 1)],
                    max_inputs: 8,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Network Cable"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Lapis Lazuli Plate"), 4),
                        BufferedInput::new(Filter::Label("Gold Plate"), 1),
                        BufferedInput::new(Filter::Label("Redstone"), 4),
                    ],
                    max_inputs: 18,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "riceSlimeball",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_crafter_0",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 18)),
            to_extract: Some(Box::new(|slot, _| slot == 18)),
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Rice Slimeball"), n_wanted: 64 }],
                inputs: vec![BufferedInput::new(Filter::Label("Rice Dough"), 4)],
                max_inputs: 32,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "empowerer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "immersiveengineering:woodencrate_15",
                bus_addr: "minecraft:ender chest_0",
            }],
            input_slots: vec![0, 1, 2, 3, 4],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Void Crystal Block"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Void Crystal Block"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Block of Black Iron"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Block of Black Quartz"), 1, vec![2]),
                        SlottedInput::new(Filter::Label("Ink Sac"), 1, vec![3]),
                        SlottedInput::new(Filter::Label("Basalt"), 1, vec![4]),
                    ],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Restonia Crystal Block"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Restonia Crystal Block"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Redstone Reception Coil"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Rhodochrosite"), 1, vec![2]),
                        SlottedInput::new(Filter::Label("Ardite Tool Rod"), 1, vec![3]),
                        SlottedInput::new(Filter::Label("Red Nether Brick"), 1, vec![4]),
                    ],
                    max_per_slot: 1,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "blockTransmute",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "immersiveengineering:woodencrate_14",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Livingwood"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Infused Wood"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Livingrock"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Arcane Stone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("End Stone"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sandstone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Starmetal Ore"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ore"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 16,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "terraVisCrystal",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "immersiveengineering:woodencrate_16",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Terra Vis Crystal"), n_wanted: 64 }],
                inputs: vec![BufferedInput::new(Filter::Label("Quartz Sliver"), 1)],
                max_inputs: 1,
            }],
            stocks: vec![
                BufferedInput::new(Filter::Label("Charcoal"), 64),
                BufferedInput::new(Filter::Label("Stone"), 64),
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterG",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_6",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Arcane Stone"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Stone"), 8),
                        BufferedInput::new(Filter::Label("Terra Vis Crystal"), 1),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Leadstone Fluxduct"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Redstone"), 6),
                        BufferedInput::new(Filter::Label("Network Cable"), 2),
                        BufferedInput::new(Filter::Label("Glass"), 1),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Hardened Fluxduct"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Leadstone Fluxduct"), 3),
                        BufferedInput::new(Filter::Label("Redstone"), 3),
                        BufferedInput::new(Filter::Label("Invar Ingot"), 1),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basalt"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Cobblestone"), 4),
                        BufferedInput::new(Filter::Label("Coal"), 4),
                    ],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dandelion Yellow"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dye Essence"), 3)],
                    max_inputs: 48,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Void Crystal"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Empowered Void Crystal Block"), 1)],
                    max_inputs: 8,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cake"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Raw Tofeeg"), 1),
                        BufferedInput::new(Filter::Label("Soy Milk"), 3),
                        BufferedInput::new(Filter::Label("Flour"), 3),
                        BufferedInput::new(Filter::Label("Sugar"), 2),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Solenoid"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Mixed Metal Ingot"), 1),
                        BufferedInput::new(Filter::Label("Copper Plate"), 4),
                        BufferedInput::new(Filter::Label("Aluminium Rod"), 2),
                        BufferedInput::new(Filter::Label("Copper Item Casing"), 2),
                    ],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterH",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_7",
                bus_addr: "minecraft:ender chest_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ink Sac"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dye Essence"), 3)],
                    max_inputs: 48,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Raw Tofeeg"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Firm Tofu"), 1),
                        BufferedInput::new(Filter::Label("Dandelion Yellow"), 1),
                    ],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminium Sheetmetal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Aluminum Plate"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Advanced Plating"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Tough Alloy"), 4),
                        BufferedInput::new(Filter::Label("Basic Plating"), 1),
                        BufferedInput::new(Filter::Label("Redstone"), 4),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Red Nether Brick"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Nether Wart"), 2),
                        BufferedInput::new(Filter::Name("minecraft:netherbrick"), 2),
                    ],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Restonia Crystal"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Empowered Restonia Crystal Block"), 1)],
                    max_inputs: 8,
                },
            ],
        });
    })
}
