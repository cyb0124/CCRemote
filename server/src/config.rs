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
        Apiary 4: Dandelion, Earthslime Cake, Enderslime Cake, Lectern, Oasis Grass
            Basalz, Blizz, Spelling, Cobalt, Redstone, Dusty
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
        for inv_addr in [
            "ironchest:diamond_chest_27",
            "ironchest:diamond_chest_28",
            "ironchest:diamond_chest_29",
            "ironchest:diamond_chest_30",
            "ironchest:diamond_chest_32",
            "ironchest:diamond_chest_33",
            "ironchest:diamond_chest_34",
            "ironchest:diamond_chest_35",
            "ironchest:diamond_chest_36",
            "ironchest:diamond_chest_37",
        ] {
            factory.add_storage(ChestConfig {
                accesses: vec![BusAccess { client: "1a", inv_addr, bus_addr: "ironchest:diamond_chest_31" }],
            })
        }
        factory.add_storage(DrawerConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "storagedrawers:controller_0",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            filters: vec![
                Filter::Label("item.resourcefulbees.shepherd_honeycomb"),
                Filter::Label("item.resourcefulbees.forest_honeycomb"),
                Filter::Label("Andesite Cobblestone"),
                Filter::Label("Drop of Glycerol"),
                Filter::Label("Aluminum Nugget"),
                Filter::Label("Spectral Arrow"),
                Filter::Label("Cinnabar Chunk"),
                Filter::Label("Emerald Chunk"),
                Filter::Label("Silver Nugget"),
                Filter::Label("Osmium Nugget"),
                Filter::Label("Nickel Nugget"),
                Filter::Label("Copper Nugget"),
                Filter::Label("Honey Bottle"),
                Filter::Label("Sulfur Dust"),
                Filter::Label("Cobblestone"),
                Filter::Label("Shiverstone"),
                Filter::Label("Lead Nugget"),
                Filter::Label("Tin Nugget"),
                Filter::Label("Coal Coke"),
                Filter::Label("Tree Bark"),
                Filter::Label("Snowball"),
                Filter::Label("Paper"),
                Filter::Label("Book"),
                Filter::Label("Tar"),
            ],
        });
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
                BufferedInput::new(Filter::Label("block.resourcefulbees.dusty_mummbee_honeycomb_block"), i32::MAX)
                    .extra_backup(512),
                BufferedInput::new(Filter::Label("block.resourcefulbees.glowstone_honeycomb_block"), i32::MAX)
                    .extra_backup(512),
                BufferedInput::new(Filter::Label("block.resourcefulbees.spelling_honeycomb_block"), i32::MAX)
                    .extra_backup(512),
                BufferedInput::new(Filter::Label("block.resourcefulbees.aluminum_honeycomb_block"), i32::MAX)
                    .extra_backup(512),
                BufferedInput::new(Filter::Label("block.resourcefulbees.uranium_honeycomb_block"), i32::MAX)
                    .extra_backup(512),
                BufferedInput::new(Filter::Label("block.resourcefulbees.osmium_honeycomb_block"), i32::MAX)
                    .extra_backup(512),
                BufferedInput::new(Filter::Label("block.resourcefulbees.cobalt_honeycomb_block"), i32::MAX)
                    .extra_backup(512),
                BufferedInput::new(Filter::Label("block.resourcefulbees.copper_honeycomb_block"), i32::MAX)
                    .extra_backup(512),
                BufferedInput::new(Filter::Label("block.resourcefulbees.silver_honeycomb_block"), i32::MAX)
                    .extra_backup(512),
                BufferedInput::new(Filter::Label("block.resourcefulbees.nickel_honeycomb_block"), i32::MAX)
                    .extra_backup(512),
                BufferedInput::new(Filter::Label("block.resourcefulbees.boobee_honeycomb_block"), i32::MAX)
                    .extra_backup(512),
                BufferedInput::new(Filter::Label("block.resourcefulbees.basalz_honeycomb_block"), i32::MAX)
                    .extra_backup(512),
                BufferedInput::new(Filter::Label("block.resourcefulbees.slimy_honeycomb_block"), i32::MAX)
                    .extra_backup(512),
                BufferedInput::new(Filter::Label("block.resourcefulbees.blaze_honeycomb_block"), i32::MAX)
                    .extra_backup(512),
                BufferedInput::new(Filter::Label("block.resourcefulbees.blitz_honeycomb_block"), i32::MAX)
                    .extra_backup(512),
                BufferedInput::new(Filter::Label("block.resourcefulbees.blizz_honeycomb_block"), i32::MAX)
                    .extra_backup(512),
                BufferedInput::new(Filter::Label("block.resourcefulbees.pcbee_honeycomb_block"), i32::MAX)
                    .extra_backup(512),
                BufferedInput::new(Filter::Label("block.resourcefulbees.zinc_honeycomb_block"), i32::MAX)
                    .extra_backup(512),
                BufferedInput::new(Filter::Label("block.resourcefulbees.lead_honeycomb_block"), i32::MAX)
                    .extra_backup(512),
                BufferedInput::new(Filter::Label("block.resourcefulbees.tin_honeycomb_block"), i32::MAX)
                    .extra_backup(512),
                BufferedInput::new(Filter::Label("Nether Quartz Honeycomb Block"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Skeleton Honeycomb Block"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Redstone Honeycomb Block"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Diamond Honeycomb Block"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Emerald Honeycomb Block"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("RGBee Honeycomb Block"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Ender Honeycomb Block"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Lapis Honeycomb Block"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Coal Honeycomb Block"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Iron Honeycomb Block"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Gold Honeycomb Block"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Industrial Hemp Fiber"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Industrial Hemp Seeds"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Acacia Blossom"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Acacia Sapling"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Pepper Seeds"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Nether Wart"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Acacia Log"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Sugar Cane"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Magebloom"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Habanero"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Pepper"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Stick"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Flint"), i32::MAX).extra_backup(512),
                BufferedInput::new(Filter::Label("Kelp"), i32::MAX).extra_backup(512),
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
                BufferedInput::new(Filter::Label("Cobblestone"), 64),
                BufferedInput::new(Filter::Label("Light Gray Concrete Powder"), 64),
                BufferedInput::new(Filter::Label("Compressed Iron Ingot"), 64),
                BufferedInput::new(Filter::Label("Sand Paper"), 1),
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
        factory.add_process(BufferedConfig {
            name: "bufferE",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_91",
                bus_addr: "ironchest:diamond_chest_31",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![
                BufferedInput::new(Filter::Label("Cooked Chicken"), 64),
                BufferedInput::new(Filter::Label("Baked Potato"), 64),
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
                        SlottedInput::new(Filter::Label("Cooked Chicken Wings"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Habanero"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Mango"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Plum Pudding"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Chicken Egg"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Milk Bottle"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Sugar"), 1, vec![2]),
                        SlottedInput::new(Filter::Label("Plum"), 1, vec![3]),
                        SlottedInput::new(Filter::Label("Rice"), 1, vec![4]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Curry Powder"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Pepper"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Cumin Seeds"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Ginger"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Phyto-Gro"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Sand"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Potassium Nitrate"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Apatite"), 2, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Blue Rockwool"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("White Rockwool"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Blue Dye"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sand Paper"), n_wanted: 1 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Paper"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Sand"), 1, vec![1]),
                    ],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Algal Blend"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Clay Ball"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Kelp"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gray Dye"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Black Dye"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("White Dye"), 1, vec![1]),
                    ],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gray Rockwool"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Gray Dye"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("White Rockwool"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Rose Quartz"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Nether Quartz"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Redstone Dust"), 8, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gunpowder"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Charcoal"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Sulfur Dust"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Potassium Nitrate Dust"), 4, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lightning Charge"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Charcoal"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Gunpowder"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Blitz Powder"), 1, vec![2]),
                    ],
                    max_per_slot: 8,
                },
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
                    SlottedInput::new(Filter::Label("Tar"), 2, vec![0]),
                    SlottedInput::new(Filter::Label("Sand"), 2, vec![1]),
                    SlottedInput::new(Filter::Label("Silicon"), 3, vec![2]),
                ],
                max_per_slot: 15,
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
                    SlottedInput::new(Filter::Label("block.kubejs.coated_machine_frame_top"), 1, vec![0]),
                    SlottedInput::new(Filter::Label("Machine Base"), 1, vec![1]),
                    SlottedInput::new(Filter::Label("Invar Plate"), 2, vec![2]),
                ],
                max_per_slot: 8,
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
                    inputs: vec![BufferedInput::new(Filter::Label("Acacia Log"), 16)],
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
                        SlottedInput::new(Filter::Label("Chicken Egg"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Sugar"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Cinder Flour"), 1, vec![2]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Reinforced Bricks"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Reinforced Stone"), 4, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cardboard Box"), n_wanted: 4 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Sawdust"), 4, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Prismarine"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Prismarine Shard"), 4, vec![0])],
                    max_per_slot: 8,
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
                    outputs: vec![Output { item: Filter::Label("Acacia Button"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Acacia Planks"), 1)],
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
                    inputs: vec![SlottedInput::new(Filter::Label("Gold Ingot"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Rod"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lead Ingot"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Rod"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Steel Ingot"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Brass Rod"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Brass Ingot"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Rod"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Copper Ingot"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminum Rod"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Aluminum Ingot"), 1, vec![0])],
                    max_per_slot: 8,
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
                    inputs: vec![SlottedInput::new(Filter::Label("Tin Nugget"), 3, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lumium Coin"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lumium Ingot"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Enderium Coin"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Enderium Ingot"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basic Processor"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Raw Basic Processor"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Improved Processor"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Raw Improved Processor"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Advanced Processor"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Raw Advanced Processor"), 1, vec![0])],
                    max_per_slot: 8,
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
                    inputs: vec![SlottedInput::new(Filter::Label("Iron Ingot"), 4, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Gear"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lead Ingot"), 4, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Invar Gear"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Invar Ingot"), 4, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Gear"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Copper Ingot"), 4, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Osmium Gear"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Osmium Ingot"), 4, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lumium Gear"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lumium Ingot"), 4, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bronze Gear"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Bronze Ingot"), 4, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electrum Gear"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Electrum Ingot"), 4, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Constantan Gear"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Constantan Ingot"), 4, vec![0])],
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
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Baked Potato"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Potato"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cooked Chicken"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Raw Chicken"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cooked Chicken Wings"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Raw Chicken Wings"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dried Kelp"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Kelp"), 1, vec![0])],
                    max_per_slot: 8,
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
                        SlottedInput::new(Filter::Label("Copper Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Aluminum Ingot"), 3, vec![1]),
                        SlottedInput::new(Filter::Label("Lithium Dust"), 4, vec![2]),
                    ],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lumium Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Silver Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Glowstone Dust"), 2, vec![1]),
                        SlottedInput::new(Filter::Label("Tin Ingot"), 3, vec![2]),
                    ],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Enderium Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Diamond Dust"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Ender Dust"), 2, vec![1]),
                        SlottedInput::new(Filter::Label("Lead Ingot"), 3, vec![2]),
                    ],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Hardened Glass"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Nether Quartz"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Obsidian"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Sand"), 1, vec![2]),
                    ],
                    max_per_slot: 8,
                },
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
                    outputs: vec![Output { item: Filter::Label("Tinker's Bronze Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Copper Ingot"), 3, vec![0]),
                        SlottedInput::new(Filter::Label("Glass"), 1, vec![1]),
                    ],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Froststeel Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Blizz Powder"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Cobalt Ingot"), 3, vec![1]),
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
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pewter Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Iron Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Lead Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Signalum Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Silver Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Copper Ingot"), 3, vec![1]),
                        SlottedInput::new(Filter::Label("Redstone Dust"), 4, vec![2]),
                    ],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Quartz Enriched Iron"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Compressed Iron Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Nether Quartz"), 1, vec![1]),
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
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bone Meal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Bone"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silicon"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Limesand"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Potassium Nitrate"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Sandstone"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sawdust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Acacia Log"), 1, vec![0])],
                    max_per_slot: 8,
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
                        SlottedInput::new(Filter::Label("Redstone Servo"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Invar Plate"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Flux Dust"), 1, vec![2]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Compressed Iron Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Obsidian"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Steel Ingot"), 2, vec![1]),
                        SlottedInput::new(Filter::Label("Tar"), 2, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empty PCB"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Copper Plate"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Plastic Sheet"), 2, vec![1]),
                        SlottedInput::new(Filter::Label("Flux Dust"), 6, vec![2]),
                    ],
                    max_per_slot: 12,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Raw Basic Processor"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Processor Binding"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Flux Dust"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Tin Coin"), 1, vec![2]),
                        SlottedInput::new(Filter::Label("Silicon"), 1, vec![3]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Raw Improved Processor"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Processor Binding"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Flux Dust"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Lumium Coin"), 1, vec![2]),
                        SlottedInput::new(Filter::Label("Silicon"), 1, vec![3]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Raw Advanced Processor"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Processor Binding"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Flux Dust"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Enderium Coin"), 1, vec![2]),
                        SlottedInput::new(Filter::Label("Silicon"), 1, vec![3]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Transistor"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Plastic Sheet"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Electrum Wire"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Dielectric Paste"), 1, vec![2]),
                        SlottedInput::new(Filter::Label("Raw Basic Processor"), 2, vec![3]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Capacitor"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Plastic Sheet"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Electrum Wire"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Aluminum Plate"), 1, vec![2]),
                        SlottedInput::new(Filter::Label("Signalum Plate"), 1, vec![3]),
                        SlottedInput::new(Filter::Label("Dielectric Paste"), 1, vec![4]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Printed Circuit Board"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Lead Wire"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Basic Processor"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Unassembled PCB"), 1, vec![2]),
                        SlottedInput::new(Filter::Label("Transistor"), 2, vec![3]),
                        SlottedInput::new(Filter::Label("Capacitor"), 2, vec![4]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("item.kubejs.memory_advanced_empty"), n_wanted: 4 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Flux Dust"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("item.kubejs.memory_basic_empty"), 2, vec![1]),
                    ],
                    max_per_slot: 8,
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
                    inputs: vec![SlottedInput::new(Filter::Label("Empty PCB"), 1, vec![0])],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("block.kubejs.rough_machine_frame_top"), n_wanted: 4 }],
                    inputs: vec![SlottedInput::new(Filter::Name("thermal:machine_frame"), 2, vec![0])],
                    max_per_slot: 2,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("rftoolsbase:machine_frame"), n_wanted: 4 }],
                    inputs: vec![SlottedInput::new(Filter::Label("block.kubejs.rough_machine_frame"), 1, vec![0])],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("CPU Core B500"), n_wanted: 1 }],
                    inputs: vec![SlottedInput::new(Filter::Label("item.kubejs.cpu_core_500_package"), 1, vec![0])],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("CPU Core S1000"), n_wanted: 1 }],
                    inputs: vec![SlottedInput::new(Filter::Label("item.kubejs.cpu_core_1000_package"), 1, vec![0])],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("CPU Core EX2000"), n_wanted: 1 }],
                    inputs: vec![SlottedInput::new(Filter::Label("item.kubejs.cpu_core_2000_package"), 1, vec![0])],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("item.kubejs.memory_basic_empty"), n_wanted: 4 }],
                    inputs: vec![SlottedInput::new(Filter::Label("item.kubejs.basic_memory_package"), 1, vec![0])],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basic Control Circuit"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("item.kubejs.basic_circuit_package"), 1, vec![0])],
                    max_per_slot: 1,
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
                    inputs: vec![SlottedInput::new(Filter::Label("Lead Plate"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Wire"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Steel Plate"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Wire"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Copper Plate"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electrum Wire"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Electrum Plate"), 1, vec![0])],
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
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Iron Plate"), 4, vec![(0, 0), (2, 0), (6, 0), (8, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Glass Pane"), 4, vec![(1, 0), (3, 0), (5, 0), (7, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Iron Gear"), 1, vec![(4, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fluid Cell Frame"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Copper Plate"),
                            4,
                            vec![(0, 0), (2, 0), (6, 0), (8, 0)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Glass Pane"), 4, vec![(1, 0), (3, 0), (5, 0), (7, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Bronze Gear"), 1, vec![(4, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fluid Cell"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Cured Rubber"),
                            4,
                            vec![(0, 0), (2, 0), (6, 0), (8, 0)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Iron Ingot"), 2, vec![(3, 0), (5, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Fluid Cell Frame"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Hardened Glass"), 1, vec![(1, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Redstone Servo"), 1, vec![(7, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Flux Cell Frame"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Lead Plate"), 4, vec![(0, 0), (2, 0), (6, 0), (8, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Glass Pane"), 4, vec![(1, 0), (3, 0), (5, 0), (7, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Electrum Gear"), 1, vec![(4, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Flux Cell"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Cured Rubber"),
                            4,
                            vec![(0, 0), (2, 0), (6, 0), (8, 0)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Iron Ingot"), 2, vec![(3, 0), (5, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Redstone Flux Cell Frame"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Block of Redstone"), 1, vec![(1, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Redstone Flux Coil"), 1, vec![(7, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Flux Coil"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Gold Nugget"), 2, vec![(0, 0), (8, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Redstone Dust"),
                            4,
                            vec![(1, 0), (3, 0), (5, 0), (7, 0)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Gold Rod"), 1, vec![(4, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Servo"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Redstone Dust"),
                            4,
                            vec![(0, 0), (2, 0), (6, 0), (8, 0)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Lead Rod"), 1, vec![(4, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Torch"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Redstone Dust"), 1, vec![(0, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Stick"), 1, vec![(3, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electron Tube"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Polished Rose Quartz"), 1, vec![(0, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Redstone Torch"), 1, vec![(3, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Iron Nugget"), 1, vec![(6, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cogwheel"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Andesite Alloy"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Acacia Button"),
                            8,
                            vec![(0, 0), (1, 0), (2, 0), (3, 0), (5, 0), (6, 0), (7, 0), (8, 0)],
                        ),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Large Cogwheel"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Andesite Alloy"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Acacia Button"),
                            4,
                            vec![(0, 0), (2, 0), (6, 0), (8, 0)],
                        ),
                        MultiInvSlottedInput::new(
                            Filter::Label("Acacia Planks"),
                            4,
                            vec![(1, 0), (3, 0), (5, 0), (7, 0)],
                        ),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Acacia Chest"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Acacia Planks"),
                            8,
                            vec![(0, 0), (1, 0), (2, 0), (3, 0), (5, 0), (6, 0), (7, 0), (8, 0)],
                        ),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Hopper"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Acacia Chest"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Iron Ingot"),
                            5,
                            vec![(0, 0), (2, 0), (3, 0), (5, 0), (7, 0)],
                        ),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Blue Connector"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Redstone Dust"), 2, vec![(3, 0), (5, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Blue Rockwool"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Hopper"), 2, vec![(1, 0), (7, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Uranium Ingot"),
                            4,
                            vec![(0, 0), (2, 0), (6, 0), (8, 0)],
                        ),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Piston"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Gold Ingot"), 1, vec![(1, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Acacia Planks"), 2, vec![(0, 0), (2, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Smooth Stone"),
                            4,
                            vec![(3, 0), (5, 0), (6, 0), (8, 0)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Gold Rod"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Redstone Dust"), 1, vec![(7, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Advanced PCB"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Redstone Dust"),
                            4,
                            vec![(0, 0), (2, 0), (6, 0), (8, 0)],
                        ),
                        MultiInvSlottedInput::new(
                            Filter::Label("Plastic Sheet"),
                            4,
                            vec![(1, 0), (3, 0), (5, 0), (7, 0)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Printed Circuit Board"), 1, vec![(4, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Machine Base"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Reinforced Stone Slab"),
                            3,
                            vec![(6, 0), (7, 0), (8, 0)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Invar Nugget"), 3, vec![(1, 0), (3, 0), (5, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Advanced PCB"), 1, vec![(4, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Bars"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Iron Ingot"),
                            6,
                            vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)],
                        ),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Glass Pane"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Glass"),
                            6,
                            vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)],
                        ),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Black Stained Glass Pane"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Black Stained Glass"),
                            6,
                            vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)],
                        ),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Reinforced Brick Wall"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Reinforced Bricks"),
                            6,
                            vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)],
                        ),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cobblestone Slab"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Cobblestone"), 3, vec![(0, 0), (1, 0), (2, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Acacia Slab"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Acacia Planks"), 3, vec![(0, 0), (1, 0), (2, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Reinforced Stone Slab"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Reinforced Stone"), 3, vec![(0, 0), (1, 0), (2, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Otherstone Slab"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Otherstone"), 3, vec![(0, 0), (1, 0), (2, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Andesite Casing"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Acacia Planks"),
                            6,
                            vec![(0, 0), (1, 0), (2, 0), (6, 0), (7, 0), (8, 0)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Andesite Alloy"), 2, vec![(3, 0), (5, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Acacia Log"), 1, vec![(4, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Glass Bottle"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Glass"), 3, vec![(0, 0), (2, 0), (4, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bucket"), n_wanted: 1 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Iron Ingot"), 3, vec![(0, 0), (2, 0), (4, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Chicken Egg"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("item.resourcefulbees.shepherd_honeycomb"),
                            2,
                            vec![(3, 0), (8, 0)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Beeswax"), 1, vec![(7, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Feather"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("item.resourcefulbees.shepherd_honeycomb"),
                            2,
                            vec![(3, 0), (8, 0)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Beeswax"), 1, vec![(4, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Raw Chicken"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("item.resourcefulbees.shepherd_honeycomb"),
                            2,
                            vec![(3, 0), (5, 0)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Beeswax"), 1, vec![(4, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Arrow"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Flint"), 1, vec![(1, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Stick"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Feather"), 1, vec![(7, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lightning Arrow"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Lightning Charge"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Arrow"), 4, vec![(1, 0), (3, 0), (5, 0), (7, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Processor Binding"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Slimeball"), 1, vec![(1, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("String"), 2, vec![(0, 0), (2, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pressure Tube"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Hardened Glass"), 1, vec![(1, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Compressed Iron Ingot"), 2, vec![(0, 0), (2, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("LV Wire Coil"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Stick"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Copper Wire"),
                            4,
                            vec![(1, 0), (3, 0), (5, 0), (7, 0)],
                        ),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Coil Block"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Iron Ingot"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("LV Wire Coil"),
                            8,
                            vec![(0, 0), (1, 0), (2, 0), (3, 0), (5, 0), (6, 0), (7, 0), (8, 0)],
                        ),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("LV Wire Connector"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Copper Ingot"), 3, vec![(1, 0), (4, 0), (7, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Terracotta"), 4, vec![(3, 0), (5, 0), (6, 0), (8, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("MV Wire Connector"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Electrum Ingot"), 3, vec![(1, 0), (4, 0), (7, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Terracotta"), 4, vec![(3, 0), (5, 0), (6, 0), (8, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("HV Wire Connector"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Aluminum Ingot"), 3, vec![(1, 0), (4, 0), (7, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Terracotta"), 4, vec![(3, 0), (5, 0), (6, 0), (8, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("LV Capacitor"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Treated Wood Planks"),
                            4,
                            vec![(0, 0), (2, 0), (6, 0), (8, 0)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Lead Plate"), 2, vec![(3, 0), (5, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Fluid Cell"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("LV Wire Connector"), 1, vec![(1, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Etching Acid Bucket"), 1, vec![(7, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("MV Capacitor"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Treated Wood Planks"),
                            4,
                            vec![(0, 0), (2, 0), (6, 0), (8, 0)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("LV Capacitor"), 2, vec![(3, 0), (5, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Lead Plate"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("MV Wire Connector"), 1, vec![(1, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Block of Electrum"), 1, vec![(7, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("HV Capacitor"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Treated Wood Planks"),
                            4,
                            vec![(0, 0), (2, 0), (6, 0), (8, 0)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("MV Capacitor"), 2, vec![(3, 0), (5, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Lead Plate"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("HV Wire Connector"), 1, vec![(1, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Block of Steel"), 1, vec![(7, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Tesla Coil"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Aluminum Ingot"), 3, vec![(0, 0), (1, 0), (2, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Copper Coil Block"), 2, vec![(4, 0), (7, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("HV Capacitor"), 2, vec![(6, 0), (8, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Hardened Integral Components"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Invar Gear"), 4, vec![(0, 0), (2, 0), (6, 0), (8, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Blue Connector"), 2, vec![(3, 0), (5, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Redstone Flux Cell"), 1, vec![(1, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Fluid Cell"), 1, vec![(7, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Advanced PCB"), 1, vec![(4, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("item.kubejs.cpu_core_500_package"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Basic Processor"),
                            4,
                            vec![(0, 0), (2, 0), (3, 0), (5, 0)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Unassembled PCB"), 1, vec![(1, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Lead Wire"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Cardboard Box"), 1, vec![(6, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("item.kubejs.cpu_core_1000_package"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Improved Processor"),
                            4,
                            vec![(0, 0), (2, 0), (3, 0), (5, 0)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("CPU Core B500"), 1, vec![(1, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Copper Wire"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Cardboard Box"), 1, vec![(6, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("item.kubejs.cpu_core_2000_package"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Advanced Processor"),
                            4,
                            vec![(0, 0), (2, 0), (3, 0), (5, 0)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("CPU Core S1000"), 1, vec![(1, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Electrum Wire"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Cardboard Box"), 1, vec![(6, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Wireless Transmitter"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Advanced Processor"), 1, vec![(2, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Aluminium Wire"),
                            4,
                            vec![(1, 0), (3, 0), (5, 0), (7, 0)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Aluminum Rod"), 2, vec![(4, 0), (6, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Scaffolding"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Steel Ingot"), 3, vec![(0, 0), (1, 0), (2, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Steel Rod"), 3, vec![(4, 0), (6, 0), (8, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Relay"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Redstone Torch"), 1, vec![(1, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Redstone Dust"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Cobblestone Slab"), 2, vec![(0, 0), (3, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Insulating Glass"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Green Dye"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Glass"), 2, vec![(1, 0), (7, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Iron Dust"), 2, vec![(3, 0), (5, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("RAM Chip 8E"), n_wanted: 6 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Circuit Backplane"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Lead Wire"), 2, vec![(3, 0), (5, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Redstone Relay"),
                            6,
                            vec![(0, 0), (1, 0), (2, 0), (6, 0), (7, 0), (8, 0)],
                        ),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("item.kubejs.basic_memory_package"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Cardboard Box"), 1, vec![(6, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Unassembled PCB"), 1, vec![(7, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("RAM Chip 8E"), 1, vec![(0, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("RAM Chip 8E"), 1, vec![(1, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("RAM Chip 8E"), 1, vec![(2, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("RAM Chip 8E"), 1, vec![(3, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("RAM Chip 8E"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("RAM Chip 8E"), 1, vec![(5, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("item.kubejs.basic_circuit_package"), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Cardboard Box"), 1, vec![(8, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Printed Circuit Board"), 2, vec![(6, 0), (7, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("item.kubejs.memory_basic_filled"),
                            2,
                            vec![(3, 0), (4, 0)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Improved Processor"), 2, vec![(0, 0), (1, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Hardened Capacitor"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Basic Capacitor (Large)"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Dielectric Paste"),
                            4,
                            vec![(0, 0), (2, 0), (6, 0), (8, 0)],
                        ),
                        MultiInvSlottedInput::new(
                            Filter::Label("Energized Steel"),
                            4,
                            vec![(1, 0), (3, 0), (5, 0), (7, 0)],
                        ),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Flourishing Archwood Sapling"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Flourishing Archwood Sapling"), 1, vec![(4, 0)])
                            .allow_backup(),
                        MultiInvSlottedInput::new(Filter::Label("Beeswax"), 2, vec![(3, 0), (5, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("item.resourcefulbees.forest_honeycomb"),
                            2,
                            vec![(1, 0), (7, 0)],
                        ),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Flourishing Archwood Log"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Flourishing Archwood Sapling"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Beeswax"), 4, vec![(0, 0), (2, 0), (6, 0), (8, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("item.resourcefulbees.forest_honeycomb"),
                            4,
                            vec![(1, 0), (3, 0), (5, 0), (7, 0)],
                        ),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Vegetable Curry"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Potato"), 1, vec![(1, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Curry Powder"), 2, vec![(0, 0), (2, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Rice"), 3, vec![(3, 0), (4, 0), (5, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Knife"), n_wanted: 1 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Iron Ingot"), 1, vec![(1, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Stick"), 1, vec![(3, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Black Stained Glass"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Black Dye"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Glass"),
                            8,
                            vec![(0, 0), (1, 0), (2, 0), (3, 0), (5, 0), (6, 0), (7, 0), (8, 0)],
                        ),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Thermal Lagging"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Gray Rockwool"), 3, vec![(3, 0), (4, 0), (5, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Black Stained Glass Pane"),
                            6,
                            vec![(0, 0), (1, 0), (2, 0), (6, 0), (7, 0), (8, 0)],
                        ),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: air_canister(false), n_wanted: 4 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Pressure Tube"), 1, vec![(1, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Compressed Iron Ingot"),
                            4,
                            vec![(3, 0), (5, 0), (6, 0), (8, 0)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Redstone Dust"), 2, vec![(4, 0), (7, 0)]),
                    ],
                    max_per_slot: 64,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Thermoelectric Generator"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(9, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Copper Coil Block"), 1, vec![(4, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Steel Ingot"), 3, vec![(0, 0), (1, 0), (2, 0)]),
                        MultiInvSlottedInput::new(
                            Filter::Label("Constantan Plate"),
                            5,
                            vec![(3, 0), (5, 0), (6, 0), (7, 0), (8, 0)],
                        ),
                    ],
                    max_per_slot: 64,
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
                inputs: vec![SlottedInput::new(Filter::Label("Glass Bottle"), 1, vec![0])],
                max_per_slot: 64,
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
                    1,
                    vec![1],
                )],
                max_per_slot: 64,
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
                    vec![Output { item: Filter::Label("Beeswax"), n_wanted: 16 }],
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
                    MultiInvSlottedInput::new(Filter::Label("Andesite Cobblestone"), 64, vec![(0, 0)]),
                    MultiInvSlottedInput::new(Filter::Label("Redstone Dust"), 16, vec![(1, 0)]),
                    MultiInvSlottedInput::new(Filter::Label("Obsidian"), 1, vec![(1, 1)]),
                ],
                max_per_slot: 64,
            }],
        });
        factory.add_process(MultiInvSlottedConfig {
            name: "lightningCrafting",
            input_slots: vec![vec![0], vec![0, 1, 2]],
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
                        MultiInvSlottedInput::new(Filter::Label("Lightning Arrow"), 1, vec![(0, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Fluorite"), 1, vec![(1, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Shiverstone"), 6, vec![(1, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Prismarine"), 6, vec![(1, 2)]),
                    ],
                    max_per_slot: 6,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Arcane Stone"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(Filter::Label("Lightning Arrow"), 1, vec![(0, 0)]),
                        MultiInvSlottedInput::new(Filter::Name("emendatusenigmatica:arcane_gem"), 1, vec![(1, 0)]),
                        MultiInvSlottedInput::new(Filter::Label("Apatite"), 1, vec![(1, 1)]),
                        MultiInvSlottedInput::new(Filter::Label("Clay"), 3, vec![(1, 2)]),
                    ],
                    max_per_slot: 3,
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
                            4,
                            vec![(1, 0), (2, 0), (3, 0), (4, 0)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Sulfur Dust"), 1, vec![(0, 0)]),
                    ],
                    max_per_slot: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Otherstone Tablet"), n_wanted: 16 }],
                    inputs: vec![
                        MultiInvSlottedInput::new(
                            Filter::Label("Otherstone Slab"),
                            4,
                            vec![(1, 0), (2, 0), (3, 0), (4, 0)],
                        ),
                        MultiInvSlottedInput::new(Filter::Label("Flux Dust"), 4, vec![(5, 0), (6, 0), (7, 0), (8, 0)]),
                        MultiInvSlottedInput::new(Filter::Name("emendatusenigmatica:arcane_gem"), 1, vec![(0, 0)]),
                    ],
                    max_per_slot: 1,
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
                    SlottedInput::new(Filter::Label("Clay Ball"), 2, vec![0]),
                    SlottedInput::new(Filter::Label("Gravel"), 2, vec![1]),
                    SlottedInput::new(Filter::Label("Slag"), 4, vec![2]),
                ],
                max_per_slot: 16,
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
                    MultiInvSlottedInput::new(Filter::Label("Plastic Sheet"), 1, vec![(0, 0)]),
                    MultiInvSlottedInput::new(Filter::Label("Aluminum Plate"), 4, vec![(1, 0)]),
                    MultiInvSlottedInput::new(Filter::Label("Dielectric Paste"), 4, vec![(2, 0)]),
                    MultiInvSlottedInput::new(Filter::Label("Signalum Plate"), 4, vec![(3, 0)]),
                ],
                max_per_slot: 16,
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
                    MultiInvSlottedInput::new(Filter::Label("Insulating Glass"), 1, vec![(0, 0)]),
                    MultiInvSlottedInput::new(Filter::Label("Copper Plate"), 1, vec![(1, 0)]),
                    MultiInvSlottedInput::new(Filter::Label("Dielectric Paste"), 1, vec![(2, 0)]),
                ],
                max_per_slot: 8,
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
                    inputs: vec![SlottedInput::new(Filter::Label("item.kubejs.memory_basic_empty"), 1, vec![0])],
                    max_per_slot: 4,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("item.kubejs.memory_advanced_filled"), n_wanted: 4 }],
                    inputs: vec![SlottedInput::new(Filter::Label("item.kubejs.memory_advanced_empty"), 1, vec![0])],
                    max_per_slot: 4,
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
                    SlottedInput::new(Filter::Label("Froststeel Ingot"), 1, vec![1]),
                    SlottedInput::new(Filter::Label("Electrum Ingot"), 1, vec![2]),
                ],
                max_per_slot: 1,
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
                inputs: vec![SlottedInput::new(Filter::Label("Phyto-Gro"), 1, vec![1])],
                max_per_slot: 64,
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
                    inputs: vec![SlottedInput::new(Filter::Label("Flourishing Archwood Sapling"), 1, vec![0])],
                    max_per_slot: 2,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Plum"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Plum Sapling"), 1, vec![0])],
                    max_per_slot: 2,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Mango"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Mango Sapling"), 1, vec![0])],
                    max_per_slot: 2,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ginger"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Ginger Seeds"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Grass"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Grass"), 1, vec![0]).allow_backup()],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Potato"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Potato"), 1, vec![0]).allow_backup()],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cumin Seeds"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Cumin Seeds"), 1, vec![0]).allow_backup()],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Rice Panicle"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Rice"), 1, vec![0]).allow_backup()],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Mystical Black Flower"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Mystical Black Flower"), 1, vec![0]).allow_backup()],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Mystical Green Flower"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Mystical Green Flower"), 1, vec![0]).allow_backup()],
                    max_per_slot: 8,
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
    })
}
