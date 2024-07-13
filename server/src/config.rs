use flexstr::LocalStr;

use crate::factory::{Factory, FactoryConfig, FluidStorageConfig};
use crate::{access::*, config_util::*, process::*, recipe::*, storage::*};
use crate::{detail_cache::DetailCache, server::Server, Tui};
use std::{cell::RefCell, rc::Rc, time::Duration};

struct Metal {
    raw: &'static str,
    dust: &'static str,
    ingot: &'static str,
}

const METALS: &[Metal] = &[
    Metal { raw: "Raw Bauxite", dust: "item.kubejs.aluminum_dust", ingot: "Aluminium Ingot" },
    Metal { raw: "Raw Uranium", dust: "Uranium Dust", ingot: "Uranium Ingot" },
    Metal { raw: "Raw Nickel", dust: "Nickel Dust", ingot: "Nickel Ingot" },
    Metal { raw: "Raw Copper", dust: "Copper Dust", ingot: "Copper Ingot" },
    Metal { raw: "Raw Silver", dust: "Silver Dust", ingot: "Silver Ingot" },
    Metal { raw: "Raw Osmium", dust: "Osmium Dust", ingot: "Osmium Ingot" },
    Metal { raw: "Raw Iron", dust: "Iron Dust", ingot: "Iron Ingot" },
    Metal { raw: "Raw Lead", dust: "Lead Dust", ingot: "Lead Ingot" },
    Metal { raw: "Raw Gold", dust: "Gold Dust", ingot: "Gold Ingot" },
    Metal { raw: "Raw Zinc", dust: "item.kubejs.zinc_dust", ingot: "Zinc Ingot" },
    Metal { raw: "Raw Tin", dust: "Tin Dust", ingot: "Tin Ingot" },
];

pub fn build_factory(tui: Rc<Tui>) -> Rc<RefCell<Factory>> {
    const BUS: &str = "minecraft:barrel_9";
    let fbus = vec![s("ae2:sky_tank_1")];
    let acc = |inv_addr| vec![BusAccess { client: s("1a"), inv_addr, bus_addr: s(BUS) }];
    let facc = |inv_addr: LocalStr| {
        vec![InvTankAccess {
            client: s("1a"),
            inv_addrs: vec![inv_addr.clone()],
            tank_addrs: vec![inv_addr],
            bus_addr: s(BUS),
            fluid_bus_addrs: fbus.clone(),
        }]
    };
    FactoryConfig {
        tui: tui.clone(),
        detail_cache: DetailCache::new(&tui, s("detail_cache.txt")),
        server: Server::new(tui, 1853),
        min_cycle_time: Duration::from_secs(1),
        log_clients: vec![s("1a")],
        bus_accesses: vec![BasicAccess { client: s("1a"), addr: s(BUS) }],
        fluid_bus_accesses: vec![FluidAccess { client: s("1a"), fluid_bus_addrs: fbus.clone() }],
        fluid_bus_capacity: 16_000,
        fluid_backups: vec![],
        backups: vec![],
    }
    .build(|factory| {
        factory.add_process(ManualUiConfig { accesses: vec![] });
        for inv_addr in ["ironchest:diamond_chest_1", "ironchest:diamond_chest_2", "ironchest:diamond_chest_3"] {
            factory.add_storage(ChestConfig { accesses: acc(s(inv_addr)), override_max_stack_size: None })
        }
        for (tank_addr, fluid) in [
            ("ae2:sky_tank_2", "minecraft:water"),
            ("ae2:sky_tank_3", "minecraft:lava"),
            ("ae2:sky_tank_4", "minecraft:lava"),
            ("ae2:sky_tank_5", "thermal:ender"),
            ("ae2:sky_tank_6", "thermal:glowstone"),
            ("ae2:sky_tank_7", "thermal:redstone"),
        ] {
            factory.add_fluid_storage(FluidStorageConfig {
                accesses: vec![TankAccess { client: s("1a"), tank_addr: s(tank_addr), fluid_bus_addrs: fbus.clone() }],
                fluid: s(fluid),
                capacity: 16_000,
            })
        }
        for (inv_addr, item, qty) in [
            ("projectexpansion:emc_link_4", "item.kubejs.cube3_packaged", 1),
            ("thermal:machine_crucible_0", "Cobblestone", 64),
            ("create:deployer_1", "Coal", 64),
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
        for (tank_addr, fluid, qty) in [("powah:reactor_part_0", "minecraft:water", 1_000)] {
            factory.add_process(FluidSlottedConfig {
                name: s("fluidStock"),
                input_slots: vec![],
                input_tanks: vec![vec![0]],
                accesses: facc(s(tank_addr)),
                to_extract: None,
                fluid_extract: None,
                strict_priority: false,
                recipes: vec![FluidSlottedRecipe {
                    outputs: ignore_outputs(1.),
                    inputs: vec![],
                    fluids: vec![FluidSlottedInput::new(s(fluid), vec![(0, 1)])],
                    max_sets: qty,
                }],
            })
        }
        for inv_addr in [
            "minecraft:barrel_2", // mixer
            "minecraft:barrel_4", // metallurgicInfuser
            "minecraft:barrel_7", // enrichmentChamber
        ] {
            factory.add_process(SlottedConfig {
                name: s("output"),
                accesses: acc(s(inv_addr)),
                input_slots: vec![],
                to_extract: extract_all(),
                recipes: vec![],
                strict_priority: false,
            })
        }
        factory.add_process(BufferedConfig {
            name: s("reactor"),
            accesses: acc(s("minecraft:hopper_3")),
            slot_filter: None,
            to_extract: None,
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![
                BufferedInput::new(label("Redstone Dust"), 64),
                BufferedInput::new(label("Uraninite"), 64),
                BufferedInput::new(label("Coal"), 64),
            ],
        });
        factory.add_process(CraftyConfig {
            name: s("craftingGrid"),
            turtles: vec![CraftyTurtle {
                client: s("2a"),
                accesses: vec![CraftyAccess {
                    client: s("1a"),
                    non_consumable_addr: s("minecraft:barrel_1"),
                    turtle_addr: s("turtle_1"),
                    bus_addr: s(BUS),
                }],
            }],
            recipes: vec![
                CraftingGridRecipe {
                    outputs: Output::new(label("item.kubejs.cube1"), 16),
                    inputs: vec![
                        CraftingGridInput::new(label("Copper Rod"), vec![1, 3]),
                        CraftingGridInput::new(label("item.kubejs.aluminum_gear"), vec![0]),
                        CraftingGridInput::new(label("Iron Gear"), vec![4]),
                    ],
                    max_sets: 64,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Hardened Integral Components"), 4),
                    inputs: vec![
                        CraftingGridInput::new(label("Invar Ingot"), vec![0, 2, 6, 8]),
                        CraftingGridInput::new(label("item.kubejs.cube1"), vec![4]),
                        CraftingGridInput::new(label("Glass"), vec![1, 7]),
                        CraftingGridInput::new(label("Redstone Dust"), vec![3, 5]),
                    ],
                    max_sets: 64,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Reinforced Integral Components"), 4),
                    inputs: vec![
                        CraftingGridInput::new(label("Electrum Ingot"), vec![0, 2, 6, 8]),
                        CraftingGridInput::new(label("Nether Quartz"), vec![4]),
                        CraftingGridInput::new(label("item.kubejs.cube2"), vec![7]),
                        CraftingGridInput::new(label("Hardened Integral Components"), vec![1]),
                        CraftingGridInput::new(label("Signalum Gear"), vec![3, 5]),
                    ],
                    max_sets: 64,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Redstone Flux Coil"), 16),
                    inputs: vec![
                        CraftingGridInput::new(label("Gold Ingot"), vec![4]),
                        CraftingGridInput::new(label("Redstone Dust"), vec![2, 6]),
                    ],
                    max_sets: 64,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Flux Linkage Amplifier"), 16),
                    inputs: vec![
                        CraftingGridInput::new(label("Redstone Flux Coil"), vec![4]),
                        CraftingGridInput::new(label("Electrum Plate"), vec![3, 5]),
                        CraftingGridInput::new(label("Lead Gear"), vec![1, 7]),
                    ],
                    max_sets: 64,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Dielectric Paste"), 16),
                    inputs: vec![
                        CraftingGridInput::new(label("Coal"), vec![0, 1]),
                        CraftingGridInput::new(label("Clay Ball"), vec![3]),
                        CraftingGridInput::new(label("Redstone Dust"), vec![4]),
                    ],
                    max_sets: 4,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("High Covalence Dust"), 64),
                    inputs: vec![
                        CraftingGridInput::new(label("Gold Ingot"), vec![0]),
                        CraftingGridInput::new(label("Coal"), vec![1]),
                    ],
                    max_sets: 1,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Andesite"), 64),
                    inputs: vec![
                        CraftingGridInput::new(label("Cobblestone"), vec![0, 1]),
                        CraftingGridInput::new(label("High Covalence Dust"), vec![3, 4]),
                    ],
                    max_sets: 32,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Iron Nugget"), 64),
                    inputs: vec![CraftingGridInput::new(label("Iron Ingot"), vec![0])],
                    max_sets: 7,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Block of Redstone"), 16),
                    inputs: vec![CraftingGridInput::new(label("Redstone Dust"), (0..9).collect())],
                    max_sets: 64,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Basic Capacitor"), 16),
                    inputs: vec![
                        CraftingGridInput::new(label("Block of Redstone"), vec![4]),
                        CraftingGridInput::new(label("Dielectric Paste"), vec![2, 6]),
                        CraftingGridInput::new(label("Iron Ingot"), vec![1, 3, 5, 7]),
                    ],
                    max_sets: 16,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Basic Capacitor (Large)"), 16),
                    inputs: vec![CraftingGridInput::new(label("Basic Capacitor"), vec![0, 1])],
                    max_sets: 64,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Hardened Capacitor"), 16),
                    inputs: vec![
                        CraftingGridInput::new(label("Basic Capacitor (Large)"), vec![4]),
                        CraftingGridInput::new(label("Dielectric Paste"), vec![0, 2, 6, 8]),
                        CraftingGridInput::new(label("Energized Steel"), vec![1, 3, 5, 7]),
                    ],
                    max_sets: 32,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Blazing Capacitor"), 16),
                    inputs: vec![
                        CraftingGridInput::new(label("Basic Capacitor (Large)"), vec![4]),
                        CraftingGridInput::new(label("Dielectric Paste"), vec![0, 2, 6, 8]),
                        CraftingGridInput::new(label("Blazing Crystal"), vec![1, 3, 5, 7]),
                    ],
                    max_sets: 32,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Soul Stone"), 1),
                    inputs: vec![
                        CraftingGridInput::new(label("Redstone Dust"), vec![4]),
                        CraftingGridInput::new(label("Glowstone Dust"), vec![0, 1, 2, 6, 7, 8]),
                        CraftingGridInput::new(label("Lapis Lazuli"), vec![3, 5]),
                    ],
                    max_sets: 1,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("item.kubejs.dormant_effigy"), 16),
                    inputs: vec![
                        CraftingGridInput::new(label("Soul Stone"), vec![1]),
                        CraftingGridInput::new(label("Calcite"), vec![3, 4, 5, 7]),
                    ],
                    max_sets: 64,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Sandstone"), 16),
                    inputs: vec![CraftingGridInput::new(label("Sand"), vec![0, 1, 3, 4])],
                    max_sets: 64,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Gunpowder"), 16),
                    inputs: vec![
                        CraftingGridInput::new(label("Charcoal"), vec![0]),
                        CraftingGridInput::new(label("Niter"), vec![1, 3]),
                        CraftingGridInput::new(label("Sulfur"), vec![4]),
                    ],
                    max_sets: 16,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Fire Charge"), 16),
                    inputs: vec![
                        CraftingGridInput::new(label("Gunpowder"), vec![0]),
                        CraftingGridInput::new(label("Blaze Powder"), vec![1]),
                        CraftingGridInput::new(label("Coal"), vec![2]),
                    ],
                    max_sets: 21,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Ice Charge"), 16),
                    inputs: vec![
                        CraftingGridInput::new(label("Gunpowder"), vec![0]),
                        CraftingGridInput::new(label("Blizz Powder"), vec![1]),
                        CraftingGridInput::new(label("Coal"), vec![2]),
                    ],
                    max_sets: 21,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Lightning Charge"), 16),
                    inputs: vec![
                        CraftingGridInput::new(label("Gunpowder"), vec![0]),
                        CraftingGridInput::new(label("Blitz Powder"), vec![1]),
                        CraftingGridInput::new(label("Coal"), vec![2]),
                    ],
                    max_sets: 21,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Earth Charge"), 16),
                    inputs: vec![
                        CraftingGridInput::new(label("Gunpowder"), vec![0]),
                        CraftingGridInput::new(label("Basalz Powder"), vec![1]),
                        CraftingGridInput::new(label("Coal"), vec![2]),
                    ],
                    max_sets: 21,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Ender Pearl"), 16),
                    inputs: vec![CraftingGridInput::new(label("Iron Ingot"), vec![0, 1, 2, 3])],
                    max_sets: 64,
                    non_consumables: vec![NonConsumable { storage_slot: 0, crafting_grid_slot: 4 }],
                },
            ],
        });
        let recipes = Rc::from_iter((METALS.iter().map(|x| (64, x.ingot, x.dust))).chain([
            (64, "Steel Ingot", "Steel Dust"),
            (64, "Charcoal", "Oak Log"),
            (64, "Stone", "Cobblestone"),
            (64, "Glass", "Sand"),
        ]));
        for inv_addr in ["create:depot_0", "create:depot_1", "create:depot_5"] {
            let recipes_ = recipes.clone();
            factory.add_process(BufferedConfig {
                name: s("lavaFan"),
                accesses: acc(s(inv_addr)),
                slot_filter: Some(Box::new(|slot| slot == 0)),
                to_extract: Some(Box::new(move |_, _, stack| {
                    recipes_.iter().all(|&(_, _, i)| stack.detail.label != i)
                })),
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
        let recipes = [(64, "Nether Quartz", "Soul Sand")];
        for inv_addr in ["create:depot_2", "create:depot_3"] {
            factory.add_process(BufferedConfig {
                name: s("waterFan"),
                accesses: acc(s(inv_addr)),
                slot_filter: Some(Box::new(|slot| slot == 0)),
                to_extract: Some(Box::new(move |_, _, stack| recipes.iter().all(|&(_, _, i)| stack.detail.label != i))),
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
        factory.add_process(FluidSlottedConfig {
            name: s("spout"),
            accesses: vec![InvTankAccess {
                client: s("1a"),
                inv_addrs: vec![s("create:depot_4")],
                tank_addrs: vec![s("create:spout_0")],
                bus_addr: s(BUS),
                fluid_bus_addrs: fbus.clone(),
            }],
            input_slots: vec![vec![0]],
            input_tanks: vec![vec![0]],
            to_extract: None,
            fluid_extract: None,
            strict_priority: false,
            recipes: vec![
                FluidSlottedRecipe {
                    outputs: Output::new(label("item.kubejs.blaze_effigy"), 16),
                    inputs: vec![MultiInvSlottedInput::new(label("item.kubejs.dormant_effigy"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("minecraft:lava"), vec![(0, 1_000)])],
                    max_sets: 1,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("item.kubejs.blizz_effigy"), 16),
                    inputs: vec![MultiInvSlottedInput::new(label("item.kubejs.dormant_effigy"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("thermal:ender"), vec![(0, 1_000)])],
                    max_sets: 1,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("item.kubejs.blitz_effigy"), 16),
                    inputs: vec![MultiInvSlottedInput::new(label("item.kubejs.dormant_effigy"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("thermal:glowstone"), vec![(0, 1_000)])],
                    max_sets: 1,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("item.kubejs.basalz_effigy"), 16),
                    inputs: vec![MultiInvSlottedInput::new(label("item.kubejs.dormant_effigy"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("thermal:redstone"), vec![(0, 1_000)])],
                    max_sets: 1,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: s("spoutOutput"),
            accesses: acc(s("create:depot_4")),
            input_slots: vec![],
            to_extract: Some(Box::new(|_, _, stack| stack.detail.label != "item.kubejs.dormant_effigy")),
            strict_priority: false,
            recipes: vec![],
        });
        factory.add_process(SlottedConfig {
            name: s("rodPress"),
            accesses: acc(s("thermal:machine_press_0")),
            input_slots: vec![0],
            to_extract: Some(Box::new(|_, slot, _| slot >= 2)),
            strict_priority: false,
            recipes: vec![
                SlottedRecipe {
                    outputs: Output::new(label("Copper Rod"), 64),
                    inputs: vec![SlottedInput::new(label("Copper Ingot"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("item.kubejs.bronze_rod"), 64),
                    inputs: vec![SlottedInput::new(label("Bronze Ingot"), vec![(0, 1)])],
                    max_sets: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: s("gearPress"),
            accesses: acc(s("thermal:machine_press_1")),
            input_slots: vec![0],
            to_extract: Some(Box::new(|_, slot, _| slot >= 2)),
            strict_priority: false,
            recipes: vec![
                SlottedRecipe {
                    outputs: Output::new(label("Tin Gear"), 16),
                    inputs: vec![SlottedInput::new(label("Tin Ingot"), vec![(0, 4)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Iron Gear"), 16),
                    inputs: vec![SlottedInput::new(label("Iron Ingot"), vec![(0, 4)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Lead Gear"), 16),
                    inputs: vec![SlottedInput::new(label("Lead Ingot"), vec![(0, 4)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Invar Gear"), 16),
                    inputs: vec![SlottedInput::new(label("Invar Ingot"), vec![(0, 4)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Signalum Gear"), 16),
                    inputs: vec![SlottedInput::new(label("Signalum Ingot"), vec![(0, 4)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("item.kubejs.steel_gear"), 16),
                    inputs: vec![SlottedInput::new(label("Steel Ingot"), vec![(0, 4)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("item.kubejs.aluminum_gear"), 16),
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
                recipes: vec![
                    SlottedRecipe {
                        outputs: Output::new(label("item.kubejs.cube3_packaged"), 16),
                        inputs: vec![SlottedInput::new(label("item.kubejs.cube3"), vec![(0, 1)])],
                        max_sets: 8,
                    },
                    SlottedRecipe {
                        outputs: Output::new(label("Electrum Plate"), 64),
                        inputs: vec![SlottedInput::new(label("Electrum Ingot"), vec![(0, 1)])],
                        max_sets: 8,
                    },
                    SlottedRecipe {
                        outputs: Output::new(label("Invar Plate"), 64),
                        inputs: vec![SlottedInput::new(label("Invar Ingot"), vec![(0, 1)])],
                        max_sets: 8,
                    },
                ],
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
                        SlottedInput::new(label("Iron Dust"), vec![(0, 2)]),
                        SlottedInput::new(label("Nickel Dust"), vec![(1, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Constantan Ingot"), 64),
                    inputs: vec![
                        SlottedInput::new(label("Copper Dust"), vec![(0, 1)]),
                        SlottedInput::new(label("Nickel Dust"), vec![(1, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Brass Ingot"), 64),
                    inputs: vec![
                        SlottedInput::new(label("item.kubejs.zinc_dust"), vec![(0, 1)]),
                        SlottedInput::new(label("Copper Dust"), vec![(1, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Electrum Ingot"), 64),
                    inputs: vec![
                        SlottedInput::new(label("Gold Dust"), vec![(0, 1)]),
                        SlottedInput::new(label("Silver Dust"), vec![(1, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Bronze Ingot"), 64),
                    inputs: vec![
                        SlottedInput::new(label("Copper Dust"), vec![(0, 3)]),
                        SlottedInput::new(label("Tin Dust"), vec![(1, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Signalum Ingot"), 64),
                    inputs: vec![
                        SlottedInput::new(label("Silver Dust"), vec![(0, 1)]),
                        SlottedInput::new(label("Copper Dust"), vec![(1, 3)]),
                        SlottedInput::new(label("Redstone Dust"), vec![(2, 4)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Calcite"), 64),
                    inputs: vec![
                        SlottedInput::new(label("Bone Meal"), vec![(0, 1)]),
                        SlottedInput::new(label("Stone"), vec![(1, 1)]),
                    ],
                    max_sets: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: s("mixer"),
            accesses: acc(s("minecraft:barrel_3")),
            input_slots: vec![0, 1, 2, 3, 4, 5],
            to_extract: None,
            strict_priority: false,
            recipes: vec![
                SlottedRecipe {
                    outputs: Output::new(label("Andesite Alloy"), 64),
                    inputs: vec![
                        SlottedInput::new(label("Andesite"), vec![(0, 1)]),
                        SlottedInput::new(label("Iron Nugget"), vec![(1, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("item.kubejs.cube2"), 16),
                    inputs: vec![
                        SlottedInput::new(label("Invar Gear"), vec![(0, 1)]),
                        SlottedInput::new(label("item.kubejs.cube1"), vec![(1, 1)]),
                        SlottedInput::new(label("item.kubejs.steel_gear"), vec![(2, 1)]),
                        SlottedInput::new(label("item.kubejs.bronze_rod"), vec![(3, 2)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("item.kubejs.cube3"), 16),
                    inputs: vec![
                        SlottedInput::new(label("item.kubejs.cube2"), vec![(0, 1)]),
                        SlottedInput::new(label("Basic Control Circuit"), vec![(1, 1)]),
                        SlottedInput::new(label("Fire Charge"), vec![(2, 1)]),
                        SlottedInput::new(label("Ice Charge"), vec![(3, 1)]),
                        SlottedInput::new(label("Lightning Charge"), vec![(4, 1)]),
                        SlottedInput::new(label("Earth Charge"), vec![(5, 1)]),
                    ],
                    max_sets: 8,
                },
            ],
        });
        for inv_addr in ["thermal:machine_pulverizer_0", "thermal:machine_pulverizer_1"] {
            factory.add_process(SlottedConfig {
                name: s("pulverizer"),
                accesses: acc(s(inv_addr)),
                input_slots: vec![0],
                to_extract: extract_all(),
                strict_priority: false,
                recipes: vec![
                    SlottedRecipe {
                        outputs: Output::new(label("Niter"), 64),
                        inputs: vec![SlottedInput::new(label("Sandstone"), vec![(0, 1)])],
                        max_sets: 8,
                    },
                    SlottedRecipe {
                        outputs: Output::new(label("Bone Meal"), 64),
                        inputs: vec![SlottedInput::new(label("Bone"), vec![(0, 1)])],
                        max_sets: 8,
                    },
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
                    SlottedRecipe {
                        outputs: Output::new(label("Lapis Lazuli"), 64),
                        inputs: vec![SlottedInput::new(label("Lapis Lazuli Ore"), vec![(0, 1)])],
                        max_sets: 8,
                    },
                    SlottedRecipe {
                        outputs: Output::new(label("Blaze Powder"), 16).and(Output::new(label("Sulfur"), 16)),
                        inputs: vec![SlottedInput::new(label("Blaze Rod"), vec![(0, 1)])],
                        max_sets: 8,
                    },
                    SlottedRecipe {
                        outputs: Output::new(label("Blizz Powder"), 16),
                        inputs: vec![SlottedInput::new(label("Blizz Cube"), vec![(0, 1)])],
                        max_sets: 8,
                    },
                    SlottedRecipe {
                        outputs: Output::new(label("Blitz Powder"), 16),
                        inputs: vec![SlottedInput::new(label("Blitz Mote"), vec![(0, 1)])],
                        max_sets: 8,
                    },
                    SlottedRecipe {
                        outputs: Output::new(label("Basalz Powder"), 16),
                        inputs: vec![SlottedInput::new(label("Basalz Shard"), vec![(0, 1)])],
                        max_sets: 8,
                    },
                ],
            })
        }
        factory.add_process(SlottedConfig {
            name: s("pyrolyzer"),
            accesses: acc(s("thermal:machine_pyrolyzer_0")),
            input_slots: vec![0],
            to_extract: extract_all(),
            strict_priority: false,
            recipes: vec![SlottedRecipe {
                outputs: Output::new(label("Coal Coke"), 64),
                inputs: vec![SlottedInput::new(label("Coal"), vec![(0, 1)])],
                max_sets: 8,
            }],
        });
        factory.add_process(SlottedConfig {
            name: s("sawmill"),
            accesses: acc(s("thermal:machine_sawmill_1")),
            input_slots: vec![0],
            to_extract: extract_all(),
            strict_priority: false,
            recipes: vec![
                SlottedRecipe {
                    outputs: Output::new(label("Oak Planks"), 64),
                    inputs: vec![SlottedInput::new(label("Oak Log"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Blaze Rod"), 16),
                    inputs: vec![SlottedInput::new(label("item.kubejs.blaze_effigy"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Blizz Cube"), 16),
                    inputs: vec![SlottedInput::new(label("item.kubejs.blizz_effigy"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Blitz Mote"), 16),
                    inputs: vec![SlottedInput::new(label("item.kubejs.blitz_effigy"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Basalz Shard"), 16),
                    inputs: vec![SlottedInput::new(label("item.kubejs.basalz_effigy"), vec![(0, 1)])],
                    max_sets: 8,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: s("energizingOrb"),
            accesses: acc(s("powah:energizing_orb_0")),
            input_slots: vec![1, 2],
            to_extract: extract_all(),
            strict_priority: false,
            recipes: vec![
                SlottedRecipe {
                    outputs: Output::new(label("Uraninite"), 64),
                    inputs: vec![SlottedInput::new(label("Uranium Ingot"), vec![(1, 1)])],
                    max_sets: 1,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Blazing Crystal"), 64),
                    inputs: vec![SlottedInput::new(label("Blaze Rod"), vec![(1, 1)])],
                    max_sets: 1,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Energized Steel"), 64),
                    inputs: vec![
                        SlottedInput::new(label("Iron Ingot"), vec![(1, 1)]),
                        SlottedInput::new(label("Gold Ingot"), vec![(2, 1)]),
                    ],
                    max_sets: 1,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: s("enrichmentChamber"),
            accesses: acc(s("minecraft:barrel_6")),
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 64,
            recipes: (METALS.iter())
                .map(|x| BufferedRecipe {
                    outputs: Output::new(label(x.dust), 64),
                    inputs: vec![BufferedInput::new(label(x.raw), 3)],
                    max_inputs: i32::MAX,
                })
                .chain([
                    BufferedRecipe {
                        outputs: Output::new(label("Enriched Redstone"), 16),
                        inputs: vec![BufferedInput::new(label("Redstone Dust"), 1)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: Output::new(label("Enriched Carbon"), 16),
                        inputs: vec![BufferedInput::new(label("Coal"), 1)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: Output::new(label("Redstone Dust"), 64),
                        inputs: vec![BufferedInput::new(label("Redstone Ore"), 1)],
                        max_inputs: i32::MAX,
                    },
                ])
                .collect(),
        });
        factory.add_process(MultiInvSlottedConfig {
            name: s("metallurgicInfuser"),
            input_slots: vec![vec![0], vec![0]],
            accesses: vec![MultiInvAccess {
                client: s("1a"),
                inv_addrs: vec![s("minecraft:hopper_0"), s("minecraft:hopper_1")],
                bus_addr: s(BUS),
            }],
            to_extract: None,
            strict_priority: false,
            recipes: vec![
                MultiInvSlottedRecipe {
                    outputs: Output::new(label("Basic Control Circuit"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Enriched Redstone"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Osmium Ingot"), vec![(1, 0, 4)]),
                    ],
                    max_sets: 2,
                },
                MultiInvSlottedRecipe {
                    outputs: Output::new(label("Infused Alloy"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Enriched Redstone"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Iron Ingot"), vec![(1, 0, 8)]),
                    ],
                    max_sets: 2,
                },
                MultiInvSlottedRecipe {
                    outputs: Output::new(label("Enriched Iron"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Enriched Carbon"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Iron Ingot"), vec![(1, 0, 8)]),
                    ],
                    max_sets: 2,
                },
                MultiInvSlottedRecipe {
                    outputs: Output::new(label("Steel Dust"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Enriched Carbon"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Enriched Iron"), vec![(1, 0, 8)]),
                    ],
                    max_sets: 2,
                },
            ],
        });
        factory.add_process(FluidSlottedConfig {
            name: s("magmaCrucible"),
            input_slots: vec![vec![0]],
            input_tanks: vec![vec![]],
            accesses: facc(s("thermal:machine_crucible_3")),
            to_extract: None,
            fluid_extract: fluid_extract_all(),
            strict_priority: false,
            recipes: vec![
                FluidSlottedRecipe {
                    outputs: FluidOutput::new(s("minecraft:lava"), 8_000),
                    inputs: vec![MultiInvSlottedInput::new(label("Cobblestone"), vec![(0, 0, 1)])],
                    fluids: vec![],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: FluidOutput::new(s("thermal:ender"), 8_000),
                    inputs: vec![MultiInvSlottedInput::new(label("Ender Pearl"), vec![(0, 0, 1)])],
                    fluids: vec![],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: FluidOutput::new(s("thermal:glowstone"), 8_000),
                    inputs: vec![MultiInvSlottedInput::new(label("Glowstone Dust"), vec![(0, 0, 1)])],
                    fluids: vec![],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: FluidOutput::new(s("thermal:redstone"), 8_000),
                    inputs: vec![MultiInvSlottedInput::new(label("Redstone Dust"), vec![(0, 0, 1)])],
                    fluids: vec![],
                    max_sets: 8,
                },
            ],
        });
        for (inv_addr, item) in [
            ("projectexpansion:emc_link_1", "Raw Copper"),
            ("projectexpansion:emc_link_24", "Raw Iron"),
            ("projectexpansion:emc_link_3", "Raw Bauxite"),
            ("projectexpansion:emc_link_4", "Cobblestone"),
            ("projectexpansion:emc_link_9", "Oak Log"),
            ("projectexpansion:emc_link_7", "Raw Gold"),
            ("projectexpansion:emc_link_8", "Raw Nickel"),
            ("projectexpansion:emc_link_10", "Coal Ore"),
            ("projectexpansion:emc_link_11", "Raw Tin"),
            ("projectexpansion:emc_link_13", "Raw Zinc"),
            ("projectexpansion:emc_link_14", "Redstone Ore"),
            ("projectexpansion:emc_link_15", "Raw Silver"),
            ("projectexpansion:emc_link_16", "Raw Lead"),
            ("projectexpansion:emc_link_17", "Raw Osmium"),
            ("projectexpansion:emc_link_18", "Soul Sand"),
            ("projectexpansion:emc_link_25", "Clay Ball"),
            ("projectexpansion:emc_link_26", "Raw Uranium"),
            ("projectexpansion:emc_link_27", "Lapis Lazuli Ore"),
            ("projectexpansion:emc_link_28", "Glowstone Dust"),
            ("projectexpansion:emc_link_29", "Bone"),
        ] {
            factory.add_process(BlockingOutputConfig {
                accesses: acc(s(inv_addr)),
                slot_filter: None,
                outputs: vec![Output { item: label(item), n_wanted: 64 }],
            })
        }
        factory.add_process(BlockingFluidOutputConfig {
            accesses: vec![TankAccess {
                client: s("1a"),
                tank_addr: s("thermal:device_water_gen_0"),
                fluid_bus_addrs: fbus.clone(),
            }],
            outputs: vec![FluidOutput { fluid: s("minecraft:water"), n_wanted: 8_000 }],
        });
    })
}
