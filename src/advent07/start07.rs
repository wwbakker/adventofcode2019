use super::code_reading;
use crate::advent07::amplifier::{highest_signal, highest_signal_looped};

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
    match code_reading::read("input/advent07/input.txt") {
        Ok(v) => {
            let result = highest_signal_looped(&v);
            println!("Successfully calculated highest signal: {}", result);
        },
        Err(e) => eprintln!("Error occurred: {}", e.to_string().as_str())
    }
}