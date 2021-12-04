use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub fn read_lines_as_ints<P>(filename: P) -> io::Result<Vec<i32>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().filter_map(|int| int.ok().unwrap().parse::<i32>().ok()).collect())
}

pub fn read_lines_as_strings<P>(filename: P) -> io::Result<Vec<String>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().filter_map(|line| line.ok()).collect())
}