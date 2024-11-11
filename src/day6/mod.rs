use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_till, take_until},
    character::{
        complete::{alpha1, anychar, digit1, line_ending, space1},
        is_alphabetic,
    },
    combinator::{eof, map, not},
    multi::{count, many0, many1, many1_count, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
use rayon::iter::{
    IntoParallelIterator, IntoParallelRefIterator, ParallelBridge, ParallelIterator,
};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    ops::Index,
};

fn parse_single_line_part_1(input: &str) -> IResult<&str, Vec<usize>> {
    let titles = tuple((take_until(":"), tag(":"), space1));
    let numbers = separated_list1(space1, map(digit1, |x: &str| x.parse::<usize>().unwrap()));
    delimited(titles, numbers, eof)(input)
}

fn parse_single_line_part_2(input: &str) -> IResult<&str, usize> {
    let titles = tuple((take_until(":"), tag(":"), space1));
    let numbers = map(separated_list1(space1, digit1), |x| {
        x.join("").parse::<usize>().unwrap()
    });
    delimited(titles, numbers, eof)(input)
}

fn calculate_number_of_winning_races(total_race_time: usize, distance_to_beat: usize) -> usize {
    (0..=total_race_time)
        .into_iter()
        .map(|x| x * (total_race_time - x))
        .filter(|x| *x > distance_to_beat)
        .count()
}

pub fn part_1(reader: &mut BufReader<File>) -> usize {
    let numbers = reader
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| parse_single_line_part_1(x.as_str()).unwrap().1)
        .collect::<Vec<Vec<usize>>>();
    let answer = numbers[0]
        .iter()
        .zip(numbers[1].iter())
        .map(|(time, distance)| calculate_number_of_winning_races(*time, *distance))
        .product::<usize>();
    println!("{answer}");
    answer
}

pub fn part_2(reader: &mut BufReader<File>) -> usize {
    let numbers = reader
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| parse_single_line_part_2(x.as_str()).unwrap().1)
        .collect::<Vec<usize>>();
    let answer = calculate_number_of_winning_races(numbers[0], numbers[1]);
    println!("{answer}");
    answer
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_works() {
        let file = File::open("src/day6/testinput.txt").unwrap();
        let mut reader = BufReader::new(file);
        assert!(part_1(&mut reader) == 288)
    }

    #[test]
    fn part_2_works() {
        let file = File::open("src/day6/testinput.txt").unwrap();
        let mut reader = BufReader::new(file);
        assert!(part_2(&mut reader) == 71503)
    }
}
