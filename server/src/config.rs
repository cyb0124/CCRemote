use crate::factory::{Factory, FactoryConfig};
use crate::{access::*, config_util::*, process::*, recipe::*, storage::*};
use crate::{detail_cache::DetailCache, server::Server, Tui};
use std::{cell::RefCell, rc::Rc, time::Duration};

pub fn build_factory(tui: Rc<Tui>) -> Rc<RefCell<Factory>> {
    const BUS: &str = "minecraft:barrel_1";
    let acc = |inv_addr| vec![BusAccess { client: s("1a"), inv_addr, bus_addr: s(BUS) }];
    FactoryConfig {
        tui: tui.clone(),
        detail_cache: DetailCache::new(&tui, s("detail_cache.txt")),
        server: Server::new(tui, 1853),
        min_cycle_time: Duration::from_secs(1),
        log_clients: vec![s("1a")],
        bus_accesses: vec![BasicAccess { client: s("1a"), addr: s(BUS) }],
        fluid_bus_accesses: vec![],
        fluid_bus_capacity: 0,
        backups: vec![],
        fluid_backups: vec![],
    }
    .build(|factory| {
        for inv_addr in ["quark:variant_chest_3"] {
            factory.add_storage(ChestConfig { accesses: acc(s(inv_addr)), override_max_stack_size: None })
        }
        factory.add_process(ManualUiConfig { accesses: vec![] });
        for (inv_addr, item, qty) in [
            ("projectexpansion:emc_link_4", "item.kubejs.cube1_packaged", 64),
            ("thermal:dynamo_stirling_0", "Oak Planks", 64),
            ("thermal:dynamo_stirling_1", "Oak Planks", 64),
            ("thermal:dynamo_stirling_2", "Oak Planks", 64),
            ("thermal:dynamo_stirling_3", "Oak Planks", 64),
        ] {
            factory.add_process(BufferedConfig {
                name: s("stock"),
                accesses: acc(s(inv_addr)),
                slot_filter: None,
                to_extract: None,
                recipes: vec![],
                max_recipe_inputs: 0,
                stocks: vec![BufferedInput::new(label(item), qty)],
            })
        }
        factory.add_process(CraftyConfig {
            name: s("craftingGrid"),
            turtles: vec![CraftyTurtle {
                client: s("2a"),
                accesses: vec![CraftyAccess {
                    client: s("1a"),
                    non_consumable_addr: s(""),
                    turtle_addr: s("turtle_0"),
                    bus_addr: s(BUS),
                }],
            }],
            recipes: vec![
                CraftingGridRecipe {
                    outputs: Output::new(label("item.kubejs.cube1"), 64),
                    inputs: vec![
                        CraftingGridInput::new(label("Copper Rod"), vec![1, 3]),
                        CraftingGridInput::new(label("item.kubejs.aluminum_gear"), vec![0]),
                        CraftingGridInput::new(label("Iron Gear"), vec![4]),
                    ],
                    max_sets: 64,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Oak Planks"), 64),
                    inputs: vec![CraftingGridInput::new(label("Oak Log"), vec![0])],
                    max_sets: 16,
                    non_consumables: vec![],
                },
            ],
        });
        let recipes = &[
            (64, "Aluminium Ingot", "Raw Bauxite"),
            (64, "Copper Ingot", "Raw Copper"),
            (64, "Nickel Ingot", "Raw Nickel"),
            (64, "Iron Ingot", "Raw Iron"),
            (64, "Gold Ingot", "Raw Gold"),
            (64, "Tin Ingot", "Raw Tin"),
            (64, "Stone", "Cobblestone"),
            (64, "Glass", "Sand"),
        ];
        for inv_addr in ["create:depot_0", "create:depot_1"] {
            factory.add_process(BufferedConfig {
                name: s("lavaFan"),
                accesses: acc(s(inv_addr)),
                slot_filter: Some(Box::new(|slot| slot == 0)),
                to_extract: Some(Box::new(|_, _, stack| recipes.iter().all(|&(_, _, i)| stack.detail.label != i))),
                max_recipe_inputs: 16,
                stocks: vec![],
                recipes: (recipes.iter())
                    .map(|&(qty, o, i)| BufferedRecipe {
                        outputs: Output::new(label(o), qty),
                        inputs: vec![BufferedInput::new(label(i), 16)],
                        max_inputs: i32::MAX,
                    })
                    .collect(),
            });
        }
        factory.add_process(SlottedConfig {
            name: s("rodPress"),
            accesses: acc(s("thermal:machine_press_0")),
            input_slots: vec![0],
            to_extract: Some(Box::new(|_, slot, _| slot >= 2)),
            strict_priority: false,
            recipes: vec![SlottedRecipe {
                outputs: Output::new(label("Copper Rod"), 64),
                inputs: vec![SlottedInput::new(label("Copper Ingot"), vec![(0, 1)])],
                max_sets: 8,
            }],
        });
        factory.add_process(SlottedConfig {
            name: s("gearPress"),
            accesses: acc(s("thermal:machine_press_1")),
            input_slots: vec![0],
            to_extract: Some(Box::new(|_, slot, _| slot >= 2)),
            strict_priority: false,
            recipes: vec![
                SlottedRecipe {
                    outputs: Output::new(label("Tin Gear"), 64),
                    inputs: vec![SlottedInput::new(label("Tin Ingot"), vec![(0, 4)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Iron Gear"), 64),
                    inputs: vec![SlottedInput::new(label("Iron Ingot"), vec![(0, 4)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Invar Gear"), 64),
                    inputs: vec![SlottedInput::new(label("Invar Ingot"), vec![(0, 4)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("item.kubejs.aluminum_gear"), 64),
                    inputs: vec![SlottedInput::new(label("Aluminium Ingot"), vec![(0, 4)])],
                    max_sets: 8,
                },
            ],
        });
        for inv_addr in ["thermal:machine_press_2", "thermal:machine_press_3"] {
            factory.add_process(SlottedConfig {
                name: s("press"),
                accesses: acc(s(inv_addr)),
                input_slots: vec![0],
                to_extract: Some(Box::new(|_, slot, _| slot >= 2)),
                strict_priority: false,
                recipes: vec![SlottedRecipe {
                    outputs: Output::new(label("item.kubejs.cube1_packaged"), 64),
                    inputs: vec![SlottedInput::new(label("item.kubejs.cube1"), vec![(0, 1)])],
                    max_sets: 8,
                }],
            })
        }
        factory.add_process(SlottedConfig {
            name: s("inductionSmelter"),
            accesses: acc(s("thermal:machine_smelter_0")),
            input_slots: vec![0, 1, 2],
            to_extract: Some(Box::new(|_, slot, _| slot >= 4)),
            strict_priority: false,
            recipes: vec![
                SlottedRecipe {
                    outputs: Output::new(label("Invar Ingot"), 64),
                    inputs: vec![
                        SlottedInput::new(label("Iron Ingot"), vec![(0, 2)]),
                        SlottedInput::new(label("Nickel Ingot"), vec![(1, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Constantan Ingot"), 64),
                    inputs: vec![
                        SlottedInput::new(label("Copper Ingot"), vec![(0, 1)]),
                        SlottedInput::new(label("Nickel Ingot"), vec![(1, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Bronze Ingot"), 64),
                    inputs: vec![
                        SlottedInput::new(label("Copper Ingot"), vec![(0, 3)]),
                        SlottedInput::new(label("Tin Ingot"), vec![(1, 1)]),
                    ],
                    max_sets: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: s("pulverizer"),
            accesses: acc(s("thermal:machine_pulverizer_0")),
            input_slots: vec![0],
            to_extract: extract_all(),
            strict_priority: false,
            recipes: vec![
                SlottedRecipe {
                    outputs: Output::new(label("Coal"), 64),
                    inputs: vec![SlottedInput::new(label("Coal Ore"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Gravel"), 64),
                    inputs: vec![SlottedInput::new(label("Cobblestone"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Sand"), 64),
                    inputs: vec![SlottedInput::new(label("Gravel"), vec![(0, 1)])],
                    max_sets: 8,
                },
            ],
        });
        for (inv_addr, item) in [
            ("projectexpansion:emc_link_1", "Raw Copper"),
            ("projectexpansion:emc_link_2", "Raw Iron"),
            ("projectexpansion:emc_link_3", "Raw Bauxite"),
            ("projectexpansion:emc_link_4", "Cobblestone"),
            ("projectexpansion:emc_link_9", "Oak Log"),
            ("projectexpansion:emc_link_7", "Raw Gold"),
            ("projectexpansion:emc_link_8", "Raw Nickel"),
            ("projectexpansion:emc_link_10", "Coal Ore"),
            ("projectexpansion:emc_link_11", "Raw Tin"),
        ] {
            factory.add_process(BlockingOutputConfig {
                accesses: acc(s(inv_addr)),
                slot_filter: None,
                outputs: vec![Output { item: label(item), n_wanted: 64 }],
            })
        }
    })
}
