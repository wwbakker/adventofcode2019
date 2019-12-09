use crate::advent03::model::{Direction, Path, Location};
use crate::advent03::model::Direction::*;
use std::collections::HashSet;
use std::borrow::BorrowMut;

pub fn closest_intersection(path1 : &Path, path2: &Path) -> i32 {
    let locations1 = draw_path(path1);
    let locations2 = draw_path(path2);
    let intersections = locations1.intersection(&locations2);
    let mut distances: Vec<i32> = intersections.map(|l| l.manhattan_distance()).collect();
    distances.sort();
    println!("distances: {:#?}", distances);
    distances[0]
}

pub fn draw_path(path: &Path) -> HashSet<Location> {
    let mut current_location = Location { x: 0, y: 0 };
    let mut traveled_locations: HashSet<Location> = HashSet::new();
    for direction in path.directions.iter() {
        move_direction(current_location.borrow_mut(), direction, traveled_locations.borrow_mut());
    }
    traveled_locations
}


pub fn move_direction(current_location: &mut Location, current_direction: &Direction, traveled_locations: &mut HashSet<Location>) {
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


        assert_eq!(draw_path(&p), expected_result)
    }
}
