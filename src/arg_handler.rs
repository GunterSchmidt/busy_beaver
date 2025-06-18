use bb_challenge::{
    file::BBFileReader,
    machine::Machine,
    transition_generic::{TransitionGeneric, TransitionTableGeneric},
    transition_symbol2::TransitionTableSymbol2,
};
use busy_beaver::FILE_PATH;

pub enum ArgValue {
    Machine(Machine),
    TransitionTableCompact(TransitionTableSymbol2),
    TransitionTableGeneric(TransitionTableGeneric),
    None,
    Error(String),
}

// TODO Clap crate
pub fn standard_args(args: &[String]) -> ArgValue {
    // if args.len() > 1 {
    //     match args[1] {
    //         "n" => {}
    //         "rewrite" => {
    //             rewrite_file(FILE_PATH).unwrap();
    //         }
    //     }
    // }
    // return;

    let mut unknown_args = false;
    match args.len() {
        2 => match args[1].as_str() {
            "--rewrite" => {
                BBFileReader::rewrite_file(FILE_PATH).unwrap();
            }
            _ => unknown_args = true,
        },
        3 => match args[1].as_str() {
            "-n" | "--name" => {
                // if let Ok(no) = args[1].parse::<u64>() {}
                let machine = Machine::build_machine(args[2].as_str());
                match machine {
                    Some(m) => return ArgValue::Machine(m),
                    None => {
                        return ArgValue::Error(format!(
                            "No machine with name '{}' found.",
                            args[2]
                        ));
                    }
                }
            }

            "-h" | "--help" => {
                println!("{}", help_string());
                return ArgValue::None;
            }

            "-m" | "--machine" => {
                let tg = TransitionTableGeneric::from_standard_tm_text_format(&args[2]);
                // let tg = TransitionTableGeneric::try_from(&args[2].as_str());
                match tg {
                    Ok(table) => {
                        return ArgValue::TransitionTableGeneric(table);
                        // let r = TransitionTableCompact::try_from(t);
                        // match r {
                        //     Ok(table) => {
                        //         // println!("{table}");
                        //         return ArgValue::TransitionTableCompact(table);
                        //     }
                        //     Err(e) => println!("{e}"),
                        // }
                    }
                    Err(e) => return ArgValue::Error(e.to_string()),
                }
            }

            // TODO path as argument
            "-fn" | "--file-number" => {
                if let Ok(no) = args[2].parse::<u64>() {
                    println!("Machine number: {}", no);
                    match BBFileReader::read_machine_single(no, FILE_PATH) {
                        Ok(machine) => return ArgValue::Machine(machine),
                        Err(e) => return ArgValue::Error(format!("{:?}", e)),
                    };
                } else {
                    return ArgValue::Error(format!("Invalid machine number: {}", args[2]));
                }
            }
            _ => unknown_args = true,
        },
        // 3 => {
        //     match args[1] {}
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
        //         Err(_) => {
        //             // not a number, get machine with that name
        //             match build_machine(args[1].as_str()) {
        //                 Some(m) => {
        //                     machine = m;
        //                 }
        //                 None => {
        //                     println!("Invalid argument");
        //                     return;
        //                 }
        //             }
        //         }
        //     }
        //     println!("1 argument: {}", args[1]);
        //     return;
        // }
        _ => unknown_args = true,
    }
    if unknown_args {
        println!("Invalid arguments: {:?}\n", &args[1..]);
        println!("{}", help_string());
    }

    return ArgValue::None;

    // if let Some(mut machine) = machine {
    //     machine.run();
    // }
}

pub fn help_string() -> String {
    let mut s = String::new();
    s.push_str("This program accepts the following arguments:\n");
    s.push_str("-fn, --file-number <number>: Read machine no (e.g. 42) from file and run it.\n");
    s.push_str("-h, --help:                  This help text\n");
    s.push_str("-m, --machine <transitions>: Run machine, e.g. '1RB1LC_1RC1RB_1RD0LE_1LA1LD_1RZ0LA' or '1RB2LB1RZ_2LA2RB1LB'\n");
    s.push_str("-n, --name <name>:           Build predefined machine");
    s.push_str("-r, --rewrite:               Experimental rewrite in smaller file format.\n");
    s
}

#[cfg(test)]
mod tests {
    use bb_challenge::{transition_generic::B, transition_symbol2::TransitionSymbol2};

    use super::*;

    #[test]
    fn test_machine_2x2_6_4() {
        // 2x2-6-4
        let text = "1RB1LB_1LA1RZ";
        let args = vec!["path".to_string(), "-m".to_string(), text.to_string()];
        let r = standard_args(&args);
        let table = match r {
            ArgValue::TransitionTableGeneric(t) => t,
            _ => todo!(),
        };
        let check_value = TransitionGeneric::try_from("1RZ").unwrap();
        let transition_b1 = table.transition_for_state_symbol(B, 1);
        println!("{}", table);
        println!("{}", table.to_standard_tm_text_format());
        assert_eq!(check_value, transition_b1);
        let tm_format = table.to_standard_tm_text_format();
        assert_eq!(text, tm_format);
    }
}
