use crate::factory::{Factory, FactoryConfig, FluidStorageConfig};
use crate::{access::*, config_util::*, process::*, recipe::*, storage::*};
use crate::{detail_cache::DetailCache, server::Server, Tui};
use flexstr::{local_fmt, LocalStr};
use std::{cell::RefCell, rc::Rc, time::Duration};

const MAIN: &str = "main";
const BUS: &str = "minecraft:barrel_0";
fn fbus() -> Vec<LocalStr> { vec![s("create:fluid_tank_0")] }
fn acc(inv_addr: LocalStr) -> Vec<BusAccess> { vec![BusAccess { client: s(MAIN), inv_addr, bus_addr: s(BUS) }] }
fn tank(tank_addr: LocalStr) -> Vec<TankAccess> { vec![TankAccess { client: s(MAIN), tank_addr, fluid_bus_addrs: fbus() }] }
fn inv_tank(addr: LocalStr) -> Vec<InvTankAccess> {
    vec![InvTankAccess { client: s(MAIN), inv_addrs: vec![addr.clone()], tank_addrs: vec![addr], bus_addr: s(BUS), fluid_bus_addrs: fbus() }]
}

pub fn build_factory(tui: Rc<Tui>) -> Rc<RefCell<Factory>> {
    FactoryConfig {
        tui: tui.clone(),
        detail_cache: DetailCache::new(&tui, s("detail_cache.txt")),
        server: Server::new(tui, 1850),
        min_cycle_time: Duration::from_secs(1),
        log_clients: vec![s(MAIN)],
        bus_accesses: vec![BasicAccess { client: s(MAIN), addr: s(BUS) }],
        fluid_bus_accesses: vec![FluidAccess { client: s(MAIN), fluid_bus_addrs: fbus() }],
        fluid_bus_capacity: 8_000,
        backups: vec![
            (label("Certus Quartz Crystal"), 16),
            (label("Sky Stone"), 16),
            (name("kubejs:earth_slimy_fern_leaf"), 16),
            (name("kubejs:ender_slimy_fern_leaf"), 16),
            (name("kubejs:sky_slimy_fern_leaf"), 16),
        ],
        fluid_backups: vec![],
    }
    .build(|factory| {
        factory.add_process(ManualUiConfig { accesses: acc(s("minecraft:barrel_13")) });
        factory.add_process(LowAlert::new(label("Copper Ingot"), 32));
        for i in [0, 1, 2, 3] {
            factory.add_storage(ChestConfig {
                accesses: acc(local_fmt!("sophisticatedstorage:chest_{i}")),
                override_max_stack_size: Some(Box::new(|x| x * 4)),
            });
        }
        for (capacity, addr, fluid) in [
            (24_000, "create:fluid_tank_1", "thermal:resin"),
            (24_000, "create:fluid_tank_2", "minecraft:water"),
            (24_000, "create:fluid_tank_3", "kubejs:volatile_sky_solution"),
            (24_000, "create:fluid_tank_6", "thermal:redstone"),
        ] {
            factory.add_fluid_storage(FluidStorageConfig { accesses: tank(s(addr)), fluid: s(fluid), capacity });
        }
        for (addr, items) in [
            ("functionalstorage:oak_4_0", &["Wheat Seeds", "Flax Seeds", "Cactus", "Poisonous Potato"][..]),
            ("functionalstorage:oak_1_2", &["Andesite"]),
            ("functionalstorage:oak_1_3", &["Cobblestone"]),
            ("functionalstorage:oak_1_4", &["Diorite"]),
            ("functionalstorage:oak_1_5", &["Granite"]),
            ("functionalstorage:oak_1_6", &["Wheat"]),
            ("functionalstorage:oak_1_7", &["Flax"]),
            ("functionalstorage:oak_1_8", &["Potato"]),
            ("functionalstorage:oak_1_9", &["Stick"]),
            ("functionalstorage:oak_1_10", &["Oak Sapling"]),
            ("functionalstorage:oak_1_11", &["Oak Log"]),
            ("functionalstorage:oak_1_12", &["Carrot"]),
            ("functionalstorage:oak_1_13", &["Kelp"]),
        ] {
            let filters = items.iter().copied().map(label).collect();
            factory.add_storage(DrawerConfig { accesses: acc(s(addr)), filters });
        }
        for addr in [
            "minecraft:barrel_3",  // cobbleGen
            "minecraft:barrel_9",  // mainFarm
            "minecraft:barrel_4",  // lavaFan
            "minecraft:barrel_10", // kelpFarm
            "create:basin_4",      // saw
            "create:basin_11",     // waterSpout
        ] {
            factory.add_process(SlottedConfig {
                name: s("output"),
                accesses: acc(s(addr)),
                input_slots: vec![],
                to_extract: extract_all(),
                recipes: vec![],
                strict_priority: false,
            });
        }
        for addr in [
            "thermal:device_tree_extractor_2", // resin
            "thermal:device_tree_extractor_3", // resin
            "thermal:device_tree_extractor_4", // resin
            "thermal:device_tree_extractor_5", // resin
        ] {
            factory.add_process(FluidSlottedConfig {
                name: s("output"),
                input_slots: vec![],
                input_tanks: vec![vec![]],
                accesses: inv_tank(s(addr)),
                to_extract: None,
                fluid_extract: fluid_extract_all(),
                recipes: vec![],
                strict_priority: false,
            });
        }
        for addr in [
            "create:basin_2",  // mixer
            "create:basin_8",  // mixer
            "create:basin_9",  // mixer
            "create:basin_10", // mixer
        ] {
            factory.add_process(FluidSlottedConfig {
                name: s("output"),
                input_slots: vec![vec![]],
                input_tanks: vec![vec![]],
                accesses: inv_tank(s(addr)),
                to_extract: multi_inv_extract_all(),
                fluid_extract: fluid_extract_all(),
                recipes: vec![],
                strict_priority: false,
            });
        }
        for (addr, item, qty) in [
            ("create:basin_0", "Charcoal", 64),            // boiler
            ("create:deployer_0", "Andesite Alloy", 64),   // kineticLine
            ("create:deployer_1", "Andesite Alloy", 64),   // kineticLine
            ("thermal:dynamo_stirling_2", "Charcoal", 64), // charger
            ("minecraft:hopper_5", "Iron Ingot", 64),      // ironSpout
            ("create:deployer_2", "Electron Tube", 64),    // precisionLine
            ("create:deployer_3", "Electron Tube", 64),    // precisionLine
        ] {
            factory.add_process(BufferedConfig {
                name: s("stock"),
                accesses: acc(s(addr)),
                slot_filter: None,
                to_extract: None,
                recipes: vec![],
                max_recipe_inputs: 0,
                stocks: vec![BufferedInput::new(label(item), qty)],
            });
        }
        factory.add_process(BufferedConfig {
            name: s("lavaFan"),
            accesses: acc(s("minecraft:barrel_5")),
            slot_filter: None,
            to_extract: None,
            recipes: [
                (32, "Glass", "Sand"),
                (32, "Charcoal", "Oak Log"),
                (32, "Stone", "Cobblestone"),
                (32, "Cured Rubber", "Rubber"),
                (32, "Sky Stone", "Sky Stone Dust"),
                (32, "Algal Brick", "Algal Blend"),
            ]
            .into_iter()
            .map(|(qty, o, i)| BufferedRecipe { outputs: Output::new(label(o), qty), inputs: vec![BufferedInput::new(label(i), 1)], max_inputs: 16 })
            .collect(),
            max_recipe_inputs: i32::MAX,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: s("saw"),
            accesses: acc(s("minecraft:hopper_2")),
            slot_filter: None,
            to_extract: None,
            recipes: [(32, "Stripped Oak Log", "Oak Log"), (32, "Oak Planks", "Stripped Oak Log"), (32, "Oak Slab", "Oak Planks")]
                .into_iter()
                .map(|(qty, o, i)| BufferedRecipe {
                    outputs: Output::new(label(o), qty),
                    inputs: vec![BufferedInput::new(label(i), 1)],
                    max_inputs: i32::MAX,
                })
                .collect(),
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: s("knife"),
            accesses: acc(s("minecraft:hopper_9")),
            slot_filter: None,
            to_extract: None,
            recipes: [
                (32, "kubejs:earth_slimy_fern_leaf", "tconstruct:earth_slime_fern"),
                (32, "kubejs:ender_slimy_fern_leaf", "tconstruct:ender_slime_fern"),
                (32, "kubejs:sky_slimy_fern_leaf", "tconstruct:sky_slime_fern"),
            ]
            .into_iter()
            .map(|(qty, o, i)| BufferedRecipe {
                outputs: Output::new(name(o), qty),
                inputs: vec![BufferedInput::new(name(i), 1)],
                max_inputs: i32::MAX,
            })
            .collect(),
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: s("spiritFire"),
            accesses: acc(s("minecraft:dropper_0")),
            slot_filter: None,
            to_extract: None,
            recipes: [
                (32, "tconstruct:earth_slime_fern", "kubejs:earth_slimy_fern_leaf"),
                (32, "tconstruct:ender_slime_fern", "kubejs:ender_slimy_fern_leaf"),
                (32, "tconstruct:sky_slime_fern", "kubejs:sky_slimy_fern_leaf"),
            ]
            .into_iter()
            .map(|(qty, o, i)| BufferedRecipe {
                outputs: Output::new(name(o), qty),
                inputs: vec![BufferedInput::new(name(i), 1).allow_backup()],
                max_inputs: i32::MAX,
            })
            .collect(),
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: s("kineticLine"),
            accesses: acc(s("minecraft:hopper_3")),
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: Output::new(name("kubejs:kinetic_mechanism"), 128),
                inputs: vec![BufferedInput::new(label("Oak Slab"), 1)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: s("precisionLine"),
            accesses: acc(s("minecraft:hopper_6")),
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: Output::new(label("Precision Mechanism"), 128),
                inputs: vec![BufferedInput::new(name("kubejs:kinetic_mechanism"), 1)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 8,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: s("crushingWheels"),
            accesses: acc(s("minecraft:barrel_17")),
            slot_filter: None,
            to_extract: None,
            recipes: [
                (32, label("Gravel"), BufferedInput::new(label("Cobblestone"), 1)),
                (32, label("Sand"), BufferedInput::new(label("Gravel"), 1)),
                (32, label("Sky Stone Dust"), BufferedInput::new(label("Sky Stone"), 1).allow_backup()),
                (32, label("Certus Quartz Dust"), BufferedInput::new(label("Certus Quartz Crystal"), 1).allow_backup()),
                (32, name("kubejs:earth_slimy_fern_paste"), BufferedInput::new(name("kubejs:earth_slimy_fern_leaf"), 1)),
                (32, name("kubejs:ender_slimy_fern_paste"), BufferedInput::new(name("kubejs:ender_slimy_fern_leaf"), 1)),
                (32, name("kubejs:sky_slimy_fern_paste"), BufferedInput::new(name("kubejs:sky_slimy_fern_leaf"), 1)),
            ]
            .into_iter()
            .map(|(qty, o, i)| BufferedRecipe { outputs: Output::new(o, qty), inputs: vec![i], max_inputs: i32::MAX })
            .collect(),
            max_recipe_inputs: 64,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: s("stick"),
            accesses: acc(s("minecraft:hopper_7")),
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: Output::new(label("Stick"), 32),
                inputs: vec![BufferedInput::new(label("Oak Planks"), 2)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 16,
            stocks: vec![],
        });
        factory.add_process(MultiInvSlottedConfig {
            name: s("singularity"),
            input_slots: vec![vec![0], vec![0]],
            accesses: vec![MultiInvAccess { client: s(MAIN), inv_addrs: vec![s("create:depot_4"), s("create:depot_5")], bus_addr: s(BUS) }],
            to_extract: None,
            recipes: vec![MultiInvSlottedRecipe {
                outputs: Output::new(label("Singularity"), 32),
                inputs: vec![
                    MultiInvSlottedInput::new(label("Cobblestone"), vec![(0, 0, 16)]),
                    MultiInvSlottedInput::new(label("Stick"), vec![(1, 0, 5)]),
                ],
                max_sets: 1,
            }],
            strict_priority: false,
        });
        factory.add_process(BufferedConfig {
            name: s("clayLine"),
            accesses: acc(s("minecraft:barrel_7")),
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: Output::new(label("Clay Ball"), 32),
                inputs: vec![BufferedInput::new(label("Cobblestone"), 1)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 64,
            stocks: vec![],
        });
        factory.add_process(BufferedConfig {
            name: s("ironLine"),
            accesses: acc(s("minecraft:barrel_11")),
            slot_filter: None,
            to_extract: None,
            recipes: vec![BufferedRecipe {
                outputs: Output::new(label("Iron Ingot"), 128).or(Output::new(label("Iron Nugget"), 129)),
                inputs: vec![BufferedInput::new(label("Cobblestone"), 1)],
                max_inputs: i32::MAX,
            }],
            max_recipe_inputs: 64,
            stocks: vec![],
        });
        factory.add_process(FluidSlottedConfig {
            name: s("packer"),
            input_slots: vec![vec![0]],
            input_tanks: vec![vec![0]],
            accesses: inv_tank(s("create:basin_1")),
            to_extract: None,
            fluid_extract: None,
            recipes: vec![
                FluidSlottedRecipe {
                    outputs: ignore_outputs(1.),
                    inputs: vec![],
                    fluids: vec![FluidSlottedInput::new(s("thermal:resin"), vec![(0, 250)]).extra_backup(1)],
                    max_sets: 4,
                },
                FluidSlottedRecipe {
                    outputs: ignore_outputs(1.),
                    inputs: vec![MultiInvSlottedInput::new(label("Iron Nugget"), vec![(0, 0, 9)]).extra_backup(120)],
                    fluids: vec![],
                    max_sets: 7,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Red Alloy Ingot"), 32),
                    inputs: vec![MultiInvSlottedInput::new(label("Copper Ingot"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("thermal:redstone"), vec![(0, 250)])],
                    max_sets: 4,
                },
            ],
            strict_priority: false,
        });
        for i in [3, 5, 6, 7] {
            factory.add_process(FluidSlottedConfig {
                name: s("mixer"),
                input_slots: vec![vec![0, 1]],
                input_tanks: vec![vec![0]],
                accesses: inv_tank(local_fmt!("create:basin_{i}")),
                to_extract: None,
                fluid_extract: None,
                recipes: vec![
                    FluidSlottedRecipe {
                        outputs: FluidOutput::new(s("kubejs:volatile_sky_solution"), 16_000),
                        inputs: vec![MultiInvSlottedInput::new(label("Sky Stone Dust"), vec![(0, 0, 4)])],
                        fluids: vec![FluidSlottedInput::new(s("minecraft:water"), vec![(0, 500)])],
                        max_sets: 2,
                    },
                    FluidSlottedRecipe {
                        outputs: FluidOutput::new(s("thermal:redstone"), 16_000),
                        inputs: vec![MultiInvSlottedInput::new(label("Charged Certus Quartz Crystal"), vec![(0, 0, 1)])],
                        fluids: vec![FluidSlottedInput::new(s("kubejs:volatile_sky_solution"), vec![(0, 250)])],
                        max_sets: 4,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(name("kubejs:certus_crystal_seed"), 32),
                        inputs: vec![
                            MultiInvSlottedInput::new(label("Certus Quartz Dust"), vec![(0, 0, 1)]),
                            MultiInvSlottedInput::new(label("Sand"), vec![(0, 1, 1)]),
                        ],
                        fluids: vec![],
                        max_sets: 8,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Algal Blend"), 32),
                        inputs: vec![
                            MultiInvSlottedInput::new(label("Clay Ball"), vec![(0, 0, 1)]),
                            MultiInvSlottedInput::new(label("Kelp"), vec![(0, 1, 1)]),
                        ],
                        fluids: vec![],
                        max_sets: 8,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Andesite Alloy"), 128),
                        inputs: vec![
                            MultiInvSlottedInput::new(label("Algal Brick"), vec![(0, 0, 1)]),
                            MultiInvSlottedInput::new(label("Andesite"), vec![(0, 1, 1)]),
                        ],
                        fluids: vec![],
                        max_sets: 8,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Polished Rose Quartz"), 32),
                        inputs: vec![MultiInvSlottedInput::new(label("Certus Quartz Crystal"), vec![(0, 0, 1)])],
                        fluids: vec![FluidSlottedInput::new(s("thermal:redstone"), vec![(0, 250)])],
                        max_sets: 4,
                    },
                ],
                strict_priority: false,
            });
        }
        factory.add_process(SlottedConfig {
            name: s("waterSpout"),
            accesses: acc(s("minecraft:barrel_15")),
            input_slots: vec![26],
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: Output::new(label("Certus Quartz Crystal"), 128),
                inputs: vec![SlottedInput::new(name("kubejs:certus_crystal_seed"), vec![(26, 1)])],
                max_sets: 8,
            }],
            strict_priority: false,
        });
        for i in [2, 3] {
            factory.add_process(BufferedConfig {
                name: s("ironSpout"),
                accesses: acc(local_fmt!("create:depot_{i}")),
                slot_filter: Some(Box::new(|i| i == 0)),
                to_extract: Some(Box::new(|_, _, x| x.detail.label == "Electron Tube")),
                recipes: vec![BufferedRecipe {
                    outputs: Output::new(label("Electron Tube"), 128),
                    inputs: vec![BufferedInput::new(label("Polished Rose Quartz"), 1)],
                    max_inputs: i32::MAX,
                }],
                max_recipe_inputs: 1,
                stocks: vec![],
            });
        }
        for i in [0, 1, 2] {
            factory.add_process(BufferedConfig {
                name: s("charger"),
                accesses: acc(local_fmt!("ae2:charger_{i}")),
                slot_filter: None,
                to_extract: Some(Box::new(|_, _, x| x.detail.label == "Charged Certus Quartz Crystal")),
                recipes: vec![BufferedRecipe {
                    outputs: Output::new(label("Charged Certus Quartz Crystal"), 32),
                    inputs: vec![BufferedInput::new(label("Certus Quartz Crystal"), 1)],
                    max_inputs: i32::MAX,
                }],
                max_recipe_inputs: 1,
                stocks: vec![],
            });
        }
        factory.add_process(BlockingFluidOutputConfig {
            accesses: tank(s("create:fluid_tank_7")),
            outputs: vec![FluidOutput { fluid: s("minecraft:water"), n_wanted: 16_000 }],
        });
    })
}
