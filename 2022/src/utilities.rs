use std::fs;

use num_traits::cast::cast;
use num_traits::NumCast;

pub fn read_file(fname: &str) -> String {
    fs::read_to_string("inputs/".to_owned() + fname + ".txt").expect("Invalid file path")
}

pub fn as_maybe_num<T>(c: char) -> Option<T>
where
    T: NumCast,
{
    c.to_digit(10).and_then(cast)
}

pub fn as_num<T>(c: char) -> T
where
    T: NumCast,
{
    as_maybe_num(c).unwrap()
}
