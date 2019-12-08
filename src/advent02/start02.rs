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