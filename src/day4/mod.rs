use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part_1(reader: &mut BufReader<File>) -> usize {
    let total: usize = reader
        .lines()
        .par_bridge()
        .filter_map(|x| x.ok())
        .map(|x| calculate_row_part_1(x.as_str()))
        .sum();
    println!("{total}");
    total
}

pub fn part_2(size: Option<usize>, reader: &mut BufReader<File>) -> usize {
    let mut number_of_wins: HashMap<usize, usize> = HashMap::new();
    let this_size;
    if let Some(s) = size {
        this_size = s;
    } else {
        this_size = 186;
    }
    for each in 0..this_size {
        number_of_wins.insert(each, 1);
    }
    reader
        .lines()
        .enumerate()
        .filter(|x| x.1.is_ok())
        .for_each(|x| {
            let line = x.1.unwrap();
            let index = x.0;
            println!("On index: {index}");
            let iterations = number_of_wins.get(&index).unwrap();
            let wins = calculate_row_part_2(line.as_str());
            for _ in 0..*iterations {
                for adding_index in index + 1..=index + wins {
                    let num = number_of_wins.get_mut(&adding_index).unwrap();
                    *num += 1;
                }
            }
        });
    let answer = number_of_wins.values().sum::<usize>();
    println!("{answer}");
    answer
}

fn calculate_row_part_1(row: &str) -> usize {
    let mut chunks: HashSet<usize> = HashSet::new();
    let numbers = row.split(" ").skip_while(|x| !x.contains(":")).skip(1);
    for (key, value) in numbers.chunk_by(|x| *x != "|").into_iter() {
        if key == true {
            if chunks.is_empty() {
                chunks.extend(
                    value
                        .into_iter()
                        .filter(|x| *x != "")
                        .map(|x| x.parse::<usize>().expect("str to be numbers")),
                );
            } else {
                chunks = chunks
                    .intersection(
                        &value
                            .into_iter()
                            .filter(|x| *x != "")
                            .map(|x| x.parse::<usize>().unwrap())
                            .collect::<HashSet<usize>>(),
                    )
                    .map(|x| *x)
                    .collect();
            }
        }
    }
    let winning_numbers = chunks.iter().count();
    match winning_numbers {
        0 => 0,
        1 => 1,
        _ => {
            let mut result = 1;
            for _ in 1..winning_numbers {
                result *= 2;
            }
            result
        }
    }
}

fn calculate_row_part_2(row: &str) -> usize {
    let mut chunks: HashSet<usize> = HashSet::new();
    let numbers = row.split(" ").skip_while(|x| !x.contains(":")).skip(1);
    for (key, value) in numbers.chunk_by(|x| *x != "|").into_iter() {
        if key == true {
            if chunks.is_empty() {
                chunks.extend(
                    value
                        .into_iter()
                        .filter(|x| *x != "")
                        .map(|x| x.parse::<usize>().expect("str to be numbers")),
                );
            } else {
                chunks = chunks
                    .intersection(
                        &value
                            .into_iter()
                            .filter(|x| *x != "")
                            .map(|x| x.parse::<usize>().unwrap())
                            .collect::<HashSet<usize>>(),
                    )
                    .map(|x| *x)
                    .collect();
            }
        }
    }
    chunks.iter().count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_works() {
        let file = File::open("src/day4/testinput.txt").unwrap();
        let mut reader = BufReader::new(file);
        assert!(part_1(&mut reader) == 13);
    }

    #[test]
    fn part_2_works() {
        let file = File::open("src/day4/testinput.txt").unwrap();
        let mut reader = BufReader::new(file);
        assert!(part_2(Some(6), &mut reader) == 30);
    }
}
