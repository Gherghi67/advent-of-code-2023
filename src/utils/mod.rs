use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}