pub fn sum_fuel_required_including_fuel_for_fuel(tally: i32, mass_as_str: &str) -> i32 {
    let mass : i32 = mass_as_str.parse().unwrap();
    tally + fuel_required_including_fuel_for_fuel(mass)
}


pub fn sum_fuel_required(tally: i32, mass_as_str: &str) -> i32 {
    let mass : i32 = mass_as_str.parse().unwrap();
    tally + fuel_required(mass)
}

fn fuel_required(mass: i32) -> i32 {
    // divide by 3 and round down (automatically rounded down by keeping it as an integer
    mass / 3 - 2
}

pub fn fuel_required_including_fuel_for_fuel(mass : i32) -> i32 {
    let fuel_for_mass = fuel_required(mass);
    if fuel_for_mass <= 0 {
        0
    } else {
        fuel_for_mass + fuel_required_including_fuel_for_fuel(fuel_for_mass)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_required() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 654);
        assert_eq!(fuel_required(100756), 33583);
    }

    #[test]
    fn test_fuel_required_including_fuel_for_fuel() {
        assert_eq!(fuel_required_including_fuel_for_fuel(14), 2);
        assert_eq!(fuel_required_including_fuel_for_fuel(1969), 966);
        assert_eq!(fuel_required_including_fuel_for_fuel(100756), 50346);
    }
}
