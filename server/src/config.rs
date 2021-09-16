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
        bus_accesses: vec![BasicAccess { client: "1a", addr: "ic2:iridium_storage_box_0" }],
        backups: vec![
            (Filter::Label("Dandelion"), 32),
            (Filter::Label("Sugar Canes"), 32),
            (Filter::Label("Potato"), 32),
            (Filter::Label("Soybean"), 32),
            (Filter::Label("Seeds"), 32),
            (Filter::Label("Pumpkin Seeds"), 32),
            (Filter::Label("Peanut"), 32),
            (Filter::Label("Cactus"), 32),
            (Filter::Label("Nether Wart"), 32),
            (Filter::Label("Psimetal Ingot"), 32),
            (Filter::Label("Psidust"), 32),
            (Filter::Label("Psigem"), 32),
            (Filter::Label("Ebony Substance"), 32),
            (Filter::Label("Ivory Substance"), 32),
            (Filter::Name("minecraft:red_mushroom"), 32),
            (Filter::Name("minecraft:brown_mushroom"), 32),
        ],
    }
    .build(|factory| {
        let nether_dust = || Filter::Both { label: "Crushed Netherrack", name: "skyresources:techitemcomponent" };
        let nether_gravel = || Filter::Name("exnihilocreatio:block_netherrack_crushed");
        let cable_modem = || Filter::Both { label: "Wired Modem", name: "computercraft:cable" };
        let clay_block = || Filter::Name("minecraft:clay");
        let clay_ball = || Filter::Name("minecraft:clay_ball");
        let nether_brick = || Filter::Name("minecraft:netherbrick");
        let rf_frame = || Filter::Name("rftools:machine_frame");
        let te_frame = || Filter::Both { label: "Machine Frame", name: "thermalexpansion:frame" };
        let mk_energy_upgrade = || Filter::Name("mekanism:energyupgrade");
        let mk_speed_upgrade = || Filter::Name("mekanism:speedupgrade");
        let pam_flour = || Filter::Name("harvestcraft:flouritem");
        let blue_slime = || Filter::Fn(Box::new(|item, _| item.name == "tconstruct:edible" && item.damage == 1));
        let green_slime = || Filter::Name("minecraft:slime_ball");
        let blue_congealed_slime =
            || Filter::Fn(Box::new(|item, detail| detail.label == "Congealed Slime Block" && item.damage == 1));
        let inferium_seeds = || Filter::Name("mysticalagriculture:tier4_inferium_seeds");
        let ic2_adv_circuit = || Filter::Both { label: "Advanced Circuit", name: "ic2:crafting" };
        let ar_adv_circuit = || Filter::Both { label: "Advanced Circuit", name: "advancedrocketry:ic" };
        let black_iron_block = || Filter::Both { label: "Block of Black Iron", name: "extendedcrafting:storage" };
        let crystaltine_trimmed =
            || Filter::Fn(Box::new(|item, _| item.name == "extendedcrafting:trimmed" && item.damage == 4));
        factory.add_storage(DrawerConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "storagedrawers:controller_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            filters: vec![
                Filter::Label("Flax Seeds"),
                Filter::Label("Dirty Apatite"),
                Filter::Label("Dirty Lapis Lazuli"),
                Filter::Label("Dirty Black Quartz"),
                Filter::Label("Dirty Certus Quartz"),
                Filter::Label("Cobalt Ore Piece"),
                Filter::Label("Ardite Ore Piece"),
                Filter::Label("Grain Bait"),
                Filter::Label("Firm Tofu"),
                Filter::Label("Dirty Amber"),
                Filter::Label("Coal Coke"),
                Filter::Label("Dragon Egg Essence"),
                Filter::Label("Dirty Aquamarine"),
                Filter::Label("Dirty Ruby"),
                Filter::Label("Dirty Peridot"),
                Filter::Label("Dirty Sapphire"),
                Filter::Label("Thorium Ore Piece"),
                Filter::Label("Magnesium Ore Piece"),
                Filter::Label("Boron Ore Piece"),
                Filter::Label("Nether Quartz"),
                Filter::Label("Bone"),
                Filter::Label("Wither Ash"),
                Filter::Label("Nether Star Essence"),
                Filter::Label("Lapis Lazuli"),
                Filter::Label("Uranium-238"),
                Filter::Label("Glowstone Dust"),
                Filter::Label("Aquamarine"),
                Filter::Label("Cyanite Ingot"),
                Filter::Label("Rice Seed"),
                inferium_seeds(),
                Filter::Label("Gold Ore Piece"),
                Filter::Label("Tiny Pile of Plutonium"),
                Filter::Label("Emerald"),
                Filter::Label("Mysterious Comb"),
                Filter::Label("Platinum Ingot"),
                Filter::Label("Iridium Ingot"),
                Filter::Label("Charged Certus Quartz Crystal"),
                Filter::Label("Diamond"),
                Filter::Label("Aluminum Ingot"),
                Filter::Label("Mana Infused Ingot"),
                Filter::Label("Aethium Crystal"),
                Filter::Label("Pladium Crystal"),
                Filter::Label("Litherite Crystal"),
                Filter::Label("Erodium Crystal"),
                Filter::Label("Ionite Crystal"),
                Filter::Label("Kyronite Crystal"),
                Filter::Label("Dimensional Shard"),
                Filter::Label("Gold Ore"),
                Filter::Label("Draconium Ore"),
                Filter::Label("Cobalt Ore"),
                Filter::Label("Ardite Ore"),
                Filter::Label("Redstone Seeds"),
                Filter::Label("Nether Seeds"),
                Filter::Label("Poisonous Potato"),
                Filter::Label("Infinity Dust"),
                Filter::Label("Ender Drone"),
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
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:cobblestone_generator_dense_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Cobblestone"), n_wanted: 64 }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_4",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Obsidian"), n_wanted: 64 }],
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
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:crystallizer_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Calcium Sulfate"), n_wanted: 64 }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:crystallizer_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Hexagonal Boron Nitride"), n_wanted: 64 }],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_refinery_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Rosin"), n_wanted: 64 }],
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
                inv_addr: "thermalexpansion:machine_crafter_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Fresh Water"), n_wanted: 64 }],
        });
        factory.add_process(SlottedConfig {
            name: "plantGathererOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:crop_recolector_tile_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![],
            to_extract: Some(Box::new(|slot, _| slot >= 6)),
            recipes: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "mobCrusherOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:mob_relocator_tile_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![],
            to_extract: Some(Box::new(|slot, _| slot >= 6)),
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "combustionOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "skyresources:combustioncollectortile_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "platePressOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_20",
                bus_addr: "ic2:iridium_storage_box_0",
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
                bus_addr: "ic2:iridium_storage_box_0",
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
                inv_addr: "minecraft:ender chest_0",
                bus_addr: "ic2:iridium_storage_box_0",
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
                inv_addr: "thermalexpansion:device_item_buffer_6",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "infuserOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_10",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "furnaceOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "combinerOutput",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_22",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: extract_all(),
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "crusher",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gravel"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cobblestone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bio Fuel"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Potato"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Coke Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Coal Coke"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 16,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "enrichment",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_3",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Dimensional Shard Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Charged Certus Quartz Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Quartz Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Diamond Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Emerald Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Glowstone"), 1).extra_backup(64)],
                    max_inputs: i32::MAX,
                },
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
                    outputs: vec![Output { item: Filter::Label("Compressed Obsidian"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Refined Obsidian Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pure Certus Quartz Crystal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Certus Quartz Seed"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pure Nether Quartz Crystal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Quartz Seed"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pure Fluix Crystal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Fluix Seed"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Draconium Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Draconium Ore"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 16,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "lavaBarrel",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_5",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("End Stone"), n_wanted: 64 }],
                inputs: vec![BufferedInput::new(Filter::Label("Glowstone Dust"), 1)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "redstoneInfuser",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_4",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
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
                    inputs: vec![BufferedInput::new(te_frame(), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "diamondInfuser",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_5",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
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
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "obsidianInfuser",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_10",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Atomic Alloy"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Reinforced Alloy"), 1)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "osmiumCompressor",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_7",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Glowstone Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Glowstone Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Refined Obsidian Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Refined Obsidian Dust"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "furnace",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Copper"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Aluminum"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Silver"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Uranium Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Manganese Oxide Dust"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Crushed Rhodochrosite"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("HOP Graphite Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("HOP Graphite Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cactus Green"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cactus"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Glass"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sand"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Brick"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(clay_ball(), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Seared Brick"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Grout"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: nether_brick(), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Netherrack"), 1)],
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
                    outputs: vec![Output { item: Filter::Label("Gold Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Gold"), 1)],
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
                    outputs: vec![Output { item: Filter::Label("Uranium Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Uranium Grit"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Boron Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Boron Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lithium Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lithium Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Magnesium Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Magnesium Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Zirconium Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Zirconium Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Beryllium Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Beryllium Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Draconium Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Draconium Dust"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Plastic"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dry Rubber"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 16,
            stocks: vec![],
        });
        factory.add_process(RedstoneConditionalConfig {
            name: Some("combustionA"),
            accesses: vec![RedstoneAccess { client: "1a", addr: Some("redstone_integrator_1"), side: WEST }],
            condition: Box::new(|x| x == 0),
            child: BufferedConfig {
                name: "combustionA",
                accesses: vec![BusAccess {
                    client: "1a",
                    inv_addr: "randomthings:irondropper_2",
                    bus_addr: "ic2:iridium_storage_box_0",
                }],
                slot_filter: None,
                to_extract: None,
                recipes: vec![
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Gunpowder"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Flint"), 1)],
                        max_inputs: 4,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Blaze Powder"), n_wanted: 16 }],
                        inputs: vec![BufferedInput::new(Filter::Label("Gunpowder"), 1)],
                        max_inputs: 4,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: blue_slime(), n_wanted: 16 }],
                        inputs: vec![
                            BufferedInput::new(green_slime(), 1),
                            BufferedInput::new(Filter::Label("Lapis Lazuli"), 2),
                        ],
                        max_inputs: 12,
                    },
                ],
                max_recipe_inputs: i32::MAX,
                stocks: vec![],
            },
        });
        factory.add_process(RedstoneConditionalConfig {
            name: Some("combustionB"),
            accesses: vec![RedstoneAccess { client: "1a", addr: Some("redstone_integrator_1"), side: EAST }],
            condition: Box::new(|x| x == 0),
            child: BufferedConfig {
                name: "combustionB",
                accesses: vec![BusAccess {
                    client: "1a",
                    inv_addr: "randomthings:irondropper_3",
                    bus_addr: "ic2:iridium_storage_box_0",
                }],
                slot_filter: None,
                to_extract: None,
                recipes: vec![
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Glowstone Dust"), n_wanted: 16 }],
                        inputs: vec![
                            BufferedInput::new(Filter::Label("Redstone"), 4),
                            BufferedInput::new(Filter::Label("Blaze Powder"), 2),
                        ],
                        max_inputs: 24,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Dark Matter"), n_wanted: 16 }],
                        inputs: vec![
                            BufferedInput::new(Filter::Label("Soul Sand"), 5),
                            BufferedInput::new(Filter::Label("Hardened Coal Block"), 3),
                            BufferedInput::new(Filter::Label("Refined Obsidian Ingot"), 1),
                            BufferedInput::new(Filter::Label("Enderium Ingot"), 2),
                            BufferedInput::new(Filter::Label("Platinum Ingot"), 4),
                        ],
                        max_inputs: 60,
                    },
                    BufferedRecipe {
                        outputs: vec![Output { item: Filter::Label("Light Matter"), n_wanted: 16 }],
                        inputs: vec![
                            BufferedInput::new(Filter::Label("Heavy Snow"), 5),
                            BufferedInput::new(Filter::Label("Frozen Iron Ingot"), 4),
                            BufferedInput::new(Filter::Label("Alchemical Gold Ingot"), 4),
                            BufferedInput::new(Filter::Label("End Stone"), 3),
                        ],
                        max_inputs: 64,
                    },
                ],
                max_recipe_inputs: i32::MAX,
                stocks: vec![],
            },
        });
        factory.add_process(SlottedConfig {
            name: "manufactory",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:manufactory_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Graphite Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Graphite Ingot"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lapis Lazuli Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lapis Lazuli"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Diamond Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Diamond"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bioplastic"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Sugar Canes"), 2, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("HOP Graphite Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Coke Dust"), 8, vec![0])],
                    max_per_slot: 64,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bone Meal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Bone"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Clay Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(clay_block(), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Birch Wood Planks"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Birch Wood"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silicon Ingot"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Sand"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sand"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Cobblestone"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crushed End Stone"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("End Stone"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Niter"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Sandstone"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Flint"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Gravel"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Coal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Coal"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Charcoal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Charcoal"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Osmium Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Osmium Ingot"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Iron"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Iron Ingot"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Nickel"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Nickel Ingot"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Tin"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Tin Ingot"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Lead"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lead Ingot"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Platinum"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Platinum Ingot"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Obsidian"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Obsidian"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cobalt Ore Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Cobalt Ore"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ardite Ore Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Ardite Ore"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Boron Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Boron Ore"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lithium Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lithium Ore"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Magnesium Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Magnesium Ore"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Thorium Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Thorium Ore"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Uranium Grit"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Uranium Ore"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electrum Blend"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Electrum Ingot"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Enderium Blend"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Enderium Ingot"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Certus Quartz Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Certus Quartz Crystal"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nether Quartz Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Nether Quartz"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fluix Dust"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Fluix Crystal"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crushed Rhodochrosite"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Rhodochrosite"), 1, vec![0])],
                    max_per_slot: 16,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "fusionTableStockA",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "skyresources:fusiontable_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 10)),
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Quartus Alchemical Dust"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "fusionTableStockB",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "skyresources:fusiontable_3",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 10)),
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Quartus Alchemical Dust"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "fusionTableStockC",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "skyresources:fusiontable_4",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 10)),
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Quartus Alchemical Dust"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "fusionTableStockD",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "skyresources:fusiontable_5",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 10)),
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Quartus Alchemical Dust"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "fusionTableA",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "skyresources:fusiontable_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Alchemical Ore Dust"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Blaze Powder"), 2, vec![2]),
                        SlottedInput::new(Filter::Label("Rotten Flesh"), 1, vec![6]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Alchemical Ore Dust"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Blaze Powder"), 2, vec![2]),
                        SlottedInput::new(clay_block(), 1, vec![7]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Tin Alchemical Ore Dust"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Blaze Powder"), 2, vec![2]),
                        SlottedInput::new(Filter::Label("Bone"), 1, vec![8]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silver Alchemical Ore Dust"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Sugar"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Blaze Powder"), 2, vec![2]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminum Alchemical Ore Dust"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Blaze Powder"), 2, vec![2]),
                        SlottedInput::new(Filter::Label("Enriched Bonemeal"), 1, vec![9]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crystal Shard"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Glass"), 1, vec![3])],
                    max_per_slot: 4,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Alchemical Diamond"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Diamond"), 1, vec![4]),
                        SlottedInput::new(Filter::Label("Redstone"), 8, vec![5]),
                    ],
                    max_per_slot: 32,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "fusionTableB",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "skyresources:fusiontable_3",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Alchemical Gold Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Glowstone Dust"), 3, vec![1]),
                        SlottedInput::new(Filter::Label("Gold Ingot"), 1, vec![7]),
                    ],
                    max_per_slot: 12,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Osmium Alchemical Ore Dust"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Glowstone Dust"), 2, vec![1]),
                        SlottedInput::new(clay_ball(), 1, vec![3]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Alchemical Ore Dust"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Gunpowder"), 2, vec![4]),
                        SlottedInput::new(Filter::Label("Pumpkin Seeds"), 1, vec![5]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nickel Alchemical Ore Dust"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Glowstone Dust"), 2, vec![1]),
                        SlottedInput::new(Filter::Label("Iron Ingot"), 1, vec![8]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Astral Starmetal Alchemical Ore Dust"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Glowstone Dust"), 2, vec![1]),
                        SlottedInput::new(Filter::Label("Stardust"), 1, vec![9]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dirt"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Plant Matter"), 6, vec![6])],
                    max_per_slot: 24,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Alchemical Coal"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Coal"), 1, vec![2]),
                        SlottedInput::new(Filter::Label("Gunpowder"), 3, vec![4]),
                    ],
                    max_per_slot: 12,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "fusionTableC",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "skyresources:fusiontable_4",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Terra Vis Crystal"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Dirt"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Quartz Sliver"), 9, vec![2]),
                    ],
                    max_per_slot: 36,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Stardust"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Glowstone Dust"), 2, vec![3]),
                        SlottedInput::new(Filter::Label("Iron Ingot"), 1, vec![4]),
                        SlottedInput::new(Filter::Label("Aquamarine"), 2, vec![5]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Alchemical Iron Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Iron Ingot"), 1, vec![4]),
                        SlottedInput::new(Filter::Label("Blaze Powder"), 3, vec![6]),
                    ],
                    max_per_slot: 12,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "fusionTableD",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "skyresources:fusiontable_5",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Quartus Alchemical Dust"), n_wanted: 64 }],
                inputs: vec![
                    SlottedInput::new(Filter::Label("Dark Matter"), 1, vec![1]),
                    SlottedInput::new(Filter::Label("Light Matter"), 1, vec![2]),
                    SlottedInput::new(Filter::Label("Alchemical Coal"), 6, vec![3]),
                    SlottedInput::new(Filter::Label("Alchemical Diamond"), 6, vec![4]),
                    SlottedInput::new(Filter::Label("Emerald"), 2, vec![5]),
                ],
                max_per_slot: 24,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "condenserStock",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_crucible_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Crystal Shard"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "condenser",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "skyresources:tilecasing_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Ingot"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Iron Alchemical Ore Dust"), 1, vec![0])],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Ingot"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lead Alchemical Ore Dust"), 1, vec![0])],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Tin Ingot"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Tin Alchemical Ore Dust"), 1, vec![0])],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silver Ingot"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Silver Alchemical Ore Dust"), 1, vec![0])],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminum Ingot"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Aluminum Alchemical Ore Dust"), 1, vec![0])],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Osmium Ingot"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Osmium Alchemical Ore Dust"), 1, vec![0])],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Ingot"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Copper Alchemical Ore Dust"), 1, vec![0])],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nickel Ingot"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Nickel Alchemical Ore Dust"), 1, vec![0])],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Starmetal Ingot"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Astral Starmetal Alchemical Ore Dust"), 1, vec![0])],
                    max_per_slot: 1,
                },
            ],
        });
        factory.add_process(ScatteringConfig {
            name: "plantSower",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:crop_sower_tile_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![6, 7, 8, 9, 11, 12, 13, 14],
            to_extract: None,
            recipes: vec![
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Seeds"), n_wanted: 64 }],
                    ScatteringInput::new(Filter::Label("Seeds")).allow_backup(),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Soybean"), n_wanted: 64 }],
                    ScatteringInput::new(Filter::Label("Soybean")).allow_backup(),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Peanut"), n_wanted: 64 }],
                    ScatteringInput::new(Filter::Label("Peanut")).allow_backup(),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Birch Wood"), n_wanted: 64 }],
                    ScatteringInput::new(Filter::Label("Birch Sapling")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("Rice"), n_wanted: 64 }],
                    ScatteringInput::new(Filter::Label("Rice Seeds")),
                ),
                ScatteringRecipe::new(
                    vec![Output { item: Filter::Label("String"), n_wanted: 64 }],
                    ScatteringInput::new(Filter::Label("Flax Seeds")),
                ),
            ],
            max_per_slot: 4,
        });
        factory.add_process(BufferedConfig {
            name: "srCrusher",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_24",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 40,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Dirty Ruby"), n_wanted: 16 },
                        Output { item: Filter::Label("Dirty Diamond"), n_wanted: 16 },
                        Output { item: Filter::Label("Dirty Sapphire"), n_wanted: 16 },
                        Output { item: Filter::Label("Dirty Aquamarine"), n_wanted: 16 },
                        Output { item: Filter::Label("Dirty Lapis Lazuli"), n_wanted: 16 },
                        Output { item: Filter::Label("Dirty Black Quartz"), n_wanted: 16 },
                        Output { item: Filter::Label("Dirty Certus Quartz"), n_wanted: 16 },
                    ],
                    inputs: vec![BufferedInput::new(Filter::Label("Stone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dirty Malachite"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Netherrack"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "srCleaner",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_26",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 12,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Dirty Quartz"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Dirty Emerald"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Dirty Coal"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Dirty Charged Certus Quartz"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(nether_dust(), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Crushed Stone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Diamond"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dirty Diamond"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Black Quartz"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dirty Black Quartz"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Certus Quartz Crystal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dirty Certus Quartz"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lapis Lazuli"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dirty Lapis Lazuli"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aquamarine"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dirty Aquamarine"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sapphire"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dirty Sapphire"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Malachite"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dirty Malachite"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ruby"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dirty Ruby"), 1)],
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
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("White Concrete Powder"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Bone Meal"), 1),
                        BufferedInput::new(Filter::Label("Gravel"), 4),
                        BufferedInput::new(Filter::Label("Sand"), 4),
                    ],
                    max_inputs: 18,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Network Cable"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Gold Plate"), 1),
                        BufferedInput::new(Filter::Label("Lapis Lazuli Plate"), 4),
                        BufferedInput::new(Filter::Label("Redstone"), 4),
                    ],
                    max_inputs: 9,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Leadstone Fluxduct"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Glass"), 1),
                        BufferedInput::new(Filter::Label("Network Cable"), 2),
                        BufferedInput::new(Filter::Label("Redstone"), 6),
                    ],
                    max_inputs: 18,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Hardened Fluxduct"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Leadstone Fluxduct"), 3),
                        BufferedInput::new(Filter::Label("Redstone"), 3),
                        BufferedInput::new(Filter::Label("Invar Ingot"), 1),
                    ],
                    max_inputs: 42,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Plant Matter"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Potato"), 5)],
                    max_inputs: 30,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basic Machine Casing"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Aluminum Plate"), 4),
                        BufferedInput::new(Filter::Label("Dense Iron Plate"), 4),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Reinforced Cell Frame (Empty)"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Silver Gear"), 2),
                        BufferedInput::new(Filter::Label("Flux Crystal"), 2),
                        BufferedInput::new(Filter::Label("Fluxed Electrum Plate"), 2),
                        BufferedInput::new(Filter::Label("Redstone Conductance Coil"), 1),
                        BufferedInput::new(Filter::Label("Hardened Cell Frame"), 1),
                        BufferedInput::new(Filter::Label("Electrum Large Plate"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Signalum Cell Frame (Empty)"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Signalum Plate"), 4),
                        BufferedInput::new(Filter::Label("Rosin"), 2),
                        BufferedInput::new(Filter::Label("Cinnabar"), 1),
                        BufferedInput::new(Filter::Label("Rich Slag"), 1),
                        BufferedInput::new(Filter::Label("Reinforced Cell Frame (Full)"), 1),
                    ],
                    max_inputs: 144,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "freezer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "skyresources:lightfreezertile_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 5)),
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Heavy Snowball"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Snowball"), 4)],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Frozen Iron Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 1)],
                    max_inputs: 64,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "autoCompressor",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "excompressum:auto_compressor_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 12)),
            to_extract: None,
            max_recipe_inputs: 128,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Infinity Dust Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Grains of Infinity"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Compressed Infinity Dust Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Infinity Dust Block"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output {
                        item: Filter::Label("Double Compressed Infinity Dust Block"),
                        n_wanted: 16,
                    }],
                    inputs: vec![BufferedInput::new(Filter::Label("Compressed Infinity Dust Block"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Heavy Snow"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Heavy Snowball"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Blaze Mesh"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Blaze Rod"), 9)],
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
                    outputs: vec![Output { item: Filter::Label("Stable-'Unstable Ingot'"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Stable-'Unstable Nugget'"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Glowstone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Glowstone Dust"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Redstone Essence"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cyanite Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cyanite Ingot"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nether Star Shard"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Star Essence"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Coal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Coal"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Iron"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lead Ingot"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Gold"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Ingot"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Enderium"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Enderium Ingot"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Elementium"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Elementium Ingot"), 9)],
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
                    outputs: vec![Output { item: Filter::Label("Tin Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Tin Ingot"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Mirion Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Mirion Ingot"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Draconium Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Draconium Ingot"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Electrum"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Electrum Ingot"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: black_iron_block(), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Black Iron Ingot"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Invar"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Invar Ingot"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Osmium Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Osmium Ingot"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Osgloglas Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Osgloglas Ingot"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Redstone"), n_wanted: 40 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Redstone"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Diamond"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Diamond"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Block of Emerald"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Emerald"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lapis Lazuli Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lapis Lazuli"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Green Slime Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Rice Slimeball"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bone Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Bone Meal"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Graphite Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Graphite Ingot"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Compressed Cobblestone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cobblestone"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Double Compressed Cobblestone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Compressed Cobblestone"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Triple Compressed Cobblestone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Double Compressed Cobblestone"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Quadruple Compressed Cobblestone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Triple Compressed Cobblestone"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Quintuple Compressed Cobblestone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Quadruple Compressed Cobblestone"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sextuple Compressed Cobblestone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Quintuple Compressed Cobblestone"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Compressed Soul Sand"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Soul Sand"), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Compressed Nether Gravel"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(nether_gravel(), 9)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Compressed Ender Gravel"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Crushed Endstone"), 9)],
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
                    outputs: vec![Output { item: Filter::Label("Gold Ore"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Ore Piece"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cobalt Ore"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Cobalt Ore Piece"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ardite Ore"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Ardite Ore Piece"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lithium Ore"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lithium Ore Piece"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Boron Ore"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Boron Ore Piece"), 4)],
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
                    outputs: vec![Output { item: Filter::Label("Uranium Ore"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Uranium Ore Piece"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dry Rubber"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Tiny Dry Rubber"), 9)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "ethyleneStock",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_38",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 0,
            stocks: vec![BufferedInput::new(Filter::Label("Bio Fuel"), i32::MAX)],
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "draconiumStock",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_35",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 0,
            stocks: vec![BufferedInput::new(Filter::Label("Draconium Ore"), 16)],
            recipes: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "trash",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "xu2:tiletrashchest_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![BufferedRecipe {
                outputs: vec![],
                inputs: vec![BufferedInput::new(Filter::Label("Stone Sword"), 1)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "rubberStock",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_7",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 0,
            stocks: vec![BufferedInput::new(Filter::Label("Birch Wood"), 4)],
            recipes: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "alloyFurnace",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:alloy_furnace_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0, 1],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lithium Manganese Dioxide Alloy"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Manganese Dioxide Dust"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Lithium Dust"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dark Steel Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Steel Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Obsidian"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("End Steel Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Dark Steel Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Dimensional Blend"), 2, vec![1]),
                    ],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Soularium Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Pulverized Gold"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Soul Sand"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Energetic Alloy Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Energetic Blend"), 2, vec![0]),
                        SlottedInput::new(Filter::Label("Pulverized Gold"), 1, vec![1]),
                    ],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Vibrant Alloy Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Energetic Alloy Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Ender Pearl"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Magnesium Diboride Alloy"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Magnesium Dust"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Boron Dust"), 2, vec![1]),
                    ],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Titanium Aluminide Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Aluminum Ingot"), 7, vec![0]),
                        SlottedInput::new(Filter::Label("Titanium Ingot"), 3, vec![1]),
                    ],
                    max_per_slot: 63,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Hard Carbon Alloy"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Graphite Ingot"), 2, vec![0]),
                        SlottedInput::new(Filter::Label("Diamond"), 1, vec![1]),
                    ],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Titanium Iridium Alloy Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Titanium Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Iridium Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electrical Steel Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Steel Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Silicon Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Iron Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Graphite Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bronze Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Copper Ingot"), 3, vec![0]),
                        SlottedInput::new(Filter::Label("Tin Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 48,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Shibuichi Alloy"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Copper Ingot"), 3, vec![0]),
                        SlottedInput::new(Filter::Label("Silver Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 48,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Platinum Alloy"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Lead Ingot"), 3, vec![0]),
                        SlottedInput::new(Filter::Label("Platinum Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 48,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Invar Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Iron Ingot"), 2, vec![0]),
                        SlottedInput::new(Filter::Label("Nickel Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Constantan Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Copper Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Nickel Ingot"), 1, vec![1]),
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
                    outputs: vec![Output { item: Filter::Label("Electrum Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Pulverized Gold"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Silver Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
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
                    outputs: vec![Output { item: Filter::Label("Fused Quartz"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Nether Quartz"), 4, vec![0]),
                        SlottedInput::new(Filter::Label("Block of Quartz"), 1, vec![1]),
                    ],
                    max_per_slot: 64,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Tin Silver Alloy"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Tin Ingot"), 3, vec![0]),
                        SlottedInput::new(Filter::Label("Silver Ingot"), 1, vec![1]),
                    ],
                    max_per_slot: 48,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Alloy Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Silicon Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Redstone"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "diamondSieve",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "excompressum:auto_heavy_sieve_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot < 21)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Gold Ore Piece"), n_wanted: 16 },
                        Output { item: Filter::Label("Boron Ore Piece"), n_wanted: 16 },
                        Output { item: Filter::Label("Magnesium Ore Piece"), n_wanted: 16 },
                        Output { item: Filter::Label("Lithium Ore Piece"), n_wanted: 16 },
                        Output { item: Filter::Label("Cobalt Ore Piece"), n_wanted: 16 },
                        Output { item: Filter::Label("Ardite Ore Piece"), n_wanted: 16 },
                        Output { item: Filter::Label("Thorium Ore Piece"), n_wanted: 16 },
                    ],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed Nether Gravel"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Ghast Tear"), n_wanted: 16 },
                        Output { item: Filter::Label("Nether Quartz"), n_wanted: 16 },
                    ],
                    inputs: vec![SlottedInput::new(Filter::Label("Compressed Soul Sand"), 1, vec![0])],
                    max_per_slot: 8,
                },
            ],
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
                outputs: vec![Output { item: Filter::Label("Uranium Ore Piece"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Compressed Ender Gravel"), 1, vec![0])],
                max_per_slot: 8,
            }],
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
                    outputs: vec![Output { item: Filter::Label("Coal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Graphite Dust"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cubic Boron Nitride"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Hexagonal Boron Nitride"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fluorite"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Crushed Fluorite"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lapis Lazuli Plate"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lapis Lazuli"), 1, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dense Iron Plate"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Iron Plate"), 9, vec![0])],
                    max_per_slot: 63,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dense Steel Plate"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Steel Plate"), 9, vec![0])],
                    max_per_slot: 63,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dense Lapis Lazuli Plate"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lapis Lazuli Plate"), 9, vec![0])],
                    max_per_slot: 63,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dense Copper Plate"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Copper Plate"), 9, vec![0])],
                    max_per_slot: 63,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "melterStock",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_44",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![
                BufferedInput::new(Filter::Label("Peanut"), 16),
                BufferedInput::new(Filter::Label("Glowstone Dust"), 16),
                BufferedInput::new(Filter::Label("Redstone"), 16),
                BufferedInput::new(Filter::Label("Experience Essence"), 16),
                BufferedInput::new(Filter::Label("Ender Pearl"), 16),
                BufferedInput::new(Filter::Label("Cryotheum Dust"), 16),
                BufferedInput::new(Filter::Label("Lumium Ingot"), 16),
                BufferedInput::new(Filter::Label("Sulfur"), 16),
                BufferedInput::new(Filter::Label("Crushed Fluorite"), 16),
                BufferedInput::new(Filter::Label("Boron Ingot"), 16),
            ],
        });
        factory.add_process(BufferedConfig {
            name: "kekimurusStock",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:placer_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![BufferedInput::new(Filter::Label("Cake"), 9)],
        });
        factory.add_process(BufferedConfig {
            name: "phytoStock",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_16",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![BufferedInput::new(Filter::Label("Fluxed Phyto-Gro"), 64)],
        });
        factory.add_process(BufferedConfig {
            name: "starClocheStock",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_12",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![BufferedInput::new(Filter::Label("Fluxed Phyto-Gro"), 4)],
        });
        factory.add_process(BufferedConfig {
            name: "eggClocheStock",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_14",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![BufferedInput::new(Filter::Label("Fluxed Phyto-Gro"), 4)],
        });
        factory.add_process(BufferedConfig {
            name: "infuserStock",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_9",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![
                BufferedInput::new(Filter::Label("Osmium Ingot"), 8),
                BufferedInput::new(Filter::Label("Compressed Obsidian"), 4),
                BufferedInput::new(Filter::Label("Compressed Redstone"), 4),
                BufferedInput::new(Filter::Label("Compressed Diamond"), 4),
            ],
        });
        factory.add_process(SlottedConfig {
            name: "impregnatedStick",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:carpenter_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![12],
            to_extract: Some(Box::new(|slot, _| slot == 10)),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Impregnated Stick"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Birch Wood"), 2, vec![12])],
                max_per_slot: 8,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "enhancedCircuitBoard",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:carpenter_3",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![12, 13],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Enhanced Circuit Board"), n_wanted: 16 }],
                inputs: vec![
                    SlottedInput::new(Filter::Label("Bronze Ingot"), 3, vec![12]),
                    SlottedInput::new(Filter::Label("Redstone"), 6, vec![13]),
                ],
                max_per_slot: 24,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "refinedCircuitBoard",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:carpenter_4",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![12, 13],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Refined Circuit Board"), n_wanted: 16 }],
                inputs: vec![
                    SlottedInput::new(Filter::Label("Iron Ingot"), 3, vec![12]),
                    SlottedInput::new(Filter::Label("Redstone"), 6, vec![13]),
                ],
                max_per_slot: 24,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "basicCircuitBoard",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:carpenter_5",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![12, 13],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Basic Circuit Board"), n_wanted: 16 }],
                inputs: vec![
                    SlottedInput::new(Filter::Label("Tin Ingot"), 1, vec![12]),
                    SlottedInput::new(Filter::Label("Redstone"), 6, vec![13]),
                ],
                max_per_slot: 24,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "intricateCircuitBoard",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:carpenter_6",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![12, 13, 14, 15, 16],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Intricate Circuit Board"), n_wanted: 16 }],
                inputs: vec![
                    SlottedInput::new(Filter::Label("Printed Engineering Circuit"), 4, vec![12]),
                    SlottedInput::new(Filter::Label("Ultimate Control Circuit"), 2, vec![13]),
                    SlottedInput::new(Filter::Label("Basic Circuit Board"), 1, vec![14]),
                    SlottedInput::new(Filter::Label("Enhanced Circuit Board"), 1, vec![15]),
                    SlottedInput::new(Filter::Label("Refined Circuit Board"), 1, vec![16]),
                ],
                max_per_slot: 16,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "hardenedCasing",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:carpenter_2",
                bus_addr: "ic2:iridium_storage_box_0",
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
        factory.add_process(BufferedConfig {
            name: "gearPress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Enderium Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Enderium Ingot"), 4)],
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
                    outputs: vec![Output { item: Filter::Label("Invar Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Invar Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lumium Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Lumium Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silver Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Silver Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electrum Gear"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Electrum Ingot"), 4)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 32,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "wirePress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
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
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "rodPress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_8",
                bus_addr: "ic2:iridium_storage_box_0",
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
            name: "platePress",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_13",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Titanium Aluminide Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Titanium Aluminide Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nickel Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nickel Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Tin Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Tin Ingot"), 1)],
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
                    outputs: vec![Output { item: Filter::Label("Gold Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Ingot"), 1)],
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
                    outputs: vec![Output { item: Filter::Label("Bronze Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Bronze Ingot"), 1)],
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
                    outputs: vec![Output { item: Filter::Label("Electrum Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Electrum Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fluxed Electrum Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Fluxed Electrum Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Signalum Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Signalum Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Invar Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Invar Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Electrum Large Plate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Block of Electrum"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "atomic",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "randomthings:irondropper_4",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Restonia Crystal Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Block of Redstone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Void Crystal Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Block of Coal"), 1)],
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
                    outputs: vec![Output { item: Filter::Label("Diamatine Crystal Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Block of Diamond"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Emeradic Crystal Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Block of Emerald"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Soul Sand"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sand"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ethetic Green Block"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Chiseled Quartz Block"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Rhodochrosite"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Ruby"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 12,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "smallPlatePresser",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:placer_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Stick"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Birch Wood Planks"), 1)],
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
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "manaPool",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:dropper_3",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
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
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Manasteel Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "elvenTrade",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "randomthings:irondropper_6",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Elementium Ingot"), n_wanted: 64 }],
                inputs: vec![BufferedInput::new(Filter::Label("Manasteel Ingot"), 2)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 4,
            stocks: vec![],
        });
        factory.add_process(InputlessConfig {
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:latex_processing_unit_tile_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            outputs: vec![Output { item: Filter::Label("Tiny Dry Rubber"), n_wanted: 64 }],
        });
        factory.add_process(BufferedConfig {
            name: "crafterB",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
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
                    outputs: vec![Output { item: Filter::Label("Sturdy Casing"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Bronze Ingot"), 4),
                        BufferedInput::new(Filter::Label("Bronze Gear"), 2),
                        BufferedInput::new(Filter::Label("Copper Gear"), 2),
                    ],
                    max_inputs: 64,
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
                    outputs: vec![Output { item: Filter::Label("Raw Carbon Fibre"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Coal"), 4)],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crafting Table"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Birch Wood Planks"), 4)],
                    max_inputs: 64,
                },
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
                    outputs: vec![Output { item: Filter::Label("Lever"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Cobblestone"), 1),
                        BufferedInput::new(Filter::Label("Stick"), 1),
                    ],
                    max_inputs: 32,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "compressor",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:compressor_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![6],
            to_extract: None,
            recipes: vec![
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
                    outputs: vec![Output { item: Filter::Label("Energy Crystal"), n_wanted: 4 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Energium Dust"), 9, vec![6])],
                    max_per_slot: 36,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "lightwellStock",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "xu2:tileuse_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Aquamarine"), 1, vec![0])],
                max_per_slot: 4,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "rolling",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:metal_former_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![6],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Item Casing"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Copper Plate"), 1, vec![6])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lead Item Casing"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Lead Plate"), 1, vec![6])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Tin Item Casing"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Tin Plate"), 1, vec![6])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "extruding",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:metal_former_1",
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
            name: "ironElectronTube",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:fabricator_0",
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
        factory.add_process(SlottedConfig {
            name: "enderElectronTube",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:fabricator_4",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0, 3, 4],
            to_extract: extract_all(),
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
            name: "teFrame",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:fabricator_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0, 3, 4, 5, 6, 7, 8],
            to_extract: extract_all(),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: te_frame(), n_wanted: 16 }],
                inputs: vec![
                    SlottedInput::new(Filter::Label("Sand"), 6, vec![0]),
                    SlottedInput::new(rf_frame(), 1, vec![3]),
                    SlottedInput::new(Filter::Label("Enori Crystal"), 4, vec![4]),
                    SlottedInput::new(Filter::Label("Heavy Engineering Block"), 1, vec![5]),
                    SlottedInput::new(Filter::Label("Device Frame"), 1, vec![6]),
                    SlottedInput::new(Filter::Label("Iron Casing"), 1, vec![7]),
                    SlottedInput::new(Filter::Label("Machine Case"), 1, vec![8]),
                ],
                max_per_slot: 48,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "resonantFrame",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "forestry:fabricator_3",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0, 3, 4, 5, 6, 7],
            to_extract: None,
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
                max_per_slot: 32,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "crafterC",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Torch"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                        BufferedInput::new(Filter::Label("Stick"), 1),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Gear"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Redstone Torch"), 4),
                        BufferedInput::new(Filter::Label("Birch Wood Planks"), 1),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Paper"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Rice"), 3)],
                    max_inputs: 18,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Raw Carbon Mesh"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Raw Carbon Fibre"), 2)],
                    max_inputs: 32,
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
                        BufferedInput::new(Filter::Label("Iron Electron Tube"), 4),
                        BufferedInput::new(Filter::Label("Polished Stone"), 4),
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
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterD",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_3",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crafter Tier 3"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Analog Crafter"), 2),
                        BufferedInput::new(Filter::Label("Redstone Gear"), 2),
                        BufferedInput::new(Filter::Label("Crafter Tier 2"), 1),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Rice Dough"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Rice"), 3)],
                    max_inputs: 24,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Itemduct"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Fused Quartz"), 1),
                        BufferedInput::new(Filter::Label("Tin Ingot"), 2),
                    ],
                    max_inputs: 9,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Grout"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(clay_block(), 1),
                        BufferedInput::new(Filter::Label("Sand"), 4),
                        BufferedInput::new(Filter::Label("Gravel"), 4),
                    ],
                    max_inputs: 18,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Enori Crystal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Enori Crystal Block"), 1)],
                    max_inputs: 2,
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
                    outputs: vec![Output { item: Filter::Label("Basic Plating"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Lead Sheetmetal"), 4),
                        BufferedInput::new(Filter::Label("Lead Item Casing"), 4),
                        BufferedInput::new(Filter::Label("Graphite Block"), 1),
                    ],
                    max_inputs: 36,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "waterInfuser",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:infuser_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: clay_block(), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Dirt"), 1, vec![0])],
                    max_per_slot: 4,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Infinite Water Source"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Empty Frame"), 1, vec![0])],
                    max_per_slot: 4,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "creosoteInfuser",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:infuser_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Treated Wood Planks"), n_wanted: 64 }],
                inputs: vec![SlottedInput::new(Filter::Label("Birch Wood Planks"), 1, vec![0])],
                max_per_slot: 8,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "riceSlime",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:fluid_crafter_tile_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![1, 3, 5, 7],
            to_extract: Some(Box::new(|slot, _| slot == 9)),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Rice Slimeball"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Rice Dough"), 4, vec![1, 3, 5, 7])],
                max_per_slot: 8,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "crafterE",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_4",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Networking Cable"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Stone"), 4),
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                    ],
                    max_inputs: 55,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Nugget"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 1)],
                    max_inputs: 2,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Hardened Fluiduct"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Invar Ingot"), 2),
                        BufferedInput::new(Filter::Label("Fused Quartz"), 1),
                    ],
                    max_inputs: 33,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Reinforced Filter"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Electrum Ingot"), 2),
                        BufferedInput::new(Filter::Label("Iron Nugget"), 2),
                        BufferedInput::new(Filter::Label("Paper"), 1),
                        BufferedInput::new(Filter::Label("Glass"), 1),
                    ],
                    max_inputs: 48,
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
                    outputs: vec![Output { item: Filter::Label("Range Addon"), n_wanted: 1 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Cobblestone"), 6),
                        BufferedInput::new(Filter::Label("Glass Pane"), 1),
                        BufferedInput::new(Filter::Label("Plastic"), 2),
                    ],
                    max_inputs: 9,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Mechanical Component"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Steel Plate"), 4),
                        BufferedInput::new(Filter::Label("Copper Ingot"), 1),
                    ],
                    max_inputs: 80,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterF",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_5",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: cable_modem(), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Stone"), 6),
                        BufferedInput::new(Filter::Label("Redstone"), 2),
                        BufferedInput::new(Filter::Label("Networking Cable"), 1),
                    ],
                    max_inputs: 144,
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
                    outputs: vec![Output { item: Filter::Label("Glass Pane"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Glass"), 6)],
                    max_inputs: 6,
                },
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
                    outputs: vec![Output { item: Filter::Label("Steel Sheetmetal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Steel Plate"), 4)],
                    max_inputs: 16,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Mechanical Component"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iron Plate"), 4),
                        BufferedInput::new(Filter::Label("Copper Ingot"), 1),
                    ],
                    max_inputs: 80,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterG",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_6",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iron Bars"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 6)],
                    max_inputs: 6,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: clay_ball(), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(clay_block(), 1)],
                    max_inputs: 4,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Blast Brick"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(nether_brick(), 4),
                        BufferedInput::new(Filter::Label("Brick"), 4),
                        BufferedInput::new(Filter::Label("Blaze Powder"), 1),
                    ],
                    max_inputs: 54,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Reinforced Blast Brick"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Blast Brick"), 1),
                        BufferedInput::new(Filter::Label("Steel Plate"), 1),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Casing"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Steel Mechanical Component"), 2),
                        BufferedInput::new(Filter::Label("Reinforced Blast Brick"), 1),
                        BufferedInput::new(Filter::Label("Osmium Ingot"), 4),
                        BufferedInput::new(Filter::Label("Osmium Block"), 2),
                    ],
                    max_inputs: 72,
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
                    outputs: vec![Output { item: rf_frame(), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Heat Vent"), 2),
                        BufferedInput::new(Filter::Label("Dry Rubber"), 2),
                        BufferedInput::new(Filter::Label("Machine Case"), 1),
                        BufferedInput::new(Filter::Label("Gold Gear"), 1),
                        BufferedInput::new(Filter::Label("Range Addon"), 1),
                        BufferedInput::new(Filter::Label("Pink Slime"), 2),
                    ],
                    max_inputs: 72,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Steel Scaffolding"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Steel Ingot"), 3),
                        BufferedInput::new(Filter::Label("Steel Rod"), 3),
                    ],
                    max_inputs: 18,
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
            to_extract: Some(Box::new(|_, item| Filter::Label("Pink Slime").apply(&item.item, &item.detail))),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Pink Slime"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Green Slime Block"), 1, vec![0])],
                max_per_slot: 2,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "crafterH",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_7",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: mk_energy_upgrade(), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Glass"), 2),
                        BufferedInput::new(Filter::Label("Enriched Alloy"), 2),
                        BufferedInput::new(Filter::Label("Pulverized Gold"), 1),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: mk_speed_upgrade(), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Glass"), 2),
                        BufferedInput::new(Filter::Label("Enriched Alloy"), 2),
                        BufferedInput::new(Filter::Label("Osmium Dust"), 1),
                    ],
                    max_inputs: 80,
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
                    outputs: vec![Output { item: Filter::Label("Heavy Engineering Block"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Uranium Plate"), 4),
                        BufferedInput::new(Filter::Label("Iron Mechanical Component"), 2),
                        BufferedInput::new(Filter::Label("Steel Scaffolding"), 1),
                        BufferedInput::new(Filter::Label("Reinforced Alloy"), 2),
                    ],
                    max_inputs: 36,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Device Frame"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Tin Ingot"), 4),
                        BufferedInput::new(Filter::Label("Glass"), 4),
                        BufferedInput::new(Filter::Label("Copper Gear"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Energy Fluxduct (Empty)"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Hardened Fluxduct"), 2),
                        BufferedInput::new(Filter::Label("Fused Quartz"), 1),
                        BufferedInput::new(Filter::Label("Electrum Plate"), 6),
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
            name: "redstoneTransposer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_transposer_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 2)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Signalum Ingot"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Shibuichi Alloy"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Flux Crystal"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Diamond"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Energy Fluxduct"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Redstone Energy Fluxduct (Empty)"), 1, vec![0])],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fluxed Electrum Blend"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Electrum Blend"), 1, vec![0])],
                    max_per_slot: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "xpTransposer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_transposer_3",
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
                    outputs: vec![Output { item: Filter::Label("Basalz Powder"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Pulverized Obsidian"), 2, vec![0])],
                    max_per_slot: 16,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Blitz Powder"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Niter"), 2, vec![0])],
                    max_per_slot: 16,
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
                outputs: vec![Output { item: Filter::Label("Enderium Ingot"), n_wanted: 64 }],
                inputs: vec![SlottedInput::new(Filter::Label("Lead Platinum Alloy"), 1, vec![0])],
                max_per_slot: 8,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "cryoTransposer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_transposer_5",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 2)),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Cryo-Stabilized Fluxduct"), n_wanted: 64 }],
                inputs: vec![SlottedInput::new(Filter::Label("Cryo-Stabilized Fluxduct (Empty)"), 1, vec![0])],
                max_per_slot: 8,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "lumiumTransposer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_transposer_6",
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
        factory.add_process(SlottedConfig {
            name: "waterTransposer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_transposer_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 2)),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Water Bottle"), n_wanted: 1 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Glass Bottle"), 1, vec![0])],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("White Concrete"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("White Concrete Powder"), 1, vec![0])],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Black Concrete"), n_wanted: 64 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Black Concrete Powder"), 1, vec![0])],
                    max_per_slot: 32,
                },
            ],
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
                max_per_slot: 4,
            }],
        });
        factory.add_process(RedstoneEmitterConfig {
            accesses: vec![RedstoneAccess { client: "1a", addr: Some("redstone_integrator_7"), side: SOUTH }],
            output: emit_when_want_item("cokeFurnace", Filter::Label("Coal Coke"), 16),
        });
        factory.add_process(RedstoneEmitterConfig {
            accesses: vec![RedstoneAccess { client: "1a", addr: Some("redstone_integrator_5"), side: NORTH }],
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
            accesses: vec![RedstoneAccess { client: "1a", addr: Some("redstone_integrator_5"), side: WEST }],
            output: emit_when_want_item("shulker", Filter::Label("Shulker Shell"), 16),
        });
        factory.add_process(RedstoneEmitterConfig {
            accesses: vec![RedstoneAccess { client: "1a", addr: Some("redstone_integrator_9"), side: SOUTH }],
            output: emit_when_want_item("fissionIC2", Filter::Label("Tiny Pile of Plutonium"), 16),
        });
        factory.add_process(RedstoneEmitterConfig {
            accesses: vec![RedstoneAccess { client: "1a", addr: Some("redstone_integrator_10"), side: NORTH }],
            output: emit_when_want_item("fissionBR", Filter::Label("Cyanite Ingot"), 64),
        });
        factory.add_process(BufferedConfig {
            name: "crafterI",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_8",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Button"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Stone"), 1)],
                    max_inputs: 16,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sandstone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sand"), 4)],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Quartz Sliver"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Quartz"), 1)],
                    max_inputs: 2,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Chest"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Button"), 1),
                        BufferedInput::new(Filter::Label("Birch Wood"), 4),
                        BufferedInput::new(Filter::Label("Treated Wood Planks"), 4),
                    ],
                    max_inputs: 72,
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
                    outputs: vec![Output { item: Filter::Label("Rich Phyto-Gro"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Pulverized Charcoal"), 1),
                        BufferedInput::new(Filter::Label("Rich Slag"), 1),
                        BufferedInput::new(Filter::Label("Niter"), 1),
                    ],
                    max_inputs: 3,
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
                        BufferedInput::new(Filter::Label("Soy Milk"), 3),
                        BufferedInput::new(Filter::Label("Sugar"), 2),
                        BufferedInput::new(Filter::Label("Raw Tofeeg"), 1),
                        BufferedInput::new(pam_flour(), 3),
                    ],
                    max_inputs: 9,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterJ",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_9",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Arcane Stone"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Stone"), 8),
                        BufferedInput::new(Filter::Label("Terra Vis Crystal"), 1),
                    ],
                    max_inputs: 18,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Glass Bottle"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Glass"), 3)],
                    max_inputs: 18,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Super Lubricent Tincture"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Seeds"), 1),
                        BufferedInput::new(Filter::Label("Water Bottle"), 1),
                        BufferedInput::new(Filter::Label("Bean"), 1),
                    ],
                    max_inputs: 12,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Super Lubricent Stone"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Stone"), 8),
                        BufferedInput::new(Filter::Label("Super Lubricent Tincture"), 1),
                    ],
                    max_inputs: 72,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: green_slime(), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Green Slime Block"), 1)],
                    max_inputs: 2,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: blue_congealed_slime(), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(blue_slime(), 4)],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Diorite"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Cobblestone"), 2),
                        BufferedInput::new(Filter::Label("Nether Quartz"), 2),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Red Nether Brick"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Nether Wart"), 2),
                        BufferedInput::new(nether_brick(), 2),
                    ],
                    max_inputs: 64,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterK",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_10",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Quartz Slab"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Block of Quartz"), 3)],
                    max_inputs: 9,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ink Sac"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dye Essence"), 3)],
                    max_inputs: 12,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ender Pearl"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Enderman Essence"), 8)],
                    max_inputs: 48,
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
                        BufferedInput::new(Filter::Label("Coal"), 2),
                        BufferedInput::new(Filter::Label("Cobblestone"), 2),
                    ],
                    max_inputs: 16,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cryotheum Dust"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Blizz Powder"), 2),
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                        BufferedInput::new(Filter::Label("Snowball"), 1),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Resonant Servo"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iron Nugget"), 2),
                        BufferedInput::new(Filter::Label("Enderium Ingot"), 2),
                        BufferedInput::new(Filter::Label("Glass"), 1),
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                    ],
                    max_inputs: 48,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "pureDaisy",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_14",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Livingrock"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Arcane Stone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Livingwood"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Infused Wood"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Prosperity Shard"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Livingrock"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "dropInLiquid",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:dropper_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Infused Wood"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Birch Wood"), 1)],
                    max_inputs: 8,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fluix Crystal"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                        BufferedInput::new(Filter::Label("Charged Certus Quartz Crystal"), 1),
                        BufferedInput::new(Filter::Label("Nether Quartz"), 1),
                    ],
                    max_inputs: 24,
                },
            ],
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "combiner",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:device_item_buffer_15",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Ore"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Redstone"), 16)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nickel Ore"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Nickel"), 8)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Platinum Ore"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulverized Platinum"), 8)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 64,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "phyto",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_17",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
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
                    outputs: vec![Output { item: Filter::Label("Pumpkin Seeds"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pumpkin Seeds"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sugar Canes"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sugar Canes"), 1).allow_backup()],
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
                    outputs: vec![Output { item: Filter::Label("Dandelion"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dandelion"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nether Wart"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Wart"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Bean"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Bean Seed"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sulfur Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sulfur Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Terrasteel Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Terrasteel Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Enderman Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Enderman Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Void Metal Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Void Metal Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Knightslime Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Knightslime Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dye Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dye Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nether Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Redstone Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Emerald Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Emerald Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Zombie Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Zombie Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Experience Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Experience Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("§eInferium Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(inferium_seeds(), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Thaumium Essence"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Thaumium Seeds"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
            max_recipe_inputs: 24,
            stocks: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "inductionSmelter",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "thermalexpansion:machine_smelter_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0, 1],
            to_extract: extract_all(),
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Sand"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Aluminum Ore"), 1, vec![1]),
                    ],
                    max_per_slot: i32::MAX,
                },
                SlottedRecipe {
                    outputs: vec![],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Sand"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Mana Infused Ore"), 1, vec![1]),
                    ],
                    max_per_slot: i32::MAX,
                },
                SlottedRecipe {
                    outputs: vec![],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Cinnabar"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Iridium Ore"), 1, vec![1]),
                    ],
                    max_per_slot: i32::MAX,
                },
                SlottedRecipe {
                    outputs: vec![],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Cinnabar"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Platinum Ore"), 1, vec![1]).extra_backup(64),
                    ],
                    max_per_slot: i32::MAX,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Rich Slag"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Clock"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Sand"), 1, vec![1]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Black Iron Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Block of Invar"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Tough Alloy"), 1, vec![1]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Platinum Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Cinnabar"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Nickel Ore"), 1, vec![1]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Iridium Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Cinnabar"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Platinum Ore"), 1, vec![1]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Fluxed Electrum Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Sand"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Fluxed Electrum Blend"), 1, vec![1]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Signalum Cell Frame (Full)"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Signalum Cell Frame (Empty)"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Block of Redstone"), 40, vec![1]),
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
                    outputs: vec![Output { item: Filter::Label("Aluminum Brass Ingot"), n_wanted: 64 }],
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
                inv_addr: "thermalexpansion:machine_charger_1",
                bus_addr: "ic2:iridium_storage_box_0",
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
            name: "pulverizer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_19",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 24,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![],
                    inputs: vec![BufferedInput::new(Filter::Label("Aquamarine Shale"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: nether_gravel(), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Netherrack"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crushed Endstone"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("End Stone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Cinnabar"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Redstone Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulverized Gold"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Ore"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dandelion Yellow"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dandelion"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: pam_flour(), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Soybean"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "fusionCrafting",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_31",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Palis Crystal Block"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Palis Crystal Block"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Dense Lapis Lazuli Plate"), 1, vec![1]),
                        SlottedInput::new(blue_congealed_slime(), 1, vec![2]),
                        SlottedInput::new(Filter::Label("Sapphire"), 1, vec![3]),
                        SlottedInput::new(Filter::Label("Cobalt Ingot"), 1, vec![4]),
                    ],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Enori Crystal Block"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Enori Crystal Block"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Bone Block"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Block of Quartz"), 1, vec![2]),
                        SlottedInput::new(Filter::Label("Osmium Ingot"), 1, vec![3]),
                        SlottedInput::new(Filter::Label("Fluorite"), 1, vec![4]),
                    ],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Emeradic Crystal Block"), n_wanted: 64 }],
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
                    outputs: vec![Output { item: Filter::Label("Empowered Restonia Crystal Block"), n_wanted: 64 }],
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
                    outputs: vec![Output { item: Filter::Label("Empowered Void Crystal Block"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Void Crystal Block"), 1, vec![0]),
                        SlottedInput::new(black_iron_block(), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Ink Sac"), 1, vec![2]),
                        SlottedInput::new(Filter::Label("Basalt"), 1, vec![3]),
                        SlottedInput::new(Filter::Label("Block of Black Quartz"), 1, vec![4]),
                    ],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Diamatine Crystal Block"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Diamatine Crystal Block"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Mana Diamond"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Zirconium Dust"), 1, vec![2]),
                        SlottedInput::new(Filter::Label("Manyullyn Ingot"), 1, vec![3]),
                        SlottedInput::new(Filter::Label("Malachite"), 1, vec![4]),
                    ],
                    max_per_slot: 1,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Hardened Cell Frame"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Energy Cell Frame"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Invar Plate"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Steel Rod"), 1, vec![2]),
                        SlottedInput::new(Filter::Label("Invar Gear"), 1, vec![3]),
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
            ],
        });
        factory.add_process(BufferedConfig {
            name: "centrifuge",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_40",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulsating Propolis"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Mysterious Comb"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silken Tofu"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Soybean"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![
                        Output { item: Filter::Label("Firm Tofu"), n_wanted: 16 },
                        Output { item: Filter::Label("Soy Milk"), n_wanted: 16 },
                    ],
                    inputs: vec![BufferedInput::new(Filter::Label("Silken Tofu"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sugar"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sugar Canes"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "ncCrusher",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:rock_crusher_1",
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
                inv_addr: "rftools:crafter3_11",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
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
                    outputs: vec![Output { item: Filter::Label("Iron Frame"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iron Mechanical Component"), 1),
                        BufferedInput::new(Filter::Label("Iron Rod"), 4),
                        BufferedInput::new(Filter::Label("Iron Bars"), 4),
                    ],
                    max_inputs: 72,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone-Iron Wiring"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Basic Coil"), 3)],
                    max_inputs: 12,
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
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterM",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_12",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
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
                    outputs: vec![Output { item: Filter::Label("Cryo-Stabilized Fluxduct (Empty)"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Resonant Fluxduct"), 1),
                        BufferedInput::new(Filter::Label("Electrum Ingot"), 4),
                        BufferedInput::new(Filter::Label("Fused Quartz"), 4),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Sulfur"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Sulfur Essence"), 3)],
                    max_inputs: 6,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pyrotheum Dust"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Blaze Powder"), 2),
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                        BufferedInput::new(Filter::Label("Sulfur"), 1),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Petrotheum Dust"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Basalz Powder"), 2),
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                        BufferedInput::new(Filter::Label("Pulverized Obsidian"), 1),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aerotheum Dust"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Blitz Powder"), 2),
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                        BufferedInput::new(Filter::Label("Niter"), 1),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Mana Dust"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Mana Diamond"), 4),
                        BufferedInput::new(Filter::Label("Cryotheum Dust"), 1),
                        BufferedInput::new(Filter::Label("Aerotheum Dust"), 1),
                        BufferedInput::new(Filter::Label("Pyrotheum Dust"), 1),
                        BufferedInput::new(Filter::Label("Petrotheum Dust"), 1),
                        BufferedInput::new(Filter::Label("Mana Powder"), 1),
                    ],
                    max_inputs: 18,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Rotten Flesh"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Zombie Essence"), 3)],
                    max_inputs: 6,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterN",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_13",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Enriched Bonemeal"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Rotten Flesh"), 1),
                        BufferedInput::new(Filter::Label("Bone Meal"), 3),
                    ],
                    max_inputs: 16,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Luminessence"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Glowstone Dust"), 2),
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                        BufferedInput::new(Filter::Label("Gunpowder"), 1),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Black Iron Slate"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Black Iron Ingot"), 2)],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basic Component"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Black Iron Slate"), 1),
                        BufferedInput::new(Filter::Label("Luminessence"), 1),
                        BufferedInput::new(Filter::Label("Iron Ingot"), 2),
                    ],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Knightslime Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Knightslime Essence"), 8)],
                    max_inputs: 128,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Advanced Component"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Black Iron Slate"), 1),
                        BufferedInput::new(Filter::Label("Luminessence"), 1),
                        BufferedInput::new(Filter::Label("Gold Ingot"), 2),
                    ],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Elite Component"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Black Iron Slate"), 1),
                        BufferedInput::new(Filter::Label("Luminessence"), 1),
                        BufferedInput::new(Filter::Label("Diamond"), 2),
                    ],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ultimate Component"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Black Iron Slate"), 1),
                        BufferedInput::new(Filter::Label("Luminessence"), 1),
                        BufferedInput::new(Filter::Label("Emerald"), 2),
                    ],
                    max_inputs: 64,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterO",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_14",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basic Catalyst"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Black Iron Ingot"), 1),
                        BufferedInput::new(Filter::Label("Basic Component"), 4),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basic Crafting Table"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Block of Iron"), 1),
                        BufferedInput::new(Filter::Label("Crafting Table"), 2),
                        BufferedInput::new(Filter::Label("Basic Catalyst"), 1),
                        BufferedInput::new(Filter::Label("Basic Component"), 4),
                        BufferedInput::new(Filter::Label("Black Iron Slate"), 1),
                    ],
                    max_inputs: 144,
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
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Palis Crystal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Empowered Palis Crystal Block"), 1)],
                    max_inputs: 2,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Aluminium Sheetmetal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Aluminum Plate"), 4)],
                    max_inputs: 16,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Emerald"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Emerald Essence"), 9)],
                    max_inputs: 144,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "vacuumTube",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:itemviewer_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
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
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
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
        factory.add_process(BufferedConfig {
            name: "crafterP",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_15",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Insulating Glass"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Cactus Green"), 1),
                        BufferedInput::new(Filter::Label("Pulverized Iron"), 2),
                        BufferedInput::new(Filter::Label("Glass"), 2),
                    ],
                    max_inputs: 40,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nether Star"), n_wanted: 40 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Star Shard"), 3)],
                    max_inputs: 120,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: ic2_adv_circuit(), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Glowstone Dust"), 2),
                        BufferedInput::new(Filter::Label("Lapis Lazuli"), 2),
                        BufferedInput::new(Filter::Label("Redstone"), 4),
                        BufferedInput::new(Filter::Label("Basic Control Circuit"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Elite Control Circuit"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(ic2_adv_circuit(), 1),
                        BufferedInput::new(Filter::Label("Reinforced Alloy"), 4),
                        BufferedInput::new(Filter::Label("Redstone"), 4),
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
                    outputs: vec![Output { item: Filter::Label("Salt"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Fresh Water"), 1)],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Certus Quartz Seed"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Sand"), 1),
                        BufferedInput::new(Filter::Label("Certus Quartz Dust"), 1),
                    ],
                    max_inputs: 16,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "brineStock",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:melter_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Salt"), 1, vec![0])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "fusionStock",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:melter_3",
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
        factory.add_process(SlottedConfig {
            name: "neutronium",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:infuser_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Titanium Iridium Alloy Ingot"), 1, vec![0])],
                max_per_slot: 1,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "crafterQ",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_16",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: Some(Box::new(|slot, _| slot >= 26)),
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nether Quartz Seed"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Sand"), 1),
                        BufferedInput::new(Filter::Label("Nether Quartz Dust"), 1),
                    ],
                    max_inputs: 16,
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
                    outputs: vec![Output { item: Filter::Label("Crystaltine Component"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Black Iron Slate"), 1),
                        BufferedInput::new(Filter::Label("Luminessence"), 1),
                        BufferedInput::new(Filter::Label("Crystaltine Ingot"), 2),
                    ],
                    max_inputs: 64,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Nether Star Nugget"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Star"), 1)],
                    max_inputs: 2,
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
                    outputs: vec![Output { item: Filter::Label("Empowered Diamatine Crystal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Empowered Diamatine Crystal Block"), 1)],
                    max_inputs: 2,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ender Casing"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Ender Pearl"), 4),
                        BufferedInput::new(Filter::Label("Empowered Diamatine Crystal"), 4),
                        BufferedInput::new(Filter::Label("Block of Black Quartz"), 1),
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
            ],
        });
        factory.add_process(BufferedConfig {
            name: "inscriber",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_34",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
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
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
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
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Osgloglas Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Refined Obsidian Ingot"), 1),
                        BufferedInput::new(Filter::Label("Osmium Ingot"), 1),
                        BufferedInput::new(Filter::Label("Glowstone Ingot"), 1),
                    ],
                    max_inputs: 24,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Titanium Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Magnesium Ore"), 2),
                        BufferedInput::new(Filter::Label("Salt"), 4),
                        BufferedInput::new(Filter::Label("Carbon Plate"), 1),
                    ],
                    max_inputs: 56,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Osmiridium Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iridium Ingot"), 1),
                        BufferedInput::new(Filter::Label("Osmium Ingot"), 1),
                    ],
                    max_inputs: 16,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Mana Infused Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Manasteel Ingot"), 4),
                        BufferedInput::new(Filter::Label("Diamond"), 1),
                    ],
                    max_inputs: 40,
                },
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
                    outputs: vec![Output { item: Filter::Label("Modularium Alloy"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Empowered Palis Crystal"), 1),
                        BufferedInput::new(Filter::Label("Electrical Steel Ingot"), 2),
                        BufferedInput::new(Filter::Label("Platinum Ingot"), 1),
                    ],
                    max_inputs: 32,
                },
            ],
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "crystaltine",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_28",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Crystaltine Ingot"), n_wanted: 64 }],
                inputs: vec![
                    BufferedInput::new(Filter::Label("Nether Star Nugget"), 4),
                    BufferedInput::new(Filter::Label("Lapis Lazuli"), 10),
                    BufferedInput::new(Filter::Label("Diamond"), 8),
                    BufferedInput::new(Filter::Label("Iron Ingot"), 4),
                    BufferedInput::new(Filter::Label("Gold Ingot"), 2),
                ],
                max_inputs: 28,
            }],
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "advancedCraftingTable",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_29",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
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
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "eliteCraftingTable",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_30",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
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
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "enderStar",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_46",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Ender Star"), n_wanted: 16 }],
                inputs: vec![
                    BufferedInput::new(Filter::Label("Nether Star"), 1),
                    BufferedInput::new(Filter::Label("Eye of Ender"), 4),
                ],
                max_inputs: 5,
            }],
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "enhancedEnderIngot",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_47",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Enhanced Ender Ingot"), n_wanted: 64 }],
                inputs: vec![
                    BufferedInput::new(Filter::Label("Ender Star"), 1),
                    BufferedInput::new(Filter::Label("Ender Ingot"), 4),
                ],
                max_inputs: 5,
            }],
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "isotopeSeparator",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:isotope_separator_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![
                    Output { item: Filter::Label("Tiny Clump of Uranium-235"), n_wanted: 16 },
                    Output { item: Filter::Label("Uranium-238"), n_wanted: 16 },
                ],
                inputs: vec![SlottedInput::new(Filter::Label("Uranium Ingot"), 1, vec![0])],
                max_per_slot: 8,
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
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Lunar Reactive Dust"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Lapis Lazuli"), 1, vec![0])],
                max_per_slot: 4,
            }],
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
            name: "fuelReprocessor",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:fuel_reprocessor_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("Depleted Uranium Nuclear Fuel"), 1, vec![0])],
                max_per_slot: i32::MAX,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "uraniumMelter",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "industrialforegoing:item_splitter_tile_9",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![],
                inputs: vec![BufferedInput::new(Filter::Label("Uranium 238"), 1)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "crafterR",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_17",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("DU Plating"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Sulfur"), 4),
                        BufferedInput::new(Filter::Label("Uranium-238"), 4),
                        BufferedInput::new(Filter::Label("Advanced Plating"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Gold Nugget"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Gold Ingot"), 1)],
                    max_inputs: 2,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Copper Solenoid"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Copper Plate"), 4),
                        BufferedInput::new(Filter::Label("Copper Item Casing"), 2),
                        BufferedInput::new(Filter::Label("Aluminium Rod"), 2),
                        BufferedInput::new(Filter::Label("Mixed Metal Ingot"), 1),
                    ],
                    max_inputs: 36,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crystal Binder"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Crushed Rhodochrosite"), 1),
                        BufferedInput::new(Filter::Label("Calcium Sulfate"), 1),
                        BufferedInput::new(Filter::Label("Pulverized Obsidian"), 1),
                        BufferedInput::new(Filter::Label("Magnesium Dust"), 1),
                    ],
                    max_inputs: 32,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Elite Plating"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Crystal Binder"), 4),
                        BufferedInput::new(Filter::Label("Boron Ingot"), 4),
                        BufferedInput::new(Filter::Label("DU Plating"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Terrasteel Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Terrasteel Essence"), 8)],
                    max_inputs: 128,
                },
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
                        BufferedInput::new(Filter::Label("Tin Block"), 4),
                    ],
                    max_inputs: 144,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterS",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_18",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Interconnect"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Black Concrete"), 1),
                        BufferedInput::new(Filter::Label("Connector"), 4),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Genetics Processor"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Printed Engineering Circuit"), 4),
                        BufferedInput::new(Filter::Label("Pure Nether Quartz Crystal"), 2),
                        BufferedInput::new(ic2_adv_circuit(), 2),
                        BufferedInput::new(Filter::Label("Ender Pearl"), 1),
                    ],
                    max_inputs: 72,
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
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Blutonium Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Blutonium Block"), 1)],
                    max_inputs: 8,
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
                    outputs: vec![Output { item: Filter::Label("Reactor Frame"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Atomic Alloy"), 1),
                        BufferedInput::new(Filter::Label("Steel Casing"), 4),
                    ],
                    max_inputs: 20,
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
                    outputs: vec![Output { item: Filter::Label("Stable-'Unstable Nugget'"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iron Nugget"), 1),
                        BufferedInput::new(Filter::Label("Stick"), 1),
                        BufferedInput::new(Filter::Label("Diamond"), 1),
                    ],
                    max_inputs: 48,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterT",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_19",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Lapotron Crystal"), n_wanted: 4 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Lapis Lazuli Dust"), 6),
                        BufferedInput::new(ic2_adv_circuit(), 2),
                        BufferedInput::new(Filter::Label("Energy Crystal"), 1),
                    ],
                    max_inputs: 9,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Moon Stone"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Lunar Reactive Dust"), 8),
                        BufferedInput::new(Filter::Label("Stable-'Unstable Ingot'"), 1),
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
                    outputs: vec![Output { item: Filter::Label("Netherrack"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Nether Essence"), 5)],
                    max_inputs: 10,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Pulsating Mesh"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Pulsating Propolis"), 5)],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Infused Diamond"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Diamond"), 1),
                        BufferedInput::new(Filter::Label("Dimensional Shard"), 8),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Redstone Alloy Grinding Ball"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Redstone Alloy Ingot"), 5)],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crystaltine Catalyst"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Black Iron Ingot"), 1),
                        BufferedInput::new(Filter::Label("Crystaltine Component"), 4),
                    ],
                    max_inputs: 80,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crystallizer",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:vulpesinputhatch_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Silicon Boule"), n_wanted: 16 }],
                inputs: vec![BufferedInput::new(Filter::Label("Silicon Ingot"), 1)],
                max_inputs: 8,
            }],
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "cuttingMachine",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:vulpesinputhatch_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Silicon Wafer"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Silicon Boule"), 1)],
                    max_inputs: 8,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: ar_adv_circuit(), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Advanced Circuit Plate"), 1)],
                    max_inputs: 8,
                },
            ],
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "precisionAssembler",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:vulpesinputhatch_2",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Advanced Circuit Plate"), n_wanted: 16 }],
                inputs: vec![
                    BufferedInput::new(Filter::Label("Intricate Circuit Board"), 1),
                    BufferedInput::new(Filter::Label("Redstone Alloy Grinding Ball"), 1),
                    BufferedInput::new(Filter::Label("Silicon Wafer"), 1),
                ],
                max_inputs: 24,
            }],
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "alumite",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_41",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Alumite Ingot"), n_wanted: 64 }],
                inputs: vec![
                    BufferedInput::new(Filter::Label("Aluminum Ingot"), 5),
                    BufferedInput::new(Filter::Label("Iron Ingot"), 2),
                    BufferedInput::new(Filter::Label("Obsidian"), 2),
                ],
                max_inputs: 9,
            }],
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "pigiron",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_48",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Pigiron Ingot"), n_wanted: 64 }],
                inputs: vec![
                    BufferedInput::new(clay_ball(), 1),
                    BufferedInput::new(Filter::Label("Iron Ingot"), 2),
                    BufferedInput::new(Filter::Label("Rotten Flesh"), 2),
                ],
                max_inputs: 10,
            }],
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "crafterU",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_20",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Enori Crystal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Empowered Enori Crystal Block"), 1)],
                    max_inputs: 2,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Void Crystal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Empowered Void Crystal Block"), 1)],
                    max_inputs: 2,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Restonia Crystal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Empowered Restonia Crystal Block"), 1)],
                    max_inputs: 2,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Empowered Emeradic Crystal"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Empowered Emeradic Crystal Block"), 1)],
                    max_inputs: 2,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Amethyst"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Elementium Ingot"), 8),
                        BufferedInput::new(Filter::Label("Terrestrial Artifact"), 1),
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
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterV",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_21",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Genetics Labware"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Glass Pane"), 4),
                        BufferedInput::new(Filter::Label("Diamond"), 1),
                    ],
                    max_inputs: 5,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ludicrite Ingot"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Ludicrite Block"), 1)],
                    max_inputs: 2,
                },
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
                    outputs: vec![Output { item: Filter::Label("Neutron Reflector"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Pulverized Coal"), 4),
                        BufferedInput::new(Filter::Label("Pulverized Tin"), 4),
                        BufferedInput::new(Filter::Label("Copper Plate"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Energetic Blend"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Redstone"), 1),
                        BufferedInput::new(Filter::Label("Glowstone Dust"), 1),
                    ],
                    max_inputs: 16,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Carbon Brick"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Charcoal"), 6)],
                    max_inputs: 384,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: crystaltine_trimmed(), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Crystaltine Nugget"), 4),
                        BufferedInput::new(black_iron_block(), 1),
                    ],
                    max_inputs: 80,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Crystaltine Nugget"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Crystaltine Ingot"), 1)],
                    max_inputs: 2,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "advCarpenterStock",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_42",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            max_recipe_inputs: 0,
            recipes: vec![],
            stocks: vec![
                BufferedInput::new(Filter::Label("Lapis Lazuli Dust"), 32),
                BufferedInput::new(Filter::Label("Osgloglas Ingot"), 16),
                BufferedInput::new(Filter::Label("Mirion Ingot"), 16),
                BufferedInput::new(Filter::Label("Genetics Labware"), 4),
                BufferedInput::new(Filter::Label("Ender Drone"), 4),
            ],
        });
        factory.add_process(BufferedConfig {
            name: "ludicrite",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "minecraft:tileiteminputbus_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Ludicrite Block"), n_wanted: 16 }],
                inputs: vec![
                    BufferedInput::new(Filter::Label("Amethyst"), 1),
                    BufferedInput::new(Filter::Label("Blutonium Block"), 1),
                    BufferedInput::new(Filter::Label("Alumite Ingot"), 1),
                    BufferedInput::new(Filter::Label("Blaze Mesh"), 1),
                    BufferedInput::new(Filter::Label("Block of Elementium"), 1),
                    BufferedInput::new(Filter::Label("Block of Enderium"), 2),
                ],
                max_inputs: 56,
            }],
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
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
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "crafterW",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_22",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
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
                    outputs: vec![Output { item: Filter::Label("Thick Neutron Reflector"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Neutron Reflector"), 4),
                        BufferedInput::new(Filter::Label("Copper Plate"), 5),
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
                    outputs: vec![Output { item: Filter::Label("Hardened Coal Block"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Block of Coal"), 8),
                        BufferedInput::new(Filter::Label("Coal"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("End Steel Grinding Ball"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("End Steel Ingot"), 5)],
                    max_inputs: 5,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dark Iron Bars"), n_wanted: 16 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Dark Steel Ingot"), 6)],
                    max_inputs: 6,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Dimensional Blend"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Pulverized Obsidian"), 4),
                        BufferedInput::new(Filter::Label("Crushed End Stone"), 1),
                    ],
                    max_inputs: 32,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "ultimateCraftingTable",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_43",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Ultimate Crafting Table"), n_wanted: 16 }],
                inputs: vec![
                    BufferedInput::new(Filter::Label("Signalum Cell Frame (Full)"), 4),
                    BufferedInput::new(crystaltine_trimmed(), 8),
                    BufferedInput::new(Filter::Label("Draconium Crystal"), 8),
                    BufferedInput::new(ar_adv_circuit(), 4),
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
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "sagMillStock",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "enderio:tile_sag_mill_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![1],
            to_extract: Some(Box::new(|slot, _| slot >= 2)),
            recipes: vec![SlottedRecipe {
                outputs: vec![],
                inputs: vec![SlottedInput::new(Filter::Label("End Steel Grinding Ball"), 1, vec![1])],
                max_per_slot: 64,
            }],
        });
        factory.add_process(SlottedConfig {
            name: "sagMill",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "enderio:tile_sag_mill_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![],
                    inputs: vec![SlottedInput::new(Filter::Label("Infinity Dust Block"), 1, vec![0]).extra_backup(64)],
                    max_per_slot: i32::MAX,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Grains of Infinity"), n_wanted: 16 }],
                    inputs: vec![SlottedInput::new(Filter::Label("Double Compressed Infinity Dust Block"), 1, vec![0])],
                    max_per_slot: 16,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterX",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_23",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
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
                        BufferedInput::new(Filter::Label("Electrum Ingot"), 2),
                        BufferedInput::new(Filter::Label("Iron Ingot"), 3),
                        BufferedInput::new(Filter::Label("LV Capacitor"), 1),
                        BufferedInput::new(Filter::Label("Block of Redstone"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("HV Capacitor"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Treated Wood Planks"), 2),
                        BufferedInput::new(Filter::Label("Lead Block"), 2),
                        BufferedInput::new(Filter::Label("Steel Ingot"), 3),
                        BufferedInput::new(Filter::Label("MV Capacitor"), 1),
                        BufferedInput::new(Filter::Label("Block of Redstone"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Basic Capacitor"), n_wanted: 16 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Grains of Infinity"), 2),
                        BufferedInput::new(Filter::Label("Redstone Transmission Coil"), 4),
                        BufferedInput::new(Filter::Label("HV Capacitor"), 1),
                    ],
                    max_inputs: 56,
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
            ],
        });
        factory.add_process(SlottedConfig {
            name: "alloySmelter",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "enderio:tile_alloy_smelter_0",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0, 1, 2],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Organic Green Dye"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Cactus Green"), 2, vec![0]),
                        SlottedInput::new(Filter::Label("Rice Slimeball"), 1, vec![1]),
                        SlottedInput::new(Filter::Label("Pulverized Charcoal"), 2, vec![2]),
                    ],
                    max_per_slot: 32,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Organic Black Dye"), n_wanted: 16 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Rice Slimeball"), 1, vec![0]),
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
            ],
        });
        factory.add_process(BufferedConfig {
            name: "crafterY",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_25",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
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
                    outputs: vec![Output { item: Filter::Label("Ebony Psimetal Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Ebony Substance"), 8),
                        BufferedInput::new(Filter::Label("Psimetal Ingot"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ivory Psimetal Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Ivory Substance"), 8),
                        BufferedInput::new(Filter::Label("Psimetal Ingot"), 1),
                    ],
                    max_inputs: 144,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Thaumium Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Thaumium Essence"), 8)],
                    max_inputs: 128,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Base Essence Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iron Ingot"), 1),
                        BufferedInput::new(Filter::Label("Prosperity Shard"), 4),
                    ],
                    max_inputs: 320,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("§eInferium Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Base Essence Ingot"), 1),
                        BufferedInput::new(Filter::Label("§eInferium Essence"), 4),
                    ],
                    max_inputs: 320,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("§aPrudentium Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("§eInferium Ingot"), 1),
                        BufferedInput::new(Filter::Label("§aPrudentium Essence"), 4),
                    ],
                    max_inputs: 320,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("§6Intermedium Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("§aPrudentium Ingot"), 1),
                        BufferedInput::new(Filter::Label("§6Intermedium Essence"), 4),
                    ],
                    max_inputs: 320,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "conjuration",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "randomthings:irondropper_7",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Psimetal Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Psimetal Ingot"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Psidust"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Psidust"), 1).allow_backup()],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Psigem"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Psigem"), 1).allow_backup()],
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
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: "crafterZ",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "rftools:crafter3_26",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: Some(Box::new(|slot| slot < 26)),
            to_extract: None,
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
            recipes: vec![
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("§bSuperium Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("§6Intermedium Ingot"), 1),
                        BufferedInput::new(Filter::Label("§bSuperium Essence"), 4),
                    ],
                    max_inputs: 320,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("§cSupremium Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("§bSuperium Ingot"), 1),
                        BufferedInput::new(Filter::Label("§cSupremium Essence"), 4),
                    ],
                    max_inputs: 320,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("§5Insanium Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("§cSupremium Ingot"), 1),
                        BufferedInput::new(Filter::Label("§5Insanium Essence"), 4),
                    ],
                    max_inputs: 320,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Ender Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        BufferedInput::new(Filter::Label("Iron Ingot"), 1),
                        BufferedInput::new(Filter::Label("Ender Pearl"), 1),
                    ],
                    max_inputs: 128,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Invar Shears"), n_wanted: 1 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Invar Ingot"), 2)],
                    max_inputs: 2,
                },
                BufferedRecipe {
                    outputs: vec![Output { item: Filter::Label("Void Metal Ingot"), n_wanted: 64 }],
                    inputs: vec![BufferedInput::new(Filter::Label("Void Metal Essence"), 8)],
                    max_inputs: 168,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: "lavaWell",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "actuallyadditions:dropper_6",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Demon Ingot"), n_wanted: 64 }],
                inputs: vec![BufferedInput::new(Filter::Label("Gold Ingot"), 1)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(SlottedConfig {
            name: "enchanter",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "xu2:tilemachinereceiver_1",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0, 1],
            to_extract: None,
            recipes: vec![
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Enchanted Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Gold Ingot"), 1, vec![0]),
                        SlottedInput::new(Filter::Label("Lapis Lazuli"), 1, vec![1]),
                    ],
                    max_per_slot: 8,
                },
                SlottedRecipe {
                    outputs: vec![Output { item: Filter::Label("Evil Infused Iron Ingot"), n_wanted: 64 }],
                    inputs: vec![
                        SlottedInput::new(Filter::Label("Iron Ingot"), 8, vec![0]),
                        SlottedInput::new(Filter::Label("Nether Star"), 1, vec![1]),
                    ],
                    max_per_slot: 16,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: "oxygenInfuser",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "nuclearcraft:infuser_3",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            input_slots: vec![0],
            to_extract: Some(Box::new(|slot, _| slot == 1)),
            recipes: vec![SlottedRecipe {
                outputs: vec![Output { item: Filter::Label("Manganese Dioxide Dust"), n_wanted: 16 }],
                inputs: vec![SlottedInput::new(Filter::Label("Manganese Oxide Dust"), 1, vec![0])],
                max_per_slot: 8,
            }],
        });
        factory.add_process(BufferedConfig {
            name: "thaumatorium",
            accesses: vec![BusAccess {
                client: "1a",
                inv_addr: "ic2:wooden_storage_box_45",
                bus_addr: "ic2:iridium_storage_box_0",
            }],
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: vec![Output { item: Filter::Label("Alchemical Brass Ingot"), n_wanted: 64 }],
                inputs: vec![BufferedInput::new(Filter::Label("Iron Ingot"), 1)],
                max_inputs: 8,
            }],
            max_recipe_inputs: i32::MAX,
            stocks: vec![
                BufferedInput::new(Filter::Label("Charcoal"), 4),
                BufferedInput::new(Filter::Label("Invar Shears"), 1),
            ],
        });
    })
}
