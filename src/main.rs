 use std::{
     error,
     fs::{read_to_string, write},
     path::Path,
     result
 };

type TResult<T> = result::Result<T, TError>;
type TError = Box<dyn error::Error>;

fn read_file(p: &str) -> TResult<String> {
    read_to_string(p).map_err(|e| e.into())
}

fn split_numbers(s: &str) -> TResult<Vec<usize>> {
    s.split_whitespace()
        .map(|x| x.parse::<usize>().map_err(|e| e.into()))
        .collect()
}

fn main() {
    println!("This is a test!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file() {
        let res = read_file("test_data/test_one.txt");
        assert!(res.is_ok());

        if let Ok(s) = res {
            assert_eq!(s, "3\n5\n");
        }
    }

    #[test]
    fn test_split_numbers() {
        let res = split_numbers(&String::from("5\n8"));

        assert!(res.is_ok());

        if let Ok(v) = res {
            assert_eq!(v, vec![5, 8]);
        }
    }
}
