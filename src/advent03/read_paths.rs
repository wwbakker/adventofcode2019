use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use crate::advent03::model::{Path, Direction};


pub fn read(filepath: &str) -> io::Result<Vec<Path>> {
    let f = File::open(filepath)?;
    let reader = BufReader::new(f);
    reader.lines().map(|s| s.map(read_path)).collect()
}

fn read_path(s : String) -> Path {
    Path { directions: s.split(",").map(read_direction).collect() }
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


#[cfg(test)]
mod tests {
    use super::*;
    use super::Direction::*;

    #[test]
    fn test_read_directions() {
        match read("input/advent03/inputtest1.txt") {
            Ok(r) => {
                assert_eq!(r[0].directions, vec!(Up(7),Right(6),Down(4),Left(4)));
                assert_eq!(r[1].directions, vec!(Up(5)))
            },
            Err(_) => assert!(false)
        }
    }
}

