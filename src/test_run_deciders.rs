#![allow(unused)]
use std::time::Duration;

use bb_challenge::{
    config::Config,
    decider_result::{DeciderResultStats, DurationDataProvider},
    generator::Generator,
    generator_full::GeneratorFull,
    machine::Machine,
    pre_decider::PreDeciderRun,
    reporter::Reporter,
    status::{MachineStatus, UndecidedReason},
};

pub const PRINT_UNDECIDED_MACHINES: usize = 3;

/// This run goes over the specified data, usually BB4 and just runs the pre-deciders.
/// It is build to check the statistics of the decider and also make a performance evaluation.
/// BB4: 'Pre Deciders New' time elapsed for run with 100,000,000 machines: 1722.79 ms, with generation: 3656.469 ms. (ResultLarge)
/// BB4: 'Pre Deciders New' time elapsed for run with 100,000,000 machines: 1420.561 ms, with generation: 3361.721 ms. (ResultSmall)
/// One third of the time is used for resulct update. See .._undecided.
pub fn run_generator_pre_deciders(config: &Config) -> DeciderResultStats {
    let start = std::time::Instant::now();
    let mut generator = GeneratorFull::new(config);
    let mut result = DeciderResultStats::new_deprecated(config.n_states(), 0);
    result.name = "Pre Deciders New".to_string();
    let mut duration_decider = Duration::new(0, 0);
    let mut reporter = Reporter::default();
    // let mut append = false;
    loop {
        let (permutations, is_last_batch) = generator.generate_permutation_batch_next();
        let d = pre_deciders_batch(&permutations, &mut result); //, generator.limit(), &mut reporter);
        duration_decider += d;

        if reporter.is_due_progress() {
            reporter.report(result.num_checked_total(), generator.limit(), &result);
        }

        // println!("last id {}", permutations.last().unwrap());
        if is_last_batch {
            break;
        }
    }
    result.duration = DurationDataProvider {
        duration_data_provider: Default::default(),
        duration_decider,
        duration_total: start.elapsed(),
    };

    result
}

fn pre_deciders_batch(
    permutations: &[Machine],
    result: &mut DeciderResultStats,
    // info on total package size for % calculation
    // total_to_check: u64,
    // reporter: &mut Reporter,
) -> Duration {
    let start = std::time::Instant::now();

    // let mut machine = MachineCompact::from(permutations.first().unwrap());
    // machine.step_limit = STEP_LIMIT;
    // machine.tape_size_limit = TAPE_SIZE_LIMIT;
    for machine in permutations.iter() {
        // machine.change_permutation(permutation);
        // let _ = machine.run();
        let mut status =
            bb_challenge::pre_decider::run_pre_decider_strict(&machine.transition_table());

        // if machine.id == 322636617 {
        //     println!("{}", machine);
        // }

        if status == MachineStatus::NoDecision {
            status = MachineStatus::Undecided(UndecidedReason::DeciderNoResult, 0, 0);
        }

        result.add(&machine, &status);
        // // only check every 1000 machines, otherwise this takes half the time
        // if machine.id() & 1023 == 0 && reporter.is_due_progress() {
        //     let mio = (result.num_checked as f64 / 100_000.0).round() / 10.0;
        //     let p = (result.num_checked as f64 / total_to_check as f64 * 1000.0).round() / 10.0;
        //     println!("Working: {} = {} million, {p}%", result.num_checked, mio);
        //     reporter.reset_last_report_progress_time();
        //     if reporter.is_due_detail() {
        //         println!("\nCurrent result\n{}", result);
        //         reporter.reset_last_report_detail_time();
        //     }
        // }
        // if machine.status == MachineStatus::DecidedEndless(EndlessReason::OnlyOneDirection) {
        //     println!("Only One {}", machine);
        //     println!();
        // }
    }
    let duration = start.elapsed();

    // println!(
    //     "\nBlock Reuse time elapsed for run with {} machines: {:?}",
    //     variants.len(),
    //     duration
    // );

    #[allow(clippy::let_and_return)]
    duration
}

/// This run goes over the specified data, usually BB4 and just runs the pre-deciders.
/// The only difference is the result detail. This just counts up the undecided counter instead of making a full evaluation of the result.
/// It is build to check the statistics of the decider and also make a performance evaluation.
/// BB4: 'Pre Deciders New Undecided' time elapsed for run with 100,000,000 machines: 1264.743 ms, with generation: 3166.068 ms.
pub fn run_generator_pre_deciders_undecided(config: &Config) -> DeciderResultStats {
    let start = std::time::Instant::now();
    let mut generator = GeneratorFull::new(config);
    let mut result = DeciderResultStats::new_deprecated(config.n_states(), 0);
    result.name = "Pre Deciders Undecided New".to_string();
    let mut duration_decider = Duration::new(0, 0);
    let mut reporter = Reporter::default();
    // let mut append = false;
    loop {
        let (permutations, is_last_batch) = generator.generate_permutation_batch_next();
        let d = pre_deciders_batch_undecided(
            &permutations,
            &mut result,
            generator.limit(),
            &mut reporter,
        );
        duration_decider += d;

        // println!("last id {}", permutations.last().unwrap());
        if is_last_batch {
            break;
        }
    }
    result.duration = DurationDataProvider {
        duration_data_provider: Default::default(),
        duration_decider,
        duration_total: start.elapsed(),
    };

    result
}

fn pre_deciders_batch_undecided(
    permutations: &[Machine],
    result: &mut DeciderResultStats,
    // info on total package size for % calculation
    total_to_check: u64,
    reporter: &mut Reporter,
) -> Duration {
    let start = std::time::Instant::now();

    // let mut machine = MachineCompact::from(permutations.first().unwrap());
    // machine.step_limit = STEP_LIMIT;
    // machine.tape_size_limit = TAPE_SIZE_LIMIT;
    for machine in permutations.iter() {
        // machine.change_permutation(machine);
        // let _ = machine.run();
        let status = bb_challenge::pre_decider::run_pre_decider_strict(&machine.transition_table());

        // if machine.id == 322636617 {
        //     println!("{}", machine);
        // }

        // TODO repair if required
        //         result.num_evaluated += 1;
        //         if status == MachineStatus::NoDecision {
        //             result.num_undecided += 1;
        //             // machine.status = MachineStatus::Undecided(UndecidedReason::DeciderNoResult, 0, 0);
        //         }
        //
        //         // result.add(&machine);
        //         // only check every 1000 machines, otherwise this takes half the time
        //         if machine.id() & 1023 == 0 && reporter.is_due_progress() {
        //             let mio = (result.num_evaluated as f64 / 100_000.0).round() / 10.0;
        //             let p = (result.num_evaluated as f64 / total_to_check as f64 * 1000.0).round() / 10.0;
        //             println!("Working: {} = {} million, {p}%", result.num_evaluated, mio);
        //             reporter.reset_last_report_progress_time();
        //             if reporter.is_due_detail() {
        //                 println!("\nCurrent result\n{}", result);
        //                 reporter.reset_last_report_detail_time();
        //             }
        //         }
        // if machine.status == MachineStatus::DecidedEndless(EndlessReason::OnlyOneDirection) {
        //     println!("Only One {}", machine);
        //     println!();
        // }
    }
    let duration = start.elapsed();

    // println!(
    //     "\nBlock Reuse time elapsed for run with {} machines: {:?}",
    //     variants.len(),
    //     duration
    // );

    #[allow(clippy::let_and_return)]
    duration
}
