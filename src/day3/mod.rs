use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    str::Chars,
    time::Instant,
    u128,
};

pub fn part_1(reader: &mut BufReader<File>) -> u128 {
    let now = Instant::now();
    let (numbers, locations) = reader
        .lines()
        .enumerate()
        .par_bridge()
        .filter(|x| x.1.is_ok())
        .map(|(row, line)| calculate_row_part_1(line.unwrap().as_str(), row))
        .reduce(
            || (HashSet::new(), HashSet::new()),
            |mut acc, e| {
                acc.0.extend(e.0.into_iter());
                acc.1.extend(e.1.into_iter());
                acc
            },
        );
    let total: u128 = numbers
        .par_iter()
        .filter(|x| x.locations.iter().any(|x| locations.contains(x)))
        .map(|x| x.value)
        .sum();
    let elapsed = now.elapsed();
    dbg!(elapsed);
    total
}

pub fn part_2(reader: &mut BufReader<File>) -> u128 {
    let now = Instant::now();
    let (numbers, gears) = reader
        .lines()
        .enumerate()
        .par_bridge()
        .filter(|x| x.1.is_ok())
        .map(|(row, line)| calculate_row_part_2(line.unwrap().as_str(), row))
        .reduce(
            || (HashSet::new(), HashSet::new()),
            |mut acc, e| {
                acc.0.extend(e.0.into_iter());
                acc.1.extend(e.1.into_iter());
                acc
            },
        );
    // let total: u128 = gears
    let total: u128 = gears
        .iter()
        .map(|x| {
            numbers
                .iter()
                .filter(|z| z.locations.iter().any(|y| x.0.contains(y)))
                .map(|x| x.value)
                .collect::<Vec<u128>>()
        })
        .filter(|x| x.len() == 2)
        .map(|x| x.iter().product::<u128>())
        .sum();
    let elapsed = now.elapsed();
    dbg!(elapsed);
    total
}

fn calculate_row_part_1(row: &str, row_index: usize) -> (HashSet<Number>, HashSet<Position>) {
    let mut currently_number = false;
    let mut numbers = HashSet::new();
    let mut active_locations = HashSet::new();
    let mut number_characters = String::new();
    let mut peek_iter = row.char_indices().peekable();
    while let Some((col, each)) = peek_iter.next() {
        if !each.is_digit(10) {
            if currently_number {
                numbers.insert(calculate_number(row_index, col, &mut number_characters));
                currently_number = false;
            }
            if each == '.' {
                continue;
            }
            for column in col - 1..=col + 1 {
                for row in row_index - 1..=row_index + 1 {
                    active_locations.insert(Position { x: row, y: column });
                }
            }
        } else {
            currently_number = true;
            number_characters.push(each);
            if peek_iter.peek().is_none() {
                numbers.insert(calculate_number(row_index, col, &mut number_characters));
            }
        }
    }
    (numbers, active_locations)
}

fn calculate_row_part_2(row: &str, row_index: usize) -> (HashSet<Number>, HashSet<Gear>) {
    let mut currently_number = false;
    let mut numbers = HashSet::new();
    let mut gears = HashSet::new();
    let mut number_characters = String::new();
    let mut peek_iter = row.char_indices().peekable();
    while let Some((col, each)) = peek_iter.next() {
        if !each.is_digit(10) {
            if currently_number {
                numbers.insert(calculate_number(row_index, col, &mut number_characters));
                currently_number = false;
            }
            if each == '.' {
                continue;
            }
            if each == '*' {
                let mut new_locs = Vec::new();
                for column in col - 1..=col + 1 {
                    for row in row_index - 1..=row_index + 1 {
                        new_locs.push(Position { x: row, y: column });
                    }
                }
                gears.insert(Gear(new_locs));
            }
        } else {
            currently_number = true;
            number_characters.push(each);
            if peek_iter.peek().is_none() {
                numbers.insert(calculate_number(row_index, col, &mut number_characters));
            }
        }
    }
    (numbers, gears)
}

fn calculate_number(row: usize, col: usize, characters: &mut String) -> Number {
    let mut positions = Vec::new();
    for each in col - characters.len()..col {
        positions.push(Position { x: row, y: each })
    }
    let composed: u128 = characters.parse().unwrap();
    characters.clear();
    Number {
        value: composed,
        locations: positions,
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Number {
    value: u128,
    locations: Vec<Position>,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Gear(Vec<Position>);

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Position {
    x: usize,
    y: usize,
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_works() {
        let file = File::open("src/day3/testinput.txt").unwrap();
        let mut reader = BufReader::new(file);

        assert!(part_1(&mut reader) == 4361);
    }

    #[test]
    fn part_2_works() {
        let file = File::open("src/day3/testinput.txt").unwrap();
        let mut reader = BufReader::new(file);

        assert!(part_2(&mut reader) == 467835);
    }
}
