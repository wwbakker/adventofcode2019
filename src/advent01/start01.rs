use super::fuel_counter_upper;
use super::file_fold;

pub fn start_a() {
    let result = file_fold::fold("input/advent01/input.txt",0,&fuel_counter_upper::sum_fuel_required);
    match result {
        Ok(r) => println!("{}", r),
        Err(e) => println!("{}", e.to_string())
    }
}

pub fn start_b() {
    let result = file_fold::fold("input/advent01/input.txt",0,&fuel_counter_upper::sum_fuel_required_including_fuel_for_fuel);
    match result {
        Ok(r) => println!("{}", r),
        Err(e) => println!("{}", e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fully() {
        let r = file_fold::fold("input/advent01/inputtest2.txt",0,&fuel_counter_upper::sum_fuel_required);
        match r {
            Ok(r) => assert_eq!(r, 2 + 2 + 654 + 33583),
            Err(_) => assert!(false)
        };
    }
}