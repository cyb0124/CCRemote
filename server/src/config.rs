use super::detail_cache::DetailCache;
use super::factory::{Factory, FactoryConfig};
use super::server::Server;
use super::{access::*, item::*, process::*, recipe::*, storage::*};
use std::{cell::RefCell, rc::Rc, time::Duration};

pub fn build_factory() -> Rc<RefCell<Factory>> {
    /*
        Apiary 1: Dandelion, Shepherd Pie, Deployer
            Shepherd, Lead, Aluminum, Tin, Osmium, Sooty, Copper, Silver, Boo
        Apiary 2: Dandelion, Chorus Flower, Skyslime Cake, Magma Cake
            Diamond, Lapis, Emerald, Blitz, RGB, Glowstone, Ender, Gold, Blaze
        Apiary 3: Dandelion, Warped Fungus, Yellow Lego
            Uranium, Iron, PCB, Quartz, Forest, Nickel, Skeleton, Zinc, Slimy
        Apiary 4: Dandelion, Earthslime Cake, Enderslime Cake, Lectern, Oasis Grass, Fluid Pipe
            Basalz, Blizz, Spelling, Cobalt, Redstone, Dusty, Clogged
    */
    FactoryConfig {
        detail_cache: DetailCache::new(),
        server: Server::new(1847),
        min_cycle_time: Duration::from_secs(1),
        log_clients: vec!["1a"],
        bus_accesses: vec![BasicAccess { client: "1a", addr: "ironchest:diamond_chest_31" }],
        backups: vec![
            (Filter::Label("Flourishing Archwood Sapling"), 8),
            (Filter::Label("Mystical Black Flower"), 8),
            (Filter::Label("Mystical Green Flower"), 8),
            (Filter::Label("Cumin Seeds"), 8),
            (Filter::Label("Dandelion"), 8),
            (Filter::Label("Potato"), 8),
            (Filter::Label("Grass"), 8),
            (Filter::Label("Rice"), 8),
        ],
    }
    .build(|factory| {
        let air_canister = |full| {
            Filter::Fn(Box::new(move |_, detail| {
                detail.label == "Air Canister"
                    && detail.others.get(&super::lua_value::Key::from("durability")).is_none() == full
            }))
        };
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "occultism:storage_controller_1",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            override_max_stack_size: Some(1024),
        });
        factory.add_process(LowAlert::new(Filter::Label("Netherrack"), 16, None));
        factory.add_process(LowAlert::new(Filter::Label("Soul Shard"), 16, None));
        factory.add_process(SyncAndRestockConfig {
            name: "farm",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_24",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            accesses_in: vec![RedstoneAccess { client: "1a", addr: None, side: LEFT, bit: None }],
            accesses_out: vec![RedstoneAccess { client: "1a", addr: None, side: BACK, bit: None }],
            stocks: Box::new(|_| vec![]),
            hold_if_unfilled: false,
        });
        factory.add_process(BufferedConfig {
            name: "crushingWheelsOutput",
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
                inv_addr: "minecraft:barrel_95",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "manaPoolOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:device_collector_0",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "arsOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_75",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "xpTransposerOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_87",
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
                BufferedInput::new(Filter::Label("Nether Quartz Honeycomb Block"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("Skeleton Honeycomb Block"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("Redstone Honeycomb Block"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("Diamond Honeycomb Block"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("Emerald Honeycomb Block"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("RGBee Honeycomb Block"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("Ender Honeycomb Block"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("Lapis Honeycomb Block"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("Coal Honeycomb Block"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("Iron Honeycomb Block"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("Gold Honeycomb Block"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("block.resourcefulbees.dusty_mummbee_honeycomb_block"), i32::MAX)
                    .extra_backup(1024),
                BufferedInput::new(Filter::Label("block.resourcefulbees.glowstone_honeycomb_block"), i32::MAX)
                    .extra_backup(1024),
                BufferedInput::new(Filter::Label("block.resourcefulbees.spelling_honeycomb_block"), i32::MAX)
                    .extra_backup(1024),
                BufferedInput::new(Filter::Label("block.resourcefulbees.aluminum_honeycomb_block"), i32::MAX)
                    .extra_backup(1024),
                BufferedInput::new(Filter::Label("block.resourcefulbees.clogged_honeycomb_block"), i32::MAX)
                    .extra_backup(1024),
                BufferedInput::new(Filter::Label("block.resourcefulbees.uranium_honeycomb_block"), i32::MAX)
                    .extra_backup(1024),
                BufferedInput::new(Filter::Label("block.resourcefulbees.osmium_honeycomb_block"), i32::MAX)
                    .extra_backup(1024),
                BufferedInput::new(Filter::Label("block.resourcefulbees.cobalt_honeycomb_block"), i32::MAX)
                    .extra_backup(1024),
                BufferedInput::new(Filter::Label("block.resourcefulbees.copper_honeycomb_block"), i32::MAX)
                    .extra_backup(1024),
                BufferedInput::new(Filter::Label("block.resourcefulbees.silver_honeycomb_block"), i32::MAX)
                    .extra_backup(1024),
                BufferedInput::new(Filter::Label("block.resourcefulbees.nickel_honeycomb_block"), i32::MAX)
                    .extra_backup(1024),
                BufferedInput::new(Filter::Label("block.resourcefulbees.boobee_honeycomb_block"), i32::MAX)
                    .extra_backup(1024),
                BufferedInput::new(Filter::Label("block.resourcefulbees.basalz_honeycomb_block"), i32::MAX)
                    .extra_backup(1024),
                BufferedInput::new(Filter::Label("block.resourcefulbees.slimy_honeycomb_block"), i32::MAX)
                    .extra_backup(1024),
                BufferedInput::new(Filter::Label("block.resourcefulbees.blaze_honeycomb_block"), i32::MAX)
                    .extra_backup(1024),
                BufferedInput::new(Filter::Label("block.resourcefulbees.blitz_honeycomb_block"), i32::MAX)
                    .extra_backup(1024),
                BufferedInput::new(Filter::Label("block.resourcefulbees.blizz_honeycomb_block"), i32::MAX)
                    .extra_backup(1024),
                BufferedInput::new(Filter::Label("block.resourcefulbees.pcbee_honeycomb_block"), i32::MAX)
                    .extra_backup(1024),
                BufferedInput::new(Filter::Label("block.resourcefulbees.zinc_honeycomb_block"), i32::MAX)
                    .extra_backup(1024),
                BufferedInput::new(Filter::Label("block.resourcefulbees.lead_honeycomb_block"), i32::MAX)
                    .extra_backup(1024),
                BufferedInput::new(Filter::Label("block.resourcefulbees.tin_honeycomb_block"), i32::MAX)
                    .extra_backup(1024),
                BufferedInput::new(Filter::Label("item.resourcefulbees.shepherd_honeycomb"), i32::MAX)
                    .extra_backup(16384),
                BufferedInput::new(Filter::Label("item.resourcefulbees.forest_honeycomb"), i32::MAX)
                    .extra_backup(16384),
                BufferedInput::new(Filter::Label("White Corundum Cluster"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("Industrial Hemp Fiber"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("Industrial Hemp Seeds"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("Andesite Cobblestone"), i32::MAX).extra_backup(16384),
                BufferedInput::new(Filter::Label("Komodo Dragon Spit"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("Drop of Glycerol"), i32::MAX).extra_backup(16384),
                BufferedInput::new(Filter::Label("White Corundum"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("Pepper Seeds"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("Cobblestone"), i32::MAX).extra_backup(16384),
                BufferedInput::new(Filter::Label("Shiverstone"), i32::MAX).extra_backup(16384),
                BufferedInput::new(Filter::Label("Nether Wart"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("Sugar Cane"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("Magebloom"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("Habanero"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("Pepper"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("Paper"), i32::MAX).extra_backup(16384),
                BufferedInput::new(Filter::Label("Stick"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("Flint"), i32::MAX).extra_backup(1024),
                BufferedInput::new(Filter::Label("Book"), i32::MAX).extra_backup(16384),
                BufferedInput::new(Filter::Label("Kelp"), i32::MAX).extra_backup(1024),
            ],
        });
        factory.add_process(BufferedConfig {
            name: "bufferA",
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
            name: "bufferB",
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
                BufferedInput::new(Filter::Label("Dandelion"), 64),
                BufferedInput::new(Filter::Label("Sand Paper"), 1),
                BufferedInput::new(Filter::Label("Cobblestone"), 64),
                BufferedInput::new(Filter::Label("Compressed Iron Ingot"), 64),
                BufferedInput::new(Filter::Label("Light Gray Concrete Powder"), 64),
            ],
        });
        factory.add_process(BufferedConfig {
            name: "bufferC",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_60",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![
                BufferedInput::new(Filter::Label("Flourishing Archwood Leaves"), 64),
                BufferedInput::new(Filter::Label("Bottle o' Enchanting"), 64),
                BufferedInput::new(Filter::Label("Redstone Dust"), 64),
                BufferedInput::new(Filter::Label("Glass Bottle"), 64),
                BufferedInput::new(Filter::Label("Milk Bottle"), 16),
                BufferedInput::new(Filter::Label("Blaze Cake"), 64),
                BufferedInput::new(Filter::Label("Charcoal"), 64),
            ],
        });
        factory.add_process(BufferedConfig {
            name: "bufferD",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_76",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![BufferedInput::new(Filter::Label("Arrow"), 64)],
        });
        factory.add_process(ItemCycleConfig {
            name: "gourmaryllis",
            file_name: "gourmaryllis.txt",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:dropper_7",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot: 0,
            items: vec![
                ScatteringInput::new(Filter::Label("Cooked Chicken")),
                ScatteringInput::new(Filter::Label("Baked Potato")),
            ],
        });
        factory.add_process(BufferedConfig {
            name: "cuttingBoard",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_93",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Raw Chicken Wings"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Raw Chicken"), 1)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 4,
            stocks: vec![BufferedInput::new(Filter::Label("Iron Knife"), 1)],
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
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Potassium Nitrate Chunk"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Source Chunk"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Sulfur Chunk"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Sulfur"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Rice"), n_wanted: 16 },
                        Output { item: Filter::Label("Straw"), n_wanted: 16 },
                    ],
                    inputs: vec![BufferedInput::new(Filter::Label("Rice Panicle"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Black Dye"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Mystical Black Petal"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Green Dye"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Mystical Green Petal"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Blaze Powder"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Blaze Rod"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Blitz Powder"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Blitz Mote"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basalz Powder"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Basalz Shard"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Blizz Powder"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Blizz Cube"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fluorite"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Fluorite Chunk"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Apatite"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Apatite Chunk"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Mana Powder"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Name("emendatusenigmatica:arcane_gem"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lapis Lazuli Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lapis Lazuli"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Potassium Nitrate Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Potassium Nitrate"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silver Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Silver Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Copper Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Diamond Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Diamond"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ender Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Ender Pearl"), 1)],
                    max_inputs: i32::MAX,
                },
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
                    outputs: vec![Output { item: Filter::Label("Limesand"), n_wanted: 16 }],
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
                    outputs: vec![Output { item: Filter::Label("Crushed Osmium Ore"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Osmium Chunk"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crushed Cobalt Ore"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cobalt Chunk"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crushed Iron Ore"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Chunk"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crushed Gold Ore"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Chunk"), 1)],
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
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Redstone Chunk"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nether Quartz"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Quartz Chunk"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lapis Lazuli"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lapis Lazuli Chunk"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Diamond"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Diamond Chunk"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Emerald"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Emerald Chunk"), 1)],
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
            input_slots: vec![0, 1, 2, 3, 4],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Mango Habanero Wings"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Cooked Chicken Wings"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Habanero"), vec![(1, 1)]),
                        SlottedInput::new(Filter::Label("Mango"), vec![(2, 1)]),
                    ],
                    max_sets: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Plum Pudding"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Chicken Egg"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Milk Bottle"), vec![(1, 1)]),
                        SlottedInput::new(Filter::Label("Sugar"), vec![(2, 1)]),
                        SlottedInput::new(Filter::Label("Plum"), vec![(3, 1)]),
                        SlottedInput::new(Filter::Label("Rice"), vec![(4, 1)]),
                    ],
                    max_sets: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Curry Powder"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Pepper"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Cumin Seeds"), vec![(1, 1)]),
                        SlottedInput::new(Filter::Label("Ginger"), vec![(2, 1)]),
                    ],
                    max_sets: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Phyto-Gro"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Sand"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Potassium Nitrate"), vec![(1, 1)]),
                        SlottedInput::new(Filter::Label("Apatite"), vec![(2, 2)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Blue Rockwool"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("White Rockwool"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Blue Dye"), vec![(1, 1)]),
                    ],
                    max_sets: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sand Paper"), n_wanted: 1 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Paper"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Sand"), vec![(1, 1)]),
                    ],
                    max_sets: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Algal Blend"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Clay Ball"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Kelp"), vec![(1, 1)]),
                    ],
                    max_sets: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gray Dye"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Black Dye"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("White Dye"), vec![(1, 1)]),
                    ],
                    max_sets: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gray Rockwool"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Gray Dye"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("White Rockwool"), vec![(1, 1)]),
                    ],
                    max_sets: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Rose Quartz"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Nether Quartz"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Redstone Dust"), vec![(1, 8)]),
                    ],
                    max_sets: 2,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gunpowder"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Charcoal"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Sulfur Dust"), vec![(1, 1)]),
                        SlottedInput::new(Filter::Label("Potassium Nitrate Dust"), vec![(2, 4)]),
                    ],
                    max_sets: 4,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lightning Charge"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Charcoal"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Gunpowder"), vec![(1, 1)]),
                        SlottedInput::new(Filter::Label("Blitz Powder"), vec![(2, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("White Concrete Powder"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("White Dye"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Sand"), vec![(1, 4)]),
                        SlottedInput::new(Filter::Label("Gravel"), vec![(2, 4)]),
                    ],
                    max_sets: 4,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Light Gray Concrete Powder"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Light Gray Dye"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Sand"), vec![(1, 4)]),
                        SlottedInput::new(Filter::Label("Gravel"), vec![(2, 4)]),
                    ],
                    max_sets: 4,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Treated Wood Planks"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Oak Planks"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("String"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Industrial Hemp Fiber"), vec![(0, 3)])],
                    max_sets: 5,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "blazeMixer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "create:basin_4",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![0, 1, 2],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Dielectric Paste"), n_wanted: 16 }],
                inputs: vec![
                    SlottedInput::new(Filter::Label("Tar"), vec![(0, 2)]),
                    SlottedInput::new(Filter::Label("Sand"), vec![(1, 2)]),
                    SlottedInput::new(Filter::Label("Silicon"), vec![(2, 3)]),
                ],
                max_sets: 5,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "heatedPacker",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "create:basin_5",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![0, 1, 2],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("block.kubejs.rough_machine_frame"), n_wanted: 4 }],
                inputs: vec![
                    SlottedInput::new(Filter::Label("block.kubejs.coated_machine_frame_top"), vec![(0, 1)]),
                    SlottedInput::new(Filter::Label("Machine Base"), vec![(1, 1)]),
                    SlottedInput::new(Filter::Label("Invar Plate"), vec![(2, 2)]),
                ],
                max_sets: 4,
            }],
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
                    outputs: vec![Output { item: Filter::Label("Flint"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gravel"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Nugget"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Crushed Iron Ore"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gold Nugget"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Crushed Gold Ore"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Nugget"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Crushed Lead Ore"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Osmium Nugget"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Crushed Osmium Ore"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cobalt Nugget"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Crushed Cobalt Ore"), 16)],
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
                    outputs: vec![Output { item: Filter::Label("Nether Brick"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Netherrack"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Brick"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Clay Ball"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sunmetal Brick"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sunmetal Blend"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Algal Brick"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Algal Blend"), 16)],
                    max_inputs: i32::MAX,
                },
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
                    inputs: vec![BufferedInput::new(Filter::Label("Oak Log"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Andesite"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Andesite Cobblestone"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Terracotta"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Clay"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Enchanted Ash"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Bone"), 16)],
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
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Blaze Cake Base"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Chicken Egg"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Sugar"), vec![(1, 1)]),
                        SlottedInput::new(Filter::Label("Cinder Flour"), vec![(2, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Reinforced Bricks"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Reinforced Stone"), vec![(0, 4)])],
                    max_sets: 4,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cardboard Box"), n_wanted: 4 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Sawdust"), vec![(0, 4)])],
                    max_sets: 2,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Prismarine"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Prismarine Shard"), vec![(0, 4)])],
                    max_sets: 2,
                },
            ],
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
                    outputs: vec![Output { item: Filter::Label("Invar Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Invar Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Steel Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Copper Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electrum Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Electrum Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Signalum Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Signalum Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminum Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Aluminum Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Constantan Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Constantan Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Thermo Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Thermoelectric Generator"), 1)],
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
                    outputs: vec![Output { item: Filter::Label("Mystical Black Petal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Mystical Black Flower"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Mystical Green Petal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Mystical Green Flower"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Potato Seeds"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Potato"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Blue Dye"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lapis Lazuli"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Invar Nugget"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Invar Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Signalum Nugget"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Signalum Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Oak Button"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Oak Planks"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "sandPaperDeployer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_53",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Polished Rose Quartz"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Rose Quartz"), 1)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "etchingAcidSpout",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_92",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Etching Acid Bucket"), n_wanted: 1 }],
                inputs: vec![BufferedInput::new(Filter::Label("Bucket"), 1)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 1,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "buildersTeaSpout",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_94",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Builder's Tea"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Glass Bottle"), 1)],
                max_inputs: i32::MAX,
            }],
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
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ars_nouveau:crystallizer_1",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Name("emendatusenigmatica:arcane_gem"), n_wanted: 16 }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "create:depot_0",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Lithium Dust"), n_wanted: 16 }],
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
                    inputs: vec![SlottedInput::new(Filter::Label("Gold Ingot"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Rod"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lead Ingot"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Rod"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Steel Ingot"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Brass Rod"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Brass Ingot"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Rod"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Copper Ingot"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminum Rod"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Aluminum Ingot"), vec![(0, 1)])],
                    max_sets: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "coinPress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_press_7",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Tin Coin"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Tin Nugget"), vec![(0, 3)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lumium Coin"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lumium Ingot"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Enderium Coin"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Enderium Ingot"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basic Processor"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Raw Basic Processor"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Improved Processor"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Raw Improved Processor"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Advanced Processor"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Raw Advanced Processor"), vec![(0, 1)])],
                    max_sets: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "gearPress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_press_6",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Gear"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Iron Ingot"), vec![(0, 4)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Gear"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lead Ingot"), vec![(0, 4)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Invar Gear"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Invar Ingot"), vec![(0, 4)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Gear"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Copper Ingot"), vec![(0, 4)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Osmium Gear"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Osmium Ingot"), vec![(0, 4)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lumium Gear"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lumium Ingot"), vec![(0, 4)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bronze Gear"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Bronze Ingot"), vec![(0, 4)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electrum Gear"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Electrum Ingot"), vec![(0, 4)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Constantan Gear"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Constantan Ingot"), vec![(0, 4)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Compressed Iron Gear"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed Iron Ingot"), vec![(0, 4)])],
                    max_sets: 8,
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
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Baked Potato"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Potato"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cooked Chicken"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Raw Chicken"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cooked Chicken Wings"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Raw Chicken Wings"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dried Kelp"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Kelp"), vec![(0, 1)])],
                    max_sets: 8,
                },
            ],
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
                    outputs: vec![Output { item: Filter::Label("Reinforced Alloy"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Copper Ingot"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Aluminum Ingot"), vec![(1, 3)]),
                        SlottedInput::new(Filter::Label("Lithium Dust"), vec![(2, 4)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lumium Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Silver Ingot"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Glowstone Dust"), vec![(1, 2)]),
                        SlottedInput::new(Filter::Label("Tin Ingot"), vec![(2, 3)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Enderium Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Diamond Dust"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Ender Dust"), vec![(1, 2)]),
                        SlottedInput::new(Filter::Label("Lead Ingot"), vec![(2, 3)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Hardened Glass"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Nether Quartz"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Obsidian"), vec![(1, 1)]),
                        SlottedInput::new(Filter::Label("Sand"), vec![(2, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Andesite Alloy"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Iron Nugget"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Andesite"), vec![(1, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Coal Coke Dust"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Iron Ingot"), vec![(1, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bronze Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Copper Ingot"), vec![(0, 3)]),
                        SlottedInput::new(Filter::Label("Tin Ingot"), vec![(1, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Tinker's Bronze Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Copper Ingot"), vec![(0, 3)]),
                        SlottedInput::new(Filter::Label("Glass"), vec![(1, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Froststeel Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Blizz Powder"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Cobalt Ingot"), vec![(1, 3)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Seared Brick"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Clay Ball"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Gravel"), vec![(1, 1)]),
                        SlottedInput::new(Filter::Label("Sand"), vec![(2, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Constantan Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Copper Ingot"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Nickel Ingot"), vec![(1, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cured Rubber"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Sulfur Dust"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Dry Rubber"), vec![(1, 2)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Invar Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Iron Ingot"), vec![(0, 2)]),
                        SlottedInput::new(Filter::Label("Nickel Ingot"), vec![(1, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electrum Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Gold Ingot"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Silver Ingot"), vec![(1, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Brass Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Copper Ingot"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Zinc Ingot"), vec![(1, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pewter Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Iron Ingot"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Lead Ingot"), vec![(1, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Signalum Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Silver Ingot"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Copper Ingot"), vec![(1, 3)]),
                        SlottedInput::new(Filter::Label("Redstone Dust"), vec![(2, 4)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Quartz Enriched Iron"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Compressed Iron Ingot"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Nether Quartz"), vec![(1, 1)]),
                    ],
                    max_sets: 8,
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
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bone Meal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Bone"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silicon"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Limesand"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Potassium Nitrate"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Sandstone"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sawdust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Oak Log"), vec![(0, 1)])],
                    max_sets: 8,
                },
            ],
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
                inputs: vec![SlottedInput::new(Filter::Label("Coal"), vec![(0, 1)])],
                max_sets: 64,
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
                    outputs: vec![Output { item: Filter::Label("Stripped Oak Log"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Oak Log"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Oak Planks"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Stripped Oak Log"), 1)],
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
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("item.resourcefulbees.dusty_mummbee_honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("item.resourcefulbees.glowstone_honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("item.resourcefulbees.spelling_honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("item.resourcefulbees.aluminum_honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("item.resourcefulbees.clogged_honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("item.resourcefulbees.uranium_honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("item.resourcefulbees.osmium_honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("item.resourcefulbees.cobalt_honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("item.resourcefulbees.copper_honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("item.resourcefulbees.silver_honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("item.resourcefulbees.nickel_honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("item.resourcefulbees.boobee_honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("item.resourcefulbees.pcbee_honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("item.resourcefulbees.slimy_honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("item.resourcefulbees.blaze_honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("item.resourcefulbees.blitz_honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("item.resourcefulbees.basalz_honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("item.resourcefulbees.blizz_honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("item.resourcefulbees.lead_honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("item.resourcefulbees.zinc_honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("item.resourcefulbees.tin_honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Skeleton Honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("RGBee Honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Ender Honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Coal Honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Lapis Honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Diamond Honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Emerald Honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Redstone Honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Quartz Honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output {
                        item: Filter::Label("block.resourcefulbees.forest_honeycomb_block"),
                        n_wanted: 16,
                    }],
                    inputs: vec![BufferedInput::new(Filter::Label("item.resourcefulbees.forest_honeycomb"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Flourishing Archwood Wood"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Flourishing Archwood Log"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Turf"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Grass"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Clay"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Clay Ball"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sandstone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sand"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Redstone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Redstone Dust"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Steel"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Steel Ingot"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Iron"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Copper"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Copper Ingot"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Electrum"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Electrum Ingot"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Compressed Iron"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Compressed Iron Ingot"), 9)],
                    max_inputs: i32::MAX,
                },
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
                    outputs: vec![Output { item: Filter::Label("Osmium Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Osmium Nugget"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cobalt Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cobalt Nugget"), 9)],
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
                inv_addr: "minecraft:hopper_2",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![0, 1, 2, 3, 4],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Turbine Blade"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Redstone Servo"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Invar Plate"), vec![(1, 1)]),
                        SlottedInput::new(Filter::Label("Flux Dust"), vec![(2, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Compressed Iron Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Obsidian"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Steel Ingot"), vec![(1, 2)]),
                        SlottedInput::new(Filter::Label("Tar"), vec![(2, 2)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empty PCB"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Copper Plate"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Plastic Sheet"), vec![(1, 2)]),
                        SlottedInput::new(Filter::Label("Flux Dust"), vec![(2, 6)]),
                    ],
                    max_sets: 2,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Raw Basic Processor"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Processor Binding"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Flux Dust"), vec![(1, 1)]),
                        SlottedInput::new(Filter::Label("Tin Coin"), vec![(2, 1)]),
                        SlottedInput::new(Filter::Label("Silicon"), vec![(3, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Raw Improved Processor"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Processor Binding"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Flux Dust"), vec![(1, 1)]),
                        SlottedInput::new(Filter::Label("Lumium Coin"), vec![(2, 1)]),
                        SlottedInput::new(Filter::Label("Silicon"), vec![(3, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Raw Advanced Processor"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Processor Binding"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Flux Dust"), vec![(1, 1)]),
                        SlottedInput::new(Filter::Label("Enderium Coin"), vec![(2, 1)]),
                        SlottedInput::new(Filter::Label("Silicon"), vec![(3, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Transistor"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Plastic Sheet"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Electrum Wire"), vec![(1, 1)]),
                        SlottedInput::new(Filter::Label("Dielectric Paste"), vec![(2, 1)]),
                        SlottedInput::new(Filter::Label("Raw Basic Processor"), vec![(3, 2)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Capacitor"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Plastic Sheet"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Electrum Wire"), vec![(1, 1)]),
                        SlottedInput::new(Filter::Label("Aluminum Plate"), vec![(2, 1)]),
                        SlottedInput::new(Filter::Label("Signalum Plate"), vec![(3, 1)]),
                        SlottedInput::new(Filter::Label("Dielectric Paste"), vec![(4, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Printed Circuit Board"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Lead Wire"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("Basic Processor"), vec![(1, 1)]),
                        SlottedInput::new(Filter::Label("Unassembled PCB"), vec![(2, 1)]),
                        SlottedInput::new(Filter::Label("Transistor"), vec![(3, 2)]),
                        SlottedInput::new(Filter::Label("Capacitor"), vec![(4, 2)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("item.kubejs.memory_advanced_empty"), n_wanted: 4 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Flux Dust"), vec![(0, 1)]),
                        SlottedInput::new(Filter::Label("item.kubejs.memory_basic_empty"), vec![(1, 2)]),
                    ],
                    max_sets: 4,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "assembly",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_59",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Unassembled PCB"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Empty PCB"), vec![(0, 1)])],
                    max_sets: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("block.kubejs.rough_machine_frame_top"), n_wanted: 4 }],
                    inputs: vec![SlottedInput::new(Filter::Name("thermal:machine_frame"), vec![(0, 2)])],
                    max_sets: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("rftoolsbase:machine_frame"), n_wanted: 4 }],
                    inputs: vec![SlottedInput::new(Filter::Label("block.kubejs.rough_machine_frame"), vec![(0, 1)])],
                    max_sets: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("CPU Core B500"), n_wanted: 1 }],
                    inputs: vec![SlottedInput::new(Filter::Label("item.kubejs.cpu_core_500_package"), vec![(0, 1)])],
                    max_sets: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("CPU Core S1000"), n_wanted: 1 }],
                    inputs: vec![SlottedInput::new(Filter::Label("item.kubejs.cpu_core_1000_package"), vec![(0, 1)])],
                    max_sets: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("CPU Core EX2000"), n_wanted: 1 }],
                    inputs: vec![SlottedInput::new(Filter::Label("item.kubejs.cpu_core_2000_package"), vec![(0, 1)])],
                    max_sets: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("item.kubejs.memory_basic_empty"), n_wanted: 4 }],
                    inputs: vec![SlottedInput::new(Filter::Label("item.kubejs.basic_memory_package"), vec![(0, 1)])],
                    max_sets: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basic Control Circuit"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("item.kubejs.basic_circuit_package"), vec![(0, 1)])],
                    max_sets: 1,
                },
            ],
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
                    inputs: vec![SlottedInput::new(Filter::Label("Lead Plate"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Wire"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Steel Plate"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Wire"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Copper Plate"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electrum Wire"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Electrum Plate"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminium Wire"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Aluminum Plate"), vec![(0, 1)])],
                    max_sets: 8,
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
                inputs: vec![SlottedInput::new(Filter::Label("Sulfur Dust"), vec![(0, 1)])],
                max_sets: 8,
            }],
        });
        factory.add_process(MultiInvSlottedConfig {
            name: "gridCrafting",
            input_slots: vec![vec![0], vec![0], vec![0], vec![0], vec![0], vec![0], vec![0], vec![0], vec![0], vec![0]],
            accesses: vec![MultiInvAccess {
                client: "1a",
                inv_addrs: vec![
                    "minecraft:hopper_3",
                    "minecraft:hopper_5",
                    "minecraft:hopper_7",
                    "create:mechanical_crafter_1",
                    "create:mechanical_crafter_2",
                    "create:mechanical_crafter_3",
                    "minecraft:hopper_4",
                    "minecraft:hopper_6",
                    "minecraft:hopper_8",
                    "minecraft:barrel_66",
                ],
                bus_addr: "ironchest:diamond_chest_31",
            }],
            to_extract: None,
            recipes: vec![
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("thermal:machine_frame"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Iron Plate"), vec![(0, 0, 1), (2, 0, 1), (6, 0, 1), (8, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Glass Pane"), vec![(1, 0, 1), (3, 0, 1), (5, 0, 1), (7, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Iron Gear"), vec![(4, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fluid Cell Frame"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Copper Plate"),
                            vec![(0, 0, 1), (2, 0, 1), (6, 0, 1), (8, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Glass Pane"), vec![(1, 0, 1), (3, 0, 1), (5, 0, 1), (7, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Bronze Gear"), vec![(4, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fluid Cell"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Cured Rubber"),
                            vec![(0, 0, 1), (2, 0, 1), (6, 0, 1), (8, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Iron Ingot"), vec![(3, 0, 1), (5, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Fluid Cell Frame"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Hardened Glass"), vec![(1, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Redstone Servo"), vec![(7, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Flux Cell Frame"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Lead Plate"), vec![(0, 0, 1), (2, 0, 1), (6, 0, 1), (8, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Glass Pane"), vec![(1, 0, 1), (3, 0, 1), (5, 0, 1), (7, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Electrum Gear"), vec![(4, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Flux Cell"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Cured Rubber"),
                            vec![(0, 0, 1), (2, 0, 1), (6, 0, 1), (8, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Iron Ingot"), vec![(3, 0, 1), (5, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Redstone Flux Cell Frame"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Block of Redstone"), vec![(1, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Redstone Flux Coil"), vec![(7, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Flux Coil"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Gold Nugget"), vec![(0, 0, 1), (8, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Redstone Dust"),
                            vec![(1, 0, 1), (3, 0, 1), (5, 0, 1), (7, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Gold Rod"), vec![(4, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Servo"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Redstone Dust"),
                            vec![(0, 0, 1), (2, 0, 1), (6, 0, 1), (8, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Lead Rod"), vec![(4, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Torch"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Redstone Dust"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Stick"), vec![(3, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electron Tube"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Polished Rose Quartz"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Redstone Torch"), vec![(3, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Iron Nugget"), vec![(6, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cogwheel"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Andesite Alloy"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Oak Button"),
                            vec![(0, 0, 1), (1, 0, 1), (2, 0, 1), (3, 0, 1), (5, 0, 1), (6, 0, 1), (7, 0, 1), (8, 0, 1)],
                        ),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Large Cogwheel"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Andesite Alloy"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Oak Button"), vec![(0, 0, 1), (2, 0, 1), (6, 0, 1), (8, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Oak Planks"), vec![(1, 0, 1), (3, 0, 1), (5, 0, 1), (7, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Oak Chest"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Oak Planks"),
                            vec![(0, 0, 1), (1, 0, 1), (2, 0, 1), (3, 0, 1), (5, 0, 1), (6, 0, 1), (7, 0, 1), (8, 0, 1)],
                        ),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Hopper"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Oak Chest"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Iron Ingot"),
                            vec![(0, 0, 1), (2, 0, 1), (3, 0, 1), (5, 0, 1), (7, 0, 1)],
                        ),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Blue Connector"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Redstone Dust"), vec![(3, 0, 1), (5, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Blue Rockwool"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Hopper"), vec![(1, 0, 1), (7, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Uranium Ingot"),
                            vec![(0, 0, 1), (2, 0, 1), (6, 0, 1), (8, 0, 1)],
                        ),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Piston"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Gold Ingot"), vec![(1, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Oak Planks"), vec![(0, 0, 1), (2, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Smooth Stone"),
                            vec![(3, 0, 1), (5, 0, 1), (6, 0, 1), (8, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Gold Rod"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Redstone Dust"), vec![(7, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Advanced PCB"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Redstone Dust"),
                            vec![(0, 0, 1), (2, 0, 1), (6, 0, 1), (8, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(
                            Filter::Label("Plastic Sheet"),
                            vec![(1, 0, 1), (3, 0, 1), (5, 0, 1), (7, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Printed Circuit Board"), vec![(4, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Machine Base"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Reinforced Stone Slab"),
                            vec![(6, 0, 1), (7, 0, 1), (8, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Invar Nugget"), vec![(1, 0, 1), (3, 0, 1), (5, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Advanced PCB"), vec![(4, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Bars"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Iron Ingot"),
                            vec![(0, 0, 1), (1, 0, 1), (2, 0, 1), (3, 0, 1), (4, 0, 1), (5, 0, 1)],
                        ),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Glass Pane"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Glass"),
                            vec![(0, 0, 1), (1, 0, 1), (2, 0, 1), (3, 0, 1), (4, 0, 1), (5, 0, 1)],
                        ),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Black Stained Glass Pane"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Black Stained Glass"),
                            vec![(0, 0, 1), (1, 0, 1), (2, 0, 1), (3, 0, 1), (4, 0, 1), (5, 0, 1)],
                        ),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Reinforced Brick Wall"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Reinforced Bricks"),
                            vec![(0, 0, 1), (1, 0, 1), (2, 0, 1), (3, 0, 1), (4, 0, 1), (5, 0, 1)],
                        ),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cobblestone Slab"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Cobblestone"), vec![(0, 0, 1), (1, 0, 1), (2, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Oak Slab"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Oak Planks"), vec![(0, 0, 1), (1, 0, 1), (2, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Reinforced Stone Slab"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Reinforced Stone"), vec![(0, 0, 1), (1, 0, 1), (2, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Otherstone Slab"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Otherstone"), vec![(0, 0, 1), (1, 0, 1), (2, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Andesite Casing"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Oak Planks"),
                            vec![(0, 0, 1), (1, 0, 1), (2, 0, 1), (6, 0, 1), (7, 0, 1), (8, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Andesite Alloy"), vec![(3, 0, 1), (5, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Oak Log"), vec![(4, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Glass Bottle"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Glass"), vec![(0, 0, 1), (2, 0, 1), (4, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bucket"), n_wanted: 1 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Iron Ingot"), vec![(0, 0, 1), (2, 0, 1), (4, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Chicken Egg"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("item.resourcefulbees.shepherd_honeycomb"),
                            vec![(3, 0, 1), (8, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Beeswax"), vec![(7, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Feather"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("item.resourcefulbees.shepherd_honeycomb"),
                            vec![(3, 0, 1), (8, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Beeswax"), vec![(4, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Raw Chicken"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("item.resourcefulbees.shepherd_honeycomb"),
                            vec![(3, 0, 1), (5, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Beeswax"), vec![(4, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Arrow"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Flint"), vec![(1, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Stick"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Feather"), vec![(7, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lightning Arrow"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Lightning Charge"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Arrow"), vec![(1, 0, 1), (3, 0, 1), (5, 0, 1), (7, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Processor Binding"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Slimeball"), vec![(1, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("String"), vec![(0, 0, 1), (2, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pressure Tube"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Hardened Glass"), vec![(1, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Compressed Iron Ingot"), vec![(0, 0, 1), (2, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("LV Wire Coil"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Stick"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Copper Wire"),
                            vec![(1, 0, 1), (3, 0, 1), (5, 0, 1), (7, 0, 1)],
                        ),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Coil Block"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Iron Ingot"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("LV Wire Coil"),
                            vec![(0, 0, 1), (1, 0, 1), (2, 0, 1), (3, 0, 1), (5, 0, 1), (6, 0, 1), (7, 0, 1), (8, 0, 1)],
                        ),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("LV Wire Connector"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Copper Ingot"), vec![(1, 0, 1), (4, 0, 1), (7, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Terracotta"), vec![(3, 0, 1), (5, 0, 1), (6, 0, 1), (8, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("MV Wire Connector"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Electrum Ingot"), vec![(1, 0, 1), (4, 0, 1), (7, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Terracotta"), vec![(3, 0, 1), (5, 0, 1), (6, 0, 1), (8, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("HV Wire Connector"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Aluminum Ingot"), vec![(1, 0, 1), (4, 0, 1), (7, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Terracotta"), vec![(3, 0, 1), (5, 0, 1), (6, 0, 1), (8, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("LV Capacitor"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Treated Wood Planks"),
                            vec![(0, 0, 1), (2, 0, 1), (6, 0, 1), (8, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Lead Plate"), vec![(3, 0, 1), (5, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Fluid Cell"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("LV Wire Connector"), vec![(1, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Etching Acid Bucket"), vec![(7, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("MV Capacitor"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Treated Wood Planks"),
                            vec![(0, 0, 1), (2, 0, 1), (6, 0, 1), (8, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("LV Capacitor"), vec![(3, 0, 1), (5, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Lead Plate"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("MV Wire Connector"), vec![(1, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Block of Electrum"), vec![(7, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("HV Capacitor"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Treated Wood Planks"),
                            vec![(0, 0, 1), (2, 0, 1), (6, 0, 1), (8, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("MV Capacitor"), vec![(3, 0, 1), (5, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Lead Plate"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("HV Wire Connector"), vec![(1, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Block of Steel"), vec![(7, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Tesla Coil"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Aluminum Ingot"), vec![(0, 0, 1), (1, 0, 1), (2, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Copper Coil Block"), vec![(4, 0, 1), (7, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("HV Capacitor"), vec![(6, 0, 1), (8, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Hardened Integral Components"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Invar Gear"), vec![(0, 0, 1), (2, 0, 1), (6, 0, 1), (8, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Blue Connector"), vec![(3, 0, 1), (5, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Redstone Flux Cell"), vec![(1, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Fluid Cell"), vec![(7, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Advanced PCB"), vec![(4, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("item.kubejs.cpu_core_500_package"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Basic Processor"),
                            vec![(0, 0, 1), (2, 0, 1), (3, 0, 1), (5, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Unassembled PCB"), vec![(1, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Lead Wire"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Cardboard Box"), vec![(6, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("item.kubejs.cpu_core_1000_package"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Improved Processor"),
                            vec![(0, 0, 1), (2, 0, 1), (3, 0, 1), (5, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("CPU Core B500"), vec![(1, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Copper Wire"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Cardboard Box"), vec![(6, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("item.kubejs.cpu_core_2000_package"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Advanced Processor"),
                            vec![(0, 0, 1), (2, 0, 1), (3, 0, 1), (5, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("CPU Core S1000"), vec![(1, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Electrum Wire"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Cardboard Box"), vec![(6, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Wireless Transmitter"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Advanced Processor"), vec![(2, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Aluminium Wire"),
                            vec![(1, 0, 1), (3, 0, 1), (5, 0, 1), (7, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Aluminum Rod"), vec![(4, 0, 1), (6, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Scaffolding"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Steel Ingot"), vec![(0, 0, 1), (1, 0, 1), (2, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Steel Rod"), vec![(4, 0, 1), (6, 0, 1), (8, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Relay"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Redstone Torch"), vec![(1, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Redstone Dust"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Cobblestone Slab"), vec![(0, 0, 1), (3, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Insulating Glass"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Green Dye"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Glass"), vec![(1, 0, 1), (7, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Iron Dust"), vec![(3, 0, 1), (5, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("RAM Chip 8E"), n_wanted: 6 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Circuit Backplane"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Lead Wire"), vec![(3, 0, 1), (5, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Redstone Relay"),
                            vec![(0, 0, 1), (1, 0, 1), (2, 0, 1), (6, 0, 1), (7, 0, 1), (8, 0, 1)],
                        ),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("item.kubejs.basic_memory_package"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Cardboard Box"), vec![(6, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Unassembled PCB"), vec![(7, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("RAM Chip 8E"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("RAM Chip 8E"), vec![(1, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("RAM Chip 8E"), vec![(2, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("RAM Chip 8E"), vec![(3, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("RAM Chip 8E"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("RAM Chip 8E"), vec![(5, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("item.kubejs.basic_circuit_package"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Cardboard Box"), vec![(8, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Printed Circuit Board"), vec![(6, 0, 1), (7, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("item.kubejs.memory_basic_filled"),
                            vec![(3, 0, 1), (4, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Improved Processor"), vec![(0, 0, 1), (1, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Hardened Capacitor"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Basic Capacitor (Large)"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Dielectric Paste"),
                            vec![(0, 0, 1), (2, 0, 1), (6, 0, 1), (8, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(
                            Filter::Label("Energized Steel"),
                            vec![(1, 0, 1), (3, 0, 1), (5, 0, 1), (7, 0, 1)],
                        ),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Flourishing Archwood Sapling"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Flourishing Archwood Sapling"), vec![(4, 0, 1)])
                            .allow_backup(),
                        MultiInvSlottedInput::new(Filter::Label("Beeswax"), vec![(3, 0, 1), (5, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("item.resourcefulbees.forest_honeycomb"),
                            vec![(1, 0, 1), (7, 0, 1)],
                        ),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Flourishing Archwood Log"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Flourishing Archwood Sapling"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Beeswax"), vec![(0, 0, 1), (2, 0, 1), (6, 0, 1), (8, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("item.resourcefulbees.forest_honeycomb"),
                            vec![(1, 0, 1), (3, 0, 1), (5, 0, 1), (7, 0, 1)],
                        ),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Vegetable Curry"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Potato"), vec![(1, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Curry Powder"), vec![(0, 0, 1), (2, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Rice"), vec![(3, 0, 1), (4, 0, 1), (5, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Knife"), n_wanted: 1 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Iron Ingot"), vec![(1, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Stick"), vec![(3, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Black Stained Glass"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Black Dye"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Glass"),
                            vec![(0, 0, 1), (1, 0, 1), (2, 0, 1), (3, 0, 1), (5, 0, 1), (6, 0, 1), (7, 0, 1), (8, 0, 1)],
                        ),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Thermal Lagging"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Gray Rockwool"), vec![(3, 0, 1), (4, 0, 1), (5, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Black Stained Glass Pane"),
                            vec![(0, 0, 1), (1, 0, 1), (2, 0, 1), (6, 0, 1), (7, 0, 1), (8, 0, 1)],
                        ),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: air_canister(false), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Pressure Tube"), vec![(1, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Compressed Iron Ingot"),
                            vec![(3, 0, 1), (5, 0, 1), (6, 0, 1), (8, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Redstone Dust"), vec![(4, 0, 1), (7, 0, 1)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Thermoelectric Generator"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(9, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Copper Coil Block"), vec![(4, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Steel Ingot"), vec![(0, 0, 1), (1, 0, 1), (2, 0, 1)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Constantan Plate"),
                            vec![(3, 0, 1), (5, 0, 1), (6, 0, 1), (7, 0, 1), (8, 0, 1)],
                        ),
                    ],
                    max_sets: 1,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "centrifugeBottle",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "resourcefulbees:centrifuge_casing_0",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot > 3)),
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Glass Bottle"), vec![(0, 1)])],
                max_sets: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "centrifugePCBee",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "resourcefulbees:centrifuge_casing_0",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![1],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(
                    Filter::Label("block.resourcefulbees.pcbee_honeycomb_block"),
                    vec![(1, 1)],
                )],
                max_sets: 64,
            }],
        });
        factory.add_process(ScatteringConfig {
            name: "centrifuge",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "resourcefulbees:centrifuge_casing_0",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![2, 3],
            to_extract: None,
            recipes: vec![
                ScatteringRecipe::new(
                    vec![
                        Output { item: Filter::Label("Bone"), n_wanted: 16 },
                        Output { item: Filter::Label("Honey Bottle"), n_wanted: 16 },
                    ],
                    ScatteringInput::new(Filter::Label("Skeleton Honeycomb Block")),
                ),
                ScatteringRecipe::new(
                    vec![
                        Output { item: Filter::Label("Lapis Lazuli Chunk"), n_wanted: 16 },
                        Output { item: Filter::Label("Apatite Chunk"), n_wanted: 16 },
                    ],
                    ScatteringInput::new(Filter::Label("Lapis Honeycomb Block")),
                ),
                ScatteringRecipe::new(
                    vec![
                        Output { item: Filter::Label("Emerald Chunk"), n_wanted: 16 },
                        Output { item: Filter::Label("Prismarine Shard"), n_wanted: 16 },
                    ],
                    ScatteringInput::new(Filter::Label("Emerald Honeycomb Block")),
                ),
                ScatteringRecipe::new(
                    vec![
                        Output { item: Filter::Label("Nether Quartz Chunk"), n_wanted: 16 },
                        Output { item: Filter::Label("Fluorite Chunk"), n_wanted: 16 },
                    ],
                    ScatteringInput::new(Filter::Label("Nether Quartz Honeycomb Block")),
                ),
                ScatteringRecipe::new(
                    vec![
                        Output { item: Filter::Label("Ghast Tear"), n_wanted: 48 },
                        Output { item: Filter::Label("Milk Bottle"), n_wanted: 16 },
                    ],
                    ScatteringInput::new(Filter::Label("block.resourcefulbees.boobee_honeycomb_block")),
                ),
                ScatteringRecipe::new(
                    vec![
                        Output { item: Filter::Label("Book"), n_wanted: 16 },
                        Output { item: Filter::Label("Paper"), n_wanted: 16 },
                        Output { item: Filter::Label("Bottle o' Enchanting"), n_wanted: 16 },
                    ],
                    ScatteringInput::new(Filter::Label("block.resourcefulbees.spelling_honeycomb_block")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Diamond Chunk"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Diamond Honeycomb Block")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Iron Chunk"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Iron Honeycomb Block")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Gold Chunk"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Gold Honeycomb Block")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Rainbow Rune"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("RGBee Honeycomb Block")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Ender Pearl"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Ender Honeycomb Block")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Coal Chunk"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Coal Honeycomb Block")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Redstone Chunk"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Redstone Honeycomb Block")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Lead Chunk"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("block.resourcefulbees.lead_honeycomb_block")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Zinc Chunk"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("block.resourcefulbees.zinc_honeycomb_block")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Tin Chunk"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("block.resourcefulbees.tin_honeycomb_block")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Osmium Chunk"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("block.resourcefulbees.osmium_honeycomb_block")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Cobalt Chunk"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("block.resourcefulbees.cobalt_honeycomb_block")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Copper Chunk"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("block.resourcefulbees.copper_honeycomb_block")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Silver Chunk"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("block.resourcefulbees.silver_honeycomb_block")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Nickel Chunk"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("block.resourcefulbees.nickel_honeycomb_block")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Slimeball"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("block.resourcefulbees.slimy_honeycomb_block")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Blaze Rod"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("block.resourcefulbees.blaze_honeycomb_block")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Blitz Mote"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("block.resourcefulbees.blitz_honeycomb_block")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Basalz Shard"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("block.resourcefulbees.basalz_honeycomb_block")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Blizz Cube"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("block.resourcefulbees.blizz_honeycomb_block")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Uranium Chunk"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("block.resourcefulbees.uranium_honeycomb_block")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Aluminum Chunk"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("block.resourcefulbees.aluminum_honeycomb_block")),
                ),
                ScatteringRecipe::new(
                    vec![
                        Output { item: Filter::Label("Oak Log"), n_wanted: 16 },
                        Output { item: Filter::Label("Beeswax"), n_wanted: 16 },
                    ],
                    ScatteringInput::new(Filter::Label("block.resourcefulbees.forest_honeycomb_block")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Glowstone Dust"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("block.resourcefulbees.glowstone_honeycomb_block")),
                ),
            ],
            max_per_slot: 2,
        });
        factory.add_process(BufferedConfig {
            name: "fluxDustOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_58",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(MultiInvSlottedConfig {
            name: "fluxDust",
            input_slots: vec![vec![0], vec![0, 1]],
            accesses: vec![MultiInvAccess {
                client: "1a",
                inv_addrs: vec!["minecraft:barrel_57", "minecraft:barrel_56"],
                bus_addr: "ironchest:diamond_chest_31",
            }],
            to_extract: None,
            recipes: vec![MultiInvSlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Flux Dust"), n_wanted: 16 }],
                inputs: vec![
                    MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), vec![(0, 0, 1)]),
                    MultiInvSlottedInput::new(Filter::Label("Redstone Dust"), vec![(1, 0, 16)]),
                    MultiInvSlottedInput::new(Filter::Label("Obsidian"), vec![(1, 1, 1)]),
                ],
                max_sets: 1
            }],
        });
        factory.add_process(MultiInvSlottedConfig {
            name: "lightningCrafting",
            input_slots: vec![vec![0], vec![0, 1, 2, 3]],
            accesses: vec![MultiInvAccess {
                client: "1a",
                inv_addrs: vec!["minecraft:dispenser_1", "minecraft:dropper_5"],
                bus_addr: "ironchest:diamond_chest_31",
            }],
            to_extract: None,
            recipes: vec![
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("block.kubejs.firmament"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Lightning Arrow"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Fluorite"), vec![(1, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Shiverstone"), vec![(1, 1, 6)]),
                        MultiInvSlottedInput::new(Filter::Label("Prismarine"), vec![(1, 2, 6)]),
                    ],
                    max_sets: 1
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Arcane Stone"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Lightning Arrow"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Name("emendatusenigmatica:arcane_gem"), vec![(1, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Apatite"), vec![(1, 1, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Clay"), vec![(1, 2, 3)]),
                    ],
                    max_sets: 1
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lodestone"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Lightning Arrow"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Block of Iron"), vec![(1, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Copper Dust"), vec![(1, 1, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Iron Dust"), vec![(1, 2, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Fluorite"), vec![(1, 3, 1)]),
                    ],
                    max_sets: 1
                },
            ],
        });
        factory.add_process(MultiInvSlottedConfig {
            name: "runicAltar",
            input_slots: vec![vec![0], vec![0, 1, 2, 3, 4, 5]],
            accesses: vec![MultiInvAccess {
                client: "1a",
                inv_addrs: vec!["engineersdecor:te_factory_dropper_3", "engineersdecor:te_factory_dropper_6"],
                bus_addr: "ironchest:diamond_chest_31",
            }],
            to_extract: None,
            recipes: vec![
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("White Rune"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Livingrock"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("White Corundum"), vec![(1, 0, 8)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Rune of Earth"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Livingrock"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Mana Powder"), vec![(1, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Manasteel Ingot"), vec![(1, 1, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Basalz Powder"), vec![(1, 2, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Shiverstone"), vec![(1, 3, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("White Rune"), vec![(1, 4, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Worm"), vec![(1, 5, 1)]),
                    ],
                    max_sets: 1,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "plasticTPP",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_61",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("block.kubejs.coated_machine_frame_top"), n_wanted: 4 }],
                inputs: vec![BufferedInput::new(Filter::Label("block.kubejs.rough_machine_frame_top"), 1)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 4,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "redstoneTPP",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_88",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Power Core (Low)"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Machine Base"), 1)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 4,
            stocks: vec![],
        });
        factory.add_process(MultiInvSlottedConfig {
            name: "arsCrafting",
            input_slots: vec![vec![0], vec![0], vec![0], vec![0], vec![0], vec![0], vec![0], vec![0], vec![0]],
            accesses: vec![MultiInvAccess {
                client: "1a",
                inv_addrs: vec![
                    "minecraft:hopper_11",
                    "ars_nouveau:arcane_pedestal_2",
                    "ars_nouveau:arcane_pedestal_3",
                    "ars_nouveau:arcane_pedestal_4",
                    "ars_nouveau:arcane_pedestal_5",
                    "ars_nouveau:arcane_pedestal_6",
                    "ars_nouveau:arcane_pedestal_7",
                    "ars_nouveau:arcane_pedestal_8",
                    "ars_nouveau:arcane_pedestal_9",
                ],
                bus_addr: "ironchest:diamond_chest_31",
            }],
            to_extract: None,
            recipes: vec![
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sunmetal Blend"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(
                            Filter::Label("Silver Dust"),
                            vec![(1, 0, 1), (2, 0, 1), (3, 0, 1), (4, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Sulfur Dust"), vec![(0, 0, 1)]),
                    ],
                    max_sets: 1
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Otherstone Tablet"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(
                            Filter::Label("Otherstone Slab"),
                            vec![(1, 0, 1), (2, 0, 1), (3, 0, 1), (4, 0, 1)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Flux Dust"), vec![(5, 0, 1), (6, 0, 1), (7, 0, 1), (8, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Name("emendatusenigmatica:arcane_gem"), vec![(0, 0, 1)]),
                    ],
                    max_sets: 1
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Arcane Gold Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Redstone Dust"), vec![(1, 0, 1), (2, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Gold Ingot"), vec![(3, 0, 1), (4, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Enchanted Ash"), vec![(5, 0, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Soul Shard"), vec![(0, 0, 1)]),
                    ],
                    max_sets: 1
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "naturalAltar",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_78",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Infused Iron Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sunmetal Brick"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sunstone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("block.kubejs.firmament"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "concreteMixer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "create:basin_7",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![0, 1, 2],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Concrete"), n_wanted: 16 }],
                inputs: vec![
                    SlottedInput::new(Filter::Label("Clay Ball"), vec![(0, 2)]),
                    SlottedInput::new(Filter::Label("Gravel"), vec![(1, 2)]),
                    SlottedInput::new(Filter::Label("Slag"), vec![(2, 4)]),
                ],
                max_sets: 4,
            }],
        });
        factory.add_process(MultiInvSlottedConfig {
            name: "3-deployers",
            input_slots: vec![vec![0], vec![0], vec![0], vec![0]],
            accesses: vec![MultiInvAccess {
                client: "1a",
                inv_addrs: vec!["minecraft:barrel_85", "create:deployer_3", "create:deployer_4", "create:deployer_5"],
                bus_addr: "ironchest:diamond_chest_31",
            }],
            to_extract: None,
            recipes: vec![MultiInvSlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Basic Capacitor (Large)"), n_wanted: 16 }],
                inputs: vec![
                    MultiInvSlottedInput::new(Filter::Label("Plastic Sheet"), vec![(0, 0, 1)]),
                    MultiInvSlottedInput::new(Filter::Label("Aluminum Plate"), vec![(1, 0, 4)]),
                    MultiInvSlottedInput::new(Filter::Label("Dielectric Paste"), vec![(2, 0, 4)]),
                    MultiInvSlottedInput::new(Filter::Label("Signalum Plate"), vec![(3, 0, 4)]),
                ],
                max_sets: 4,
            }],
        });
        factory.add_process(MultiInvSlottedConfig {
            name: "2-deployers",
            input_slots: vec![vec![0], vec![0], vec![0]],
            accesses: vec![MultiInvAccess {
                client: "1a",
                inv_addrs: vec!["minecraft:barrel_86", "create:deployer_7", "create:deployer_8"],
                bus_addr: "ironchest:diamond_chest_31",
            }],
            to_extract: None,
            recipes: vec![MultiInvSlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Circuit Backplane"), n_wanted: 16 }],
                inputs: vec![
                    MultiInvSlottedInput::new(Filter::Label("Insulating Glass"), vec![(0, 0, 1)]),
                    MultiInvSlottedInput::new(Filter::Label("Copper Plate"), vec![(1, 0, 1)]),
                    MultiInvSlottedInput::new(Filter::Label("Dielectric Paste"), vec![(2, 0, 1)]),
                ],
                max_sets: 8,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "pureDaisy",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_83",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Livingrock"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sunstone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Livingwood"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Flourishing Archwood Wood"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "xpTransposer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_bottler_4",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("item.kubejs.memory_basic_filled"), n_wanted: 4 }],
                    inputs: vec![SlottedInput::new(Filter::Label("item.kubejs.memory_basic_empty"), vec![(0, 1)])],
                    max_sets: 4,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("item.kubejs.memory_advanced_filled"), n_wanted: 4 }],
                    inputs: vec![SlottedInput::new(Filter::Label("item.kubejs.memory_advanced_empty"), vec![(0, 1)])],
                    max_sets: 4,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "energizingOrb",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "powah:energizing_orb_0",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![1, 2],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Energized Steel"), n_wanted: 16 }],
                inputs: vec![
                    SlottedInput::new(Filter::Label("Froststeel Ingot"), vec![(1, 1)]),
                    SlottedInput::new(Filter::Label("Electrum Ingot"), vec![(2, 1)]),
                ],
                max_sets: 1,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "spiritFire",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_90",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Otherstone"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Arcane Stone"), 1)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "phytoStock",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_insolator_0",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![1],
            to_extract: Some(Box::new(|slot, _| slot > 1)),
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Phyto-Gro"), vec![(1, 1)])],
                max_sets: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "phyto",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_insolator_0",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Apple"), n_wanted: 16 },
                        Output { item: Filter::Label("Flourishing Archwood Leaves"), n_wanted: 16 },
                    ],
                    inputs: vec![SlottedInput::new(Filter::Label("Flourishing Archwood Sapling"), vec![(0, 1)])],
                    max_sets: 2,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Wheat"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Wheat Seeds"), vec![(0, 1)])],
                    max_sets: 2,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Plum"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Plum Sapling"), vec![(0, 1)])],
                    max_sets: 2,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Mango"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Mango Sapling"), vec![(0, 1)])],
                    max_sets: 2,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ginger"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Ginger Seeds"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Grass"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Grass"), vec![(0, 1)]).allow_backup()],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Potato"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Potato"), vec![(0, 1)]).allow_backup()],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dandelion"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Dandelion"), vec![(0, 1)]).allow_backup()],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cumin Seeds"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Cumin Seeds"), vec![(0, 1)]).allow_backup()],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Rice Panicle"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Rice"), vec![(0, 1)]).allow_backup()],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Mystical Black Flower"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Mystical Black Flower"), vec![(0, 1)]).allow_backup()],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Mystical Green Flower"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Mystical Green Flower"), vec![(0, 1)]).allow_backup()],
                    max_sets: 8,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "airCharger",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:hopper_15",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: air_canister(true), n_wanted: 4 }],
                inputs: vec![BufferedInput::new(air_canister(false), 1)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 1,
            stocks: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "wormFarm",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "create:deployer_9",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Worm"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Baked Potato"), vec![(0, 1)])],
                max_sets: 8,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "sunStoneManaPool",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "engineersdecor:te_factory_dropper_0",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Manasteel Ingot"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Froststeel Ingot"), 1)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
    })
}
