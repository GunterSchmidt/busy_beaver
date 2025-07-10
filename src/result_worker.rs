use bb_challenge::{
    config::{Config, StepTypeSmall},
    decider::Decider,
    decider_cycler_v4,
    decider_result::{BatchData, EndReason},
    decider_result_worker::ResultWorker,
    machine_info::MachineInfo,
};

const MIN_STEPS_HTML: StepTypeSmall = 50;
const MAX_FILES_PER_BATCH: usize = 1_000;

pub fn cycler_html_filter(batch_data: &mut BatchData) -> ResultWorker {
    if !batch_data.config.write_html_file() {
        return Ok(());
    }
    let mut counter = 0;
    let config_html = Config::builder_from_config(batch_data.config)
        .write_html_file(true)
        .build();
    for (i, status) in batch_data.machines_decided.states.iter().enumerate() {
        match status {
            bb_challenge::status::MachineStatus::DecidedEndless(endless_reason) => {
                match endless_reason {
                    bb_challenge::status::EndlessReason::Cycle(steps, _) => {
                        if *steps > MIN_STEPS_HTML {
                            let machine = batch_data.machines_decided.machines[i];
                            let mi = MachineInfo::new(
                                machine.id(),
                                *machine.transition_table(),
                                *status,
                            );
                            println!("Cycler Html: {mi}");
                            decider_cycler_v4::DeciderCyclerV4::decide_single_machine(
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
                    _ => {}
                }
            }
            _ => {}
        }
    }

    Ok(())
}
