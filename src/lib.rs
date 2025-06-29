// This lib only exists to allow criterion tests.

use bb_challenge::config::StepTypeBig;

pub mod test_run_deciders;

// use std::time::Duration;

// #[cfg(debug_assertions)]
// use bb_challenge::status::EndlessReason;

// const FILE_PATH: &str = "res/seed_test_file";
/// Limit generation, 0 for number of touring machines
pub const GENERATOR_LIMIT: u64 = 50_000_000;
pub const GENERATOR_BATCH_SIZE_REQUEST: usize = 10_000_000; // GENERATOR_BATCH_SIZE_RECOMMENDATION;
pub const N_STATES: usize = 4;
pub const STEP_LIMIT: StepTypeBig = 110; // 50_000_000;
pub const TAPE_SIZE_LIMIT: usize = 20000;
pub const USE_CPU_PERCENT: usize = 100;

// // TODO remove, Expanding Sinus
// pub fn run_decider_bouncer_one(n_states: usize) {
//     let start = std::time::Instant::now();
//     let config = Config::new_default(n_states);
//     let mut generator = GeneratorFull::new(&config);
//     let mut result = ResultDecider::new_deprecated(n_states, 0);
//     let mut duration_run = Duration::new(0, 0);
//     let mut reporter = Reporter::default();
//     // let mut append = false;
//     loop {
//         let (permutations, is_last_batch) = generator.generate_permutation_batch_next();
//         // if variants.len() < generator.batch_size {
//         // Permutation::write_to_disk(&variants, false, append).expect("File error");
//         // append = true;
//         let d = decider_bouncer_batch(&permutations, &mut result, generator.limit(), &mut reporter);
//         duration_run += d;
//
//         // println!("last id {}", permutations.last().unwrap());
//         if is_last_batch {
//             break;
//         }
//     }
//     let duration_with_generation = start.elapsed();
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
//         "\nCompact time elapsed for run with {} machines: {:?}, with generation: {:?}",
//         result.num_checked_total(),
//         duration_run,
//         duration_with_generation
//     );
// }

// #[allow(unused)]
// fn decider_bouncer_batch(
//     variants: &[Machine],
//     result: &mut ResultDecider,
//     // info on total package size for % calculation
//     total_to_check: u64,
//     reporter: &mut Reporter,
// ) -> Duration {
//     let start = std::time::Instant::now();
//     if variants.is_empty() {
//         return start.elapsed();
//     }
//
//     // let mut machine = MachineCompact::from(variants.first().unwrap());
//     // machine.step_limit = STEP_LIMIT;
//     // machine.tape_size_limit = TAPE_SIZE_LIMIT;
//     let mut decider = DeciderBouncer::new();
//     for machine in variants.iter() {
//         // machine.change_permutation(variant);
//         // let _ = machine.run();
//         let mut status = bb_challenge::pre_decider::run_pre_decider(&machine.transition_table());
//         if status == MachineStatus::NoDecision {
//             // machine.status = decider_holds(&machine);
//             // match machine.status {
//             //     MachineStatus::DecidedHolds(_, _) => {}
//             //     _ => machine.status = dl4.decider_loop_v4_compact(&machine),
//             // }
//             status = decider.decider_expanding_sinus(&machine);
//         }
//
//         // if machine.id == 322636617 {
//         //     println!("{}", machine);
//         // }
//
//         #[cfg(debug_assertions)]
//         #[allow(clippy::collapsible_match)]
//         match status {
//             MachineStatus::NoDecision => {}
//             // MachineStatus::Running => todo!(),
//             MachineStatus::DecidedEndless(endless_reason) => match endless_reason {
//                 EndlessReason::Loop(_, _) => {
//                     println!("Loop check for {}", &machine);
//                     println!("Result: {}", &status);
//                 }
//                 // deciders::EndlessReason::StartRecursive => {}
//                 _ => {
//                     // println!("Loop check for {}", &machine);
//                     // println!("Result: {}", &machine.status);
//                 } // _ => {}
//             },
//             // MachineStatus::DecidedHolds(_, _)| //  => {}
//             // MachineStatus::Undecided(_, _) => {
//             //     // todo!("ff");
//             //     println!("Loop check for {}", &machine);
//             //     println!("Result: {}", &machine.status);
//             // }
//             _ => {
//                 // println!("Loop check for {}", &machine);
//                 // println!("Result: {}", &machine.status);
//             }
//         }
//
//         result.add(&machine, &status);
//         // only check every 1000 machines, otherwise this takes half the time
//         // TODO repair if required
//         // if machine.id() & 1023 == 0 && reporter.is_due_progress() {
//         //     let mio = (result.num_evaluated as f64 / 100_000.0).round() / 10.0;
//         //     let p = (result.num_evaluated as f64 / total_to_check as f64 * 1000.0).round() / 10.0;
//         //     println!("Working: {} = {} million, {p}%", result.num_evaluated, mio);
//         //     reporter.reset_last_report_progress_time();
//         //     if reporter.is_due_detail() {
//         //         println!("\nCurrent result\n{}", result);
//         //         reporter.reset_last_report_detail_time();
//         //     }
//         // }
//         // if machine.status == MachineStatus::DecidedEndless(EndlessReason::OnlyOneDirection) {
//         //     println!("Only One {}", machine);
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
