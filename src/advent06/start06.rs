use crate::advent06::orbit_reader::read;
use crate::advent06::orbit_calculator::{num_universal_direct_indirect_orbits, path_with_transfer};

pub fn start_a() {
    let space_objects = read("input/advent06/input.txt").unwrap();
    println!("number of direct and indirect orbits {}", num_universal_direct_indirect_orbits(&space_objects));
}

pub fn start_b() {
    let space_objects = read("input/advent06/input.txt").unwrap();
    let p = path_with_transfer(&space_objects, &String::from("YOU"), &String::from("SAN"));
    // path includes the place we are already, so result - 1
    println!("minimum number of transfers required from me to santa {}", p.len() - 1);
}

#[cfg(test)]
mod tests {
    use crate::advent06::orbit_reader::read;
    use crate::advent06::orbit_calculator::{num_universal_direct_indirect_orbits, path, path_with_transfer};
    use std::collections::LinkedList;
    use std::iter::FromIterator;

    #[test]
    fn test_calculation() {
        let space_objects = read("input/advent06/inputtest1.txt").unwrap();
        let num_direct_indirect_orbits = num_universal_direct_indirect_orbits(&space_objects);
        assert_eq!(num_direct_indirect_orbits, 42)
    }

    #[test]
    fn test_path() {
        let space_objects = read("input/advent06/inputtest2.txt").unwrap();
        let p = path(&space_objects, &String::from("SAN"), &String::from("COM"), false);
        let exp = vec!("SAN", "I", "D", "C", "B", "COM");

        assert_eq!(p, LinkedList::from_iter(exp.iter().map( |s| String::from(*s))))
    }

    #[test]
    fn test_path_with_transfer() {
        let space_objects = read("input/advent06/inputtest2.txt").unwrap();
        let p = path_with_transfer(&space_objects, &String::from("YOU"), &String::from("SAN"));
        let exp = vec!("K", "J", "E", "D", "I");
        assert_eq!(p, LinkedList::from_iter(exp.iter().map( |s| String::from(*s))))
    }
}


