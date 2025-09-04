use bb_challenge::{
    config::Config,
    data_provider::DataProviderBatch,
    decider::decider_result::{DeciderResultStats, PreDeciderCount},
    machine_binary::MachineBinary,
    machine_generic::TransitionGeneric,
    machine_info::MachineInfo,
    status::MachineStatus,
    transition_binary::TransitionBinary,
};

struct MachineId {
    id: u64,
    /// Field 0 is used for more information to keep the size of this struct small.
    transition_table: MachineBinary,
    // has_self_referencing_transition: bool,
}

struct MachineIdOption {
    id: Option<u128>,
    /// Field 0 is used for more information to keep the size of this struct small.
    transition_table: MachineBinary,
    // has_self_referencing_transition: bool,
}

pub fn show_struct_sizes() {
    println!("Config: {}", std::mem::size_of::<Config>());
    println!("Result: {}", std::mem::size_of::<DeciderResultStats>());
    println!(
        "DataProviderResult: {}",
        std::mem::size_of::<DataProviderBatch>()
    );
    println!(
        "PreDeciderCount: {}",
        std::mem::size_of::<PreDeciderCount>()
    );

    println!("MachineBinary: {}", std::mem::size_of::<MachineBinary>());
    // println!("Machine 2: {}", std::mem::size_of::<Machine2>());
    println!("Machine Id: {}", std::mem::size_of::<MachineId>());
    println!(
        "MachineIdOption: {}",
        std::mem::size_of::<MachineIdOption>()
    );
    println!("MachineInfo: {}", std::mem::size_of::<MachineInfo>());
    println!("MachineStatus: {}", std::mem::size_of::<MachineStatus>());
    println!(
        "TransitionGeneric: {}",
        std::mem::size_of::<TransitionGeneric>()
    );
    println!(
        "TransitionBinary: {}",
        std::mem::size_of::<TransitionBinary>()
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
