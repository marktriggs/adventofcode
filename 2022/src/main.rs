// (cd ~/projects/adventofcode/2022 && cargo run)

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(dead_code)]

extern crate regex;
extern crate rand;
extern crate anyhow;
extern crate itertools;

mod shared {
    pub use regex::Regex;

    // pub use intcode::{self, IntCode};
    pub use std::cell::RefCell;
    pub use std::cell::RefMut;
    pub use std::cmp::{self, Ordering, Reverse};
    pub use std::collections::BTreeMap;
    pub use std::collections::BTreeSet;
    pub use std::collections::BinaryHeap;
    pub use std::collections::HashMap;
    pub use std::collections::HashSet;
    pub use std::collections::LinkedList;
    pub use std::collections::VecDeque;
    pub use std::convert::TryInto;
    pub use std::fmt::{self, Display};
    pub use std::fs::{self, File};
    pub use std::io::{self, BufRead, BufReader, Read, Write};
    pub use std::iter::FromIterator;
    pub use std::ops::RangeInclusive;
    pub use std::rc::Rc;
    pub use std::str::{self, FromStr};
    pub use std::sync::{Arc, Mutex};

    pub use rand::Rng;

    pub use anyhow::{anyhow, bail, Error};

    pub const ALPHABET: &str = "abcdefghijlkmnopqrstuvwxyz";
    pub const ALPHABET_UPPER: &str = "ABCDEFGHIJLKMNOPQRSTUVWXYZ";

    fn regex_examples() {
        let simple_match = Regex::new(r"s.mple match").unwrap();
        if simple_match.is_match("simple match") {
            println!("Matched!");
        }

        let extract_numbers = Regex::new(r"dimensions (\d+)x(\d+) left (\d+) top (\d+)").unwrap();
        for cap in extract_numbers.captures_iter("dimensions 640x480 left 100 top 100") {
            println!(
                "width: {}; height: {}; left: {}; top: {}.  Full line: {}",
                &cap[1], &cap[2], &cap[3], &cap[4], &cap[0]
            );
        }

        let replace_regex = Regex::new(r"h.llo").unwrap();
        println!(
            "{}",
            replace_regex.replace_all("hello hello hello", "goodbye")
        );
    }

    pub fn read_file(file: &str) -> String {
        fs::read_to_string(file).unwrap().trim().to_owned()
    }

    // No trim!
    pub fn read_file_raw(file: &str) -> String {
        fs::read_to_string(file).unwrap()
    }

    pub fn input_lines(file: &str) -> impl Iterator<Item = String> {
        let f = File::open(file).unwrap_or_else(|_| panic!("Failed to open input file: {}", &file));
        BufReader::new(f).lines().map(Result::unwrap)
    }

    pub fn sample_input(input: &str) -> Vec<String> {
        input.trim().split('\n').map(str::to_owned).collect()
    }

    pub fn permutations<T>(inputs: Vec<T>) -> Vec<Vec<T>>
    where
        T: Clone + std::fmt::Debug,
    {
        if inputs.is_empty() {
            vec![Vec::new()]
        } else {
            let elt = inputs.get(0).unwrap();
            let subperms = permutations(inputs.iter().skip(1).cloned().collect());

            subperms
                .iter()
                .flat_map(|subperm: &Vec<T>| {
                    (0..=subperm.len()).map(move |idx| {
                        let mut r = subperm.clone();
                        r.insert(idx, elt.clone());
                        r
                    })
                })
                .collect()
        }
    }
}

mod day1 {
    use crate::shared::*;

    pub fn part1() {
        let readings: Vec<String> = input_lines("input_files/day1.txt").collect();

        let mut max_calories = 0;

        for elf in readings.split(|s| s.is_empty()) {
            let total_calories = elf.iter().map(|s| s.parse::<usize>().expect("int parse")).sum();

            if total_calories > max_calories {
                max_calories = total_calories;
            }
        }

        println!("Max calories: {}", max_calories);
    }

    pub fn part2() {
        let readings: Vec<String> = input_lines("input_files/day1.txt").collect();

        let mut elf_calories: Vec<usize> = readings.split(|s| s.is_empty()).map(|elf| {
            elf.iter().map(|s| s.parse::<usize>().expect("int parse")).sum()
        }).collect();

        elf_calories.sort();
        elf_calories.reverse();

        println!("Top three: {}", elf_calories[0] + elf_calories[1] + elf_calories[2]);
    }
}

mod day2 {
    use itertools::Itertools;

    use crate::shared::*;

    #[derive(Clone, Copy)]
    enum Hand {
        Rock,
        Paper,
        Scissors,
    }

    #[derive(Eq, PartialEq)]
    enum Outcome {
        WeWin,
        TheyWin,
        Draw,
    }

    fn score_hand(h: Hand) -> usize {
        match h {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn score_round(opponent: Hand, us: Hand) -> Outcome {
        match (opponent, us) {
            (Hand::Rock, Hand::Rock) => Outcome::Draw,
            (Hand::Scissors, Hand::Scissors) => Outcome::Draw,
            (Hand::Paper, Hand::Paper) => Outcome::Draw,
            (Hand::Rock, Hand::Paper) => Outcome::WeWin,
            (Hand::Rock, Hand::Scissors) => Outcome::TheyWin,
            (Hand::Scissors, Hand::Paper) => Outcome::TheyWin,
            (Hand::Scissors, Hand::Rock) => Outcome::WeWin,
            (Hand::Paper, Hand::Rock) => Outcome::TheyWin,
            (Hand::Paper, Hand::Scissors) => Outcome::WeWin,
        }
    }

    fn parse_hand(ch: char) -> Hand {
        match ch {
            'A' | 'X' => Hand::Rock,
            'B' | 'Y' => Hand::Paper,
            'C' | 'Z' => Hand::Scissors,
            _ => panic!("Parse error: {}", ch),
        }
    }

    fn parse_outcome(ch: char) -> Outcome {
        match ch {
            'X' => Outcome::TheyWin,
            'Y' => Outcome::Draw,
            'Z' => Outcome::WeWin,
            _ => panic!("Parse error: {}", ch),
        }
    }


    pub fn part1() {
        let mut total_score = 0;

        for line in input_lines("input_files/day2.txt") {
            if let Some((opponent, _, me)) = line.chars().collect_tuple() {
                let opponent = parse_hand(opponent);
                let me = parse_hand(me);

                let round_score = score_hand(me) + match score_round(opponent, me) {
                    Outcome::TheyWin => 0,
                    Outcome::Draw => 3,
                    Outcome::WeWin => 6,
                };

                total_score += round_score;
            }
        }

        println!("Total score: {}", total_score);
    }

    pub fn part2() {
        let mut total_score = 0;

        for line in input_lines("input_files/day2.txt") {
            if let Some((opponent, _, target_outcome)) = line.chars().collect_tuple() {
                let opponent = parse_hand(opponent);
                let target_outcome = parse_outcome(target_outcome);

                for &candidate_move in &[Hand::Rock, Hand::Paper, Hand::Scissors] {
                    if score_round(opponent, candidate_move) == target_outcome {
                        let round_score = score_hand(candidate_move) + match score_round(opponent, candidate_move) {
                            Outcome::TheyWin => 0,
                            Outcome::Draw => 3,
                            Outcome::WeWin => 6,
                        };

                        total_score += round_score;

                        break;
                    }
                }
            }
        }

        println!("Total score: {}", total_score);
    }
}


mod dayn {
    use crate::shared::*;

    pub fn part1() {

    }
    pub fn part2() {}
}

fn main() {
    if false {
        day1::part1();
        day1::part2();
    }

    day2::part1();
    day2::part2();

}
