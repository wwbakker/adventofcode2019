fn fuel_required(mass : i32) -> i32 {
    // divide by 3 and round down (automatically rounded down by keeping it as an integer
    let step1 = mass / 3;
    step1 - 2
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
}
