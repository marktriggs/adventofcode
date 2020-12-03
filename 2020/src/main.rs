// (cd ../; cargo run --release)

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(dead_code)]

extern crate lazy_static;
extern crate regex;

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
    pub use std::str;

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
        T: Clone + Copy + std::fmt::Debug,
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
                        r.insert(idx, *elt);
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
        let amounts: Vec<usize> = input_lines("input_files/day1.txt")
            .map(|s| s.parse().unwrap())
            .collect();

        for i in 0..amounts.len() {
            for j in (i + 1)..amounts.len() {
                if (amounts[i] + amounts[j]) == 2020 {
                    println!(
                        "{} * {} = {}",
                        amounts[i],
                        amounts[j],
                        amounts[i] * amounts[j]
                    );
                }
            }
        }
    }

    pub fn part2() {
        let amounts: Vec<usize> = input_lines("input_files/day1.txt")
            .map(|s| s.parse().unwrap())
            .collect();

        for i in 0..amounts.len() {
            for j in (i + 1)..amounts.len() {
                for k in (j + 1)..amounts.len() {
                    if (amounts[i] + amounts[j] + amounts[k]) == 2020 {
                        println!(
                            "{} * {} * {} = {}",
                            amounts[i],
                            amounts[j],
                            amounts[k],
                            amounts[i] * amounts[j] * amounts[k]
                        );
                    }
                }
            }
        }
    }
}

mod day2 {
    use crate::shared::*;

    #[derive(Debug)]
    struct Password {
        min: usize,
        max: usize,
        letter: char,
        candidate_password: String,
    }

    impl Password {
        fn from_line(line: String) -> Password {
            let pattern = Regex::new(r"^([0-9]+)-([0-9]+) (.): (.+)$").unwrap();

            if let Some(cap) = pattern.captures(&line) {
                return Password {
                    min: cap[1].parse().unwrap(),
                    max: cap[2].parse().unwrap(),
                    letter: cap[3].parse().unwrap(),
                    candidate_password: cap[4].to_owned(),
                };
            }

            panic!("Password parse error for {}", line);
        }

        fn is_valid(&self) -> bool {
            let c = self
                .candidate_password
                .chars()
                .filter(|&ch| ch == self.letter)
                .count();

            (c >= self.min && c <= self.max)
        }

        fn is_valid_new_policy(&self) -> bool {
            let chars: Vec<char> = self.candidate_password.chars().collect();

            (chars[self.min - 1] == self.letter) ^ (chars[self.max - 1] == self.letter)
        }
    }

    pub fn part1() {
        let candidate_passwords: Vec<Password> = input_lines("input_files/day2.txt")
            .map(Password::from_line)
            .collect();

        println!(
            "Valid password count: {}",
            candidate_passwords.iter().filter(|p| p.is_valid()).count()
        )
    }

    pub fn part2() {
        let candidate_passwords: Vec<Password> = input_lines("input_files/day2.txt")
            .map(Password::from_line)
            .collect();

        println!(
            "Valid password count: {}",
            candidate_passwords
                .iter()
                .filter(|p| p.is_valid_new_policy())
                .count()
        )
    }
}

mod day3 {
    use crate::shared::*;

    struct Map {
        width: usize,
        height: usize,
        grid: Vec<Vec<char>>,
    }

    impl Map {
        fn from_input(path: &str) -> Map {
            let grid: Vec<Vec<char>> = input_lines(path)
                .map(|line| line.chars().collect())
                .collect();

            Map {
                width: grid[0].len(),
                height: grid.len(),
                grid,
            }
        }

        fn is_tree(&self, x: usize, y: usize) -> bool {
            self.grid[y][x] == '#'
        }
    }

    fn trees_hit(map: &Map, right: usize, down: usize) -> usize {
        let mut x = 0;
        let mut y = 0;
        let mut trees_hit = 0;

        while y < map.height {
            if map.is_tree(x, y) {
                trees_hit += 1;
            }

            x = (x + right) % map.width;
            y += down;
        }

        trees_hit
    }

    pub fn part1() {
        let map = Map::from_input("input_files/day3.txt");

        println!(
            "Whacked into {} trees on our way down",
            trees_hit(&map, 3, 1)
        );
    }

    pub fn part2() {
        let map = Map::from_input("input_files/day3.txt");

        println!(
            "{}",
            trees_hit(&map, 1, 1)
                * trees_hit(&map, 3, 1)
                * trees_hit(&map, 5, 1)
                * trees_hit(&map, 7, 1)
                * trees_hit(&map, 1, 2)
        );
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
    }

    day3::part1();
    day3::part2();
}
