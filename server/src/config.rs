use super::detail_cache::DetailCache;
use super::factory::{Factory, FactoryConfig};
use super::server::Server;
use super::{access::*, item::*, process::*, recipe::*, storage::*};
use std::{cell::RefCell, rc::Rc, time::Duration};

pub fn build_factory() -> Rc<RefCell<Factory>> {
    let congealed_green =
        || Filter::Fn(Box::new(|item, detail| detail.label == "Congealed Slime Block" && item.damage == 0));
    let congealed_blue =
        || Filter::Fn(Box::new(|item, detail| detail.label == "Congealed Slime Block" && item.damage == 1));
    let crystaltine_trimmed =
        || Filter::Fn(Box::new(|item, _| item.name == "extendedcrafting:trimmed" && item.damage == 4));
    FactoryConfig {
        detail_cache: DetailCache::new(),
        server: Server::new(1847),
        min_cycle_time: Duration::from_secs(1),
        log_clients: vec!["1a"],
        bus_accesses: vec![BasicAccess { client: "1a", addr: "ic2:iridium_storage_box_0" }],
        backups: vec![
            (Filter::Label("Corn"), 32),
            (Filter::Label("Seeds"), 32),
            (Filter::Label("Potato"), 32),
            (Filter::Label("Cotton"), 32),
            (Filter::Label("Peanut"), 32),
            (Filter::Label("Cactus"), 32),
            (Filter::Label("Soybean"), 32),
            (Filter::Label("Dandelion"), 32),
            (Filter::Label("Cranberry"), 32),
            (Filter::Label("Sugar Canes"), 32),
            (Filter::Label("Nether Wart"), 32),
            (Filter::Label("Sweet Potato"), 32),
            (Filter::Label("Psimetal Ingot"), 32),
            (Filter::Label("Ebony Substance"), 32),
            (Filter::Label("Ivory Substance"), 32),
            (Filter::Label("Blue Slime Sapling"), 32),
            (Filter::Name("minecraft:red_mushroom"), 32),
            (Filter::Name("minecraft:brown_mushroom"), 32),
        ],
    }
    .build(|factory| {
        factory.add_storage(DrawerConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "storagedrawers:controller_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            filters: vec![
                Filter::Label("Coal"),
                Filter::Label("Bone"),
                Filter::Label("Seeds"),
                Filter::Label("Flint"),
                Filter::Label("Emerald"),
                Filter::Label("Diamond"),
                Filter::Label("Coal Coke"),
                Filter::Label("Firm Tofu"),
                Filter::Label("Wither Ash"),
                Filter::Label("Witherbone"),
                Filter::Label("Aquamarine"),
                Filter::Label("Lapis Lazuli"),
                Filter::Label("Uranium Seeds"),
                Filter::Label("Cyanite Ingot"),
                Filter::Label("Iridium Ingot"),
                Filter::Label("Tin Ore Piece"),
                Filter::Label("Iron Ore Piece"),
                Filter::Label("Gold Ore Piece"),
                Filter::Label("Lead Ore Piece"),
                Filter::Label("Platinum Ingot"),
                Filter::Label("Glowstone Dust"),
                Filter::Label("Boron Ore Piece"),
                Filter::Label("Pulverized Gold"),
                Filter::Label("Experience Seeds"),
                Filter::Label("Silver Ore Piece"),
                Filter::Label("Nickel Ore Piece"),
                Filter::Label("Copper Ore Piece"),
                Filter::Label("Osmium Ore Piece"),
                Filter::Label("Cobalt Ore Piece"),
                Filter::Label("Ardite Ore Piece"),
                Filter::Label("Lithium Ore Piece"),
                Filter::Label("Dimensional Shard"),
                Filter::Label("Mana Infused Ingot"),
                Filter::Label("Aluminium Ore Piece"),
                Filter::Label("Magnesium Ore Piece"),
                Filter::Label("Crushed Black Quartz"),
                Filter::Label("Certus Quartz Crystal"),
                Filter::Label("Tiny Clump of Uranium-235"),
                Filter::Label("Charged Certus Quartz Crystal"),
                Filter::Label("Nether Star Essence"),
                Filter::Label("Grains of Infinity"),
                Filter::Label("Sky Stone Dust"),
                Filter::Label("Draconium Ore"),
                Filter::Label("Nether Quartz"),
                Filter::Label("Oak Sapling"),
                Filter::Label("Uranium-233"),
                Filter::Label("Cobalt Ore"),
                Filter::Label("Ardite Ore"),
                Filter::Label("Grain Bait"),
                Filter::Label("Rich Slag"),
                Filter::Label("Gold Ore"),
                Filter::Name("mysticalagriculture:tier4_inferium_seeds"),
                congealed_green(),
                Filter::Label("Litherite Crystal"),
                Filter::Label("Kyronite Crystal"),
                Filter::Label("Erodium Crystal"),
                Filter::Label("Pladium Crystal"),
                Filter::Label("Aethium Crystal"),
                Filter::Label("Ionite Crystal"),
            ],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchestlarge_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchestlarge_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchestlarge_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchestlarge_3",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchestlarge_4",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchestlarge_5",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
        });
        factory.add_process(BufferedConfig {
            name: "platePressOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_4",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            stocks: vec![],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "gearPressOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_5",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            stocks: vec![],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "wirePressOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_9",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            stocks: vec![],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "rodPressOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_19",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            stocks: vec![],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "harvesterOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:crop_recolector_tile_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: Some(Box::new(|slot, _| slot >= 6)),
            stocks: vec![],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "transmutationOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:ender chest_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            stocks: vec![],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "atomicOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:rangedcollector_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            stocks: vec![],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "phytoOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_14",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            stocks: vec![],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "pulverizerOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_17",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            stocks: vec![],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:cobblestone_generator_compact_5",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Cobblestone"), n_wanted: 64 }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_13",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: congealed_blue(), n_wanted: 64 }],
        });
        factory.add_process(BufferedConfig {
            name: "ethylene",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_10",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![BufferedInput::new(Filter::Label("Bio Fuel"), 64)],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "mkCrusher",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_15",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 24,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gunpowder"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Flint"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bio Fuel"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Potato"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gravel"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cobblestone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Coal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Coal"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Charcoal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Charcoal"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lapis Lazuli Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lapis Lazuli"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nether Quartz Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Quartz"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Certus Quartz Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Certus Quartz Crystal"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fluix Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Fluix Crystal"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Diamond Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Diamond"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Coke Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Coal Coke"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterA",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("String"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cotton"), 3)],
                    max_inputs: 48,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("White Wool"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("String"), 4)],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("White Concrete Powder"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Bone Meal"), 1),
                        BufferedInput::new(Filter::Label("Sand"), 4),
                        BufferedInput::new(Filter::Label("Gravel"), 4),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Torch"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Stick"), 1),
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lever"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Stick"), 1),
                        BufferedInput::new(Filter::Label("Cobblestone"), 1),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crafting Table"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Oak Wood Planks"), 4)],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Button"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Stone"), 1)],
                    max_inputs: 16,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sturdy Casing"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Copper Gear"), 2),
                        BufferedInput::new(Filter::Label("Bronze Gear"), 2),
                        BufferedInput::new(Filter::Label("Bronze Ingot"), 4),
                    ],
                    max_inputs: 64,
                },
            ],
        });
        factory.add_process(ScatteringConfig {
            name: "planter",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:crop_sower_tile_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![6, 7, 8, 9, 11, 12, 13, 14],
            to_extract: None,
            recipes: vec![
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Soybean"), n_wanted: 64 }],
                    ScatteringInput::new(Filter::Label("Soybean")).allow_backup(),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Cotton"), n_wanted: 64 }],
                    ScatteringInput::new(Filter::Label("Cotton")).allow_backup(),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Peanut"), n_wanted: 64 }],
                    ScatteringInput::new(Filter::Label("Peanut")).allow_backup(),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Sweet Potato"), n_wanted: 64 }],
                    ScatteringInput::new(Filter::Label("Sweet Potato")).allow_backup(),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Cranberry"), n_wanted: 64 }],
                    ScatteringInput::new(Filter::Label("Cranberry")).allow_backup(),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Corn"), n_wanted: 64 }],
                    ScatteringInput::new(Filter::Label("Corn")).allow_backup(),
                ),
            ],
            max_per_slot: 4,
        });
        factory.add_process(ScatteringConfig {
            name: "furnace",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:itemviewer_5",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0, 1, 2, 3, 4, 5, 6],
            to_extract: None,
            max_per_slot: 8,
            recipes: vec![
                ScatteringRecipe::new(vec![], ScatteringInput::new(Filter::Label("Pulverized Mana Infused Metal"))),
                ScatteringRecipe::new(vec![], ScatteringInput::new(Filter::Label("Pulverized Platinum"))),
                ScatteringRecipe::new(vec![], ScatteringInput::new(Filter::Label("Pulverized Iridium"))),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Cactus Green"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Cactus")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Plastic"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Dry Rubber")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Name("minecraft:netherbrick"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Netherrack")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Brick"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Name("minecraft:clay_ball")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Bread"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Both { name: "appliedenergistics2:material", label: "Flour" }),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Baked Potato"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Potato")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Glass"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Sand")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Stone"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Cobblestone")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Charcoal"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Oak Wood")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Graphite Ingot"), n_wanted: 64 }],
                    ScatteringInput::new(Filter::Label("Charcoal")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Aluminum Ingot"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Pulverized Aluminum")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Silver Ingot"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Pulverized Silver")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Osmium Ingot"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Osmium Dust")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Magnesium Ingot"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Magnesium Dust")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Draconium Ingot"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Draconium Dust")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Thorium Ingot"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Thorium Dust")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Copper Ingot"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Pulverized Copper")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Iron Ingot"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Pulverized Iron")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Gold Ingot"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Pulverized Gold")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Lead Ingot"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Pulverized Lead")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Tin Ingot"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Pulverized Tin")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Cobalt Ingot"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Cobalt Ore Dust")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Boron Ingot"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Boron Dust")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Ardite Ingot"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Ardite Ore Dust")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Starmetal Ingot"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Stardust")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Cooked Tofurkey"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Raw Tofurkey")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Black Quartz"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Crushed Black Quartz")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Thickened Glass"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Sandy Glass")),
                ),
            ],
        });
        factory.add_process(SlottedConfig {
            name: "pressurizer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:pressurizer_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Diamond"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Graphite Ingot"), 64, vec![0])],
                    max_per_slot: 64,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fluorite"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Crushed Fluorite"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cubic Boron Nitride"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Hexagonal Boron Nitride"), 1, vec![0])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "decay",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_15",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Tiny Clump of Neptunium-236"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Tiny Clump of Neptunium-237"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Tiny Clump of Plutonium-239"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Tiny Clump of Americium-241"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Tiny Clump of Americium-243"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Tiny Clump of Curium-245"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Tiny Clump of Berkelium-248"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Tiny Clump of Californium-251"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Plutonium-242"), 1).extra_backup(64)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Curium-247"), 1).extra_backup(64)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Californium-249"), 1).extra_backup(64)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "manufactory",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:manufactory_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bioplastic"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Sugar Canes"), 2, vec![0])],
                    max_per_slot: 64,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bone Meal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Bone"), 1, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Flint"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Gravel"), 1, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silicon Ingot"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Sand"), 1, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Clay Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Name("minecraft:clay"), 1, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Oak Wood Planks"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Oak Wood"), 1, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sand"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Cobblestone"), 1, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Aluminum"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Aluminum Ore"), 1, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Silver"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Silver Ore"), 1, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Osmium Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Osmium Ore"), 1, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cobalt Ore Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Cobalt Ore"), 1, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ardite Ore Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Ardite Ore"), 1, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Magnesium Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Magnesium Ore"), 1, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Niter"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Sandstone"), 1, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electrum Blend"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Electrum Ingot"), 1, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output {
                        item: Filter::Both { name: "appliedenergistics2:material", label: "Flour" },
                        n_wanted: 16,
                    }],
                    inputs: vec![SlottedInput::new(Filter::Label("Wheat"), 1, vec![0])],
                    max_per_slot: 32,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "alloyFurnace",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:alloy_furnace_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0, 1],
            to_extract: Some(Box::new(|slot, _| slot == 2)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dark Steel Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Steel Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Obsidian"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Alloy Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Silicon Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Redstone"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electrical Steel Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Steel Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Silicon Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Titanium Aluminide Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Pulverized Aluminum"), 7, vec![0]),
                        SlottedInput::new(Filter::Label("Titanium Ingot"), 3, vec![1]),
                    ],
                    max_per_slot: 63,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Pulverized Iron"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Graphite Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ferroboron Alloy"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Steel Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Boron Dust"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Tough Alloy"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Ferroboron Alloy"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Lithium Dust"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Titanium Iridium Alloy Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Titanium Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Iridium Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Manyullyn Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Cobalt Ore Dust"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Ardite Ore Dust"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bronze Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Pulverized Copper"), 3, vec![0]),
                        SlottedInput::new(Filter::Label("Pulverized Tin"), 1, vec![1]),
                    ],
                    max_per_slot: 48,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electrum Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Pulverized Gold"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Pulverized Silver"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Invar Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Pulverized Iron"), 2, vec![0]),
                        SlottedInput::new(Filter::Label("Nickel Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fused Quartz"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Nether Quartz"), 4, vec![0]),
                        SlottedInput::new(Filter::Label("Block of Quartz"), 1, vec![1]),
                    ],
                    max_per_slot: 64,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Tin Silver Alloy"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Pulverized Tin"), 3, vec![0]),
                        SlottedInput::new(Filter::Label("Pulverized Silver"), 1, vec![1]),
                    ],
                    max_per_slot: 48,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Platinum Alloy"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Pulverized Lead"), 3, vec![0]),
                        SlottedInput::new(Filter::Label("Platinum Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 48,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Shibuichi Alloy"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Pulverized Copper"), 3, vec![0]),
                        SlottedInput::new(Filter::Label("Pulverized Silver"), 1, vec![1]),
                    ],
                    max_per_slot: 48,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Constantan Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Pulverized Copper"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Nickel Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Magnesium Diboride Alloy"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Magnesium Dust"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Boron Dust"), 2, vec![1]),
                    ],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Hard Carbon Alloy"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Graphite Ingot"), 2, vec![0]),
                        SlottedInput::new(Filter::Label("Diamond"), 1, vec![1]),
                    ],
                    max_per_slot: 32,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "platePress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_5",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 8,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Tin Item Casing"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Tin Plate"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Item Casing"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Copper Plate"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Item Casing"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lead Plate"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lead Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 1)],
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
                    outputs: vec![Output { item: Filter::Label("Invar Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Invar Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bronze Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Bronze Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Tin Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Tin Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminum Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Aluminum Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Uranium Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Uranium Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gold Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nickel Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nickel Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Signalum Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Signalum Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Brass Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Alchemical Brass Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electrum Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Electrum Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electrum Large Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Block of Electrum"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fluxed Electrum Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Fluxed Electrum Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Titanium Aluminide Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Titanium Aluminide Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "gearPress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_6",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 32,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dark Bimetal Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dark Steel Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Enderium Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Enderium Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bronze Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Bronze Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Copper Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
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
                    outputs: vec![Output { item: Filter::Label("Invar Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Invar Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silver Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Silver Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lumium Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lumium Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electrum Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Electrum Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "wirePress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_7",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 8,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminium Wire"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Aluminum Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Wire"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Copper Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "rodPress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_12",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 8,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Ardite Tool Rod"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Ardite Ingot"), 1)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "waterInfuser",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:infuser_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("White Concrete"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("White Concrete Powder"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Infinite Water Source"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Empty Frame"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Black Concrete"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Black Concrete Powder"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Water Cooler"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Empty Cooler"), 1, vec![0])],
                    max_per_slot: 16,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "heliumInfuser",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:infuser_4",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Liquid Helium Cooler"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Empty Cooler"), 1, vec![0])],
                max_per_slot: 8,
            }],
        });
        for inv_addr in ["excompressum:auto_compressor_0", "excompressum:auto_compressor_1"] {
            factory.add_process(BufferedConfig {
                name: "autoCompressor",
                accesses: vec![BusAccess { client: "1a", inv_addr, bus_addr: "ic2:iridium_storage_box_0" }],
                slot_filter: Some(Box::new(|slot| slot < 12)),
                to_extract: Some(Box::new(|slot, _| slot >= 12)),
                stocks: vec![],
                max_recipe_inputs: 128,
                recipes: vec![
                    BufferedRecipe {
                        outputs: vec![],
                        inputs: vec![BufferedInput::new(Filter::Label("Tiny Clump of Thorium-230"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![],
                        inputs: vec![BufferedInput::new(Filter::Label("Tiny Clump of Thorium-232"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![],
                        inputs: vec![BufferedInput::new(Filter::Label("Tiny Clump of Uranium-233"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![],
                        inputs: vec![BufferedInput::new(Filter::Label("Tiny Clump of Plutonium-241"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![],
                        inputs: vec![BufferedInput::new(Filter::Label("Tiny Clump of Plutonium-242"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![],
                        inputs: vec![BufferedInput::new(Filter::Label("Tiny Clump of Curium-246"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![],
                        inputs: vec![BufferedInput::new(Filter::Label("Tiny Clump of Curium-247"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![],
                        inputs: vec![BufferedInput::new(Filter::Label("Tiny Clump of Californium-249"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![],
                        inputs: vec![BufferedInput::new(Filter::Label("Tiny Clump of Californium-250"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![],
                        inputs: vec![BufferedInput::new(Filter::Label("Tiny Clump of Californium-252"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![],
                        inputs: vec![BufferedInput::new(Filter::Label("Thorium-230"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Redstone"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Redstone Essence"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Draconium Block"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Draconium Ingot"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Pladium"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Pladium Crystal"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Aethium"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Aethium Crystal"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Stable-'Unstable Ingot'"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Stable-'Unstable Nugget'"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("TBU Fuel"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Thorium-232"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Block of Tin"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Tin Ingot"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Cyanite Block"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Cyanite Ingot"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Block of Cobalt"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Cobalt Ingot"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Block of Mana Infused Metal"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Mana Infused Ingot"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Double Compressed Crafting Table"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Compressed Crafting Table"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Compressed Crafting Table"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Crafting Table"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Mirion Block"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Mirion Ingot"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Block of Enderium"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Enderium Ingot"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Blaze Mesh"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Blaze Rod"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Block of Elementium"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Elementium Ingot"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Nether Star Shard"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Nether Star Essence"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Dry Rubber"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Tiny Dry Rubber"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Bone Block"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Bone Meal"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Block of Quartz"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Nether Quartz"), 4)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Block of Black Quartz"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Black Quartz"), 4)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output {
                            item: Filter::Both { label: "Block of Black Iron", name: "extendedcrafting:storage" },
                            n_wanted: 16,
                        }],
                        inputs: vec![BufferedInput::new(Filter::Label("Black Iron Ingot"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Block of Diamond"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Diamond"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Block of Invar"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Invar Ingot"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Compressed Soul Sand"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Soul Sand"), 9)],
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
                        outputs: vec![Output { item: Filter::Label("Compressed Ender Gravel"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Crushed Endstone"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Compressed Nether Gravel"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Crushed Netherrack"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Graphite Block"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Graphite Ingot"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Lapis Lazuli Block"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Lapis Lazuli"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Osmium Block"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Osmium Ingot"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Block of Iron"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Block of Lead"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Lead Ingot"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Block of Emerald"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Emerald"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Block of Electrum"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Electrum Ingot"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Block of Redstone"), n_wanted: 40 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Redstone"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Block of Coal"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Coal"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Compressed Cobblestone"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Cobblestone"), 9)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Magnesium Ore"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Magnesium Ore Piece"), 4)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Thorium Ore"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Thorium Ore Piece"), 4)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Lithium Ore"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Lithium Ore Piece"), 4)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Ardite Ore"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Ardite Ore Piece"), 4)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Cobalt Ore"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Cobalt Ore Piece"), 4)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Boron Ore"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Boron Ore Piece"), 4)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Copper Ore"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Copper Ore Piece"), 4)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Aluminum Ore"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Aluminium Ore Piece"), 4)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Nickel Ore"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Nickel Ore Piece"), 4)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Silver Ore"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Silver Ore Piece"), 4)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Iron Ore"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Iron Ore Piece"), 4)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Lead Ore"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Lead Ore Piece"), 4)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Tin Ore"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Tin Ore Piece"), 4)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Osmium Ore"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Osmium Ore Piece"), 4)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Gold Ore"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Gold Ore Piece"), 4)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Green Slime Block"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Name("minecraft:slime_ball"), 4)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Osgloglas Block"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Osgloglas Ingot"), 9)],
                        max_inputs: i32::MAX,
                    },
                ],
            });
        }
        factory.add_process(BufferedConfig {
            name: "atomic",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "randomthings:irondropper_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 8,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Diamatine Crystal Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Block of Diamond"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Restonia Crystal Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Block of Redstone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Palis Crystal Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lapis Lazuli Block"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Enori Crystal Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Block of Iron"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Emeradic Crystal Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Block of Emerald"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Void Crystal Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Block of Coal"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Soul Sand"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sand"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Rhodochrosite"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Ruby"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ethetic Green Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Chiseled Quartz Block"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "waterBarrel",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_9",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 8,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Name("minecraft:clay"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Dust"), 1)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "factorizer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_factorizer_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![],
                    inputs: vec![SlottedInput::new(Filter::Label("Americium-243"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ludicrite Ingot"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Ludicrite Block"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Enori Crystal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Enori Crystal Block"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Palis Crystal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Palis Crystal Block"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Palis Crystal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Empowered Palis Crystal Block"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Diamatine Crystal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Empowered Diamatine Crystal Block"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Enori Crystal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Empowered Enori Crystal Block"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Void Crystal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Empowered Void Crystal Block"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Emeradic Crystal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Empowered Emeradic Crystal Block"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Restonia Crystal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Empowered Restonia Crystal Block"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Restonia Crystal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Restonia Crystal Block"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Nugget"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Iron Ingot"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gold Nugget"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Gold Ingot"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crystaltine Nugget"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Crystaltine Ingot"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Blutonium Ingot"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Blutonium Block"), 1, vec![0])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "arPress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:placer_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 8,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Stick"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Oak Wood Planks"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Rod"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Steel Sheetmetal"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Rod"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Sheetmetal"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminium Rod"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Aluminium Sheetmetal"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "melters",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_8",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![
                BufferedInput::new(Filter::Label("Salt"), 64),
                BufferedInput::new(Filter::Label("Sulfur"), 64),
                BufferedInput::new(Filter::Label("Peanut"), 64),
                BufferedInput::new(Filter::Label("Redstone"), 64),
                BufferedInput::new(Filter::Label("Boron Dust"), 64),
                BufferedInput::new(Filter::Label("Ender Pearl"), 64),
                BufferedInput::new(Filter::Label("Lumium Ingot"), 64),
                BufferedInput::new(Filter::Label("Cryotheum Dust"), 64),
                BufferedInput::new(Filter::Label("Glowstone Dust"), 64),
                BufferedInput::new(Filter::Label("Crushed Fluorite"), 64),
                BufferedInput::new(Filter::Label("Lapis Lazuli Dust"), 64),
                BufferedInput::new(Filter::Label("Blue Slime Sapling"), 64),
                BufferedInput::new(Filter::Label("Experience Essence"), 64),
            ],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "advCarpenters",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_32",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![
                BufferedInput::new(Filter::Label("Genetics Labware"), 64),
                BufferedInput::new(Filter::Label("Osgloglas Ingot"), 64),
                BufferedInput::new(Filter::Label("Spectral Drone"), 64),
                BufferedInput::new(Filter::Label("Mirion Ingot"), 64),
            ],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "pulverizer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_15",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![BufferedInput::new(Filter::Label("Fluxed Phyto-Gro"), 64)],
            max_recipe_inputs: 48,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Cinnabar"), n_wanted: 16 },
                        Output { item: Filter::Label("Pulverized Gold"), n_wanted: 16 },
                    ],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("harvestcraft:flouritem"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Wheat"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dandelion Yellow"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dandelion"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ground Cinnamon"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cinnamon"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sand"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crushed Endstone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("End Stone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crushed Netherrack"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Netherrack"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crushed Rhodochrosite"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Rhodochrosite"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Enderium Blend"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Enderium Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "mkInfuser",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_13",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![
                BufferedInput::new(Filter::Label("Compressed Redstone"), 64),
                BufferedInput::new(Filter::Label("Compressed Obsidian"), 64),
                BufferedInput::new(Filter::Label("Compressed Diamond"), 64),
                BufferedInput::new(Filter::Label("Osmium Ingot"), 64),
            ],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "kekimurus",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:placer_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![BufferedInput::new(Filter::Label("Cake"), 9)],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "supplies",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:ender chest_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![
                BufferedInput::new(Filter::Label("White Concrete"), 256),
                BufferedInput::new(Filter::Label("Super Lubricent Stone"), 256),
                BufferedInput::new(Filter::Label("Transparent Fission Reactor Casing"), 256),
                BufferedInput::new(Filter::Label("Cryotheum Cooler"), 256),
                BufferedInput::new(Filter::Label("Reactor Cell"), 256),
            ],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "impregStick",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:carpenter_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![12],
            to_extract: Some(Box::new(|slot, _| slot == 10)),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Impregnated Stick"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Oak Wood"), 2, vec![12])],
                max_per_slot: 16,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "overclocker",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:carpenter_3",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![12, 13, 14],
            to_extract: Some(Box::new(|slot, _| slot == 10)),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Overclocker Upgrade"), n_wanted: 16 }],
                inputs: vec![
                    SlottedInput::new(Filter::Label("Tin Plate"), 6, vec![12]),
                    SlottedInput::new(Filter::Label("Copper Cable"), 2, vec![13]),
                    SlottedInput::new(Filter::Label("Basic Control Circuit"), 1, vec![14]),
                ],
                max_per_slot: 24,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "lightwell",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "xu2:tileuse_6",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Aquamarine"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "fusion",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:melter_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Lithium Dust"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "dropInLiquid",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "randomthings:irondropper_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Infused Wood"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Oak Wood"), 1)],
                    max_inputs: 8,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fluix Crystal"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Charged Certus Quartz Crystal"), 1),
                        BufferedInput::new(Filter::Label("Nether Quartz"), 1),
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                    ],
                    max_inputs: 24,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "flintSieve",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "excompressum:auto_heavy_sieve_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot < 21)),
            recipes: vec![SlottedRecipe {
                outputs: vec![
                    Output { item: Filter::Label("Aluminium Ore Piece"), n_wanted: 16 },
                    Output { item: Filter::Label("Nickel Ore Piece"), n_wanted: 16 },
                    Output { item: Filter::Label("Copper Ore Piece"), n_wanted: 16 },
                    Output { item: Filter::Label("Silver Ore Piece"), n_wanted: 16 },
                    Output { item: Filter::Label("Iron Ore Piece"), n_wanted: 16 },
                    Output { item: Filter::Label("Lead Ore Piece"), n_wanted: 16 },
                    Output { item: Filter::Label("Tin Ore Piece"), n_wanted: 16 },
                    Output { item: Filter::Label("Lapis Lazuli"), n_wanted: 16 },
                    Output { item: Filter::Label("Coal"), n_wanted: 16 },
                ],
                inputs: vec![SlottedInput::new(Filter::Label("Compressed Gravel"), 1, vec![0])],
                max_per_slot: 8,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "ironSieve",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "excompressum:auto_heavy_sieve_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot < 21)),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Glowstone Dust"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Compressed Soul Sand"), 1, vec![0])],
                max_per_slot: 8,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "diamondSieve",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "excompressum:auto_heavy_sieve_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot < 21)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Diamond"), n_wanted: 16 },
                        Output { item: Filter::Label("Emerald"), n_wanted: 16 },
                        Output { item: Filter::Label("Osmium Ore Piece"), n_wanted: 16 },
                        Output { item: Filter::Label("Crushed Black Quartz"), n_wanted: 16 },
                    ],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed Gravel"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aquamarine"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed Sand"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nether Quartz"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed Soul Sand"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Magnesium Ore Piece"), n_wanted: 16 },
                        Output { item: Filter::Label("Thorium Ore Piece"), n_wanted: 16 },
                        Output { item: Filter::Label("Lithium Ore Piece"), n_wanted: 16 },
                        Output { item: Filter::Label("Ardite Ore Piece"), n_wanted: 16 },
                        Output { item: Filter::Label("Cobalt Ore Piece"), n_wanted: 16 },
                        Output { item: Filter::Label("Boron Ore Piece"), n_wanted: 16 },
                        Output { item: Filter::Label("Gold Ore Piece"), n_wanted: 16 },
                    ],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed Nether Gravel"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Ruby"), n_wanted: 16 },
                        Output { item: Filter::Label("Sapphire"), n_wanted: 16 },
                        Output { item: Filter::Label("Malachite"), n_wanted: 16 },
                    ],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed Ender Gravel"), 1, vec![0])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "ironElectronTube",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:fabricator_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0, 3, 4],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Iron Electron Tube"), n_wanted: 16 }],
                inputs: vec![
                    SlottedInput::new(Filter::Label("Sand"), 1, vec![0]),
                    SlottedInput::new(Filter::Label("Iron Ingot"), 10, vec![3]),
                    SlottedInput::new(Filter::Label("Redstone"), 4, vec![4]),
                ],
                max_per_slot: 40,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "enderElectronTube",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:fabricator_5",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0, 3, 4],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Ender Electron Tube"), n_wanted: 16 }],
                inputs: vec![
                    SlottedInput::new(Filter::Label("Sand"), 1, vec![0]),
                    SlottedInput::new(Filter::Label("End Stone"), 10, vec![3]),
                    SlottedInput::new(Filter::Label("Eye of Ender"), 4, vec![4]),
                ],
                max_per_slot: 40,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "tinElectronTube",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:fabricator_1",
                bus_addr: "ic2:iridium_storage_box_0",
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
            name: "crafterB",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_5",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Stone Bricks"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Stone"), 4)],
                    max_inputs: 16,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Polished Stone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Stone Bricks"), 4)],
                    max_inputs: 16,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Gear"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Redstone Torch"), 4),
                        BufferedInput::new(Filter::Label("Oak Wood Planks"), 1),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Analog Crafter"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Crafting Table"), 8),
                        BufferedInput::new(Filter::Label("Lever"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Machine Block"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Polished Stone"), 4),
                        BufferedInput::new(Filter::Label("Iron Electron Tube"), 4),
                        BufferedInput::new(Filter::Label("Sturdy Casing"), 1),
                    ],
                    max_inputs: 54,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crafter Tier 1"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Analog Crafter"), 2),
                        BufferedInput::new(Filter::Label("Redstone Gear"), 2),
                        BufferedInput::new(Filter::Label("Machine Block"), 1),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crafter Tier 2"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Analog Crafter"), 2),
                        BufferedInput::new(Filter::Label("Redstone Gear"), 2),
                        BufferedInput::new(Filter::Label("Crafter Tier 1"), 1),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crafter Tier 3"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Analog Crafter"), 2),
                        BufferedInput::new(Filter::Label("Redstone Gear"), 2),
                        BufferedInput::new(Filter::Label("Crafter Tier 2"), 1),
                    ],
                    max_inputs: 80,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterC",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_27",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Sheetmetal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Steel Plate"), 4)],
                    max_inputs: 16,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Scaffolding"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Steel Ingot"), 3),
                        BufferedInput::new(Filter::Label("Steel Rod"), 3),
                    ],
                    max_inputs: 18,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basic Coil"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Enori Crystal"), 2),
                        BufferedInput::new(Filter::Label("Impregnated Stick"), 3),
                        BufferedInput::new(Filter::Label("Aluminium Wire"), 4),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Sheetmetal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lead Plate"), 4)],
                    max_inputs: 16,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Raw Carbon Fibre"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Coal"), 4)],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Raw Carbon Mesh"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Raw Carbon Fibre"), 2)],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Mixed Metal Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iron Plate"), 3),
                        BufferedInput::new(Filter::Label("Bronze Plate"), 3),
                        BufferedInput::new(Filter::Label("Tin Plate"), 3),
                    ],
                    max_inputs: 72,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basic Machine Casing"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Aluminum Plate"), 4),
                        BufferedInput::new(Filter::Label("Dense Iron Plate"), 4),
                    ],
                    max_inputs: 32,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "rubber",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "appliedenergistics2:interface_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![BufferedInput::new(Filter::Label("Oak Wood"), 64)],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:latex_processing_unit_tile_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Tiny Dry Rubber"), n_wanted: 64 }],
        });
        factory.add_process(SlottedConfig {
            name: "compressor",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:compressor_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![6],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dense Lapis Lazuli Plate"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lapis Lazuli Plate"), 9, vec![6])],
                    max_per_slot: 63,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lapis Lazuli Plate"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lapis Lazuli Dust"), 1, vec![6])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Advanced Alloy"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Mixed Metal Ingot"), 1, vec![6])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Carbon Plate"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Raw Carbon Mesh"), 1, vec![6])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Blaze Rod"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Blaze Powder"), 5, vec![6])],
                    max_per_slot: 40,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dense Iron Plate"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Iron Plate"), 9, vec![6])],
                    max_per_slot: 63,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dense Steel Plate"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Steel Plate"), 9, vec![6])],
                    max_per_slot: 63,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dense Copper Plate"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Copper Plate"), 9, vec![6])],
                    max_per_slot: 63,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Energy Crystal"), n_wanted: 4 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Energium Dust"), 9, vec![6])],
                    max_per_slot: 36,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterD",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_6",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Advanced Machine Casing"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Steel Plate"), 4),
                        BufferedInput::new(Filter::Label("Carbon Plate"), 2),
                        BufferedInput::new(Filter::Label("Advanced Alloy"), 2),
                        BufferedInput::new(Filter::Label("Basic Machine Casing"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basic Plating"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Lead Sheetmetal"), 4),
                        BufferedInput::new(Filter::Label("Lead Item Casing"), 4),
                        BufferedInput::new(Filter::Label("Graphite Block"), 1),
                    ],
                    max_inputs: 36,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empty Frame"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Basic Plating"), 4),
                        BufferedInput::new(Filter::Label("Iron Ingot"), 2),
                        BufferedInput::new(Filter::Label("Tin Ingot"), 2),
                    ],
                    max_inputs: 128,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Compact Infinite Water Source"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Infinite Water Source"), 8),
                        BufferedInput::new(Filter::Label("Bronze Ingot"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Device Frame"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Glass"), 4),
                        BufferedInput::new(Filter::Label("Tin Ingot"), 4),
                        BufferedInput::new(Filter::Label("Copper Gear"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("minecraft:clay_ball"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Name("minecraft:clay"), 1)],
                    max_inputs: 4,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Mechanical Component"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iron Plate"), 4),
                        BufferedInput::new(Filter::Label("Copper Ingot"), 1),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Heavy Engineering Block"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Uranium Plate"), 4),
                        BufferedInput::new(Filter::Label("Reinforced Alloy"), 2),
                        BufferedInput::new(Filter::Label("Iron Mechanical Component"), 2),
                        BufferedInput::new(Filter::Label("Steel Scaffolding"), 1),
                    ],
                    max_inputs: 36,
                },
            ],
        });
        factory.add_process(ScatteringConfig {
            name: "enrichment",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:itemviewer_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
            to_extract: None,
            max_per_slot: 8,
            recipes: vec![
                ScatteringRecipe::new(vec![], ScatteringInput::new(Filter::Label("Dimensional Shard Ore"))),
                ScatteringRecipe::new(vec![], ScatteringInput::new(Filter::Label("Nether Quartz Ore"))),
                ScatteringRecipe::new(vec![], ScatteringInput::new(Filter::Label("Diamond Ore"))),
                ScatteringRecipe::new(vec![], ScatteringInput::new(Filter::Label("Emerald Ore"))),
                ScatteringRecipe::new(vec![], ScatteringInput::new(Filter::Label("Glowstone"))),
                ScatteringRecipe::new(vec![], ScatteringInput::new(Filter::Label("Charged Certus Quartz Ore"))),
                ScatteringRecipe::new(vec![], ScatteringInput::new(Filter::Label("Mana Infused Ore"))),
                ScatteringRecipe::new(vec![], ScatteringInput::new(Filter::Label("Platinum Ore"))),
                ScatteringRecipe::new(vec![], ScatteringInput::new(Filter::Label("Iridium Ore"))),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Compressed Obsidian"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Refined Obsidian Dust")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Compressed Redstone"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Redstone")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Compressed Diamond"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Diamond")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Pulverized Obsidian"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Obsidian")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Pulverized Copper"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Copper Ore")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Pulverized Lead"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Lead Ore")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Pulverized Iron"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Iron Ore")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Pulverized Tin"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Tin Ore")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Thorium Dust"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Thorium Ore")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Lithium Dust"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Lithium Ore")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Boron Dust"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Boron Ore")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Draconium Dust"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Draconium Ore")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Pure Nether Quartz Crystal"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Nether Quartz Seed")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Pure Certus Quartz Crystal"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Certus Quartz Seed")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Pure Fluix Crystal"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Fluix Seed")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Stardust"), n_wanted: 16 }],
                    ScatteringInput::new(Filter::Label("Starmetal Ore")),
                ),
            ],
        });
        factory.add_process(BufferedConfig {
            name: "redstoneInfuser",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_6",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 8,
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
                    outputs: vec![Output { item: Filter::Label("Redstone Conductance Coil"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Electrum Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Transmission Coil"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Silver Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Energy Cell Frame"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(
                        Filter::Both { name: "thermalexpansion:frame", label: "Machine Frame" },
                        1,
                    )],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "diamondInfuser",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_7",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 8,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Reinforced Alloy"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Enriched Alloy"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Refined Obsidian Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Obsidian"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "obsidianInfuser",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_8",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 8,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Atomic Alloy"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Reinforced Alloy"), 1)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "mkCompressor",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_9",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 8,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Glowstone Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Glowstone Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Refined Obsidian Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Refined Obsidian Dust"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "pinkSlime",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "storagedrawers:compdrawers_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Pink Slime"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Green Slime Block"), 1, vec![0])],
                max_per_slot: 2,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "crafterE",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_12",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Grout"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Gravel"), 4),
                        BufferedInput::new(Filter::Label("Sand"), 4),
                        BufferedInput::new(Filter::Name("minecraft:clay"), 1),
                    ],
                    max_inputs: 18,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Reinforced Stone"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Stone"), 4),
                        BufferedInput::new(Filter::Label("Grout"), 4),
                        BufferedInput::new(Filter::Label("Clay Dust"), 1),
                    ],
                    max_inputs: 9,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Machine Case"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Reinforced Stone"), 4),
                        BufferedInput::new(Filter::Label("Plastic"), 4),
                        BufferedInput::new(Filter::Label("Advanced Machine Casing"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Sheetmetal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Plate"), 4)],
                    max_inputs: 16,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Casing"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iron Sheetmetal"), 4),
                        BufferedInput::new(Filter::Label("Tin Electron Tube"), 4),
                        BufferedInput::new(Filter::Label("Hardened Casing"), 1),
                    ],
                    max_inputs: 72,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Glass Pane"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Glass"), 6)],
                    max_inputs: 6,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Range Addon"), n_wanted: 4 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Cobblestone"), 6),
                        BufferedInput::new(Filter::Label("Plastic"), 2),
                        BufferedInput::new(Filter::Label("Glass Pane"), 1),
                    ],
                    max_inputs: 36,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Bars"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 6)],
                    max_inputs: 6,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterF",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_16",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Coil"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Copper Cable"), 8),
                        BufferedInput::new(Filter::Label("Iron Ingot"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electric Motor"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Coil"), 2),
                        BufferedInput::new(Filter::Label("Tin Item Casing"), 2),
                        BufferedInput::new(Filter::Label("Iron Ingot"), 1),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Heat Vent"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iron Bars"), 4),
                        BufferedInput::new(Filter::Label("Iron Plate"), 4),
                        BufferedInput::new(Filter::Label("Electric Motor"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("rftools:machine_frame"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Heat Vent"), 2),
                        BufferedInput::new(Filter::Label("Pink Slime"), 2),
                        BufferedInput::new(Filter::Label("Dry Rubber"), 2),
                        BufferedInput::new(Filter::Label("Machine Case"), 1),
                        BufferedInput::new(Filter::Label("Range Addon"), 1),
                        BufferedInput::new(Filter::Label("Gold Gear"), 1),
                    ],
                    max_inputs: 72,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Itemduct"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Tin Ingot"), 2),
                        BufferedInput::new(Filter::Label("Fused Quartz"), 1),
                    ],
                    max_inputs: 9,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Hardened Fluiduct"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Invar Ingot"), 2),
                        BufferedInput::new(Filter::Label("Fused Quartz"), 1),
                    ],
                    max_inputs: 9,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Network Cable"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Lapis Lazuli Plate"), 4),
                        BufferedInput::new(Filter::Label("Gold Plate"), 1),
                        BufferedInput::new(Filter::Label("Redstone"), 4),
                    ],
                    max_inputs: 9,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Leadstone Fluxduct"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Redstone"), 6),
                        BufferedInput::new(Filter::Label("Network Cable"), 2),
                        BufferedInput::new(Filter::Label("Glass"), 1),
                    ],
                    max_inputs: 18,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "extruder",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:metal_former_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![6],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Cable"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Copper Ingot"), 1, vec![6])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gold Cable"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Gold Ingot"), 1, vec![6])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Shaft (Iron)"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Block of Iron"), 1, vec![6])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "teFrame",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:fabricator_3",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0, 3, 4, 5, 6, 7, 8],
            to_extract: extract_all(),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output {
                    item: Filter::Both { name: "thermalexpansion:frame", label: "Machine Frame" },
                    n_wanted: 16,
                }],
                inputs: vec![
                    SlottedInput::new(Filter::Label("Sand"), 6, vec![0]),
                    SlottedInput::new(Filter::Label("Enori Crystal"), 4, vec![3]),
                    SlottedInput::new(Filter::Label("Device Frame"), 1, vec![4]),
                    SlottedInput::new(Filter::Label("Heavy Engineering Block"), 1, vec![5]),
                    SlottedInput::new(Filter::Label("Iron Casing"), 1, vec![6]),
                    SlottedInput::new(Filter::Label("Machine Case"), 1, vec![7]),
                    SlottedInput::new(Filter::Name("rftools:machine_frame"), 1, vec![8]),
                ],
                max_per_slot: 24,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "resonantFrame",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:fabricator_4",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0, 3, 4, 5, 6, 7],
            to_extract: extract_all(),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Resonant Cell Frame (Empty)"), n_wanted: 16 }],
                inputs: vec![
                    SlottedInput::new(Filter::Label("Sand"), 4, vec![0]),
                    SlottedInput::new(Filter::Label("Lumium Ingot"), 2, vec![3]),
                    SlottedInput::new(Filter::Label("Ender Casing"), 1, vec![4]),
                    SlottedInput::new(Filter::Label("Hardened Enderium Glass"), 2, vec![5]),
                    SlottedInput::new(Filter::Label("Signalum Cell Frame (Full)"), 1, vec![6]),
                    SlottedInput::new(Filter::Label("Mana Dust"), 3, vec![7]),
                ],
                max_per_slot: 16,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "xpTransposer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_transposer_9",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 2)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Blizz Powder"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Snowball"), 2, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Blitz Powder"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Niter"), 2, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basalz Powder"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Pulverized Obsidian"), 2, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Blaze Powder"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Sulfur"), 2, vec![0])],
                    max_per_slot: 16,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "glowstoneTransposer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_transposer_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 2)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lumium Ingot"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Tin Silver Alloy"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Impulse Itemduct"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Itemduct"), 1, vec![0])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "lumiumTransposer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_transposer_11",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 2)),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Reinforced Cell Frame (Full)"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Reinforced Cell Frame (Empty)"), 1, vec![0])],
                max_per_slot: 8,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "crafterG",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_20",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sandstone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sand"), 4)],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Clock"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Gold Ingot"), 4),
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                    ],
                    max_inputs: 80,
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
                    outputs: vec![Output { item: Filter::Label("Paper"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sugar Canes"), 3)],
                    max_inputs: 18,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Frame"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iron Mechanical Component"), 1),
                        BufferedInput::new(Filter::Label("Iron Bars"), 4),
                        BufferedInput::new(Filter::Label("Iron Rod"), 4),
                    ],
                    max_inputs: 72,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone-Iron Wiring"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Basic Coil"), 3)],
                    max_inputs: 12,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Servo"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iron Ingot"), 1),
                        BufferedInput::new(Filter::Label("Redstone"), 2),
                    ],
                    max_inputs: 48,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Oak Chest"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Button"), 1),
                        BufferedInput::new(Filter::Label("Oak Wood"), 4),
                        BufferedInput::new(Filter::Label("Treated Wood Planks"), 4),
                    ],
                    max_inputs: 72,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "inductionSmelter",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_smelter_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0, 1],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Rich Slag"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Clock"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Sand"), 1, vec![1]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Nickel Ingot"), n_wanted: 16 },
                        Output { item: Filter::Label("Platinum Ingot"), n_wanted: 16 },
                    ],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Cinnabar"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Nickel Ore"), 1, vec![1]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Black Iron Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Block of Invar"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Tough Alloy"), 1, vec![1]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fluxed Electrum Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Sand"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Fluxed Electrum Blend"), 1, vec![1]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Signalum Cell Frame (Full)"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Block of Redstone"), 40, vec![0]),
                        SlottedInput::new(Filter::Label("Signalum Cell Frame (Empty)"), 1, vec![1]),
                    ],
                    max_per_slot: 40,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Hardened Enderium Glass"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Enderium Blend"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Hardened Glass"), 2, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Hardened Glass"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Pulverized Lead"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Pulverized Obsidian"), 4, vec![1]),
                    ],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminum Brass Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Aluminum Ingot"), 3, vec![0]),
                        SlottedInput::new(Filter::Label("Copper Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 24,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "charger",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_charger_3",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fluxed Phyto-Gro"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Rich Phyto-Gro"), 1, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Charged Certus Quartz Crystal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Certus Quartz Crystal"), 1, vec![0])],
                    max_per_slot: 32,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "phyto",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_16",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Wheat"), n_wanted: 64 },
                        Output { item: Filter::Label("Seeds"), n_wanted: 64 },
                    ],
                    inputs: vec![BufferedInput::new(Filter::Label("Seeds"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Potato"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Potato"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cactus"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cactus"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sugar Canes"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sugar Canes"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nether Wart"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Wart"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dandelion"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dandelion"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Oak Wood"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Oak Sapling"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cinnamon"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cinnamon Sapling"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ender Pearl"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Ender Lilly"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bean"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Bean Seed"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("minecraft:red_mushroom"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Name("minecraft:red_mushroom"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("minecraft:brown_mushroom"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Name("minecraft:brown_mushroom"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Quicksilver"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Shimmerleaf"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pumpkin"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pumpkin Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Blue Slime Sapling"), n_wanted: 64 },
                        Output { item: congealed_green(), n_wanted: 64 },
                    ],
                    inputs: vec![BufferedInput::new(Filter::Label("Blue Slime Sapling"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("§eInferium Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Name("mysticalagriculture:tier4_inferium_seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dye Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dye Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sulfur Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sulfur Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Redstone Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iridium Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iridium Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Uranium Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Uranium Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Knightslime Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Knightslime Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Thaumium Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Thaumium Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fiery Ingot Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Fiery Ingot Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Experience Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Experience Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Void Metal Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Void Metal Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Terrasteel Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Terrasteel Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nether Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "enderTransposer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_transposer_4",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 2)),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Enderium Ingot"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Lead Platinum Alloy"), 1, vec![0])],
                max_per_slot: 8,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "creosoteInfuser",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:infuser_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Treated Wood Planks"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Oak Wood Planks"), 1, vec![0])],
                max_per_slot: 8,
            }],
        });
        factory.add_process(RedstoneEmitterConfig {
            accesses: vec![RedstoneAccess { client: "1a", addr: Some("redstone_integrator_1"), side: EAST }],
            output: emit_when_want_item("cokeFurnace", Filter::Label("Coal Coke"), 16),
        });
        factory.add_process(SlottedConfig {
            name: "cokeFurnace",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_furnace_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: extract_all(),
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Coal"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_11",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Obsidian"), n_wanted: 64 }],
        });
        factory.add_process(BufferedConfig {
            name: "pureDaisy",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "appliedenergistics2:interface_4",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Livingwood"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Infused Wood"), 1)],
                    max_inputs: 8,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Livingrock"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Arcane Stone"), 1)],
                    max_inputs: 8,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Prosperity Shard"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Netherrack"), 1)],
                    max_inputs: 64,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "transmutation",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "appliedenergistics2:interface_3",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 8,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Starmetal Ore"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("End Stone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sandstone"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "redstoneTransposer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_transposer_5",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 2)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Signalum Ingot"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Shibuichi Alloy"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Flux Crystal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Diamond"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fluxed Electrum Blend"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Electrum Blend"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Energy Fluxduct"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Redstone Energy Fluxduct (Empty)"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Flux Crystal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Diamond"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Wyvern Energy Core"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Draconic Core"), 1, vec![0])],
                    max_per_slot: 4,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterH",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_14",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("minecraft:slime_ball"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(congealed_green(), 1)],
                    max_inputs: 4,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Piston"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                        BufferedInput::new(Filter::Label("Copper Plate"), 1),
                        BufferedInput::new(Filter::Label("Treated Wood Planks"), 3),
                        BufferedInput::new(Filter::Label("Compressed Cobblestone"), 4),
                    ],
                    max_inputs: 72,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Glass Bottle"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Glass"), 3)],
                    max_inputs: 18,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Super Lubricent Tincture"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Bean"), 1),
                        BufferedInput::new(Filter::Label("Seeds"), 1),
                        BufferedInput::new(Filter::Label("Water Bottle"), 1),
                    ],
                    max_inputs: 12,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Super Lubricent Stone"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Super Lubricent Tincture"), 1),
                        BufferedInput::new(Filter::Label("Stone"), 8),
                    ],
                    max_inputs: 18,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fluix Seed"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Sand"), 1),
                        BufferedInput::new(Filter::Label("Fluix Dust"), 1),
                    ],
                    max_inputs: 16,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Certus Quartz Seed"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Sand"), 1),
                        BufferedInput::new(Filter::Label("Certus Quartz Dust"), 1),
                    ],
                    max_inputs: 16,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nether Quartz Seed"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Sand"), 1),
                        BufferedInput::new(Filter::Label("Nether Quartz Dust"), 1),
                    ],
                    max_inputs: 16,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "waterTransposer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_transposer_3",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Water Bottle"), n_wanted: 1 }],
                inputs: vec![SlottedInput::new(Filter::Label("Glass Bottle"), 1, vec![0])],
                max_per_slot: 1,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "inscriber",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_20",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Printed Silicon"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Silicon Ingot"), 1)],
                    max_inputs: 8,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Printed Engineering Circuit"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Diamond"), 1)],
                    max_inputs: 8,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Printed Logic Circuit"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Ingot"), 1)],
                    max_inputs: 8,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Printed Calculation Circuit"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pure Certus Quartz Crystal"), 1)],
                    max_inputs: 8,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Logic Processor"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Printed Silicon"), 1),
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                        BufferedInput::new(Filter::Label("Printed Logic Circuit"), 1),
                    ],
                    max_inputs: 24,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Engineering Processor"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Printed Silicon"), 1),
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                        BufferedInput::new(Filter::Label("Printed Engineering Circuit"), 1),
                    ],
                    max_inputs: 24,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Calculation Processor"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Printed Silicon"), 1),
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                        BufferedInput::new(Filter::Label("Printed Calculation Circuit"), 1),
                    ],
                    max_inputs: 24,
                },
            ],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_precipitator_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Snowball"), n_wanted: 64 }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_refinery_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Rosin"), n_wanted: 64 }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_crafter_3",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Fresh Water"), n_wanted: 64 }],
        });
        factory.add_process(BufferedConfig {
            name: "crafterI",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_30",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Invar Shears"), n_wanted: 1 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Invar Ingot"), 2)],
                    max_inputs: 2,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Quartz Sliver"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Quartz"), 1)],
                    max_inputs: 2,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Arcane Stone"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Instrumentum Vis Crystal"), 1),
                        BufferedInput::new(Filter::Label("Stone"), 8),
                    ],
                    max_inputs: 18,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Raw Tofeeg"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Firm Tofu"), 1),
                        BufferedInput::new(Filter::Label("Dandelion Yellow"), 1),
                    ],
                    max_inputs: 16,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cake"), n_wanted: 1 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Name("harvestcraft:flouritem"), 3),
                        BufferedInput::new(Filter::Label("Raw Tofeeg"), 1),
                        BufferedInput::new(Filter::Label("Soy Milk"), 3),
                        BufferedInput::new(Filter::Label("Sugar"), 2),
                    ],
                    max_inputs: 9,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Raw Tofurkey"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Cooking Oil"), 1),
                        BufferedInput::new(Filter::Label("Firm Tofu"), 1),
                        BufferedInput::new(Filter::Label("Bread"), 1),
                    ],
                    max_inputs: 12,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Salt"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Fresh Water"), 1)],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Butter"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Silken Tofu"), 1),
                        BufferedInput::new(Filter::Label("Salt"), 1),
                    ],
                    max_inputs: 32,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "thaumatorium",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_23",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![
                BufferedInput::new(Filter::Label("Charcoal"), 4),
                BufferedInput::new(Filter::Label("Invar Shears"), 1),
            ],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Alchemical Brass Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 1)],
                    max_inputs: 8,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Instrumentum Vis Crystal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Quartz Sliver"), 1)],
                    max_inputs: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "centrifuge",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_centrifuge_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cooking Oil"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Pumpkin"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silken Tofu"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Soybean"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sugar"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Sugar Canes"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Firm Tofu"), n_wanted: 16 },
                        Output { item: Filter::Label("Soy Milk"), n_wanted: 16 },
                    ],
                    inputs: vec![SlottedInput::new(Filter::Label("Silken Tofu"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulsating Propolis"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Mysterious Comb"), 1, vec![0])],
                    max_per_slot: 16,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterJ",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_18",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Buttered Potato"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Baked Potato"), 1),
                        BufferedInput::new(Filter::Label("Butter"), 1),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Marshmallows"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Sugar"), 1),
                        BufferedInput::new(Filter::Label("Raw Tofeeg"), 1),
                        BufferedInput::new(Filter::Label("Fresh Water"), 1),
                    ],
                    max_inputs: 48,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dough"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Name("harvestcraft:flouritem"), 1),
                        BufferedInput::new(Filter::Label("Fresh Water"), 1),
                        BufferedInput::new(Filter::Label("Salt"), 1),
                    ],
                    max_inputs: 48,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sweet Potato Pie"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Ground Cinnamon"), 1),
                        BufferedInput::new(Filter::Label("Sweet Potato"), 1),
                        BufferedInput::new(Filter::Label("Marshmallows"), 1),
                        BufferedInput::new(Filter::Label("Dough"), 1),
                    ],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cranberry Jelly"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Cranberry"), 1),
                        BufferedInput::new(Filter::Label("Sugar"), 1),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Mashed Potatoes"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Buttered Potato"), 1),
                        BufferedInput::new(Filter::Label("Salt"), 1),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Toast"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Butter"), 1),
                        BufferedInput::new(Filter::Label("Bread"), 1),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Thankful Dinner"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Sweet Potato Pie"), 1),
                        BufferedInput::new(Filter::Label("Cranberry Jelly"), 1),
                        BufferedInput::new(Filter::Label("Cooked Tofurkey"), 1),
                        BufferedInput::new(Filter::Label("Mashed Potatoes"), 1),
                        BufferedInput::new(Filter::Label("Toast"), 1),
                        BufferedInput::new(Filter::Label("Corn"), 1),
                        BufferedInput::new(Filter::Label("Bean"), 1),
                    ],
                    max_inputs: 112,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "conjuration",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:dropper_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Psigem"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Psigem"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Psidust"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Psidust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Psimetal Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Psimetal Ingot"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ebony Substance"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Ebony Substance"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ivory Substance"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Ivory Substance"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "manaPool",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:dropper_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Manasteel Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Mana Diamond"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Diamond"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Mana Powder"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sugar"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "elven",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "randomthings:irondropper_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 4,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Elementium Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Manasteel Ingot"), 2)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dragonstone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Mana Diamond"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterK",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_19",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Red Nether Brick"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Name("minecraft:netherbrick"), 2),
                        BufferedInput::new(Filter::Label("Nether Wart"), 2),
                    ],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Diorite"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Nether Quartz"), 2),
                        BufferedInput::new(Filter::Label("Cobblestone"), 2),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminium Sheetmetal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Aluminum Plate"), 4)],
                    max_inputs: 16,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Solenoid"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Copper Plate"), 4),
                        BufferedInput::new(Filter::Label("Aluminium Rod"), 2),
                        BufferedInput::new(Filter::Label("Copper Item Casing"), 2),
                        BufferedInput::new(Filter::Label("Mixed Metal Ingot"), 1),
                    ],
                    max_inputs: 36,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Quartz Slab"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Block of Quartz"), 3)],
                    max_inputs: 9,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Chiseled Quartz Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Quartz Slab"), 2)],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Andesite"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Cobblestone"), 1),
                        BufferedInput::new(Filter::Label("Diorite"), 1),
                    ],
                    max_inputs: 16,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basalt"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Cobblestone"), 2),
                        BufferedInput::new(Filter::Label("Coal"), 2),
                    ],
                    max_inputs: 16,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "combinationCrafting",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_30",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Restonia Crystal Block"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Restonia Crystal Block"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Ardite Tool Rod"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Redstone Reception Coil"), 1, vec![2]),
                        SlottedInput::new(Filter::Label("Rhodochrosite"), 1, vec![3]),
                        SlottedInput::new(Filter::Label("Red Nether Brick"), 1, vec![4]),
                    ],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Enori Crystal Block"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Enori Crystal Block"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Bone Block"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Osmium Ingot"), 1, vec![2]),
                        SlottedInput::new(Filter::Label("Block of Quartz"), 1, vec![3]),
                        SlottedInput::new(Filter::Label("Fluorite"), 1, vec![4]),
                    ],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Palis Crystal Block"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Palis Crystal Block"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Dense Lapis Lazuli Plate"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Cobalt Ingot"), 1, vec![2]),
                        SlottedInput::new(Filter::Label("Sapphire"), 1, vec![3]),
                        SlottedInput::new(congealed_blue(), 1, vec![4]),
                    ],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Diamatine Crystal Block"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Diamatine Crystal Block"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Zirconium Dust"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Mana Diamond"), 1, vec![2]),
                        SlottedInput::new(Filter::Label("Manyullyn Ingot"), 1, vec![3]),
                        SlottedInput::new(Filter::Label("Malachite"), 1, vec![4]),
                    ],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Emeradic Crystal Block"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Emeradic Crystal Block"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Emerald"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Cactus Green"), 1, vec![2]),
                        SlottedInput::new(Filter::Label("Beryllium Dust"), 1, vec![3]),
                        SlottedInput::new(Filter::Label("Ethetic Green Block"), 1, vec![4]),
                    ],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Void Crystal Block"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Void Crystal Block"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Ink Sac"), 1, vec![1]),
                        SlottedInput::new(
                            Filter::Both { label: "Block of Black Iron", name: "extendedcrafting:storage" },
                            1,
                            vec![2],
                        ),
                        SlottedInput::new(Filter::Label("Basalt"), 1, vec![3]),
                        SlottedInput::new(Filter::Label("Block of Black Quartz"), 1, vec![4]),
                    ],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Hardened Cell Frame"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Energy Cell Frame"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Invar Plate"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Invar Gear"), 1, vec![2]),
                        SlottedInput::new(Filter::Label("Steel Rod"), 1, vec![3]),
                        SlottedInput::new(Filter::Label("Steel Casing"), 1, vec![4]),
                    ],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ultimate Control Circuit"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Elite Control Circuit"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Atomic Alloy"), 4, vec![1]),
                    ],
                    max_per_slot: 4,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Extreme Crafting Table"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Double Compressed Crafting Table"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Crystal Matrix Ingot"), 4, vec![1]),
                        SlottedInput::new(Filter::Label("Crystaltine Catalyst"), 2, vec![2]),
                    ],
                    max_per_slot: 4,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Charged Draconium Block"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Draconium Block"), 2, vec![0])],
                    max_per_slot: 2,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Resonant Cell Frame (Full)"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Resonant Cell Frame (Empty)"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Moon Stone"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Dense Steel Plate"), 1, vec![2]),
                        SlottedInput::new(Filter::Label("Lapotron Crystal"), 1, vec![3]),
                        SlottedInput::new(Filter::Label("Ender Electron Tube"), 1, vec![4]),
                        SlottedInput::new(Filter::Label("Pulsating Mesh"), 1, vec![5]),
                        SlottedInput::new(Filter::Label("Reactor Frame"), 1, vec![6]),
                        SlottedInput::new(Filter::Label("Structure Frame Tier 1"), 1, vec![7]),
                        SlottedInput::new(Filter::Label("Infused Diamond"), 1, vec![8]),
                        SlottedInput::new(Filter::Label("Ultimate Control Circuit"), 1, vec![9]),
                        SlottedInput::new(Filter::Label("Enderium Gear"), 1, vec![10]),
                        SlottedInput::new(Filter::Label("Litherite Crystal"), 1, vec![11]),
                        SlottedInput::new(Filter::Label("Bioplastic"), 1, vec![12]),
                        SlottedInput::new(Filter::Label("Cubic Boron Nitride"), 1, vec![13]),
                        SlottedInput::new(Filter::Label("Genetics Processor"), 1, vec![14]),
                    ],
                    max_per_slot: 1,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "isotopeSeparator",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:isotope_separator_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot < 3)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Uranium-238"), n_wanted: 16 },
                        Output { item: Filter::Label("Tiny Clump of Uranium-235"), n_wanted: 16 },
                    ],
                    inputs: vec![SlottedInput::new(Filter::Label("Uranium Ingot"), 1, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Thorium-232"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Thorium Dust"), 1, vec![0])],
                    max_per_slot: 24,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "ncCrusher",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:rock_crusher_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| 1 <= slot && slot <= 3)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Crushed Fluorite"), n_wanted: 16 },
                        Output { item: Filter::Label("Zirconium Dust"), n_wanted: 16 },
                    ],
                    inputs: vec![SlottedInput::new(Filter::Label("Diorite"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Beryllium Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Andesite"), 1, vec![0])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterL",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_21",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("§aPrudentium Essence"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("§eInferium Essence"), 4)],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("§6Intermedium Essence"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("§aPrudentium Essence"), 4)],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("§bSuperium Essence"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("§6Intermedium Essence"), 4)],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("§cSupremium Essence"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("§bSuperium Essence"), 4)],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("§5Insanium Essence"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("§cSupremium Essence"), 4)],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ink Sac"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dye Essence"), 3)],
                    max_inputs: 12,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Knightslime Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Knightslime Essence"), 8)],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sandy Glass"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Sand"), 2),
                        BufferedInput::new(Filter::Label("Glass"), 2),
                    ],
                    max_inputs: 16,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterM",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_22",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Glowing Glass"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Glowstone Dust"), 1),
                        BufferedInput::new(Filter::Label("Thickened Glass"), 2),
                    ],
                    max_inputs: 24,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cryotheum Dust"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                        BufferedInput::new(Filter::Label("Snowball"), 1),
                        BufferedInput::new(Filter::Label("Blizz Powder"), 2),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pyrotheum Dust"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                        BufferedInput::new(Filter::Label("Sulfur"), 1),
                        BufferedInput::new(Filter::Label("Blaze Powder"), 2),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Hardened Fluxduct"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Leadstone Fluxduct"), 3),
                        BufferedInput::new(Filter::Label("Invar Ingot"), 1),
                        BufferedInput::new(Filter::Label("Redstone"), 3),
                    ],
                    max_inputs: 42,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Hardened Fluxduct"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Leadstone Fluxduct"), 3),
                        BufferedInput::new(Filter::Label("Invar Ingot"), 1),
                        BufferedInput::new(Filter::Label("Redstone"), 3),
                    ],
                    max_inputs: 42,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Energy Fluxduct (Empty)"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Hardened Fluxduct"), 2),
                        BufferedInput::new(Filter::Label("Electrum Plate"), 6),
                        BufferedInput::new(Filter::Label("Fused Quartz"), 1),
                    ],
                    max_inputs: 9,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Signalum Fluxduct"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Redstone Energy Fluxduct"), 3),
                        BufferedInput::new(Filter::Label("Signalum Ingot"), 1),
                        BufferedInput::new(Filter::Label("Redstone"), 3),
                    ],
                    max_inputs: 42,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Resonant Fluxduct"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Signalum Fluxduct"), 3),
                        BufferedInput::new(Filter::Label("Enderium Ingot"), 1),
                        BufferedInput::new(Filter::Label("Redstone"), 3),
                    ],
                    max_inputs: 42,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cryo-Stabilized Fluxduct (Empty)"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Resonant Fluxduct"), 1),
                        BufferedInput::new(Filter::Label("Electrum Ingot"), 4),
                        BufferedInput::new(Filter::Label("Fused Quartz"), 4),
                    ],
                    max_inputs: 144,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "cryoTransposer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_transposer_10",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 2)),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Cryo-Stabilized Fluxduct"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Cryo-Stabilized Fluxduct (Empty)"), 1, vec![0])],
                max_per_slot: 8,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "crafterN",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_23",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Tubing"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Aluminium Wire"), 4),
                        BufferedInput::new(Filter::Label("Basic Coil"), 3),
                    ],
                    max_inputs: 28,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Black Iron Slate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Black Iron Ingot"), 2)],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Luminessence"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Glowstone Dust"), 2),
                        BufferedInput::new(Filter::Label("Gunpowder"), 1),
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basic Component"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iron Ingot"), 2),
                        BufferedInput::new(Filter::Label("Luminessence"), 1),
                        BufferedInput::new(Filter::Label("Black Iron Slate"), 1),
                    ],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Advanced Component"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Gold Ingot"), 2),
                        BufferedInput::new(Filter::Label("Luminessence"), 1),
                        BufferedInput::new(Filter::Label("Black Iron Slate"), 1),
                    ],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Elite Component"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Diamond"), 2),
                        BufferedInput::new(Filter::Label("Luminessence"), 1),
                        BufferedInput::new(Filter::Label("Black Iron Slate"), 1),
                    ],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ultimate Component"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Emerald"), 2),
                        BufferedInput::new(Filter::Label("Luminessence"), 1),
                        BufferedInput::new(Filter::Label("Black Iron Slate"), 1),
                    ],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Petrotheum Dust"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Pulverized Obsidian"), 1),
                        BufferedInput::new(Filter::Label("Basalz Powder"), 2),
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                    ],
                    max_inputs: 32,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "rosin",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_tapper_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![BufferedInput::new(Filter::Label("Fluxed Phyto-Gro"), 64)],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "crafterO",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_24",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Blast Brick"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Name("minecraft:netherbrick"), 4),
                        BufferedInput::new(Filter::Label("Blaze Powder"), 1),
                        BufferedInput::new(Filter::Label("Brick"), 4),
                    ],
                    max_inputs: 54,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Mechanical Component"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Steel Plate"), 4),
                        BufferedInput::new(Filter::Label("Copper Ingot"), 1),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Reinforced Blast Brick"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Steel Plate"), 1),
                        BufferedInput::new(Filter::Label("Blast Brick"), 1),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Casing"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Osmium Ingot"), 4),
                        BufferedInput::new(Filter::Label("Osmium Block"), 2),
                        BufferedInput::new(Filter::Label("Reinforced Blast Brick"), 1),
                        BufferedInput::new(Filter::Label("Steel Mechanical Component"), 2),
                    ],
                    max_inputs: 72,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basic Catalyst"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Black Iron Ingot"), 1),
                        BufferedInput::new(Filter::Label("Basic Component"), 4),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Advanced Catalyst"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Black Iron Ingot"), 1),
                        BufferedInput::new(Filter::Label("Advanced Component"), 4),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Elite Catalyst"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Black Iron Ingot"), 1),
                        BufferedInput::new(Filter::Label("Elite Component"), 4),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ultimate Catalyst"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Black Iron Ingot"), 1),
                        BufferedInput::new(Filter::Label("Ultimate Component"), 4),
                    ],
                    max_inputs: 80,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterP",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_28",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Reinforced Cell Frame (Empty)"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Redstone Conductance Coil"), 1),
                        BufferedInput::new(Filter::Label("Fluxed Electrum Plate"), 2),
                        BufferedInput::new(Filter::Label("Electrum Large Plate"), 1),
                        BufferedInput::new(Filter::Label("Hardened Cell Frame"), 1),
                        BufferedInput::new(Filter::Label("Flux Crystal"), 2),
                        BufferedInput::new(Filter::Label("Silver Gear"), 2),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Signalum Cell Frame (Empty)"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Reinforced Cell Frame (Full)"), 1),
                        BufferedInput::new(Filter::Label("Signalum Plate"), 4),
                        BufferedInput::new(Filter::Label("Rich Slag"), 1),
                        BufferedInput::new(Filter::Label("Cinnabar"), 1),
                        BufferedInput::new(Filter::Label("Rosin"), 2),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basic Crafting Table"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Black Iron Slate"), 1),
                        BufferedInput::new(Filter::Label("Basic Component"), 4),
                        BufferedInput::new(Filter::Label("Basic Catalyst"), 1),
                        BufferedInput::new(Filter::Label("Block of Iron"), 1),
                        BufferedInput::new(Filter::Label("Crafting Table"), 2),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aerotheum Dust"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Niter"), 1),
                        BufferedInput::new(Filter::Label("Blitz Powder"), 2),
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Mana Dust"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Mana Diamond"), 4),
                        BufferedInput::new(Filter::Label("Petrotheum Dust"), 1),
                        BufferedInput::new(Filter::Label("Aerotheum Dust"), 1),
                        BufferedInput::new(Filter::Label("Cryotheum Dust"), 1),
                        BufferedInput::new(Filter::Label("Pyrotheum Dust"), 1),
                        BufferedInput::new(Filter::Label("Mana Powder"), 1),
                    ],
                    max_inputs: 72,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fiery Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Fiery Ingot Essence"), 8)],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("LV Capacitor"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Treated Wood Planks"), 2),
                        BufferedInput::new(Filter::Label("Copper Ingot"), 2),
                        BufferedInput::new(Filter::Label("Iron Ingot"), 3),
                        BufferedInput::new(Filter::Label("Lead Ingot"), 1),
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("MV Capacitor"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Treated Wood Planks"), 2),
                        BufferedInput::new(Filter::Label("Block of Redstone"), 1),
                        BufferedInput::new(Filter::Label("Electrum Ingot"), 2),
                        BufferedInput::new(Filter::Label("LV Capacitor"), 1),
                        BufferedInput::new(Filter::Label("Iron Ingot"), 3),
                    ],
                    max_inputs: 144,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "mobCrusher",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:mob_relocator_tile_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![],
            to_extract: Some(Box::new(|slot, _| slot >= 6)),
            recipes: vec![],
        });
        factory.add_process(RedstoneEmitterConfig {
            accesses: vec![RedstoneAccess { client: "1a", addr: Some("redstone_integrator_3"), side: SOUTH }],
            output: Box::new(|factory| {
                if factory.search_n_stored(&Filter::Label("Bone")) < 16
                    || factory.search_n_stored(&Filter::Label("Wither Skeleton Skull")) < 16
                {
                    factory.log(super::action::Log { text: "witherSkeleton: on".to_owned(), color: 10 });
                    15
                } else {
                    0
                }
            }),
        });
        factory.add_process(RedstoneEmitterConfig {
            accesses: vec![RedstoneAccess { client: "1a", addr: Some("redstone_integrator_3"), side: NORTH }],
            output: emit_when_want_item("shulker", Filter::Label("Shulker Shell"), 16),
        });
        factory.add_process(BufferedConfig {
            name: "trash",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "xu2:tiletrashchest_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![BufferedRecipe {
                outputs: vec![],
                inputs: vec![BufferedInput::new(Filter::Label("Stone Sword"), 1)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "crafterQ",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_26",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("HV Capacitor"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Treated Wood Planks"), 2),
                        BufferedInput::new(Filter::Label("Block of Redstone"), 1),
                        BufferedInput::new(Filter::Label("Block of Lead"), 2),
                        BufferedInput::new(Filter::Label("MV Capacitor"), 1),
                        BufferedInput::new(Filter::Label("Steel Ingot"), 3),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basic Capacitor"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Redstone Transmission Coil"), 4),
                        BufferedInput::new(Filter::Label("Grains of Infinity"), 2),
                        BufferedInput::new(Filter::Label("HV Capacitor"), 1),
                    ],
                    max_inputs: 56,
                },
                BufferedRecipe {
                    outputs: vec![Output {
                        item: Filter::Both { label: "Advanced Circuit", name: "ic2:crafting" },
                        n_wanted: 16,
                    }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Redstone"), 4),
                        BufferedInput::new(Filter::Label("Lapis Lazuli"), 2),
                        BufferedInput::new(Filter::Label("Glowstone Dust"), 2),
                        BufferedInput::new(Filter::Label("Basic Control Circuit"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Elite Control Circuit"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Redstone"), 4),
                        BufferedInput::new(Filter::Label("Reinforced Alloy"), 4),
                        BufferedInput::new(Filter::Both { label: "Advanced Circuit", name: "ic2:crafting" }, 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Thaumium Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Thaumium Essence"), 8)],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Void Metal Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Void Metal Essence"), 8)],
                    max_inputs: 48,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Base Essence Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Prosperity Shard"), 4),
                        BufferedInput::new(Filter::Label("Iron Ingot"), 1),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("§eInferium Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Base Essence Ingot"), 1),
                        BufferedInput::new(Filter::Label("§eInferium Essence"), 4),
                    ],
                    max_inputs: 80,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "metalFab",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:tileiteminputbus_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Mirion Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Glass"), 10),
                        BufferedInput::new(Filter::Label("Manasteel Ingot"), 1),
                        BufferedInput::new(Filter::Label("Elementium Ingot"), 1),
                        BufferedInput::new(Filter::Label("Terrasteel Ingot"), 1),
                    ],
                    max_inputs: 104,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Osmiridium Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iridium Ingot"), 1),
                        BufferedInput::new(Filter::Label("Osmium Ingot"), 1),
                    ],
                    max_inputs: 16,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Osgloglas Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Refined Obsidian Ingot"), 1),
                        BufferedInput::new(Filter::Label("Glowstone Ingot"), 1),
                        BufferedInput::new(Filter::Label("Osmium Ingot"), 1),
                    ],
                    max_inputs: 24,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Titanium Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Magnesium Ore"), 2),
                        BufferedInput::new(Filter::Label("Carbon Plate"), 1),
                        BufferedInput::new(Filter::Label("Salt"), 4),
                    ],
                    max_inputs: 56,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Mana Infused Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Manasteel Ingot"), 4),
                        BufferedInput::new(Filter::Label("Diamond"), 1),
                    ],
                    max_inputs: 40,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Modularium Alloy"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Empowered Palis Crystal"), 1),
                        BufferedInput::new(Filter::Label("Electrical Steel Ingot"), 2),
                        BufferedInput::new(Filter::Label("Platinum Ingot"), 1),
                    ],
                    max_inputs: 32,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "advancedCraftingTable",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_27",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Advanced Crafting Table"), n_wanted: 16 }],
                inputs: vec![
                    BufferedInput::new(Filter::Label("Basic Crafting Table"), 1),
                    BufferedInput::new(Filter::Label("Basic Component"), 1),
                    BufferedInput::new(Filter::Label("Advanced Component"), 1),
                    BufferedInput::new(Filter::Label("Elite Component"), 1),
                    BufferedInput::new(Filter::Label("Ultimate Component"), 1),
                    BufferedInput::new(Filter::Label("Basic Catalyst"), 1),
                    BufferedInput::new(Filter::Label("Advanced Catalyst"), 1),
                    BufferedInput::new(Filter::Label("Elite Catalyst"), 1),
                    BufferedInput::new(Filter::Label("Ultimate Catalyst"), 1),
                ],
                max_inputs: 9,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "eliteCraftingTable",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_28",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Elite Crafting Table"), n_wanted: 16 }],
                inputs: vec![
                    BufferedInput::new(Filter::Label("Advanced Crafting Table"), 2),
                    BufferedInput::new(Filter::Label("Elite Component"), 8),
                    BufferedInput::new(Filter::Label("Mana Dust"), 4),
                    BufferedInput::new(Filter::Label("Osgloglas Block"), 2),
                    BufferedInput::new(Filter::Label("Crafter Tier 3"), 1),
                    BufferedInput::new(Filter::Label("Fluxed Phyto-Gro"), 2),
                    BufferedInput::new(Filter::Label("Hardened Cell Frame"), 4),
                    BufferedInput::new(Filter::Label("Signalum Cell Frame (Full)"), 2),
                ],
                max_inputs: 25,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "ultimateCraftingTable",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_33",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Ultimate Crafting Table"), n_wanted: 16 }],
                inputs: vec![
                    BufferedInput::new(Filter::Label("Signalum Cell Frame (Full)"), 4),
                    BufferedInput::new(crystaltine_trimmed(), 8),
                    BufferedInput::new(Filter::Label("Draconium Crystal"), 8),
                    BufferedInput::new(Filter::Both { label: "Advanced Circuit", name: "advancedrocketry:ic" }, 4),
                    BufferedInput::new(Filter::Label("Double Compressed Crafting Table"), 8),
                    BufferedInput::new(Filter::Label("Advanced Crafting Table"), 4),
                    BufferedInput::new(Filter::Label("Resonant Cell Frame (Full)"), 2),
                    BufferedInput::new(Filter::Label("Elite Crafting Table"), 2),
                    BufferedInput::new(Filter::Label("Wyvern Core"), 2),
                    BufferedInput::new(Filter::Label("Iridium Neutron Reflector"), 2),
                    BufferedInput::new(Filter::Label("Extreme Crafting Table"), 1),
                    BufferedInput::new(Filter::Label("Aethium"), 1),
                    BufferedInput::new(Filter::Label("Block of Enderium"), 1),
                    BufferedInput::new(Filter::Label("Ludicrite Block"), 1),
                    BufferedInput::new(Filter::Label("Mirion Block"), 1),
                ],
                max_inputs: 49,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "crafterR",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_29",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("§aPrudentium Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("§eInferium Ingot"), 1),
                        BufferedInput::new(Filter::Label("§aPrudentium Essence"), 4),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("§6Intermedium Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("§aPrudentium Ingot"), 1),
                        BufferedInput::new(Filter::Label("§6Intermedium Essence"), 4),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("§bSuperium Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("§6Intermedium Ingot"), 1),
                        BufferedInput::new(Filter::Label("§bSuperium Essence"), 4),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("§cSupremium Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("§bSuperium Ingot"), 1),
                        BufferedInput::new(Filter::Label("§cSupremium Essence"), 4),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("§5Insanium Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("§cSupremium Ingot"), 1),
                        BufferedInput::new(Filter::Label("§5Insanium Essence"), 4),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nether Star Nugget"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Star"), 1)],
                    max_inputs: 2,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nether Star"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Star Shard"), 3)],
                    max_inputs: 48,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iridium Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iridium Essence"), 8)],
                    max_inputs: 128,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crystaltine",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_29",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Crystaltine Ingot"), n_wanted: 16 }],
                inputs: vec![
                    BufferedInput::new(Filter::Label("Nether Star Nugget"), 4),
                    BufferedInput::new(Filter::Label("Lapis Lazuli"), 10),
                    BufferedInput::new(Filter::Label("Iron Ingot"), 4),
                    BufferedInput::new(Filter::Label("Gold Ingot"), 2),
                    BufferedInput::new(Filter::Label("Diamond"), 8),
                ],
                max_inputs: 28,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "cloche",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "immersiveengineering:belljar_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Fluxed Phyto-Gro"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "crafterS",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_31",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ender Casing"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Ender Pearl"), 4),
                        BufferedInput::new(Filter::Label("Block of Black Quartz"), 1),
                        BufferedInput::new(Filter::Label("Empowered Diamatine Crystal"), 4),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Enriched Uranium Nuclear Fuel"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Uranium Ingot"), 6),
                        BufferedInput::new(Filter::Label("Tiny Clump of Uranium-235"), 3),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Advanced Plating"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Redstone"), 4),
                        BufferedInput::new(Filter::Label("Tough Alloy"), 4),
                        BufferedInput::new(Filter::Label("Basic Plating"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("LV Wire Coil"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Copper Wire"), 4),
                        BufferedInput::new(Filter::Label("Stick"), 1),
                    ],
                    max_inputs: 20,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Coil Block"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("LV Wire Coil"), 8),
                        BufferedInput::new(Filter::Label("Iron Ingot"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Insulating Glass"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Pulverized Iron"), 2),
                        BufferedInput::new(Filter::Label("Cactus Green"), 1),
                        BufferedInput::new(Filter::Label("Glass"), 2),
                    ],
                    max_inputs: 40,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Terrasteel Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Terrasteel Essence"), 8)],
                    max_inputs: 128,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("DU Plating"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Advanced Plating"), 1),
                        BufferedInput::new(Filter::Label("Uranium-238"), 4),
                        BufferedInput::new(Filter::Label("Sulfur"), 4),
                    ],
                    max_inputs: 144,
                },
            ],
        });
        factory.add_process(RedstoneEmitterConfig {
            accesses: vec![RedstoneAccess { client: "1a", addr: Some("redstone_integrator_1"), side: NORTH }],
            output: emit_when_want_item("fissionIC2", Filter::Label("Tiny Pile of Plutonium"), 16),
        });
        factory.add_process(RedstoneEmitterConfig {
            accesses: vec![RedstoneAccess { client: "1a", addr: Some("redstone_integrator_5"), side: WEST }],
            output: emit_when_want_item("fissionBR", Filter::Label("Cyanite Ingot"), 16),
        });
        factory.add_process(RedstoneEmitterConfig {
            accesses: vec![RedstoneAccess { client: "1a", addr: Some("redstone_integrator_8"), side: SOUTH }],
            output: emit_when_want_item("fissionTBU", Filter::Label("Uranium-233"), 16),
        });
        factory.add_process(RedstoneEmitterConfig {
            accesses: vec![RedstoneAccess { client: "1a", addr: Some("redstone_integrator_7"), side: SOUTH }],
            output: Box::new(move |factory| {
                if factory.search_n_stored(&Filter::Label("Plutonium-241")) < 16
                    || factory.search_n_stored(&Filter::Label("Plutonium-242")) < 16
                {
                    factory.log(super::action::Log { text: "fissionLEU: on".to_owned(), color: 10 });
                    15
                } else {
                    0
                }
            }),
        });
        factory.add_process(RedstoneEmitterConfig {
            accesses: vec![RedstoneAccess { client: "1a", addr: Some("redstone_integrator_8"), side: NORTH }],
            output: Box::new(move |factory| {
                if factory.search_n_stored(&Filter::Label("Curium-246")) < 16
                    || factory.search_n_stored(&Filter::Label("Curium-247")) < 16
                {
                    factory.log(super::action::Log { text: "fissionHEP: on".to_owned(), color: 10 });
                    15
                } else {
                    0
                }
            }),
        });
        factory.add_process(RedstoneEmitterConfig {
            accesses: vec![RedstoneAccess { client: "1a", addr: Some("redstone_integrator_9"), side: EAST }],
            output: Box::new(move |factory| {
                if factory.search_n_stored(&Filter::Label("Californium-249")) < 16
                    || factory.search_n_stored(&Filter::Label("Californium-252")) < 16
                {
                    factory.log(super::action::Log { text: "fissionHECm: on".to_owned(), color: 10 });
                    15
                } else {
                    0
                }
            }),
        });
        factory.add_process(RedstoneEmitterConfig {
            accesses: vec![RedstoneAccess { client: "1a", addr: Some("redstone_integrator_9"), side: WEST }],
            output: emit_when_want_item("fissionLECf", Filter::Label("Californium-250"), 16),
        });
        factory.add_process(SlottedConfig {
            name: "fissionIC2",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:fission_controller_new_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Enriched Uranium Nuclear Fuel"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "fissionBR",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:fission_controller_new_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Uranium Ingot"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "fissionTBU",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:fission_controller_new_5",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("TBU Fuel"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "fissionLEU",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:fission_controller_new_4",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("LEU-233 Fuel"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "fissionHEP",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:fission_controller_new_6",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("HEP-241 Fuel"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "fissionHECm",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:fission_controller_new_7",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("HECm-247 Fuel"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "fissionLECf",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:fission_controller_new_8",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("LECf-249 Fuel"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "fuelReprocessor",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:fuel_reprocessor_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: extract_all(),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![],
                    inputs: vec![SlottedInput::new(Filter::Label("Depleted Uranium Nuclear Fuel"), 1, vec![0])],
                    max_per_slot: i32::MAX,
                },
                SlottedRecipe {
                    outputs: vec![],
                    inputs: vec![SlottedInput::new(Filter::Label("Depleted LECf-249 Fuel"), 1, vec![0])],
                    max_per_slot: i32::MAX,
                },
                SlottedRecipe {
                    outputs: vec![],
                    inputs: vec![SlottedInput::new(Filter::Label("Depleted HECm-247 Fuel"), 1, vec![0])],
                    max_per_slot: i32::MAX,
                },
                SlottedRecipe {
                    outputs: vec![],
                    inputs: vec![SlottedInput::new(Filter::Label("Depleted HEP-241 Fuel"), 1, vec![0])],
                    max_per_slot: i32::MAX,
                },
                SlottedRecipe {
                    outputs: vec![],
                    inputs: vec![SlottedInput::new(Filter::Label("Depleted LEU-233 Fuel"), 1, vec![0])],
                    max_per_slot: i32::MAX,
                },
                SlottedRecipe {
                    outputs: vec![],
                    inputs: vec![SlottedInput::new(Filter::Label("Depleted TBU Fuel"), 1, vec![0])],
                    max_per_slot: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "uraniumMelter",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_13",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![BufferedRecipe {
                outputs: vec![],
                inputs: vec![BufferedInput::new(Filter::Label("Uranium 238"), 1)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "waterCarpenter",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:tileiteminputbus_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Hardened Casing"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Sturdy Casing"), 1),
                        BufferedInput::new(Filter::Label("Diamond"), 4),
                    ],
                    max_inputs: 40,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Refined Circuit Board"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iron Ingot"), 1),
                        BufferedInput::new(Filter::Label("Redstone"), 2),
                    ],
                    max_inputs: 24,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Enhanced Circuit Board"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Bronze Ingot"), 1),
                        BufferedInput::new(Filter::Label("Redstone"), 2),
                    ],
                    max_inputs: 24,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basic Circuit Board"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Tin Ingot"), 1),
                        BufferedInput::new(Filter::Label("Redstone"), 2),
                    ],
                    max_inputs: 24,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Intricate Circuit Board"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Basic Circuit Board"), 1),
                        BufferedInput::new(Filter::Label("Enhanced Circuit Board"), 1),
                        BufferedInput::new(Filter::Label("Refined Circuit Board"), 1),
                        BufferedInput::new(Filter::Label("Printed Engineering Circuit"), 2),
                        BufferedInput::new(Filter::Label("Ultimate Control Circuit"), 1),
                    ],
                    max_inputs: 48,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "vacuumTube",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:itemviewer_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Vacuum Tube"), n_wanted: 16 }],
                inputs: vec![
                    BufferedInput::new(Filter::Label("Glass"), 1),
                    BufferedInput::new(Filter::Label("Nickel Plate"), 1),
                    BufferedInput::new(Filter::Label("Copper Wire"), 1),
                    BufferedInput::new(Filter::Label("Redstone"), 1),
                ],
                max_inputs: 24,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "circuitBoard",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:itemviewer_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Circuit Board"), n_wanted: 16 }],
                inputs: vec![
                    BufferedInput::new(Filter::Label("Insulating Glass"), 1),
                    BufferedInput::new(Filter::Label("Copper Plate"), 1),
                    BufferedInput::new(Filter::Label("Vacuum Tube"), 2),
                ],
                max_inputs: 24,
            }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:crystallizer_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Calcium Sulfate"), n_wanted: 64 }],
        });
        factory.add_process(BufferedConfig {
            name: "crafterT",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_32",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crystal Binder"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Crushed Rhodochrosite"), 1),
                        BufferedInput::new(Filter::Label("Pulverized Obsidian"), 1),
                        BufferedInput::new(Filter::Label("Calcium Sulfate"), 1),
                        BufferedInput::new(Filter::Label("Magnesium Dust"), 1),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Elite Plating"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("DU Plating"), 1),
                        BufferedInput::new(Filter::Label("Boron Ingot"), 4),
                        BufferedInput::new(Filter::Label("Crystal Binder"), 4),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Terrestrial Artifact"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Empowered Restonia Crystal"), 4),
                        BufferedInput::new(Filter::Label("Empowered Diamatine Crystal"), 1),
                        BufferedInput::new(Filter::Label("Empowered Emeradic Crystal"), 1),
                        BufferedInput::new(Filter::Label("Empowered Enori Crystal"), 1),
                        BufferedInput::new(Filter::Label("Empowered Palis Crystal"), 1),
                        BufferedInput::new(Filter::Label("Empowered Void Crystal"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ender Amethyst"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Elementium Ingot"), 8),
                        BufferedInput::new(Filter::Label("Terrestrial Artifact"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crystaltine Component"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Black Iron Slate"), 1),
                        BufferedInput::new(Filter::Label("Luminessence"), 1),
                        BufferedInput::new(Filter::Label("Crystaltine Ingot"), 2),
                    ],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crystaltine Catalyst"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Black Iron Ingot"), 1),
                        BufferedInput::new(Filter::Label("Crystaltine Component"), 4),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: crystaltine_trimmed(), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Crystaltine Nugget"), 4),
                        BufferedInput::new(
                            Filter::Both { label: "Block of Black Iron", name: "extendedcrafting:storage" },
                            1,
                        ),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Blutonium Block"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Empowered Palis Crystal Block"), 1),
                        BufferedInput::new(Filter::Label("Block of Mana Infused Metal"), 2),
                        BufferedInput::new(Filter::Label("Tiny Pile of Plutonium"), 3),
                        BufferedInput::new(Filter::Label("Block of Cobalt"), 1),
                        BufferedInput::new(Filter::Label("Cyanite Block"), 2),
                    ],
                    max_inputs: 144,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "cuttingMachine",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:vulpesinputhatch_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silicon Wafer"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Silicon Boule"), 1)],
                    max_inputs: 8,
                },
                BufferedRecipe {
                    outputs: vec![Output {
                        item: Filter::Both { label: "Advanced Circuit", name: "advancedrocketry:ic" },
                        n_wanted: 16,
                    }],
                    inputs: vec![BufferedInput::new(Filter::Label("Advanced Circuit Plate"), 1)],
                    max_inputs: 8,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterU",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_33",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Black Concrete Powder"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Ink Sac"), 1),
                        BufferedInput::new(Filter::Label("Gravel"), 4),
                        BufferedInput::new(Filter::Label("Sand"), 4),
                    ],
                    max_inputs: 18,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Connector"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Aluminum Brass Ingot"), 1),
                        BufferedInput::new(Filter::Label("Signalum Ingot"), 4),
                        BufferedInput::new(Filter::Label("Block of Tin"), 4),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Interconnect"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Black Concrete"), 1),
                        BufferedInput::new(Filter::Label("Connector"), 4),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Structure Frame Tier 1"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Interconnect"), 1),
                        BufferedInput::new(Filter::Label("Litherite Crystal"), 2),
                        BufferedInput::new(Filter::Label("Lapis Lazuli"), 1),
                        BufferedInput::new(Filter::Label("Iron Ingot"), 1),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Genetics Processor"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Both { label: "Advanced Circuit", name: "ic2:crafting" }, 2),
                        BufferedInput::new(Filter::Label("Printed Engineering Circuit"), 4),
                        BufferedInput::new(Filter::Label("Pure Nether Quartz Crystal"), 2),
                        BufferedInput::new(Filter::Label("Ender Pearl"), 1),
                    ],
                    max_inputs: 72,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Reactor Frame"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Steel Casing"), 4),
                        BufferedInput::new(Filter::Label("Atomic Alloy"), 1),
                    ],
                    max_inputs: 20,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Stable-'Unstable Nugget'"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iron Nugget"), 1),
                        BufferedInput::new(Filter::Label("Stick"), 1),
                        BufferedInput::new(Filter::Label("Diamond"), 1),
                    ],
                    max_inputs: 48,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Moon Stone"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Lunar Reactive Dust"), 8),
                        BufferedInput::new(Filter::Label("Stable-'Unstable Ingot'"), 1),
                    ],
                    max_inputs: 144,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "alumite",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_31",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Alumite Ingot"), n_wanted: 16 }],
                inputs: vec![
                    BufferedInput::new(Filter::Label("Pulverized Aluminum"), 5),
                    BufferedInput::new(Filter::Label("Pulverized Iron"), 2),
                    BufferedInput::new(Filter::Label("Obsidian"), 2),
                ],
                max_inputs: 9,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "resonator",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "xu2:tileresonator_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: extract_all(),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Lunar Reactive Dust"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Lapis Lazuli"), 1, vec![0])],
                max_per_slot: 4,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "crafterV",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_34",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Diamond Lattice"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Diamond"), 5)],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crystal Matrix Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Diamond Lattice"), 4),
                        BufferedInput::new(Filter::Label("Nether Star"), 2),
                    ],
                    max_inputs: 96,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Alloy Grinding Ball"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Redstone Alloy Ingot"), 5)],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Energium Dust"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Diamond Dust"), 4),
                        BufferedInput::new(Filter::Label("Redstone"), 5),
                    ],
                    max_inputs: 18,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("LEU-233 Fuel"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Uranium-238"), 8),
                        BufferedInput::new(Filter::Label("Uranium-233"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lapotron Crystal"), n_wanted: 4 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Lapis Lazuli Dust"), 6),
                        BufferedInput::new(Filter::Both { label: "Advanced Circuit", name: "ic2:crafting" }, 2),
                        BufferedInput::new(Filter::Label("Energy Crystal"), 1),
                    ],
                    max_inputs: 36,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Thick Neutron Reflector"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Neutron Reflector"), 4),
                        BufferedInput::new(Filter::Label("Copper Plate"), 5),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Neutron Reflector"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Pulverized Coal"), 4),
                        BufferedInput::new(Filter::Label("Pulverized Tin"), 4),
                        BufferedInput::new(Filter::Label("Copper Plate"), 1),
                    ],
                    max_inputs: 144,
                },
            ],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:crystallizer_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Hexagonal Boron Nitride"), n_wanted: 64 }],
        });
        factory.add_process(BufferedConfig {
            name: "dissolution",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_14",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![BufferedInput::new(Filter::Label("Draconium Ore"), 64)],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "xu2:tileminchest_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Draconium Crystal"), n_wanted: 64 }],
        });
        factory.add_process(BufferedConfig {
            name: "crystallizer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:vulpesinputhatch_3",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Silicon Boule"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Silicon Ingot"), 1)],
                max_inputs: 4,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "assembler",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:vulpesinputhatch_4",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Advanced Circuit Plate"), n_wanted: 16 }],
                inputs: vec![
                    BufferedInput::new(Filter::Label("Intricate Circuit Board"), 1),
                    BufferedInput::new(Filter::Label("Redstone Alloy Grinding Ball"), 1),
                    BufferedInput::new(Filter::Label("Silicon Wafer"), 1),
                ],
                max_inputs: 12,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "crafterW",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_35",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iridium Reinforced Plate"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Advanced Alloy"), 4),
                        BufferedInput::new(Filter::Label("Iridium Ingot"), 4),
                        BufferedInput::new(Filter::Label("Diamond"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iridium Neutron Reflector"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Thick Neutron Reflector"), 6),
                        BufferedInput::new(Filter::Label("Dense Copper Plate"), 2),
                        BufferedInput::new(Filter::Label("Iridium Reinforced Plate"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Eye of Ender"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Ender Pearl"), 1),
                        BufferedInput::new(Filter::Label("Blaze Powder"), 1),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empty Cooler"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Tough Alloy"), 4),
                        BufferedInput::new(Filter::Label("Steel Ingot"), 4),
                    ],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cryotheum Cooler"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Cryotheum Dust"), 8),
                        BufferedInput::new(Filter::Label("Empty Cooler"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Reactor Cell"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Tough Alloy"), 4),
                        BufferedInput::new(Filter::Label("Glass"), 4),
                    ],
                    max_inputs: 128,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("HEP-241 Fuel"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Plutonium-241"), 4),
                        BufferedInput::new(Filter::Label("Plutonium-242"), 5),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Infused Diamond"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Diamond"), 1),
                        BufferedInput::new(Filter::Label("Dimensional Shard"), 8),
                    ],
                    max_inputs: 144,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterX",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_36",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Transparent Fission Reactor Casing"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Glass"), 4),
                        BufferedInput::new(Filter::Label("Tough Alloy"), 1),
                        BufferedInput::new(Filter::Label("Basic Plating"), 4),
                    ],
                    max_inputs: 36,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dislocator"), n_wanted: 4 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Eye of Ender"), 1),
                        BufferedInput::new(Filter::Label("Blaze Powder"), 4),
                        BufferedInput::new(Filter::Label("Draconium Dust"), 4),
                    ],
                    max_inputs: 36,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Draconic Core"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Dislocator"), 1),
                        BufferedInput::new(Filter::Label("Blutonium Ingot"), 1),
                        BufferedInput::new(Filter::Label("Genetics Processor"), 1),
                        BufferedInput::new(Filter::Label("Litherite Crystal"), 2),
                        BufferedInput::new(Filter::Label("Draconium Block"), 2),
                        BufferedInput::new(Filter::Label("Elite Plating"), 2),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Uranium Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Uranium Essence"), 8)],
                    max_inputs: 48,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Netherrack"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Essence"), 5)],
                    max_inputs: 5,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("HECm-247 Fuel"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Curium-247"), 4),
                        BufferedInput::new(Filter::Label("Curium-246"), 5),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("LECf-249 Fuel"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Californium-249"), 1),
                        BufferedInput::new(Filter::Label("Californium-252"), 8),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Californium RTG"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Advanced Plating"), 4),
                        BufferedInput::new(Filter::Label("Californium-250"), 1),
                        BufferedInput::new(Filter::Label("Graphite Ingot"), 4),
                    ],
                    max_inputs: 144,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "neutronium",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:infuser_3",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Titanium Iridium Alloy Ingot"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "alloySmelter",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "enderio:tile_alloy_smelter_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0, 1, 2],
            to_extract: extract_all(),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Organic Green Dye"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Cactus Green"), 2, vec![0]),
                        SlottedInput::new(Filter::Label("Slimeball"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Pulverized Charcoal"), 2, vec![2]),
                    ],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Organic Black Dye"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Slimeball"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Pulverized Charcoal"), 6, vec![1]),
                    ],
                    max_per_slot: 60,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Industrial Machine Chassis"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Simple Machine Chassis"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Industrial Dye Blend"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Energetic Alloy Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Redstone"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Gold Ingot"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Glowstone Dust"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Vibrant Alloy Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Energetic Alloy Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Ender Pearl"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterY",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_37",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Industrial Dye Blend"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Lapis Lazuli Dust"), 2),
                        BufferedInput::new(Filter::Label("Nether Quartz Dust"), 4),
                        BufferedInput::new(Filter::Label("Organic Green Dye"), 2),
                        BufferedInput::new(Filter::Label("Organic Black Dye"), 1),
                    ],
                    max_inputs: 27,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Infinity Bimetal Gear"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iron Ingot"), 4),
                        BufferedInput::new(Filter::Label("Iron Nugget"), 4),
                        BufferedInput::new(Filter::Label("Grains of Infinity"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dark Iron Bars"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dark Steel Ingot"), 6)],
                    max_inputs: 6,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Simple Machine Chassis"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Titanium Aluminide Plate"), 4),
                        BufferedInput::new(Filter::Label("Infinity Bimetal Gear"), 2),
                        BufferedInput::new(Filter::Label("Dark Iron Bars"), 2),
                        BufferedInput::new(Filter::Label("Hardened Cell Frame"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Double-Layer Capacitor"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Basic Capacitor"), 2),
                        BufferedInput::new(Filter::Label("Energetic Alloy Ingot"), 2),
                        BufferedInput::new(Filter::Label("Coke Dust"), 1),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Octadic Capacitor"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Ferroboron Alloy"), 4),
                        BufferedInput::new(Filter::Label("Double-Layer Capacitor"), 2),
                        BufferedInput::new(Filter::Label("Vibrant Alloy Ingot"), 2),
                        BufferedInput::new(Filter::Label("Charged Draconium Block"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Genetics Labware"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Glass Pane"), 4),
                        BufferedInput::new(Filter::Label("Diamond"), 1),
                    ],
                    max_inputs: 5,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulsating Mesh"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulsating Propolis"), 5)],
                    max_inputs: 80,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "wyvern",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:tileiteminputbus_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Wyvern Core"), n_wanted: 16 }],
                inputs: vec![
                    BufferedInput::new(Filter::Label("Pladium"), 1),
                    BufferedInput::new(Filter::Label("Nether Star"), 1),
                    BufferedInput::new(Filter::Label("Shulker Shell"), 1),
                    BufferedInput::new(Filter::Label("Ludicrite Ingot"), 2),
                    BufferedInput::new(Filter::Label("Draconic Core"), 3),
                ],
                max_inputs: 64,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "ludicrite",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:tileiteminputbus_3",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Ludicrite Block"), n_wanted: 16 }],
                inputs: vec![
                    BufferedInput::new(Filter::Label("Ender Amethyst"), 1),
                    BufferedInput::new(Filter::Label("Blutonium Block"), 1),
                    BufferedInput::new(Filter::Label("Alumite Ingot"), 1),
                    BufferedInput::new(Filter::Label("Blaze Mesh"), 1),
                    BufferedInput::new(Filter::Label("Block of Elementium"), 1),
                    BufferedInput::new(Filter::Label("Block of Enderium"), 2),
                ],
                max_inputs: 56,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "crafterZ",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_38",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Sulfur"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Sulfur Essence"), 3)],
                max_inputs: 6,
            }],
        });
    })
}
