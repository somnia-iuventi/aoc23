mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use std::{fs::File, io::BufReader};

fn main() {
    let file = File::open("src/day9/input.txt").unwrap();
    let mut reader = BufReader::new(file);
    day9::part_2(&mut reader);
}
