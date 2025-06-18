use bb_challenge::{machine::Machine, result::ResultDecider};

pub fn run_machine(machine: &mut Machine, repeat: usize) {
    // println!("\nV2 Machine:\n{}", machine);

    for _ in 0..repeat {
        let start = std::time::Instant::now();
        let status = machine.decide_hold();
        let duration = start.elapsed();
        println!("Info for {}\n", &machine);
        println!("Time elapsed for run with {status} steps: {:?}", duration);
        let mut result = ResultDecider::default();
        result.add(machine, &status);
        println!("Result: {}", result);
    }
}
