use super::detail_cache::DetailCache;
use super::factory::{Factory, FactoryConfig};
use super::server::Server;
use super::{access::*, item::*, process::*, recipe::*, storage::*};
use std::{cell::RefCell, rc::Rc, time::Duration};

pub fn build_factory() -> Rc<RefCell<Factory>> {
    /*
       Igneous
       1: Basalt
       2: Diorite
       3: Gabbro
       4: Andesite

       Herbal
       1: Azure
       2: Goldenrod
       3: Azure
       4: Olive

       Volatile
       1: Blazing
       2: Obsidian
       3: Obsidian
       4: Obsidian

       Crystalline
       1: Nitric
       2: Nether Quartz
       3: Apatite
       4: Nitric

       Metallurgic
       1: Iron
       2: Iron
       3: Iron
       4: Nickel

       Gemstone
       1: Ruby
       2: Emerald
       3: Sapphire
       4: Sapphire

       Chaos
       0x Igneous
       2x Herbal
    */
    FactoryConfig {
        detail_cache: DetailCache::new(),
        server: Server::new(1849),
        min_cycle_time: Duration::from_secs(1),
        log_clients: vec!["1a"],
        bus_accesses: vec![BasicAccess { client: "1a", addr: "minecraft:barrel_38" }],
        backups: vec![
            (Filter::Label("Pure Certus Quartz Crystal"), 16),
            (Filter::Label("Pure Nether Quartz Crystal"), 16),
            (Filter::Name("kubejs:ender_slimy_fern_leaf"), 16),
            (Filter::Name("kubejs:earth_slimy_fern_leaf"), 16),
            (Filter::Name("kubejs:sky_slimy_fern_leaf"), 16),
            (Filter::Name("kubejs:substrate_basalt"), 16),
            (Filter::Name("kubejs:substrate_sulfur"), 16),
            (Filter::Name("kubejs:substrate_slime"), 16),
            (Filter::Name("kubejs:substrate_olive"), 16),
            (Filter::Name("kubejs:substrate_ruby"), 16),
        ],
    }
    .build(|factory| {
        factory.add_storage(DrawerConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "storagedrawers:controller_0",
                bus_addr: "minecraft:barrel_38",
            }],
            filters: vec![
                Filter::Label("Spruce Log"),
                Filter::Label("Spruce Sapling"),
                Filter::Label("Stick"),
                Filter::Label("Kelp"),
                Filter::Label("Cobblestone"),
                Filter::Label("Diorite Cobblestone"),
                Filter::Label("Granite Cobblestone"),
                Filter::Label("Andesite Cobblestone"),
                Filter::Label("Gabbro Cobblestone"),
                Filter::Label("Flax Seeds"),
                Filter::Label("Flint"),
                Filter::Label("Flax"),
                Filter::Label("Rubber"),
                Filter::Label("Sweet Berries"),
                Filter::Label("Weeping Vines"),
                Filter::Label("Twisting Vines"),
                Filter::Label("Sugar Cane"),
                Filter::Label("Wheat Seeds"),
                Filter::Label("Wheat"),
                Filter::Label("Egg"),
                Filter::Label("Beetroot"),
                Filter::Label("Beetroot Seeds"),
                Filter::Label("Buddybeans"),
            ],
        });
        for inv_addr in [
            "minecraft:chest_0",
            "minecraft:chest_1",
            "minecraft:chest_2",
            "minecraft:chest_3",
            "minecraft:chest_4",
            "minecraft:chest_5",
            "minecraft:chest_6",
            "minecraft:chest_7",
            "minecraft:chest_8",
            "minecraft:chest_9",
            "minecraft:chest_10",
            "minecraft:chest_11",
        ] {
            factory.add_storage(ChestConfig {
                accesses: vec![BusAccess { client: "1a", inv_addr, bus_addr: "minecraft:barrel_38" }],
            })
        }
        factory.add_process(SyncAndRestockConfig {
            name: "treeFarm",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_3", bus_addr: "minecraft:barrel_38" }],
            accesses_in: vec![RedstoneAccess { client: "1a", addr: None, side: TOP, bit: Some(0) }],
            accesses_out: vec![RedstoneAccess { client: "1a", addr: None, side: TOP, bit: Some(11) }],
            hold_if_unfilled: false,
            stocks: Box::new(|factory| {
                let mut result = vec![BufferedInput::new(Filter::Label("Spruce Sapling"), 64)];
                if factory.search_n_stored(&Filter::Label("Spruce Log")) < 16 {
                    result.push(BufferedInput::new(Filter::Label("Phyto-Gro"), 8))
                }
                result
            }),
        });
        factory.add_process(BufferedConfig {
            name: "output",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_39",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            stocks: vec![],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "packer",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:basin_0", bus_addr: "minecraft:barrel_38" }],
            input_slots: vec![0, 1, 2],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:dye_entangled_singularity"), n_wanted: 1 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Red Dye"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Quantum Entangled Singularity"), 1, vec![1]),
                    ],
                    max_per_slot: 2,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Blaze Cake Base"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Egg"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Sugar"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Cinder Flour"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("White Wool"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("String"), 4, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Ingot"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Iron Nugget"), 9, vec![0])],
                    max_per_slot: 9,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gold Ingot"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Gold Nugget"), 9, vec![0])],
                    max_per_slot: 9,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Red Sandstone"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Red Sand"), 4, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Charcoal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Charcoal"), 9, vec![0])],
                    max_per_slot: 9,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Emerald"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Emerald Dust"), 1, vec![0])],
                    max_per_slot: 16,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "mixer",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:basin_1", bus_addr: "minecraft:barrel_38" }],
            input_slots: vec![0, 1, 2],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Algal Blend"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Kelp"), 2, vec![0]),
                        SlottedInput::new(Filter::Label("Clay Ball"), 2, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Andesite Alloy"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Algal Brick"), 2, vec![0]),
                        SlottedInput::new(Filter::Label("Andesite Cobblestone"), 2, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Phyto-Gro"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Red Sand"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Apatite Dust"), 2, vec![1]),
                        SlottedInput::new(Filter::Label("Niter"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silica Steel Mix"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Iron Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Charcoal"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Nether Quartz"), 3, vec![2]),
                    ],
                    max_per_slot: 15,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "lavaFan",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:dropper_0",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 128,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Glass"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Red Sand"), 16)],
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
                    outputs: vec![Output { item: Filter::Label("Granite"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Granite Cobblestone"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Charcoal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Spruce Log"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Algal Brick"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Algal Blend"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cured Rubber"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Rubber"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silica Steel"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Silica Steel Mix"), 16)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "waterFan",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_19",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 128,
            recipes: vec![
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
                    outputs: vec![Output { item: Filter::Label("Clay Ball"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sand"), 16)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crushingWheel",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_25",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 128,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output {
                        item: Filter::Fn(Box::new(|_, detail| {
                            detail.label == "Chromatic Compound" || detail.label.ends_with("Paint Ball")
                        })),
                        n_wanted: 4,
                    }],
                    inputs: vec![BufferedInput::new(Filter::Name("kubejs:dye_entangled_singularity"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:ender_slime_fern_paste"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Name("kubejs:ender_slimy_fern_leaf"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:earth_slime_fern_paste"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Name("kubejs:earth_slimy_fern_leaf"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:sky_slime_fern_paste"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Name("kubejs:sky_slimy_fern_leaf"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cinder Flour"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Netherrack"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sugar"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sugar Cane"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sand"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gravel"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("String"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Flax"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gravel"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cobblestone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Red Sand"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Granite"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sky Stone Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sky Stone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Singularity"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Crushing Wheel"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Red Dye"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Beetroot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gold Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:zinc_dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Zinc Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lead Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Copper Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nickel Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nickel Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Powdered Obsidian"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Obsidian"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nether Quartz Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pure Nether Quartz Crystal"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Certus Quartz Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pure Certus Quartz Crystal"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Niter Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Niter"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "saw",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_6", bus_addr: "minecraft:barrel_38" }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Stripped Spruce Log"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Spruce Log"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Spruce Planks"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Stripped Spruce Log"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "slabSaw",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_7", bus_addr: "minecraft:barrel_38" }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Spruce Slab"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Spruce Planks"), 1)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "kineticA",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:deployer_1", bus_addr: "minecraft:barrel_38" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Andesite Alloy"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "kineticB",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:deployer_0", bus_addr: "minecraft:barrel_38" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Andesite Alloy"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "kinetic",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_17",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Name("kubejs:kinetic_mechanism"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Spruce Slab"), 1)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "liquidSkyStone",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:basin_2", bus_addr: "minecraft:barrel_38" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Sky Stone Dust"), 4, vec![0])],
                max_per_slot: 16,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "liquidRedstone",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:basin_4", bus_addr: "minecraft:barrel_38" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Charged Certus Quartz Crystal"), 1, vec![0])],
                max_per_slot: 16,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "charger",
            accesses: vec![BusAccess { client: "1a", inv_addr: "minecraft:barrel_9", bus_addr: "minecraft:barrel_38" }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Charged Certus Quartz Crystal"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Certus Quartz Crystal"), 1)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "waterSpout",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_13",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:tiny_certus_crystal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Certus Quartz Seed"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:small_certus_crystal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Name("kubejs:tiny_certus_crystal"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pure Certus Quartz Crystal"), n_wanted: 32 }],
                    inputs: vec![BufferedInput::new(Filter::Name("kubejs:small_certus_crystal"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:tiny_nether_crystal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Quartz Seed"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:small_nether_crystal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Name("kubejs:tiny_nether_crystal"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pure Nether Quartz Crystal"), n_wanted: 32 }],
                    inputs: vec![BufferedInput::new(Filter::Name("kubejs:small_nether_crystal"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "electronTube",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:basin_7", bus_addr: "minecraft:barrel_38" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Electron Tube"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Pure Certus Quartz Crystal"), 1, vec![0])],
                max_per_slot: 16,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "liquidIronFuel",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "tconstruct:heater_0",
                bus_addr: "minecraft:barrel_38",
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
            name: "liquidIron",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_15",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 64)],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "exchangeCoin",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_16",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 256,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Gold Coin"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Silver Coin"), 64)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "sellBerry",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_press_14",
                bus_addr: "minecraft:barrel_38",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Silver Coin"), n_wanted: 64 }],
                inputs: vec![SlottedInput::new(Filter::Label("Sweet Berries"), 8, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "buyCopper",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_press_12",
                bus_addr: "minecraft:barrel_38",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Copper Ingot"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Silver Coin"), 16, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "buyZinc",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_press_9",
                bus_addr: "minecraft:barrel_38",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Zinc Ingot"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Silver Coin"), 16, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "buyNickel",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_press_8",
                bus_addr: "minecraft:barrel_38",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Nickel Ingot"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Silver Coin"), 32, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "buyLead",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_press_13",
                bus_addr: "minecraft:barrel_38",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Lead Ingot"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Silver Coin"), 32, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "precisionA",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:deployer_2", bus_addr: "minecraft:barrel_38" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Electron Tube"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "precisionB",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:deployer_3", bus_addr: "minecraft:barrel_38" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Electron Tube"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "precision",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_18",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Precision Mechanism"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Name("kubejs:kinetic_mechanism"), 1)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "redAlloy",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:basin_8", bus_addr: "minecraft:barrel_38" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Red Alloy Ingot"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Copper Ingot"), 1, vec![0])],
                max_per_slot: 16,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "liquidCopperFuel",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "tconstruct:heater_1",
                bus_addr: "minecraft:barrel_38",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Charcoal"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "liquidZincFuel",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "tconstruct:heater_2",
                bus_addr: "minecraft:barrel_38",
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
            name: "liquidCopper",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_21",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![BufferedInput::new(Filter::Label("Copper Ingot"), 64)],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "liquidZinc",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_22",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![BufferedInput::new(Filter::Label("Zinc Ingot"), 64)],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "storagedrawers:standard_drawers_1_4",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Brass Ingot"), n_wanted: 16 }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "storagedrawers:standard_drawers_1_5",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Silver Ingot"), n_wanted: 16 }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_chiller_4",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Block of Redstone"), n_wanted: 16 }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_chiller_3",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Obsidian"), n_wanted: 16 }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_refinery_0",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Signalum Ingot"), n_wanted: 16 }],
        });
        factory.add_process(BufferedConfig {
            name: "1x1",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_23",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Certus Quartz Seed"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pure Certus Quartz Crystal"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nether Quartz Seed"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pure Nether Quartz Crystal"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:radiant_coil"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Name("kubejs:radiant_sheet"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Block of Redstone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Certus Quartz Crystal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Certus Quartz Block"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "bufferA",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_24",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crushing Wheel"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Cobblestone"), 16),
                        BufferedInput::new(Filter::Label("Stick"), 5),
                    ],
                    max_inputs: 84,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Framed Glass"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Glass"), 1)],
                    max_inputs: 16,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "bufferB",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_31",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: Some(Box::new(|slot| slot >= 25)),
            to_extract: Some(Box::new(|_, stack| {
                stack.detail.label != "Block of Charcoal" && stack.detail.label != "Framed Glass"
            })),
            stocks: vec![
                BufferedInput::new(Filter::Label("Block of Charcoal"), 64),
                BufferedInput::new(Filter::Label("Framed Glass"), 64),
            ],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "press",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_32",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:radiant_sheet"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Refined Radiance"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:zinc_sheet"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Zinc Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Sheet"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Sheet"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Copper Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Golden Sheet"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Brass Sheet"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Brass Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "netherrack",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_33",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Netherrack"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Cobblestone"), 1)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "lavaSpout",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_36",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Blaze Cake"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Blaze Cake Base"), 1)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "lavaGenBasin",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:basin_9", bus_addr: "minecraft:barrel_38" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Cobblestone"), 1, vec![0])],
                max_per_slot: 16,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "lavaGenHeater",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:deployer_5", bus_addr: "minecraft:barrel_38" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Blaze Cake"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "liquidSoulBasin",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:basin_10", bus_addr: "minecraft:barrel_38" }],
            input_slots: vec![0, 1],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![
                    SlottedInput::new(Filter::Label("Twisting Vines"), 1, vec![0]),
                    SlottedInput::new(Filter::Label("Weeping Vines"), 1, vec![1]),
                ],
                max_per_slot: 16,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "liquidSoulHeater",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:deployer_6", bus_addr: "minecraft:barrel_38" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Charcoal"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "infernal",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_48",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Name("kubejs:infernal_mechanism"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Precision Mechanism"), 1)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "knife",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_43",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:ender_slimy_fern_leaf"), n_wanted: 32 }],
                    inputs: vec![BufferedInput::new(Filter::Name("tconstruct:ender_slime_fern"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:earth_slimy_fern_leaf"), n_wanted: 32 }],
                    inputs: vec![BufferedInput::new(Filter::Name("tconstruct:earth_slime_fern"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:sky_slimy_fern_leaf"), n_wanted: 32 }],
                    inputs: vec![BufferedInput::new(Filter::Name("tconstruct:sky_slime_fern"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "spiritFire",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_45",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("tconstruct:ender_slime_fern"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Name("kubejs:ender_slimy_fern_leaf"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("tconstruct:earth_slime_fern"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Name("kubejs:earth_slimy_fern_leaf"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Name("tconstruct:sky_slime_fern"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Name("kubejs:sky_slimy_fern_leaf"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "stove",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:deployer_7", bus_addr: "minecraft:barrel_38" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gunpowder"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Name("kubejs:earth_slime_fern_paste"), 1, vec![0])],
                    max_per_slot: 6,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bone Meal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Name("kubejs:sky_slime_fern_paste"), 1, vec![0])],
                    max_per_slot: 6,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ender Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Name("kubejs:ender_slime_fern_paste"), 1, vec![0])],
                    max_per_slot: 6,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "tnt",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_47",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("TNT"), n_wanted: 16 }],
                inputs: vec![
                    BufferedInput::new(Filter::Label("Gunpowder"), 5),
                    BufferedInput::new(Filter::Label("Sand"), 4),
                ],
                max_inputs: 36,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "explosionTNT",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:deployer_8", bus_addr: "minecraft:barrel_38" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("TNT"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "explosion",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_49",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Quantum Entangled Singularity"), n_wanted: 1 }],
                inputs: vec![
                    BufferedInput::new(Filter::Label("Singularity"), 1),
                    BufferedInput::new(Filter::Label("Ender Dust"), 1),
                ],
                max_inputs: 2,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "drain",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_50",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![
                BufferedInput::new(Filter::Label("Red Paint Ball"), i32::MAX),
                BufferedInput::new(Filter::Label("Yellow Paint Ball"), i32::MAX),
                BufferedInput::new(Filter::Label("Green Paint Ball"), i32::MAX),
                BufferedInput::new(Filter::Label("Blue Paint Ball"), i32::MAX),
            ],
            max_recipe_inputs: i32::MAX,
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "2x2",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_52",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![BufferedRecipe {
                outputs: vec![],
                inputs: vec![
                    BufferedInput::new(Filter::Label("Magenta Paint Ball"), 1),
                    BufferedInput::new(Filter::Label("Magenta Paint Ball"), 1),
                    BufferedInput::new(Filter::Label("Magenta Paint Ball"), 1),
                    BufferedInput::new(Filter::Label("Magenta Paint Ball"), 1),
                ],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "nickelCompound",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_53",
                bus_addr: "minecraft:barrel_38",
            }],
            input_slots: vec![0, 1],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Name("kubejs:nickel_compound"), n_wanted: 16 }],
                inputs: vec![
                    SlottedInput::new(Filter::Label("Nickel Ingot"), 1, vec![0]),
                    SlottedInput::new(Filter::Label("Iron Dust"), 4, vec![1]),
                ],
                max_per_slot: 16,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "invar",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:dropper_1",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Invar Ingot"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Name("kubejs:nickel_compound"), 1)],
                max_inputs: 16,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "torch",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_54",
                bus_addr: "minecraft:barrel_38",
            }],
            input_slots: vec![0, 1],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Torch"), n_wanted: 16 }],
                inputs: vec![
                    SlottedInput::new(Filter::Label("Charcoal"), 1, vec![0]),
                    SlottedInput::new(Filter::Label("Stick"), 1, vec![1]),
                ],
                max_per_slot: 16,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "bufferD",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_55",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![
                BufferedInput::new(Filter::Label("Torch"), 64),
                BufferedInput::new(Filter::Label("Charcoal"), 64),
            ],
            max_recipe_inputs: i32::MAX,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Refined Radiance"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Chromatic Compound"), 1)],
                max_inputs: 16,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "inductiveA",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:deployer_9", bus_addr: "minecraft:barrel_38" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Name("kubejs:radiant_coil"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "inductiveB",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:deployer_10", bus_addr: "minecraft:barrel_38" }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Name("kubejs:radiant_coil"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "inductive",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_56",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Name("kubejs:inductive_mechanism"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Precision Mechanism"), 1)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "bufferC",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_57",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: i32::MAX,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Networking Cable"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Red Alloy Ingot"), 1),
                        BufferedInput::new(Filter::Label("Stone"), 4),
                    ],
                    max_inputs: 20,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gantry Shaft"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Redstone Dust"), 1),
                        BufferedInput::new(Filter::Label("Andesite Alloy"), 2),
                    ],
                    max_inputs: 12,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "liquidGlass",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_crucible_0",
                bus_addr: "minecraft:barrel_38",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Red Sand"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "bottler",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_bottler_2",
                bus_addr: "minecraft:barrel_38",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_obsidian"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Powdered Obsidian"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_diorite"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Diorite Cobblestone"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_gabbro"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Gabbro Cobblestone"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_andesite"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Andesite Cobblestone"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_granite"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Granite Cobblestone"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_cobblestone"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Cobblestone"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_gunpowder"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Gunpowder"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_certus"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Certus Quartz Dust"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_quartz"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Nether Quartz Dust"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_iron"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Iron Dust"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_gold"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Gold Dust"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_zinc"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Name("kubejs:zinc_dust"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_lead"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lead Dust"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_copper"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Copper Dust"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_nickel"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Nickel Dust"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_silver"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Silver Dust"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_niter"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Niter Dust"), 1, vec![0])],
                    max_per_slot: 16,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "bufferE",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_61",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![
                BufferedInput::new(Filter::Label("Charcoal"), 64),
                BufferedInput::new(Filter::Label("Silver Coin"), 64),
            ],
            max_recipe_inputs: 0,
            recipes: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "inductionSmelter",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_smelter_0",
                bus_addr: "minecraft:barrel_38",
            }],
            input_slots: vec![0, 1, 2],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Hardened Glass"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Nether Quartz Dust"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Obsidian"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Red Sand"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_nether"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Name("kubejs:substrate_volatile"), 1, vec![0]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_red"), 2, vec![1]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_gabbro"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_blaze"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Name("kubejs:substrate_volatile"), 1, vec![0]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_orange"), 2, vec![1]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_andesite"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_slime"), n_wanted: 32 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Name("kubejs:substrate_volatile"), 1, vec![0]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_green"), 2, vec![1]).allow_backup(),
                        SlottedInput::new(Filter::Name("kubejs:substrate_granite"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_basalt"), n_wanted: 32 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Name("kubejs:substrate_igneous"), 1, vec![0]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_ruby"), 2, vec![1]).allow_backup(),
                        SlottedInput::new(Filter::Name("kubejs:substrate_nickel"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_prismarine"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Name("kubejs:substrate_volatile"), 1, vec![0]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_blue"), 2, vec![1]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_cobblestone"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "heatedMixer",
            accesses: vec![BusAccess { client: "1a", inv_addr: "create:basin_11", bus_addr: "minecraft:barrel_38" }],
            input_slots: vec![0, 1, 2],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_magenta"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Name("kubejs:substrate_herbal"), 1, vec![0]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_andesite"), 2, vec![1]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_gabbro"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_yellow"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Name("kubejs:substrate_herbal"), 1, vec![0]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_cobblestone"), 2, vec![1]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_granite"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_red"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Name("kubejs:substrate_herbal"), 1, vec![0]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_diorite"), 2, vec![1]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_andesite"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_orange"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Name("kubejs:substrate_herbal"), 1, vec![0]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_granite"), 2, vec![1]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_diorite"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_arcane"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Name("kubejs:substrate_crystal"), 1, vec![0]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_nether"), 2, vec![1]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_magenta"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_sapphire"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Name("kubejs:substrate_gem"), 1, vec![0]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_copper"), 2, vec![1]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_arcane"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_diamond"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Name("kubejs:substrate_gem"), 1, vec![0]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_gold"), 2, vec![1]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_niter"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_emerald"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Name("kubejs:substrate_gem"), 1, vec![0]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_lead"), 2, vec![1]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_certus"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_green"), n_wanted: 32 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Name("kubejs:substrate_herbal"), 1, vec![0]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_basalt"), 2, vec![1]).allow_backup(),
                        SlottedInput::new(Filter::Name("kubejs:substrate_cobblestone"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_sulfur"), n_wanted: 32 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Name("kubejs:substrate_crystal"), 1, vec![0]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_slime"), 2, vec![1]).allow_backup(),
                        SlottedInput::new(Filter::Name("kubejs:substrate_yellow"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_ruby"), n_wanted: 32 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Name("kubejs:substrate_gem"), 1, vec![0]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_zinc"), 2, vec![1]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_sulfur"), 1, vec![2]).allow_backup(),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_lapis"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Name("kubejs:substrate_gem"), 1, vec![0]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_nickel"), 2, vec![1]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_quartz"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_blue"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Name("kubejs:substrate_herbal"), 1, vec![0]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_gabbro"), 2, vec![1]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_basalt"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_apatite"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Name("kubejs:substrate_crystal"), 1, vec![0]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_prismarine"), 2, vec![1]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_green"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_cinnabar"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Name("kubejs:substrate_gem"), 1, vec![0]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_iron"), 2, vec![1]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_apatite"), 1, vec![2]),
                    ],
                    max_per_slot: 16,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "pulverizer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_pulverizer_0",
                bus_addr: "minecraft:barrel_38",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silver Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Silver Ingot"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Niter"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Red Sandstone"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nether Quartz"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Block of Quartz"), 1, vec![0])],
                    max_per_slot: 16,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "unbottler",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_sawmill_0",
                bus_addr: "minecraft:barrel_38",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Apatite Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Name("kubejs:substrate_apatite"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Emerald Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Name("kubejs:substrate_emerald"), 1, vec![0])],
                    max_per_slot: 16,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "ring",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_62",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Certus Quartz Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pure Certus Quartz Crystal"), 8)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Quartz"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pure Nether Quartz Crystal"), 8)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "brassCasing",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_63",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Brass Casing"), n_wanted: 16 }],
                inputs: vec![
                    BufferedInput::new(Filter::Label("Brass Sheet"), 1),
                    BufferedInput::new(Filter::Label("Spruce Log"), 1),
                ],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "flywheel",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_64",
                bus_addr: "minecraft:barrel_38",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 18,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Flywheel"), n_wanted: 16 }],
                inputs: vec![
                    BufferedInput::new(Filter::Label("Brass Casing"), 1),
                    BufferedInput::new(Filter::Label("Brass Ingot"), 8),
                ],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "laser",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "moreminecarts:minecart_loader_te_3",
                bus_addr: "minecraft:barrel_38",
            }],
            input_slots: vec![0, 1, 2, 3],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Name("kubejs:substrate_igneous"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Name("kubejs:substrate_basalt"), 1, vec![0]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_diorite"), 1, vec![1]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_gabbro"), 1, vec![2]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_andesite"), 1, vec![3]),
                    ],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Glowstone Dust"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Name("kubejs:substrate_cobblestone"), 1, vec![0]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_diorite"), 1, vec![1]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_gabbro"), 1, vec![2]),
                        SlottedInput::new(Filter::Name("kubejs:substrate_andesite"), 1, vec![3]),
                    ],
                    max_per_slot: 1,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "centrifuge",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermal:machine_centrifuge_0",
                bus_addr: "minecraft:barrel_38",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Name("kubejs:failed_alchemy_7"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        /*
        factory.add_process(SyncAndRestockConfig {
            name: "bore",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:barrel_58",
                bus_addr: "minecraft:barrel_38",
            }],
            accesses_in: vec![RedstoneAccess { client: "1a", addr: None, side: LEFT, bit: None }],
            accesses_out: vec![RedstoneAccess { client: "1a", addr: None, side: FRONT, bit: None }],
            hold_if_unfilled: true,
            stocks: Box::new(|_| {
                vec![
                    BufferedInput::new(Filter::Label("Networking Cable"), 64),
                    BufferedInput::new(Filter::Label("Gantry Shaft"), 64),
                ]
            }),
        });
        */
    })
}
