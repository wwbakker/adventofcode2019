use super::read_ints;
use super::intcode;

pub fn start_a() {
    match read_ints::read("input/advent05/input.txt") {
        Ok(mut v) => {
            let code = v.as_mut();
            intcode::exec(code);
            println!("Successfully executed intcode");
            println!("{:?}", code)

        },
        Err(e) => eprintln!("Error occurred: {}", e.to_string().as_str())
    }
}

