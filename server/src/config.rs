use crate::factory::{Factory, FactoryConfig, FluidStorageConfig};
use crate::{access::*, config_util::*, item::Filter, process::*, recipe::*, storage::*};
use crate::{detail_cache::DetailCache, server::Server, Tui};
use flexstr::{local_fmt, LocalStr};
use fnv::{FnvHashMap, FnvHashSet};
use std::{cell::RefCell, rc::Rc, time::Duration};

const MAIN: &str = "main";
const BUS: &str = "minecraft:barrel_6";

fn fbus() -> Vec<LocalStr> {
    vec![
        s("enderio:pressurized_fluid_tank_4"),
        s("enderio:pressurized_fluid_tank_5"),
        s("enderio:pressurized_fluid_tank_6"),
        s("enderio:pressurized_fluid_tank_2"),
    ]
}

fn acc(inv_addr: LocalStr) -> Vec<BusAccess> { vec![BusAccess { client: s(MAIN), inv_addr, bus_addr: s(BUS) }] }
fn tank(tank_addr: LocalStr) -> Vec<TankAccess> { vec![TankAccess { client: s(MAIN), tank_addr, fluid_bus_addrs: fbus() }] }
fn inv_tank(addr: LocalStr) -> Vec<InvTankAccess> {
    vec![InvTankAccess { client: s(MAIN), inv_addrs: vec![addr.clone()], tank_addrs: vec![addr], bus_addr: s(BUS), fluid_bus_addrs: fbus() }]
}

fn ore_variants(x: &str) -> Filter {
    let variants = [
        local_fmt!("Raw {x}"),
        local_fmt!("{x} Ore"),
        local_fmt!("End {x} Ore"),
        local_fmt!("Talc {x} Ore"),
        local_fmt!("Nether {x} Ore"),
        local_fmt!("Marble {x} Ore"),
        local_fmt!("Deepslate {x} Ore"),
    ];
    custom(local_fmt!("Any {x} Ore"), move |_, x| variants.iter().any(|y| y == x.label))
}

fn cold_metal_output(x: &str) -> Rc<dyn Outputs> { Output::new(label!("{x} Dust"), 65).or(Output::new(label!("{x} Ingot"), 65)) }

fn less_than(x: Filter, y: Filter) -> Rc<dyn Outputs> {
    Rc::new(move |f: &Factory| {
        let x = f.search_n_stored(&x);
        let y = f.search_n_stored(&y);
        (!(x < y)).then_some(1.)
    })
}

pub fn build_factory(tui: Rc<Tui>) -> Rc<RefCell<Factory>> {
    let cold_metals = ["Tantalum", "Gallium", "Silver", "Copper", "Gold", "Tin"];
    let washed_ores = ["Tricalcium Phosphate", "Apatite", "Chromite", "Uraninite", "Coal"];
    let mercury_bathed_ores = ["Silver", "Chalcopyrite"];
    let persulfate_bathed_ores = ["Cobaltite"];
    let all_ores = FnvHashSet::from_iter(washed_ores.into_iter().chain(mercury_bathed_ores).chain(persulfate_bathed_ores));
    let refined_ores = all_ores.clone();
    let washing_byproducts: FnvHashMap<_, _> = [("Chalcopyrite", cold_metal_output("Gold"))].into_iter().collect();
    let ore_final_outputs: Box<_> = ["Tricalcium Phosphate", "Apatite", "Chromite", "Uraninite", "Coal", "Chalcopyrite", "Cobaltite"]
        .into_iter()
        .map(|x| (x, Output::new(label!("{x} Dust"), 16)))
        .chain(["Silver"].into_iter().map(|x| (x, cold_metal_output(x))))
        .collect();
    let ore_washing_outputs = |x: &str| {
        let mut out = Output::new(label!("Purified {x} Ore"), 16);
        if let Some(x) = washing_byproducts.get(x) {
            out = out.and(x.clone());
        }
        out
    };
    FactoryConfig {
        tui: tui.clone(),
        detail_cache: DetailCache::new(&tui, s("detail_cache.txt")),
        server: Server::new(tui, 1850),
        min_cycle_time: Duration::from_secs(1),
        log_clients: vec![s(MAIN)],
        bus_accesses: vec![BasicAccess { client: s(MAIN), addr: s(BUS) }],
        fluid_bus_accesses: vec![FluidAccess { client: s(MAIN), fluid_bus_addrs: fbus() }],
        fluid_bus_capacity: 32_000,
        backups: vec![],
        fluid_backups: vec![],
    }
    .build(|factory| {
        factory.add_process(LowAlert::new(label("End Stone"), 64));
        factory.add_process(LowAlert::new(label("Lead Dust"), 64));
        factory.add_process(LowAlert::new(label("Iron Dust"), 64));
        factory.add_process(LowAlert::new(label("Zinc Dust"), 64));
        factory.add_process(LowAlert::new(label("Netherrack"), 64));
        factory.add_process(LowAlert::new(label("Nickel Dust"), 64));
        factory.add_process(LowAlert::new(label("Sulfur Dust"), 64));
        factory.add_process(LowAlert::new(label("Bauxite Dust"), 64));
        factory.add_process(LowAlert::new(label("Diamond Dust"), 64));
        factory.add_process(LowAlert::new(label("Nether Quartz"), 64));
        factory.add_process(LowAlert::new(label("Antimony Dust"), 64));
        factory.add_process(LowAlert::new(label("Redstone Dust"), 64));
        factory.add_process(LowAlert::new(label("Platinum Ingot"), 64));
        factory.add_process(LowAlert::new(label("Glowstone Dust"), 64));
        factory.add_process(LowAlert::new(label("Potassium Dust"), 64));
        factory.add_process(LowAlert::new(label("Aluminium Dust"), 64));
        factory.add_process(LowAlert::new(label("Manganese Dust"), 64));
        factory.add_process(LowAlert::new(label("Silicon Dioxide Dust"), 64));
        for x in ["Tin", "Gallium", "Tantalum"] {
            factory.add_process(LowAlert::new(
                custom(local_fmt!("{x} Ingot/Dust"), move |_, y| {
                    let Some(suffix) = y.label.strip_prefix(x) else { return false };
                    suffix == " Ingot" || suffix == " Dust"
                }),
                65,
            ));
        }
        for x in ["Coal", "Silver", "Chromite", "Uraninite", "Apatite", "Tricalcium Phosphate", "Cobaltite"] {
            factory.add_process(LowAlert::new(ore_variants(x), 64));
        }
        factory.add_process(ManualUiConfig { accesses: acc(s("minecraft:barrel_4")) });
        for i in [4, 5, 1, 3] {
            factory.add_storage(ChestConfig { accesses: acc(local_fmt!("gtceu:titanium_crate_{i}")), override_max_stack_size: None });
        }
        for (capacity, addr, fluid) in [
            (32_000 * 8 * 8, "functionalstorage:fluid_1_0", "gtceu:oxygen"),
            (32_000 * 8, "functionalstorage:fluid_1_41", "gtceu:styrene_butadiene_rubber"),
            (32_000 * 8 * 8, "functionalstorage:fluid_1_2", "gtceu:hydrogen"),
            (32_000 * 8, "functionalstorage:fluid_1_3", "gtceu:hydrogen_sulfide"),
            (32_000 * 8, "functionalstorage:fluid_1_4", "gtceu:sulfuric_acid"),
            (32_000 * 8, "functionalstorage:fluid_1_34", "gtceu:oxalic_acid_solution"),
            (32_000 * 8, "functionalstorage:fluid_1_6", "gtceu:polyethylene"),
            (32_000 * 8, "functionalstorage:fluid_1_7", "gtceu:chlorine"),
            (32_000 * 8 * 8, "functionalstorage:fluid_1_8", "minecraft:water"),
            (32_000 * 8, "functionalstorage:fluid_1_33", "gtceu:glowstone"),
            (32_000 * 8, "functionalstorage:fluid_1_24", "gtceu:soldering_alloy"),
            (32_000 * 8, "functionalstorage:fluid_1_11", "gtceu:polyvinyl_chloride"),
            (32_000 * 8, "functionalstorage:fluid_1_35", "gtceu:elemental_reduction_fluid"),
            (32_000 * 8, "functionalstorage:fluid_1_13", "gtceu:sodium_persulfate"),
            (32_000 * 8, "functionalstorage:fluid_1_36", "gtceu:ice"),
            (32_000 * 8, "functionalstorage:fluid_1_15", "gtceu:hydrofluoric_acid"),
            (32_000 * 8, "functionalstorage:fluid_1_16", "gtceu:hexafluorosilicic_acid"),
            (32_000 * 8, "functionalstorage:fluid_1_17", "gtceu:mercury"),
            (32_000 * 8, "functionalstorage:fluid_1_37", "gtceu:mana"),
            (32_000 * 8, "functionalstorage:fluid_1_19", "gtceu:epoxy"),
            (32_000 * 8, "functionalstorage:fluid_1_25", "gtceu:salt_water"),
            (32_000 * 8 * 8, "functionalstorage:fluid_1_26", "gtceu:hydrochloric_acid"),
            (32_000 * 8, "functionalstorage:fluid_1_27", "gtceu:distilled_water"),
            (32_000 * 8, "functionalstorage:fluid_1_28", "gtceu:titanium_tetrachloride"),
            (32_000 * 8, "functionalstorage:fluid_1_29", "gtceu:carbon_monoxide"),
            (32_000 * 8, "functionalstorage:fluid_1_30", "gtceu:nitrogen"),
            (32_000 * 8, "functionalstorage:fluid_1_31", "gtceu:lubricant"),
            (32_000 * 8, "functionalstorage:fluid_1_32", "gtceu:polytetrafluoroethylene"),
            (32_000 * 8, "functionalstorage:fluid_1_38", "gtceu:phosphoric_acid"),
            (32_000 * 8, "functionalstorage:fluid_1_39", "gtceu:helium"),
            (32_000 * 8, "functionalstorage:fluid_1_40", "gtceu:sulfur_dioxide"),
            (32_000 * 8, "functionalstorage:fluid_1_42", "gtceu:glue"),
        ] {
            factory.add_fluid_storage(FluidStorageConfig { accesses: tank(s(addr)), fluid: s(fluid), capacity });
        }
        for addr in [
            "gtceu:mv_output_bus_3", // ebf
            "gtceu:mv_output_bus_2", // freezer
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
            "gtceu:mv_output_hatch_3", // ebf
            "gtceu:mv_output_hatch_0", // freezer
        ] {
            factory.add_process(FluidSlottedConfig {
                name: s("fluidOutput"),
                accesses: inv_tank(s(addr)),
                input_slots: vec![],
                input_tanks: vec![vec![]],
                to_extract: None,
                fluid_extract: fluid_extract_all(),
                recipes: vec![],
                strict_priority: false,
            });
        }
        for (addr, fluid, qty) in [
            // ("gtceu:hv_combustion_4", "gtceu:high_octane_gasoline", 16_000),
            // ("gtceu:hv_combustion_5", "gtceu:high_octane_gasoline", 16_000),
            // ("gtceu:hv_combustion_6", "gtceu:high_octane_gasoline", 16_000),
            // ("gtceu:hv_combustion_7", "gtceu:high_octane_gasoline", 16_000),
        ] {
            factory.add_process(FluidSlottedConfig {
                name: s("fluidStock"),
                input_slots: vec![],
                input_tanks: vec![vec![0]],
                accesses: inv_tank(s(addr)),
                to_extract: None,
                fluid_extract: None,
                recipes: vec![FluidSlottedRecipe {
                    outputs: ignore_outputs(1.),
                    inputs: vec![],
                    fluids: vec![FluidSlottedInput::new(s(fluid), vec![(0, 1)])],
                    max_sets: qty,
                }],
                strict_priority: false,
            });
        }
        factory.add_process(BufferedConfig {
            name: s("export"),
            accesses: acc(s("minecraft:barrel_5")),
            slot_filter: None,
            to_extract: None,
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![
                BufferedInput::new(label("Microchip Processor"), 64),
                BufferedInput::new(label("Microprocessor"), 64),
                BufferedInput::new(label("Nanoprocessor"), 64),
                BufferedInput::new(label("Nanoprocessor Assembly"), 16),
                BufferedInput::new(label("Nanoprocessor Supercomputer"), 16),
                BufferedInput::new(label("Nickel Zinc Ferrite Ring"), 64),
                BufferedInput::new(label("Plastic Printed Circuit Board"), 64),
                BufferedInput::new(label("Epoxy Printed Circuit Board"), 64),
                BufferedInput::new(label("SMD Resistor"), 64),
                BufferedInput::new(label("SMD Capacitor"), 64),
                BufferedInput::new(label("SMD Transistor"), 64),
                BufferedInput::new(label("SMD Inductor"), 64),
                BufferedInput::new(label("Chromium Ingot"), 64),
                BufferedInput::new(label("Phosphorus Dust"), 64),
            ],
        });
        factory.add_process(SlottedConfig {
            name: s("microverse-chalcopyrite"),
            accesses: acc(s("gtceu:mv_input_bus_4")),
            input_slots: (0..9).collect(),
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: ignore_outputs(1.),
                inputs: vec![
                    SlottedInput::new(label("item.kubejs.microminer_t1"), vec![(0, 1)]),
                    SlottedInput::new(label("item.kubejs.basic_drilling_kit"), vec![(1, 1)]),
                    SlottedInput::new(label("Bronze Drill Head"), vec![(2, 1)]),
                ],
                max_sets: 1,
            }],
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("microverse-galena"),
            accesses: acc(s("gtceu:mv_input_bus_7")),
            input_slots: (0..9).collect(),
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: ignore_outputs(1.),
                inputs: vec![
                    SlottedInput::new(label("item.kubejs.microminer_t1"), vec![(0, 1)]),
                    SlottedInput::new(label("item.kubejs.basic_drilling_kit"), vec![(1, 1)]),
                    SlottedInput::new(label("Sterling Silver Drill Head"), vec![(2, 1)]),
                ],
                max_sets: 1,
            }],
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("microverse-goethite"),
            accesses: acc(s("gtceu:mv_input_bus_8")),
            input_slots: (0..9).collect(),
            to_extract: None,
            recipes: vec![SlottedRecipe {
                outputs: ignore_outputs(1.),
                inputs: vec![
                    SlottedInput::new(label("item.kubejs.microminer_t1"), vec![(0, 1)]),
                    SlottedInput::new(label("item.kubejs.basic_drilling_kit"), vec![(1, 1)]),
                    SlottedInput::new(label("Invar Drill Head"), vec![(2, 1)]),
                ],
                max_sets: 1,
            }],
            strict_priority: false,
        });
        for i in [2, 3] {
            factory.add_process(SlottedConfig {
                name: s("macerator"),
                accesses: acc(local_fmt!("gtceu:hv_macerator_{i}")),
                input_slots: vec![0],
                to_extract: extract_all(),
                recipes: (all_ores.iter())
                    .map(|&x| SlottedRecipe {
                        outputs: Output::new(label!("Crushed {x} Ore"), 16),
                        inputs: vec![SlottedInput::new(ore_variants(x), vec![(0, 1)])],
                        max_sets: 2,
                    })
                    .chain(ore_final_outputs.iter().map(|(i, o)| SlottedRecipe {
                        outputs: o.clone(),
                        inputs: vec![SlottedInput::new(label!("Refined {i} Ore"), vec![(0, 1)])],
                        max_sets: 2,
                    }))
                    .chain(cold_metals.iter().map(|&x| SlottedRecipe {
                        outputs: Output::new(label!("{x} Dust"), 64),
                        inputs: vec![SlottedInput::new(label!("{x} Ingot"), vec![(0, 1)]).extra_backup(64)],
                        max_sets: 8,
                    }))
                    .chain([
                        SlottedRecipe { outputs: ignore_outputs(2.), inputs: vec![SlottedInput::new(label("Apatite"), vec![(0, 1)])], max_sets: 8 },
                        SlottedRecipe {
                            outputs: Output::new(label("Crushed Ice"), 16),
                            inputs: vec![SlottedInput::new(label("Ice"), vec![(0, 1)])],
                            max_sets: 8,
                        },
                        SlottedRecipe {
                            outputs: Output::new(label("Obsidian Dust"), 16),
                            inputs: vec![SlottedInput::new(label("Obsidian"), vec![(0, 1)])],
                            max_sets: 8,
                        },
                        SlottedRecipe {
                            outputs: Output::new(label("Endstone Dust"), 16),
                            inputs: vec![SlottedInput::new(label("End Stone"), vec![(0, 1)])],
                            max_sets: 8,
                        },
                        SlottedRecipe {
                            outputs: Output::new(label("Netherrack Dust"), 16),
                            inputs: vec![SlottedInput::new(label("Netherrack"), vec![(0, 1)])],
                            max_sets: 8,
                        },
                    ])
                    .collect(),
                strict_priority: false,
            });
        }
        factory.add_process(FluidSlottedConfig {
            name: s("oreWasher"),
            input_slots: vec![vec![0, 1]],
            input_tanks: vec![vec![0]],
            accesses: inv_tank(s("gtceu:hv_ore_washer_1")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: None,
            recipes: (washed_ores.iter())
                .map(|&x| FluidSlottedRecipe {
                    outputs: ore_washing_outputs(x),
                    inputs: vec![MultiInvSlottedInput::new(label!("Crushed {x} Ore"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:distilled_water"), vec![(0, 100)])],
                    max_sets: 8,
                })
                .collect(),
            strict_priority: false,
        });
        for i in [1, 2] {
            factory.add_process(SlottedConfig {
                name: s("thermalCentrifuge"),
                accesses: acc(local_fmt!("gtceu:hv_thermal_centrifuge_{i}")),
                input_slots: vec![0],
                to_extract: extract_all(),
                recipes: (refined_ores.iter())
                    .map(|&x| SlottedRecipe {
                        outputs: Output::new(label!("Refined {x} Ore"), 16),
                        inputs: vec![SlottedInput::new(label!("Purified {x} Ore"), vec![(0, 1)])],
                        max_sets: 2,
                    })
                    .collect(),
                strict_priority: false,
            });
        }
        factory.add_process(FluidSlottedConfig {
            name: s("ebf-2/none"),
            input_slots: vec![(0..9).collect(), (0..9).collect()],
            input_tanks: vec![vec![0]],
            accesses: vec![InvTankAccess {
                client: s(MAIN),
                inv_addrs: vec![s("gtceu:mv_input_bus_6"), s("gtceu:mv_input_bus_5")],
                tank_addrs: vec![s("gtceu:mv_input_hatch_4")],
                bus_addr: s(BUS),
                fluid_bus_addrs: fbus(),
            }],
            to_extract: None,
            fluid_extract: None,
            recipes: vec![
                FluidSlottedRecipe {
                    outputs: Output::new(label("Phosphorus-doped Monocrystalline Silicon Boule"), 1),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Silicon Dust"), vec![(1, 0, 64)]),
                        MultiInvSlottedInput::new(label("Phosphorus Dust"), vec![(1, 1, 8)]),
                        MultiInvSlottedInput::new(label("Small Pile of Gallium Arsenide Dust"), vec![(1, 2, 2)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:nitrogen"), vec![(0, 8_000)])],
                    max_sets: 1,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Arsenic Trioxide Dust"), 16),
                    inputs: vec![MultiInvSlottedInput::new(label("Cobaltite Dust"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:oxygen"), vec![(0, 3_000)])],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Nickel Zinc Ferrite Ingot"), 64),
                    inputs: vec![MultiInvSlottedInput::new(label("Ferrite Mixture Dust"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:oxygen"), vec![(0, 2_000)])],
                    max_sets: 2,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Chromium Ingot"), 64),
                    inputs: vec![MultiInvSlottedInput::new(label("Chromium Dust"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:nitrogen"), vec![(0, 1_000)])],
                    max_sets: 4,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Stainless Steel Ingot"), 64),
                    inputs: vec![MultiInvSlottedInput::new(label("Stainless Steel Dust"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:nitrogen"), vec![(0, 1_000)])],
                    max_sets: 4,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Sterling Silver Ingot"), 64),
                    inputs: vec![MultiInvSlottedInput::new(label("Sterling Silver Dust"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:nitrogen"), vec![(0, 1_000)])],
                    max_sets: 4,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Hot material.gtceu.lumium Ingot"), 1),
                    inputs: vec![MultiInvSlottedInput::new(label("material.gtceu.lumium Dust"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:helium"), vec![(0, 100)])],
                    max_sets: 1,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Hot Nichrome Ingot"), 1),
                    inputs: vec![MultiInvSlottedInput::new(label("Nichrome Dust"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:nitrogen"), vec![(0, 1_000)])],
                    max_sets: 1,
                },
            ],
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("freezer"),
            input_slots: vec![(0..9).collect()],
            input_tanks: vec![vec![0]],
            accesses: vec![InvTankAccess {
                client: s(MAIN),
                inv_addrs: vec![s("gtceu:mv_input_bus_3")],
                tank_addrs: vec![s("gtceu:mv_input_hatch_3")],
                bus_addr: s(BUS),
                fluid_bus_addrs: fbus(),
            }],
            to_extract: None,
            fluid_extract: None,
            recipes: vec![
                FluidSlottedRecipe {
                    outputs: FluidOutput::new(s("gtceu:ice"), 64_000),
                    inputs: vec![],
                    fluids: vec![FluidSlottedInput::new(s("minecraft:water"), vec![(0, 1_000)])],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("material.gtceu.lumium Ingot"), 64),
                    inputs: vec![MultiInvSlottedInput::new(label("Hot material.gtceu.lumium Ingot"), vec![(0, 0, 1)])],
                    fluids: vec![],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Nichrome Ingot"), 64),
                    inputs: vec![MultiInvSlottedInput::new(label("Hot Nichrome Ingot"), vec![(0, 0, 1)])],
                    fluids: vec![],
                    max_sets: 8,
                },
            ],
            strict_priority: false,
        });
        let reactor_any = || {
            [
                FluidSlottedRecipe {
                    outputs: FluidOutput::new(s("gtceu:elemental_reduction_fluid"), 64_000),
                    inputs: vec![MultiInvSlottedInput::new(label("item.kubejs.pulsating_dust"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:hydrofluoric_acid"), vec![(0, 1_000)])],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: FluidOutput::new(s("gtceu:hydrogen_sulfide"), 64_000),
                    inputs: vec![MultiInvSlottedInput::new(label("Sulfur Dust"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:hydrogen"), vec![(0, 2_000)])],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: FluidOutput::new(s("gtceu:hydrochloric_acid"), 64_000),
                    inputs: vec![],
                    fluids: vec![
                        FluidSlottedInput::new(s("gtceu:hydrogen"), vec![(0, 1_000)]),
                        FluidSlottedInput::new(s("gtceu:chlorine"), vec![(0, 1_000)]),
                    ],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: FluidOutput::new(s("gtceu:phosphoric_acid"), 64_000),
                    inputs: vec![MultiInvSlottedInput::new(label("Phosphorus Pentoxide Dust"), vec![(0, 0, 14)])],
                    fluids: vec![FluidSlottedInput::new(s("minecraft:water"), vec![(0, 6_000)])],
                    max_sets: 2,
                },
                FluidSlottedRecipe {
                    outputs: FluidOutput::new(s("gtceu:hexafluorosilicic_acid"), 64_000),
                    inputs: vec![MultiInvSlottedInput::new(label("Silicon Dioxide Dust"), vec![(0, 0, 3)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:hydrofluoric_acid"), vec![(0, 6_000)])],
                    max_sets: 2,
                },
                FluidSlottedRecipe {
                    outputs: FluidOutput::new(s("gtceu:titanium_tetrachloride"), 64_000),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Carbon Dust"), vec![(0, 0, 2)]),
                        MultiInvSlottedInput::new(label("Rutile Dust"), vec![(0, 1, 1)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:chlorine"), vec![(0, 4_000)])],
                    max_sets: 4,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("item.kubejs.energized_clathrate"), 4),
                    inputs: vec![MultiInvSlottedInput::new(label("Nether Quartz"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:glowstone"), vec![(0, 288)])],
                    max_sets: 4,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Plastic Circuit Board"), 16),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Polytetrafluoroethylene Sheet"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Copper Foil"), vec![(0, 1, 4)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:hexafluorosilicic_acid"), vec![(0, 30)])],
                    max_sets: 4,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Plastic Printed Circuit Board"), 16),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Plastic Circuit Board"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Copper Foil"), vec![(0, 1, 6)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:oxalic_acid_solution"), vec![(0, 125)])],
                    max_sets: 4,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Epoxy Circuit Board"), 16),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Epoxy Sheet"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Gold Foil"), vec![(0, 1, 8)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:hexafluorosilicic_acid"), vec![(0, 60)])],
                    max_sets: 4,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Epoxy Printed Circuit Board"), 16),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Epoxy Circuit Board"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Electrum Foil"), vec![(0, 1, 8)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:oxalic_acid_solution"), vec![(0, 250)])],
                    max_sets: 4,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Saltpeter Dust"), 16),
                    inputs: vec![MultiInvSlottedInput::new(label("Potassium Dust"), vec![(0, 0, 1)])],
                    fluids: vec![
                        FluidSlottedInput::new(s("gtceu:oxygen"), vec![(0, 3_000)]),
                        FluidSlottedInput::new(s("gtceu:nitrogen"), vec![(0, 1_000)]),
                    ],
                    max_sets: 5,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Blizz Powder"), 16),
                    inputs: vec![MultiInvSlottedInput::new(label("Crushed Ice"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:elemental_reduction_fluid"), vec![(0, 100)])],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Blaze Powder"), 16),
                    inputs: vec![MultiInvSlottedInput::new(label("Netherrack Dust"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:elemental_reduction_fluid"), vec![(0, 100)])],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Basalz Powder"), 16),
                    inputs: vec![MultiInvSlottedInput::new(label("Coal Dust"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:elemental_reduction_fluid"), vec![(0, 100)])],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Blitz Powder"), 16),
                    inputs: vec![MultiInvSlottedInput::new(label("Endstone Dust"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:elemental_reduction_fluid"), vec![(0, 100)])],
                    max_sets: 8,
                },
            ]
        };
        factory.add_process(FluidSlottedConfig {
            name: s("reactor-1"),
            input_slots: vec![vec![0, 1]],
            input_tanks: vec![vec![0, 1, 2]],
            accesses: inv_tank(s("gtceu:hv_chemical_reactor_2")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: fluid_extract_all(),
            recipes: (reactor_any().into_iter())
                .chain([
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Sodium Bisulfate Dust"), 16),
                        inputs: vec![MultiInvSlottedInput::new(label("Salt"), vec![(0, 0, 2)])],
                        fluids: vec![FluidSlottedInput::new(s("gtceu:sulfuric_acid"), vec![(0, 1_000)])],
                        max_sets: 8,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Phosphorus Pentoxide Dust"), 16),
                        inputs: vec![MultiInvSlottedInput::new(label("Phosphorus Dust"), vec![(0, 0, 4)])],
                        fluids: vec![FluidSlottedInput::new(s("gtceu:oxygen"), vec![(0, 10_000)])],
                        max_sets: 1,
                    },
                ])
                .collect(),
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("reactor-2"),
            input_slots: vec![vec![0, 1]],
            input_tanks: vec![vec![0, 1, 2]],
            accesses: inv_tank(s("gtceu:hv_chemical_reactor_3")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: fluid_extract_all(),
            recipes: (reactor_any().into_iter())
                .chain([FluidSlottedRecipe {
                    outputs: FluidOutput::new(s("gtceu:sulfuric_acid"), 64_000),
                    inputs: vec![],
                    fluids: vec![
                        FluidSlottedInput::new(s("gtceu:hydrogen_sulfide"), vec![(0, 1_000)]),
                        FluidSlottedInput::new(s("gtceu:oxygen"), vec![(0, 4_000)]),
                    ],
                    max_sets: 4,
                }])
                .collect(),
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("reactor-CR-none"),
            input_slots: vec![vec![0, 1]],
            input_tanks: vec![vec![0, 1, 2]],
            accesses: inv_tank(s("gtceu:ev_chemical_reactor_0")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: fluid_extract_all(),
            recipes: (reactor_any().into_iter())
                .chain([FluidSlottedRecipe {
                    outputs: Output::new(label("Nano CPU Wafer"), 4),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("CPU Wafer"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Raw Carbon Fibers"), vec![(0, 1, 16)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:glowstone"), vec![(0, 576)])],
                    max_sets: 1,
                }])
                .collect(),
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("solidifier-plate"),
            input_slots: vec![vec![]],
            input_tanks: vec![vec![0]],
            accesses: inv_tank(s("gtceu:hv_fluid_solidifier_3")),
            to_extract: Some(Box::new(|_, _, i, _| i == 1)),
            fluid_extract: None,
            recipes: [
                (16, "Polytetrafluoroethylene Sheet", "gtceu:polytetrafluoroethylene"),
                (16, "Polyvinyl Chloride Sheet", "gtceu:polyvinyl_chloride"),
                (16, "Epoxy Sheet", "gtceu:epoxy"),
            ]
            .into_iter()
            .map(|(qty, o, i)| FluidSlottedRecipe {
                outputs: Output::new(label(o), qty),
                inputs: vec![],
                fluids: vec![FluidSlottedInput::new(s(i), vec![(0, 144)])],
                max_sets: 16,
            })
            .collect(),
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("solidifier-block"),
            input_slots: vec![vec![]],
            input_tanks: vec![vec![0]],
            accesses: inv_tank(s("gtceu:hv_fluid_solidifier_5")),
            to_extract: Some(Box::new(|_, _, i, _| i == 1)),
            fluid_extract: None,
            recipes: vec![FluidSlottedRecipe {
                outputs: Output::new(label("Ice"), 16),
                inputs: vec![],
                fluids: vec![FluidSlottedInput::new(s("gtceu:ice"), vec![(0, 144)])],
                max_sets: 16,
            }],
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("solidifier-ball"),
            input_slots: vec![vec![]],
            input_tanks: vec![vec![0]],
            accesses: inv_tank(s("gtceu:hv_fluid_solidifier_4")),
            to_extract: Some(Box::new(|_, _, i, _| i == 1)),
            fluid_extract: None,
            recipes: vec![FluidSlottedRecipe {
                outputs: Output::new(label("Snowball"), 16),
                inputs: vec![],
                fluids: vec![FluidSlottedInput::new(s("minecraft:water"), vec![(0, 250)])],
                max_sets: 16,
            }],
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("electrolyzer"),
            input_slots: vec![vec![0, 1]],
            input_tanks: vec![vec![0]],
            accesses: inv_tank(s("gtceu:hv_electrolyzer_1")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: fluid_extract_all(),
            recipes: [(1_000, "gtceu:carbon_monoxide"), (1_000, "gtceu:sulfur_dioxide")]
                .into_iter()
                .map(|(qty, x)| FluidSlottedRecipe {
                    outputs: ignore_outputs(2.),
                    inputs: vec![],
                    fluids: vec![FluidSlottedInput::new(s(x), vec![(0, qty)]).extra_backup(64_000)],
                    max_sets: (16_000 / qty).min(8) as i32,
                })
                .chain([
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Arsenic Dust"), 16),
                        inputs: vec![MultiInvSlottedInput::new(label("Arsenic Trioxide Dust"), vec![(0, 0, 5)])],
                        fluids: vec![],
                        max_sets: 8,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Rutile Dust"), 16),
                        inputs: vec![MultiInvSlottedInput::new(label("Bauxite Dust"), vec![(0, 0, 15)])],
                        fluids: vec![],
                        max_sets: 8,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Chromium Dust"), 16),
                        inputs: vec![MultiInvSlottedInput::new(label("Chromite Dust"), vec![(0, 0, 7)])],
                        fluids: vec![],
                        max_sets: 8,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Phosphorus Dust"), 16)
                            .or(less_than(ore_variants("Tricalcium Phosphate"), ore_variants("Apatite"))),
                        inputs: vec![MultiInvSlottedInput::new(label("Phosphate Dust"), vec![(0, 0, 5)])],
                        fluids: vec![],
                        max_sets: 8,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Phosphorus Dust"), 16)
                            .or(<_>::not(less_than(ore_variants("Tricalcium Phosphate"), ore_variants("Apatite")))),
                        inputs: vec![MultiInvSlottedInput::new(label("Apatite Dust"), vec![(0, 0, 9)])],
                        fluids: vec![],
                        max_sets: 8,
                    },
                    FluidSlottedRecipe {
                        outputs: FluidOutput::new(s("gtceu:sodium_persulfate"), 64_000),
                        inputs: vec![MultiInvSlottedInput::new(label("Sodium Bisulfate Dust"), vec![(0, 0, 7)])],
                        fluids: vec![],
                        max_sets: 8,
                    },
                ])
                .collect(),
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("cutter"),
            input_slots: vec![vec![0]],
            input_tanks: vec![vec![0]],
            accesses: inv_tank(s("gtceu:ev_cutter_0")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: None,
            recipes: [
                (4, "Phosphorus-doped Wafer", "Phosphorus-doped Monocrystalline Silicon Boule", 250),
                (16, "SoC", "SoC Wafer", 250),
                (16, "RAM Chip", "RAM Wafer", 67),
                (16, "CPU Chip", "CPU Wafer", 84),
                (16, "Nano CPU Chip", "Nano CPU Wafer", 250),
                (16, "NOR Memory Chip", "NOR Memory Wafer", 135),
            ]
            .into_iter()
            .map(|(qty, o, i, l)| FluidSlottedRecipe {
                outputs: Output::new(label(o), qty),
                inputs: vec![MultiInvSlottedInput::new(label(i), vec![(0, 0, 1)])],
                fluids: vec![FluidSlottedInput::new(s("gtceu:lubricant"), vec![(0, l)])],
                max_sets: 1,
            })
            .collect(),
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("autoclave"),
            input_slots: vec![vec![0, 1]],
            input_tanks: vec![vec![0]],
            accesses: inv_tank(s("gtceu:hv_autoclave_1")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: fluid_extract_all(),
            recipes: vec![FluidSlottedRecipe {
                outputs: Output::new(label("Raw Carbon Fibers"), 16),
                inputs: vec![MultiInvSlottedInput::new(label("Carbon Dust"), vec![(0, 0, 4)])],
                fluids: vec![FluidSlottedInput::new(s("gtceu:epoxy"), vec![(0, 9)])],
                max_sets: 8,
            }],
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("centrifuge"),
            input_slots: vec![vec![0, 1]],
            input_tanks: vec![vec![0]],
            accesses: inv_tank(s("gtceu:hv_centrifuge_1")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: fluid_extract_all(),
            recipes: vec![
                FluidSlottedRecipe {
                    outputs: FluidOutput::new(s("gtceu:mercury"), 64_000),
                    inputs: vec![MultiInvSlottedInput::new(label("Redstone Dust"), vec![(0, 0, 10)])],
                    fluids: vec![],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Phosphate Dust"), 16),
                    inputs: vec![MultiInvSlottedInput::new(label("Tricalcium Phosphate Dust"), vec![(0, 0, 5)])],
                    fluids: vec![],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Salt"), 16),
                    inputs: vec![],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:salt_water"), vec![(0, 1_000)])],
                    max_sets: 8,
                },
            ],
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("formingPress"),
            accesses: acc(s("gtceu:hv_forming_press_1")),
            input_slots: (0..6).collect(),
            to_extract: extract_all(),
            recipes: vec![
                SlottedRecipe {
                    outputs: Output::new(label("Bronze Drill Head"), 4),
                    inputs: vec![
                        SlottedInput::new(label("Double Steel Plate"), vec![(0, 2)]),
                        SlottedInput::new(label("Bronze Plate"), vec![(1, 4)]),
                    ],
                    max_sets: 1,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Sterling Silver Drill Head"), 4),
                    inputs: vec![
                        SlottedInput::new(label("Double Steel Plate"), vec![(0, 2)]),
                        SlottedInput::new(label("Sterling Silver Plate"), vec![(1, 4)]),
                    ],
                    max_sets: 1,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Invar Drill Head"), 4),
                    inputs: vec![SlottedInput::new(label("Double Steel Plate"), vec![(0, 2)]), SlottedInput::new(label("Invar Plate"), vec![(1, 4)])],
                    max_sets: 1,
                },
            ],
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("packer-2"),
            accesses: acc(s("gtceu:hv_packer_1")),
            input_slots: vec![0, 1],
            to_extract: extract_all(),
            recipes: vec![
                SlottedRecipe {
                    outputs: Output::new(label("2x Cupronickel Wire"), 16),
                    inputs: vec![SlottedInput::new(label("1x Cupronickel Wire"), vec![(0, 2)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Small Pile of Gallium Arsenide Dust"), 16),
                    inputs: vec![SlottedInput::new(label("Gallium Arsenide Dust"), vec![(0, 1)])],
                    max_sets: 8,
                },
            ],
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("distillery-1"),
            input_slots: vec![vec![]],
            input_tanks: vec![vec![0]],
            accesses: inv_tank(s("gtceu:hv_distillery_1")),
            to_extract: Some(Box::new(|_, _, i, _| i == 1)),
            fluid_extract: fluid_extract_all(),
            recipes: vec![FluidSlottedRecipe {
                outputs: FluidOutput::new(s("gtceu:distilled_water"), 64_000),
                inputs: vec![],
                fluids: vec![FluidSlottedInput::new(s("minecraft:water"), vec![(0, 288)])],
                max_sets: 8,
            }],
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("arcFurnace"),
            input_slots: vec![vec![0]],
            input_tanks: vec![vec![0]],
            accesses: inv_tank(s("gtceu:hv_arc_furnace_0")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: fluid_extract_all(),
            recipes: vec![FluidSlottedRecipe {
                outputs: Output::new(label("Glass"), 64),
                inputs: vec![MultiInvSlottedInput::new(label("Sand"), vec![(0, 0, 1)])],
                fluids: vec![FluidSlottedInput::new(s("gtceu:oxygen"), vec![(0, 20)])],
                max_sets: 16,
            }],
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("forgeHammer"),
            accesses: acc(s("gtceu:hv_forge_hammer_0")),
            input_slots: vec![0],
            to_extract: extract_all(),
            recipes: vec![
                SlottedRecipe {
                    outputs: Output::new(label("Gravel"), 64),
                    inputs: vec![SlottedInput::new(label("Cobblestone"), vec![(0, 1)])],
                    max_sets: 64,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Sand"), 64),
                    inputs: vec![SlottedInput::new(label("Gravel"), vec![(0, 1)])],
                    max_sets: 64,
                },
            ],
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("polarizer"),
            accesses: acc(s("gtceu:hv_polarizer_0")),
            input_slots: vec![0],
            to_extract: extract_all(),
            recipes: (["Steel Rod"].into_iter())
                .map(|x| SlottedRecipe {
                    outputs: Output::new(label!("Magnetic {x}"), 16),
                    inputs: vec![SlottedInput::new(label(x), vec![(0, 1)])],
                    max_sets: 8,
                })
                .collect(),
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("lathe"),
            accesses: acc(s("gtceu:hv_lathe_0")),
            input_slots: vec![0],
            to_extract: extract_all(),
            recipes: (["Steel", "Aluminium"].into_iter())
                .map(|x| SlottedRecipe {
                    outputs: Output::new(label!("{x} Rod"), 16),
                    inputs: vec![SlottedInput::new(label!("{x} Ingot"), vec![(0, 1)])],
                    max_sets: 8,
                })
                .collect(),
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("bender-1"),
            accesses: acc(s("gtceu:hv_bender_2")),
            input_slots: vec![0, 1],
            to_extract: extract_all(),
            recipes: (["Invar", "Bronze", "Sterling Silver"].into_iter())
                .map(|x| SlottedRecipe {
                    outputs: Output::new(label!("{x} Plate"), 16),
                    inputs: vec![SlottedInput::new(label!("{x} Ingot"), vec![(0, 1)])],
                    max_sets: 8,
                })
                .chain([SlottedRecipe {
                    outputs: Output::new(label("Thin Polyvinyl Chloride Sheet"), 16),
                    inputs: vec![SlottedInput::new(label("Polyvinyl Chloride Sheet"), vec![(0, 1)])],
                    max_sets: 8,
                }])
                .collect(),
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("bender-2"),
            accesses: acc(s("gtceu:hv_bender_4")),
            input_slots: vec![0, 1],
            to_extract: extract_all(),
            recipes: ["Steel"]
                .into_iter()
                .map(|x| SlottedRecipe {
                    outputs: Output::new(label!("Double {x} Plate"), 16),
                    inputs: vec![SlottedInput::new(label!("{x} Ingot"), vec![(0, 2)])],
                    max_sets: 4,
                })
                .collect(),
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("bender-10"),
            accesses: acc(s("gtceu:hv_bender_3")),
            input_slots: vec![0, 1],
            to_extract: extract_all(),
            recipes: ["Gold", "Copper", "Gallium", "Tantalum", "Electrum"]
                .into_iter()
                .map(|x| SlottedRecipe {
                    outputs: Output::new(label!("{x} Foil"), 16),
                    inputs: vec![SlottedInput::new(label!("{x} Ingot"), vec![(0, 1)])],
                    max_sets: 8,
                })
                .collect(),
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("wiremill-1"),
            accesses: acc(s("gtceu:hv_wiremill_2")),
            input_slots: vec![0, 1],
            to_extract: extract_all(),
            recipes: ["Cupronickel", "Copper"]
                .into_iter()
                .map(|x| SlottedRecipe {
                    outputs: Output::new(label!("1x {x} Wire"), 16),
                    inputs: vec![SlottedInput::new(label!("{x} Ingot"), vec![(0, 1)])],
                    max_sets: 8,
                })
                .collect(),
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("wiremill-3"),
            accesses: acc(s("gtceu:hv_wiremill_1")),
            input_slots: vec![0, 1],
            to_extract: extract_all(),
            recipes: ["Electrum", "Tantalum", "Red Alloy", "Copper", "material.gtceu.lumium", "Platinum"]
                .into_iter()
                .map(|x| SlottedRecipe {
                    outputs: Output::new(label!("Fine {x} Wire"), 16),
                    inputs: vec![SlottedInput::new(label!("{x} Ingot"), vec![(0, 1)])],
                    max_sets: 8,
                })
                .collect(),
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("extruder-ring"),
            accesses: acc(s("gtceu:hv_extruder_2")),
            input_slots: vec![0],
            to_extract: Some(Box::new(|_, i, _| i == 2)),
            recipes: vec![SlottedRecipe {
                outputs: Output::new(label("Nickel Zinc Ferrite Ring"), 16),
                inputs: vec![SlottedInput::new(label("Nickel Zinc Ferrite Ingot"), vec![(0, 1)])],
                max_sets: 4,
            }],
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("extruder-bolt"),
            accesses: acc(s("gtceu:hv_extruder_3")),
            input_slots: vec![0],
            to_extract: Some(Box::new(|_, i, _| i == 2)),
            recipes: vec![SlottedRecipe {
                outputs: Output::new(label("Tin Bolt"), 16),
                inputs: vec![SlottedInput::new(label("Tin Ingot"), vec![(0, 1)])],
                max_sets: 4,
            }],
            strict_priority: false,
        });
        for (addr, c, o) in [
            ("gtceu:hv_laser_engraver_0", "lightBlue", "CPU Wafer"),
            ("gtceu:hv_laser_engraver_1", "yellow", "SoC Wafer"),
            ("gtceu:hv_laser_engraver_2", "green", "RAM Wafer"),
            ("gtceu:hv_laser_engraver_3", "pink", "NOR Memory Wafer"),
        ] {
            factory.add_process(SlottedConfig {
                name: local_fmt!("engraver-{c}"),
                accesses: acc(s(addr)),
                input_slots: vec![0],
                to_extract: Some(Box::new(|_, i, _| i == 2)),
                recipes: vec![SlottedRecipe {
                    outputs: Output::new(label(o), 4),
                    inputs: vec![SlottedInput::new(label("Phosphorus-doped Wafer"), vec![(0, 1)])],
                    max_sets: 1,
                }],
                strict_priority: false,
            });
        }
        factory.add_process(SlottedConfig {
            name: s("furnace"),
            accesses: acc(s("gtceu:hv_electric_furnace_1")),
            input_slots: vec![0],
            to_extract: extract_all(),
            recipes: (cold_metals.iter())
                .map(|&x| SlottedRecipe {
                    outputs: Output::new(label!("{x} Ingot"), 64),
                    inputs: vec![SlottedInput::new(label!("{x} Dust"), vec![(0, 1)]).extra_backup(64)],
                    max_sets: 8,
                })
                .chain([
                    SlottedRecipe {
                        outputs: cold_metal_output("Copper"),
                        inputs: vec![SlottedInput::new(label("Chalcopyrite Dust"), vec![(0, 1)])],
                        max_sets: 8,
                    },
                    SlottedRecipe {
                        outputs: Output::new(label("item.kubejs.pulsating_dust"), 16),
                        inputs: vec![SlottedInput::new(label("Uraninite Dust"), vec![(0, 1)])],
                        max_sets: 8,
                    },
                ])
                .collect(),
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("alloySmelter"),
            accesses: acc(s("gtceu:hv_alloy_smelter_2")),
            input_slots: vec![0, 1],
            to_extract: extract_all(),
            recipes: vec![
                SlottedRecipe {
                    outputs: Output::new(label("Cupronickel Ingot"), 64),
                    inputs: vec![SlottedInput::new(label("Copper Dust"), vec![(0, 1)]), SlottedInput::new(label("Nickel Dust"), vec![(1, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Red Alloy Ingot"), 64),
                    inputs: vec![SlottedInput::new(label("Copper Dust"), vec![(0, 1)]), SlottedInput::new(label("Redstone Dust"), vec![(1, 4)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Electrum Ingot"), 64),
                    inputs: vec![SlottedInput::new(label("Gold Dust"), vec![(0, 1)]), SlottedInput::new(label("Silver Dust"), vec![(1, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Invar Ingot"), 64),
                    inputs: vec![SlottedInput::new(label("Iron Dust"), vec![(0, 2)]), SlottedInput::new(label("Nickel Dust"), vec![(1, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Bronze Ingot"), 64),
                    inputs: vec![SlottedInput::new(label("Copper Dust"), vec![(0, 3)]), SlottedInput::new(label("Tin Dust"), vec![(1, 1)])],
                    max_sets: 8,
                },
            ],
            strict_priority: false,
        });
        let asm_any = || {
            [
                FluidSlottedRecipe {
                    outputs: Output::new(label("1x Copper Cable"), 16),
                    inputs: vec![MultiInvSlottedInput::new(label("1x Copper Wire"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:styrene_butadiene_rubber"), vec![(0, 36)])],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("MV Electric Motor"), 16),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("1x Copper Cable"), vec![(0, 0, 2)]),
                        MultiInvSlottedInput::new(label("Aluminium Rod"), vec![(0, 1, 2)]),
                        MultiInvSlottedInput::new(label("Magnetic Steel Rod"), vec![(0, 2, 1)]),
                        MultiInvSlottedInput::new(label("2x Cupronickel Wire"), vec![(0, 3, 4)]),
                    ],
                    fluids: vec![],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("item.kubejs.basic_drilling_kit"), 4),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Double Steel Plate"), vec![(0, 0, 3)]),
                        MultiInvSlottedInput::new(label("MV Electric Motor"), vec![(0, 1, 1)]),
                        MultiInvSlottedInput::new(label("Microchip Processor"), vec![(0, 2, 2)]),
                        MultiInvSlottedInput::new(label("Glass"), vec![(0, 3, 2)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:glue"), vec![(0, 200)])],
                    max_sets: 1,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("SMD Resistor"), 16),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Carbon Dust"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Fine Tantalum Wire"), vec![(0, 1, 4)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:polyethylene"), vec![(0, 288)])],
                    max_sets: 2,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("SMD Capacitor"), 16),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Thin Polyvinyl Chloride Sheet"), vec![(0, 0, 2)]),
                        MultiInvSlottedInput::new(label("Tantalum Foil"), vec![(0, 1, 1)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:polyethylene"), vec![(0, 72)])],
                    max_sets: 2,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("SMD Transistor"), 16),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Gallium Foil"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Fine Tantalum Wire"), vec![(0, 1, 8)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:polyethylene"), vec![(0, 144)])],
                    max_sets: 2,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("SMD Inductor"), 16),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Nickel Zinc Ferrite Ring"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Fine Tantalum Wire"), vec![(0, 1, 4)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:polyethylene"), vec![(0, 144)])],
                    max_sets: 2,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("SMD Diode"), 16),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Gallium Arsenide Dust"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Fine Platinum Wire"), vec![(0, 1, 8)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:polyethylene"), vec![(0, 288)])],
                    max_sets: 2,
                },
            ]
        };
        factory.add_process(FluidSlottedConfig {
            name: s("asm-1"),
            input_slots: vec![(0..9).collect()],
            input_tanks: vec![vec![0]],
            accesses: inv_tank(s("gtceu:hv_assembler_1")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: None,
            recipes: asm_any().into_iter().collect(),
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("circuitAsm"),
            input_slots: vec![(0..6).collect()],
            input_tanks: vec![vec![0]],
            accesses: inv_tank(s("gtceu:ev_circuit_assembler_0")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: None,
            recipes: vec![
                FluidSlottedRecipe {
                    outputs: Output::new(label("Microchip Processor"), 16),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Plastic Printed Circuit Board"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("SoC"), vec![(0, 1, 1)]),
                        MultiInvSlottedInput::new(label("Fine Copper Wire"), vec![(0, 2, 2)]),
                        MultiInvSlottedInput::new(label("Tin Bolt"), vec![(0, 3, 2)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:soldering_alloy"), vec![(0, 72)])],
                    max_sets: 4,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Microprocessor"), 16),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Plastic Printed Circuit Board"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("CPU Chip"), vec![(0, 1, 1)]),
                        MultiInvSlottedInput::new(label("SMD Resistor"), vec![(0, 2, 4)]),
                        MultiInvSlottedInput::new(label("SMD Capacitor"), vec![(0, 3, 4)]),
                        MultiInvSlottedInput::new(label("SMD Transistor"), vec![(0, 4, 4)]),
                        MultiInvSlottedInput::new(label("Fine Red Alloy Wire"), vec![(0, 5, 4)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:soldering_alloy"), vec![(0, 72)])],
                    max_sets: 4,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Nanoprocessor"), 16),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Epoxy Printed Circuit Board"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Nano CPU Chip"), vec![(0, 1, 1)]),
                        MultiInvSlottedInput::new(label("SMD Resistor"), vec![(0, 2, 8)]),
                        MultiInvSlottedInput::new(label("SMD Capacitor"), vec![(0, 3, 8)]),
                        MultiInvSlottedInput::new(label("SMD Transistor"), vec![(0, 4, 8)]),
                        MultiInvSlottedInput::new(label("Fine Electrum Wire"), vec![(0, 5, 8)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:soldering_alloy"), vec![(0, 72)])],
                    max_sets: 4,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Nanoprocessor Assembly"), 16),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Epoxy Printed Circuit Board"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Nanoprocessor"), vec![(0, 1, 2)]),
                        MultiInvSlottedInput::new(label("SMD Inductor"), vec![(0, 2, 4)]),
                        MultiInvSlottedInput::new(label("SMD Capacitor"), vec![(0, 3, 8)]),
                        MultiInvSlottedInput::new(label("RAM Chip"), vec![(0, 4, 8)]),
                        MultiInvSlottedInput::new(label("Fine Electrum Wire"), vec![(0, 5, 16)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:soldering_alloy"), vec![(0, 144)])],
                    max_sets: 4,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Nanoprocessor Supercomputer"), 16),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Epoxy Printed Circuit Board"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Nanoprocessor Assembly"), vec![(0, 1, 2)]),
                        MultiInvSlottedInput::new(label("SMD Diode"), vec![(0, 2, 8)]),
                        MultiInvSlottedInput::new(label("NOR Memory Chip"), vec![(0, 3, 4)]),
                        MultiInvSlottedInput::new(label("RAM Chip"), vec![(0, 4, 16)]),
                        MultiInvSlottedInput::new(label("Fine material.gtceu.lumium Wire"), vec![(0, 5, 16)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:soldering_alloy"), vec![(0, 144)])],
                    max_sets: 4,
                },
            ],
            strict_priority: false,
        });
        let mixer_any = || {
            [
                FluidSlottedRecipe {
                    outputs: Output::new(label("item.kubejs.cryotheum_dust"), 16),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Blizz Powder"), vec![(0, 0, 2)]),
                        MultiInvSlottedInput::new(label("Redstone Dust"), vec![(0, 1, 1)]),
                        MultiInvSlottedInput::new(label("Snowball"), vec![(0, 2, 1)]),
                    ],
                    fluids: vec![],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("item.kubejs.pyrotheum_dust"), 16),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Blaze Powder"), vec![(0, 0, 2)]),
                        MultiInvSlottedInput::new(label("Redstone Dust"), vec![(0, 1, 1)]),
                        MultiInvSlottedInput::new(label("Sulfur Dust"), vec![(0, 2, 1)]),
                    ],
                    fluids: vec![],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("item.kubejs.petrotheum_dust"), 16),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Basalz Powder"), vec![(0, 0, 2)]),
                        MultiInvSlottedInput::new(label("Redstone Dust"), vec![(0, 1, 1)]),
                        MultiInvSlottedInput::new(label("Obsidian Dust"), vec![(0, 2, 1)]),
                    ],
                    fluids: vec![],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("item.kubejs.aerotheum_dust"), 16),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Blitz Powder"), vec![(0, 0, 2)]),
                        MultiInvSlottedInput::new(label("Redstone Dust"), vec![(0, 1, 1)]),
                        MultiInvSlottedInput::new(label("Saltpeter Dust"), vec![(0, 2, 1)]),
                    ],
                    fluids: vec![],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("item.kubejs.primal_mana"), 16),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("item.kubejs.petrotheum_dust"), vec![(0, 0, 2)]),
                        MultiInvSlottedInput::new(label("item.kubejs.pyrotheum_dust"), vec![(0, 1, 2)]),
                        MultiInvSlottedInput::new(label("item.kubejs.aerotheum_dust"), vec![(0, 2, 2)]),
                        MultiInvSlottedInput::new(label("item.kubejs.cryotheum_dust"), vec![(0, 3, 2)]),
                        MultiInvSlottedInput::new(label("Diamond Dust"), vec![(0, 4, 1)]),
                    ],
                    fluids: vec![],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Luminessence"), 16),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Redstone Dust"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Glowstone Dust"), vec![(0, 1, 1)]),
                        MultiInvSlottedInput::new(label("Aluminium Dust"), vec![(0, 2, 2)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:phosphoric_acid"), vec![(0, 4_000)])],
                    max_sets: 4,
                },
            ]
        };
        factory.add_process(FluidSlottedConfig {
            name: s("mixer-1"),
            input_slots: vec![(0..6).collect()],
            input_tanks: vec![vec![0, 1]],
            accesses: inv_tank(s("gtceu:hv_mixer_3")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: fluid_extract_all(),
            recipes: (mixer_any().into_iter())
                .chain([
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Gallium Arsenide Dust"), 16),
                        inputs: vec![
                            MultiInvSlottedInput::new(label("Gallium Dust"), vec![(0, 0, 1)]),
                            MultiInvSlottedInput::new(label("Arsenic Dust"), vec![(0, 1, 1)]),
                        ],
                        fluids: vec![],
                        max_sets: 4,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Sterling Silver Dust"), 16),
                        inputs: vec![
                            MultiInvSlottedInput::new(label("Copper Dust"), vec![(0, 0, 1)]),
                            MultiInvSlottedInput::new(label("Silver Dust"), vec![(0, 1, 4)]),
                        ],
                        fluids: vec![],
                        max_sets: 4,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Tin Alloy Dust"), 16),
                        inputs: vec![
                            MultiInvSlottedInput::new(label("Tin Dust"), vec![(0, 0, 1)]),
                            MultiInvSlottedInput::new(label("Iron Dust"), vec![(0, 1, 1)]),
                        ],
                        fluids: vec![],
                        max_sets: 4,
                    },
                ])
                .collect(),
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("mixer-2"),
            input_slots: vec![(0..6).collect()],
            input_tanks: vec![vec![0, 1]],
            accesses: inv_tank(s("gtceu:hv_mixer_4")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: fluid_extract_all(),
            recipes: (mixer_any().into_iter())
                .chain([
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Nichrome Dust"), 16),
                        inputs: vec![
                            MultiInvSlottedInput::new(label("Nickel Dust"), vec![(0, 0, 4)]),
                            MultiInvSlottedInput::new(label("Chromium Dust"), vec![(0, 1, 1)]),
                        ],
                        fluids: vec![],
                        max_sets: 4,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Ferrite Mixture Dust"), 16),
                        inputs: vec![
                            MultiInvSlottedInput::new(label("Nickel Dust"), vec![(0, 0, 1)]),
                            MultiInvSlottedInput::new(label("Zinc Dust"), vec![(0, 1, 1)]),
                            MultiInvSlottedInput::new(label("Iron Dust"), vec![(0, 2, 4)]),
                        ],
                        fluids: vec![],
                        max_sets: 4,
                    },
                ])
                .collect(),
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("mixer-3"),
            input_slots: vec![(0..6).collect()],
            input_tanks: vec![vec![0, 1]],
            accesses: inv_tank(s("gtceu:hv_mixer_5")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: fluid_extract_all(),
            recipes: (mixer_any().into_iter())
                .chain([
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Soldering Alloy Dust"), 16),
                        inputs: vec![
                            MultiInvSlottedInput::new(label("Tin Dust"), vec![(0, 0, 6)]),
                            MultiInvSlottedInput::new(label("Lead Dust"), vec![(0, 1, 3)]),
                            MultiInvSlottedInput::new(label("Antimony Dust"), vec![(0, 2, 1)]),
                        ],
                        fluids: vec![],
                        max_sets: 4,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Stainless Steel Dust"), 16),
                        inputs: vec![
                            MultiInvSlottedInput::new(label("Iron Dust"), vec![(0, 0, 6)]),
                            MultiInvSlottedInput::new(label("Nickel Dust"), vec![(0, 1, 1)]),
                            MultiInvSlottedInput::new(label("Manganese Dust"), vec![(0, 2, 1)]),
                            MultiInvSlottedInput::new(label("Chromium Dust"), vec![(0, 3, 1)]),
                        ],
                        fluids: vec![],
                        max_sets: 4,
                    },
                ])
                .collect(),
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("mixer-EV-none"),
            input_slots: vec![(0..6).collect()],
            input_tanks: vec![vec![0, 1]],
            accesses: inv_tank(s("gtceu:ev_mixer_0")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: fluid_extract_all(),
            recipes: (mixer_any().into_iter())
                .chain([FluidSlottedRecipe {
                    outputs: Output::new(label("material.gtceu.lumium Dust"), 1),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Tin Alloy Dust"), vec![(0, 0, 4)]),
                        MultiInvSlottedInput::new(label("Sterling Silver Dust"), vec![(0, 1, 2)]),
                        MultiInvSlottedInput::new(label("Luminessence"), vec![(0, 2, 2)]),
                        MultiInvSlottedInput::new(label("item.kubejs.energized_clathrate"), vec![(0, 3, 1)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:mana"), vec![(0, 1_000)])],
                    max_sets: 4,
                }])
                .collect(),
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("extractor"),
            input_slots: vec![vec![0]],
            input_tanks: vec![vec![]],
            accesses: inv_tank(s("gtceu:hv_extractor_1")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: fluid_extract_all(),
            recipes: vec![
                FluidSlottedRecipe {
                    outputs: FluidOutput::new(s("gtceu:soldering_alloy"), 64_000),
                    inputs: vec![MultiInvSlottedInput::new(label("Soldering Alloy Dust"), vec![(0, 0, 1)])],
                    fluids: vec![],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: FluidOutput::new(s("gtceu:glowstone"), 64_000),
                    inputs: vec![MultiInvSlottedInput::new(label("Glowstone Dust"), vec![(0, 0, 1)])],
                    fluids: vec![],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: FluidOutput::new(s("gtceu:mana"), 64_000),
                    inputs: vec![MultiInvSlottedInput::new(label("item.kubejs.primal_mana"), vec![(0, 0, 1)])],
                    fluids: vec![],
                    max_sets: 8,
                },
            ],
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("chemicalBath"),
            input_slots: vec![vec![0]],
            input_tanks: vec![vec![0]],
            accesses: inv_tank(s("gtceu:hv_chemical_bath_1")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: fluid_extract_all(),
            recipes: (mercury_bathed_ores.iter())
                .map(|&x| FluidSlottedRecipe {
                    outputs: ore_washing_outputs(x),
                    inputs: vec![MultiInvSlottedInput::new(label!("Crushed {x} Ore"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:mercury"), vec![(0, 100)])],
                    max_sets: 4,
                })
                .chain(persulfate_bathed_ores.iter().map(|&x| FluidSlottedRecipe {
                    outputs: ore_washing_outputs(x),
                    inputs: vec![MultiInvSlottedInput::new(label!("Crushed {x} Ore"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:sodium_persulfate"), vec![(0, 100)])],
                    max_sets: 4,
                }))
                .collect(),
            strict_priority: false,
        });
        factory.add_process(BlockingOutputConfig {
            accesses: acc(s("gtceu:hv_rock_crusher_1")),
            slot_filter: None,
            outputs: vec![Output { item: label("Obsidian"), n_wanted: 64 }],
        });
        factory.add_process(BlockingFluidOutputConfig {
            accesses: tank(s("ae2:cable_bus_3")),
            outputs: vec![
                FluidOutput { fluid: s("gtceu:oxygen"), n_wanted: 64_000 },
                // FluidOutput { fluid: s("gtceu:high_octane_gasoline"), n_wanted: 64_000 },
                FluidOutput { fluid: s("minecraft:water"), n_wanted: 64_000 },
                FluidOutput { fluid: s("gtceu:chlorine"), n_wanted: 64_000 },
                FluidOutput { fluid: s("gtceu:hydrogen"), n_wanted: 64_000 },
                FluidOutput { fluid: s("gtceu:epoxy"), n_wanted: 64_000 },
                FluidOutput { fluid: s("gtceu:salt_water"), n_wanted: 64_000 },
                FluidOutput { fluid: s("gtceu:hydrofluoric_acid"), n_wanted: 64_000 },
                FluidOutput { fluid: s("gtceu:nitrogen"), n_wanted: 64_000 },
                FluidOutput { fluid: s("gtceu:lubricant"), n_wanted: 64_000 },
                FluidOutput { fluid: s("gtceu:polytetrafluoroethylene"), n_wanted: 64_000 },
                FluidOutput { fluid: s("gtceu:polyvinyl_chloride"), n_wanted: 64_000 },
                FluidOutput { fluid: s("gtceu:polyethylene"), n_wanted: 64_000 },
                FluidOutput { fluid: s("gtceu:oxalic_acid_solution"), n_wanted: 64_000 },
                FluidOutput { fluid: s("gtceu:helium"), n_wanted: 64_000 },
                FluidOutput { fluid: s("gtceu:styrene_butadiene_rubber"), n_wanted: 64_000 },
                FluidOutput { fluid: s("gtceu:glue"), n_wanted: 64_000 },
            ],
        });
        factory.add_process(BlockingOutputConfig {
            accesses: acc(s("ae2:cable_bus_4")),
            slot_filter: None,
            outputs: vec![
                Output { item: label("item.kubejs.microminer_t1"), n_wanted: 1 },
                Output { item: label("Raw Chalcopyrite"), n_wanted: 64 },
                Output { item: label("Aluminium Ingot"), n_wanted: 64 },
                Output { item: label("Silicon Dust"), n_wanted: 64 },
                Output { item: label("Carbon Dust"), n_wanted: 64 },
                Output { item: label("Steel Ingot"), n_wanted: 64 },
                Output { item: label("Cobblestone"), n_wanted: 64 },
            ],
        });
        factory.add_process(BufferedConfig {
            name: s("dump"),
            accesses: acc(s("ae2:cable_bus_4")),
            slot_filter: Some(Box::new(|i| i == 8)),
            to_extract: None,
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![BufferedInput::new(label("Calcium Dust"), i32::MAX), BufferedInput::new(label("Stone Dust"), i32::MAX)],
        });
    })
}
