use std::collections::HashMap;
use crate::advent06::model::SpaceObject;


pub fn universal_direct_indirect_orbits(h : &HashMap<String, SpaceObject>) -> i32 {
    let mut sum = 0;
    for space_object in h.values() {
        sum += direct_indirect_orbits_of_space_object(h, &space_object.name);
    }
    sum
}

fn direct_indirect_orbits_of_space_object(h : &HashMap<String, SpaceObject>, name : &String) -> i32 {
    if name == "COM" {
        0
    } else {
        let name_of_direct_orbit = &h.get(name).unwrap().in_orbit_around;
        1 + direct_indirect_orbits_of_space_object(h, name_of_direct_orbit)
    }
}