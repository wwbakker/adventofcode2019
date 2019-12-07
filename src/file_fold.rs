use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn file_fold<A>(filepath : &str, initial : A, op : &dyn Fn(A, &str) -> A) -> io::Result<A> {
    let mut current: A = initial;
    let mut f = File::open(filepath)?;
    let mut reader = BufReader::new(f);
    for line in reader.lines() {
        current = op(current, line?.as_str());
    }
    Ok(current)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn concat(a : &str, b : &str) -> &str {
        a + b
    }

    #[test]
    fn test_filefold() {
        let result = file_fold("inputtest.txt","", &concat);
        assert!(result.is_ok());
        match result {
            Ok(r) => assert_eq!(r, "abcdef"),
            Err(e) => assert!(false)
        };
    }
}
