use super::read_ints;
use super::intcode;

pub fn start_a() {
    match read_ints::read("input/advent02/input_before_fire.txt") {
        Ok(mut v) => {
            let code = v.as_mut();
            intcode::exec(code);
            println!("Successfully executed intcode");
            println!("{:?}", code)

        },
        Err(e) => eprintln!("Error occurred: {}", e.to_string().as_str())
    }
}

pub fn start_b() {
    match read_ints::read("input/advent02/input_before_fire.txt") {
        Ok(v) => {
            for noun in 1..100 {
                for verb in 1..100 {
                    let result = intcode::exec_with_noun_and_verb(v.as_ref(), noun, verb);
                    if result == 19690720 {
                        println!("noun: {} verb: {} result: {}", noun, verb, result);
                    }
                }
            }
        },
        Err(e) => eprintln!("Error occurred: {}", e.to_string().as_str())
    }
}