mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use std::{fs::File, io::BufReader};

fn main() {
    let file = File::open("src/day8/input.txt").unwrap();
    let mut reader = BufReader::new(file);
    day8::part_1(&mut reader);
}
