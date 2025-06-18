use bb_challenge::{
    decider_bouncer::DeciderBouncer, decider_expanding_loop::DeciderExpandingLoop, machine::Machine,
};

#[allow(clippy::vec_init_then_push)]
pub fn test_expanding_loop() {
    let mut decider = DeciderExpandingLoop::new();

    #[cfg(all(debug_assertions, feature = "debug"))]
    println!("test");

    // let mut transitions: Vec<(&str, &str)> = Vec::new();
    // transitions.push(("0LB", "1LC"));
    // transitions.push(("0LC", "---"));
    // transitions.push(("1LD", "1RC"));
    // transitions.push(("1RA", "0RA"));
    // let mut machine = MachineCompact::new(transitions);
    // machine.id = 38250788;
    // machine.status = decider.decider_expanding_loop(&machine);

    // BB5 Max, should not apply
    let mut transitions: Vec<(&str, &str)> = Vec::new();
    transitions.push(("1RB", "1LC"));
    transitions.push(("1RC", "1RB"));
    transitions.push(("1RD", "0LE"));
    transitions.push(("1LA", "1LD"));
    transitions.push(("---", "0LA"));
    let machine = Machine::from_string_tuple(64379691, &transitions);
    let status = decider.decider_expanding_loop(&machine);

    println!("Expanding Loop: {}\n{}", machine, status);
}

#[allow(clippy::vec_init_then_push)]
pub fn test_expanding_sinus() {
    let mut decider = DeciderBouncer::new();

    #[allow(clippy::vec_init_then_push)]
    let mut transitions: Vec<(&str, &str)> = Vec::new();
    transitions.push(("0LB", "1LC"));
    transitions.push(("0LC", "---"));
    transitions.push(("1LD", "1RC"));
    transitions.push(("1RA", "0RA"));
    let machine = Machine::from_string_tuple(38250788, &transitions);
    let status = decider.decider_expanding_sinus(&machine);

    // BB5 Max, should not apply
    // let mut transitions: Vec<(&str, &str)> = Vec::new();
    // transitions.push(("1RB", "1LC"));
    // transitions.push(("1RC", "1RB"));
    // transitions.push(("1RD", "0LE"));
    // transitions.push(("1LA", "1LD"));
    // transitions.push(("---", "0LA"));
    // let mut machine = MachineCompact::new(transitions);
    // machine.id = 64379691;
    // let check_result = decider.decider_expanding_sinus(&machine);

    println!("Expanding Sinus: {}\n{}", machine, status);
    // println!("Result: {}", check_result);
}
