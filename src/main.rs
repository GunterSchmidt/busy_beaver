#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused)]
// #![allow(unreachable_code)]

// mod test_bb5;
// mod test_machines;
mod result_worker;
mod test;
mod test_run_deciders;
mod test_single_deciders;

use std::{
    env::current_dir,
    thread,
    time::{Duration, Instant},
};

use bb_challenge::{
    arg_handler,
    bb_file_reader::{BBFileDataProviderBuilder, BBFileReader},
    config::{self, Config},
    data_provider::DataProvider,
    decider::{self, Decider, DeciderConfig, DeciderStandard},
    decider_bouncer::DeciderBouncer,
    decider_engine::{
        run_decider_chain_data_provider_single_thread,
        run_decider_chain_data_provider_single_thread_reporting,
        run_decider_chain_threaded_data_provider_multi_thread,
        run_decider_chain_threaded_data_provider_single_thread,
        run_decider_chain_threaded_data_provider_single_thread_reporting,
    },
    decider_hold_u128_long::DeciderHoldU128Long,
    decider_result::{BatchResult, DeciderResultStats},
    decider_result_worker::{self},
    generator::Generator,
    generator_full::GeneratorFull,
    generator_reduced::GeneratorReduced,
    html,
    machine::Machine,
    machine_info::MachineInfo,
    pre_decider::{self, PreDecider, PreDeciderRun},
    reporter::Reporter,
    single_thread_worker::SingleThreadWorker,
    status::MachineStatus,
    transition_symbol2::TransitionTableSymbol2,
    Cores,
};

use busy_beaver::{
    test_run_deciders::{run_generator_pre_deciders, run_generator_pre_deciders_undecided},
    GENERATOR_BATCH_SIZE_REQUEST, GENERATOR_LIMIT, N_STATES, STEP_LIMIT, TAPE_SIZE_LIMIT,
};

// use test_machines::run_machine;
// use test_single_deciders::{test_expanding_loop, test_expanding_sinus};

// TODO HTML Table
// TODO Worker as single thread
// TODO review bb_challenge article
// TODO status increasing pre-decider
//  Machine No.   191,658,921: 1RB1LB_1LA0LC_---1LD_1RD0RA
//    Machine No. 5,721,093,031: 1RB1LB_1LA0LD_1RC0RA_---1LC
// TODO generator backwards?
// TODO threaded: undefined and worker
// TODO threaded: atomic update save
// TODO threaded: recycle threads / thread pool
// TODO threaded: Why so slow under windows?

fn test_single_thread_worker() {
    // Example of the single thread worker, addressing "TODO Worker as single thread"
    println!("--- Single Thread Worker Example ---");
    let (worker, worker_handle) = SingleThreadWorker::new();

    // Submit a job from the main thread.
    worker.execute(|| {
        println!("[Worker] This is job 1, running in the worker thread.");
        thread::sleep(Duration::from_millis(500));
        println!("[Worker] Job 1 finished.");
    });
    println!("[Main] Job 1 submitted from main thread.");

    // To submit a job from another thread, we clone the worker handle.
    let worker_for_spawn = worker.clone();
    let spawned_handle = thread::spawn(move || {
        println!("[Spawned Thread] Submitting job 2 from a spawned thread.");
        worker_for_spawn.execute(|| {
            println!("[Worker] This is job 2, running in the worker thread.");
            thread::sleep(Duration::from_millis(100));
            println!("[Worker] Job 2 finished.");
        });
        // worker_for_spawn is dropped here when the thread finishes.
    });

    // We must wait for the spawned thread to complete. This ensures its handle is dropped.
    spawned_handle.join().unwrap();

    // Drop the original worker handle to signal shutdown.
    // The worker will finish all queued jobs before stopping.
    drop(worker);
    worker_handle.join().unwrap();
    println!("[Main] Worker thread has finished.");
    println!("--- End of Example ---\n");
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
    // println!("Arguments: {:?}", args);)

    // No arguments
    // Done: what is the issue after 409_975_399?
    if args.len() < 2 {
        let start = std::time::Instant::now();
        // test_single_thread_worker();
        bb_challenge::decider_hold_u128_long::test_decider_hold_u128_applies_bb5_max();
        let duration = start.elapsed();
        println!("Duration: {duration:?}");
        let start = std::time::Instant::now();
        // test_single_thread_worker();
        bb_challenge::decider_hold_u128_long_v2::test_decider_hold_u128_applies_bb5_max();
        let duration = start.elapsed();
        println!("Duration: {duration:?}");
        return;

        let n_states = 5;
        let decider_last = 1;
        let config_1 = Config::builder(n_states)
            // 10_000_000_000 for BB4
            .machine_limit(100_500_000_000)
            .step_limit_hold(1000)
            .step_limit_cycler(1500)
            .step_limit_bouncer(200)
            // .limit_machines_decided(100_000)
            .limit_machines_undecided(100)
            // .file_id_range(0..1_000_000)
            // .generator_batch_size_request_full(5_000_000)
            // .generator_reduced_batch_size_request(100_000)
            // .limit_machines_undecided(20)
            // .write_html_file(true)
            .cpu_utilization(100)
            .build();
        println!("Config 1: {config_1}");
        let config_2 = Config::builder_from_config(&config_1)
            // .machine_limit(100_000_000_000)
            .step_limit_cycler(500)
            .step_limit_bouncer(150_000)
            .limit_machines_decided(100_000)
            // .step_limit_cycler(50_000)
            // .step_limit_bouncer(200_000)
            .build();
        println!("Config 2: {config_2}");

        let decider_configs = build_decider_config(&config_1, &config_2);

        // single machine
        // let config_single = Config::builder(4).write_html_file(true).build();
        // let id = 1636268618;
        // let machine =
        //     Machine::from_standard_tm_text_format(id, &"1RB0RB_1LC1RA_1RD1LC_---1RB").unwrap();
        // let ms = bb_challenge::decider_cycler_v5::DeciderCycler::decide_single_machine(
        //     &machine,
        //     &config_single,
        // );
        // println!("M {id}: {}", ms);
        // return;

        let result = test_run_multiple_decider(&decider_configs[0..decider_last], Cores::MultiCore);
        // let result = test_file_read_v2(
        //     &decider_configs[0..decider_last],
        //     MultiCore::SingleCoreGeneratorMultiCoreDecider,
        // );

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

        // write undecided to html
        if let Some(m_undecided) = result.machines_undecided_sorted() {
            println!(
                "Writing {} undecided machines to html...",
                m_undecided.len()
            );
            let config = Config::builder_from_config(&config_1)
                .write_html_file(true)
                .build();
            if config.write_html_file() {
                for m_info in m_undecided.iter().take(1000) {
                    let machine = Machine::from(m_info);
                    bb_challenge::decider_cycler_v5::DeciderCycler::decide_single_machine(
                        &machine, &config,
                    );
                    DeciderHoldU128Long::decide_single_machine(&machine, &config);
                }
                println!("done.");
            }
        }

        // show_struct_sizes();

        // run single machine
        // let machine = TM::build_machine("SA_BB2_MAX").unwrap();
        // let status = DeciderU128Long::<DeciderDummy>::run_decider(&machine);
        // println!("{status}");

        // run_file(FILE_PATH, 1, 10000);
        // return;
        // variants();
        // variants_batches();
        // test_transition_compare_create();
        // test_transition_compare_retrieve();
        // test_transition_compare_lookup();
        // test_generator_new();
        // test_generator_old();
        // run_decider_loop_threaded(N_STATES);
        // let r = run_decider_loop_generator_threaded(N_STATES);
        // println!("{}", r.to_string_extended());
        // run_decider_back_to_square_one(N_STATES);
        // run_decider_loop_compact(N_STATES);
        // test_expanding_sinus();
        // test_expanding_loop();
        // perf_test_number_type();
        // let r = run_generator_pre_deciders(N_STATES);
        // r.print_result_extended();
        // let r = run_generator_pre_deciders_undecided(N_STATES);
        // r.print_result_extended();

        // let r = run_generator_decider_loop_as_decider(N_STATES, true);
        // println!("{}", r.to_string_extended());
        // bb_challenge::decider_hold::test_decider_hold_u128_long_applies_bb5_max();
        // bb_challenge::decider_u128::test_decider_hold_u128_applies_not_bb5_max();
        // bb_challenge::decider_u128_long::test_decider_hold_u128_applies_bb5_max();
        // return;

        // let mut machine = build_machine("BB5_S107").unwrap();
        // let mut machine = build_machine("BB4_MAX_V2").unwrap();
        // let mut machine = build_machine("BB3_TEST").unwrap();
        // #[allow(unused_mut)]
        // let mut machine = build_machine("BB3_SINUS").unwrap();
        // machine.info.id = 322636617;
        //         let _steps = machine.n_states();
        //         println!("{}", machine);
        //
        //         return;
    } else {
        // use argument handler
        let arg_value = arg_handler::standard_args(&args);
        let mut machine: Option<Machine> = None;
        match arg_value {
            arg_handler::ArgValue::Machine(m) => machine = Some(*m),
            arg_handler::ArgValue::TransitionTableGeneric(table) => {
                if table.has_two_symbols() {
                    let t = TransitionTableSymbol2::try_from(*table);
                    match t {
                        Ok(tc) => machine = Some(Machine::new(0, tc)),
                        Err(e) => println!("{e}"),
                    }
                } else {
                    println!(
                        "This machine has {} symbols and cannot be handled here.",
                        table.dimensions_slow().n_symbols
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
                .step_limit_hold(1000)
                .write_html_file(true)
                .build();
            let res = pre_decider::run_pre_decider_simple(machine.transition_table());
            if res == MachineStatus::NoDecision {
                let res = DeciderHoldU128Long::decide_single_machine(&machine, &config);
                let res = bb_challenge::decider_cycler_v5::DeciderCycler::decide_single_machine(
                    &machine, &config,
                );
            }
            println!("Result: {res}");
        }
    }
}

fn build_decider_config<'a>(config_1: &'a Config, config_2: &'a Config) -> Vec<DeciderConfig<'a>> {
    // Decider
    let mut dc_cycler_1 = DeciderStandard::Cycler.decider_config(config_1);
    dc_cycler_1.fo_result_worker = Some(result_worker::cycler_html_filter);
    // let dc_cycler_1 = DeciderConfig::new(
    //     bb_challenge::decider_cycler_v5::DeciderCycler::decider_id(),
    //     bb_challenge::decider_cycler_v5::DeciderCycler::decider_run_batch,
    //     // result_worker::cycler_html_filter,
    //     &config_1,
    // );
    // let dc_bouncer_1 = DeciderStandard::Bouncer.decider_config(config_1);
    let dc_hold = DeciderStandard::Hold.decider_config(config_1);
    // let dc_cycler_2 = DeciderStandard::Cycler.decider_config(config_2);
    let dc_cycler_2 = DeciderConfig::new_with_worker(
        bb_challenge::decider_cycler_v4::DeciderCycler::decider_id(),
        bb_challenge::decider_cycler_v4::DeciderCycler::decider_run_batch,
        result_worker::cycler_html_filter,
        config_2,
    );
    let dc_bouncer_2 = DeciderStandard::Bouncer.decider_config(config_2);

    let decider_config = vec![
        dc_cycler_1,
        // dc_bouncer_1,
        dc_cycler_2,
        dc_bouncer_2,
        dc_hold,
    ];

    decider_config
}

fn test_run_multiple_decider(
    decider_config: &[DeciderConfig],
    multi_core: Cores,
) -> DeciderResultStats {
    // let f_result_worker = decider_result_worker::no_worker;
    let first_config = decider_config.first().expect("No decider given").config();

    // let generator = GeneratorFull::new(first_config);
    let generator = GeneratorReduced::new(first_config);
    match multi_core {
        Cores::SingleCore => {
            run_decider_chain_data_provider_single_thread(decider_config, generator)
        }
        Cores::SingleCoreGeneratorMultiCoreDecider => {
            run_decider_chain_threaded_data_provider_single_thread(decider_config, generator)
        }
        Cores::MultiCore => {
            run_decider_chain_threaded_data_provider_multi_thread(decider_config, generator)
        }
        _ => panic!("use 0: single, 1: multi with single generator, 2: multi"),
    }
}

fn test_file_read(decider_config: &[DeciderConfig], multi_core: Cores) -> DeciderResultStats {
    let first_config = decider_config.first().unwrap().config();
    let r = BBFileDataProviderBuilder::new()
        .id_range(first_config.file_id_range())
        .batch_size(200)
        .build();
    let mut bb_file_reader;
    match r {
        Ok(f) => bb_file_reader = f,
        Err(e) => {
            panic!("File Reader could not be build: {e}");
        }
    }
    // println!("Reader: {:?}", bb_file_reader);
    // let r = bb_file_reader.machine_batch_next();
    // println!("machines: {}", r.machines.len());

    match multi_core {
        Cores::SingleCore => {
            run_decider_chain_data_provider_single_thread(decider_config, bb_file_reader)
        }
        Cores::SingleCoreGeneratorMultiCoreDecider => {
            run_decider_chain_threaded_data_provider_single_thread(decider_config, bb_file_reader)
        }
        // 2 => run_decider_chain_threaded_data_provider_multi_thread(decider_config, bb_file_reader),
        _ => panic!("use 0: single, 1: multi with single generator, 2: multi not allowed"),
    }
}

fn bench_generate_reduced(n_states: usize, generate_limit: u64) {
    let config = Config::builder(n_states)
        .machine_limit(generate_limit)
        .build();
    let mut generator = bb_challenge::generator_reduced::GeneratorReduced::new(&config);

    let mut p_count: usize = 0;
    loop {
        let (permutations, is_last_batch) = generator.generate_permutation_batch_next();
        p_count += permutations.len();
        // println!("Size: {}", permutations.len());

        if is_last_batch {
            break;
        }
    }
    println!("Size Total: {p_count}");
}

fn show_struct_sizes() {
    println!(
        "Config: {}",
        std::mem::size_of::<bb_challenge::config::Config>()
    );
    println!(
        "Result: {}",
        std::mem::size_of::<bb_challenge::decider_result::DeciderResultStats>()
    );
    println!(
        "DataProviderResult: {}",
        std::mem::size_of::<bb_challenge::data_provider::DataProviderBatch>()
    );
    println!(
        "PreDeciderCount: {}",
        std::mem::size_of::<bb_challenge::decider_result::PreDeciderCount>()
    );

    println!("Machine: {}", std::mem::size_of::<Machine>());
    println!("MachineInfo: {}", std::mem::size_of::<MachineInfo>());
    println!("MachineStatus: {}", std::mem::size_of::<MachineStatus>());
    println!(
        "TransitionCompact: {}",
        std::mem::size_of::<bb_challenge::transition_symbol2::TransitionSymbol2>()
    );

    // struct TestString {
    //     name: String,
    // }
    // struct TestOption {
    //     // name: Option<String>,
    //     machine_max_steps: Option<Box<MachineInfo>>,
    //     pub machines_max_steps: Option<Vec<MachineInfo>>,
    //     // machine_undecided: Option<MachineInfo>,
    //     pub machines_undecided: Option<Vec<MachineInfo>>,
    // }
    // println!("String: {}", std::mem::size_of::<TestString>());
    // println!("Option: {}", std::mem::size_of::<TestOption>());
}

// This triggers machine.run() for the given permutations on a single thread.
// This is outdated and highly inefficient.
// TODO implement general machine run decider or use existing one
// Solve issue with progress, possibly use progress bar
// Solve issue with batch size, possibly use threading
// fn permutations_machine_run(
//     machines: &[Machine],
//     result: &mut ResultDecider,
//     // info on total package size for % calculation
//     total_to_check: usize,
//     reporter: &mut Reporter,
// ) -> Duration {
//     let start = std::time::Instant::now();
//
//     // let reporting = if STEP_LIMIT < 1000 {
//     //     // 8 million
//     //     0b0011_1111_1111_1111_1111_1111
//     // } else {
//     //     0b0000_1111_1111_1111
//     // };
//
//     let mut machine = MachineCompactDeprecated::from(machines.first().unwrap());
//     machine.step_limit = STEP_LIMIT;
//     machine.tape_size_limit = TAPE_SIZE_LIMIT;
//     for permutation in machines.iter() {
//         machine.change_permutation(permutation);
//         let _ = machine.run();
//         result.add(&machine);
//         // if result.num_total & 0b0011_1111_1111_1111_1111_1111 == 0 {
//         // if result.num_total & reporting == 0 {
//         if reporter.is_due_progress() {
//             let mio = (result.num_checked as f64 / 100_000.0).round() / 10.0;
//             let p = (result.num_checked as f64 / total_to_check as f64 * 1000.0).round() / 10.0;
//             println!("Working: {} = {} million, {p}%", result.num_checked, mio);
//             reporter.reset_last_report_progress_time();
//             if reporter.is_due_detail() {
//                 println!("\nCurrent result\n{}", result);
//                 reporter.reset_last_report_detail_time();
//             }
//         }
//         // if machine.info.status == MachineStatus::DecidedEndless(EndlessReason::OnlyOneDirection) {
//         //     println!("Only One {}", machine.info);
//         //     println!();
//         // }
//     }
//     let duration = start.elapsed();
//
//     // println!(
//     //     "\nBlock Reuse time elapsed for run with {} machines: {:?}",
//     //     variants.len(),
//     //     duration
//     // );
//
//     #[allow(clippy::let_and_return)]
//     duration
// }

// / Read data from BB file and run machine.
// / Count can be a number or 0 for all machines in the file.
// / TODO call more efficient decider
// fn run_file(file_path: &str, first_id: usize, count: usize) {
//     // Read file
//     let mut file = match BBFileReader::new(file_path) {
//         Ok(file) => file,
//         Err(e) => {
//             println!("Error: {:?}", e);
//             return;
//         }
//     };
//
//     println!("\nHeader: {:?}", file.header);
//
//     let mut remaining = if count == 0 {
//         file.header.num_undecided_machines as usize
//     } else {
//         count
//     };
//     let total = remaining;
//     let mut start_id = first_id;
//
//     // Read file data into permutation vector (using batches of batch_size)
//     let start = std::time::Instant::now();
//     let batch_size = 100000;
//     let mut duration_run = Duration::new(0, 0);
//     let mut permutations;
//     let mut result = ResultDecider::new(N_STATES, 0);
//     let mut reporter = Reporter::default();
//
//     loop {
//         let package = batch_size.min(remaining);
//         permutations = match file.read_machine_range_as_permutations(start_id as u64, package) {
//             Ok(p) => p,
//             Err(e) => {
//                 println!("Error: {:?}", e);
//                 return;
//             }
//         };
//         start_id += permutations.len();
//         remaining -= permutations.len();
//
//         let d = permutations_machine_run(&permutations, &mut result, total, &mut reporter);
//         duration_run += d;
//
//         if remaining == 0 || permutations.is_empty() {
//             break;
//         }
//     }
//
//     let duration_with_file_read = start.elapsed();
//
//     // for u in result.machines_undecided.iter().take(10) {
//     //     println!("{}\n", u);
//     // }
//
//     println!("\n{}", result);
//     if let Some(m) = result.machine_max_steps() {
//         println!("Most Steps:\n{}", m);
//     }
//
//     println!(
//         "\nTotal time elapsed for run with {} machines: {:?}, with load from file: {:?}",
//         result.num_checked, duration_run, duration_with_file_read
//     );
// }

// #[cfg(test)]
// mod tests {
//
//     use super::*;
// }
