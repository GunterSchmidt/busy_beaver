#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused)]
// #![allow(unreachable_code)]

// mod test_bb5;
// mod test_machines;
// mod result_worker;
// mod test;
// mod test_run_deciders;
mod struct_sizes;
// mod test_single_deciders;

use std::{
    env::current_dir,
    thread,
    time::{Duration, Instant},
};

use bb_challenge::{
    arg_handler,
    config::{Config, CoreUsage},
    data_provider::{bb_file_reader, enumerator_binary::EnumeratorType},
    decider::{
        decider_bouncer_128::DeciderBouncer128,
        decider_cycler::DeciderCycler,
        decider_engine,
        decider_hold_long::DeciderHoldLong,
        pre_decider::{self, PreDecider, PreDeciderRun},
        Decider, DeciderConfig, DeciderStandard,
    },
    html,
    machine_binary::{MachineBinary, NotableMachineBinary},
    status::MachineStatus,
};
// use bb_challenge_work::{
//     data_provider::{
//         generator::{Generator, GeneratorStandard},
//         generator_binary::GeneratorType,
//         DataProvider,
//     },
//     decider::{
//         self, decider_bouncer_128_speed_up, decider_bouncer_apex, decider_cycler, decider_engine,
//         decider_hold_compact::DeciderHoldCompact, decider_hold_long_v3::DeciderHoldLong, Decider,
//         DeciderConfig, DeciderStandard,
//     },
//     html,
//     machine_id::MachineId,
//     machine_info::MachineInfo,
//     reporter::Reporter,
//     single_thread_worker::SingleThreadWorker,
//     CoreUsage,
// };

use busy_beaver::{
    // test_run_deciders::{run_generator_pre_deciders, run_generator_pre_deciders_undecided},
    GENERATOR_BATCH_SIZE_REQUEST,
    GENERATOR_LIMIT,
    N_STATES,
    STEP_LIMIT,
    TAPE_SIZE_LIMIT,
};

use crate::struct_sizes::show_struct_sizes;

// use test_machines::run_machine;
// use test_single_deciders::{test_expanding_loop, test_expanding_sinus};

// TODO tape: speed_up, move data to tape
// TODO benchmark: Why is GeneratorReverse so slow?

// Cycler ID: 60202 1RB0RZ_0RC0RC_1LD1RC_0LD0LE_1RA1RC: left shift panic at line 339
// TODO Validate Cycler / Bouncer by checking DecidedEndless machines by Hold decider
// TODO HTML Step Limit -> Line Limit
// TODO Benchmarks really bad

// TODO Worker as single thread
// TODO review bb_challenge article
// TODO status increasing pre-decider
//  Machine No.   191,658,921: 1RB1LB_1LA0LC_---1LD_1RD0RA
//    Machine No. 5,721,093,031: 1RB1LB_1LA0LD_1RC0RA_---1LC
// TODO generator backwards?
// TODO threaded: Worker as single thread, atomic update save (see below test_single_thread_worker)
// TODO threaded: recycle threads / thread pool
// TODO threaded: Why so slow under windows?

fn test_timed() {
    let start = std::time::Instant::now();
    bb_challenge::decider::decider_hold_long::test_decider_hold_u128_applies_bb5_max();
    let duration = start.elapsed();
    println!("Duration: {duration:?}");
}

fn test_single_machine_binary() {
    let start = std::time::Instant::now();
    let machine = NotableMachineBinary::BB3Rado.machine();
    let config_single = Config::builder(machine.n_states())
        .write_html_file(true)
        .write_html_line_limit(25_000)
        .step_limit_decider_cycler(50_000)
        .step_limit_decider_bouncer(5000)
        .build();
    // let status = DeciderHoldLong::decide_single_machine(&machine, &config_single);
    // println!("Machine: {}", status);
    let duration = start.elapsed();
    println!("Duration: {duration:?}");
}

/// Main function for tests, running deciders and other stuff.
/// Arguments:
///   - No Arguments: manually defined code
///   - \<number>: Machine ID from bb_challenge provided BB5 file.
///   - \<name>: Machine from defined set in fn 'build_machine'.
///   - rewrite: Rewrites the bb_challenge file into a smaller format (no reader exists)
fn main() {
    // read the arguments of the command line for 30 arguments
    let args: Vec<String> = std::env::args().collect();
    // println!("Arguments len: {}", args.len());
    println!("Arguments: {:?}", args);

    // No arguments
    if args.len() < 2 {
        // show_struct_sizes();
        // bb_challenge::decider::decider_hold_long::test_decider_hold_u128_applies_bb5_max();
        // test_single_machine_binary();
        evaluate_bb_challenge_file();
        return;

        let n_states = 3;
        let decider_last = 4;
        let config_1 = Config::builder(n_states)
            // 10_000_000_000 for BB4
            .machine_limit(1000_000_000_000)
            // .limit_machines_undecided(200)
            // .machine_limit(50_000_000)
            // .step_limit_cycler(1500)
            // .step_limit_bouncer(5000)
            // .step_limit_hold(1_000_000)
            // .limit_machines_decided(100000)
            // if set, then these machines will be printed to disk
            // .generator_first_rotate_field_front(true)
            // .generator_full_batch_size_request(10_000)
            // .generator_reduced_batch_size_request(8_000_000)
            // .write_html_file(true)
            // .cpu_utilization(25)
            .build();
        println!("Config 1: {config_1}");
        let config_2 = Config::builder_from_config(&config_1)
            // .machine_limit(100_000_000_000)
            .step_limit_decider_cycler(110_000)
            // .step_limit_bouncer(5_000)
            // .limit_machines_undecided(100)
            // .step_limit_cycler(50_000)
            // .step_limit_bouncer(200_000)
            // .limit_machines_decided(100)
            // .limit_machines_undecided(100)
            // .write_html_file(true)
            .build();
        println!("Config 2: {config_2}");

        let decider_configs = build_decider_config(&config_1, &config_2);

        let result = decider_engine::run_decider_chain_gen(
            &decider_configs[0..decider_last],
            // EnumeratorType::EnumeratorFullForward,
            // EnumeratorType::EnumeratorReducedForward,
            EnumeratorType::EnumeratorReducedBackward,
            CoreUsage::MultiCore,
        );

        let mut names = Vec::new();
        for d in decider_configs[0..decider_last].iter() {
            names.push(d.decider_id().name);
        }
        println!();
        println!("Decider: {}", names.join(", "));
        println!("Config 1: {config_1}");
        if decider_last > 2 {
            println!("Config 2: {config_2}");
        }
        println!("\n{}", result.to_string_with_duration());

        let mr = result.machine_max_steps();
        match mr {
            Some(m) => {
                println!("M Max: {m}");
                println!("ID normalized: {}", m.calc_normalized_id());
            }
            None => println!("M Max: no machine halts"),
        }

        // write undecided to html
        if let Some(m_undecided) = result.machines_undecided() {
            let config = Config::builder_from_config(&config_1)
                .step_limit_decider_cycler(100_000)
                .step_limit_decider_bouncer(100_000)
                .step_limit_decider_halt(100_000)
                .write_html_line_limit(25_000)
                .build();
            html::write_machines_to_html(&m_undecided, "undecided", &config, 1000, false);
        }

        // write decided hold to html
        if let Some(m_decided) = result.machines_decided() {
            let config = Config::builder_from_config(&config_1)
                .step_limit_decider_cycler(100_000)
                .step_limit_decider_bouncer(100_000)
                .step_limit_decider_halt(100_000)
                .write_html_line_limit(25_000)
                .build();
            // let hold_count = m_decided.iter().filter(|m| m.status() == MachineStatus::)
            let mut m_hold = Vec::new();
            for m in m_decided.iter() {
                if let MachineStatus::DecidedHalts(_) = m.status() {
                    m_hold.push(*m);
                }
            }
            html::write_machines_to_html(&m_hold, "hold", &config, 1000, false);
        }
    } else {
        // use argument handler
        let arg_value = arg_handler::standard_args(&args);
        let mut machine: Option<MachineBinary> = None;
        match arg_value {
            // arg_handler::ArgValue::Machine(m) => machine = Some(*m),
            arg_handler::ArgValue::Machine(table) => {
                if table.has_two_symbols() {
                    let t = MachineBinary::try_from(*table);
                    match t {
                        Ok(tc) => machine = Some(tc),
                        Err(e) => println!("{e}"),
                    }
                } else {
                    println!(
                        "This machine has {} symbols and cannot be handled here.",
                        table.dimensions().n_symbols
                    );
                    println!("{table}");
                }
            }
            arg_handler::ArgValue::Done => {}
            arg_handler::ArgValue::None => {}
            arg_handler::ArgValue::Error(e) => {
                println!("Could not parse arguments: {e}");
                return;
            }
            _ => todo!(),
        }

        if let Some(machine) = machine {
            println!("{machine}");
            // let res = machine.decide_hold();
            let config = Config::builder(machine.n_states())
                .step_limit_decider_halt(1000)
                .write_html_file(true)
                .build();
            let mut res = pre_decider::run_pre_decider_simple(&machine);
            if res == MachineStatus::NoDecision {
                res = DeciderCycler::decide_single_machine(&machine, &config);
            }
            if let MachineStatus::Undecided(_, _, _) = res {
                res = DeciderBouncer128::decide_single_machine(&machine, &config);
            }
            if let MachineStatus::Undecided(_, _, _) = res {
                res = DeciderHoldLong::decide_single_machine(&machine, &config);
            }
            println!("Result: {res}");
        }
    }
}

fn build_decider_config<'a>(config_1: &'a Config, config_2: &'a Config) -> Vec<DeciderConfig<'a>> {
    // Decider
    let mut dc_cycler_1 = DeciderStandard::Cycler.decider_config(config_1);
    // dc_cycler_1.fo_result_worker = Some(result_worker::cycler_html_filter);
    // let dc_cycler_1 = DeciderConfig::new(
    //     bb_challenge_work::decider_cycler_v5::DeciderCycler::decider_id(),
    //     bb_challenge_work::decider_cycler::DeciderCycler::decider_run_batch,
    //     // result_worker::cycler_html_filter,
    //     &config_1,
    // );
    let dc_bouncer_1 = DeciderStandard::Bouncer128.decider_config(config_2);
    // let dc_bouncer_1_self_ref = DeciderConfig::new(
    //     bb_challenge_work::decider::decider_bouncer_128::DeciderBouncer128::decider_id(),
    //     decider_bouncer_128_speed_up::DeciderBouncer128::decider_run_batch,
    //     config_1,
    // );
    // let dc_bouncer_1_apex = DeciderConfig::new(
    //     decider_bouncer_apex::DeciderBouncerApex::decider_id(),
    //     decider_bouncer_apex::DeciderBouncerApex::decider_run_batch,
    //     config_1,
    // );
    // let dc_bouncer_1 = DeciderConfig::new_with_worker(
    //     bb_challenge_work::decider_bouncer_v2::DeciderBouncerV2::decider_id(),
    //     bb_challenge_work::decider_bouncer_v2::DeciderBouncerV2::decider_run_batch,
    //     result_worker::bouncer_html_filter,
    //     config_2,
    // );
    let dc_hold = DeciderStandard::Hold.decider_config(config_1);
    let dc_cycler_2 = DeciderStandard::Cycler.decider_config(config_2);
    // let dc_cycler_2 = DeciderConfig::new_with_worker(
    //     bb_challenge_work::decider_cycler_v4::DeciderCycler::decider_id(),
    //     bb_challenge_work::decider_cycler_v4::DeciderCycler::decider_run_batch,
    //     result_worker::cycler_html_filter,
    //     config_2,
    // );
    // let dc_bouncer_2 = DeciderStandard::Bouncer128.decider_config(config_2);

    let decider_config = vec![
        dc_cycler_1,
        // dc_bouncer_1_apex,
        dc_bouncer_1,
        // dc_bouncer_1_self_ref,
        dc_cycler_2,
        // dc_bouncer_2,
        dc_hold,
    ];

    decider_config
}

fn evaluate_bb_challenge_file() {
    let n_states = 5;
    let decider_last = 3;
    let config_1 = Config::builder(n_states)
        // .step_limit_cycler(1500)
        // .step_limit_bouncer(5000)
        // .step_limit_hold(1_000_000)
        // .limit_machines_decided(100000)
        // if set, then these machines will be printed to disk
        // .limit_machines_undecided(200)
        .file_id_range(0..30_000)
        .write_html_file(true)
        .build();
    println!("Config 1: {config_1}");
    let config_2 = Config::builder_from_config(&config_1)
        // .write_html_file(true)
        .build();
    println!("Config 2: {config_2}");

    let decider_configs = build_decider_config(&config_1, &config_2);

    // run bb_challenge file
    assert_eq!(5, config_1.n_states());
    let result = bb_file_reader::run_deciders_bb_challenge_file(
        &decider_configs[0..decider_last],
        CoreUsage::SingleCoreEnumeratorMultiCoreDecider,
    );

    let mut names = Vec::new();
    for d in decider_configs[0..decider_last].iter() {
        names.push(d.decider_id().name);
    }
    println!();
    println!("Decider: {}", names.join(", "));
    println!("Config 1: {config_1}");
    if decider_last > 2 {
        println!("Config 2: {config_2}");
    }
    println!("\n{}", result.to_string_with_duration());

    let mr = result.machine_max_steps();
    match mr {
        Some(m) => {
            println!("M Max: {m}");
            println!("ID normalized: {}", m.calc_normalized_id());
        }
        None => println!("M Max: no machine halts"),
    }

    // write undecided to html
    if let Some(m_undecided) = result.machines_undecided() {
        let config = Config::builder_from_config(&config_1)
            .step_limit_decider_cycler(100_000)
            .step_limit_decider_bouncer(100_000)
            .step_limit_decider_halt(100_000)
            .write_html_line_limit(25_000)
            .build();
        html::write_machines_to_html(&m_undecided, "undecided", &config, 1000, false);
    }

    // // write decided hold to html
    // if let Some(m_decided) = result.machines_decided() {
    //     let config = Config::builder_from_config(&config_1)
    //         .step_limit_decider_cycler(100_000)
    //         .step_limit_decider_bouncer(100_000)
    //         .step_limit_decider_halt(100_000)
    //         .write_html_line_limit(25_000)
    //         .build();
    //     // let hold_count = m_decided.iter().filter(|m| m.status() == MachineStatus::)
    //     let mut m_hold = Vec::new();
    //     for m in m_decided.iter() {
    //         if let MachineStatus::DecidedHalts(_) = m.status() {
    //             m_hold.push(*m);
    //         }
    //     }
    //     html::write_machines_to_html(&m_hold, "hold", &config, 1000, false);
    // }
}
