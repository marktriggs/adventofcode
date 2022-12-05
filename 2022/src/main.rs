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

    pub use itertools::Itertools;
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

mod day3 {
    use std::ops::Index;

    use itertools::Itertools;

    use crate::shared::*;

    fn priority(item: char) -> usize {
        let priorities: Vec<char> = ('a' ..= 'z').chain('A' ..= 'Z').collect();

        priorities.iter().position(|&ch| ch == item).unwrap() + 1
    }

    pub fn part1() {
        let mut result = 0;

        for line in input_lines("input_files/day3.txt") {
            let compartment_size = line.chars().count() / 2;

            let compartment_one: HashSet<char> = line.chars().take(compartment_size).collect();
            let compartment_two: HashSet<char> = line.chars().skip(compartment_size).collect();

            let overlap = compartment_one.intersection(&compartment_two).next().unwrap();

            result += priority(*overlap);
        }

        println!("Total priority: {}", result);
    }

    pub fn part2() {
        let mut total = 0;

        for group in &input_lines("input_files/day3.txt").chunks(3) {
            let group_lines: Vec<String> = group.collect();

            let all_chars: HashSet<char> = group_lines.join("").chars().collect();
            let group_sets: Vec<HashSet<char>> = group_lines.iter().map(|line| line.chars().collect::<HashSet<char>>()).collect();

            for ch in all_chars {
                let count: usize = group_sets.iter().map(|set| usize::from(set.contains(&ch))).sum();

                if count == 3 {
                    total += priority(ch);
                }
            }
        }

        println!("Total priority (pt2): {}", total);
    }
}

mod day4 {
    use crate::shared::*;

    pub fn part1() {
        let mut count = 0;
        for line in input_lines("input_files/day4.txt") {
            if let Some((elf1_s, elf2_s)) = line.split(',').collect_tuple() {
                let (elf1_start, elf1_end) = elf1_s.split('-').map(|s| s.parse::<usize>().unwrap()).collect_tuple().unwrap();
                let (elf2_start, elf2_end) = elf2_s.split('-').map(|s| s.parse::<usize>().unwrap()).collect_tuple().unwrap();

                if (elf1_start >= elf2_start && elf1_end <= elf2_end) || (elf2_start >= elf1_start && elf2_end <= elf1_end) {
                    count += 1;
                }
            }
        }

        println!("Fully contained assignment count: {}", count);
    }

    pub fn part2() {
        let mut count = 0;
        for line in input_lines("input_files/day4.txt") {
            if let Some((elf1_s, elf2_s)) = line.split(',').collect_tuple() {
                let (elf1_start, elf1_end) = elf1_s.split('-').map(|s| s.parse::<usize>().unwrap()).collect_tuple().unwrap();
                let (elf2_start, elf2_end) = elf2_s.split('-').map(|s| s.parse::<usize>().unwrap()).collect_tuple().unwrap();

                if (elf1_end < elf2_start) || (elf1_start > elf2_end) {
                    // no overlap
                } else {
                    count += 1;
                }
            }
        }

        println!("Any overlap count: {}", count);
    }
}


mod day5 {
    use crate::shared::*;

    pub fn part1() {
        // Not even showing the common decency to parse the starting state.  Just mangled it using shell/cut/emacs
        let mut stacks: Vec<VecDeque<char>> = vec![
            VecDeque::new(),
            "QWPSZRHD".chars().collect(),
            "VBRWQHF".chars().collect(),
            "CVSH".chars().collect(),
            "HFG".chars().collect(),
            "PGJBZ".chars().collect(),
            "QTJHWFL".chars().collect(),
            "ZTWDLVJN".chars().collect(),
            "DTZCJGHF".chars().collect(),
            "WPVMBH".chars().collect(),
        ];

        let move_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        for line in input_lines("input_files/day5.txt") {
            for cap in move_regex.captures_iter(&line) {
                if cap.len() > 0 {
                    let count = cap[1].parse::<usize>().unwrap();
                    let source = cap[2].parse::<usize>().unwrap();
                    let destination = cap[3].parse::<usize>().unwrap();

                    for _ in (0..count) {
                        let elt = stacks[source].pop_back().unwrap();
                        stacks[destination].push_back(elt);
                    }
                }
            }
        }

        for stack in &mut stacks[1..] {
            print!("{}", stack.pop_back().unwrap());
        }

        println!();
    }


    pub fn part2() {
        // Not even showing the common decency to parse the starting state.  Just mangled it using shell/cut/emacs
        let mut stacks: Vec<VecDeque<char>> = vec![
            VecDeque::new(),
            "QWPSZRHD".chars().collect(),
            "VBRWQHF".chars().collect(),
            "CVSH".chars().collect(),
            "HFG".chars().collect(),
            "PGJBZ".chars().collect(),
            "QTJHWFL".chars().collect(),
            "ZTWDLVJN".chars().collect(),
            "DTZCJGHF".chars().collect(),
            "WPVMBH".chars().collect(),
        ];

        let move_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        for line in input_lines("input_files/day5.txt") {
            for cap in move_regex.captures_iter(&line) {
                if cap.len() > 0 {
                    let count = cap[1].parse::<usize>().unwrap();
                    let source = cap[2].parse::<usize>().unwrap();
                    let destination = cap[3].parse::<usize>().unwrap();

                    let mut buffer = VecDeque::new();

                    for _ in (0..count) {
                        let elt = stacks[source].pop_back().unwrap();
                        buffer.push_front(elt);
                    }

                    while !buffer.is_empty() {
                        let elt = buffer.pop_front().unwrap();
                        stacks[destination].push_back(elt);
                    }
                }
            }
        }

        for stack in &mut stacks[1..] {
            print!("{}", stack.pop_back().unwrap());
        }

        println!();
    }
}

mod dayn {
    use crate::shared::*;

    pub fn part1() {}
    pub fn part2() {}
}

fn main() {
    if false {
        day1::part1();
        day1::part2();

        day2::part1();
        day2::part2();

        day3::part1();
        day3::part2();

        day4::part1();
        day4::part2();
    }

    day5::part1();
    day5::part2();
}
