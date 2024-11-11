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
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    ops::Index,
};

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(PartialEq, Ord, Eq, Debug)]
struct Hand {
    card_strength: Vec<usize>,
    hand_type: HandType,
    bid: usize,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let Some(cmp) = self.hand_type.partial_cmp(&other.hand_type) else {
            return None;
        };
        if cmp != Ordering::Equal {
            return Some(cmp);
        } else {
            return self.card_strength.partial_cmp(&other.card_strength);
        }
    }
}

fn parse_single_line(input: &str) -> IResult<&str, (String, usize)> {
    let cards = map(take_until(" "), |x: &str| x.to_owned());
    let bid = preceded(space1, map(digit1, |x: &str| x.parse::<usize>().unwrap()));
    terminated(tuple((cards, bid)), eof)(input)
}

fn parse_cards_part_1(input: &str, bid: usize) -> Hand {
    let card_strength = input
        .chars()
        .map(|x| match x {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => x.to_string().parse::<usize>().unwrap(),
        })
        .collect::<Vec<usize>>();
    let dupes = input
        .chars()
        .sorted()
        .dedup_with_count()
        .map(|x| x.0)
        .collect::<Vec<usize>>();
    let hand_type = match *dupes.iter().max().unwrap() {
        5 => HandType::FiveKind,
        4 => HandType::FourKind,
        3 => {
            if dupes.contains(&2) {
                HandType::FullHouse
            } else {
                HandType::ThreeKind
            }
        }
        2 => {
            if dupes.iter().filter(|x| **x == 2).count() == 2 {
                HandType::TwoPair
            } else {
                HandType::Pair
            }
        }
        _ => HandType::HighCard,
    };
    Hand {
        card_strength,
        hand_type,
        bid,
    }
}

fn parse_cards_part_2(input: &str, bid: usize) -> Hand {
    let card_strength = input
        .chars()
        .map(|x| match x {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            _ => x.to_string().parse::<usize>().unwrap(),
        })
        .collect::<Vec<usize>>();
    let mut dupes = input
        .chars()
        .sorted()
        .dedup_with_count()
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .collect::<Vec<(usize, char)>>();
    if dupes.len() == 1 {
        return Hand {
            card_strength,
            hand_type: HandType::FiveKind,
            bid,
        };
    }
    if let Some(num) = dupes.iter().map(|x| x.1).position(|x| x == 'J') {
        let number_of_jokers = dupes.remove(num).0;
        let highest_number_of_same_cards = dupes.last_mut().unwrap();
        highest_number_of_same_cards.0 += number_of_jokers;
    }
    let just_nums = dupes.iter().map(|x| x.0).collect::<Vec<usize>>();
    let hand_type = match *just_nums.iter().max().unwrap() {
        5 => HandType::FiveKind,
        4 => HandType::FourKind,
        3 => {
            if just_nums.contains(&2) {
                HandType::FullHouse
            } else {
                HandType::ThreeKind
            }
        }
        2 => {
            if just_nums.iter().filter(|x| **x == 2).count() == 2 {
                HandType::TwoPair
            } else {
                HandType::Pair
            }
        }
        _ => HandType::HighCard,
    };
    Hand {
        card_strength,
        hand_type,
        bid,
    }
}
pub fn part_1(reader: &mut BufReader<File>) -> usize {
    let hands = reader
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| parse_single_line(x.as_str()).unwrap().1)
        .map(|x| parse_cards_part_1(x.0.as_str(), x.1))
        .sorted()
        .enumerate()
        .map(|(ind, x)| x.bid * (ind + 1))
        .sum();
    println!("{hands}");
    hands
}

pub fn part_2(reader: &mut BufReader<File>) -> usize {
    let hands = reader
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| parse_single_line(x.as_str()).unwrap().1)
        .map(|x| parse_cards_part_2(x.0.as_str(), x.1))
        .sorted()
        .enumerate()
        .map(|(ind, x)| x.bid * (ind + 1))
        .sum();
    println!("{hands}");
    hands
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_works() {
        let file = File::open("src/day7/testinput.txt").unwrap();
        let mut reader = BufReader::new(file);
        assert!(part_1(&mut reader) == 6440)
    }

    #[test]
    fn part_2_works() {
        let file = File::open("src/day7/testinput.txt").unwrap();
        let mut reader = BufReader::new(file);
        assert!(part_2(&mut reader) == 5905)
    }
}
