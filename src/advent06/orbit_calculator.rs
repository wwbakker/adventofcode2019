use std::collections::{HashMap, LinkedList};
use crate::advent06::model::SpaceObject;
use std::iter::FromIterator;


pub fn num_universal_direct_indirect_orbits(h : &HashMap<String, SpaceObject>) -> i32 {
    let mut sum = 0;
    for space_object in h.values() {
        sum += num_direct_indirect_orbits_of_space_object(h, &space_object.name);
    }
    sum
}

fn num_direct_indirect_orbits_of_space_object(h : &HashMap<String, SpaceObject>, name : &String) -> i32 {
    if name == "COM" {
        0
    } else {
        let name_of_direct_orbit = &h.get(name).unwrap().in_orbit_around;
        1 + num_direct_indirect_orbits_of_space_object(h, name_of_direct_orbit)
    }
}

pub fn path_with_transfer(space_objects : &HashMap<String, SpaceObject>, from : &String, to : &String) -> LinkedList<String> {
    let com = String::from("COM");
    let path_from_to_com = path(space_objects, from, &com, true);
    let path_to_to_com = path(space_objects, to, &com, true);
    let crossroads = crossroads_space_object_name(&path_from_to_com, &path_to_to_com);
    let path_from_to_crossroads = path(space_objects, from, &crossroads, true);
    let path_to_to_crossroads = path(space_objects, to, &crossroads, true);
    LinkedList::from_iter(path_from_to_crossroads.iter().chain(path_to_to_crossroads.iter().rev().skip(1)).cloned())
}

pub fn path(h : &HashMap<String, SpaceObject>, from : &String, to : &String, skip_from : bool) -> LinkedList<String> {
    let mut list : LinkedList<String>;
    if from == to {
        list = LinkedList::new();
    } else {
        let name_of_direct_orbit = &h.get(from).unwrap().in_orbit_around;
        list = path(h, name_of_direct_orbit, to, false);
    }
    if !skip_from {
        list.push_front(from.clone());
    }
    list
}

fn crossroads_space_object_name(path1 : &LinkedList<String>, path2: &LinkedList<String>) -> String {
    for space_object_name in path1 {
        if path2.contains(&space_object_name) {
            return space_object_name.clone();
        }
    }
    panic!("no overlapping paths")
}
