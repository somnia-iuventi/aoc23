use itertools::Itertools;
use nom::{
    branch::{self, alt},
    bytes::complete::{is_not, tag, take_till, take_until},
    character::{
        complete::{alpha1, anychar, char, digit1, line_ending, space1},
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

fn parser(input: &str) -> IResult<&str, (&str, Vec<(&str, &str, &str)>)> {
    let instructions = take_till(|x| x != 'L' && x != 'R');
    let nodes = tuple((
        alpha1,
        preceded(tag(" = ("), alpha1),
        delimited(tag(", "), alpha1, tag(")")),
    ));
    terminated(tuple((instructions, many1(nodes))), eof)(input)
}

pub fn part_1(reader: &mut BufReader<File>) -> usize {
    let all_lines = reader.lines().filter_map(|x| x.ok()).collect::<String>();
    println!("loaded file into memory");
    let (_, (instructions, node_strings)) = parser(all_lines.as_str()).unwrap();
    println!("file parsed");
    let mut current = node_strings[0].0;
    let mut iterations = 0;
    let mut answer = 0;
    let nodes = node_strings
        .par_iter()
        .map(|x| (x.0, (x.1, x.2)))
        .collect::<HashMap<&str, (&str, &str)>>();
    println!("nodes created");
    while answer == 0 {
        instructions
            .chars()
            .enumerate()
            .for_each(|(ind, direction)| {
                match direction {
                    'L' => current = nodes.get(current).unwrap().0,
                    'R' => current = nodes.get(current).unwrap().1,
                    _ => {}
                };
                if current == "ZZZ" {
                    answer = ind + 1 + (nodes.len() * iterations);
                    return;
                }
            });
        iterations += 1;
    }
    println!("{answer}");
    answer
}

// pub fn part_2(reader: &mut BufReader<File>) -> usize {
//     let hands = reader
//         .lines()
//         .filter_map(|x| x.ok())
//         .map(|x| parse_single_line(x.as_str()).unwrap().1)
//         .map(|x| parse_cards_part_2(x.0.as_str(), x.1))
//         .sorted()
//         .enumerate()
//         .map(|(ind, x)| x.bid * (ind + 1))
//         .sum();
//     println!("{hands}");
//     hands
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_works() {
        let file = File::open("src/day8/testinput.txt").unwrap();
        let mut reader = BufReader::new(file);
        assert!(part_1(&mut reader) == 6)
    }

    // #[test]
    // fn part_2_works() {
    //     let file = File::open("src/day7/testinput.txt").unwrap();
    //     let mut reader = BufReader::new(file);
    //     assert!(part_2(&mut reader) == 5905)
    // }
}
