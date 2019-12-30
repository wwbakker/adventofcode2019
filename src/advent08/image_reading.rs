use std::fs::read_to_string;
use std::io;


pub fn read(filepath: &str) -> io::Result<Vec<i32>> {
    read_to_string(filepath).map(|a| a.bytes().map( parse_as_int ).collect())
}

fn parse_as_int(s : u8) -> i32 { String::from_utf8(vec!(s)).unwrap().parse().unwrap() }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filefold() {
        match read("input/advent08/input_test.txt") {
            Ok(r) => assert_eq!(r, [1,2,3,4,5,6,7,8]),
            Err(_) => assert!(false)
        }
    }
}

