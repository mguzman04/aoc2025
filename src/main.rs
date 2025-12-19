use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::day1::count_zero_landings;
use crate::day2::invalid_id_sum;

mod day1;
mod day2;

fn main() {
    // day1_solution();
    day2_solution();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn day1_solution() {
    match read_lines("src/inputs/day1-1.txt") {
        Ok(lines) => {
            let (total_zeros, total_crossings) = count_zero_landings(lines);
            println!("Total zeros: {}", total_zeros);
            println!("Total crossings: {}", total_crossings);
        }
        Err(err) => println!("You Suck: {}", err),
    };
}

fn day2_solution() {
    match read_lines("src/inputs/day2-1.txt") {
        Ok(lines) => {
            let sum = invalid_id_sum(lines.last().unwrap().unwrap());
            println!("Invalid IDs Sum: {}", sum);
        }
        Err(err) => println!("You Suck: {}", err),
    };
}
