use crate::factory::{Factory, FactoryConfig, FluidStorageConfig};
use crate::{access::*, config_util::*, process::*, recipe::*, storage::*};
use crate::{detail_cache::DetailCache, server::Server, Tui};
use flexstr::{local_fmt, LocalStr};
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

pub fn build_factory(tui: Rc<Tui>) -> Rc<RefCell<Factory>> {
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
        factory.add_process(LowAlert::new(label("Salt"), 64));
        factory.add_process(LowAlert::new(label("Iron Dust"), 64));
        factory.add_process(LowAlert::new(label("Rock Salt"), 64));
        factory.add_process(LowAlert::new(label("Carbon Dust"), 64));
        factory.add_process(LowAlert::new(label("Sodium Dust"), 64));
        factory.add_process(LowAlert::new(label("Sulfur Dust"), 64));
        factory.add_process(LowAlert::new(label("Copper Ingot"), 64));
        factory.add_process(LowAlert::new(label("Gallium Dust"), 64));
        factory.add_process(LowAlert::new(label("Redstone Dust"), 64));
        factory.add_process(LowAlert::new(label("Tantalite Dust"), 64));
        factory.add_process(LowAlert::new(label("Lepidolite Dust"), 64));
        factory.add_process(LowAlert::new(label("Silicon Dioxide Dust"), 64));
        factory.add_process(ManualUiConfig { accesses: vec![] });
        for i in [1, 3, 0] {
            factory.add_storage(ChestConfig { accesses: acc(local_fmt!("gtceu:titanium_crate_{i}")), override_max_stack_size: None });
        }
        for (capacity, addr, fluid) in [
            (2048_000, "functionalstorage:fluid_1_0", "gtceu:oxygen"),
            (256_000, "functionalstorage:fluid_1_1", "gtceu:high_octane_gasoline"),
            (2048_000, "functionalstorage:fluid_1_2", "gtceu:hydrogen"),
            (256_000, "functionalstorage:fluid_1_3", "gtceu:hydrogen_sulfide"),
            (256_000, "functionalstorage:fluid_1_4", "gtceu:sulfuric_acid"),
            (256_000, "functionalstorage:fluid_1_5", "gtceu:ethylene"),
            (256_000, "functionalstorage:fluid_1_6", "gtceu:polyethylene"),
            (256_000, "functionalstorage:fluid_1_7", "gtceu:chlorine"),
            (256_000, "functionalstorage:fluid_1_8", "minecraft:water"),
            (256_000, "functionalstorage:fluid_1_9", "gtceu:vinyl_chloride"),
            (256_000, "functionalstorage:fluid_1_10", "gtceu:hydrochloric_acid"),
            (256_000, "functionalstorage:fluid_1_11", "gtceu:polyvinyl_chloride"),
            (256_000, "functionalstorage:fluid_1_12", "gtceu:iron_iii_chloride"),
            (256_000, "functionalstorage:fluid_1_13", "gtceu:sodium_persulfate"),
            (256_000, "functionalstorage:fluid_1_14", "gtceu:fluorine"),
            (256_000, "functionalstorage:fluid_1_15", "gtceu:hydrofluoric_acid"),
            (256_000, "functionalstorage:fluid_1_16", "gtceu:hexafluorosilicic_acid"),
        ] {
            factory.add_fluid_storage(FluidStorageConfig { accesses: tank(s(addr)), fluid: s(fluid), capacity });
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
        let reactor_n = || {
            [
                FluidSlottedRecipe {
                    outputs: FluidOutput::new(s("gtceu:hydrogen"), 64_000),
                    inputs: vec![MultiInvSlottedInput::new(label("Sodium Dust"), vec![(0, 0, 1)])],
                    fluids: vec![FluidSlottedInput::new(s("minecraft:water"), vec![(0, 1_000)])],
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
                    outputs: FluidOutput::new(s("gtceu:hydrofluoric_acid"), 64_000),
                    inputs: vec![],
                    fluids: vec![
                        FluidSlottedInput::new(s("gtceu:hydrogen"), vec![(0, 1_000)]),
                        FluidSlottedInput::new(s("gtceu:fluorine"), vec![(0, 1_000)]),
                    ],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: FluidOutput::new(s("gtceu:hexafluorosilicic_acid"), 4_000),
                    inputs: vec![MultiInvSlottedInput::new(label("Silicon Dioxide Dust"), vec![(0, 0, 3)])],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:hydrofluoric_acid"), vec![(0, 6_000)])],
                    max_sets: 2,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Plastic Circuit Board"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Polyvinyl Chloride Sheet"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Copper Foil"), vec![(0, 1, 4)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:hexafluorosilicic_acid"), vec![(0, 30)])],
                    max_sets: 8,
                },
                FluidSlottedRecipe {
                    outputs: Output::new(label("Plastic Printed Circuit Board"), 64),
                    inputs: vec![
                        MultiInvSlottedInput::new(label("Plastic Circuit Board"), vec![(0, 0, 1)]),
                        MultiInvSlottedInput::new(label("Copper Foil"), vec![(0, 1, 6)]),
                    ],
                    fluids: vec![FluidSlottedInput::new(s("gtceu:iron_iii_chloride"), vec![(0, 250)])],
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
            recipes: (reactor_n().into_iter())
                .chain([
                    FluidSlottedRecipe {
                        outputs: FluidOutput::new(s("gtceu:polyethylene"), 64_000),
                        inputs: vec![],
                        fluids: vec![
                            FluidSlottedInput::new(s("gtceu:ethylene"), vec![(0, 144)]),
                            FluidSlottedInput::new(s("gtceu:oxygen"), vec![(0, 1_000)]),
                        ],
                        max_sets: 8,
                    },
                    FluidSlottedRecipe {
                        outputs: FluidOutput::new(s("gtceu:polyvinyl_chloride"), 64_000),
                        inputs: vec![],
                        fluids: vec![
                            FluidSlottedInput::new(s("gtceu:vinyl_chloride"), vec![(0, 144)]),
                            FluidSlottedInput::new(s("gtceu:oxygen"), vec![(0, 1_000)]),
                        ],
                        max_sets: 8,
                    },
                    FluidSlottedRecipe {
                        outputs: FluidOutput::new(s("gtceu:vinyl_chloride"), 64_000),
                        inputs: vec![],
                        fluids: vec![
                            FluidSlottedInput::new(s("gtceu:ethylene"), vec![(0, 1_000)]),
                            FluidSlottedInput::new(s("gtceu:chlorine"), vec![(0, 2_000)]),
                        ],
                        max_sets: 8,
                    },
                    FluidSlottedRecipe {
                        outputs: FluidOutput::new(s("gtceu:iron_iii_chloride"), 64_000),
                        inputs: vec![MultiInvSlottedInput::new(label("Iron Dust"), vec![(0, 0, 1)])],
                        fluids: vec![FluidSlottedInput::new(s("gtceu:hydrochloric_acid"), vec![(0, 3_000)])],
                        max_sets: 5,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Sodium Bisulfate Dust"), 64),
                        inputs: vec![MultiInvSlottedInput::new(label("Salt"), vec![(0, 0, 2)])],
                        fluids: vec![FluidSlottedInput::new(s("gtceu:sulfuric_acid"), vec![(0, 1_000)])],
                        max_sets: 8,
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
            recipes: (reactor_n().into_iter())
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
            name: s("plateSolidifier"),
            input_slots: vec![vec![]],
            input_tanks: vec![vec![0]],
            accesses: inv_tank(s("gtceu:hv_fluid_solidifier_0")),
            to_extract: Some(Box::new(|_, _, i, _| i == 1)),
            fluid_extract: None,
            recipes: vec![FluidSlottedRecipe {
                outputs: Output::new(label("Polyvinyl Chloride Sheet"), 64),
                inputs: vec![],
                fluids: vec![FluidSlottedInput::new(s("gtceu:polyvinyl_chloride"), vec![(0, 144)])],
                max_sets: 16,
            }],
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("electrolyzer"),
            input_slots: vec![vec![0]],
            input_tanks: vec![vec![0]],
            accesses: inv_tank(s("gtceu:hv_electrolyzer_0")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: fluid_extract_all(),
            recipes: [("material.gtceu.niobium_pentoxide Dust", 7), ("material.gtceu.manganese_difluoride Dust", 3)]
                .into_iter()
                .map(|(item, qty)| FluidSlottedRecipe {
                    outputs: ignore_outputs(2.),
                    inputs: vec![MultiInvSlottedInput::new(label(item), vec![(0, 0, qty)])],
                    fluids: vec![],
                    max_sets: 8,
                })
                .chain([
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Sodium Dust"), 64),
                        inputs: vec![MultiInvSlottedInput::new(label("Sodium Hydroxide Dust"), vec![(0, 0, 3)])],
                        fluids: vec![],
                        max_sets: 8,
                    },
                    FluidSlottedRecipe {
                        outputs: Output::new(label("Tantalum Dust"), 64),
                        inputs: vec![MultiInvSlottedInput::new(label("material.gtceu.tantalum_pentoxide Dust"), vec![(0, 0, 7)])],
                        fluids: vec![],
                        max_sets: 8,
                    },
                    FluidSlottedRecipe {
                        outputs: FluidOutput::new(s("gtceu:chlorine"), 64_000),
                        inputs: vec![MultiInvSlottedInput::new(label("Rock Salt"), vec![(0, 0, 2)])],
                        fluids: vec![],
                        max_sets: 8,
                    },
                    FluidSlottedRecipe {
                        outputs: FluidOutput::new(s("gtceu:fluorine"), 64_000),
                        inputs: vec![MultiInvSlottedInput::new(label("Lepidolite Dust"), vec![(0, 0, 20)])],
                        fluids: vec![],
                        max_sets: 3,
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
        factory.add_process(SlottedConfig {
            name: s("bender-10"),
            accesses: acc(s("gtceu:hv_bender_0")),
            input_slots: vec![0, 1],
            to_extract: extract_all(),
            recipes: vec![
                SlottedRecipe {
                    outputs: Output::new(label("Copper Foil"), 64),
                    inputs: vec![SlottedInput::new(label("Copper Ingot"), vec![(0, 1)])],
                    max_sets: 4,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Gallium Foil"), 64),
                    inputs: vec![SlottedInput::new(label("Gallium Ingot"), vec![(0, 1)])],
                    max_sets: 4,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Tantalum Foil"), 64),
                    inputs: vec![SlottedInput::new(label("Tantalum Ingot"), vec![(0, 1)])],
                    max_sets: 4,
                },
            ],
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("bender-1"),
            accesses: acc(s("gtceu:hv_bender_1")),
            input_slots: vec![0, 1],
            to_extract: extract_all(),
            recipes: vec![SlottedRecipe {
                outputs: Output::new(label("Thin Polyvinyl Chloride Sheet"), 64),
                inputs: vec![SlottedInput::new(label("Polyvinyl Chloride Sheet"), vec![(0, 1)])],
                max_sets: 4,
            }],
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("wiremill-3"),
            accesses: acc(s("gtceu:hv_wiremill_0")),
            input_slots: vec![0, 1],
            to_extract: extract_all(),
            recipes: vec![
                SlottedRecipe {
                    outputs: Output::new(label("Fine Tantalum Wire"), 64),
                    inputs: vec![SlottedInput::new(label("Tantalum Ingot"), vec![(0, 1)])],
                    max_sets: 4,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Fine Red Alloy Wire"), 64),
                    inputs: vec![SlottedInput::new(label("Red Alloy Ingot"), vec![(0, 1)])],
                    max_sets: 4,
                },
            ],
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("furnace"),
            accesses: acc(s("gtceu:hv_electric_furnace_0")),
            input_slots: vec![0],
            to_extract: extract_all(),
            recipes: vec![
                SlottedRecipe {
                    outputs: Output::new(label("Gallium Ingot"), 64),
                    inputs: vec![SlottedInput::new(label("Gallium Dust"), vec![(0, 1)])],
                    max_sets: 4,
                },
                SlottedRecipe {
                    outputs: Output::new(label("Tantalum Ingot"), 64),
                    inputs: vec![SlottedInput::new(label("Tantalum Dust"), vec![(0, 1)])],
                    max_sets: 4,
                },
            ],
            strict_priority: false,
        });
        factory.add_process(SlottedConfig {
            name: s("alloySmelter"),
            accesses: acc(s("gtceu:hv_alloy_smelter_0")),
            input_slots: vec![0, 1],
            to_extract: extract_all(),
            recipes: vec![SlottedRecipe {
                outputs: Output::new(label("Red Alloy Ingot"), 64),
                inputs: vec![SlottedInput::new(label("Copper Ingot"), vec![(0, 1)]), SlottedInput::new(label("Redstone Dust"), vec![(1, 4)])],
                max_sets: 8,
            }],
            strict_priority: false,
        });
        let asm_n = || {
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
            ]
        };
        factory.add_process(FluidSlottedConfig {
            name: s("asm-1"),
            input_slots: vec![vec![0, 1, 2, 3, 4, 5, 6, 7, 8]],
            input_tanks: vec![vec![0]],
            accesses: inv_tank(s("gtceu:hv_assembler_0")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: None,
            recipes: asm_n().into_iter().collect(),
            strict_priority: false,
        });
        factory.add_process(FluidSlottedConfig {
            name: s("chemicalBath"),
            input_slots: vec![vec![0]],
            input_tanks: vec![vec![0]],
            accesses: inv_tank(s("gtceu:hv_chemical_bath_0")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: fluid_extract_all(),
            recipes: vec![FluidSlottedRecipe {
                outputs: Output::new(label("material.gtceu.tantalum_pentoxide Dust"), 64),
                inputs: vec![MultiInvSlottedInput::new(label("Tantalite Dust"), vec![(0, 0, 9)])],
                fluids: vec![FluidSlottedInput::new(s("gtceu:hydrofluoric_acid"), vec![(0, 2_000)])],
                max_sets: 7,
            }],
            strict_priority: false,
        });
        factory.add_process(BlockingFluidOutputConfig {
            accesses: tank(s("ae2:cable_bus_0")),
            outputs: vec![
                FluidOutput { fluid: s("gtceu:oxygen"), n_wanted: 64_000 },
                FluidOutput { fluid: s("gtceu:high_octane_gasoline"), n_wanted: 64_000 },
                FluidOutput { fluid: s("gtceu:ethylene"), n_wanted: 64_000 },
                FluidOutput { fluid: s("minecraft:water"), n_wanted: 64_000 },
            ],
        });
    })
}
