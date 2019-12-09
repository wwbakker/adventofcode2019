use crate::advent03::path_calculator;
use crate::advent03::read_paths;

pub fn start_a() {
    let paths = read_paths::read("input/advent03/input.txt").unwrap();
    println!("closest intersecting manhattan distance: {}",
             path_calculator::closest_intersection_manhattan(&paths[0], &paths[1]))
}

pub fn start_b() {
    let paths = read_paths::read("input/advent03/input.txt").unwrap();
    println!("closest intersection minimizing signal delay: {}",
             path_calculator::closest_intersection_minimizing_signal_delay(&paths[0], &paths[1]))
}

#[cfg(test)]
mod tests {
    use crate::advent03::{read_paths, path_calculator};

    #[test]
    fn test_example_1() {
        let paths_result = read_paths::read("input/advent03/inputtest2.txt").unwrap();
        let result = path_calculator::closest_intersection_manhattan(&paths_result[0], &paths_result[1]);
        assert_eq!(result, 159)
    }

    #[test]
    fn test_example_2() {
        let paths_result = read_paths::read("input/advent03/inputtest3.txt").unwrap();
        let result = path_calculator::closest_intersection_manhattan(&paths_result[0], &paths_result[1]);
        assert_eq!(result, 135)
    }

    #[test]
    fn test_example_3() {
        let paths_result = read_paths::read("input/advent03/inputtest4.txt").unwrap();
        let result = path_calculator::closest_intersection_minimizing_signal_delay(&paths_result[0], &paths_result[1]);
        assert_eq!(result, 610)
    }

    #[test]
    fn test_example_4() {
        let paths_result = read_paths::read("input/advent03/inputtest5.txt").unwrap();
        let result = path_calculator::closest_intersection_minimizing_signal_delay(&paths_result[0], &paths_result[1]);
        assert_eq!(result, 410)
    }
}
