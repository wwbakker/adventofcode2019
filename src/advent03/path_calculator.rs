use crate::advent03::read_directions::Direction;
use crate::advent03::read_directions::Direction::*;
use std::collections::HashSet;
use std::borrow::BorrowMut;

pub fn draw_path(directions : Vec<Direction>) -> HashSet<Location> {
    let mut current_location = Location {x:0, y:0};
    let mut traveled_locations: HashSet<Location> = HashSet::new();
    for direction in directions.iter() {
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

    for i in 0..*distance {
        match current_direction {
            Up(_) | Down(_) => current_location.y += step,
            Left(_) | Right(_) => current_location.x += step
        }
        traveled_locations.insert(current_location.clone());
    }
}

//// klopt niet, want eerste stap blijf je op dezelfde plek:
//fn calculate_locations(current_location : Location, direction : Direction) -> Vec<Location> {
//    match direction {
//        Up(distance) =>
//            (current_location.y..(current_location.y + distance))
//                .map(|cy| Location { x:current_location.x, y:cy })
//                .collect(),
//        Down(distance) =>
//            (current_location.y..(current_location.y - distance))
//                .map(|cy| Location { x:current_location.x, y:cy })
//                .collect(),
//        Left(distance) =>
//            (current_location.x..(current_location.x - distance))
//                .map(|cx| Location { x:cx, y:current_location.y })
//                .collect(),
//        Right(distance) =>
//            (current_location.x..(current_location.x + distance))
//                .map(|cx| Location { x:cx, y:current_location.y })
//                .collect(),
//    }
//}



#[derive(Hash,PartialEq,Eq,Clone)]
pub struct Location {
    x : i32,
    y : i32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_directions() {
        assert!(false)
    }
}
