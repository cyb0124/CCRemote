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
    FactoryConfig {
        detail_cache: DetailCache::new(),
        server: Server::new(1847),
        min_cycle_time: Duration::from_secs(1),
        log_clients: vec!["1a"],
        bus_accesses: vec![BasicAccess { client: "1a", addr: "ic2:wooden_storage_box_0" }],
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
            (Filter::Label("Netherrack"), 32),
            (Filter::Label("Sugar Canes"), 32),
            (Filter::Label("Nether Wart"), 32),
            (Filter::Label("Sweet Potato"), 32),
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
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            filters: vec![
                Filter::Label("Coal"),
                Filter::Label("Flint"),
                Filter::Label("Coal Coke"),
                Filter::Label("Firm Tofu"),
                Filter::Label("Tin Ore Piece"),
                Filter::Label("Iron Ore Piece"),
                Filter::Label("Gold Ore Piece"),
                Filter::Label("Lead Ore Piece"),
                Filter::Label("Pulverized Gold"),
                Filter::Label("Silver Ore Piece"),
                Filter::Label("Nickel Ore Piece"),
                Filter::Label("Copper Ore Piece"),
                Filter::Label("Osmium Ore Piece"),
                Filter::Label("Aluminium Ore Piece"),
                Filter::Label("Crushed Black Quartz"),
                Filter::Label("Certus Quartz Crystal"),
                Filter::Label("Sky Stone Dust"),
                Filter::Label("Oak Sapling"),
                Filter::Label("Grain Bait"),
                congealed_green(),
            ],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchestlarge_0",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchestlarge_1",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
        });
        factory.add_storage(ChestConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:giantchestlarge_2",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
        });
        factory.add_process(BufferedConfig {
            name: "platePressOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_4",
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            stocks: vec![],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "mkCrusherOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_2",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            stocks: vec![],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "mkInfuserOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_12",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            stocks: vec![],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "enrichmentOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_11",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            stocks: vec![],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "inscriberOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_20",
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
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
                inv_addr: "ic2:wooden_storage_box_24",
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Cobblestone"), n_wanted: 64 }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_13",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: congealed_blue(), n_wanted: 64 }],
        });
        factory.add_process(BufferedConfig {
            name: "ethylene",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_10",
                bus_addr: "ic2:wooden_storage_box_0",
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
                inv_addr: "industrialforegoing:item_splitter_tile_3",
                bus_addr: "ic2:wooden_storage_box_0",
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
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterA",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_0",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
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
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            input_slots: vec![6, 7, 8, 9, 11, 12, 13, 14],
            to_extract: None,
            recipes: vec![
                ScatteringRecipe::new(
                    vec![
                        Output { item: Filter::Label("Seeds"), n_wanted: 64 },
                        Output { item: Filter::Label("Wheat"), n_wanted: 64 },
                    ],
                    ScatteringInput::new(Filter::Label("Seeds")).allow_backup(),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Potato"), n_wanted: 64 }],
                    ScatteringInput::new(Filter::Label("Potato")).allow_backup(),
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
                    vec![Output { item: Filter::Label("Soybean"), n_wanted: 64 }],
                    ScatteringInput::new(Filter::Label("Soybean")).allow_backup(),
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
        factory.add_process(BufferedConfig {
            name: "furnace",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_4",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 24,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cactus Green"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cactus"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Plastic"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dry Rubber"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("minecraft:netherbrick"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Netherrack"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Brick"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Name("minecraft:clay_ball"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bread"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(
                        Filter::Both { name: "appliedenergistics2:material", label: "Flour" },
                        1,
                    )],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Baked Potato"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Potato"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Glass"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sand"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Stone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cobblestone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Charcoal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Oak Wood"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Graphite Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Charcoal"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Uranium Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Uranium Grit"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminum Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Aluminum"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silver Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Silver"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Osmium Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Osmium Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Magnesium Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Magnesium Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Thorium Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Thorium Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Copper"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Iron"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gold Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Gold"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Lead"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Tin Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Tin"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cobalt Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cobalt Ore Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ardite Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Ardite Ore Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Starmetal Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Stardust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cooked Tofurkey"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Raw Tofurkey"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Black Quartz"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Crushed Black Quartz"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Thickened Glass"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sandy Glass"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "pressurizer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:pressurizer_0",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
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
            ],
        });
        factory.add_process(SlottedConfig {
            name: "manufactory",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:manufactory_0",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bone Meal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Bone"), 1, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Flint"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Gravel"), 1, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silicon Ingot"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Sand"), 1, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Clay Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Name("minecraft:clay"), 1, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Oak Wood Planks"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Oak Wood"), 1, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sand"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Cobblestone"), 1, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Uranium Grit"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Uranium Ore"), 1, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Aluminum"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Aluminum Ore"), 1, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Silver"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Silver Ore"), 1, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Osmium Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Osmium Ore"), 1, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Copper"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Copper Ore"), 1, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Iron"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Iron Ore"), 1, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Lead"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lead Ore"), 1, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Tin"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Tin Ore"), 1, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cobalt Ore Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Cobalt Ore"), 1, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ardite Ore Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Ardite Ore"), 1, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lithium Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lithium Ore"), 1, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Magnesium Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Magnesium Ore"), 1, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Thorium Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Thorium Ore"), 1, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Boron Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Boron Ore"), 1, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Niter"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Sandstone"), 1, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electrum Blend"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Electrum Ingot"), 1, vec![0])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output {
                        item: Filter::Both { name: "appliedenergistics2:material", label: "Flour" },
                        n_wanted: 16,
                    }],
                    inputs: vec![SlottedInput::new(Filter::Label("Wheat"), 1, vec![0])],
                    max_per_slot: 24,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "alloyFurnace",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:alloy_furnace_0",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            input_slots: vec![0, 1],
            to_extract: Some(Box::new(|slot, _| slot == 2)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electrical Steel Ingot"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Steel Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Silicon Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
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
            ],
        });
        factory.add_process(BufferedConfig {
            name: "platePress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_5",
                bus_addr: "ic2:wooden_storage_box_0",
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
            ],
        });
        factory.add_process(BufferedConfig {
            name: "gearPress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_6",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 32,
            recipes: vec![
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
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("White Concrete"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("White Concrete Powder"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Infinite Water Source"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Empty Frame"), 1, vec![0])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "autoCompressor",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "excompressum:auto_compressor_0",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 12)),
            to_extract: Some(Box::new(|slot, _| slot >= 12)),
            stocks: vec![],
            max_recipe_inputs: 128,
            recipes: vec![
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
                    outputs: vec![Output { item: Filter::Label("Block of Black Iron"), n_wanted: 16 }],
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
                    outputs: vec![Output { item: Filter::Label("Compressed Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dust"), 9)],
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
                    outputs: vec![Output { item: Filter::Label("Uranium Ore"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Uranium Ore Piece"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Green Slime Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Name("minecraft:slime_ball"), 4)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "atomic",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "randomthings:irondropper_0",
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
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
            ],
        });
        factory.add_process(BufferedConfig {
            name: "arPress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:placer_0",
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![
                BufferedInput::new(Filter::Label("Peanut"), 64),
                BufferedInput::new(Filter::Label("Redstone"), 64),
                BufferedInput::new(Filter::Label("Ender Pearl"), 64),
                BufferedInput::new(Filter::Label("Lumium Ingot"), 64),
                BufferedInput::new(Filter::Label("Cryotheum Dust"), 64),
                BufferedInput::new(Filter::Label("Glowstone Dust"), 64),
                BufferedInput::new(Filter::Label("Lapis Lazuli Dust"), 64),
                BufferedInput::new(Filter::Label("Blue Slime Sapling"), 64),
                BufferedInput::new(Filter::Label("Experience Essence"), 64),
            ],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "pulverizer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_15",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![BufferedInput::new(Filter::Label("Fluxed Phyto-Gro"), 64)],
            max_recipe_inputs: 16,
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
                    outputs: vec![Output { item: Filter::Label("Sulfur"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Blaze Rod"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "mkInfuser",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_13",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![
                BufferedInput::new(Filter::Label("Compressed Redstone"), 64),
                BufferedInput::new(Filter::Label("Compressed Diamond"), 64),
            ],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "kekimurus",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:placer_1",
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![
                BufferedInput::new(Filter::Label("White Concrete"), 256),
                BufferedInput::new(Filter::Label("Super Lubricent Stone"), 256),
            ],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "impregStick",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:carpenter_0",
                bus_addr: "ic2:wooden_storage_box_0",
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
            name: "hardenedCasing",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:carpenter_2",
                bus_addr: "ic2:wooden_storage_box_0",
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
        factory.add_process(SlottedConfig {
            name: "overclocker",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:carpenter_3",
                bus_addr: "ic2:wooden_storage_box_0",
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
            accesses: vec![BusAccess { client: "1a", inv_addr: "xu2:tileuse_6", bus_addr: "ic2:wooden_storage_box_0" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Aquamarine"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "dropInLiquid",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "randomthings:irondropper_1",
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot < 21)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Glowstone Dust"), n_wanted: 16 },
                        Output { item: Filter::Label("Blaze Powder"), n_wanted: 16 },
                    ],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed Soul Sand"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Uranium Ore Piece"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed Ender Gravel"), 1, vec![0])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "diamondSieve",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "excompressum:auto_heavy_sieve_2",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot < 21)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![
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
                    outputs: vec![Output { item: Filter::Label("Redstone"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed Dust"), 1, vec![0])],
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
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            input_slots: vec![0, 3, 4],
            to_extract: extract_all(),
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
            name: "tinElectronTube",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:fabricator_1",
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
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
                inv_addr: "rftools:crafter3_4",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
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
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Tiny Dry Rubber"), n_wanted: 64 }],
        });
        factory.add_process(SlottedConfig {
            name: "compressor",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:compressor_0",
                bus_addr: "ic2:wooden_storage_box_0",
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
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterD",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_6",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
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
        factory.add_process(BufferedConfig {
            name: "enrichment",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_5",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 8,
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
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Obsidian"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Obsidian"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pure Nether Quartz Crystal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Quartz Seed"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pure Certus Quartz Crystal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Certus Quartz Seed"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pure Fluix Crystal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Fluix Seed"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Stardust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Starmetal Ore"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "redstoneInfuser",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_6",
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
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
        factory.add_process(SlottedConfig {
            name: "pinkSlime",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "storagedrawers:compdrawers_0",
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
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
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
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
            ],
        });
        factory.add_process(SlottedConfig {
            name: "teFrame",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:fabricator_3",
                bus_addr: "ic2:wooden_storage_box_0",
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
            name: "xpTransposer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_transposer_6",
                bus_addr: "ic2:wooden_storage_box_0",
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
            ],
        });
        factory.add_process(SlottedConfig {
            name: "glowstoneTransposer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_transposer_0",
                bus_addr: "ic2:wooden_storage_box_0",
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
                inv_addr: "thermalexpansion:machine_transposer_8",
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
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
            ],
        });
        factory.add_process(SlottedConfig {
            name: "charger",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_charger_3",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fluxed Phyto-Gro"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Rich Phyto-Gro"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Charged Certus Quartz Crystal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Certus Quartz Crystal"), 1, vec![0])],
                    max_per_slot: 16,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "phyto",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_16",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![
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
                    outputs: vec![Output { item: Filter::Label("Knightslime Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Knightslime Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Experience Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Experience Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "enderTransposer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_transposer_4",
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Obsidian"), n_wanted: 64 }],
        });
        factory.add_process(BufferedConfig {
            name: "pureDaisy",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "appliedenergistics2:interface_4",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 8,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Livingwood"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Infused Wood"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Livingrock"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Arcane Stone"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "transmutation",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "appliedenergistics2:interface_3",
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
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
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterH",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_14",
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
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
                inv_addr: "ic2:wooden_storage_box_25",
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Snowball"), n_wanted: 64 }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_refinery_1",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Rosin"), n_wanted: 64 }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_crafter_2",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Fresh Water"), n_wanted: 64 }],
        });
        factory.add_process(BufferedConfig {
            name: "crafterI",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_17",
                bus_addr: "ic2:wooden_storage_box_0",
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
                    outputs: vec![Output { item: Filter::Label("Salt"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Fresh Water"), 1)],
                    max_inputs: 16,
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
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cooking Oil"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Pumpkin"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silken Tofu"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Soybean"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sugar"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Sugar Canes"), 1, vec![0])],
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
        factory.add_process(BufferedConfig {
            name: "crafterJ",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_18",
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Netherrack"), n_wanted: 64 }],
                inputs: vec![BufferedInput::new(Filter::Label("Netherrack"), 1).allow_backup()],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "manaPool",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:dropper_2",
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 4,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Elementium Ingot"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Manasteel Ingot"), 2)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "crafterK",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_19",
                bus_addr: "ic2:wooden_storage_box_0",
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
            name: "empowerer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_26",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            input_slots: vec![0, 1, 2, 3, 4],
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
                        SlottedInput::new(Filter::Label("Block of Black Iron"), 1, vec![2]),
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
            ],
        });
        factory.add_process(SlottedConfig {
            name: "isotopeSeparator",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:isotope_separator_0",
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: extract_all(),
            recipes: vec![SlottedRecipe {
                outputs: vec![
                    Output { item: Filter::Label("Uranium-238"), n_wanted: 16 },
                    Output { item: Filter::Label("Tiny Clump of Uranium-235"), n_wanted: 16 },
                ],
                inputs: vec![SlottedInput::new(Filter::Label("Uranium Grit"), 1, vec![0])],
                max_per_slot: 8,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "ncCrusher",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:rock_crusher_0",
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
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
                bus_addr: "ic2:wooden_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
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
                inv_addr: "thermalexpansion:machine_transposer_7",
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
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
                inv_addr: "thermalexpansion:device_tapper_0",
                bus_addr: "ic2:wooden_storage_box_0",
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
                bus_addr: "ic2:wooden_storage_box_0",
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
                inv_addr: "rftools:crafter3_25",
                bus_addr: "ic2:wooden_storage_box_0",
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
            ],
        });
        factory.add_process(SlottedConfig {
            name: "mobCrusher",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:mob_relocator_tile_0",
                bus_addr: "ic2:wooden_storage_box_0",
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
        factory.add_process(BufferedConfig {
            name: "trash",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "xu2:tiletrashchest_1",
                bus_addr: "ic2:wooden_storage_box_0",
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
    })
}
