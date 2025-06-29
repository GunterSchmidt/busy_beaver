#![allow(unused)]
use bb_challenge::{
    decider::{self, Decider},
    decider_cycler_v4::DeciderCyclerV4,
    generator_reduced::GeneratorReduced,
    pre_decider::PreDeciderRun,
};

pub struct DeciderBB5;

impl Decider for DeciderBB5 {
    fn decide_machine(
        &mut self,
        machine: &bb_challenge::machine::Machine,
    ) -> bb_challenge::status::MachineStatus {
        todo!();
        //         // decider loop V4 for BB4
        //         let decider = DeciderLoopV4::new(STEP_LIMIT_DECIDER_LOOP);
        //         // let decider = DeciderU128Long::<SubDeciderDummy>::new(&config);
        //         // let generator = GeneratorFull::new(&config);
        //         let generator = GeneratorReduced::new(&config);
        //         // let result = decider::run_decider_generator_single_thread(decider, generator);
        //         let result = decider::run_decider_generator_threaded(decider, generator, 100);
        //
        //         result
    }

    fn name(&self) -> &str {
        "Decider BB5"
    }

    fn decider_run_batch(
        machines: &[bb_challenge::machine::Machine],
        run_predecider: PreDeciderRun,
        config: &bb_challenge::config::Config,
    ) -> Option<bb_challenge::decider_result::BatchResult> {
        todo!()
    }

    fn decide_single_machine(
        machine: &bb_challenge::machine::Machine,
        config: &bb_challenge::config::Config,
    ) -> bb_challenge::status::MachineStatus {
        todo!()
    }

    // fn new_from_self(&self) -> Self {
    //     todo!()
    // }
}
