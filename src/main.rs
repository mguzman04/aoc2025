use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::day1::{Rotation, move_dial};

mod day1;

fn main() {
    let mut total_zeros = 0;
    match read_lines("src/inputs/day1-1.txt") {
        Ok(lines) => {
            let mut dial_position = 50;
            for line in lines.map_while(Result::ok) {
                let rotation = Rotation::new(line);
                dial_position = move_dial(dial_position, rotation.direction, rotation.clicks);
                if dial_position == 0 {
                    total_zeros += 1;
                }
            }
            println!("Total zeros: {}", total_zeros);
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
