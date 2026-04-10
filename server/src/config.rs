use crate::factory::{Factory, FactoryConfig, FluidStorageConfig};
use crate::{access::*, config_util::*, item::Filter, process::*, recipe::*, storage::*};
use crate::{detail_cache::DetailCache, server::Server, Tui};
use flexstr::{local_fmt, LocalStr};
use fnv::FnvHashSet;
use std::{cell::RefCell, rc::Rc, time::Duration};

const MAIN: &str = "main";
const BUS: &str = "minecraft:barrel_0";

fn fbus() -> Vec<LocalStr> {
    vec![
        s("enderio:pressurized_fluid_tank_0"),
        s("enderio:pressurized_fluid_tank_1"),
        s("enderio:pressurized_fluid_tank_2"),
        s("enderio:pressurized_fluid_tank_3"),
    ]
}

fn acc(inv_addr: LocalStr) -> Vec<BusAccess> { vec![BusAccess { client: s(MAIN), inv_addr, bus_addr: s(BUS) }] }
fn tank(tank_addr: LocalStr) -> Vec<TankAccess> { vec![TankAccess { client: s(MAIN), tank_addr, fluid_bus_addrs: fbus() }] }
fn inv_tank(addr: LocalStr) -> Vec<InvTankAccess> {
    vec![InvTankAccess { client: s(MAIN), inv_addrs: vec![addr.clone()], tank_addrs: vec![addr], bus_addr: s(BUS), fluid_bus_addrs: fbus() }]
}

fn ore_variants(x: &str) -> impl Iterator<Item = LocalStr> {
    <_>::into_iter([
        local_fmt!("Raw {x}"),
        local_fmt!("{x} Ore"),
        local_fmt!("End {x} Ore"),
        local_fmt!("Talc {x} Ore"),
        local_fmt!("Nether {x} Ore"),
        local_fmt!("Deepslate {x} Ore"),
    ])
}

fn cold_metal_output(x: &str) -> Rc<dyn Outputs> { Output::new(label!("{x} Dust"), 65).or(Output::new(label!("{x} Ingot"), 65)) }

pub fn build_factory(tui: Rc<Tui>) -> Rc<RefCell<Factory>> {
    let cold_metals = ["Tantalum", "Gallium", "Silver", "Copper", "Gold", "Tin"];
    let washed_ores = ["Tricalcium Phosphate", "Apatite", "Chromite", "Uraninite", "Coal"];
    let mercury_bathed_ores = ["Silver", "Gold"];
    let all_ores = FnvHashSet::from_iter(washed_ores.into_iter().chain(mercury_bathed_ores));
    let refined_ores = all_ores.clone();
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
        factory.add_process(LowAlert::new(label("Glowstone Dust"), 64));
        factory.add_process(LowAlert::new(label("Potassium Dust"), 64));
        factory.add_process(LowAlert::new(label("Aluminium Dust"), 64));
        factory.add_process(LowAlert::new(label("Silicon Dioxide Dust"), 64));
        factory.add_process(LowAlert::new(label("Small Pile of Gallium Arsenide Dust"), 64));
        for x in ["Tin", "Copper", "Gallium", "Tantalum"] {
            factory.add_process(LowAlert::new(
                custom(local_fmt!("{x} Ingot/Dust"), move |_, y| {
                    let Some(suffix) = y.label.strip_prefix(x) else { return false };
                    suffix == " Ingot" || suffix == " Dust"
                }),
                65,
            ));
        }
        for x in ["Coal", "Gold", "Silver", "Chromite", "Uraninite"] {
            factory.add_process(LowAlert::new(custom(local_fmt!("{x} Ore"), move |_, y| ore_variants(x).any(|i| y.label == i)), 64));
        }
        factory.add_process(LowAlert::new(
            custom(s("Apatite/Tricalcium-Phosphate Ore"), |_, x| {
                ore_variants("Apatite").chain(ore_variants("Tricalcium Phosphate")).any(|i| x.label == i)
            }),
            64,
        ));
        factory.add_process(ManualUiConfig { accesses: acc(s("minecraft:barrel_4")) });
        for i in [1, 3, 0, 2] {
            factory.add_storage(ChestConfig { accesses: acc(local_fmt!("gtceu:titanium_crate_{i}")), override_max_stack_size: None });
        }
        for (capacity, addr, fluid) in [
            (32_000 * 8 * 8, "functionalstorage:fluid_1_0", "gtceu:oxygen"),
            (32_000 * 8, "functionalstorage:fluid_1_1", "gtceu:high_octane_gasoline"),
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
        ] {
            factory.add_fluid_storage(FluidStorageConfig { accesses: tank(s(addr)), fluid: s(fluid), capacity });
        }
        for addr in [
            "gtceu:mv_output_bus_1", // ebf
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
            ("gtceu:hv_combustion_4", "gtceu:high_octane_gasoline", 16_000),
            ("gtceu:hv_combustion_5", "gtceu:high_octane_gasoline", 16_000),
            ("gtceu:hv_combustion_6", "gtceu:high_octane_gasoline", 16_000),
            ("gtceu:hv_combustion_7", "gtceu:high_octane_gasoline", 16_000),
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
            accesses: acc(s("minecraft:barrel_3")),
            slot_filter: None,
            to_extract: None,
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![
                BufferedInput::new(label("Microchip Processor"), 64),
                BufferedInput::new(label("Nanoprocessor"), 64),
                BufferedInput::new(label("Microprocessor"), 64),
                BufferedInput::new(label("Nickel Zinc Ferrite Ring"), 64),
                BufferedInput::new(label("Plastic Printed Circuit Board"), 64),
                BufferedInput::new(label("Epoxy Printed Circuit Board"), 64),
                BufferedInput::new(label("SMD Resistor"), 64),
                BufferedInput::new(label("SMD Capacitor"), 64),
                BufferedInput::new(label("SMD Transistor"), 64),
                BufferedInput::new(label("SMD Inductor"), 64),
                BufferedInput::new(label("Chromium Ingot"), 64),
            ],
        });
        for i in [0, 1] {
            factory.add_process(SlottedConfig {
                name: s("macerator"),
                accesses: acc(local_fmt!("gtceu:hv_macerator_{i}")),
                input_slots: vec![0],
                to_extract: extract_all(),
                recipes: (all_ores.iter())
                    .flat_map(|&x| {
                        ore_variants(x).map(move |i| SlottedRecipe {
                            outputs: Output::new(label!("Crushed {x} Ore"), 64),
                            inputs: vec![SlottedInput::new(Filter::Label(i), vec![(0, 1)])],
                            max_sets: 8,
                        })
                    })
                    .chain(["Coal", "Apatite", "Chromite", "Uraninite", "Tricalcium Phosphate"].into_iter().map(|x| SlottedRecipe {
                        outputs: Output::new(label!("{x} Dust"), 64),
                        inputs: vec![SlottedInput::new(label!("Refined {x} Ore"), vec![(0, 1)])],
                        max_sets: 8,
                    }))
                    .chain(["Silver", "Gold"].into_iter().map(|x| SlottedRecipe {
                        outputs: cold_metal_output(x),
                        inputs: vec![SlottedInput::new(label!("Refined {x} Ore"), vec![(0, 1)])],
                        max_sets: 8,
                    }))
                    .chain(cold_metals.iter().map(|&x| SlottedRecipe {
                        outputs: Output::new(label!("{x} Dust"), 64),
                        inputs: vec![SlottedInput::new(label!("{x} Ingot"), vec![(0, 1)]).extra_backup(64)],
                        max_sets: 8,
                    }))
                    .chain([
                        SlottedRecipe { outputs: ignore_outputs(2.), inputs: vec![SlottedInput::new(label("Apatite"), vec![(0, 1)])], max_sets: 8 },
                        SlottedRecipe {
                            outputs: Output::new(label("Crushed Ice"), 64),
                            inputs: vec![SlottedInput::new(label("Ice"), vec![(0, 1)])],
                            max_sets: 8,
                        },
                        SlottedRecipe {
                            outputs: Output::new(label("Obsidian Dust"), 64),
                            inputs: vec![SlottedInput::new(label("Obsidian"), vec![(0, 1)])],
                            max_sets: 8,
                        },
                        SlottedRecipe {
                            outputs: Output::new(label("Endstone Dust"), 64),
                            inputs: vec![SlottedInput::new(label("End Stone"), vec![(0, 1)])],
                            max_sets: 8,
                        },
                        SlottedRecipe {
                            outputs: Output::new(label("Netherrack Dust"), 64),
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
            accesses: inv_tank(s("gtceu:hv_ore_washer_0")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: None,
            recipes: (washed_ores.iter())
                .map(|&x| FluidSlottedRecipe {
                    outputs: Output::new(label!("Purified {x} Ore"), 64),
                    inputs: vec![MultiInvSlottedInput::new(label!("Crushed {x} Ore"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:distilled_water"), vec![(0, 100)])],
                    max_sets: 8,
                })
                .collect(),
            strict_priority: false,
        });
        let ebf_any = || {
            [FluidSlottedRecipe {
                outputs: Output::new(label("Nickel Zinc Ferrite Ingot"), 64),
                inputs: vec![MultiInvSlottedInput::new(label("Ferrite Mixture Dust"), vec![(0, 0, 1)])],
                fluids: vec![FluidSlottedInput::new(s("gtceu:oxygen"), vec![(0, 2_000)])],
                max_sets: 2,
            }]
        };
        factory.add_process(SlottedConfig {
            name: s("thermalCentrifuge"),
            accesses: acc(s("gtceu:hv_thermal_centrifuge_0")),
            input_slots: vec![0],
            to_extract: extract_all(),
            recipes: (refined_ores.iter())
                .map(|&x| SlottedRecipe {
                    outputs: Output::new(label!("Refined {x} Ore"), 64),
                    inputs: vec![SlottedInput::new(label!("Purified {x} Ore"), vec![(0, 1)])],
                    max_sets: 8,
                })
                .collect(),
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("ebf-none"),
            input_slots: vec![(0..9).collect()],
            input_tanks: vec![vec![0]],
            accesses: vec![InvTankAccess {
                client: s(MAIN),
                inv_addrs: vec![s("gtceu:mv_input_bus_2")],
                tank_addrs: vec![s("gtceu:mv_input_hatch_2")],
                bus_addr: s(BUS),
                fluid_bus_addrs: fbus(),
            }],
            to_extract: None,
            fluid_extract: None,
            recipes: (ebf_any().into_iter())
                .chain([FluidSlottedRecipe {
                    outputs: Output::new(label("Phosphorus-doped Monocrystalline Silicon Boule"), 1),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Silicon Dust"), vec![(0, 0, 64)]),
                        MultiInvSlottedInput::new(label("Phosphorus Dust"), vec![(0, 1, 8)]),
                        MultiInvSlottedInput::new(label("Small Pile of Gallium Arsenide Dust"), vec![(0, 2, 2)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:nitrogen"), vec![(0, 8_000)])],
                    max_sets: 1,
                }])
                .collect(),
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("ebf-2"),
            input_slots: vec![(0..9).collect()],
            input_tanks: vec![vec![0]],
            accesses: vec![InvTankAccess {
                client: s(MAIN),
                inv_addrs: vec![s("gtceu:mv_input_bus_1")],
                tank_addrs: vec![s("gtceu:mv_input_hatch_1")],
                bus_addr: s(BUS),
                fluid_bus_addrs: fbus(),
            }],
            to_extract: None,
            fluid_extract: None,
            recipes: (ebf_any().into_iter())
                .chain([
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Hot material.gtceu.lumium Ingot"), 1),
                        inputs: vec![MultiInvSlottedInput::new(label("material.gtceu.lumium Dust"), vec![(0, 0, 1)])],
                        fluids: vec![FluidSlottedInput::new(s("gtceu:helium"), vec![(0, 100)])],
                        max_sets: 1,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Chromium Ingot"), 64),
                        inputs: vec![MultiInvSlottedInput::new(label("Chromium Dust"), vec![(0, 0, 1)])],
                        fluids: vec![FluidSlottedInput::new(s("gtceu:nitrogen"), vec![(0, 1_000)])],
                        max_sets: 4,
                    },
                ])
                .collect(),
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
                    outputs: Output::new(label("Plastic Circuit Board"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Polytetrafluoroethylene Sheet"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Copper Foil"), vec![(0, 1, 4)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:hexafluorosilicic_acid"), vec![(0, 30)])],
                    max_sets: 4,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Plastic Printed Circuit Board"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Plastic Circuit Board"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Copper Foil"), vec![(0, 1, 6)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:oxalic_acid_solution"), vec![(0, 125)])],
                    max_sets: 4,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Epoxy Circuit Board"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Epoxy Sheet"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Gold Foil"), vec![(0, 1, 8)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:hexafluorosilicic_acid"), vec![(0, 60)])],
                    max_sets: 4,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Epoxy Printed Circuit Board"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Epoxy Circuit Board"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Electrum Foil"), vec![(0, 1, 8)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:oxalic_acid_solution"), vec![(0, 250)])],
                    max_sets: 4,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Saltpeter Dust"), 64),
                    inputs: vec![MultiInvSlottedInput::new(label("Potassium Dust"), vec![(0, 0, 1)])],
                    fluids: vec![
                        FluidSlottedInput::new(s("gtceu:oxygen"), vec![(0, 3_000)]),
                        FluidSlottedInput::new(s("gtceu:nitrogen"), vec![(0, 1_000)]),
                    ],
                    max_sets: 5,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Blizz Powder"), 64),
                    inputs: vec![MultiInvSlottedInput::new(label("Crushed Ice"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:elemental_reduction_fluid"), vec![(0, 100)])],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Blaze Powder"), 64),
                    inputs: vec![MultiInvSlottedInput::new(label("Netherrack Dust"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:elemental_reduction_fluid"), vec![(0, 100)])],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Basalz Powder"), 64),
                    inputs: vec![MultiInvSlottedInput::new(label("Coal Dust"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:elemental_reduction_fluid"), vec![(0, 100)])],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Blitz Powder"), 64),
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
            accesses: inv_tank(s("gtceu:hv_chemical_reactor_1")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: fluid_extract_all(),
            recipes: (reactor_any().into_iter())
                .chain([
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Sodium Bisulfate Dust"), 64),
                        inputs: vec![MultiInvSlottedInput::new(label("Salt"), vec![(0, 0, 2)])],
                        fluids: vec![FluidSlottedInput::new(s("gtceu:sulfuric_acid"), vec![(0, 1_000)])],
                        max_sets: 8,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Phosphorus Pentoxide Dust"), 64),
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
            accesses: inv_tank(s("gtceu:hv_chemical_reactor_0")),
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
            accesses: inv_tank(s("gtceu:hv_fluid_solidifier_0")),
            to_extract: Some(Box::new(|_, _, i, _| i == 1)),
            fluid_extract: None,
            recipes: [
                (64, "Polytetrafluoroethylene Sheet", "gtceu:polytetrafluoroethylene"),
                (64, "Polyvinyl Chloride Sheet", "gtceu:polyvinyl_chloride"),
                (64, "Epoxy Sheet", "gtceu:epoxy"),
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
            accesses: inv_tank(s("gtceu:hv_fluid_solidifier_1")),
            to_extract: Some(Box::new(|_, _, i, _| i == 1)),
            fluid_extract: None,
            recipes: vec![FluidSlottedRecipe {
                outputs: Output::new(label("Ice"), 64),
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
            accesses: inv_tank(s("gtceu:hv_fluid_solidifier_2")),
            to_extract: Some(Box::new(|_, _, i, _| i == 1)),
            fluid_extract: None,
            recipes: vec![FluidSlottedRecipe {
                outputs: Output::new(label("Snowball"), 64),
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
            accesses: inv_tank(s("gtceu:hv_electrolyzer_0")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: fluid_extract_all(),
            recipes: [/*("material.gtceu.niobium_pentoxide Dust", 7), ("material.gtceu.manganese_difluoride Dust", 3)*/]
                .into_iter()
                .map(|(item, qty)| FluidSlottedRecipe {
                    outputs: ignore_outputs(2.),
                    inputs: vec![MultiInvSlottedInput::new(label(item), vec![(0, 0, qty)])],
                    fluids: vec![],
                    max_sets: 8,
                })
                .chain([
                    FluidSlottedRecipe {
                        outputs: ignore_outputs(2.),
                        inputs: vec![],
                        fluids: vec![FluidSlottedInput::new(s("gtceu:carbon_monoxide"), vec![(0, 1_000)]).extra_backup(64_000)],
                        max_sets: 8,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Rutile Dust"), 64),
                        inputs: vec![MultiInvSlottedInput::new(label("Bauxite Dust"), vec![(0, 0, 15)])],
                        fluids: vec![],
                        max_sets: 8,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Chromium Dust"), 64),
                        inputs: vec![MultiInvSlottedInput::new(label("Chromite Dust"), vec![(0, 0, 7)])],
                        fluids: vec![],
                        max_sets: 8,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Phosphorus Dust"), 64),
                        inputs: vec![MultiInvSlottedInput::new(label("Phosphate Dust"), vec![(0, 0, 5)])],
                        fluids: vec![],
                        max_sets: 8,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Phosphorus Dust"), 64),
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
            recipes: vec![
                FluidSlottedRecipe {
                    outputs: Output::new(label("Phosphorus-doped Wafer"), 4),
                    inputs: vec![MultiInvSlottedInput::new(label("Phosphorus-doped Monocrystalline Silicon Boule"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:lubricant"), vec![(0, 250)])],
                    max_sets: 1,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("SoC"), 16),
                    inputs: vec![MultiInvSlottedInput::new(label("SoC Wafer"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:lubricant"), vec![(0, 250)])],
                    max_sets: 1,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("CPU Chip"), 16),
                    inputs: vec![MultiInvSlottedInput::new(label("CPU Wafer"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:lubricant"), vec![(0, 84)])],
                    max_sets: 1,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Nano CPU Chip"), 16),
                    inputs: vec![MultiInvSlottedInput::new(label("Nano CPU Wafer"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:lubricant"), vec![(0, 250)])],
                    max_sets: 1,
                },
            ],
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("autoclave"),
            input_slots: vec![vec![0, 1]],
            input_tanks: vec![vec![0]],
            accesses: inv_tank(s("gtceu:hv_autoclave_0")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: fluid_extract_all(),
            recipes: vec![FluidSlottedRecipe {
                outputs: Output::new(label("Raw Carbon Fibers"), 64),
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
            accesses: inv_tank(s("gtceu:hv_centrifuge_0")),
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
                    outputs: Output::new(label("Phosphate Dust"), 64),
                    inputs: vec![MultiInvSlottedInput::new(label("Tricalcium Phosphate Dust"), vec![(0, 0, 5)])],
                    fluids: vec![],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Salt"), 64),
                    inputs: vec![],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:salt_water"), vec![(0, 1_000)])],
                    max_sets: 8,
                },
            ],
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("distillery-1"),
            input_slots: vec![vec![]],
            input_tanks: vec![vec![0]],
            accesses: inv_tank(s("gtceu:hv_distillery_0")),
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
        factory.add_process(SlottedConfig {
            name: s("bender-10"),
            accesses: acc(s("gtceu:hv_bender_0")),
            input_slots: vec![0, 1],
            to_extract: extract_all(),
            recipes: ["Gold", "Copper", "Gallium", "Tantalum", "Electrum"]
                .into_iter()
                .map(|x| SlottedRecipe {
                    outputs: Output::new(label!("{x} Foil"), 64),
                    inputs: vec![SlottedInput::new(label!("{x} Ingot"), vec![(0, 1)])],
                    max_sets: 8,
                })
                .collect(),
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("bender-1"),
            accesses: acc(s("gtceu:hv_bender_1")),
            input_slots: vec![0, 1],
            to_extract: extract_all(),
            recipes: vec![
                SlottedRecipe {
                    outputs: Output::new(label("Thin Polyvinyl Chloride Sheet"), 64),
                    inputs: vec![SlottedInput::new(label("Polyvinyl Chloride Sheet"), vec![(0, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Invar Plate"), 64),
                    inputs: vec![SlottedInput::new(label("Invar Ingot"), vec![(0, 1)])],
                    max_sets: 8,
                },
            ],
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("wiremill-3"),
            accesses: acc(s("gtceu:hv_wiremill_0")),
            input_slots: vec![0, 1],
            to_extract: extract_all(),
            recipes: ["Electrum", "Tantalum", "Red Alloy", "Copper"]
                .into_iter()
                .map(|x| SlottedRecipe {
                    outputs: Output::new(label!("Fine {x} Wire"), 64),
                    inputs: vec![SlottedInput::new(label!("{x} Ingot"), vec![(0, 1)])],
                    max_sets: 8,
                })
                .collect(),
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("extruder-ring"),
            accesses: acc(s("gtceu:hv_extruder_0")),
            input_slots: vec![0],
            to_extract: Some(Box::new(|_, i, _| i == 2)),
            recipes: vec![SlottedRecipe {
                outputs: Output::new(label("Nickel Zinc Ferrite Ring"), 64),
                inputs: vec![SlottedInput::new(label("Nickel Zinc Ferrite Ingot"), vec![(0, 1)])],
                max_sets: 4,
            }],
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("extruder-bolt"),
            accesses: acc(s("gtceu:hv_extruder_1")),
            input_slots: vec![0],
            to_extract: Some(Box::new(|_, i, _| i == 2)),
            recipes: vec![SlottedRecipe {
                outputs: Output::new(label("Tin Bolt"), 64),
                inputs: vec![SlottedInput::new(label("Tin Ingot"), vec![(0, 1)])],
                max_sets: 4,
            }],
            strict_priority: false,
        });
        for (addr, c, o) in [("gtceu:hv_laser_engraver_0", "lightBlue", "CPU Wafer"), ("gtceu:hv_laser_engraver_1", "yellow", "SoC Wafer")] {
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
            accesses: acc(s("gtceu:hv_electric_furnace_0")),
            input_slots: vec![0],
            to_extract: extract_all(),
            recipes: (cold_metals.iter())
                .map(|&x| SlottedRecipe {
                    outputs: Output::new(label!("{x} Ingot"), 64),
                    inputs: vec![SlottedInput::new(label!("{x} Dust"), vec![(0, 1)]).extra_backup(64)],
                    max_sets: 8,
                })
                .chain([SlottedRecipe {
                    outputs: Output::new(label("item.kubejs.pulsating_dust"), 64),
                    inputs: vec![SlottedInput::new(label("Uraninite Dust"), vec![(0, 1)])],
                    max_sets: 8,
                }])
                .collect(),
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("alloySmelter"),
            accesses: acc(s("gtceu:hv_alloy_smelter_0")),
            input_slots: vec![0, 1],
            to_extract: extract_all(),
            recipes: vec![
                SlottedRecipe {
                    outputs: Output::new(label("Red Alloy Ingot"), 64),
                    inputs: vec![SlottedInput::new(label("Copper Ingot"), vec![(0, 1)]), SlottedInput::new(label("Redstone Dust"), vec![(1, 4)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Electrum Ingot"), 64),
                    inputs: vec![SlottedInput::new(label("Gold Ingot"), vec![(0, 1)]), SlottedInput::new(label("Silver Ingot"), vec![(1, 1)])],
                    max_sets: 8,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Invar Ingot"), 64),
                    inputs: vec![SlottedInput::new(label("Iron Dust"), vec![(0, 2)]), SlottedInput::new(label("Nickel Dust"), vec![(1, 1)])],
                    max_sets: 8,
                },
            ],
            strict_priority: false,
        });
        let asm_any = || {
            [
                FluidSlottedRecipe {
                    outputs: Output::new(label("SMD Resistor"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Carbon Dust"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Fine Tantalum Wire"), vec![(0, 1, 4)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:polyethylene"), vec![(0, 288)])],
                    max_sets: 2,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("SMD Capacitor"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Thin Polyvinyl Chloride Sheet"), vec![(0, 0, 2)]),
                        MultiInvSlottedInput::new(label("Tantalum Foil"), vec![(0, 1, 1)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:polyethylene"), vec![(0, 72)])],
                    max_sets: 2,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("SMD Transistor"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Gallium Foil"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Fine Tantalum Wire"), vec![(0, 1, 8)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:polyethylene"), vec![(0, 144)])],
                    max_sets: 2,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("SMD Inductor"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Nickel Zinc Ferrite Ring"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Fine Tantalum Wire"), vec![(0, 1, 4)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:polyethylene"), vec![(0, 144)])],
                    max_sets: 2,
                },
            ]
        };
        factory.add_process(FluidSlottedConfig {
            name: s("asm-1"),
            input_slots: vec![(0..9).collect()],
            input_tanks: vec![vec![0]],
            accesses: inv_tank(s("gtceu:hv_assembler_0")),
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
                    outputs: Output::new(label("Microchip Processor"), 64),
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
                    outputs: Output::new(label("Microprocessor"), 64),
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
                    outputs: Output::new(label("Nanoprocessor"), 64),
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
            ],
            strict_priority: false,
        });
        let mixer_any = || {
            [
                FluidSlottedRecipe {
                    outputs: Output::new(label("item.kubejs.cryotheum_dust"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Blizz Powder"), vec![(0, 0, 2)]),
                        MultiInvSlottedInput::new(label("Redstone Dust"), vec![(0, 1, 1)]),
                        MultiInvSlottedInput::new(label("Snowball"), vec![(0, 2, 1)]),
                    ],
                    fluids: vec![],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("item.kubejs.pyrotheum_dust"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Blaze Powder"), vec![(0, 0, 2)]),
                        MultiInvSlottedInput::new(label("Redstone Dust"), vec![(0, 1, 1)]),
                        MultiInvSlottedInput::new(label("Sulfur Dust"), vec![(0, 2, 1)]),
                    ],
                    fluids: vec![],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("item.kubejs.petrotheum_dust"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Basalz Powder"), vec![(0, 0, 2)]),
                        MultiInvSlottedInput::new(label("Redstone Dust"), vec![(0, 1, 1)]),
                        MultiInvSlottedInput::new(label("Obsidian Dust"), vec![(0, 2, 1)]),
                    ],
                    fluids: vec![],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("item.kubejs.aerotheum_dust"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Blitz Powder"), vec![(0, 0, 2)]),
                        MultiInvSlottedInput::new(label("Redstone Dust"), vec![(0, 1, 1)]),
                        MultiInvSlottedInput::new(label("Saltpeter Dust"), vec![(0, 2, 1)]),
                    ],
                    fluids: vec![],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("item.kubejs.primal_mana"), 64),
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
                    outputs: Output::new(label("Luminessence"), 64),
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
            accesses: inv_tank(s("gtceu:hv_mixer_2")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: fluid_extract_all(),
            recipes: (mixer_any().into_iter())
                .chain([
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Sterling Silver Dust"), 64),
                        inputs: vec![
                            MultiInvSlottedInput::new(label("Copper Dust"), vec![(0, 0, 1)]),
                            MultiInvSlottedInput::new(label("Silver Dust"), vec![(0, 1, 4)]),
                        ],
                        fluids: vec![],
                        max_sets: 4,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Tin Alloy Dust"), 64),
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
            accesses: inv_tank(s("gtceu:hv_mixer_1")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: fluid_extract_all(),
            recipes: (mixer_any().into_iter())
                .chain([
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Nichrome Dust"), 64),
                        inputs: vec![
                            MultiInvSlottedInput::new(label("Nickel Dust"), vec![(0, 0, 4)]),
                            MultiInvSlottedInput::new(label("Chromium Dust"), vec![(0, 1, 1)]),
                        ],
                        fluids: vec![],
                        max_sets: 4,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Ferrite Mixture Dust"), 64),
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
            accesses: inv_tank(s("gtceu:hv_mixer_0")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: fluid_extract_all(),
            recipes: (mixer_any().into_iter())
                .chain([FluidSlottedRecipe {
                    outputs: Output::new(label("Soldering Alloy Dust"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Tin Dust"), vec![(0, 0, 6)]),
                        MultiInvSlottedInput::new(label("Lead Dust"), vec![(0, 1, 3)]),
                        MultiInvSlottedInput::new(label("Antimony Dust"), vec![(0, 2, 1)]),
                    ],
                    fluids: vec![],
                    max_sets: 4,
                }])
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
            accesses: inv_tank(s("gtceu:hv_extractor_0")),
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
            accesses: inv_tank(s("gtceu:hv_chemical_bath_0")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: fluid_extract_all(),
            recipes: (mercury_bathed_ores.iter())
                .map(|&x| FluidSlottedRecipe {
                    outputs: Output::new(label!("Purified {x} Ore"), 64),
                    inputs: vec![MultiInvSlottedInput::new(label!("Crushed {x} Ore"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:mercury"), vec![(0, 100)])],
                    max_sets: 4,
                })
                .collect(),
            strict_priority: false,
        });
        factory.add_process(BlockingOutputConfig {
            accesses: acc(s("gtceu:hv_rock_crusher_0")),
            slot_filter: None,
            outputs: vec![Output { item: label("Obsidian"), n_wanted: 64 }],
        });
        factory.add_process(BlockingFluidOutputConfig {
            accesses: tank(s("ae2:cable_bus_1")),
            outputs: vec![
                FluidOutput { fluid: s("gtceu:oxygen"), n_wanted: 64_000 },
                FluidOutput { fluid: s("gtceu:high_octane_gasoline"), n_wanted: 64_000 },
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
            ],
        });
        factory.add_process(BlockingOutputConfig {
            accesses: acc(s("ae2:cable_bus_2")),
            slot_filter: None,
            outputs: vec![Output { item: label("Silicon Dust"), n_wanted: 64 }, Output { item: label("Carbon Dust"), n_wanted: 64 }],
        });
        factory.add_process(BufferedConfig {
            name: s("dump"),
            accesses: acc(s("ae2:cable_bus_2")),
            slot_filter: Some(Box::new(|i| i >= 2)),
            to_extract: None,
            recipes: vec![],
            max_recipe_inputs: 0,
            stocks: vec![BufferedInput::new(label("Calcium Dust"), i32::MAX), BufferedInput::new(label("Stone Dust"), i32::MAX)],
        });
    })
}
