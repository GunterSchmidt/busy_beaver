use bb_challenge::{
    config::{Config, StepTypeSmall},
    decider::Decider,
    decider_result::{BatchData, EndReason},
    decider_result_worker::ResultWorker,
    machine_info::MachineInfo,
};

const MIN_STEPS_HTML: StepTypeSmall = 0;
const MAX_FILES_PER_BATCH: usize = 1000;

/// requires config:
/// * .write_html_file: false, or all files will be written
/// * .limit_machines_decided(100_000) or any other reasonable number
pub fn cycler_html_filter(batch_data: &mut BatchData) -> ResultWorker {
    let mut counter = 0;
    let config_html = Config::builder_from_config(batch_data.config)
        .write_html_file(true)
        .build();
    for (i, status) in batch_data.machines_decided.states.iter().enumerate() {
        if let bb_challenge::status::MachineStatus::DecidedEndless(
            bb_challenge::status::EndlessReason::Cycler(steps, _),
        ) = status
        {
            if *steps > MIN_STEPS_HTML {
                let machine = batch_data.machines_decided.machines[i];
                let mi = MachineInfo::new(machine.id(), *machine.transition_table(), *status);
                println!("Cycler Html: {mi}");
                bb_challenge::decider_cycler::DeciderCycler::decide_single_machine(
                    &machine,
                    &config_html,
                );
                counter += 1;
                if counter == MAX_FILES_PER_BATCH {
                    return Err(EndReason::StopRequested(
                        0,
                        "Found enough examples".to_string(),
                    ));
                }
            }
        }
    }

    Ok(())
}

/// requires config:
/// * .write_html_file: false, or all files will be written
/// * .limit_machines_decided(100_000) or any other reasonable number
pub fn bouncer_html_filter(batch_data: &mut BatchData) -> ResultWorker {
    let mut counter = 0;
    let config_html = Config::builder_from_config(batch_data.config)
        .write_html_file(true)
        .build();
    for (i, status) in batch_data.machines_decided.states.iter().enumerate() {
        if let bb_challenge::status::MachineStatus::DecidedEndless(
            bb_challenge::status::EndlessReason::Bouncer(steps),
        ) = status
        {
            if *steps > MIN_STEPS_HTML {
                let machine = batch_data.machines_decided.machines[i];
                let mi = MachineInfo::new(machine.id(), *machine.transition_table(), *status);
                println!("Bouncer Html: {mi}");
                bb_challenge::decider_bouncer_128::DeciderBouncer128::decide_single_machine(
                    &machine,
                    &config_html,
                );
                counter += 1;
                if counter == MAX_FILES_PER_BATCH {
                    return Err(EndReason::StopRequested(
                        0,
                        "Found enough examples".to_string(),
                    ));
                }
            }
        }
    }

    Ok(())
}
