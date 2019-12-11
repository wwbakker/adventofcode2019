use std::fs::read_to_string;
use std::io;


pub fn read(filepath: &str) -> io::Result<Vec<i32>> {
    read_to_string(filepath).map(|a| a.split(",").map( parse_as_int ).collect())
}

fn parse_as_int(s : &str) -> i32 {
    s.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filefold() {
        match read("input/advent02/inputtest1.txt") {
            Ok(r) => assert_eq!(r, [1, 5, 20]),
            Err(_) => assert!(false)
        }
    }
}

