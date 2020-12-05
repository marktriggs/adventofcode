// (cd ../; cargo run --release)

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(dead_code)]

#[macro_use]
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
    pub use std::sync::Mutex;

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

mod day4 {
    use crate::shared::*;

    const REQUIRED_PASSPORT_FIELDS: &[&str] = &[
        "byr", // (Birth Year)
        "iyr", // (Issue Year)
        "eyr", // (Expiration Year)
        "hgt", // (Height)
        "hcl", // (Hair Color)
        "ecl", // (Eye Color)
        "pid", // (Passport ID)
               // "cid", // (Country ID) - ignored due to trickery
    ];

    type PassportRules = HashMap<&'static str, fn(&str) -> bool>;

    fn validation_rules() -> PassportRules {
        let mut rules: PassportRules = HashMap::new();

        rules.insert("byr", |s| {
            if let Ok(n) = s.parse::<usize>() {
                n >= 1920 && n <= 2002
            } else {
                false
            }
        });

        rules.insert("iyr", |s| {
            if let Ok(n) = s.parse::<usize>() {
                n >= 2010 && n <= 2020
            } else {
                false
            }
        });

        rules.insert("eyr", |s| {
            if let Ok(n) = s.parse::<usize>() {
                n >= 2020 && n <= 2030
            } else {
                false
            }
        });

        rules.insert("hgt", |s| {
            if let Ok(n) = s
                .trim_end_matches(|c: char| !c.is_numeric())
                .parse::<usize>()
            {
                if s.ends_with("cm") {
                    return n >= 150 && n <= 193;
                } else if s.ends_with("in") {
                    return n >= 59 && n <= 76;
                }
            }

            false
        });

        rules.insert("hcl", |s| {
            Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(s)
        });

        rules.insert("ecl", |s| {
            Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$")
                .unwrap()
                .is_match(s)
        });

        rules.insert("pid", |s| Regex::new(r"^[0-9]{9}$").unwrap().is_match(s));

        rules
    }

    struct Passport {
        values: HashMap<String, String>,
    }

    impl Passport {
        fn from_lines(lines: &[String]) -> Passport {
            assert!(!lines.is_empty());

            let mut result = Passport {
                values: HashMap::new(),
            };

            for line in lines {
                for chunk in line.split(' ') {
                    let bits: Vec<&str> = chunk.split(':').collect();
                    result.values.insert(bits[0].to_owned(), bits[1].to_owned());
                }
            }

            result
        }

        fn is_valid(&self) -> bool {
            for &field in REQUIRED_PASSPORT_FIELDS {
                if (!self.values.contains_key(field)) {
                    return false;
                }
            }

            true
        }

        fn is_valid_by_rules(&self, rules: &PassportRules) -> bool {
            for &field in REQUIRED_PASSPORT_FIELDS {
                if (!self.values.contains_key(field)) {
                    return false;
                }

                let rule = rules.get(field).unwrap();
                let field_value = self.values.get(field).unwrap();

                if (!rule(field_value)) {
                    return false;
                }
            }

            true
        }
    }

    pub fn part1() {
        let lines: Vec<String> = input_lines("input_files/day4.txt").collect();

        println!(
            "There are {} valid passports",
            lines
                .split(|s| s.is_empty())
                .map(|passport_lines| Passport::from_lines(passport_lines))
                .filter(|passport| passport.is_valid())
                .count()
        );
    }

    pub fn part2() {
        let lines: Vec<String> = input_lines("input_files/day4.txt").collect();

        let rules = validation_rules();

        println!(
            "There are {} valid passports",
            lines
                .split(|s| s.is_empty())
                .map(|passport_lines| Passport::from_lines(passport_lines))
                .filter(|passport| passport.is_valid_by_rules(&rules))
                .count()
        );
    }
}

mod day5 {
    use crate::shared::*;

    #[derive(Debug)]
    enum WhichHalf {
        Lower,
        Upper,
    }

    #[derive(PartialEq, Eq, Debug, Hash)]
    pub struct Seat {
        row: usize,
        column: usize,
    }

    fn search_range(lower_inclusive: usize, upper_inclusive: usize, input: &[WhichHalf]) -> usize {
        let mut low = lower_inclusive;
        let mut high = upper_inclusive;

        for half in input {
            match half {
                WhichHalf::Lower => {
                    high = low + ((high - low) / 2);
                }
                WhichHalf::Upper => {
                    low = (low as f32 + ((high - low) as f32 / 2.0).ceil()) as usize;
                }
            }
        }

        assert_eq!(low, high);

        low
    }

    pub fn calculate_seat(instructions: &str) -> Seat {
        let instructions: Vec<WhichHalf> = instructions
            .chars()
            .map(|ch| {
                if ch == 'F' || ch == 'L' {
                    WhichHalf::Lower
                } else if ch == 'B' || ch == 'R' {
                    WhichHalf::Upper
                } else {
                    panic!("Bad mojo: {}", ch);
                }
            })
            .collect();

        let (row_instructions, column_instructions) = instructions.split_at(7);

        let row = search_range(0, 127, row_instructions);
        let column = search_range(0, 7, column_instructions);

        Seat { row, column }
    }

    pub fn part1() {
        let max_seat_id = input_lines("input_files/day5.txt")
            .map(|line| {
                let seat = calculate_seat(&line);
                (seat.row * 8) + seat.column
            })
            .max()
            .unwrap();

        println!("Maximum seat ID is {}", max_seat_id);
    }

    pub fn part2() {
        let mut all_seats: HashSet<Seat> = HashSet::new();

        input_lines("input_files/day5.txt").for_each(|line| {
            let seat = calculate_seat(&line);
            all_seats.insert(seat);
        });

        // Printed it out and eyeballed it :)
        for row in 0..=127 {
            for column in 0..=7 {
                if (all_seats.contains(&Seat { row, column })) {
                    print!("X");
                } else {
                    print!("?")
                }
            }
            println!();
        }

        for row in 1..=126 {
            for column in 0..=7 {
                if (!all_seats.contains(&Seat { row, column })) {
                    println!(
                        "Free seat: {:?} with id {}",
                        Seat { row, column },
                        (row * 8) + column
                    );
                }
            }
        }
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
