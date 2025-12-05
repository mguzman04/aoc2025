use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::day1::count_zero_landings;

mod day1;

fn main() {
    match read_lines("src/inputs/day1-1sample.txt") {
        Ok(lines) => {
            let (total_zeros, total_crossings) = count_zero_landings(lines);
            println!("Total zeros: {}", total_zeros);
            println!("Total crossings: {}", total_crossings);
        }
        Err(err) => println!("You Suck: {}", err),
    };
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
