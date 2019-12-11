use crate::advent03::model::{Direction, Path, Location};
use crate::advent03::model::Direction::*;
use std::collections::{HashSet, HashMap};
use std::borrow::BorrowMut;

pub fn closest_intersection_manhattan(path1 : &Path, path2: &Path) -> i32 {
    let (locations1, _) = draw_path(path1);
    let (locations2, _) = draw_path(path2);
    let intersections = locations1.intersection(&locations2);
    let mut distances: Vec<i32> = intersections.map(|l| l.manhattan_distance()).collect();
    distances.sort();
    distances[0]
}

pub fn closest_intersection_minimizing_signal_delay(path1 : &Path, path2: &Path) -> i32 {
    let (locations1, number_of_steps_per_location1) = draw_path(path1);
    let (locations2, number_of_steps_per_location2) = draw_path(path2);
    let intersections = locations1.intersection(&locations2);
    let mut signal_delay_steps: Vec<i32> = intersections
        .map(|l| calculate_signal_delay_steps(
            &l,
            &number_of_steps_per_location1,
            &number_of_steps_per_location2)).collect();
    signal_delay_steps.sort();
    signal_delay_steps[0]
}
fn calculate_signal_delay_steps(location : &Location,
                                number_of_steps_per_location1 : &HashMap<u64, i32>,
                                number_of_steps_per_location2 : &HashMap<u64, i32>) -> i32 {
    let hash = location.calculate_hash();
    number_of_steps_per_location1.get(&hash).unwrap() + number_of_steps_per_location2.get(&hash).unwrap()
}

pub fn draw_path(path: &Path) -> (HashSet<Location>, HashMap<u64, i32>) {
    let mut current_location = Location { x: 0, y: 0 };
    let mut traveled_locations: HashSet<Location> = HashSet::new();
    let mut number_of_steps_per_location : HashMap<u64, i32> = HashMap::new();
    let mut number_of_steps : i32 = 0;
    for direction in path.directions.iter() {
        move_direction(current_location.borrow_mut(),
                       direction,
                       traveled_locations.borrow_mut(),
                       number_of_steps.borrow_mut(),
                       number_of_steps_per_location.borrow_mut());
    }
    (traveled_locations, number_of_steps_per_location)
}


pub fn move_direction(current_location: &mut Location,
                      current_direction: &Direction,
                      traveled_locations: &mut HashSet<Location>,
                      number_of_steps: &mut i32,
                      number_of_steps_per_location : &mut HashMap<u64, i32>) {
    let (distance, step) = match current_direction {
        Up(distance) => (distance, 1),
        Down(distance) => (distance, -1),
        Left(distance) => (distance, -1),
        Right(distance) => (distance, 1)
    };

    for _ in 0..*distance {
        match current_direction {
            Up(_) | Down(_) => current_location.y += step,
            Left(_) | Right(_) => current_location.x += step
        }
        *number_of_steps += 1;
        number_of_steps_per_location.insert(current_location.calculate_hash(), *number_of_steps);
        traveled_locations.insert(current_location.clone());
    }
}




#[cfg(test)]
mod tests {
    use crate::advent03::path_calculator::draw_path;
    use crate::advent03::model::*;
    use crate::advent03::model::Direction::*;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn test_read_directions() {
        let p = Path {
            directions: vec!(Up(3),Left(1),Down(2))
        };
        let expected_locations = [
            Location {x:0,y:1},
            Location {x:0,y:2},
            Location {x:0,y:3},
            Location {x:-1,y:3},
            Location {x:-1,y:2},
            Location {x:-1,y:1}
        ];
        let expected_result : HashSet<Location> =
            HashSet::from_iter(expected_locations.iter().map(|a| *a));

        let (locations, _) = draw_path(&p);
        assert_eq!(locations, expected_result)
    }
}
