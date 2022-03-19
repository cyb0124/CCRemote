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
        bus_accesses: vec![BasicAccess { client: "1a", addr: "ironchest:diamond_chest_31" }],
        backups: vec![],
    }
    .build(|factory| {
        for inv_addr in [
            "ironchest:diamond_chest_27",
            "ironchest:diamond_chest_28",
            "ironchest:diamond_chest_29",
            "ironchest:diamond_chest_30",
        ] {
            factory.add_storage(ChestConfig {
                accesses: vec![BusAccess { client: "1a", inv_addr, bus_addr: "ironchest:diamond_chest_31" }],
            })
        }
        factory.add_process(LowAlert::new(Filter::Label("Coal Chunk"), 16, None));
        factory.add_process(LowAlert::new(Filter::Label("Lead Chunk"), 16, None));
        factory.add_process(LowAlert::new(Filter::Label("Copper Chunk"), 16, None));
        factory.add_process(LowAlert::new(Filter::Label("Silver Chunk"), 16, None));
        factory.add_process(LowAlert::new(Filter::Label("Aluminum Chunk"), 16, None));
        factory.add_process(LowAlert::new(Filter::Label("Uranium Chunk"), 16, None));
        factory.add_process(LowAlert::new(Filter::Label("Nickel Chunk"), 16, None));
        factory.add_process(LowAlert::new(Filter::Label("Zinc Chunk"), 16, None));
        factory.add_process(LowAlert::new(Filter::Label("Tin Chunk"), 16, None));
        factory.add_process(LowAlert::new(Filter::Label("Netherrack"), 16, None));
        factory.add_process(SyncAndRestockConfig {
            name: "farm",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_24",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            accesses_in: vec![RedstoneAccess { client: "1a", addr: None, side: LEFT, bit: None }],
            accesses_out: vec![RedstoneAccess { client: "1a", addr: None, side: BACK, bit: None }],
            stocks: Box::new(|_| vec![BufferedInput::new(Filter::Label("Acacia Sapling"), 64)]),
            hold_if_unfilled: false,
        });
        factory.add_process(BufferedConfig {
            name: "crushingWheelOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_18",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "gridCraftingOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_40",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "trash",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:dropper_3",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![
                BufferedInput::new(Filter::Label("Industrial Hemp Fiber"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Industrial Hemp Seeds"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Andesite Cobblestone"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Granite Cobblestone"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Drop of Glycerol"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("Acacia Blossom"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Acacia Sapling"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Cobblestone"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Chicken Egg"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Acacia Log"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Sugar Cane"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Limesand"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Potato"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Stick"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Flint"), i32::MAX).extra_backup(512),
            ],
        });
        factory.add_process(BufferedConfig {
            name: "biodiesel",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_19",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![
                BufferedInput::new(Filter::Label("Sugar"), 64),
                BufferedInput::new(Filter::Label("Charcoal"), 64),
                BufferedInput::new(Filter::Label("Potato Seeds"), 64),
            ],
        });
        factory.add_process(BufferedConfig {
            name: "generators",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_26",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![
                BufferedInput::new(Filter::Label("Charcoal"), 64),
                BufferedInput::new(Filter::Label("Cobblestone"), 64),
                BufferedInput::new(Filter::Label("Light Gray Concrete Powder"), 64),
                BufferedInput::new(Filter::Label("Compressed Iron Ingot"), 64),
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crushingWheels",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_38",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gravel"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cobblestone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sand"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gravel"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Coal"), n_wanted: 16 },
                        Output { item: Filter::Label("Sulfur Dust"), n_wanted: 16 },
                    ],
                    inputs: vec![BufferedInput::new(Filter::Label("Coal Chunk"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Bone Meal"), n_wanted: 16 },
                        Output { item: Filter::Label("Limesand"), n_wanted: 16 },
                    ],
                    inputs: vec![BufferedInput::new(Filter::Label("Sand"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("White Dye"), n_wanted: 16 },
                        Output { item: Filter::Label("Light Gray Dye"), n_wanted: 16 },
                    ],
                    inputs: vec![BufferedInput::new(Filter::Label("Bone Meal"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sugar"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sugar Cane"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Coal Coke Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Coal Coke"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cinder Flour"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Netherrack"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Red Sand"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Granite"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crushed Lead Ore"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lead Chunk"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crushed Copper Ore"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Copper Chunk"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crushed Silver Ore"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Silver Chunk"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crushed Aluminum Ore"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Aluminum Chunk"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crushed Uranium Ore"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Uranium Chunk"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crushed Nickel Ore"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nickel Chunk"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crushed Zinc Ore"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Zinc Chunk"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crushed Tin Ore"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Tin Chunk"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 16,
            stocks: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "creosoteMixer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "create:basin_1",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![0, 1, 2],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("White Concrete Powder"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("White Dye"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Sand"), 4, vec![1]),
                        SlottedInput::new(Filter::Label("Gravel"), 4, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Light Gray Concrete Powder"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Light Gray Dye"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Sand"), 4, vec![1]),
                        SlottedInput::new(Filter::Label("Gravel"), 4, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Treated Wood Planks"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Acacia Planks"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("String"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Industrial Hemp Fiber"), 3, vec![0])],
                    max_per_slot: 15,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "waterFan",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_10",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Crushed Iron Ore"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Crushed Gold Ore"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Clay Ball"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sand"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("White Concrete"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("White Concrete Powder"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Nugget"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gravel"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gold Nugget"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Red Sand"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Nugget"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Crushed Lead Ore"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Nugget"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Crushed Copper Ore"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silver Nugget"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Crushed Silver Ore"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminum Nugget"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Crushed Aluminum Ore"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Uranium Nugget"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Crushed Uranium Ore"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nickel Nugget"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Crushed Nickel Ore"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Zinc Nugget"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Crushed Zinc Ore"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Tin Nugget"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Crushed Tin Ore"), 16)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 64,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "lavaFan",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:dropper_2",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("White Rockwool"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Slag"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Slag"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gravel"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Glass"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sand"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Stone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cobblestone"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Smooth Stone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Stone"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Charcoal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Acacia Log"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Granite"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Granite Cobblestone"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Andesite"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Andesite Cobblestone"), 16)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 64,
            stocks: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "slottedPacker",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "create:basin_3",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![0, 1, 2],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Blaze Cake Base"), n_wanted: 16 }],
                inputs: vec![
                    SlottedInput::new(Filter::Label("Chicken Egg"), 1, vec![0]),
                    SlottedInput::new(Filter::Label("Sugar"), 1, vec![1]),
                    SlottedInput::new(Filter::Label("Cinder Flour"), 1, vec![2]),
                ],
                max_per_slot: 8,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "press",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_15",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Tin Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Tin Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gold Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lead Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Brass Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Brass Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Copper Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminum Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Aluminum Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "1x1",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_22",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Potato Seeds"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Potato"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Invar Nugget"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Invar Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_25",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Plastic Sheet"), n_wanted: 16 }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_bottler_2",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Reinforced Stone"), n_wanted: 16 }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:hopper_0",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Obsidian"), n_wanted: 16 }],
        });
        factory.add_process(SlottedConfig {
            name: "rodPress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_press_2",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gold Rod"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Gold Ingot"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Rod"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lead Ingot"), 1, vec![0])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "gearPress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_press_4",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Gear"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Iron Ingot"), 4, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Invar Gear"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Invar Ingot"), 4, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bronze Gear"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Bronze Ingot"), 4, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Compressed Iron Gear"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed Iron Ingot"), 4, vec![0])],
                    max_per_slot: 32,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "furnace",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_furnace_1",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Baked Potato"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Potato"), 1, vec![0])],
                max_per_slot: 8,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "inductionSmelter",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_smelter_0",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![0, 1, 2],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Andesite Alloy"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Iron Nugget"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Andesite"), 1, vec![1]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Coal Coke Dust"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Iron Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bronze Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Copper Ingot"), 3, vec![0]),
                        SlottedInput::new(Filter::Label("Tin Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Seared Brick"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Clay Ball"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Gravel"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Sand"), 1, vec![2]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Constantan Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Copper Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Nickel Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cured Rubber"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Sulfur Dust"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Dry Rubber"), 2, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Invar Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Iron Ingot"), 2, vec![0]),
                        SlottedInput::new(Filter::Label("Nickel Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electrum Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Gold Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Silver Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Brass Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Copper Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Zinc Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "pulverizer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_pulverizer_0",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Silicon"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Limesand"), 1, vec![0])],
                max_per_slot: 8,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "pyrolyzer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_pyrolyzer_2",
                bus_addr: "ironchest:diamond_chest_31",
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
            accesses: vec![RedstoneAccess { client: "1a", addr: None, side: BOTTOM, bit: None }],
            output: emit_when_want_item(
                "pyrolyzer",
                vec![
                    Output { item: Filter::Label("Coal Coke"), n_wanted: 16 },
                    Output { item: Filter::Label("Tar"), n_wanted: 16 },
                ],
            ),
        });
        factory.add_process(BufferedConfig {
            name: "saw",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_34",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Stripped Acacia Log"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Acacia Log"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Acacia Planks"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Stripped Acacia Log"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 16,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "bufferedPacker",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_35",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Nugget"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gold Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Nugget"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lead Nugget"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Copper Nugget"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silver Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Silver Nugget"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminum Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Aluminum Nugget"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Uranium Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Uranium Nugget"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nickel Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nickel Nugget"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Zinc Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Zinc Nugget"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Tin Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Tin Nugget"), 9)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 64,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "lavaSpout",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_36",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Blaze Cake"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Blaze Cake Base"), 1)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 16,
            stocks: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "pressureChamber",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_37",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![0, 1, 2],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Compressed Iron Ingot"), n_wanted: 16 }],
                inputs: vec![
                    SlottedInput::new(Filter::Label("Obsidian"), 1, vec![0]),
                    SlottedInput::new(Filter::Label("Steel Ingot"), 2, vec![1]),
                    SlottedInput::new(Filter::Label("Tar"), 2, vec![2]),
                ],
                max_per_slot: 16,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "wirePress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_press_5",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Wire"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lead Plate"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Wire"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Copper Plate"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminium Wire"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Aluminum Plate"), 1, vec![0])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "latexTransposer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_bottler_3",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Dry Rubber"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Sulfur Dust"), 1, vec![0])],
                max_per_slot: 8,
            }],
        });
        factory.add_process(MultiInvSlottedConfig {
            name: "gridCrafting",
            input_slots: vec![vec![0], vec![0], vec![0], vec![0], vec![0], vec![0], vec![0], vec![0], vec![0]],
            accesses: vec![MultiInvAccess {
                client: "1a",
                inv_addrs: vec![
                    "minecraft:barrel_41",
                    "minecraft:barrel_44",
                    "minecraft:barrel_47",
                    "minecraft:barrel_42",
                    "minecraft:barrel_45",
                    "minecraft:barrel_48",
                    "minecraft:barrel_43",
                    "minecraft:barrel_46",
                    "minecraft:barrel_49",
                ],
                bus_addr: "ironchest:diamond_chest_31",
            }],
            to_extract: None,
            recipes: vec![
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Machine Frame"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Iron Plate"), 4, vec![(0, 0), (2, 0), (6, 0), (8, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Glass Pane"), 4, vec![(1, 0), (3, 0), (5, 0), (7, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Iron Gear"), 1, vec![(4, 0)]),
                    ],
                    max_per_slot: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Glass Pane"), n_wanted: 16 }],
                    inputs: vec![MultiInvSlottedInput::new(
                        Filter::Label("Glass"),
                        6,
                        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)],
                    )],
                    max_per_slot: 1,
                },
            ],
        })
    })
}
