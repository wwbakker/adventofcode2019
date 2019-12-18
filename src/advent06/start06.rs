use crate::advent06::orbit_reader::read;
use crate::advent06::orbit_calculator::universal_direct_indirect_orbits;

pub fn start_a() {
    let space_objects = read("input/advent06/input.txt").unwrap();
    println!("number of direct and indirect orbits {}", universal_direct_indirect_orbits(&space_objects));
}

#[cfg(test)]
mod tests {
    use crate::advent06::orbit_reader::read;
    use crate::advent06::orbit_calculator::universal_direct_indirect_orbits;

    #[test]
    fn test_calculation() {
        let space_objects = read("input/advent06/inputtest1.txt").unwrap();
        let num_direct_indirect_orbits = universal_direct_indirect_orbits(&space_objects);
        assert_eq!(num_direct_indirect_orbits, 42)
    }
}


