use crate::factory::{Factory, FactoryConfig, FluidStorageConfig};
use crate::{access::*, config_util::*, process::*, recipe::*, storage::*};
use crate::{detail_cache::DetailCache, server::Server, Tui};
use flexstr::{local_fmt, LocalStr};
use std::{cell::RefCell, rc::Rc, time::Duration};

const MAIN: &str = "main";
const BUS: &str = "minecraft:barrel_0";
fn fbus() -> Vec<LocalStr> { vec![s("enderio:pressurized_fluid_tank_0"), s("enderio:pressurized_fluid_tank_1")] }
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
        factory.add_process(LowAlert::new(label("Sodium Dust"), 64));
        factory.add_process(LowAlert::new(label("Sulfur Dust"), 64));
        factory.add_process(ManualUiConfig { accesses: vec![] });
        for i in [0, 1] {
            factory.add_storage(ChestConfig { accesses: acc(local_fmt!("sophisticatedstorage:chest_{i}")), override_max_stack_size: None });
        }
        for (capacity, addr, fluid) in [
            (256_000, "functionalstorage:fluid_1_0", "gtceu:oxygen"),
            (256_000, "functionalstorage:fluid_1_1", "gtceu:naphtha"),
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
        ] {
            factory.add_fluid_storage(FluidStorageConfig { accesses: tank(s(addr)), fluid: s(fluid), capacity });
        }
        for (addr, fluid, qty) in [
            ("gtceu:hv_combustion_4", "gtceu:naphtha", 16_000),
            ("gtceu:hv_combustion_5", "gtceu:naphtha", 16_000),
            ("gtceu:hv_combustion_6", "gtceu:naphtha", 16_000),
            ("gtceu:hv_combustion_7", "gtceu:naphtha", 16_000),
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
            ]
        };
        factory.add_process(FluidSlottedConfig {
            name: s("reactor-1"),
            input_slots: vec![vec![0]],
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
            input_slots: vec![vec![0]],
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
            name: s("electrolyzer"),
            input_slots: vec![vec![0]],
            input_tanks: vec![vec![0]],
            accesses: inv_tank(s("gtceu:hv_electrolyzer_0")),
            to_extract: multi_inv_extract_all(),
            fluid_extract: fluid_extract_all(),
            recipes: vec![
                FluidSlottedRecipe {
                    outputs: Output::new(label("Sodium Dust"), 64),
                    inputs: vec![MultiInvSlottedInput::new(label("Sodium Hydroxide Dust"), vec![(0, 0, 3)])],
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
                    outputs: FluidOutput::new(s("gtceu:sodium_persulfate"), 64_000),
                    inputs: vec![MultiInvSlottedInput::new(label("Sodium Bisulfate Dust"), vec![(0, 0, 7)])],
                    fluids: vec![],
                    max_sets: 8,
                },
            ],
            strict_priority: false,
        });
        factory.add_process(BlockingFluidOutputConfig {
            accesses: tank(s("ae2:cable_bus_0")),
            outputs: vec![
                FluidOutput { fluid: s("gtceu:oxygen"), n_wanted: 64_000 },
                FluidOutput { fluid: s("gtceu:naphtha"), n_wanted: 64_000 },
                FluidOutput { fluid: s("gtceu:ethylene"), n_wanted: 64_000 },
                FluidOutput { fluid: s("minecraft:water"), n_wanted: 64_000 },
            ],
        });
    })
}
