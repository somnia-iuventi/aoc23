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

struct AllMappers(Vec<Mapper>);
impl AllMappers {
    fn new(numbers: Vec<Vec<Vec<usize>>>) -> Self {
        let mappers = numbers
            .into_iter()
            .map(|x| Mapper::new(x))
            .collect::<Vec<Mapper>>();
        Self(mappers)
    }

    fn calc(&self, input: usize) -> usize {
        let mut answer = input;
        for (ind, each) in self.0.iter().enumerate() {
            answer = each.calc(answer);
        }
        answer
    }
}

struct Mapper(Vec<MapRange>);

impl Mapper {
    fn new(numbers: Vec<Vec<usize>>) -> Self {
        let ranges = numbers
            .into_par_iter()
            .map(|x| MapRange::new(x))
            .collect::<Vec<MapRange>>();
        Self(ranges)
    }
    fn calc(&self, input: usize) -> usize {
        let answers = self
            .0
            .iter()
            .map(|x| x.calc(input))
            .filter(|x| *x != input)
            .collect::<Vec<usize>>();
        if answers.len() > 0 {
            answers[0]
        } else {
            input
        }
    }
}

struct MapRange {
    source_range_start: usize,
    dest_range_start: usize,
    range_size: usize,
}

impl MapRange {
    fn new(input: Vec<usize>) -> Self {
        Self {
            source_range_start: input[1],
            dest_range_start: input[0],
            range_size: input[2],
        }
    }
    fn calc(&self, input: usize) -> usize {
        let Some(dest_index) = input.checked_sub(self.source_range_start) else {
            return input;
        };
        if input > self.source_range_start + self.range_size - 1 {
            input
        } else {
            self.dest_range_start + dest_index
        }
    }
}

fn seeds(input: &str) -> IResult<&str, Vec<usize>> {
    let seed_title = tuple((alpha1, tag(":"), space1));
    let numbers = separated_list1(tag(" "), map(digit1, |x: &str| x.parse::<usize>().unwrap()));
    delimited(seed_title, numbers, eof)(input)
}

fn all_mappers(input: &str) -> IResult<&str, AllMappers> {
    let numbers_parse = terminated(
        separated_list1(tag(" "), map(digit1, |x: &str| x.parse::<usize>().unwrap())),
        tag("\n"),
    );
    let title_parse = tuple((take_until("\n"), tag("\n")));
    let mapper_parse = preceded(title_parse, many1(numbers_parse));
    let mut all_mappers = map(many1(mapper_parse), |x| AllMappers::new(x));

    all_mappers(input)
    // mapper_parse(input)
}

pub fn part_1(reader: &mut BufReader<File>) -> usize {
    let seed_line = reader
        .lines()
        .take(1)
        .filter_map(|x| x.ok())
        .collect::<String>();
    let seeds = seeds(seed_line.as_str()).unwrap().1;
    println!("Got seeds: {seeds:?}");
    let mappers: String = reader
        .lines()
        .filter_map(|x| x.ok())
        .filter(|x| x != "")
        .intersperse("\n".to_owned())
        .collect();
    println!("Constructed remaining string");
    let omni = all_mappers(mappers.as_str()).unwrap().1;
    println!("Constructed omni-mapper");
    let answer = seeds.par_iter().map(|x| omni.calc(*x)).min().unwrap();
    println!("{answer}");
    answer
}

pub fn part_2(reader: &mut BufReader<File>) -> usize {
    let seed_line = reader
        .lines()
        .take(1)
        .filter_map(|x| x.ok())
        .collect::<String>();
    let mut seeds_list = seeds(seed_line.as_str()).unwrap().1.into_iter();
    let mut seeds = Vec::new();
    while let Some(num) = seeds_list.next() {
        if let Some(length) = seeds_list.next() {
            for each in num..num + length {
                seeds.push(each);
            }
        }
    }
    println!("got seeds");
    let mappers: String = reader
        .lines()
        .filter_map(|x| x.ok())
        .filter(|x| x != "")
        .intersperse("\n".to_owned())
        .collect();
    println!("Constructed remaining string");
    let omni = all_mappers(mappers.as_str()).unwrap().1;
    println!("Constructed omni-mapper");
    let answer = seeds.par_iter().map(|x| omni.calc(*x)).min().unwrap();
    println!("{answer}");
    answer
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_works() {
        let file = File::open("src/day5/testinput.txt").unwrap();
        let mut reader = BufReader::new(file);
        assert!(part_1(&mut reader) == 35)
    }

    #[test]
    fn part_2_works() {
        let file = File::open("src/day5/testinput.txt").unwrap();
        let mut reader = BufReader::new(file);
        assert!(part_2(&mut reader) == 46)
    }
}
