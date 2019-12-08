use std::fs::read_to_string;
use std::io;


pub fn read(filepath: &str) -> io::Result<Vec<Direction>> {
    read_to_string(filepath).map(|a| a.split(",").map(read_direction).collect())
}

fn read_direction(s : &str) -> Direction {
    let path_length : i32 = s[1..].parse().unwrap();
    let direction_character = s.chars().nth(0).unwrap();
    match direction_character {
        'L' => Direction::Left(path_length),
        'R' => Direction::Right(path_length),
        'U' => Direction::Up(path_length),
        'D' => Direction::Down(path_length),
        _ => panic!("Unknown direction: {}", direction_character)
    }
}

#[derive(PartialEq,Debug,Clone)]
pub enum Direction {
    Left(i32),
    Right(i32),
    Up(i32),
    Down(i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Direction::*;

    #[test]
    fn test_read_directions() {
        match read("input/advent03/inputtest1.txt") {
            Ok(r) => assert_eq!(r, vec!(Up(7),Right(6),Down(4),Left(4))),
            Err(_) => assert!(false)
        }
    }
}

