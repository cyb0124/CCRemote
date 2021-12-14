use super::detail_cache::DetailCache;
use super::factory::{Factory, FactoryConfig};
use super::server::Server;
use super::{access::*, item::*, process::*, recipe::*, storage::*};
use std::{cell::RefCell, rc::Rc, time::Duration};

pub fn build_factory() -> Rc<RefCell<Factory>> {
    FactoryConfig {
        detail_cache: DetailCache::new(),
        server: Server::new(1848),
        min_cycle_time: Duration::from_secs(1),
        log_clients: vec!["1a"],
        bus_accesses: vec![BasicAccess { client: "1a", addr: "minecraft:barrel_1" }],
        backups: vec![(Filter::Label("Pure Certus Quartz Crystal"), 32)],
    }
    .build(|factory| {
        factory.add_storage(DrawerConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "storagedrawers:standard_drawers_4_0",
                bus_addr: "minecraft:barrel_1",
            }],
            filters: vec![
                Filter::Label("Cobblestone"),
                Filter::Label("Gabbro Cobblestone"),
                Filter::Label("Diorite Cobblestone"),
                Filter::Label("Andesite Cobblestone"),
            ],
        });
        factory.add_storage(DrawerConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "storagedrawers:standard_drawers_4_1",
                bus_addr: "minecraft:barrel_1",
            }],
            filters: vec![
                Filter::Label("Kelp"),
                Filter::Label("Carrot"),
                Filter::Label("Wheat Seeds"),
                Filter::Label("Granite Cobblestone"),
            ],
        });
        factory.add_storage(DrawerConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "storagedrawers:standard_drawers_4_3",
                bus_addr: "minecraft:barrel_1",
            }],
            filters: vec![
                Filter::Label("Birch Log"),
                Filter::Label("Yellow Autumn Sapling"),
                Filter::Label("Stick"),
                Filter::Label("Flax Seeds"),
            ],
        });
        factory.add_storage(DrawerConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "storagedrawers:standard_drawers_4_4",
                bus_addr: "minecraft:barrel_1",
            }],
            filters: vec![
                Filter::Label("Sugar Cane"),
                Filter::Label("Rubber"),
                Filter::Label("Wheat"),
                Filter::Label("Flint"),
            ],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:chest_0", bus_addr: "minecraft:barrel_1" }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:chest_1", bus_addr: "minecraft:barrel_1" }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:chest_2", bus_addr: "minecraft:barrel_1" }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:chest_3", bus_addr: "minecraft:barrel_1" }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:chest_4", bus_addr: "minecraft:barrel_1" }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:chest_5", bus_addr: "minecraft:barrel_1" }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:chest_6", bus_addr: "minecraft:barrel_1" }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:chest_7", bus_addr: "minecraft:barrel_1" }],
        });
        factory.add_process(SyncAndRestockConfig {
            name: "treeFarm",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_0", bus_addr: "minecraft:barrel_1" }],
            accesses_in: vec![RedstoneAccess { client: "1a", addr: None, side: RIGHT, bit: Some(0) }],
            accesses_out: vec![RedstoneAccess { client: "1a", addr: None, side: RIGHT, bit: Some(12) }],
            hold_if_unfilled: false,
            stocks: Box::new(|_| vec![BufferedInput::new(Filter::Label("Yellow Autumn Sapling"), 64)]),
        });
        factory.add_process(BufferedConfig {
            name: "cobbleGen",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_2", bus_addr: "minecraft:barrel_1" }],
            slot_filter: None,
            to_extract: extract_all(),
            stocks: vec![],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "strainer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "waterstrainer:strainer_1",
                bus_addr: "minecraft:barrel_1",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            stocks: vec![],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "waterFanOutput",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_15", bus_addr: "minecraft:barrel_1" }],
            slot_filter: None,
            to_extract: extract_all(),
            stocks: vec![],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "waterFan",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_16", bus_addr: "minecraft:barrel_1" }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Clay Ball"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sand"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Nugget"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gravel"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "lavaFanOutput",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_32", bus_addr: "minecraft:barrel_1" }],
            slot_filter: None,
            to_extract: extract_all(),
            stocks: vec![],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "lavaFan",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:dropper_0", bus_addr: "minecraft:barrel_1" }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
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
                    outputs: vec![Output { item: Filter::Label("Smooth Stone"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Stone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Algal Brick"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Algal Blend"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Charcoal"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Birch Log"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cured Rubber"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Rubber"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Seared Brick"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Grout"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "mixerOutput",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_12", bus_addr: "minecraft:barrel_1" }],
            slot_filter: None,
            to_extract: extract_all(),
            stocks: vec![],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "mixer",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:basin_1", bus_addr: "minecraft:barrel_1" }],
            input_slots: vec![0, 1, 2],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Algal Blend"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Kelp"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Clay Ball"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Andesite Alloy"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Andesite Cobblestone"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Algal Brick"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Grout"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Clay Ball"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Gravel"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Sand"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "kineticA",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:deployer_2", bus_addr: "minecraft:barrel_1" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Andesite Alloy"), 1, vec![0])],
                max_per_slot: 16,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "kineticB",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:deployer_3", bus_addr: "minecraft:barrel_1" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Andesite Alloy"), 1, vec![0])],
                max_per_slot: 16,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "kinetic",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_13", bus_addr: "minecraft:barrel_1" }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("item.kubejs.kinetic_mechanism"), n_wanted: 64 }],
                inputs: vec![BufferedInput::new(Filter::Label("Birch Slab"), 1)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "millstone",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:millstone_2", bus_addr: "minecraft:barrel_1" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sky Stone Dust"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Sky Stone"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gravel"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Cobblestone"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("String"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Flax"), 1, vec![0])],
                    max_per_slot: 16,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "rubber",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_17", bus_addr: "minecraft:barrel_1" }],
            slot_filter: None,
            to_extract: extract_all(),
            stocks: vec![],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "saw",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_29", bus_addr: "minecraft:barrel_1" }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Stripped Birch Log"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Birch Log"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Birch Planks"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Stripped Birch Log"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "slabSaw",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_19", bus_addr: "minecraft:barrel_1" }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Birch Slab"), n_wanted: 64 }],
                inputs: vec![BufferedInput::new(Filter::Label("Birch Planks"), 1)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "waterSpoutOutput",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_24", bus_addr: "minecraft:barrel_1" }],
            slot_filter: None,
            to_extract: extract_all(),
            stocks: vec![],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "waterSpout",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_25", bus_addr: "minecraft:barrel_1" }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("item.kubejs.tiny_certus_crystal"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Certus Quartz Seed"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("item.kubejs.small_certus_crystal"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("item.kubejs.tiny_certus_crystal"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pure Certus Quartz Crystal"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("item.kubejs.small_certus_crystal"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "liquidRedstone",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:basin_6", bus_addr: "minecraft:barrel_1" }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|_, stack| stack.detail.label == "Certus Quartz Crystal")),
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Charged Certus Quartz Crystal"), 1, vec![0])],
                max_per_slot: 16,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "liquidSkystone",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:basin_5", bus_addr: "minecraft:barrel_1" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Sky Stone Dust"), 1, vec![0])],
                max_per_slot: 16,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "generator",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:dynamo_stirling_5",
                bus_addr: "minecraft:barrel_1",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Charcoal"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "charger",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_28", bus_addr: "minecraft:barrel_1" }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Charged Certus Quartz Crystal"), n_wanted: 64 }],
                inputs: vec![BufferedInput::new(Filter::Label("Certus Quartz Crystal"), 1)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "redstoneMixerOutput",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_23", bus_addr: "minecraft:barrel_1" }],
            slot_filter: None,
            to_extract: extract_all(),
            stocks: vec![],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "liquidRedstoneMixer",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:basin_7", bus_addr: "minecraft:barrel_1" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Polished Rose Quartz"), n_wanted: 64 }],
                inputs: vec![SlottedInput::new(Filter::Label("Pure Certus Quartz Crystal"), 1, vec![0])],
                max_per_slot: 16,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "packer",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:basin_8", bus_addr: "minecraft:barrel_1" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Ingot"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Iron Nugget"), 9, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("White Wool"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("String"), 4, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Carrot Crate"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Carrot"), 9, vec![0])],
                    max_per_slot: 16,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "liquidIron",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_27", bus_addr: "minecraft:barrel_1" }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![BufferedRecipe {
                outputs: vec![],
                inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 1)],
                max_inputs: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "liquidIronFuel",
            accesses: vec![BusAccess { client: "1a", inv_addr: "tconstruct:heater_0", bus_addr: "minecraft:barrel_1" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Charcoal"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "liquidIronSpout",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_26", bus_addr: "minecraft:barrel_1" }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Electron Tube"), n_wanted: 64 }],
                inputs: vec![BufferedInput::new(Filter::Label("Polished Rose Quartz"), 1)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "precisionA",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:deployer_4", bus_addr: "minecraft:barrel_1" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Electron Tube"), 1, vec![0])],
                max_per_slot: 16,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "precisionB",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:deployer_5", bus_addr: "minecraft:barrel_1" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Electron Tube"), 1, vec![0])],
                max_per_slot: 16,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "precision",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_20", bus_addr: "minecraft:barrel_1" }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Precision Mechanism"), n_wanted: 64 }],
                inputs: vec![BufferedInput::new(Filter::Label("item.kubejs.kinetic_mechanism"), 1)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "sellWood",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_press_1",
                bus_addr: "minecraft:barrel_1",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 2)),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Silver Coin"), n_wanted: 64 }],
                inputs: vec![SlottedInput::new(Filter::Label("Carrot Crate"), 1, vec![0])],
                max_per_slot: 16,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "buyCopper",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_press_0",
                bus_addr: "minecraft:barrel_1",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 2)),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Copper Ingot"), n_wanted: 64 }],
                inputs: vec![SlottedInput::new(Filter::Label("Silver Coin"), 16, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "buyZinc",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_press_3",
                bus_addr: "minecraft:barrel_1",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 2)),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Zinc Ingot"), n_wanted: 64 }],
                inputs: vec![SlottedInput::new(Filter::Label("Silver Coin"), 16, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "buyGoldCoin",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_press_4",
                bus_addr: "minecraft:barrel_1",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 2)),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Gold Coin"), n_wanted: 64 }],
                inputs: vec![SlottedInput::new(Filter::Label("Silver Coin"), 64, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "liquidCopper",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_30", bus_addr: "minecraft:barrel_1" }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![BufferedRecipe {
                outputs: vec![],
                inputs: vec![BufferedInput::new(Filter::Label("Copper Ingot"), 1)],
                max_inputs: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "liquidCopperFuel",
            accesses: vec![BusAccess { client: "1a", inv_addr: "tconstruct:heater_1", bus_addr: "minecraft:barrel_1" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Charcoal"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "liquidZinc",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_31", bus_addr: "minecraft:barrel_1" }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![BufferedRecipe {
                outputs: vec![],
                inputs: vec![BufferedInput::new(Filter::Label("Zinc Ingot"), 1)],
                max_inputs: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "liquidZincFuel",
            accesses: vec![BusAccess { client: "1a", inv_addr: "tconstruct:heater_2", bus_addr: "minecraft:barrel_1" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Charcoal"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "storagedrawers:standard_drawers_1_1",
                bus_addr: "minecraft:barrel_1",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Brass Ingot"), n_wanted: 64 }],
        });
        factory.add_process(BufferedConfig {
            name: "1x1",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_33", bus_addr: "minecraft:barrel_1" }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Certus Quartz Seed"), n_wanted: 64 }],
                inputs: vec![BufferedInput::new(Filter::Label("Pure Certus Quartz Crystal"), 1)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "crushingWheel",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_34", bus_addr: "minecraft:barrel_1" }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Sand"), n_wanted: 64 }],
                inputs: vec![BufferedInput::new(Filter::Label("Gravel"), 1)],
                max_inputs: i32::MAX,
            }],
        });
    })
}
