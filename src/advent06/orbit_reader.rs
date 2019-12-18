use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::*;

use crate::advent06::model::SpaceObject;

pub fn read(filepath: &str) -> io::Result<HashMap<String, SpaceObject>> {
    let f = File::open(filepath)?;
    let reader = BufReader::new(f);
    let space_objects : io::Result<Vec<SpaceObject>> =
        reader.lines()
            .map(|s| s.map(read_space_object))
            .collect();

     space_objects.map(|v| convert_vec_to_hash_map(v))
}

fn read_space_object(line : String) -> SpaceObject {
    let split_line : Vec<_> = line.split(')').collect();
    SpaceObject {
        name: String::from(&split_line[1][..]),
        in_orbit_around: String::from(&split_line[0][..])
    }
}

fn convert_vec_to_hash_map(v : Vec<SpaceObject>) -> HashMap<String, SpaceObject> {
    let mut result = HashMap::with_capacity(v.len());
    for space_object in v {
        result.insert(space_object.name.clone(), space_object);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let r = read("input/advent06/inputtest1.txt").unwrap();
        let r1 = SpaceObject { name: String::from("B"), in_orbit_around: String::from("COM") };
        let r2 = SpaceObject { name: String::from("L"), in_orbit_around: String::from("K") };
        assert_eq!(r.get("B"), Some( &r1 ));
        assert_eq!(r.get("L"), Some( &r2 ))
    }
}

