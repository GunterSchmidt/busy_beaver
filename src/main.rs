#![allow(dead_code)]
// #![allow(unreachable_code)]
#![allow(unused_imports)]
// #![allow(unused)]

mod arg_handler;
mod test_bb5;
// mod test_machines;
mod test_run_deciders;
mod test_single_deciders;

use std::{
    env::current_dir,
    time::{Duration, Instant},
};

use bb_challenge::{
    config::Config,
    decider::{self, run_decider_generator_single_thread_deprecated, Decider, DeciderEnum},
    decider_hold_u128_long::DeciderHoldU128Long,
    decider_loop_v4::{DeciderLoopV4, STEP_LIMIT_DECIDER_LOOP},
    // decider_u128_long::DeciderU128Long,
    file::BBFileReader,
    generator::Generator,
    generator_full::GeneratorFull,
    generator_reduced::GeneratorReduced,
    machine::Machine,
    machine_info::MachineInfo,
    pre_decider::PreDecider,
    reporter::Reporter,
    result::ResultDecider,
    status::MachineStatus,
    sub_decider::SubDeciderDummy,
    transition_symbol2::TransitionTableSymbol2,
};

use busy_beaver::{
    test_run_deciders::{run_generator_pre_deciders, run_generator_pre_deciders_undecided},
    FILE_PATH, GENERATOR_BATCH_SIZE_REQUEST, GENERATOR_LIMIT, N_STATES, STEP_LIMIT,
    TAPE_SIZE_LIMIT,
};
// use test_machines::run_machine;
// use test_single_deciders::{test_expanding_loop, test_expanding_sinus};

// TODO Decider Long like LoopV4, decide machine and remove machine from self. recycle
// TODO Decider Long after LoopV4 with just 510 steps
// TODO save undecided (with id)
// TODO threaded: Why not just have x gen packs ready and then skip producing them?
// TODO threaded: recycle threads
// TODO threaded: Why so slow under windows?
// TODO multiple deciders
// TODO pre-decider as decider

#[allow(unused)]
fn test_run_decider_deprecated(config: &Config) {
    let start = Instant::now();

    // decider loop V4 for BB4
    let decider = DeciderLoopV4::new(DeciderLoopV4::step_limit(config.n_states()));
    // let decider = PreDecider;
    // let decider = DeciderU128Long::<SubDeciderDummy>::new(&config);
    // let generator = GeneratorFull::new(&config);
    let generator = GeneratorReduced::new(config);
    // let result = decider::run_decider_generator_single_thread(decider, generator);
    // let result = decider::run_decider_data_provider_single_thread(decider, generator);
    // let result = decider::run_decider_generator_threaded(decider, generator);
    let result = decider::run_decider_data_provider_threaded_deprecated(decider, generator);
    println!("{}", result.to_string_extended());
    let duration = start.elapsed();
    // println!("Duration: {:?}", duration);

    // println!(
    //     "\n Machine Max\n{}",
    //     result
    //         .machine_max_steps()
    //         .unwrap()
    //         .transition_table()
    //         .to_table_string(false)
    // );
}

#[allow(unused)]
fn test_run_decider(config: &Config) {
    let start = Instant::now();

    // decider loop V4 for BB4
    let decider = DeciderLoopV4::new(DeciderLoopV4::step_limit(config.n_states()));
    // let decider = PreDecider;
    // let decider = DeciderU128Long::<SubDeciderDummy>::new(&config);
    // let generator = GeneratorFull::new(&config);
    let generator = GeneratorReduced::new(config);
    // let result = decider::run_decider_generator_single_thread(decider, generator);
    // let result = decider::run_decider_data_provider_single_thread(decider, generator);
    // let result = decider::run_decider_generator_threaded(decider, generator);
    let result = decider::run_decider_data_provider_threaded(
        DeciderLoopV4::decider_run_batch,
        generator,
        config,
    );
    println!("{}", result.to_string_extended());
    let duration = start.elapsed();
    // println!("Duration: {:?}", duration);

    // println!(
    //     "\n Machine Max\n{}",
    //     result
    //         .machine_max_steps()
    //         .unwrap()
    //         .transition_table()
    //         .to_table_string(false)
    // );
}

#[allow(unused)]
fn test_run_multiple_decider(config: &Config) {
    let start = Instant::now();

    // let deciders: Vec<DeciderEnum> = vec![
    //     DeciderEnum::PreDecider(PreDecider),
    //     DeciderEnum::LoopV4(DeciderLoopV4::new(STEP_LIMIT_DECIDER_LOOP)),
    //     // DeciderEnum::HoldLong(DeciderHoldU128Long::new(&config)),
    // ];
    // let generator = GeneratorFull::new(&config);
    let generator = GeneratorReduced::new(config);
    // let result = decider::run_decider_chain_data_provider_single_thread(
    //     &vec![
    //         DeciderLoopV4::decider_run_batch,
    //         // DeciderHoldU128Long::decider_run_batch,
    //     ],
    //     generator,
    //     &config,
    // );
    let result = decider::run_decider_chain_data_provider_threaded(
        &vec![
            DeciderLoopV4::decider_run_batch,
            DeciderHoldU128Long::decider_run_batch,
        ],
        generator,
        &config,
    );
    println!("{}", result.to_string_extended());
    // let duration = start.elapsed();
    // println!("Duration: {:?}", duration);

    // println!(
    //     "\n Machine Max\n{}",
    //     result
    //         .machine_max_steps()
    //         .unwrap()
    //         .transition_table()
    //         .to_table_string(false)
    // );
}

/// Main function for tests, running deciders and other stuff.
/// Arguments:
///   - No Arguments: manually defined code
///   - \<number>: Machine ID from bbchallenge provided BB5 file.
///   - \<name>: Machine from defined set in fn 'build_machine'.
///   - rewrite: Rewrites the bb_challenge file into a smaller format (no reader exists)
fn main() {
    // read the arguments of the command line for 30 arguments
    let args: Vec<String> = std::env::args().collect();
    // println!("Arguments len: {}", args.len());
    // println!("Arguments: {:?}", args);)

    // No arguments
    // TODO what is the issue after 409_975_399?
    #[allow(unused)]
    if args.len() < 2 {
        let config = Config::builder(4)
            .generate_limit(0_000_000_000)
            // .generator_batch_size_request_full(5_000_000)
            .generator_batch_size_request_reduced(80_000_000)
            // .limit_machines_undecided(20)
            .cpu_utilization(100)
            .build();

        test_run_decider(&config);
        test_run_decider_deprecated(&config);
        test_run_multiple_decider(&config);

        // let config = Config::builder(3).generate_limit(350_000_000).build();
        // // decider loop V4 for BB4
        // // let decider = DeciderLoopV4::new(STEP_LIMIT_DECIDER_LOOP);
        // let decider = DeciderU128Long::<SubDeciderDummy>::new(&config);
        // // let generator = GeneratorFull::new(&config);
        // let generator = GeneratorReduced::new(&config);
        // let r = decider::run_decider_generator_single_thread(decider, generator);
        // // let r = decider::run_decider_generator_threaded(decider, generator, 100);
        // println!("{}", r.to_string_extended());

        // let decider = DeciderU128Long::<SubDeciderDummy>::new(&config);
        // let generator = GeneratorFull::new(&config);
        // // let r = decider::run_decider_generator_single_thread(decider, generator);
        // let r = decider::run_decider_generator_threaded(decider, generator, 100);
        // println!("{}", r.to_string_extended());

        // let decider = DeciderU128Long::<SubDeciderDummy>::new(&config);
        // let generator = GeneratorReduced::new(&config);
        // // let r = decider::run_decider_generator_single_thread(decider, generator);
        // let r = decider::run_decider_generator_threaded(decider, generator, 100);
        // println!("{}", r.to_string_extended());

        // let generator = GeneratorFull::new(n_states, GENERATOR_BATCH_SIZE_REQUEST, limit);
        // let r = run_decider_generator_threaded(generator, 100);
        // println!("{}", r.to_string_extended());
        // let generator = GeneratorReduced::new(n_states, GENERATOR_BATCH_SIZE_REQUEST, limit);
        // let r = run_decider_generator_threaded(generator, 100);
        // println!("{}", r.to_string_extended());
        // TODO progress change to batches
        // bench_generate_reduced(n_states, limit);
        // println!("{:?}", current_dir());

        // run single machine
        // let machine = TM::build_machine("SA_BB2_MAX").unwrap();
        // let status = DeciderU128Long::<DeciderDummy>::run_decider(&machine);
        // println!("{status}");

        // run_file(FILE_PATH, 1, 10000);
        // show_struct_sizes();
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
        return;

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
            arg_handler::ArgValue::Machine(m) => machine = Some(m),
            arg_handler::ArgValue::TransitionTableCompact(_) => todo!(),
            arg_handler::ArgValue::TransitionTableGeneric(table) => {
                if table.has_two_symbols() {
                    let t = TransitionTableSymbol2::try_from(table);
                    match t {
                        Ok(tc) => machine = Some(Machine::new(0, tc)),
                        Err(e) => println!("{e}"),
                    }
                } else {
                    println!(
                        "This machine has {} symbols and cannot be handled here.",
                        table.dimensions_slow().n_symbols
                    );
                    println!("{}", table);
                }
            }
            arg_handler::ArgValue::None => {}
            arg_handler::ArgValue::Error(e) => {
                println!("Could not parse arguments: {e}");
                return;
            }
        }
        if let Some(machine) = machine {
            let mut d = DeciderLoopV4::new(STEP_LIMIT_DECIDER_LOOP);
            let res = d.decide_machine(&machine);
            println!("Result: {res}");
        }
        return;
    }

    // One argument
    // if args.len() == 2 {
    //     // assume machine number and run machine id from file
    //     match args[1].parse::<u64>() {
    //         Ok(no) => {
    //             println!("Machine number: {}", no);
    //             machine = match BBFileReader::get_machine(FILE_PATH, no) {
    //                 Ok(machine) => machine,
    //                 Err(e) => {
    //                     println!("Error: {:?}", e);
    //                     return;
    //                 }
    //             }
    //         }
    //     }
    // }

    // print_number_of_turing_machines();
    // run_machine(&mut machine);
    // let mut m2 = MachineV2::new_from_machine(&machine);
    // run_machine(&mut machine, 1);
}

fn bench_generate_reduced(n_states: usize, generate_limit: u64) {
    let config = Config::builder(n_states)
        .generate_limit(generate_limit)
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
    println!("Size Total: {}", p_count);
}

struct TestString {
    name: String,
}
struct TestOption {
    // name: Option<String>,
    machine_max_steps: Option<Box<MachineInfo>>,
    pub machines_max_steps: Option<Vec<MachineInfo>>,
    // machine_undecided: Option<MachineInfo>,
    pub machines_undecided: Option<Vec<MachineInfo>>,
}

fn show_struct_sizes() {
    // println!(
    //     "ResultSmall: {}",
    //     std::mem::size_of::<bb_challenge::result::ResultSmall>()
    // );
    println!(
        "Result: {}",
        std::mem::size_of::<bb_challenge::result::ResultDecider>()
    );
    println!("Machine: {}", std::mem::size_of::<Machine>());
    println!("MachineInfo: {}", std::mem::size_of::<MachineInfo>());
    println!("MachineStatus: {}", std::mem::size_of::<MachineStatus>());
    println!(
        "TransitionCompact: {}",
        std::mem::size_of::<bb_challenge::transition_symbol2::TransitionSymbol2>()
    );
    println!("String: {}", std::mem::size_of::<TestString>());
    println!("Option: {}", std::mem::size_of::<TestOption>());
}

// This triggers machine.run() for the given permutations on a single thread.
// This is outdated and highly inefficient.
// TODO implement general machine run decider or use existing one
// Solve issue with progress, evtl. use progress bar
// Solve issue with batch size, evtl. use threading
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

/// Read data from BB file and run machine.
/// Count can be a number or 0 for all machines in the file.
/// TODO call more efficient decider
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

#[cfg(test)]
mod tests {

    use bb_challenge::{config::Config, sub_decider::SubDeciderDummy};

    use super::*;

    //     #[test]
    //     // https://bbchallenge.org/story#will-it-halt-or-not, Machine 1
    //     fn test_machine_steps_4() {
    //         let mut transitions: Vec<(&str, &str)> = Vec::new();
    //         transitions.push(("1RB", "1RB"));
    //         transitions.push(("1LA", "---"));
    //
    //         let machine = Machine::from_string_tuple(0, &transitions);
    //         println!("{}", machine);
    //         let result = machine.decide_hold();
    //         let steps = match result {
    //             MachineStatus::DecidedHolds(s) => s,
    //             _ => 0,
    //         };
    //         assert_eq!(steps, 4);
    //     }

    // #[test]
    // // https://bbchallenge.org/story#will-it-halt-or-not, Machine 2
    // fn test_machine_steps_bb5_105() {
    //     let mut transitions: Vec<(&str, &str)> = Vec::new();
    //     transitions.push(("1RB", "1LC"));
    //     transitions.push(("0LB", "1LA"));
    //     transitions.push(("1RD", "1LB"));
    //     transitions.push(("1RE", "0RD"));
    //     transitions.push(("0RA", "---"));
    //     let machine = Machine::from_string_tuple(0, &transitions);
    //     let result = machine.decide_hold();
    //     let steps = match result {
    //         MachineStatus::DecidedHolds(s) => s,
    //         _ => 0,
    //     };
    //     assert_eq!(steps, 105);
    // }

    //     #[test]
    //     // https://bbchallenge.org/story#will-it-halt-or-not, Machine 2
    //     fn test_machine_steps_bb5_max() {
    //         let config = Config::new_default(5);
    //         let machine = Machine::build_machine("BB5_MAX").unwrap();
    //         let mut d: DeciderU128Long<SubDeciderDummy> =
    //             bb_challenge::decider_u128_long::DeciderU128Long::new(&config);
    //         let result = d.decide_machine(&machine);
    //
    //         // let mut machine = build_machine("BB5_MAX").unwrap();
    //         // let result = machine.run();
    //         // println!("{}", machine);
    //         assert_eq!(result, MachineStatus::DecidedHolds(47_176_870));
    //         // assert_eq!(result, MachineStatus::DecidedHoldsOld(47176870, 4097));
    //         // let steps = match result {
    //         //     MachineStatus::DecidedHolds(s, _) => s,
    //         //     _ => 0,
    //         // };
    //
    //         // assert_eq!(steps, 47176870);
    //         // println!("{}", machine.status);
    //         // let ok = match machine.status {
    //         //     MachineStatus::DecidedHolds(47176870, 4097) => true,
    //         //     _ => false,
    //         // };
    //         // assert!(ok);
    //     }
}
