use super::code_reading;
use crate::advent07::amplifier::{highest_signal, execute_amplifiers_looped};

pub fn start_a() {
    match code_reading::read("input/advent07/input.txt") {
        Ok(v) => {
            let result = highest_signal(&v);
            println!("Successfully calculated highest signal: {}", result);
        },
        Err(e) => eprintln!("Error occurred: {}", e.to_string().as_str())
    }
}

pub fn start_b() {
    let code =
        vec!(3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26,
             27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5);

    assert_eq!(execute_amplifiers_looped(&code, vec!(9, 8, 7, 6, 5)), 139629729);
}