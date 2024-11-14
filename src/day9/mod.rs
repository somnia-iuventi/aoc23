use itertools::Itertools;
use nom::{
    branch::{self, alt},
    bytes::complete::{is_not, tag, take_till, take_until},
    character::{
        complete::{alpha1, anychar, char, digit1, line_ending, space1},
        is_alphabetic,
    },
    combinator::{eof, map, not, opt},
    multi::{count, many0, many1, many1_count, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};
use rayon::iter::{
    IntoParallelIterator, IntoParallelRefIterator, ParallelBridge, ParallelIterator,
};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    ops::Index,
};

fn parser(input: &str) -> IResult<&str, Vec<isize>> {
    let signed_digit = map(
        pair(opt(tag("-")), digit1),
        |(neg, num): (Option<&str>, &str)| {
            let p = num.parse::<isize>().unwrap();
            if let Some(_) = neg {
                -p
            } else {
                p
            }
        },
    );
    separated_list1(space1, signed_digit)(input)
}

fn recursive_diff_part_1(input: Vec<isize>) -> isize {
    let res = input
        .iter()
        .tuple_windows::<(&isize, &isize)>()
        .map(|x| x.1 - x.0)
        .collect::<Vec<isize>>();
    input.last().unwrap()
        + match res.iter().all_equal() {
            true => res[0],
            false => recursive_diff_part_1(res),
        }
}

fn recursive_diff_part_2(input: Vec<isize>) -> isize {
    let res = input
        .iter()
        .tuple_windows::<(&isize, &isize)>()
        .map(|x| x.1 - x.0)
        .collect::<Vec<isize>>();
    input.first().unwrap()
        - match res.iter().all_equal() {
            true => res[0],
            false => recursive_diff_part_2(res),
        }
}

pub fn part_1(reader: &mut BufReader<File>) -> isize {
    let answer = reader
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| recursive_diff_part_1(parser(x.as_str()).unwrap().1))
        .sum();
    println!("{answer}");
    answer
}

pub fn part_2(reader: &mut BufReader<File>) -> isize {
    let answer = reader
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| recursive_diff_part_2(parser(x.as_str()).unwrap().1))
        .sum();
    println!("{answer}");
    answer
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_works() {
        let file = File::open("src/day9/testinput.txt").unwrap();
        let mut reader = BufReader::new(file);
        assert!(part_1(&mut reader) == 117)
    }

    #[test]
    fn part_2_works() {
        let file = File::open("src/day9/testinput.txt").unwrap();
        let mut reader = BufReader::new(file);
        assert!(part_2(&mut reader) == -16)
    }
}
