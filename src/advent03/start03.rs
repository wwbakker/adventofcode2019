use crate::advent03::path_calculator;
use crate::advent03::read_paths;

pub fn start_a() {
    let paths_result = read_paths::read("input/advent03/input.txt");
    match paths_result {
        Ok(paths) => {
            println!("closest intersecting distance: {}",
                     path_calculator::closest_intersection(&paths[0], &paths[1]))
        }
        Err(_) => eprintln!("Error while reading from input file.")
    }
}

#[cfg(test)]
mod tests {
    use crate::advent03::{read_paths, path_calculator};

    #[test]
    fn test_example_1() {
        let paths_result = read_paths::read("input/advent03/inputtest2.txt").unwrap();
        let result = path_calculator::closest_intersection(&paths_result[0], &paths_result[1]);
        assert_eq!(result, 159)
    }

    #[test]
    fn test_example_2() {
        let paths_result = read_paths::read("input/advent03/inputtest3.txt").unwrap();
        let result = path_calculator::closest_intersection(&paths_result[0], &paths_result[1]);
        assert_eq!(result, 135)
    }
}
