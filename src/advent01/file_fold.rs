use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub fn fold<A>(filepath: &str, initial: A, op: &dyn Fn(A, &str) -> A) -> io::Result<A> {
    let mut current: A = initial;
    let f = File::open(filepath)?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        current = op(current, line?.as_str());
    }
    Ok(current)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn concat(a: String, b: &str) -> String {
        let mut s = String::new();
        s.push_str(a.as_str());
        s.push_str(b);
        s
    }

    #[test]
    fn test_filefold() {
        let result = fold("input/advent01/inputtest1.txt", String::new(), &concat);
        assert!(result.is_ok());
        match result {
            Ok(r) => assert_eq!(r, "abcdef"),
            Err(_) => assert!(false)
        };
    }
}

