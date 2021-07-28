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
        bus_accesses: vec![BasicAccess { client: "1a", addr: "enderstorage:ender_chest_0" }],
        backups: vec![],
    }
    .build(|factory| {
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "colossalchests:interface_1",
                bus_addr: "enderstorage:ender_chest_0",
            }],
        });
        factory.add_storage(DrawerConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "storagedrawers:controller_0",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            filters: vec![
                Filter::Label("Netherrack"),
                Filter::Label("Coal"),
                Filter::Label("Carrot"),
                Filter::Label("Potato"),
                Filter::Label("Fluorite"),
                Filter::Label("Cobblestone"),
                Filter::Label("Redstone"),
                Filter::Label("Gold Ingot"),
                Filter::Label("Iron Ingot"),
                Filter::Label("Osmium Ingot"),
                Filter::Label("Tin Ingot"),
                Filter::Label("Block of Steel"),
            ],
        });
        factory.add_process(BufferedConfig {
            name: "mainOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "enderstorage:ender_chest_1",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            max_recipe_inputs: 0,
            recipes: vec![],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "blastFurnaceOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:dropper_16",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            max_recipe_inputs: 0,
            recipes: vec![],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "furnaceOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:dropper_10",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            max_recipe_inputs: 0,
            recipes: vec![],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "sawOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:dropper_23",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            max_recipe_inputs: 0,
            recipes: vec![],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "platePressOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:dropper_15",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            max_recipe_inputs: 0,
            recipes: vec![],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "gearPressOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:dropper_20",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            max_recipe_inputs: 0,
            recipes: vec![],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "plantGathererOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:dropper_13",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            max_recipe_inputs: 0,
            recipes: vec![],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "crushingWheelOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:dropper_26",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            max_recipe_inputs: 0,
            recipes: vec![],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "pureDaisyOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "modularrouters:item_router_3",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            max_recipe_inputs: 0,
            recipes: vec![],
            stocks: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "creosoteOven",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "immersiveengineering:cokeoven_3",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Redwood Log"), 1, vec![0])],
                max_per_slot: 4,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "cokeOven",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "immersiveengineering:cokeoven_0",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Coal Coke"), n_wanted: 256 }],
                inputs: vec![SlottedInput::new(Filter::Label("Coal"), 1, vec![0])],
                max_per_slot: 4,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "blastFurnace",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:dropper_17",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Steel Ingot"), n_wanted: 256 }],
                inputs: vec![
                    BufferedInput::new(Filter::Label("Charcoal"), 2),
                    BufferedInput::new(Filter::Label("Iron Ingot"), 1),
                ],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 6,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "plantSower",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "modularrouters:item_router_2",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 8,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Redwood Log"), n_wanted: 1024 }],
                inputs: vec![BufferedInput::new(Filter::Label("Redwood Sapling"), 1)],
                max_inputs: i32::MAX,
            }],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "furnace",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "modularrouters:item_router_6",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 12,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Raw Uraninite"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Tin Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Zinc Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Lead Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Copper Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Osmium Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Silver Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Nickel Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Bismuth Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Uranium Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Aluminum Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Platinum Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Baked Potato"), n_wanted: 256 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Potato"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Charcoal"), n_wanted: 256 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Redwood Log"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Stone"), n_wanted: 256 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cobblestone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Glass"), n_wanted: 256 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sand"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Refined Iron Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "enrichment",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "modularrouters:item_router_5",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 12,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Tin Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Zinc Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Uranium Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Osmium Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Platinum Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Copper Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Aluminum Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Silver Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Bismuth Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Nickel Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Lead Ore"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "platePress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:dropper_12",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 4,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gold Plate"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Plate"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Plate"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Copper Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Plate"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Steel Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Zinc Plate"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Zinc Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "gearPress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:dropper_19",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 8,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gold Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminum Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Aluminum Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Copper Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Steel Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
            ],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "saw",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:dropper_24",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 4,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Stripped Redwood Log"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Redwood Log"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redwood Planks"), n_wanted: 256 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Stripped Redwood Log"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "creosoteMixer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:dropper_25",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 8,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Treated Wood Planks"), n_wanted: 256 }],
                inputs: vec![BufferedInput::new(Filter::Label("Redwood Planks"), 1)],
                max_inputs: i32::MAX,
            }],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "crushingWheel",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:dropper_27",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 8,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Coke Dust"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Coal Coke"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gravel"), n_wanted: 256 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cobblestone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sand"), n_wanted: 256 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gravel"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("White Dye"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lily of the Valley"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "crafterA",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftoolsutility:crafter3_0",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            slot_filter: Some(Box::new(|slot| 9 <= slot && slot < 36)),
            to_extract: Some(Box::new(|slot, _| slot >= 36)),
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("White Concrete Powder"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("White Dye"), 1),
                        BufferedInput::new(Filter::Label("Gravel"), 4),
                        BufferedInput::new(Filter::Label("Sand"), 4),
                    ],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Stick"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Redwood Planks"), 2)],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Torch"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Stick"), 1),
                        BufferedInput::new(Filter::Label("Redstone Dust"), 1),
                    ],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Repeater"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Stone"), 3),
                        BufferedInput::new(Filter::Label("Redstone Dust"), 1),
                        BufferedInput::new(Filter::Label("Redstone Torch"), 2),
                    ],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Black Portuguese Pavement"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Livingrock"), 1),
                        BufferedInput::new(Filter::Label("Cobblestone"), 1),
                        BufferedInput::new(Filter::Label("Gravel"), 1),
                        BufferedInput::new(Filter::Label("Coal"), 1),
                    ],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Piston"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Black Portuguese Pavement"), 4),
                        BufferedInput::new(Filter::Label("Treated Wood Planks"), 3),
                        BufferedInput::new(Filter::Label("Redstone Repeater"), 1),
                        BufferedInput::new(Filter::Label("Iron Gear"), 1),
                    ],
                    max_inputs: 64,
                },
            ],
            stocks: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "waterSolidify",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "cyclic:solidifier_0",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            input_slots: vec![0, 1, 2],
            to_extract: extract_all(),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("White Concrete"), n_wanted: 64 }],
                inputs: vec![SlottedInput::new(Filter::Label("White Concrete Powder"), 3, vec![0, 1, 2])],
                max_per_slot: 4,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "pureDaisy",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "cyclic:structure_1",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Livingrock"), n_wanted: 256 }],
                    inputs: vec![SlottedInput::new(Filter::Label("White Concrete"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Livingwood"), n_wanted: 256 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Treated Wood Planks"), 1, vec![0])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "reactor",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "enderstorage:ender_chest_3",
                bus_addr: "enderstorage:ender_chest_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            recipes: vec![BufferedRecipe {
                outputs: vec![],
                inputs: vec![BufferedInput::new(Filter::Label("Uraninite"), 1)],
                max_inputs: i32::MAX,
            }],
            stocks: vec![],
        });
    })
}
