use crate::factory::{Factory, FactoryConfig, FluidStorageConfig};
use crate::{access::*, config_util::*, item::*, process::*, recipe::*, storage::*};
use crate::{detail_cache::DetailCache, server::Server, Tui};
use flexstr::{local_fmt, LocalStr};
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
    const BUS: &str = "ironchest:diamond_chest_11";
    let fbus = vec![s("ae2:sky_tank_1"), s("ae2:sky_tank_22")];
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
        backups: vec![(label("Substrate"), 16)],
    }
    .build(|factory| {
        factory.add_process(ManualUiConfig { accesses: vec![] });
        for inv_addr in [
            "ironchest:diamond_chest_1",
            "ironchest:diamond_chest_2",
            "ironchest:diamond_chest_3",
            "ironchest:diamond_chest_4",
            "ironchest:diamond_chest_6",
            "ironchest:diamond_chest_7",
            "ironchest:diamond_chest_8",
            "ironchest:diamond_chest_9",
        ] {
            factory.add_storage(ChestConfig { accesses: acc(s(inv_addr)), override_max_stack_size: None })
        }
        for (tank_addr, fluid) in [
            ("ae2:sky_tank_2", "minecraft:water"),
            ("ae2:sky_tank_21", "minecraft:water"),
            ("ae2:sky_tank_3", "minecraft:lava"),
            ("ae2:sky_tank_14", "minecraft:lava"),
            ("ae2:sky_tank_5", "thermal:ender"),
            ("ae2:sky_tank_6", "thermal:glowstone"),
            ("ae2:sky_tank_7", "thermal:redstone"),
            ("ae2:sky_tank_9", "thermal:crude_oil"),
            ("ae2:sky_tank_10", "thermal:light_oil"),
            ("ae2:sky_tank_11", "thermal:heavy_oil"),
            ("ae2:sky_tank_12", "thermal:refined_fuel"),
            ("ae2:sky_tank_19", "createchromaticreturn:refined_mixture"),
        ] {
            factory.add_fluid_storage(FluidStorageConfig {
                accesses: vec![TankAccess { client: s("1a"), tank_addr: s(tank_addr), fluid_bus_addrs: fbus.clone() }],
                fluid: s(fluid),
                capacity: 16_000,
            })
        }
        for (inv_addr, item, qty) in [
            ("projectexpansion:emc_link_4", "item.kubejs.cube4_packaged", 1),
            ("minecraft:barrel_18", "Uranium Ingot", 64),
            ("create:deployer_1", "Blaze Cake", 64),
            ("minecraft:barrel_17", "Fluorite", 64),
            ("minecraft:hopper_10", "Bio Fuel", 64),
            ("minecraft:barrel_32", "Coal", 64),
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
        for (tank_addr, fluid, qty) in [
            ("powah:reactor_part_0", "minecraft:water", 1_000),
            ("ae2:sky_tank_13", "minecraft:lava", 16_000),  // boiler
            ("ae2:sky_tank_17", "minecraft:water", 16_000), // phyto
            ("ae2:sky_tank_18", "minecraft:water", 16_000), // fissileFuel
        ] {
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
        for tank_addr in ["ae2:sky_tank_20"] {
            factory.add_process(FluidSlottedConfig {
                name: s("fluidOutput"),
                input_slots: vec![],
                input_tanks: vec![vec![]],
                accesses: facc(s(tank_addr)),
                to_extract: None,
                fluid_extract: fluid_extract_all(),
                strict_priority: false,
                recipes: vec![],
            })
        }
        for inv_addr in [
            "minecraft:barrel_2",  // mixer
            "minecraft:barrel_4",  // metallurgicInfuser
            "minecraft:barrel_7",  // enrichmentChamber
            "minecraft:barrel_11", // mainBelt
            "minecraft:barrel_20", // crusher
            "minecraft:barrel_24", // fissileFuelPellet
            "minecraft:barrel_29", // PRC
            "extendedcrafting:compressor_0",
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
                BufferedInput::new(label("Packed Ice"), 64),
                BufferedInput::new(label("Uraninite"), 64),
                BufferedInput::new(label("Coal"), 64),
            ],
        });
        factory.add_process(BufferedConfig {
            name: s("witherFarm"),
            accesses: acc(s("minecraft:barrel_26")),
            slot_filter: None,
            to_extract: None,
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![
                BufferedInput::new(label("Soul Sand"), 32),
                BufferedInput::new(label("Wither Skeleton Skull"), 32),
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
                    outputs: Output::new(label("Resonant Integral Components"), 4),
                    inputs: vec![
                        CraftingGridInput::new(label("Enderium Ingot"), vec![0, 2, 6, 8]),
                        CraftingGridInput::new(label("Hardened Glass"), vec![4]),
                        CraftingGridInput::new(label("item.kubejs.cube3"), vec![7]),
                        CraftingGridInput::new(label("Reinforced Integral Components"), vec![1]),
                        CraftingGridInput::new(label("Lumium Gear"), vec![3, 5]),
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
                    outputs: Output::new(label("Cogwheel"), 64),
                    inputs: vec![
                        CraftingGridInput::new(label("Shaft"), vec![0]),
                        CraftingGridInput::new(label("Oak Planks"), vec![1]),
                    ],
                    max_sets: 64,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Large Cogwheel"), 64),
                    inputs: vec![
                        CraftingGridInput::new(label("Shaft"), vec![0]),
                        CraftingGridInput::new(label("Oak Planks"), vec![1, 2]),
                    ],
                    max_sets: 64,
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
                    outputs: Output::new(label("Block Of Blazing Crystal"), 16),
                    inputs: vec![CraftingGridInput::new(label("Blazing Crystal"), (0..9).collect())],
                    max_sets: 64,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Bone Block"), 16),
                    inputs: vec![CraftingGridInput::new(label("Bone Meal"), (0..9).collect())],
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
                    outputs: Output::new(label("Niotic Capacitor"), 16),
                    inputs: vec![
                        CraftingGridInput::new(label("Basic Capacitor (Large)"), vec![4]),
                        CraftingGridInput::new(label("Dielectric Paste"), vec![0, 2, 6, 8]),
                        CraftingGridInput::new(label("Niotic Crystal"), vec![1, 3, 5, 7]),
                    ],
                    max_sets: 32,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Spirited Capacitor"), 16),
                    inputs: vec![
                        CraftingGridInput::new(label("Basic Capacitor (Large)"), vec![4]),
                        CraftingGridInput::new(label("Dielectric Paste"), vec![0, 2, 6, 8]),
                        CraftingGridInput::new(label("Spirited Crystal"), vec![1, 3, 5, 7]),
                    ],
                    max_sets: 32,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Nitro Capacitor"), 16),
                    inputs: vec![
                        CraftingGridInput::new(label("Basic Capacitor (Large)"), vec![4]),
                        CraftingGridInput::new(label("Dielectric Paste"), vec![0, 2, 6, 8]),
                        CraftingGridInput::new(label("Nitro Crystal"), vec![1, 3, 5, 7]),
                    ],
                    max_sets: 32,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Body Stone"), 1),
                    inputs: vec![
                        CraftingGridInput::new(label("Lapis Lazuli"), vec![4]),
                        CraftingGridInput::new(label("Sugar"), vec![0, 1, 2, 6, 7, 8]),
                        CraftingGridInput::new(label("Mobius Fuel"), vec![3, 5]),
                    ],
                    max_sets: 1,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Skeleton Skull"), 16),
                    inputs: vec![
                        CraftingGridInput::new(label("Body Stone"), vec![4]),
                        CraftingGridInput::new(label("Bone Block"), vec![0, 1, 2, 3, 5, 6, 7, 8]),
                    ],
                    max_sets: 64,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Sandstone"), 64),
                    inputs: vec![CraftingGridInput::new(label("Sand"), vec![0, 1, 3, 4])],
                    max_sets: 64,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Clay"), 64),
                    inputs: vec![CraftingGridInput::new(label("Clay Ball"), vec![0, 1, 3, 4])],
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
                    outputs: Output::new(label("Advanced Control Circuit"), 16),
                    inputs: vec![
                        CraftingGridInput::new(label("Basic Control Circuit"), vec![4]),
                        CraftingGridInput::new(label("Infused Alloy"), vec![3, 5]),
                    ],
                    max_sets: 64,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Elite Control Circuit"), 16),
                    inputs: vec![
                        CraftingGridInput::new(label("Advanced Control Circuit"), vec![4]),
                        CraftingGridInput::new(label("Reinforced Alloy"), vec![3, 5]),
                    ],
                    max_sets: 64,
                    non_consumables: vec![],
                },
                CraftingGridRecipe {
                    outputs: Output::new(label("Ultimate Control Circuit"), 16),
                    inputs: vec![
                        CraftingGridInput::new(label("Elite Control Circuit"), vec![4]),
                        CraftingGridInput::new(label("Atomic Alloy"), vec![3, 5]),
                    ],
                    max_sets: 64,
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
        let recipes = Rc::from_iter((METALS.iter().map(|x| (1000, x.ingot, x.dust))).chain([
            (1000, "Steel Ingot", "Steel Dust"),
            (64, "Sky Stone", "Sky Stone Dust"),
            (64, "Stone", "Cobblestone"),
            (64, "Charcoal", "Oak Log"),
            (64, "Glass", "Sand"),
        ]));
        for inv_addr in [
            "create:depot_0",
            "create:depot_1",
            "create:depot_5",
            "create:depot_8",
            "create:depot_9",
            "create:depot_10",
            "create:depot_11",
        ] {
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
        let recipes = [(64, "Nether Quartz", "Soul Sand"), (64, "Packed Ice", "Ice"), (64, "Dough", "Wheat Flour")];
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
        let recipes = [(64, "Netherrack", "Clay"), (64, "Egg", "Dough")];
        for inv_addr in ["create:depot_6", "create:depot_7"] {
            factory.add_process(BufferedConfig {
                name: s("soulFan"),
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
        for (depot, spout) in [
            ("create:depot_4", "create:spout_0"),
            ("create:depot_12", "create:spout_1"),
            ("create:depot_13", "create:spout_2"),
            ("create:depot_14", "create:spout_3"),
        ] {
            factory.add_process(FluidSlottedConfig {
                name: s("spout"),
                accesses: vec![InvTankAccess {
                    client: s("1a"),
                    inv_addrs: vec![s(depot)],
                    tank_addrs: vec![s(spout)],
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
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Blaze Cake"), 16),
                        inputs: vec![MultiInvSlottedInput::new(label("Blaze Cake Base"), vec![(0, 0, 1)])],
                        fluids: vec![FluidSlottedInput::new(s("minecraft:lava"), vec![(0, 250)])],
                        max_sets: 1,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Glowing Ingot"), 16),
                        inputs: vec![MultiInvSlottedInput::new(label("Brass Ingot"), vec![(0, 0, 1)])],
                        fluids: vec![FluidSlottedInput::new(s("minecraft:lava"), vec![(0, 1_000)])],
                        max_sets: 1,
                    },
                ],
            });
            factory.add_process(SlottedConfig {
                name: s("spoutOutput"),
                accesses: acc(s(depot)),
                input_slots: vec![],
                to_extract: Some(Box::new(|_, _, stack| {
                    stack.detail.label != "item.kubejs.dormant_effigy"
                        && stack.detail.label != "Blaze Cake Base"
                        && stack.detail.label != "Brass Ingot"
                })),
                strict_priority: false,
                recipes: vec![],
            })
        }
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
                    outputs: Output::new(label("Lumium Gear"), 16),
                    inputs: vec![SlottedInput::new(label("Lumium Ingot"), vec![(0, 4)])],
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
                        outputs: Output::new(label("item.kubejs.cube4_packaged"), 16),
                        inputs: vec![SlottedInput::new(label("item.kubejs.cube4"), vec![(0, 1)]).extra_backup(63)],
                        max_sets: 8,
                    },
                    SlottedRecipe {
                        outputs: Output::new(label("Electrum Plate"), 64),
                        inputs: vec![SlottedInput::new(label("Electrum Ingot"), vec![(0, 1)])],
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
                    outputs: Output::new(label("Invar Ingot"), 1000),
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
                        SlottedInput::new(label("Zinc Ingot"), vec![(0, 1)]),
                        SlottedInput::new(label("Copper Ingot"), vec![(1, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Electrum Ingot"), 1000),
                    inputs: vec![
                        SlottedInput::new(label("Gold Dust"), vec![(0, 1)]),
                        SlottedInput::new(label("Silver Dust"), vec![(1, 1)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Bronze Ingot"), 1000),
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
                    outputs: Output::new(label("Enderium Ingot"), 64),
                    inputs: vec![
                        SlottedInput::new(label("Diamond Dust"), vec![(0, 1)]),
                        SlottedInput::new(label("Lead Dust"), vec![(1, 3)]),
                        SlottedInput::new(label("Ender Pearl"), vec![(2, 2)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Lumium Ingot"), 64),
                    inputs: vec![
                        SlottedInput::new(label("Silver Dust"), vec![(0, 1)]),
                        SlottedInput::new(label("Tin Dust"), vec![(1, 3)]),
                        SlottedInput::new(label("Glowstone Dust"), vec![(2, 2)]),
                    ],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Hardened Glass"), 64),
                    inputs: vec![
                        SlottedInput::new(label("Nether Quartz"), vec![(0, 1)]),
                        SlottedInput::new(label("Obsidian"), vec![(1, 1)]),
                        SlottedInput::new(label("Sand"), vec![(2, 1)]),
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
        factory.add_process(FluidSlottedConfig {
            name: s("mixer"),
            accesses: vec![InvTankAccess {
                client: s("1a"),
                inv_addrs: vec![s("minecraft:barrel_3")],
                tank_addrs: vec![s("ae2:sky_tank_8")],
                bus_addr: s(BUS),
                fluid_bus_addrs: fbus.clone(),
            }],
            input_slots: vec![vec![0, 1, 2, 3, 4, 5]],
            input_tanks: vec![vec![0]],
            fluid_extract: None,
            to_extract: None,
            strict_priority: false,
            recipes: vec![
                FluidSlottedRecipe {
                    outputs: FluidOutput::new(s("createchromaticreturn:refined_mixture"), 8_000),
                    fluids: vec![FluidSlottedInput::new(s("minecraft:water"), vec![(0, 1_000)])],
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Gold Ingot"), vec![(0, 0, 8)]),
                        MultiInvSlottedInput::new(label("Mobius Fuel"), vec![(0, 1, 4)]),
                        MultiInvSlottedInput::new(label("Glowstone Dust"), vec![(0, 2, 8)]),
                        MultiInvSlottedInput::new(label("HDPE Sheet"), vec![(0, 3, 4)]),
                    ],
                    max_sets: 1,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Luminessence"), 64),
                    fluids: vec![FluidSlottedInput::new(s("thermal:glowstone"), vec![(0, 500)])],
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Redstone Dust"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Gunpowder"), vec![(0, 1, 1)]),
                    ],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Refined Radiance"), 64),
                    fluids: vec![FluidSlottedInput::new(s("createchromaticreturn:refined_mixture"), vec![(0, 100)])],
                    inputs: vec![MultiInvSlottedInput::new(label("Chromatic Compound"), vec![(0, 0, 1)])],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Andesite Alloy"), 64),
                    fluids: vec![],
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Andesite"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Iron Nugget"), vec![(0, 1, 1)]),
                    ],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Chromatic Compound"), 16),
                    fluids: vec![],
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Powdered Obsidian"), vec![(0, 0, 3)]),
                        MultiInvSlottedInput::new(label("Andesite Alloy"), vec![(0, 1, 3)]),
                        MultiInvSlottedInput::new(label("Glowing Ingot"), vec![(0, 2, 3)]),
                        MultiInvSlottedInput::new(label("Polished Rose Quartz"), vec![(0, 3, 3)]),
                    ],
                    max_sets: 2,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("item.kubejs.cube2"), 64),
                    fluids: vec![],
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Invar Gear"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("item.kubejs.cube1"), vec![(0, 1, 1)]),
                        MultiInvSlottedInput::new(label("item.kubejs.steel_gear"), vec![(0, 2, 1)]),
                        MultiInvSlottedInput::new(label("item.kubejs.bronze_rod"), vec![(0, 3, 2)]),
                    ],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("item.kubejs.cube3"), 64),
                    fluids: vec![],
                    inputs: vec![
                        MultiInvSlottedInput::new(label("item.kubejs.cube2"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Basic Control Circuit"), vec![(0, 1, 1)]),
                        MultiInvSlottedInput::new(label("Fire Charge"), vec![(0, 2, 1)]),
                        MultiInvSlottedInput::new(label("Ice Charge"), vec![(0, 3, 1)]),
                        MultiInvSlottedInput::new(label("Lightning Charge"), vec![(0, 4, 1)]),
                        MultiInvSlottedInput::new(label("Earth Charge"), vec![(0, 5, 1)]),
                    ],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("item.kubejs.cube4_inert"), 64),
                    fluids: vec![],
                    inputs: vec![
                        MultiInvSlottedInput::new(label("item.kubejs.cube3"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("item.kubejs.fission_pellet"), vec![(0, 1, 1)]),
                        MultiInvSlottedInput::new(label("Elite Control Circuit"), vec![(0, 2, 1)]),
                        MultiInvSlottedInput::new(label("Mobius Fuel"), vec![(0, 3, 1)]),
                    ],
                    max_sets: 8,
                },
            ],
        });
        for inv_addr in [
            "thermal:machine_pulverizer_0",
            "thermal:machine_pulverizer_1",
            "thermal:machine_pulverizer_2",
            "thermal:machine_pulverizer_3",
        ] {
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
                        outputs: Output::new(label("Diamond"), 1000),
                        inputs: vec![SlottedInput::new(label("Diamond Ore"), vec![(0, 1)])],
                        max_sets: 8,
                    },
                    SlottedRecipe {
                        outputs: Output::new(label("Emerald"), 1000),
                        inputs: vec![SlottedInput::new(label("Emerald Ore"), vec![(0, 1)])],
                        max_sets: 8,
                    },
                    SlottedRecipe {
                        outputs: Output::new(label("Bone Meal"), 64),
                        inputs: vec![SlottedInput::new(label("Bone"), vec![(0, 1)])],
                        max_sets: 8,
                    },
                    SlottedRecipe {
                        outputs: Output::new(label("Coal"), 1000),
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
                        outputs: Output::new(label("Lapis Lazuli"), 1000),
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
        for inv_addr in [
            "thermal:machine_insolator_0",
            "thermal:machine_insolator_1",
            "thermal:machine_insolator_2",
            "thermal:machine_insolator_3",
        ] {
            factory.add_process(SlottedConfig {
                name: s("phyto"),
                accesses: acc(s(inv_addr)),
                input_slots: vec![0],
                to_extract: extract_all(),
                strict_priority: false,
                recipes: vec![
                    SlottedRecipe {
                        outputs: Output::new(label("Wheat"), 64),
                        inputs: vec![SlottedInput::new(label("Wheat Seeds"), vec![(0, 1)])],
                        max_sets: 8,
                    },
                    SlottedRecipe {
                        outputs: Output::new(label("Slitake Spores"), 128),
                        inputs: vec![SlottedInput::new(label("Slitake Spores"), vec![(0, 1)])],
                        max_sets: 8,
                    },
                ],
            })
        }
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
            input_slots: vec![1, 2, 3, 4],
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
                    outputs: Output::new(label("Niotic Crystal"), 64),
                    inputs: vec![SlottedInput::new(label("Diamond"), vec![(1, 1)])],
                    max_sets: 1,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Spirited Crystal"), 64),
                    inputs: vec![SlottedInput::new(label("Emerald"), vec![(1, 1)])],
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
                SlottedRecipe {
                    outputs: Output::new(label("Nitro Crystal"), 64),
                    inputs: vec![
                        SlottedInput::new(label("Nether Star"), vec![(1, 1)]),
                        SlottedInput::new(label("Block of Redstone"), vec![(2, 1), (3, 1)]),
                        SlottedInput::new(label("Block Of Blazing Crystal"), vec![(4, 1)]),
                    ],
                    max_sets: 1,
                },
            ],
        });
        factory.add_process(SlottedConfig {
            name: s("packer"),
            accesses: acc(s("minecraft:barrel_14")),
            input_slots: vec![0, 1, 2],
            to_extract: None,
            strict_priority: false,
            recipes: vec![SlottedRecipe {
                outputs: Output::new(label("Blaze Cake Base"), 16),
                inputs: vec![
                    SlottedInput::new(label("Egg"), vec![(0, 1)]),
                    SlottedInput::new(label("Sugar"), vec![(1, 1)]),
                    SlottedInput::new(label("Cinder Flour"), vec![(2, 1)]),
                ],
                max_sets: 8,
            }],
        });
        let recipes = [
            ("Bronze", "Bronze Ingot"),
            ("Electrum", "Electrum Ingot"),
            ("Steel", "Steel Ingot"),
            ("Invar", "Invar Ingot"),
            ("Lapis Lazuli", "Lapis Lazuli"),
            ("Uranium", "Uranium Ingot"),
            ("Emerald", "Emerald"),
            ("Coal", "Coal"),
            ("Zinc", "Zinc Ingot"),
            ("Redstone", "Redstone Dust"),
            ("Aluminum", "Aluminium Ingot"),
            ("Silver", "Silver Ingot"),
            ("Tin", "Tin Ingot"),
            ("Diamond", "Diamond"),
            ("Iron", "Iron Ingot"),
            ("Copper", "Copper Ingot"),
            ("Lead", "Lead Ingot"),
            ("Nickel", "Nickel Ingot"),
            ("Glowstone", "Glowstone Dust"),
        ];
        factory.add_process(SlottedConfig {
            name: s("singularity"),
            accesses: acc(s("minecraft:barrel_25")),
            input_slots: (0..27).collect(),
            to_extract: None,
            strict_priority: false,
            recipes: (recipes.iter())
                .map(|(o, i)| SlottedRecipe {
                    outputs: Output::new(label!("{o} Singularity"), 1),
                    inputs: (0..26)
                        .map(|slot| SlottedInput::new(label(i), vec![(slot, 38)]))
                        .chain([SlottedInput::new(label(i), vec![(26, 12)])])
                        .collect(),
                    max_sets: 1,
                })
                .collect(),
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
                        outputs: Output::new(label("Enriched Obsidian"), 16),
                        inputs: vec![BufferedInput::new(label("Refined Obsidian Dust"), 1)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: Output::new(label("Enriched Redstone"), 16),
                        inputs: vec![BufferedInput::new(label("Redstone Dust"), 1)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: Output::new(label("Enriched Diamond"), 16),
                        inputs: vec![BufferedInput::new(label("Diamond"), 1)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: Output::new(label("Enriched Carbon"), 16),
                        inputs: vec![BufferedInput::new(label("Coal"), 1)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: Output::new(label("Obsidian Dust"), 16),
                        inputs: vec![BufferedInput::new(label("Obsidian"), 1)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: Output::new(label("Redstone Dust"), 1000),
                        inputs: vec![BufferedInput::new(label("Redstone Ore"), 1)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: Output::new(label("Fluorite"), 64),
                        inputs: vec![BufferedInput::new(label("Fluorite Ore"), 1)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: Output::new(label("Polished Rose Quartz"), 64),
                        inputs: vec![BufferedInput::new(label("Rose Quartz"), 1)],
                        max_inputs: i32::MAX,
                    },
                    BufferedRecipe {
                        outputs: Output::new(label("HDPE Sheet"), 64),
                        inputs: vec![BufferedInput::new(label("HDPE Pellet"), 3)],
                        max_inputs: i32::MAX,
                    },
                ])
                .collect(),
        });
        factory.add_process(BufferedConfig {
            name: s("fissileFuelPellet"),
            accesses: acc(s("minecraft:hopper_7")),
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![BufferedRecipe {
                outputs: Output::new(label("item.kubejs.fission_pellet"), 16),
                inputs: vec![BufferedInput::new(label("Fluorite Dust"), 1)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(BufferedConfig {
            name: s("substrate"),
            accesses: acc(s("minecraft:hopper_9")),
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![BufferedRecipe {
                outputs: Output::new(label("Substrate"), 64),
                inputs: vec![BufferedInput::new(label("Substrate"), 1).allow_backup()],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(BufferedConfig {
            name: s("HDPE"),
            accesses: acc(s("minecraft:hopper_11")),
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![BufferedRecipe {
                outputs: Output::new(label("HDPE Pellet"), 64),
                inputs: vec![BufferedInput::new(label("Substrate"), 1)],
                max_inputs: i32::MAX,
            }],
        });
        factory.add_process(BufferedConfig {
            name: s("crusher"),
            accesses: acc(s("minecraft:barrel_23")),
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 64,
            recipes: vec![
                BufferedRecipe {
                    outputs: Output::new(label("Gravel"), 64),
                    inputs: vec![BufferedInput::new(label("Cobblestone"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: Output::new(label("Sand"), 64),
                    inputs: vec![BufferedInput::new(label("Gravel"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: Output::new(label("Fluorite Dust"), 64),
                    inputs: vec![BufferedInput::new(label("Fluorite"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: Output::new(label("Diamond Dust"), 64),
                    inputs: vec![BufferedInput::new(label("Diamond"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: Output::new(label("Bio Fuel"), 64),
                    inputs: vec![BufferedInput::new(label("Sugar Cane"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(MultiInvSlottedConfig {
            name: s("metallurgicInfuser"),
            input_slots: vec![vec![0], vec![0]],
            accesses: vec![MultiInvAccess {
                client: s("1a"),
                inv_addrs: vec![s("minecraft:hopper_0"), s("minecraft:barrel_22")],
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
                    outputs: Output::new(label("Reinforced Alloy"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Enriched Diamond"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Infused Alloy"), vec![(1, 0, 4)]),
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
                MultiInvSlottedRecipe {
                    outputs: Output::new(label("Refined Obsidian Dust"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Enriched Diamond"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Obsidian Dust"), vec![(1, 0, 8)]),
                    ],
                    max_sets: 2,
                },
                MultiInvSlottedRecipe {
                    outputs: Output::new(label("item.kubejs.enriched_black_essence"), 16),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Enriched Obsidian"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("item.kubejs.black_essence"), vec![(1, 0, 2)]),
                    ],
                    max_sets: 4,
                },
                MultiInvSlottedRecipe {
                    outputs: Output::new(label("Wither Skeleton Skull"), 16),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("item.kubejs.enriched_black_essence"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Skeleton Skull"), vec![(1, 0, 5)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: Output::new(label("Black Iron Ingot"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("item.kubejs.enriched_black_essence"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Iron Ingot"), vec![(1, 0, 20)]),
                    ],
                    max_sets: 1,
                },
                MultiInvSlottedRecipe {
                    outputs: Output::new(label("Atomic Alloy"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Enriched Obsidian"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Reinforced Alloy"), vec![(1, 0, 2)]),
                    ],
                    max_sets: 4,
                },
                MultiInvSlottedRecipe {
                    outputs: Output::new(label("Rose Quartz"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Enriched Redstone"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Nether Quartz"), vec![(1, 0, 1)]),
                    ],
                    max_sets: 8,
                },
            ],
        });
        factory.add_process(MultiInvSlottedConfig {
            name: s("1-deployer"),
            input_slots: vec![vec![0], vec![0]],
            accesses: vec![MultiInvAccess {
                client: s("1a"),
                inv_addrs: vec![s("minecraft:hopper_5"), s("create:deployer_2")],
                bus_addr: s(BUS),
            }],
            to_extract: None,
            strict_priority: false,
            recipes: vec![
                MultiInvSlottedRecipe {
                    outputs: Output::new(label("Andesite Casing"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Stripped Oak Log"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Andesite Alloy"), vec![(1, 0, 1)]),
                    ],
                    max_sets: 8,
                },
                MultiInvSlottedRecipe {
                    outputs: Output::new(label("Brass Casing"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Stripped Oak Log"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Brass Ingot"), vec![(1, 0, 1)]),
                    ],
                    max_sets: 8,
                },
            ],
        });
        factory.add_process(MultiInvSlottedConfig {
            name: s("3-deployer-loop"),
            input_slots: vec![vec![26], vec![0], vec![0], vec![0]],
            accesses: vec![MultiInvAccess {
                client: s("1a"),
                inv_addrs: vec![
                    s("minecraft:barrel_12"),
                    s("create:deployer_3"),
                    s("create:deployer_4"),
                    s("create:deployer_5"),
                ],
                bus_addr: s(BUS),
            }],
            to_extract: None,
            strict_priority: false,
            recipes: vec![MultiInvSlottedRecipe {
                outputs: Output::new(label("Precision Mechanism"), 64),
                inputs: vec![
                    MultiInvSlottedInput::new(label("Golden Sheet"), vec![(0, 26, 1)]),
                    MultiInvSlottedInput::new(label("Cogwheel"), vec![(1, 0, 5)]),
                    MultiInvSlottedInput::new(label("Large Cogwheel"), vec![(2, 0, 5)]),
                    MultiInvSlottedInput::new(label("Iron Nugget"), vec![(3, 0, 5)]),
                ],
                max_sets: 4,
            }],
        });
        factory.add_process(MultiInvSlottedConfig {
            name: s("-deployer-loop"),
            input_slots: vec![vec![26], vec![0], vec![0], vec![0], vec![0], vec![0], vec![0], vec![0]],
            accesses: vec![MultiInvAccess {
                client: s("1a"),
                inv_addrs: vec![
                    s("minecraft:barrel_30"),
                    s("create:deployer_6"),
                    s("create:deployer_7"),
                    s("create:deployer_8"),
                    s("create:deployer_9"),
                    s("create:deployer_10"),
                    s("create:deployer_11"),
                    s("create:deployer_12"),
                ],
                bus_addr: s(BUS),
            }],
            to_extract: None,
            strict_priority: false,
            recipes: vec![MultiInvSlottedRecipe {
                outputs: Output::new(label("Final Star Shard"), 64),
                inputs: vec![
                    MultiInvSlottedInput::new(label("Nether Star"), vec![(0, 26, 1)]),
                    MultiInvSlottedInput::new(label("Refined Radiance"), vec![(1, 0, 20)]),
                    MultiInvSlottedInput::new(label("Nitro Crystal"), vec![(2, 0, 20)]),
                    MultiInvSlottedInput::new(label("Precision Mechanism"), vec![(3, 0, 20)]),
                    MultiInvSlottedInput::new(label("item.kubejs.cube1"), vec![(4, 0, 20)]),
                    MultiInvSlottedInput::new(label("item.kubejs.cube2"), vec![(5, 0, 20)]),
                    MultiInvSlottedInput::new(label("item.kubejs.cube3"), vec![(6, 0, 20)]),
                    MultiInvSlottedInput::new(label("item.kubejs.cube4"), vec![(7, 0, 20)]),
                ],
                max_sets: 2,
            }],
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
                    max_sets: 16,
                },
                FluidSlottedRecipe {
                    outputs: FluidOutput::new(s("thermal:ender"), 8_000),
                    inputs: vec![MultiInvSlottedInput::new(label("Ender Pearl"), vec![(0, 0, 1)])],
                    fluids: vec![],
                    max_sets: 16,
                },
                FluidSlottedRecipe {
                    outputs: FluidOutput::new(s("thermal:glowstone"), 8_000),
                    inputs: vec![MultiInvSlottedInput::new(label("Glowstone Dust"), vec![(0, 0, 1)])],
                    fluids: vec![],
                    max_sets: 16,
                },
                FluidSlottedRecipe {
                    outputs: FluidOutput::new(s("thermal:redstone"), 8_000),
                    inputs: vec![MultiInvSlottedInput::new(label("Redstone Dust"), vec![(0, 0, 1)])],
                    fluids: vec![],
                    max_sets: 16,
                },
                FluidSlottedRecipe {
                    outputs: FluidOutput::new(s("thermal:crude_oil"), 8_000),
                    inputs: vec![MultiInvSlottedInput::new(label("item.kubejs.oil_clump"), vec![(0, 0, 1)])],
                    fluids: vec![],
                    max_sets: 16,
                },
            ],
        });
        factory.add_process(FluidSlottedConfig {
            name: s("blastChiller"),
            input_slots: vec![vec![]],
            input_tanks: vec![vec![0]],
            accesses: facc(s("thermal:machine_chiller_0")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: None,
            strict_priority: false,
            recipes: vec![
                FluidSlottedRecipe {
                    outputs: Output::new(label("Ice"), 64),
                    fluids: vec![FluidSlottedInput::new(s("minecraft:water"), vec![(0, 1_000)])],
                    inputs: vec![],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Obsidian"), 64),
                    fluids: vec![FluidSlottedInput::new(s("minecraft:lava"), vec![(0, 1_000)])],
                    inputs: vec![],
                    max_sets: 8,
                },
            ],
        });
        for inv_addr in [
            "thermal:machine_refinery_0",
            "thermal:machine_refinery_1",
            "thermal:machine_refinery_2",
            "thermal:machine_refinery_3",
            "thermal:machine_refinery_4",
        ] {
            factory.add_process(FluidSlottedConfig {
                name: s("fractionatingStill"),
                input_slots: vec![vec![]],
                input_tanks: vec![vec![0]],
                accesses: facc(s(inv_addr)),
                to_extract: multi_inv_extract_all(),
                fluid_extract: fluid_extract_slots(|_, slot| slot >= 1),
                strict_priority: false,
                recipes: vec![
                    FluidSlottedRecipe {
                        outputs: FluidOutput::new(s("thermal:light_oil"), 4_000)
                            .and(FluidOutput::new(s("thermal:heavy_oil"), 4_000)),
                        fluids: vec![FluidSlottedInput::new(s("thermal:crude_oil"), vec![(0, 100)])],
                        inputs: vec![],
                        max_sets: 8,
                    },
                    FluidSlottedRecipe {
                        outputs: FluidOutput::new(s("thermal:refined_fuel"), 8_000)
                            .and(FluidOutput::new(s("thermal:light_oil"), 8_000).not()),
                        fluids: vec![FluidSlottedInput::new(s("thermal:light_oil"), vec![(0, 100)])],
                        inputs: vec![],
                        max_sets: 8,
                    },
                    FluidSlottedRecipe {
                        outputs: FluidOutput::new(s("thermal:refined_fuel"), 8_000)
                            .and(FluidOutput::new(s("thermal:heavy_oil"), 8_000).not()),
                        fluids: vec![FluidSlottedInput::new(s("thermal:heavy_oil"), vec![(0, 100)])],
                        inputs: vec![],
                        max_sets: 8,
                    },
                ],
            })
        }
        for inv_addr in [
            "thermal:machine_bottler_0",
            "thermal:machine_bottler_1",
            "thermal:machine_bottler_2",
            "thermal:machine_bottler_3",
            "thermal:machine_bottler_4",
            "thermal:machine_bottler_5",
            "thermal:machine_bottler_6",
            "thermal:machine_bottler_7",
            "thermal:machine_bottler_8",
            "thermal:machine_bottler_9",
            "thermal:machine_bottler_10",
        ] {
            factory.add_process(FluidSlottedConfig {
                name: s("bottler"),
                input_slots: vec![vec![0]],
                input_tanks: vec![vec![0]],
                accesses: facc(s(inv_addr)),
                to_extract: multi_inv_extract_all(),
                fluid_extract: None,
                strict_priority: false,
                recipes: vec![FluidSlottedRecipe {
                    outputs: Output::new(label("item.kubejs.cube4"), 64),
                    fluids: vec![FluidSlottedInput::new(s("thermal:refined_fuel"), vec![(0, 1_000)])],
                    inputs: vec![MultiInvSlottedInput::new(label("item.kubejs.cube4_inert"), vec![(0, 0, 1)])],
                    max_sets: 2,
                }],
            })
        }
        factory.add_process(BufferedConfig {
            name: s("mechanicalSaw"),
            accesses: acc(s("minecraft:hopper_4")),
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![
                BufferedRecipe {
                    outputs: Output::new(label("Stripped Oak Log"), 64),
                    inputs: vec![BufferedInput::new(label("Oak Log"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: Output::new(label("Shaft"), 64),
                    inputs: vec![BufferedInput::new(label("Andesite Alloy"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: s("mechanicalPress"),
            accesses: acc(s("minecraft:hopper_6")),
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 16,
            recipes: vec![
                BufferedRecipe {
                    outputs: Output::new(label("Golden Sheet"), 64),
                    inputs: vec![BufferedInput::new(label("Gold Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: Output::new(label("Invar Plate"), 64),
                    inputs: vec![BufferedInput::new(label("Invar Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: Output::new(label("Brass Sheet"), 64),
                    inputs: vec![BufferedInput::new(label("Brass Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: Output::new(label("Iron Sheet"), 64),
                    inputs: vec![BufferedInput::new(label("Iron Ingot"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: s("crushingWheels"),
            accesses: acc(s("minecraft:barrel_13")),
            slot_filter: None,
            to_extract: None,
            stocks: vec![],
            max_recipe_inputs: 64,
            recipes: vec![
                BufferedRecipe {
                    outputs: Output::new(label("Sugar"), 64),
                    inputs: vec![BufferedInput::new(label("Sugar Cane"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: Output::new(label("Cinder Flour"), 64),
                    inputs: vec![BufferedInput::new(label("Netherrack"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: Output::new(label("Wheat Flour"), 64),
                    inputs: vec![BufferedInput::new(label("Wheat"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: Output::new(label("Powdered Obsidian"), 64),
                    inputs: vec![BufferedInput::new(label("Obsidian"), 1)],
                    max_inputs: i32::MAX,
                },
                BufferedRecipe {
                    outputs: Output::new(label("Sky Stone Dust"), 64),
                    inputs: vec![BufferedInput::new(label("Sky Stone"), 1)],
                    max_inputs: i32::MAX,
                },
            ],
        });
        factory.add_process(BufferedConfig {
            name: s("crafterA"),
            accesses: vec![BusAccess { client: s("1a"), inv_addr: s("rftoolsutility:crafter2_0"), bus_addr: s(BUS) }],
            slot_filter: Some(Box::new(|slot| slot >= 10 && slot < 36)),
            to_extract: None,
            recipes: (0..16)
                .map(|_| BufferedRecipe {
                    outputs: Output::new(label("item.kubejs.dormant_effigy"), 16),
                    inputs: vec![BufferedInput::new(label("Soul Stone"), 1), BufferedInput::new(label("Calcite"), 4)],
                    max_inputs: i32::MAX,
                })
                .chain([BufferedRecipe {
                    outputs: Output::new(label("Soul Stone"), 16),
                    inputs: vec![
                        BufferedInput::new(label("Redstone Dust"), 1),
                        BufferedInput::new(label("Lapis Lazuli"), 2),
                        BufferedInput::new(label("Glowstone Dust"), 6),
                    ],
                    max_inputs: i32::MAX,
                }])
                .collect(),
            max_recipe_inputs: 64 * 8,
            stocks: vec![],
        });
        for (n_wanted, inv_addr, item) in [
            (64, "projectexpansion:emc_link_1", "Raw Copper"),
            (64, "projectexpansion:emc_link_38", "Raw Iron"),
            (64, "projectexpansion:emc_link_3", "Raw Bauxite"),
            (64, "projectexpansion:emc_link_4", "Cobblestone"),
            (64, "projectexpansion:emc_link_9", "Oak Log"),
            (64, "projectexpansion:emc_link_7", "Raw Gold"),
            (64, "projectexpansion:emc_link_8", "Raw Nickel"),
            (64, "projectexpansion:emc_link_40", "Coal Ore"),
            (64, "projectexpansion:emc_link_11", "Raw Tin"),
            (64, "projectexpansion:emc_link_13", "Raw Zinc"),
            (64, "projectexpansion:emc_link_14", "Redstone Ore"),
            (64, "projectexpansion:emc_link_15", "Raw Silver"),
            (64, "projectexpansion:emc_link_16", "Raw Lead"),
            (64, "projectexpansion:emc_link_17", "Raw Osmium"),
            (64, "projectexpansion:emc_link_18", "Soul Sand"),
            (64, "projectexpansion:emc_link_25", "Clay Ball"),
            (64, "projectexpansion:emc_link_26", "Raw Uranium"),
            (64, "projectexpansion:emc_link_27", "Lapis Lazuli Ore"),
            (1000, "projectexpansion:emc_link_28", "Glowstone Dust"),
            (64, "projectexpansion:emc_link_29", "Bone"),
            (64, "projectexpansion:emc_link_30", "Diamond Ore"),
            (64, "projectexpansion:emc_link_32", "Emerald Ore"),
            (64, "projectexpansion:emc_link_33", "item.kubejs.black_essence"),
            (64, "projectexpansion:emc_link_34", "Mobius Fuel"),
            (64, "projectexpansion:emc_link_35", "Sugar Cane"),
            (64, "projectexpansion:emc_link_36", "item.kubejs.oil_clump"),
            (64, "projectexpansion:emc_link_37", "Fluorite Ore"),
            (64, "minecraft:barrel_27", "Nether Star"),
        ] {
            factory.add_process(BlockingOutputConfig {
                accesses: acc(s(inv_addr)),
                slot_filter: None,
                outputs: vec![Output { item: label(item), n_wanted }],
            })
        }
        factory.add_process(BlockingFluidOutputConfig {
            accesses: vec![TankAccess {
                client: s("1a"),
                tank_addr: s("thermal:device_water_gen_0"),
                fluid_bus_addrs: fbus.clone(),
            }],
            outputs: vec![FluidOutput { fluid: s("minecraft:water"), n_wanted: 24_000 }],
        });
    })
}
