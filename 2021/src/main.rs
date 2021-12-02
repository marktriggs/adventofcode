// (cd ~/projects/adventofcode/2021 && cargo run)

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(dead_code)]

mod shared {
    pub use regex::Regex;

    // pub use intcode::{self, IntCode};
    pub use std::cell::RefCell;
    pub use std::cmp::{self, Ordering};
    pub use std::collections::BTreeMap;
    pub use std::collections::BTreeSet;
    pub use std::collections::HashMap;
    pub use std::collections::HashSet;
    pub use std::collections::LinkedList;
    pub use std::collections::VecDeque;
    pub use std::convert::TryInto;
    pub use std::fmt::{self, Display};
    pub use std::fs::{self, File};
    pub use std::io::{self, BufRead, BufReader, Read, Write};
    pub use std::iter::FromIterator;
    pub use std::rc::Rc;
    pub use std::str;
    pub use std::sync::{Arc, Mutex};

    pub use anyhow::{bail, Error};

    pub use itertools::Itertools;

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
        let readings: Vec<usize> = input_lines("input_files/day1.txt")
            .map(|s| s.parse().unwrap())
            .collect();

        let mut increases = 0;

        for idx in 1..readings.len() {
            if readings[idx] > readings[idx - 1] {
                increases += 1;
            }
        }

        println!("Increases: {}", increases);
    }

    pub fn part2() {
        let readings: Vec<usize> = input_lines("input_files/day1.txt")
            .map(|s| s.parse().unwrap())
            .collect();

        let window_sums: Vec<usize> = readings.iter().tuple_windows().map(|(a, b, c)| a + b + c).collect();


        let mut increases = 0;

        for idx in 1..window_sums.len() {
            if window_sums[idx] > window_sums[idx - 1] {
                increases += 1;
            }
        }

        println!("Increases: {}", increases);
    }
}


mod day2 {
    use crate::shared::*;

    #[derive(Debug)]
    enum Op {
        Forward,
        Down,
        Up,
    }

    #[derive(Debug)]
    struct Command {
        op: Op,
        n: i64,
    }

    impl std::str::FromStr for Command {
        type Err = Error;

        fn from_str(s: &str) -> Result<Command, Error> {
            let words: Vec<&str> = s.split(' ').collect();

            Ok(Command {
                op: match words[0] {
                    "forward" => Op::Forward,
                    "down" => Op::Down,
                    "up" => Op::Up,
                    _ => bail!("Parse error"),
                },
                n: words[1].parse()?,
            })
        }
    }

    pub fn part1() {
        let mut horizontal: i64 = 0;
        let mut depth: i64 = 0;

        let commands: Vec<Command> = input_lines("input_files/day2.txt")
            .map(|s| s.parse().unwrap())
            .collect();

        for command in commands {
            match command.op {
                Op::Forward => horizontal += command.n,
                Op::Down => depth += command.n,
                Op::Up => depth -= command.n,
            }
        }

        println!("Horizontal: {}, Depth: {}.  Product: {}", horizontal, depth, horizontal * depth);
    }

    pub fn part2() {
        let mut horizontal: i64 = 0;
        let mut depth: i64 = 0;
        let mut aim: i64 = 0;

        let commands: Vec<Command> = input_lines("input_files/day2.txt")
            .map(|s| s.parse().unwrap())
            .collect();

        for command in commands {
            match command.op {
                Op::Forward => {
                    horizontal += command.n;
                    depth += (aim * command.n);
                },
                Op::Down => aim += command.n,
                Op::Up => aim -= command.n,
            }
        }

        println!("Horizontal: {}, Depth: {}.  Product: {}", horizontal, depth, horizontal * depth);
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
    }

    day2::part1();
    day2::part2();
}