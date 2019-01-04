// (cd ../; cargo run --release)

#![allow(unused_parens)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate regex;

mod shared {
    pub use regex::Regex;

    pub use std::cmp::{self, Ordering};
    pub use std::collections::HashMap;
    pub use std::collections::HashSet;
    pub use std::collections::LinkedList;
    pub use std::fmt::{self, Display};
    pub use std::fs::File;
    pub use std::io::{self, BufRead, BufReader, Write};
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

        let replace_regex = Regex::new(r"hello").unwrap();
        println!(
            "{}",
            replace_regex.replace_all("hello hello hello", "goodbye")
        );
    }

    pub fn input_lines(file: &str) -> impl Iterator<Item = String> {
        let f = File::open(file).expect(&format!("Failed to open input file: {}", &file));
        BufReader::new(f).lines().map(Result::unwrap)
    }

    fn sample_input(input: &str) -> Vec<String> {
        input.trim().split("\n").map(str::to_owned).collect()
    }

    // Points
    #[derive(Hash, Eq, PartialEq, Debug, Clone)]
    pub struct Point {
        pub x: u64,
        pub y: u64,
    }

    impl Point {
        pub fn parse_csv(s: &str) -> Point {
            let parsed: Vec<u64> = s
                .replace(" ", "")
                .split(",")
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
            Point {
                x: parsed[0],
                y: parsed[1],
            }
        }
    }

    pub fn format_grid<T>(grid: &Vec<Vec<T>>) -> String
    where
        T: Display,
    {
        let mut result = String::new();

        for row in grid {
            for cell in row {
                result.push_str(&format!("{}", cell));
            }

            result.push_str("\n");
        }

        result
    }

}

mod santasm {
    use crate::shared::*;

    pub type Registers = Vec<usize>;

    pub trait Operation {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize);
        fn pretty(&self, a: usize, b: usize, c: usize);
    }

    // Addition
    pub struct OpAddr;
    impl Operation for OpAddr {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = regs[a] + regs[b];
        }

        fn pretty(&self, a: usize, b: usize, c: usize) {
            println!("regs[{}] = regs[{}] + regs[{}]", c, a, b);
        }
    }

    pub struct OpAddi;
    impl Operation for OpAddi {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = regs[a] + b;
        }

        fn pretty(&self, a: usize, b: usize, c: usize) {
            println!("regs[{}] = regs[{}] + {}", c, a, b);
        }
    }

    // Multiplication
    pub struct OpMulr;
    impl Operation for OpMulr {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = regs[a] * regs[b];
        }

        fn pretty(&self, a: usize, b: usize, c: usize) {
            println!("regs[{}] = regs[{}] * regs[{}]", c, a, b);
        }
    }

    pub struct OpMuli;
    impl Operation for OpMuli {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = regs[a] * b;
        }

        fn pretty(&self, a: usize, b: usize, c: usize) {
            println!("regs[{}] = regs[{}] * {}", c, a, b);
        }
    }

    // Bitwise AND
    pub struct OpBanr;
    impl Operation for OpBanr {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = regs[a] & regs[b];
        }

        fn pretty(&self, a: usize, b: usize, c: usize) {
            println!("regs[{}] = regs[{}] & regs[{}]", c, a, b);
        }
    }

    pub struct OpBani;
    impl Operation for OpBani {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = regs[a] & b;
        }

        fn pretty(&self, a: usize, b: usize, c: usize) {
            println!("regs[{}] = regs[{}] & {}", c, a, b);
        }
    }

    // Bitwise OR
    pub struct OpBorr;
    impl Operation for OpBorr {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = regs[a] | regs[b];
        }

        fn pretty(&self, a: usize, b: usize, c: usize) {
            println!("regs[{}] = regs[{}] | regs[{}]", c, a, b);
        }
    }

    pub struct OpBori;
    impl Operation for OpBori {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = regs[a] | b;
        }

        fn pretty(&self, a: usize, b: usize, c: usize) {
            println!("regs[{}] = regs[{}] | {}", c, a, b);
        }
    }

    // Assignment
    pub struct OpSetr;
    impl Operation for OpSetr {
        fn invoke(&self, regs: &mut Registers, a: usize, _b: usize, c: usize) {
            regs[c] = regs[a];
        }

        fn pretty(&self, a: usize, _b: usize, c: usize) {
            println!("regs[{}] = regs[{}]", c, a);
        }
    }

    pub struct OpSeti;
    impl Operation for OpSeti {
        fn invoke(&self, regs: &mut Registers, a: usize, _b: usize, c: usize) {
            regs[c] = a
        }

        fn pretty(&self, a: usize, _b: usize, c: usize) {
            println!("regs[{}] = {}", c, a);
        }
    }

    // Greater-than testing
    pub struct OpGtir;
    impl Operation for OpGtir {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = if a > regs[b] { 1 } else { 0 };
        }

        fn pretty(&self, a: usize, b: usize, c: usize) {
            println!("regs[{}] = if {} > regs[{}] {{ 1 }} else {{ 0 }}", c, a, b);
        }
    }

    pub struct OpGtri;
    impl Operation for OpGtri {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = if regs[a] > b { 1 } else { 0 };
        }

        fn pretty(&self, a: usize, b: usize, c: usize) {
            println!("regs[{}] = if regs[{}] > {} {{ 1 }} else {{ 0 }}", c, a, b);
        }
    }

    pub struct OpGtrr;
    impl Operation for OpGtrr {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = if regs[a] > regs[b] { 1 } else { 0 };
        }

        fn pretty(&self, a: usize, b: usize, c: usize) {
            println!(
                "regs[{}] = if regs[{}] > regs[{}] {{ 1 }} else {{ 0 }}",
                c, a, b
            );
        }
    }

    // Equality testing
    pub struct OpEqir;
    impl Operation for OpEqir {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = if a == regs[b] { 1 } else { 0 };
        }

        fn pretty(&self, a: usize, b: usize, c: usize) {
            println!("regs[{}] = if {} == regs[{}] {{ 1 }} else {{ 0 }}", c, a, b);
        }
    }

    pub struct OpEqri;
    impl Operation for OpEqri {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = if regs[a] == b { 1 } else { 0 };
        }

        fn pretty(&self, a: usize, b: usize, c: usize) {
            println!("regs[{}] = if regs[{}] == {} {{ 1 }} else {{ 0 }}", c, a, b);
        }
    }

    pub struct OpEqrr;
    impl Operation for OpEqrr {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = if regs[a] == regs[b] { 1 } else { 0 };
        }

        fn pretty(&self, a: usize, b: usize, c: usize) {
            println!(
                "regs[{}] = if regs[{}] == regs[{}] {{ 1 }} else {{ 0 }}",
                c, a, b
            );
        }
    }

    pub fn instruction_set() -> HashMap<&'static str, &'static Operation> {
        let mut map: HashMap<&'static str, &'static Operation> = HashMap::new();
        map.insert("addr", &OpAddr);
        map.insert("addi", &OpAddi);
        map.insert("mulr", &OpMulr);
        map.insert("muli", &OpMuli);
        map.insert("banr", &OpBanr);
        map.insert("bani", &OpBani);
        map.insert("borr", &OpBorr);
        map.insert("bori", &OpBori);
        map.insert("setr", &OpSetr);
        map.insert("seti", &OpSeti);
        map.insert("gtir", &OpGtir);
        map.insert("gtri", &OpGtri);
        map.insert("gtrr", &OpGtrr);
        map.insert("eqir", &OpEqir);
        map.insert("eqri", &OpEqri);
        map.insert("eqrr", &OpEqrr);

        map
    }

}

mod day1 {
    use crate::shared::*;

    pub fn part1() {
        let frequency = input_lines("input_files/day1.txt")
            .map(|s| s.parse().unwrap())
            .fold(0, |acc: i64, n: i64| acc + n);

        println!("Final frequency: {}", frequency);
    }

    pub fn part2() {
        let mut seen_frequencies = HashSet::new();

        let frequencies: Vec<i64> = input_lines("input_files/day1.txt")
            .map(|s| s.parse().unwrap_or(0))
            .collect();

        let mut frequency = 0;
        for &f in frequencies.iter().cycle() {
            frequency += f;

            if seen_frequencies.contains(&frequency) {
                println!("First repeated frequency: {}", frequency);
                break;
            } else {
                seen_frequencies.insert(frequency);
            }
        }
    }
}

mod day2 {
    use crate::shared::*;

    pub fn part1() {
        let mut two_repeats = 0;
        let mut three_repeats = 0;

        for code in input_lines("input_files/day2.txt") {
            let mut freqs = HashMap::new();

            for ch in code.chars() {
                let entry = freqs.entry(ch).or_insert(0);
                *entry += 1
            }

            let mut two_increment = 0;
            let mut three_increment = 0;
            for &count in freqs.values() {
                if count == 2 {
                    two_increment = 1;
                } else if count == 3 {
                    three_increment = 1;
                }
            }

            two_repeats += two_increment;
            three_repeats += three_increment;
        }

        println!("Twos: {}; threes: {}", two_repeats, three_repeats);
        println!("Checksum: {}", two_repeats * three_repeats);
    }

    pub fn part2() {
        let mut tokens = HashMap::new();

        for code in input_lines("input_files/day2.txt") {
            for idx in 0..code.len() {
                let mut key = code.to_owned();
                key.remove(idx);

                let entry = tokens.entry(key).or_insert(HashSet::new());
                entry.insert(code.to_owned());
            }
        }

        for (key, token) in tokens {
            if token.len() == 2 {
                println!("Yep: {} {:?}", key, token);
            }
        }
    }
}

mod day3 {
    use crate::shared::*;

    pub fn part1() {
        let input = input_lines("input_files/day3.txt");
        let mut used = HashMap::new();

        for claim in input {
            let bits: Vec<u64> = claim
                .split(|ch| " x:,".contains(ch))
                .enumerate()
                .filter(|(i, _)| [2, 3, 5, 6].contains(i))
                .map(|(_, elt)| elt.parse::<u64>().unwrap())
                .collect();

            let left = bits[0];
            let top = bits[1];
            let width = bits[2];
            let height = bits[3];

            for y in 0..height {
                for x in 0..width {
                    let entry = used
                        .entry(Point {
                            x: x + left,
                            y: y + top,
                        })
                        .or_insert(0);
                    *entry += 1;
                }
            }
        }

        let mut total = 0;
        for (_point, count) in used {
            if count >= 2 {
                total += 1;
            }
        }

        println!("Result: {}", total);
    }

    pub fn part2() {
        let mut used = HashMap::new();

        for claim in input_lines("input_files/day3.txt") {
            let bits: Vec<u64> = claim
                .split(|ch| " x:,".contains(ch))
                .enumerate()
                .filter(|(i, _)| [2, 3, 5, 6].contains(i))
                .map(|(_, elt)| elt.parse::<u64>().unwrap())
                .collect();

            let left = bits[0];
            let top = bits[1];
            let width = bits[2];
            let height = bits[3];

            for y in 0..height {
                for x in 0..width {
                    let entry = used
                        .entry(Point {
                            x: x + left,
                            y: y + top,
                        })
                        .or_insert(0);
                    *entry += 1;
                }
            }
        }

        for claim in input_lines("input_files/day3.txt") {
            let bits: Vec<u64> = claim
                .split(|ch| " x:,".contains(ch))
                .enumerate()
                .filter(|(i, _)| [2, 3, 5, 6].contains(i))
                .map(|(_, elt)| elt.parse::<u64>().unwrap())
                .collect();

            let left = bits[0];
            let top = bits[1];
            let width = bits[2];
            let height = bits[3];

            let mut found = true;
            for y in 0..height {
                for x in 0..width {
                    let entry = used
                        .get(&Point {
                            x: x + left,
                            y: y + top,
                        })
                        .unwrap();
                    if *entry != 1 {
                        found = false;
                        break;
                    }
                }
            }

            if found {
                println!("Found claim: {}", claim);
            }
        }
    }
}

mod day4 {
    use crate::shared::*;

    #[derive(Debug)]
    struct Guard {
        id: String,
        sleep_time: usize,
        sleep_minutes: Vec<usize>,
    }

    fn new_guard(id: String) -> Guard {
        Guard {
            id: id,
            sleep_time: 0,
            sleep_minutes: vec![0; 60],
        }
    }

    pub fn part1() {
        let mut events: Vec<String> = input_lines("input_files/day4.txt").collect();
        events.sort();

        let start_shift =
            Regex::new(r"\[\d{4}-\d{2}-\d{2} (\d+):(\d+)\] Guard #(\d+) begins shift").unwrap();
        let start_sleep = Regex::new(r"\[\d{4}-\d{2}-\d{2} 00:(\d+)\] falls asleep").unwrap();
        let end_sleep = Regex::new(r"\[\d{4}-\d{2}-\d{2} 00:(\d+)\] wakes up").unwrap();

        let mut guards: HashMap<String, Guard> = HashMap::new();
        let mut active_guard: Option<String> = None;
        let mut sleep_started = 0;

        for event in events {
            if let Some(cap) = start_shift.captures(&event) {
                let name = cap[3].to_string();

                guards
                    .entry(name.clone())
                    .or_insert_with(|| new_guard(name.clone()));
                active_guard = Some(name);
            } else if let Some(cap) = start_sleep.captures(&event) {
                sleep_started = cap[1].parse().unwrap();
            } else if let Some(cap) = end_sleep.captures(&event) {
                let sleep_ended = cap[1].parse().unwrap();
                let guard = guards.get_mut(active_guard.as_ref().unwrap()).unwrap();

                for time in sleep_started..sleep_ended {
                    guard.sleep_time += 1;
                    guard.sleep_minutes[time] += 1;
                }
            } else {
                panic!("Bad input: {}", event);
            }
        }

        let laziest_guard = guards
            .values()
            .max_by_key(|guard| guard.sleep_time)
            .unwrap();

        let most_sleepy_time: (usize, &usize) = laziest_guard
            .sleep_minutes
            .iter()
            .enumerate()
            .max_by_key(|(_minute, &sleep_occurrences)| sleep_occurrences)
            .unwrap();

        println!(
            "Laziest guard ({}) slept {} minutes",
            laziest_guard.id, laziest_guard.sleep_time
        );
        println!(
            "We'll strike at {} minutes past midnight",
            most_sleepy_time.0
        );
    }

    pub fn part2() {
        let mut events: Vec<String> = input_lines("input_files/day4.txt").collect();
        events.sort();

        let start_shift =
            Regex::new(r"\[\d{4}-\d{2}-\d{2} (\d+):(\d+)\] Guard #(\d+) begins shift").unwrap();
        let start_sleep = Regex::new(r"\[\d{4}-\d{2}-\d{2} 00:(\d+)\] falls asleep").unwrap();
        let end_sleep = Regex::new(r"\[\d{4}-\d{2}-\d{2} 00:(\d+)\] wakes up").unwrap();

        let mut guards: HashMap<String, Guard> = HashMap::new();
        let mut active_guard: Option<String> = None;
        let mut sleep_started = 0;

        for event in events {
            if let Some(cap) = start_shift.captures(&event) {
                let name = cap[3].to_string();
                guards
                    .entry(name.clone())
                    .or_insert_with(|| new_guard(name.clone()));
                active_guard = Some(name);
            } else if let Some(cap) = start_sleep.captures(&event) {
                sleep_started = cap[1].parse().unwrap();
            } else if let Some(cap) = end_sleep.captures(&event) {
                let sleep_ended = cap[1].parse().unwrap();
                let guard = guards.get_mut(active_guard.as_ref().unwrap()).unwrap();

                for time in sleep_started..sleep_ended {
                    guard.sleep_time += 1;
                    guard.sleep_minutes[time] += 1;
                }
            } else {
                panic!("Bad input: {}", event);
            }
        }

        let mut laziest_guard: Option<String> = None;
        let mut laziest_minute = 0;
        let mut laziest_count = 0;

        for guard in guards.values() {
            for (minute, &count) in guard.sleep_minutes.iter().enumerate() {
                if count > laziest_count {
                    laziest_minute = minute;
                    laziest_guard = Some(guard.id.clone());
                    laziest_count = count;
                }
            }
        }

        println!(
            "Laziest guard: {} was at minute {}",
            laziest_guard.unwrap(),
            laziest_minute
        );
    }
}

mod day5 {
    use crate::shared::*;

    pub fn part1() {
        let mut polymer: String = include_str!("../input_files/day5.txt").trim().to_owned();

        let patterns: Vec<String> = ALPHABET
            .chars()
            .zip(ALPHABET_UPPER.chars())
            .map(|(lower, upper)| format!("{}{}|{}{}", lower, upper, upper, lower))
            .collect();

        let replace_regex = Regex::new(&patterns.join("|")).unwrap();

        loop {
            let new_polymer = replace_regex.replace_all(&polymer, "");

            if polymer != new_polymer {
                polymer = new_polymer.to_string();
            } else {
                println!("End length: {}", polymer.len());
                break;
            }
        }
    }

    pub fn part2() {
        let patterns: Vec<String> = ALPHABET
            .chars()
            .zip(ALPHABET_UPPER.chars())
            .map(|(lower, upper)| format!("{}{}|{}{}", lower, upper, upper, lower))
            .collect();

        let replace_regex = Regex::new(&patterns.join("|")).unwrap();

        let mut lengths = Vec::new();

        for kill in ALPHABET.chars() {
            let mut polymer: String = include_str!("../input_files/day5.txt").trim().to_owned();

            let killupper: String = kill.to_uppercase().to_string();

            polymer = polymer.replace(kill, "");
            polymer = polymer.replace(&killupper, "");

            loop {
                let new_polymer = replace_regex.replace_all(&polymer, "");

                if polymer != new_polymer {
                    polymer = new_polymer.to_string();
                } else {
                    lengths.push(polymer.len());
                    break;
                }
            }
        }

        println!("Best: {}", lengths.iter().min().unwrap());
    }

    pub fn part1_alternative() {
        let polymer = include_str!("../input_files/day5.txt").trim();

        let mut input = polymer.as_bytes().to_vec();

        loop {
            let mut done = true;
            let mut i = 0;

            // Mark
            while i < input.len() - 1 {
                if (input[i] as i32 - input[i + 1] as i32).abs() == 32 {
                    done = false;
                    input[i] = 0;
                    input[i + 1] = 0;
                    i += 2;
                } else {
                    i += 1;
                }
            }

            if done {
                break;
            }

            // Sweep
            input.retain(|&b| b > 0);
        }

        println!("End length: {}", input.len());
    }

    pub fn part2_alternative() {
        let mut lengths = Vec::new();

        for kill in ALPHABET.chars() {
            let killupper: String = kill.to_uppercase().to_string();

            let mut polymer: String = include_str!("../input_files/day5.txt").trim().to_owned();

            polymer = polymer.replace(kill, "");
            polymer = polymer.replace(&killupper, "");

            let mut input = polymer.as_bytes().to_vec();

            loop {
                let mut done = true;
                let mut i = 0;

                // Mark
                while i < input.len() - 1 {
                    if (input[i] as i32 - input[i + 1] as i32).abs() == 32 {
                        done = false;
                        input[i] = 0;
                        input[i + 1] = 0;
                        i += 2;
                    } else {
                        i += 1;
                    }
                }

                if done {
                    break;
                }

                // Sweep
                input.retain(|&b| b > 0);
            }

            lengths.push(input.len());
        }

        println!("Best: {}", lengths.iter().min().unwrap());
    }
}

mod day6 {
    use crate::shared::*;

    fn abs_diff(a: u64, b: u64) -> u64 {
        if a > b {
            a - b
        } else {
            b - a
        }
    }

    fn manhattan_distance(p1: &Point, p2: &Point) -> u64 {
        abs_diff(p1.x, p2.x) + abs_diff(p1.y, p2.y)
    }

    fn closest_point(points: &Vec<Point>, point: &Point) -> Option<usize> {
        let mut distances: Vec<u64> = points
            .iter()
            .map(|p| manhattan_distance(&point, &p))
            .collect();

        distances.sort();

        if distances[0] != distances[1] {
            // We have a distinct closest point.  Find it and return its index.
            for i in 0..points.len() {
                if manhattan_distance(&points[i], &point) == distances[0] {
                    return Some(i);
                }
            }

            unreachable!();
        } else {
            // Equidistant
            None
        }
    }

    pub fn part1() {
        let input: Vec<String> = input_lines("input_files/day6.txt").collect();

        let points: Vec<Point> = input.iter().map(|s| Point::parse_csv(s)).collect();

        // we'll define a grid whose top-left is 0,0 and whose bottom right is max_x+1, max_y+1
        let max_x: usize = (&points.iter().map(|p| p.x).max().unwrap() + 1) as usize;
        let max_y: usize = (&points.iter().map(|p| p.y).max().unwrap() + 1) as usize;

        let mut grid: Vec<Vec<Option<usize>>> = (0..max_y).map(|_| vec![None; max_x]).collect();

        for y in 0..max_y {
            for x in 0..max_x {
                let point = Point {
                    x: x as u64,
                    y: y as u64,
                };

                grid[y][x] = closest_point(&points, &point);
            }
        }

        // Any point on the outer edge of our grid is infinite and not counted
        let mut excluded_points = HashSet::new();

        for e in grid.first().unwrap() {
            excluded_points.insert(e);
        }
        for e in grid.last().unwrap() {
            excluded_points.insert(e);
        }
        for y in 0..max_y {
            excluded_points.insert(&grid[y][0]);
        }

        let mut frequencies = HashMap::new();
        for p in grid.iter().flatten() {
            if p.is_some() && !excluded_points.contains(p) {
                let entry = frequencies.entry(p).or_insert(0);
                *entry += 1;
            }
        }

        println!("Winner: {}", frequencies.values().max().unwrap());
    }

    fn distance_sums(points: &Vec<Point>, point: &Point) -> u64 {
        points.iter().map(|p| manhattan_distance(&point, &p)).sum()
    }

    pub fn part2() {
        let input: Vec<String> = input_lines("input_files/day6.txt").collect();

        let points: Vec<Point> = input.iter().map(|s| Point::parse_csv(s)).collect();

        let max_x = (&points.iter().map(|p| p.x).max().unwrap() + 1) as usize;
        let max_y = (&points.iter().map(|p| p.y).max().unwrap() + 1) as usize;

        let mut region_size = 0;

        for y in 0..max_y {
            for x in 0..max_x {
                let point = Point {
                    x: x as u64,
                    y: y as u64,
                };
                if distance_sums(&points, &point) < 10000 {
                    region_size += 1;
                }
            }
        }

        println!("Region size: {}", region_size);
    }

}

mod day7 {
    use crate::shared::*;

    pub fn part1() {
        let input: Vec<String> = input_lines("input_files/day7.txt").collect();

        // K requires V[] to start
        let mut dependencies: HashMap<String, HashSet<String>> = HashMap::new();

        for s in input {
            let mut bits: Vec<String> = s.split(" ").map(str::to_owned).collect();

            let dependent = bits.remove(7);
            let depends_on = bits.remove(1);

            dependencies
                .entry(depends_on.clone())
                .or_insert(HashSet::new());
            let dep_e = dependencies
                .entry(dependent.clone())
                .or_insert(HashSet::new());
            dep_e.insert(depends_on.clone());
        }

        // Steps we've already run
        let mut completed_steps = HashSet::new();

        // Our final ordering
        let mut result = String::new();

        while completed_steps.len() != dependencies.len() {
            // Find the dependencies that haven't yet been run, and whose
            // dependencies have been satisfied.
            let mut ready: Vec<String> = dependencies
                .keys()
                .filter(|&k| {
                    !completed_steps.contains(k)
                        && dependencies.get(k).unwrap().is_subset(&completed_steps)
                })
                .map(|s| s.to_owned())
                .collect();

            // Run the first in alphabetic order
            ready.sort();
            let next_step = ready.remove(0);
            result.push_str(&next_step);
            completed_steps.insert(next_step.clone());
        }

        println!("{}", result);
    }

    #[derive(Clone, Debug)]
    enum Worker {
        Active { work_remaining: usize, task: String },

        Idle,
    }

    fn task_cost(task: &str) -> usize {
        (task.chars().next().unwrap() as i64 - 64) as usize
    }

    pub fn part2() {
        let input: Vec<String> = input_lines("input_files/day7.txt").collect();

        const BASE_COST: usize = 60;
        const WORKER_COUNT: usize = 5;

        // Task K requires all of Tasks V[] to start
        let dependencies = {
            let mut dependencies: HashMap<String, HashSet<String>> = HashMap::new();

            for s in input {
                let mut bits: Vec<String> = s.split(" ").map(str::to_owned).collect();

                let task = bits.remove(7);
                let prerequisite_task = bits.remove(1);

                dependencies
                    .entry(prerequisite_task.clone())
                    .or_insert(HashSet::new());

                dependencies
                    .entry(task.clone())
                    .or_insert(HashSet::new())
                    .insert(prerequisite_task.clone());
            }

            dependencies
        };

        let ordered_tasks: Vec<String> = {
            let mut v: Vec<String> = dependencies.keys().cloned().collect();
            v.sort();
            v
        };

        // Tasks we've already run
        let mut completed_tasks = HashSet::new();

        // Our faithful workers, and the set of tasks they're currently chewing on
        let mut workers: Vec<Worker> = vec![Worker::Idle; WORKER_COUNT];
        let mut work_in_progress = HashSet::new();

        let mut seconds_elapsed = 0;

        loop {
            // Handle work currently running
            for i in 0..workers.len() {
                // If the worker is doing something, decrement their workload
                if let Worker::Active {
                    ref mut work_remaining,
                    ref task,
                } = workers[i]
                {
                    if *work_remaining == 1 {
                        // Task complete!
                        completed_tasks.insert(work_in_progress.take(task).unwrap());
                        workers[i] = Worker::Idle;
                    } else {
                        *work_remaining -= 1;
                    }
                };
            }

            if completed_tasks.len() == ordered_tasks.len() {
                // We're done!
                break;
            }

            // Allocate new work to anyone who needs it
            for i in 0..workers.len() {
                if let Worker::Active { .. } = workers[i] {
                    // Worker is occupied
                    continue;
                }

                if let Some(next_task) = ordered_tasks.iter().find(|&k| {
                    !completed_tasks.contains(k)
                        && !work_in_progress.contains(k)
                        && dependencies.get(k).unwrap().is_subset(&completed_tasks)
                }) {
                    // If the worker is free, assign some work.
                    work_in_progress.insert(next_task.clone());

                    workers[i] = Worker::Active {
                        task: next_task.clone(),
                        work_remaining: BASE_COST + &task_cost(next_task),
                    };
                }
            }

            seconds_elapsed += 1;
        }

        println!("{}", seconds_elapsed);
    }

}

mod day8 {
    fn sum_metadata(input: &mut Vec<u64>, total: u64) -> u64 {
        if input.is_empty() {
            return total;
        }

        let child_count = input.remove(0);
        let metadata_count = input.remove(0);

        let mut new_total = total;
        for _ in 0..child_count {
            new_total += sum_metadata(input, 0)
        }

        for _ in 0..metadata_count {
            new_total += input.remove(0);
        }

        new_total
    }

    pub fn part1() {
        let input_s = include_str!("../input_files/day8.txt").trim().to_owned();

        let mut input: Vec<u64> = input_s
            .trim()
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect();

        println!("{}", sum_metadata(&mut input, 0));
    }

    #[derive(Debug)]
    struct Node {
        idx: usize,
        metadata: Vec<usize>,
        child_indexes: Vec<usize>,
    }

    // Really could have just used regular ownership here: have Node own its children.  Oh well!
    fn parse_nodes(input: &mut Vec<usize>, result: &mut Vec<Node>) {
        if input.is_empty() {
            // Done!
            return;
        }

        let child_count = input.remove(0);
        let metadata_count = input.remove(0);

        let new_node = Node {
            idx: result.len(),
            metadata: Vec::new(),
            child_indexes: Vec::new(),
        };

        let my_idx = result.len();
        result.push(new_node);

        for _ in 0..child_count as usize {
            let idx = result.len();
            result[my_idx].child_indexes.push(idx);
            parse_nodes(input, result);
        }

        for _ in 0..metadata_count {
            result[my_idx].metadata.push(input.remove(0));
        }
    }

    fn calculate_value(nodes: &Vec<Node>, idx: usize) -> usize {
        let target_node = &nodes[idx];

        if target_node.child_indexes.is_empty() {
            // sum of metadata entries
            target_node.metadata.iter().sum()
        } else {
            // metadata entries are indexes!
            let mut result = 0;

            for &m in &target_node.metadata {
                if m > 0 && (m - 1) < target_node.child_indexes.len() {
                    result += calculate_value(nodes, target_node.child_indexes[m - 1]);
                }
            }

            result
        }
    }

    pub fn part2() {
        let input_s = include_str!("../input_files/day8.txt").trim().to_owned();

        let mut input: Vec<usize> = input_s
            .trim()
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect();
        let mut output: Vec<Node> = Vec::new();

        parse_nodes(&mut input, &mut output);

        println!("Value of root node: {}", calculate_value(&output, 0));
    }
}

mod day9 {
    pub fn part1() {
        // 452 players; last marble is worth 70784 points

        let max_marble = 70784;
        let players = 452;

        let mut board = vec![0];
        board.reserve(max_marble);
        let mut player_scores = vec![0; players];

        let mut current_marble_idx = 0;
        let mut current_player = 0;

        for marble in 1..=max_marble {
            if marble % 23 == 0 {
                // player scores!
                player_scores[current_player] += marble;
                let len = board.len() as i64;
                let idx_to_remove =
                    ((((current_marble_idx as i64 - 7) % len) + len) % len) as usize;
                let removed = board.remove(idx_to_remove);
                player_scores[current_player] += removed;
                current_marble_idx = idx_to_remove;

            // println!("Player {} gets {} and {}", current_player, marble, removed);
            } else {
                // default case: place the next marble after the marble clockwise one step from current.
                let mut place_at_pos = current_marble_idx + 2;

                if place_at_pos >= board.len() {
                    place_at_pos = place_at_pos - board.len();
                }

                board.insert(place_at_pos, marble);
                current_marble_idx = place_at_pos;
            }

            current_player += 1;

            if current_player == players {
                current_player = 0;
            }
        }

        println!("{:?}", player_scores.iter().max().unwrap());
    }

    pub fn part2() {
        let max_marble = 7078400;
        let players = 452;

        // V[x] is to the left of x
        let mut left_relationships: Vec<usize> = vec![0; max_marble + 1];

        // V[x] is to the right of x
        let mut right_relationships: Vec<usize> = vec![0; max_marble + 1];

        let mut player_scores = vec![0; players];

        let mut current_marble_idx = 0;
        let mut current_player = 0;

        for marble in 1..=max_marble {
            if marble % 23 == 0 {
                // player scores!
                player_scores[current_player] += marble;

                // Find the marble seven to the left and remove it.
                let to_remove = (0..7).fold(current_marble_idx, |idx, _| left_relationships[idx]);

                player_scores[current_player] += to_remove;

                let left_of_victim = left_relationships[to_remove];
                let right_of_victim = right_relationships[to_remove];

                left_relationships[right_of_victim] = left_of_victim;
                right_relationships[left_of_victim] = right_of_victim;

                current_marble_idx = right_of_victim;
            } else {
                // default case: place the next marble after the marble clockwise one step from current.
                let insert_after = right_relationships[current_marble_idx];

                let old_right = right_relationships[insert_after];
                right_relationships[insert_after] = marble;
                left_relationships[old_right] = marble;

                right_relationships[marble] = old_right;
                left_relationships[marble] = insert_after;

                current_marble_idx = marble;
            }

            current_player += 1;

            if current_player == players {
                current_player = 0;
            }
        }

        println!("{:?}", player_scores.iter().max().unwrap());
    }

}

mod day10 {
    use crate::shared::*;

    #[derive(Debug)]
    struct PointOfLight {
        position: (i64, i64),
        velocity: (i64, i64),
    }

    lazy_static! {
        static ref POINT_OF_LIGHT_REGEX: Regex =
            { Regex::new(r"position=<(.+)> velocity=<(.*)>").unwrap() };
    }

    impl PointOfLight {
        fn from_str(input: &str) -> PointOfLight {
            fn parse_pair(s: &str) -> Vec<i64> {
                s.replace(" ", "")
                    .split(",")
                    .map(|s| s.parse().unwrap())
                    .collect()
            }

            let cap = POINT_OF_LIGHT_REGEX.captures(input).unwrap();
            let position = parse_pair(&cap[1]);
            let velocity = parse_pair(&cap[2]);

            PointOfLight {
                position: (position[0], position[1]),
                velocity: (velocity[0], velocity[1]),
            }
        }

        fn tick(&mut self) {
            self.position = (
                self.position.0 + self.velocity.0,
                self.position.1 + self.velocity.1,
            );
        }
    }

    // Write out a frame as a PPM image
    fn write_frame(grid: &Vec<Vec<char>>, out: &mut impl Write) {
        const PIXEL_SIZE: usize = 5;

        let img_width = PIXEL_SIZE * grid[0].len();
        let img_height = PIXEL_SIZE * grid.len();

        out.write_all(b"P6\n").unwrap();
        out.write_all(format!("{}\n", img_width).as_bytes())
            .unwrap();
        out.write_all(format!("{}\n", img_height).as_bytes())
            .unwrap();
        out.write_all(b"255\n").unwrap();

        let mut output_row: Vec<u8> = Vec::new();

        for row in grid {
            output_row.clear();

            for &cell in row {
                let val = if cell == ' ' { 0 } else { 255 };

                for _ in 0..PIXEL_SIZE {
                    // RGB
                    output_row.push(val);
                    output_row.push(val);
                    output_row.push(val);
                }
            }

            // Repeat the row PIXEL_SIZE to make square pixels.
            for _ in 0..PIXEL_SIZE {
                out.write_all(&output_row).unwrap();
            }
        }
    }

    // WARNING: dumps ppm files to stdout
    //
    // Run with: target/release/adventofcode2018 | ffmpeg -vcodec ppm -f image2pipe -framerate 60 -i - out.mp4
    //
    pub fn part1() {
        fn to_uniform(value: i64, min_value: i64, max_value: i64) -> f64 {
            (value - min_value) as f64 / (max_value - min_value) as f64
        }

        let input = input_lines("input_files/day10.txt");

        const GRID_SIZE: usize = 200;
        const FRAMES_TO_GENERATE: usize = 15000;

        let mut points: Vec<PointOfLight> =
            input.map(|line| PointOfLight::from_str(&line)).collect();

        let stdout = io::stdout();
        let mut handle = stdout.lock();

        for _frame in 0..FRAMES_TO_GENERATE {
            for p in &mut points {
                p.tick();
            }

            let min_pos = points
                .iter()
                .map(|p| vec![p.position.0, p.position.1])
                .flatten()
                .min()
                .unwrap();
            let max_pos = points
                .iter()
                .map(|p| vec![p.position.0, p.position.1])
                .flatten()
                .max()
                .unwrap();

            let mut grid: Vec<Vec<char>> = (0..GRID_SIZE).map(|_| vec![' '; GRID_SIZE]).collect();

            for p in &points {
                let x =
                    (to_uniform(p.position.0, min_pos, max_pos) * (GRID_SIZE - 1) as f64).floor();
                let y =
                    (to_uniform(p.position.1, min_pos, max_pos) * (GRID_SIZE - 1) as f64).floor();

                grid[y as usize][x as usize] = '#';
            }

            write_frame(&grid, &mut handle);
        }
    }

    pub fn part2() {
        fn to_uniform(value: i64, min_value: i64, max_value: i64) -> f64 {
            (value - min_value) as f64 / (max_value - min_value) as f64
        }

        let input = input_lines("input_files/day10.txt");

        const GRID_SIZE: usize = 200;

        let mut points: Vec<PointOfLight> =
            input.map(|line| PointOfLight::from_str(&line)).collect();

        let mut seconds = 0;

        for _ in 0..10123 {
            seconds += 1;
            for p in &mut points {
                p.tick();
            }
        }

        for frame in 0..1 {
            seconds += 1;
            println!("Produced frame {} at second {}", frame, seconds);

            for p in &mut points {
                p.tick();
            }

            let min_pos = points
                .iter()
                .map(|p| vec![p.position.0, p.position.1])
                .flatten()
                .min()
                .unwrap();
            let max_pos = points
                .iter()
                .map(|p| vec![p.position.0, p.position.1])
                .flatten()
                .max()
                .unwrap();

            let mut grid: Vec<Vec<char>> = (0..GRID_SIZE).map(|_| vec![' '; GRID_SIZE]).collect();

            for p in &points {
                let x =
                    (to_uniform(p.position.0, min_pos, max_pos) * (GRID_SIZE - 1) as f64).floor();
                let y =
                    (to_uniform(p.position.1, min_pos, max_pos) * (GRID_SIZE - 1) as f64).floor();

                grid[y as usize][x as usize] = '#';
            }

            // Show the grid
            let mut out = File::create(format!("frame_{:07}.ppm", frame)).unwrap();
            write_frame(&grid, &mut out);
        }
    }
}

mod day11 {
    fn three_by_three_power(grid: &Vec<Vec<i64>>, x: usize, y: usize) -> i64 {
        let mut result = 0;

        for yoff in 0..3 {
            for xoff in 0..3 {
                result += grid[y + yoff][x + xoff];
            }
        }

        result
    }

    pub fn part1() {
        const GRID_SIZE: usize = 300;
        const INPUT: i64 = 9435;

        let mut grid: Vec<Vec<i64>> = (0..GRID_SIZE).map(|_| vec![0; GRID_SIZE]).collect();

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let rack_id: i64 = (x as i64 + 1) + 10;
                let mut power_level: i64 = rack_id as i64 * (y as i64 + 1);

                power_level += INPUT;
                power_level *= rack_id;
                power_level = (power_level / 100) % 10;
                power_level -= 5;

                grid[y][x] = power_level;
            }
        }

        let mut best_three_by_three: i64 = i64::min_value();
        let mut best_coordinate: (usize, usize) = (0, 0);

        for y in 0..(GRID_SIZE - 3) {
            for x in 0..(GRID_SIZE - 3) {
                let value = three_by_three_power(&grid, x, y);

                if value > best_three_by_three {
                    best_three_by_three = value;
                    best_coordinate = (x + 1, y + 1);
                }
            }
        }

        println!(
            "Best value was at {:?} with value {}",
            best_coordinate, best_three_by_three
        );
    }

    fn grid_power(grid: &Vec<Vec<i64>>, x: usize, y: usize, size: usize) -> i64 {
        let mut result = 0;

        for yoff in 0..size {
            for xoff in 0..size {
                result += grid[y + yoff][x + xoff];
            }
        }

        result
    }

    pub fn part2() {
        const GRID_SIZE: usize = 300;
        const INPUT: i64 = 9435;

        let mut grid: Vec<Vec<i64>> = (0..GRID_SIZE).map(|_| vec![0; GRID_SIZE]).collect();

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let rack_id: i64 = (x as i64 + 1) + 10;
                let mut power_level: i64 = rack_id as i64 * (y as i64 + 1);

                power_level += INPUT;
                power_level *= rack_id;
                power_level = (power_level / 100) % 10;
                power_level -= 5;

                grid[y][x] = power_level;
            }
        }

        let mut best_grid_value: i64 = i64::min_value();
        let mut best_coordinate: (usize, usize) = (0, 0);
        let mut best_size: usize = 0;

        for size in 1..GRID_SIZE {
            for y in 0..(GRID_SIZE - size) {
                for x in 0..(GRID_SIZE - size) {
                    let value = grid_power(&grid, x, y, size);

                    if value > best_grid_value {
                        best_grid_value = value;
                        best_coordinate = (x + 1, y + 1);
                        best_size = size;
                    }
                }
            }
        }

        println!(
            "Best value was at {:?} with value {} and size {}",
            best_coordinate, best_grid_value, best_size
        );
    }
}

mod day12 {
    use crate::shared::*;

    fn load_rules() -> HashSet<Vec<u8>> {
        let mut result = HashSet::new();

        result.insert(b"#..#.".to_vec());
        result.insert(b"#...#".to_vec());
        result.insert(b".##.#".to_vec());
        result.insert(b"##...".to_vec());
        result.insert(b"##.#.".to_vec());
        result.insert(b".#.##".to_vec());
        result.insert(b"#.#..".to_vec());
        result.insert(b"#####".to_vec());
        result.insert(b"..#.#".to_vec());
        result.insert(b"...#.".to_vec());
        result.insert(b"####.".to_vec());
        result.insert(b".#...".to_vec());
        result.insert(b"#.#.#".to_vec());
        result.insert(b".##..".to_vec());
        result.insert(b".#..#".to_vec());
        result.insert(b"##.##".to_vec());
        result.insert(b".###.".to_vec());

        result
    }

    pub fn part1() {
        const RULE_LEN: usize = 5;
        let initial_state = b"#....#.#....#....#######..##....###.##....##.#.#.##...##.##.#...#..###....#.#...##.###.##.###...#..#";

        let padding: i64 = 11000;
        let mut state: Vec<u8> = Vec::new();
        let pot_numbers = ((0 - padding as i64)..(padding + initial_state.len() as i64));

        state.extend(vec![b'.'; padding as usize]);
        state.extend(initial_state.iter());
        state.extend(vec![b'.'; padding as usize]);

        let rules: HashSet<Vec<u8>> = load_rules();

        for generation in 0..21 {
            // Print our running total
            let result = pot_numbers.clone().zip(&state).fold(0, |acc, (idx, &elt)| {
                acc + if elt == b'#' { idx } else { 0 }
            });

            println!("Generation result {}: {}", generation, result);

            // Generate new state
            let mut new_state = vec![b'.'; state.len()];

            for i in 0..state.len() - RULE_LEN {
                if rules.contains(&state[i..i + RULE_LEN]) {
                    new_state[i + 2] = b'#';
                }
            }

            state = new_state;
        }
    }

    pub fn part2() {
        // After much messing around, recognised that the above total for each
        // subsequent generation ends up increasing by a constant of 98.  Used
        // arithmetic to work out the 50b case!
    }

}

mod day13 {
    use crate::shared::*;

    #[derive(Debug, Eq, PartialEq)]
    struct Vector2 {
        x: i64,
        y: i64,
    }

    // [.0 .1
    //  .2 .3]
    struct Rotation(i64, i64, i64, i64);

    const RIGHT_ROTATE: Rotation = Rotation(0, -1, 1, 0);
    const LEFT_ROTATE: Rotation = Rotation(0, 1, -1, 0);
    const STRAIGHT_ROTATE: Rotation = Rotation(1, 0, 0, 1);

    const UP: Vector2 = Vector2 { x: 0, y: -1 };
    const DOWN: Vector2 = Vector2 { x: 0, y: 1 };
    const LEFT: Vector2 = Vector2 { x: -1, y: 0 };
    const RIGHT: Vector2 = Vector2 { x: 1, y: 0 };

    const INTERSECTION_TURNS: &[Rotation] = &[LEFT_ROTATE, STRAIGHT_ROTATE, RIGHT_ROTATE];

    #[derive(Debug)]
    struct Cart {
        direction: Vector2,
        next_intersection_turn_idx: usize,
        last_move_tick: usize,
    }

    type Track = char;

    impl Default for Cart {
        fn default() -> Cart {
            Cart {
                direction: UP,
                next_intersection_turn_idx: 0,
                last_move_tick: 0,
            }
        }
    }

    impl Cart {
        fn adjust_direction(&mut self, current_track: Track) {
            match current_track {
                '-' | '|' => {
                    // Stay the course
                }
                '\\' => {
                    self.direction = Vector2 {
                        x: self.direction.y,
                        y: self.direction.x,
                    }
                }
                '/' => {
                    self.direction = Vector2 {
                        x: self.direction.y * -1,
                        y: self.direction.x * -1,
                    }
                }
                '+' => {
                    self.direction = rotate_vector2(
                        &self.direction,
                        &INTERSECTION_TURNS[self.next_intersection_turn_idx],
                    );
                    self.next_intersection_turn_idx =
                        (self.next_intersection_turn_idx + 1) % INTERSECTION_TURNS.len();
                }
                _ => {
                    panic!("Unknown type of track: {}", current_track);
                }
            }
        }
    }

    fn rotate_vector2(v: &Vector2, r: &Rotation) -> Vector2 {
        Vector2 {
            x: (v.x * r.0) + (v.y * r.1),
            y: (v.x * r.2) + (v.y * r.3),
        }
    }

    struct World {
        map: Vec<Vec<Track>>,
        carts: Vec<Vec<Option<Cart>>>,
    }

    impl fmt::Display for World {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for y in 0..self.map.len() {
                for x in 0..self.map[0].len() {
                    if let Some(cart) = &self.carts[y][x] {
                        write!(
                            f,
                            "{}",
                            match cart.direction {
                                LEFT => "<",
                                UP => "^",
                                DOWN => "v",
                                RIGHT => ">",
                                _ => unreachable!(),
                            }
                        )?;
                    } else {
                        write!(f, "{}", self.map[y][x])?;
                    }
                }
                write!(f, "\n")?;
            }

            Ok(())
        }
    }

    fn parse_cart_world(input: &str) -> World {
        World {
            map: input
                .replace(">", "-")
                .replace("<", "-")
                .replace("^", "|")
                .replace("v", "|")
                .split("\n")
                .filter(|&row| !row.is_empty())
                .map(|row| row.chars().collect::<Vec<Track>>())
                .collect(),

            carts: input
                .split("\n")
                .filter(|&row| !row.is_empty())
                .map(|row| {
                    row.chars()
                        .map(|ch| match ch {
                            '>' => Some(Cart {
                                direction: RIGHT,
                                ..Default::default()
                            }),
                            '^' => Some(Cart {
                                direction: UP,
                                ..Default::default()
                            }),
                            'v' => Some(Cart {
                                direction: DOWN,
                                ..Default::default()
                            }),
                            '<' => Some(Cart {
                                direction: LEFT,
                                ..Default::default()
                            }),
                            _ => None,
                        })
                        .collect()
                })
                .collect(),
        }
    }

    pub fn part1() {
        let input = include_str!("../input_files/day13.txt").to_owned();
        let mut world = parse_cart_world(&input);

        let mut tick = 0;
        loop {
            tick += 1;

            // println!("{}", world);

            for y in 0..world.map.len() {
                for x in 0..world.map[0].len() {
                    if let Some(cart) = &world.carts[y][x] {
                        if (cart.last_move_tick == tick) {
                            // Already moved this cart during this tick
                            continue;
                        }

                        world.carts[y].push(None);
                        let mut cart = world.carts[y].swap_remove(x).unwrap();

                        cart.adjust_direction(world.map[y][x]);
                        cart.last_move_tick = tick;

                        let new_x = (x as i64 + cart.direction.x) as usize;
                        let new_y = (y as i64 + cart.direction.y) as usize;

                        if let Some(_) = world.carts[new_y][new_x] {
                            println!("Collision at {} {} (coming from {} {})", new_x, new_y, x, y);
                            return;
                        } else {
                            world.carts[new_y][new_x] = Some(cart);
                        }
                    }
                }
            }
        }
    }

    pub fn part2() {
        let input = include_str!("../input_files/day13.txt").to_owned();
        let mut world = parse_cart_world(&input);

        let mut tick = 0;
        loop {
            tick += 1;

            // println!("{}", world);

            for y in 0..world.map.len() {
                for x in 0..world.map[0].len() {
                    if let Some(cart) = &world.carts[y][x] {
                        if (cart.last_move_tick == tick) {
                            // Already moved this cart during this tick
                            continue;
                        }

                        world.carts[y].push(None);
                        let mut cart = world.carts[y].swap_remove(x).unwrap();

                        cart.adjust_direction(world.map[y][x]);
                        cart.last_move_tick = tick;

                        let new_x = (x as i64 + cart.direction.x) as usize;
                        let new_y = (y as i64 + cart.direction.y) as usize;

                        if let Some(_) = world.carts[new_y][new_x] {
                            // Collision.  Remove the other cart too
                            world.carts[new_y][new_x] = None;
                        } else {
                            world.carts[new_y][new_x] = Some(cart);
                        }
                    }
                }
            }

            let remaining_carts = world.carts.iter().flatten().filter(|&cart| cart.is_some());
            if remaining_carts.count() == 1 {
                for y in 0..world.map.len() {
                    for x in 0..world.map[0].len() {
                        if let Some(_) = &world.carts[y][x] {
                            println!("The loneliest cart: {},{}", x, y);
                            return;
                        }
                    }
                }
            }
        }
    }
}

mod day14 {
    pub fn part1() {
        let mut scores = vec![3, 7];

        let mut elf1_current_idx = 0;
        let mut elf2_current_idx = 1;

        let input = 580741;
        let output_length = 10;

        while scores.len() < input + output_length {
            // Expand scores
            let mut sum = scores[elf1_current_idx] + scores[elf2_current_idx];
            let pos = scores.len();
            loop {
                scores.insert(pos, sum % 10);
                sum = sum / 10;

                if sum == 0 {
                    break;
                }
            }

            // Update elf current indexes
            elf1_current_idx = (elf1_current_idx + scores[elf1_current_idx] + 1) % scores.len();
            elf2_current_idx = (elf2_current_idx + scores[elf2_current_idx] + 1) % scores.len();
        }

        println!(
            "{} to {}: {}",
            input,
            input + output_length,
            &scores[input..input + output_length]
                .iter()
                .map(|score| format!("{}", score))
                .collect::<String>()
        );
    }

    pub fn part2() {
        let mut scores = vec![3, 7];

        let mut elf1_current_idx = 0;
        let mut elf2_current_idx = 1;

        let input = &[5, 8, 0, 7, 4, 1];

        loop {
            // Expand scores
            let mut sum = scores[elf1_current_idx] + scores[elf2_current_idx];
            let mut new_digits = Vec::new();
            loop {
                new_digits.insert(0, sum % 10);

                sum = sum / 10;

                if sum == 0 {
                    break;
                }
            }

            while !new_digits.is_empty() {
                scores.push(new_digits.remove(0));

                if scores.len() >= input.len()
                    && &scores[(scores.len() - input.len())..scores.len()] == input
                {
                    println!("{:?}", &scores[(scores.len() - input.len())..scores.len()]);
                    println!("Result: {}", scores.len() - input.len());
                    return;
                }
            }

            // Update elf current indexes
            elf1_current_idx = (elf1_current_idx + scores[elf1_current_idx] + 1) % scores.len();
            elf2_current_idx = (elf2_current_idx + scores[elf2_current_idx] + 1) % scores.len();
        }
    }
}

mod day15 {
    use crate::shared::*;

    const DAMAGE: i64 = 3;
    const DEFAULT_HITPOINTS: i64 = 200;

    #[derive(Eq, PartialEq, Hash, Debug)]
    enum Race {
        Elf,
        Goblin,
    }

    #[derive(Debug, Eq, PartialEq)]
    struct Unit {
        race: Race,
        hitpoints: i64,
        last_ticked: i64,
    }

    impl Unit {
        pub fn from_ch(ch: char) -> Unit {
            Unit {
                race: if ch == 'E' { Race::Elf } else { Race::Goblin },
                hitpoints: DEFAULT_HITPOINTS,
                last_ticked: -1,
            }
        }

        pub fn to_string(&self) -> &str {
            if self.race == Race::Elf {
                "E"
            } else {
                "G"
            }
        }

        pub fn dislikes(&self, other_unit: &Unit) -> bool {
            // Yeesh...
            self.race != other_unit.race
        }
    }

    #[derive(PartialEq, Eq)]
    enum Tile {
        Open,
        Wall,
        Occupied(Unit),
    }

    struct World {
        grid: Vec<Vec<Tile>>,
        dead_units: Vec<Unit>,
    }

    impl fmt::Debug for World {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for y in 0..self.height() {
                for x in 0..self.width() {
                    write!(
                        f,
                        "{}",
                        match &self.grid[y][x] {
                            Tile::Open => ".",
                            Tile::Wall => "#",
                            Tile::Occupied(unit) => unit.to_string(),
                        }
                    )?;
                }

                write!(f, "\n")?;
            }

            Ok(())
        }
    }

    impl World {
        pub fn height(&self) -> usize {
            self.grid.len()
        }

        pub fn width(&self) -> usize {
            self.grid[0].len()
        }

        pub fn from_str(s: &str) -> World {
            World {
                dead_units: Vec::new(),
                grid: s
                    .trim()
                    .split("\n")
                    .map(|line| {
                        line.chars()
                            .map(|ch| match ch {
                                '.' => Tile::Open,
                                '#' => Tile::Wall,
                                _ => Tile::Occupied(Unit::from_ch(ch)),
                            })
                            .collect()
                    })
                    .collect(),
            }
        }

        pub fn is_complete(&self, active_unit: &Unit) -> bool {
            let mut seen_types = HashSet::new();
            seen_types.insert(&active_unit.race);

            // We're complete if everyone is dead, or if one side is dead.
            for tile in self.grid.iter().flatten() {
                match tile {
                    Tile::Occupied(unit) => {
                        seen_types.insert(&unit.race);
                    }
                    _ => {}
                }
            }

            seen_types.len() <= 1
        }

        // Note: Must be in reading order!
        fn adjacent_tiles(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
            vec![(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)]
        }

        fn pop_tile(&mut self, x: usize, y: usize) -> Tile {
            self.grid[y].push(Tile::Open);
            self.grid[y].swap_remove(x)
        }

        fn try_attack(
            &mut self,
            active_unit: &mut Unit,
            x: usize,
            y: usize,
            attack_points: i64,
        ) -> bool {
            let mut attackable_positions: Vec<(usize, usize)> = Vec::new();

            // Don't just stand there: do something!
            for (adj_x, adj_y) in self.adjacent_tiles(x, y) {
                if let Tile::Occupied(other_unit) = &self.grid[adj_y][adj_x] {
                    if active_unit.dislikes(&other_unit) {
                        attackable_positions.push((adj_x, adj_y))
                    }
                }
            }

            if attackable_positions.is_empty() {
                // Well that didn't work
                return false;
            }

            // Unit with the lowest HP gets whacked.  If there are multiple with
            // the same HP, pick the reading order one (which in our case is the
            // position in the original array)
            let min_hp = attackable_positions
                .iter()
                .map(|&(x, y)| match &self.grid[y][x] {
                    Tile::Occupied(unit) => unit.hitpoints,
                    _ => unreachable!(),
                })
                .min()
                .unwrap();

            for (attackable_x, attackable_y) in attackable_positions {
                if let Tile::Occupied(mut victim) = self.pop_tile(attackable_x, attackable_y) {
                    if victim.hitpoints == min_hp {
                        // Whack!
                        victim.hitpoints -= attack_points;
                        if victim.hitpoints > 0 {
                            self.grid[attackable_y][attackable_x] = Tile::Occupied(victim);
                        } else {
                            // Victim died
                            self.dead_units.push(victim);
                        }
                        break;
                    } else {
                        self.grid[attackable_y][attackable_x] = Tile::Occupied(victim);
                    }
                }
            }

            true
        }

        fn all_elves_survived(&self) -> bool {
            !self.dead_units.iter().any(|u| u.race == Race::Elf)
        }

        fn next_move(&self, unit: &Unit, init_x: usize, init_y: usize) -> Option<(usize, usize)> {
            let mut exploration_paths: Vec<Vec<(usize, usize)>> = Vec::new();
            let mut visited_tiles: HashSet<(usize, usize)> = HashSet::new();

            // Contains a list of the points making up a path currently under consideration
            exploration_paths.push(vec![(init_x, init_y)]);

            // Contains the coordinates of tiles we've already captured in an earlier path
            visited_tiles.insert((init_x, init_y));

            let mut found_paths: Vec<Vec<(usize, usize)>> = Vec::new();

            while !exploration_paths.is_empty() {
                let path = exploration_paths.remove(0);

                if !found_paths.is_empty() && path.len() > found_paths[0].len() {
                    // We're not getting any better...
                    break;
                }

                let &(x, y) = path.last().unwrap();

                for (adj_x, adj_y) in self.adjacent_tiles(x, y) {
                    if visited_tiles.contains(&(adj_x, adj_y)) {
                        // Already checked this one
                        continue;
                    }

                    visited_tiles.insert((adj_x, adj_y));

                    // If we find a path, we need to keep looking for others
                    // with the same cost and then choose the winner based on
                    // the minimum reading order of the REACHABLE SQUARE, NOT of
                    // the next square we'll move to.
                    //
                    // Test case from reddit cleared this up:
                    //
                    // #######
                    // #.E..G#
                    // #.#####
                    // #G#####
                    // #######
                    //
                    // E should move right here, not left, because the
                    // destination goblin in reading order is the one on the
                    // same Y as the elf.
                    //
                    if let Tile::Occupied(other_unit) = &self.grid[adj_y][adj_x] {
                        if unit.dislikes(&other_unit) {
                            found_paths.push(path.clone());
                        }
                    } else if let Tile::Open = &self.grid[adj_y][adj_x] {
                        // One to explore
                        let mut new_path = path.clone();
                        new_path.push((adj_x, adj_y));

                        exploration_paths.push(new_path);
                    }
                }
            }

            if found_paths.is_empty() {
                None
            } else {
                // Find the minimum based on our destination square...
                let best = found_paths
                    .iter()
                    .min_by(|path1, path2| {
                        let p1_xy = path1.last().unwrap();
                        let p2_xy = path2.last().unwrap();

                        // reading order...
                        ((p1_xy.1, p1_xy.0)).cmp(&(p2_xy.1, p2_xy.0))
                    })
                    .unwrap();

                // ... but return the next move.
                Some(best[1])
            }
        }

        pub fn next_round(&mut self, round: i64, elf_attack_points: i64) -> bool {
            for y in 0..self.height() {
                for x in 0..self.width() {
                    if let Tile::Occupied(_) = &self.grid[y][x] {
                        if let Tile::Occupied(mut active_unit) = self.pop_tile(x, y) {
                            if active_unit.last_ticked == round {
                                // We've already simulated this unit during this round
                                self.grid[y][x] = Tile::Occupied(active_unit);
                                continue;
                            }

                            // If there's no one left to kill, that's this round
                            // (and game) over.
                            //
                            // "the number of full rounds that were completed
                            // (not counting the round in which combat ends)
                            // multiplied by the sum of the hit points of all
                            // remaining units at the moment combat ends. (Combat
                            // only ends when a unit finds no targets during its
                            // turn.)"

                            if self.is_complete(&active_unit) {
                                self.grid[y][x] = Tile::Occupied(active_unit);
                                return true;
                            }

                            active_unit.last_ticked = round;

                            let attack_points = if active_unit.race == Race::Elf {
                                elf_attack_points
                            } else {
                                DAMAGE
                            };

                            if self.try_attack(&mut active_unit, x, y, attack_points) {
                                // Attack successful.  Stay in position
                                self.grid[y][x] = Tile::Occupied(active_unit);
                            } else {
                                // Try moving
                                if let Some((new_x, new_y)) = self.next_move(&mut active_unit, x, y)
                                {
                                    assert!(self.grid[new_y][new_x] == Tile::Open);

                                    // Once we've moved, we can try another attack
                                    self.try_attack(&mut active_unit, new_x, new_y, attack_points);

                                    self.grid[new_y][new_x] = Tile::Occupied(active_unit);
                                } else {
                                    // Hold position
                                    self.grid[y][x] = Tile::Occupied(active_unit);
                                }
                            }
                        }
                    }
                }
            }

            // Game still in play
            false
        }

        pub fn remaining_hp(&self) -> usize {
            let mut result = 0;

            for y in 0..self.height() {
                for x in 0..self.width() {
                    if let Tile::Occupied(unit) = &self.grid[y][x] {
                        result += unit.hitpoints as usize;
                    }
                }
            }

            result
        }
    }

    pub fn part1() {
        for elf_attack_points in 3..4 {
            // My input
            let mut world = World::from_str(
                "
################################
#########.######################
#########..#####################
#########..G####################
########....#GG#################
########G......#################
########........################
###.####...#....################
#....###.###...G.###############
##......####.....#.#G..#.#######
###G.G...###.........#...#######
###......##...........##########
#............G#####...##########
#..G##G......#######..##########
#.G.#.......#########..#########
####..G.....#########...#.######
#...........#########..........#
##.#.....#..#########.E.....E..#
##.###..G.G.#########..........#
##...........#######E.#.......##
#.............#####..........###
#....#.....E................####
##.............##.E...........##
#....G.G.................###..##
#..............#.....E...###..##
#..##.##.G.....##E.......###.###
###G..##.......###.###...##...##
#####.E##.E.G..######...E.#..###
####...###..#..#######.......###
####...###############.#########
#####..#########################
################################
",
            );

            //             // Payten's input
            //             let mut world = World::from_str(
            //                 "
            // ################################
            // ########.#######################
            // #######..#######################
            // ######..########################
            // ###....####...##################
            // ###.#..####G..##################
            // ###G#.G#####..####G#############
            // ##....G..###.......#############
            // #G#####...#..G.....#############
            // #G.###..#..G........############
            // #..G.G..........G.....#.G.######
            // ###......GG..G............######
            // #######....G..#####.G...#.######
            // #######......#######....########
            // #######.....#########..........#
            // #######.....#########.........##
            // #######...#.#########.........##
            // #######.....#########........###
            // #######.....#########.........##
            // #######....E.#######........#..#
            // #######.......#####E........####
            // ###.#.E..#.....G.........#..####
            // ###......#E......E..G...E...####
            // ##...........#.............#####
            // #####.###..............E...#####
            // #############..............#####
            // #############..E.....###...#####
            // ###############..E...###...#####
            // #################.E#.####..#####
            // #################..#.###########
            // #################..#.###########
            // ################################
            // ",
            //             );

            let mut round = 1;

            loop {
                let completed = world.next_round(round, elf_attack_points as i64);

                if completed {
                    round -= 1;
                    break;
                } else {
                    round += 1
                }
            }

            println!("Combat complete after {} round(s)", round);
            println!("Hitpoints remaining: {}", world.remaining_hp());

            println!("Outcome: {}", world.remaining_hp() * round as usize);
            break;
        }
    }

    pub fn part2() {
        for elf_attack_points in 4..1000 {
            let mut world = World::from_str(
                "
################################
#########.######################
#########..#####################
#########..G####################
########....#GG#################
########G......#################
########........################
###.####...#....################
#....###.###...G.###############
##......####.....#.#G..#.#######
###G.G...###.........#...#######
###......##...........##########
#............G#####...##########
#..G##G......#######..##########
#.G.#.......#########..#########
####..G.....#########...#.######
#...........#########..........#
##.#.....#..#########.E.....E..#
##.###..G.G.#########..........#
##...........#######E.#.......##
#.............#####..........###
#....#.....E................####
##.............##.E...........##
#....G.G.................###..##
#..............#.....E...###..##
#..##.##.G.....##E.......###.###
###G..##.......###.###...##...##
#####.E##.E.G..######...E.#..###
####...###..#..#######.......###
####...###############.#########
#####..#########################
################################
",
            );

            let mut round = 1;

            loop {
                let completed = world.next_round(round, elf_attack_points as i64);

                if completed {
                    round -= 1;
                    break;
                } else {
                    round += 1
                }
            }

            if world.all_elves_survived() {
                println!("Minimum damage needed: {}", elf_attack_points);
                println!("Combat complete after {} round(s)", round);
                println!("Hitpoints remaining: {}", world.remaining_hp());

                println!("Outcome: {}", world.remaining_hp() * round as usize);
                break;
            }
        }
    }
}

mod day16 {
    use crate::santasm::*;
    use crate::shared::*;

    fn parse_state(s: &str) -> Registers {
        let numbers = s.split("[").nth(1).unwrap().split("]").nth(0).unwrap();

        numbers.split(", ").map(|n| n.parse().unwrap()).collect()
    }

    pub fn part1() {
        let operations: Vec<&Operation> = vec![
            &OpAddr, &OpAddi, &OpMulr, &OpMuli, &OpBanr, &OpBani, &OpBorr, &OpBori, &OpSetr,
            &OpSeti, &OpGtir, &OpGtri, &OpGtrr, &OpEqir, &OpEqri, &OpEqrr,
        ];

        let input: Vec<String> = input_lines("input_files/day16.txt").collect();
        let mut result = 0;

        let mut i = 0;
        while i < input.len() {
            if !input[i].starts_with("Before:") {
                break;
            }

            let test_before = parse_state(&input[i]);
            let operands: Vec<usize> = input[i + 1]
                .split(" ")
                .map(|n| n.parse().unwrap())
                .collect();
            let test_after = parse_state(&input[i + 2]);

            let mut hit_count = 0;
            for operation in &operations {
                let mut regs = test_before.clone();
                operation.invoke(&mut regs, operands[1], operands[2], operands[3]);
                if regs == test_after {
                    hit_count += 1;
                }
            }

            if hit_count >= 3 {
                result += 1;
            }

            i += 4
        }

        println!("Result: {}", result);
    }

    pub fn part2() {
        let operations: Vec<&Operation> = vec![
            &OpAddr, &OpAddi, &OpMulr, &OpMuli, &OpBanr, &OpBani, &OpBorr, &OpBori, &OpSetr,
            &OpSeti, &OpGtir, &OpGtri, &OpGtrr, &OpEqir, &OpEqri, &OpEqrr,
        ];

        let mut possible_mappings: Vec<HashSet<usize>> = Vec::new();

        // Initially, everything's on the table
        for _ in 0..operations.len() {
            let mut set = HashSet::new();

            for i in 0..operations.len() {
                set.insert(i);
            }

            possible_mappings.push(set);
        }

        let input: Vec<String> = input_lines("input_files/day16.txt").collect();

        let mut i = 0;
        while i < input.len() {
            if !input[i].starts_with("Before:") {
                break;
            }

            let test_before = parse_state(&input[i]);
            let operands: Vec<usize> = input[i + 1]
                .split(" ")
                .map(|n| n.parse().unwrap())
                .collect();
            let test_after = parse_state(&input[i + 2]);

            for op_idx in 0..operations.len() {
                let operation = operations[op_idx];
                let mut regs = test_before.clone();
                operation.invoke(&mut regs, operands[1], operands[2], operands[3]);
                if regs != test_after {
                    // This operation can't be operands[0]
                    possible_mappings[op_idx].remove(&operands[0]);
                }
            }

            i += 4
        }

        // Compact our set of possible mappings as much as possible: where a
        // given operation only has one possible assignment, that's the
        // assignment.
        let mut final_mappings = vec![None; operations.len()];
        let mut assigned_mappings: HashSet<usize> = HashSet::new();

        loop {
            let mut progressed = false;
            for op_idx in 0..operations.len() {
                if final_mappings[op_idx].is_none() {
                    if possible_mappings[op_idx]
                        .difference(&assigned_mappings)
                        .count()
                        == 1
                    {
                        let mapping = possible_mappings[op_idx]
                            .difference(&assigned_mappings)
                            .nth(0)
                            .unwrap()
                            .clone();
                        final_mappings[op_idx] = Some(mapping);
                        assigned_mappings.insert(mapping);
                        progressed = true;
                    }
                }
            }

            if !progressed {
                panic!("Splono");
            }

            if assigned_mappings.len() == operations.len() {
                break;
            }
        }

        println!("CHICKEN DINNER: {:?}", final_mappings);

        let mut opcode_to_operation = vec![0; final_mappings.len()];
        for op_idx in 0..final_mappings.len() {
            let opcode = final_mappings[op_idx].unwrap();
            opcode_to_operation[opcode] = op_idx;
        }

        // Finally, execute our test program
        let mut regs = vec![0, 0, 0, 0];
        for instruction in input_lines("input_files/day16_test_program.txt") {
            let args: Vec<usize> = instruction.split(" ").map(|s| s.parse().unwrap()).collect();

            let operation_idx = opcode_to_operation[args[0]];
            let operation = operations[operation_idx];

            operation.invoke(&mut regs, args[1], args[2], args[3]);
        }

        println!("Result: {:?}", regs);
    }
}

mod day17 {
    use crate::shared::*;

    #[derive(Default, Debug)]
    struct ClayReading {
        x_start: usize,
        x_end: usize,

        y_start: usize,
        y_end: usize,
    }

    lazy_static! {
        static ref CLAY_READING_REGEX: Regex = { Regex::new(r"(x|y)=(.+), (x|y)=(.+)").unwrap() };
    }

    fn parse_range(s: &str) -> (usize, usize) {
        let bits: Vec<&str> = s.split("..").collect();

        if bits.len() == 1 {
            (bits[0].parse().unwrap(), bits[0].parse().unwrap())
        } else {
            (bits[0].parse().unwrap(), bits[1].parse().unwrap())
        }
    }

    impl ClayReading {
        pub fn parse(line: &str) -> ClayReading {
            let mut result = ClayReading {
                ..Default::default()
            };

            let cap = CLAY_READING_REGEX.captures(line).unwrap();
            for &pair in &[1, 3] {
                let (start, end) = parse_range(&cap[pair + 1]);

                if &cap[pair] == "x" {
                    result.x_start = start;
                    result.x_end = end;
                } else {
                    result.y_start = start;
                    result.y_end = end;
                }
            }

            result
        }
    }

    #[derive(Debug)]
    struct World {
        grid: Vec<Vec<Cell>>,
        width: usize,
        height: usize,
        x_min: usize,
        y_min: usize,
    }

    #[derive(PartialEq, Eq, Clone, Debug)]
    enum Cell {
        Sand,
        Clay,
        Void,
        Wet,
        Float,
    }

    impl World {
        fn xval(&self, world_idx: usize) -> usize {
            if world_idx >= self.x_min {
                world_idx - self.x_min
            } else {
                panic!(
                    "xval too small for min: val: {} min: {}",
                    world_idx, self.x_min
                );
            }
        }

        fn yval(&self, world_idx: usize) -> usize {
            if world_idx >= self.y_min {
                world_idx - self.y_min
            } else {
                panic!(
                    "yval too small for min: val: {} min: {}",
                    world_idx, self.y_min
                );
            }
        }

        fn from_readings(readings: &Vec<ClayReading>) -> World {
            let mut x_min = std::usize::MAX;
            let mut x_max = 0;
            let mut y_min = std::usize::MAX;
            let mut y_max = 0;

            // Determine the world boundaries
            for reading in readings {
                x_min = cmp::min(x_min, reading.x_start);
                x_max = cmp::max(x_max, reading.x_end);
                y_min = cmp::min(y_min, reading.y_start);
                y_max = cmp::max(y_max, reading.y_end);
            }

            let height = y_max - y_min + 1;

            // ANY X IS VALID.  So add some padding!
            let width = x_max - x_min + 50;

            let mut world = World {
                x_min: x_min,
                y_min: y_min,
                height: height,
                width: width,
                grid: (0..height).map(|_| vec![Cell::Sand; width]).collect(),
            };

            // Populate our clay areas
            for reading in readings {
                for y in reading.y_start..=reading.y_end {
                    for x in reading.x_start..=reading.x_end {
                        let x_idx = world.xval(x);
                        let y_idx = world.yval(y);
                        world.grid[y_idx][x_idx] = Cell::Clay;
                    }
                }
            }

            world
        }

        pub fn get(&self, x: usize, y: usize) -> &Cell {
            if x < self.width && y < self.height {
                &self.grid[y][x]
            } else {
                &Cell::Void
            }
        }

        pub fn count_wet_cells(&self) -> usize {
            self.grid
                .iter()
                .map(|row| {
                    row.iter()
                        .filter(|&c| c == &Cell::Wet || c == &Cell::Float)
                        .count()
                })
                .sum()
        }

        pub fn count_float_cells(&self) -> usize {
            self.grid
                .iter()
                .map(|row| row.iter().filter(|&c| c == &Cell::Float).count())
                .sum()
        }

        pub fn wet_cell(&mut self, x: usize, y: usize) {
            self.grid[y][x] = Cell::Wet;
            // println!("{}", self.to_string());
        }
        pub fn float_cell(&mut self, x: usize, y: usize) {
            self.grid[y][x] = Cell::Float;
            // println!("{}", self.to_string());
        }

        pub fn dry_cell(&mut self, x: usize, y: usize) {
            self.grid[y][x] = Cell::Sand;
            // println!("{}", self.to_string());
        }

        pub fn to_string(&self) -> String {
            let mut result = String::new();
            let mut idx = self.y_min;

            for row in &self.grid {
                result.push_str(&format!("{:5}", idx));

                for cell in row {
                    result.push(match cell {
                        Cell::Sand => '.',
                        Cell::Clay => '#',
                        Cell::Void => 'X',
                        Cell::Wet => '~',
                        Cell::Float => 'F',
                    })
                }
                result.push_str("\n");

                idx += 1;
            }

            result
        }
    }

    #[derive(Clone, Debug)]
    struct Drip {
        x: usize,
        y: usize,
    }

    fn offset(idx: usize, offset: i64) -> usize {
        (idx as i64 + offset) as usize
    }

    pub fn part1_and_2() {
        let clay_readings: Vec<ClayReading> = input_lines("input_files/day17.txt")
            .map(|line| ClayReading::parse(&line))
            .collect();

        let mut world = World::from_readings(&clay_readings);

        let mut drips_to_check = vec![Drip {
            x: world.xval(500),
            y: world.yval(world.y_min),
        }];

        let mut _iteration = 0;

        while !drips_to_check.is_empty() {
            _iteration += 1;

            let mut drip = drips_to_check.remove(0);

            while world.get(drip.x, drip.y) == &Cell::Float {
                drip.y -= 1;
            }

            world.wet_cell(drip.x, drip.y);

            // The drip moves down while there's sand below.
            while world.get(drip.x, drip.y + 1) == &Cell::Sand {
                drip.y += 1;
                world.wet_cell(drip.x, drip.y);
            }

            let mut edges_hit = 0;
            let mut xvals = Vec::new();

            // Then we scan left and right until we either hit clay, or fall off something
            for &off in &[-1, 1] {
                let mut drip = drip.clone();

                loop {
                    if (world.get(offset(drip.x, off), drip.y) == &Cell::Sand
                        || world.get(offset(drip.x, off), drip.y) == &Cell::Wet)
                        && (world.get(offset(drip.x, off), drip.y + 1) == &Cell::Clay
                            || world.get(offset(drip.x, off), drip.y + 1) == &Cell::Float)
                    {
                        // Sand to the left and clay below means we're on the bottom
                        // level and can move.
                        drip.x = offset(drip.x, off);
                        world.wet_cell(drip.x, drip.y);
                    } else {
                        if world.get(offset(drip.x, off), drip.y) == &Cell::Clay {
                            // We've hit an edge
                            xvals.push(drip.x);
                            edges_hit += 1;
                        } else if world.get(drip.x, drip.y + 1) == &Cell::Wet {
                            // Fall straight down.  No need for further checks.
                        } else if world.get(offset(drip.x, off), drip.y + 1) == &Cell::Sand {
                            // We've fallen off something!
                            drips_to_check.push(Drip {
                                x: offset(drip.x, off),
                                y: drip.y,
                            });
                        }

                        break;
                    }
                }
            }

            if edges_hit == 2 {
                for x in xvals[0]..=xvals[1] {
                    world.float_cell(x, drip.y);
                }

                // If we hit an edge in each direction, we are in a space that
                // can fill with water.
                //
                // Explore one row up
                drips_to_check.push(Drip {
                    x: drip.x,
                    y: drip.y - 1,
                });
            }
        }

        // Our original drip is really the spring, so subtract one to adjust.
        println!("{}", world.to_string());
        println!("Explored {} cells", world.count_wet_cells());
        println!("Retained {} cells", world.count_float_cells());
    }
}

mod day18 {
    use crate::shared::*;

    fn neighbours_of(world: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<char> {
        vec![
            world[y + 1][x + 1],
            world[y - 1][x + 1],
            world[y][x + 1],
            world[y + 1][x - 1],
            world[y - 1][x - 1],
            world[y][x - 1],
            world[y + 1][x],
            world[y - 1][x],
        ]
    }

    fn count_of(neighbours: &Vec<char>, square: char) -> usize {
        neighbours.iter().filter(|&&c| c == square).count()
    }

    pub fn part1() {
        let mut world: Vec<Vec<char>> = input_lines("input_files/day18.txt")
            .map(|row| row.chars().collect())
            .collect();
        let height = world.len();
        let width = world[0].len();

        // Add a border so we don't have to think too hard
        world.insert(0, vec!['!'; width]);
        world.push(vec!['!'; width]);

        for row in world.iter_mut() {
            row.insert(0, '!');
            row.push('!');
        }

        println!("{}", format_grid(&world));

        for _minute in 1..=10 {
            let mut new_world = world.clone();

            for y in 1..=height {
                for x in 1..=width {
                    let neighbours = neighbours_of(&world, x, y);

                    if world[y][x] == '.' && count_of(&neighbours, '|') >= 3 {
                        new_world[y][x] = '|';
                    } else if world[y][x] == '|' && count_of(&neighbours, '#') >= 3 {
                        new_world[y][x] = '#';
                    } else if world[y][x] == '#' {
                        if count_of(&neighbours, '#') >= 1 && count_of(&neighbours, '|') >= 1 {
                            new_world[y][x] = '#';
                        } else {
                            new_world[y][x] = '.';
                        }
                    } else {
                        new_world[y][x] = world[y][x];
                    }
                }
            }

            world = new_world;
            println!("{}", format_grid(&world));
        }

        println!(
            "{}",
            count_of(&world.iter().flatten().cloned().collect(), '|')
                * count_of(&world.iter().flatten().cloned().collect(), '#')
        )
    }

    pub fn part2() {
        let mut world: Vec<Vec<char>> = input_lines("input_files/day18.txt")
            .map(|row| row.chars().collect())
            .collect();
        let height = world.len();
        let width = world[0].len();

        // Add a border so we don't have to think too hard
        world.insert(0, vec!['!'; width]);
        world.push(vec!['!'; width]);

        for row in world.iter_mut() {
            row.insert(0, '!');
            row.push('!');
        }

        println!("{}", format_grid(&world));

        for minute in 1..=15000 {
            let mut new_world = world.clone();

            for y in 1..=height {
                for x in 1..=width {
                    let neighbours = neighbours_of(&world, x, y);

                    if world[y][x] == '.' && count_of(&neighbours, '|') >= 3 {
                        new_world[y][x] = '|';
                    } else if world[y][x] == '|' && count_of(&neighbours, '#') >= 3 {
                        new_world[y][x] = '#';
                    } else if world[y][x] == '#' {
                        if count_of(&neighbours, '#') >= 1 && count_of(&neighbours, '|') >= 1 {
                            new_world[y][x] = '#';
                        } else {
                            new_world[y][x] = '.';
                        }
                    } else {
                        new_world[y][x] = world[y][x];
                    }
                }
            }

            world = new_world;

            // Looking for cycles...
            // Right.  We're cycling every 28 minutes, so 1000000000 will have the same number as 11976 (since (1000000000 - 11976) mod 28.0 == 0)
            if minute > 10000 {
                println!(
                    "{} {}",
                    minute,
                    count_of(&world.iter().flatten().cloned().collect(), '|')
                        * count_of(&world.iter().flatten().cloned().collect(), '#')
                )
            }
        }
    }
}

mod day19 {
    use crate::santasm::*;
    use crate::shared::*;

    pub fn part1() {
        let instructions = instruction_set();

        let input = input_lines("input_files/day19.txt");

        let mut ip_bound_register = 0;
        let mut program: Vec<(String, usize, usize, usize)> = Vec::new();

        for line in input {
            let bits: Vec<&str> = line.split(" ").collect();
            if bits.len() == 1 {
                // empty
                continue;
            } else if bits.len() == 2 {
                assert_eq!(bits[0], "#ip");
                ip_bound_register = bits[1].parse().unwrap();
            } else {
                program.push((
                    bits[0].to_owned(),
                    bits[1].parse().unwrap(),
                    bits[2].parse().unwrap(),
                    bits[3].parse().unwrap(),
                ));
            }
        }

        let mut ip: usize = 0;
        let mut registers: Registers = vec![0; 6];

        loop {
            if ip >= program.len() {
                // Invalid.  Halt.
                break;
            }

            registers[ip_bound_register] = ip;

            let (op, a, b, c) = program[ip].clone();

            instructions
                .get::<str>(&op)
                .unwrap()
                .invoke(&mut registers, a, b, c);

            ip = registers[ip_bound_register];
            ip += 1;
        }

        println!("{:?}", registers);
    }

    pub fn print_program() {
        let instructions = instruction_set();

        let input = input_lines("input_files/day19.txt");

        let mut program: Vec<(String, usize, usize, usize)> = Vec::new();

        for line in input {
            let bits: Vec<&str> = line.split(" ").collect();
            if bits.len() == 1 {
                // empty
                continue;
            } else if bits.len() == 2 {
                assert_eq!(bits[0], "#ip");
            } else {
                program.push((
                    bits[0].to_owned(),
                    bits[1].parse().unwrap(),
                    bits[2].parse().unwrap(),
                    bits[3].parse().unwrap(),
                ));
            }
        }

        for (op, a, b, c) in program {
            instructions.get::<str>(&op).unwrap().pretty(a, b, c);
        }
    }

    pub fn part2() {
        // Strategy here was:
        //
        //  Print the program using the above
        //
        //  Port the program to basic C using gotos instead of IP manipulations
        //
        //  Refactor C code to use structured programming stuff -- ifs, while loops, etc.
        //
        // Eventually figured out that it was basically just doing this:
        //
        // total = 0
        // (1..10551347).each do |divisor|
        //   if 10551347 % divisor == 0
        //     total += divisor
        //   end
        // end
        //
        // total # => 10695960
        //
        // Full gore in day19_ported.c
        //
    }
}

mod day20 {
    use crate::shared::*;
    use std::iter::Peekable;

    #[derive(Debug)]
    enum SimpleRegex {
        Literal(Vec<char>),
        Disjunction(Vec<SimpleRegex>),
        Conjunction(Vec<SimpleRegex>),
    }

    // Produce a list of matching inputs from a simple regex
    fn parse_regex(regex: &str) -> SimpleRegex {
        let mut input = regex.chars().peekable();

        assert_eq!('^', input.next().unwrap());

        let mut result = Vec::new();
        while *input.peek().unwrap() != '$' {
            let next_regex = read_next_regex(&mut input);
            result.push(next_regex);
        }

        SimpleRegex::Conjunction(result)
    }

    fn is_literal(ch: char) -> bool {
        ch == 'N' || ch == 'E' || ch == 'W' || ch == 'S'
    }

    fn read_next_regex(input: &mut Peekable<impl Iterator<Item = char>>) -> SimpleRegex {
        let &ch = input.peek().unwrap();

        if ch == '(' {
            // Eat the opener
            input.next().unwrap();

            let mut subexpressions = Vec::new();

            let mut conjunctions = Vec::new();
            loop {
                let subexpr = read_next_regex(input);
                conjunctions.push(subexpr);

                let &nextch = input.peek().unwrap();

                if nextch == ')' || nextch == '|' {
                    input.next().unwrap();
                    // We've read a full disjunction
                    if conjunctions.len() == 1 {
                        // Only one bit, so just add it directly
                        subexpressions.push(conjunctions.remove(0));
                    } else {
                        subexpressions.push(SimpleRegex::Conjunction(conjunctions));
                        conjunctions = Vec::new();
                    }

                    if nextch == ')' {
                        break;
                    }
                } else {
                    // More of the current expression to read...
                }
            }

            SimpleRegex::Disjunction(subexpressions)
        } else {
            // zero or more literals
            let mut chars = Vec::new();
            while is_literal(*input.peek().unwrap()) {
                chars.push(input.next().unwrap());
            }

            SimpleRegex::Literal(chars)
        }
    }

    fn explore(
        regex: &SimpleRegex,
        locations_to_explore: Vec<Location>,
        min_costs: &mut HashMap<(i64, i64), usize>,
    ) -> Vec<Location> {
        let mut new_locations = match regex {
            SimpleRegex::Conjunction(parts) => {
                let mut result = locations_to_explore.clone();
                for part in parts {
                    result = explore(part, result, min_costs);
                }

                result
            }
            SimpleRegex::Literal(chars) => {
                let mut result = locations_to_explore.clone();
                for &ch in chars {
                    for mut location in result.iter_mut() {
                        match ch {
                            'N' => location.y -= 1,
                            'E' => location.x += 1,
                            'S' => location.y += 1,
                            'W' => location.x -= 1,
                            _ => unreachable!(),
                        }

                        location.cost += 1;

                        // Gross duplication
                        let entry = min_costs
                            .entry((location.x, location.y))
                            .or_insert(location.cost);
                        if location.cost < *entry {
                            *entry = location.cost;
                        }
                    }
                }

                result
            }
            SimpleRegex::Disjunction(parts) => {
                let mut result = Vec::new();
                for part in parts {
                    let mut part_result = explore(part, locations_to_explore.clone(), min_costs);
                    result.append(&mut part_result);
                }

                result
            }
        };

        for location in new_locations.iter_mut() {
            let entry = min_costs
                .entry((location.x, location.y))
                .or_insert(location.cost);

            // If we've got a better path the this location, record it!
            if location.cost < *entry {
                *entry = location.cost;
            }
        }

        new_locations.sort();
        new_locations.dedup();

        new_locations
    }

    // # #
    // ###
    //  X

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Location {
        x: i64,
        y: i64,
        cost: usize,
    }

    impl Ord for Location {
        fn cmp(&self, other: &Location) -> Ordering {
            (self.x, self.y, self.cost).cmp(&(other.x, other.y, other.cost))
        }
    }

    impl PartialOrd for Location {
        fn partial_cmp(&self, other: &Location) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    pub fn part1() {
        let input_s = include_str!("../input_files/day20.txt").trim().to_owned();
        let regex = parse_regex(&input_s);

        let mut min_costs: HashMap<(i64, i64), usize> = HashMap::new();

        explore(
            &regex,
            vec![Location {
                x: 0,
                y: 0,
                cost: 0,
            }],
            &mut min_costs,
        );

        println!("{}", min_costs.values().max().unwrap());
    }

    pub fn part2() {
        let input_s = include_str!("../input_files/day20.txt").trim().to_owned();
        let regex = parse_regex(&input_s);

        let mut min_costs: HashMap<(i64, i64), usize> = HashMap::new();

        explore(
            &regex,
            vec![Location {
                x: 0,
                y: 0,
                cost: 0,
            }],
            &mut min_costs,
        );

        println!("{}", min_costs.values().filter(|&&v| v >= 1000).count());
    }

}

mod day21 {
    use crate::santasm::*;
    use crate::shared::*;

    // Part 1: Same trick as last time: pretty print the code, rewrite into structured C.
    // r0 = 212115 got us out of the loop as quickly as possible;

    pub fn part1() {
        let instructions = instruction_set();

        let input = input_lines("input_files/day21.txt");

        let mut program: Vec<(String, usize, usize, usize)> = Vec::new();

        for line in input {
            let bits: Vec<&str> = line.split(" ").collect();
            if bits.len() == 1 {
                // empty
                continue;
            } else if bits.len() == 2 {
                assert_eq!(bits[0], "#ip");
            } else {
                program.push((
                    bits[0].to_owned(),
                    bits[1].parse().unwrap(),
                    bits[2].parse().unwrap(),
                    bits[3].parse().unwrap(),
                ));
            }
        }

        for (op, a, b, c) in program {
            instructions.get::<str>(&op).unwrap().pretty(a, b, c);
        }
    }

    pub fn part2() {
        // I figured the value of r3 must cycle at some point, so I used a
        // binary search and some shell crap to find the cycle length.  The
        // first cycle: ./day21_ported | head -11546 | cut -d'=' -f2 | sort | uniq -d
        //
        // So I guess we want the last number in the cycle to maximise our run...
        //
        //
        // ./day21_ported | head -11545 | cut -d'=' -f2 | tail -1
        // 9258470
    }
}

mod day22 {
    use crate::shared::*;

    #[derive(Clone, Hash, Eq, PartialEq, Debug)]
    enum TerrainType {
        Rocky,
        Narrow,
        Wet,
    }

    fn calculate_terrain(
        depth: usize,
        target_x: usize,
        target_y: usize,
        width: usize,
        height: usize,
    ) -> Vec<Vec<TerrainType>> {
        let mut terrain: Vec<Vec<TerrainType>> = (0..height)
            .map(|_| vec![TerrainType::Rocky; width])
            .collect();

        let mut erosion_levels: Vec<Vec<usize>> = (0..height).map(|_| vec![0; width]).collect();
        let mut geologic_indexes: Vec<Vec<usize>> = (0..height).map(|_| vec![0; width]).collect();

        // Fill out the bits we know
        for y in 0..height {
            for x in 0..width {
                if y == 0 {
                    geologic_indexes[y][x] = x * 16807;
                    erosion_levels[y][x] = (geologic_indexes[y][x] + depth) % 20183;
                }

                if x == 0 {
                    geologic_indexes[y][x] = y * 48271;
                    erosion_levels[y][x] = (geologic_indexes[y][x] + depth) % 20183;
                }
            }
        }

        // Derive the bits we don't...
        for y in 1..height {
            for x in 1..width {
                let geologic_index = if x == target_x && y == target_y {
                    0
                } else {
                    erosion_levels[y - 1][x] * erosion_levels[y][x - 1]
                };

                geologic_indexes[y][x] = geologic_index;
                erosion_levels[y][x] = (geologic_index + depth) % 20183;
            }
        }

        // Now we can determine our terrain types
        for y in 0..height {
            for x in 0..width {
                terrain[y][x] = match erosion_levels[y][x] % 3 {
                    0 => TerrainType::Rocky,
                    1 => TerrainType::Wet,
                    2 => TerrainType::Narrow,
                    _ => unreachable!(),
                }
            }
        }

        terrain
    }

    pub fn part1() {
        let depth = 4080;
        let (target_x, target_y) = (14, 785);

        let terrain = calculate_terrain(depth, target_x, target_y, target_x + 1, target_y + 1);

        let mut total_risk = 0;
        for y in 0..terrain.len() {
            for x in 0..terrain[0].len() {
                total_risk += match terrain[y][x] {
                    TerrainType::Rocky => 0,
                    TerrainType::Wet => 1,
                    TerrainType::Narrow => 2,
                };
            }
        }

        println!("Total risk: {}", total_risk);
    }

    #[derive(Clone, Hash, Eq, PartialEq, Debug)]
    enum Equipment {
        ClimbingGear,
        Torch,
        Fists,
    }

    impl fmt::Display for TerrainType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let s = match self {
                TerrainType::Rocky => ".",
                TerrainType::Wet => "=",
                TerrainType::Narrow => "|",
            };

            write!(f, "{}", s)
        }
    }

    #[derive(Debug, Hash, Eq, PartialEq, Clone)]
    struct PathPosition {
        equipped: Equipment,
        x: usize,
        y: usize,
    }

    #[derive(Debug)]
    struct Path {
        pos: PathPosition,
        cost: usize,
    }

    struct BorderedTerrain {
        terrain: Vec<Vec<TerrainType>>,
        width: usize,
        height: usize,
    }

    impl BorderedTerrain {
        pub fn new(
            depth: usize,
            target_x: usize,
            target_y: usize,
            width: usize,
            height: usize,
        ) -> BorderedTerrain {
            let terrain = calculate_terrain(depth, target_x, target_y, width, height);

            BorderedTerrain {
                terrain,
                width,
                height,
            }
        }

        pub fn possible_paths(&self, current: &Path) -> Vec<Path> {
            let mut result = Vec::new();

            for &yoff in &[-1, 0, 1i64] {
                for &xoff in &[-1, 0, 1i64] {
                    if (yoff + xoff).abs() != 1 {
                        // No diagonals or [0][0]
                        continue;
                    }

                    let mut new_x = current.pos.x as i64;
                    let mut new_y = current.pos.y as i64;

                    new_x += xoff;
                    new_y += yoff;

                    if (new_x >= 0 && new_x < self.width as i64)
                        && (new_y >= 0 && new_y < self.height as i64)
                    {
                        // The position is in bounds...
                        let current_terrain =
                            &self.terrain[current.pos.y as usize][current.pos.x as usize];
                        let target_terrain = &self.terrain[new_y as usize][new_x as usize];

                        if target_terrain == current_terrain {
                            // Terrain types are compatible.  An easy move.
                            // THINKME: Is there any advantage to exploring the "switch equipment" case here?  Seems like not...
                            result.push(Path {
                                pos: PathPosition {
                                    x: new_x as usize,
                                    y: new_y as usize,
                                    equipped: current.pos.equipped.clone(),
                                },
                                cost: current.cost + 1,
                            });
                        } else {
                            // If the terrain type changed, we may need to switch equipment.
                            // rocky -> wet
                            //   if we have the climbing gear -> 1
                            //   if we have the torch -> switch to climbing gear -> 8

                            // rocky -> narrow
                            //   if we have the climbing gear -> switch to torch -> 8
                            //   if we have the torch -> 1

                            // wet -> rocky
                            //   if we have the climbing gear -> 1
                            //   if we have nothing -> switch to climbing gear -> 8

                            // wet -> narrow
                            //   if we have the climbing gear -> neither -> 8
                            //   if we have nothing -> 1

                            // narrow -> rocky
                            //   if we have nothing -> switch to torch -> 8
                            //   if we have torch -> 1

                            // narrow -> wet
                            //   if we have nothing -> 1
                            //   if we have torch -> switch to nothing -> 1

                            let next_path = match &(current_terrain, target_terrain) {
                                &(TerrainType::Rocky, TerrainType::Wet) => Path {
                                    pos: PathPosition {
                                        x: new_x as usize,
                                        y: new_y as usize,
                                        equipped: Equipment::ClimbingGear,
                                    },
                                    cost: current.cost
                                        + if current.pos.equipped == Equipment::ClimbingGear {
                                            1
                                        } else {
                                            8
                                        },
                                },

                                &(TerrainType::Rocky, TerrainType::Narrow) => Path {
                                    pos: PathPosition {
                                        x: new_x as usize,
                                        y: new_y as usize,
                                        equipped: Equipment::Torch,
                                    },
                                    cost: current.cost
                                        + if current.pos.equipped == Equipment::Torch {
                                            1
                                        } else {
                                            8
                                        },
                                },

                                &(TerrainType::Wet, TerrainType::Rocky) => Path {
                                    pos: PathPosition {
                                        x: new_x as usize,
                                        y: new_y as usize,
                                        equipped: Equipment::ClimbingGear,
                                    },
                                    cost: current.cost
                                        + if current.pos.equipped == Equipment::ClimbingGear {
                                            1
                                        } else {
                                            8
                                        },
                                },

                                &(TerrainType::Wet, TerrainType::Narrow) => Path {
                                    pos: PathPosition {
                                        x: new_x as usize,
                                        y: new_y as usize,
                                        equipped: Equipment::Fists,
                                    },
                                    cost: current.cost
                                        + if current.pos.equipped == Equipment::Fists {
                                            1
                                        } else {
                                            8
                                        },
                                },

                                &(TerrainType::Narrow, TerrainType::Rocky) => Path {
                                    pos: PathPosition {
                                        x: new_x as usize,
                                        y: new_y as usize,
                                        equipped: Equipment::Torch,
                                    },
                                    cost: current.cost
                                        + if current.pos.equipped == Equipment::Torch {
                                            1
                                        } else {
                                            8
                                        },
                                },

                                &(TerrainType::Narrow, TerrainType::Wet) => Path {
                                    pos: PathPosition {
                                        x: new_x as usize,
                                        y: new_y as usize,
                                        equipped: Equipment::Fists,
                                    },
                                    cost: current.cost
                                        + if current.pos.equipped == Equipment::Fists {
                                            1
                                        } else {
                                            8
                                        },
                                },
                                _ => unreachable!(),
                            };

                            result.push(next_path);
                        }
                    }
                }
            }

            result
        }

        pub fn to_string(&self) -> String {
            format_grid(&self.terrain)
        }
    }

    pub fn part2() {
        // let depth = 510;
        // let (target_x, target_y) = (10, 10);

        let depth = 4080;
        let (target_x, target_y) = (14, 785);

        let terrain = BorderedTerrain::new(depth, target_x, target_y, target_x + 50, target_y + 50);

        println!("{}", terrain.to_string());

        let mut paths = vec![Path {
            pos: PathPosition {
                equipped: Equipment::Torch,
                x: 0,
                y: 0,
            },
            cost: 0,
        }];

        let mut successful_paths = Vec::new();
        let mut best_costs: HashMap<PathPosition, usize> = HashMap::new();

        while !paths.is_empty() {
            // Each iteration will process all active paths, culling where
            // appropriate, and adding them as new_paths for the next round.
            //
            // At the end of a round, if any of our new paths have hit the
            // target, we'll choose the best one.  NOTE: we will need to add 7
            // if we hit the target with the wrong type equipped.  Perhaps this
            // means we need to keep iterating until all of the paths in play
            // have a cost > than what we found.

            let mut next_paths = Vec::new();

            while !paths.is_empty() {
                // Generate our set of possible moves.  This will be a move in
                // an adjacent direction (cost 1) plus switching cost if needed.
                //
                // If a move would take us to a square already visited
                // previously with the same equipped item and higher cost, we
                // can cull it.  THINKME: This strategy probably isn't as
                // aggressive as it could be, but we'll see if something
                // cleverer is needed...

                let path = paths.remove(0);

                for next_path in terrain.possible_paths(&path) {
                    if next_path.pos.x == target_x && next_path.pos.y == target_y {
                        // println!("Found the target!");

                        let mut completed = next_path;

                        if completed.pos.equipped != Equipment::Torch {
                            completed.pos.equipped = Equipment::Torch;
                            completed.cost += 7;
                        }
                        successful_paths.push(completed);

                        continue;
                    }

                    let entry = best_costs
                        .entry(next_path.pos.clone())
                        .or_insert(std::usize::MAX);

                    if next_path.cost < *entry {
                        // Worth a look!
                        *entry = next_path.cost;
                        next_paths.push(next_path);
                    } else {
                        // println!("Culled!");
                    }
                }
            }

            if successful_paths.len() > 0
                && (next_paths.iter().map(|path| path.cost).min().unwrap()
                    > successful_paths.iter().map(|path| path.cost).min().unwrap())
            {
                // If the only paths under consideration are more expensive
                // than one we've already found, there's no point continuing
                // the search
                break;
            }

            // If any of our new paths have found our target, record them.  If
            // any path in play has a lower cost than the path we just found,
            // continue.
            paths = next_paths;
        }

        println!(
            "Lowest cost was: {}",
            successful_paths.iter().map(|path| path.cost).min().unwrap()
        );
    }
}

mod day23 {
    use crate::shared::*;

    #[derive(Debug, Clone)]
    struct Nanobot {
        x: i64,
        y: i64,
        z: i64,

        radius: i64,
    }

    impl Nanobot {
        pub fn from_str(s: &str) -> Nanobot {
            let pat = Regex::new(r"pos=<(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)>, r=([0-9]+)").unwrap();

            if let Some(cap) = pat.captures(s) {
                Nanobot {
                    x: cap[1].parse().unwrap(),
                    y: cap[2].parse().unwrap(),
                    z: cap[3].parse().unwrap(),
                    radius: cap[4].parse().unwrap(),
                }
            } else {
                panic!("Failed to parse line: {}", s);
            }
        }

        pub fn in_range(&self, other: &Nanobot) -> bool {
            ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs())
                <= self.radius as i64
        }
    }
    pub fn part1() {
        let nanobots: Vec<Nanobot> = input_lines("input_files/day23.txt")
            .map(|s| Nanobot::from_str(&s))
            .collect();
        let strongest_nanobot = nanobots.iter().max_by_key(|n| n.radius).unwrap();
        println!(
            "Nanobots in range: {}",
            nanobots
                .iter()
                .filter(|&n| strongest_nanobot.in_range(n))
                .count()
        );
    }

    #[derive(Debug, Clone)]
    struct NanobotNorm {
        x: f64,
        y: f64,
        z: f64,

        radius: f64,
    }

    /************************************************************************
         * Scratchings...
        fn manhattan_distance(x1: f64, y1: f64, z1: f64,
        x2: f64, y2: f64, z2: f64) -> f64 {
        ((x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs())
    }

        fn chunked_range(min: i64, max: i64, chunks: usize) -> impl Iterator<Item=(i64, i64)> {
        let mut chunk_size = (max - min) / chunks as i64;

        if chunk_size == 0 {
        chunk_size = 1;
    }

        ((min - chunk_size)..=(max + chunk_size)).step_by(chunk_size as usize).map(move |xmin| (xmin, xmin + chunk_size))
    }

        struct SearchRegion {
        min_x: i64,
        min_y: i64,
        min_z: i64,
        max_x: i64,
        max_y: i64,
        max_z: i64,
    }

        // THINKME: Is this solvable in 2d?  First look for significant overlaps in
        // XY, then in XZ?  If something overlaps in both, it'll overlap in 3d
        // space?
        //
        // Oh... actually this is all in manhattan distance, so they're not really spheres?
        //
        // New plan: calculate the edges of all circles.  I think we can do this like:

        //      #
        //     ###
        //    #####
        //   #######
        //  #########
        // #####o#####
        //  #########
        //   #######
        //    #####
        //     ###
        //      #
        // That's a 2d radius = 5 as manhattan distance.  It seems like we don't
        // need to fill out an entire array to figure out overlaps: just walk the
        // edges of this shape (which should have predictable coordinates) and load
        // them into a hashmap or something?  Keep track of how many overlaps we see
        // in a given grid square?
        //
        // Maybe the key to all of this is just to exploit the fact that the circles
        // are heavily overlapping and use a sparse data structure?  maybe I've been
        // overthinking this...
        //
        // Is there any level we can quantize things to without losing precision I
        // wonder?  What's the smallest pixel distance between two grids?



        // Hypothesis: since we're using Manhattan distance, the radius of each nanobot makes a kind of star shape like this:
        //
        //      #
        //     ###
        //    #####
        //   #######
        //  #########
        // #####o#####
        //  #########
        //   #######
        //    #####
        //     ###
        //      #
        //
        // Whenever you intersect two of these shapes, one of the N/E/S/W points
        // necessarily overlaps, so those are the only places you need to check to
        // find the maximum number of overlaps.

        impl Nanobot {

        pub fn extremes(&self) -> Vec<(i64, i64, i64)> {
        let mut result = Vec::new();

        result.push((self.x + self.radius, 0, 0));
        result.push((self.x - self.radius, 0, 0));

        result.push((0, self.y + self.radius, 0));
        result.push((0, self.y - self.radius, 0));

        result.push((0, 0, self.z + self.radius));
        result.push((0, 0, self.z - self.radius));

        result
    }
    }

        // Maybe it's time to return to the original strategy:

        //   * divide the world into large squares
        //
        //   * A given square overlaps the large square if... any of its edge coordinates is in range
        //
        //   * Count which large square got the most matches and drill down.  If there's multiple, try them all.

        // pub fn part2() {
        //     let nanobots: Vec<Nanobot> = input_lines("input_files/day23.txt").map(|s| Nanobot::from_str(&s)).collect();
        //
        //     let centroid_x: i64 = nanobots.iter().map(|n| n.x).sum::<i64>() / nanobots.len() as i64;
        //     let centroid_y: i64 = nanobots.iter().map(|n| n.y).sum::<i64>() / nanobots.len() as i64;
        //     let centroid_z: i64 = nanobots.iter().map(|n| n.z).sum::<i64>() / nanobots.len() as i64;
        //
        //     let mut best = 0;
        //
        //     // Can we scale worldspace?
        //     let point = Nanobot { x: centroid_x, y: centroid_y, z: centroid_z, radius: 1 };
        //     // Search out from centroid...
        //     for xoff in (-2000000..2000000i64).step_by(10000) {
        //         for yoff in (-2000000..2000000i64).step_by(10000) {
        //             for zoff in (-2000000..2000000i64).step_by(10000) {
        //                 let mut test = point.clone();
        //                 test.x += xoff;
        //                 test.y += yoff;
        //                 test.z += zoff;
        //
        //                 let count = nanobots.iter().filter(|&n| n.in_range(&test)).count();
        //
        //                 if count > best {
        //                     best = count;
        //                     println!("New best: {}", best);
        //                 }
        //             }
        //         }
        //     }
        //
        //
        //     println!("Best was: {}", best);
        // }

        pub fn part2() {
        let nanobots: Vec<Nanobot> = input_lines("input_files/day23.txt").map(|s| Nanobot::from_str(&s)).collect();

        let mut vals: Vec<_> = nanobots.iter().map(|n| ((n.x as f64 / 300_000_000.0) * 200_000_000.0) as i64).collect();
        vals.sort();
        vals.dedup();
        println!("vals: {}", vals.len());
    }
         ************************************************************************/

    #[derive(Debug, Clone)]
    struct Point3D {
        x: i64,
        y: i64,
        z: i64,
        r: i64,
    }

    impl Point3D {
        pub fn distance(&self, other: &Point3D) -> i64 {
            (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
        }
    }

    #[derive(Debug)]
    struct Cube {
        x1: i64,
        y1: i64,
        z1: i64,
        x2: i64,
        y2: i64,
        z2: i64,
    }

    #[derive(Debug)]
    enum PlaneDirection {
        XY,
        XZ,
        YZ,
    }

    #[derive(Debug)]
    struct Plane {
        p1: Point3D,
        p2: Point3D,
        direction: PlaneDirection,
    }

    impl Cube {
        pub fn len(&self) -> i64 {
            ((self.x2 - self.x1) + 1) * ((self.y2 - self.y1) + 1) * ((self.z2 - self.z1) + 1)
        }

        pub fn is_single_position(&self) -> bool {
            self.x1 == self.x2 && self.y1 == self.y2 && self.z1 == self.z2
        }

        pub fn overlaps(&self, p: &Point3D) -> bool {
            if p.x >= self.x1
                && p.y >= self.y1
                && p.z >= self.z1
                && p.x <= self.x2
                && p.y <= self.y2
                && p.z <= self.z2
            {
                return true;
            }

            let planes = self.planes();
            let closest_plane = planes
                .iter()
                .min_by_key(|plane| plane.p1.distance(p).abs() + plane.p2.distance(p).abs())
                .unwrap();

            closest_plane.overlaps(p)
        }

        fn planes(&self) -> Vec<Plane> {
            vec![
                // front
                Plane {
                    p1: Point3D {
                        x: self.x1,
                        y: self.y1,
                        z: self.z1,
                        r: 1,
                    },
                    p2: Point3D {
                        x: self.x2,
                        y: self.y2,
                        z: self.z1,
                        r: 1,
                    },
                    direction: PlaneDirection::XY,
                },
                // top
                Plane {
                    p1: Point3D {
                        x: self.x1,
                        y: self.y1,
                        z: self.z1,
                        r: 1,
                    },
                    p2: Point3D {
                        x: self.x2,
                        y: self.y1,
                        z: self.z2,
                        r: 1,
                    },
                    direction: PlaneDirection::XZ,
                },
                // left
                Plane {
                    p1: Point3D {
                        x: self.x1,
                        y: self.y1,
                        z: self.z1,
                        r: 1,
                    },
                    p2: Point3D {
                        x: self.x1,
                        y: self.y2,
                        z: self.z2,
                        r: 1,
                    },
                    direction: PlaneDirection::YZ,
                },
                // back
                Plane {
                    p1: Point3D {
                        x: self.x1,
                        y: self.y1,
                        z: self.z2,
                        r: 1,
                    },
                    p2: Point3D {
                        x: self.x2,
                        y: self.y2,
                        z: self.z2,
                        r: 1,
                    },
                    direction: PlaneDirection::XY,
                },
                // right
                Plane {
                    p1: Point3D {
                        x: self.x2,
                        y: self.y1,
                        z: self.z1,
                        r: 1,
                    },
                    p2: Point3D {
                        x: self.x2,
                        y: self.y2,
                        z: self.z2,
                        r: 1,
                    },
                    direction: PlaneDirection::YZ,
                },
                // bottom
                Plane {
                    p1: Point3D {
                        x: self.x1,
                        y: self.y2,
                        z: self.z1,
                        r: 1,
                    },
                    p2: Point3D {
                        x: self.x2,
                        y: self.y2,
                        z: self.z2,
                        r: 1,
                    },
                    direction: PlaneDirection::XZ,
                },
            ]
        }
    }

    fn falls_on_plane(p: &Point3D, p1: &Point3D, p2: &Point3D) -> bool {
        assert!(p1.x <= p2.x);
        assert!(p1.y <= p2.y);
        assert!(p1.z <= p2.z);

        between(p.x, p1.x, p2.x) && between(p.y, p1.y, p2.y) && between(p.z, p1.z, p2.z)
    }

    fn between(n: i64, a: i64, b: i64) -> bool {
        if a > b {
            between(n, b, a)
        } else {
            a <= n && n <= b
        }
    }

    impl Plane {
        fn all_points(&self) -> Vec<Point3D> {
            match self.direction {
                PlaneDirection::XY => vec![
                    self.p1.clone(),
                    Point3D {
                        x: self.p2.x,
                        y: self.p1.y,
                        z: self.p1.z,
                        r: 1,
                    },
                    Point3D {
                        x: self.p1.x,
                        y: self.p2.y,
                        z: self.p1.z,
                        r: 1,
                    },
                    self.p2.clone(),
                ],
                PlaneDirection::YZ => vec![
                    self.p1.clone(),
                    Point3D {
                        x: self.p1.x,
                        y: self.p2.y,
                        z: self.p1.z,
                        r: 1,
                    },
                    Point3D {
                        x: self.p1.x,
                        y: self.p1.y,
                        z: self.p2.z,
                        r: 1,
                    },
                    self.p2.clone(),
                ],
                PlaneDirection::XZ => vec![
                    self.p1.clone(),
                    Point3D {
                        x: self.p2.x,
                        y: self.p1.y,
                        z: self.p1.z,
                        r: 1,
                    },
                    Point3D {
                        x: self.p1.x,
                        y: self.p1.y,
                        z: self.p2.z,
                        r: 1,
                    },
                    self.p2.clone(),
                ],
            }
        }

        pub fn overlaps(&self, p: &Point3D) -> bool {
            let all_points = self.all_points();
            let closest_point = all_points
                .iter()
                .min_by_key(|point| point.distance(p))
                .unwrap();

            if closest_point.distance(p) <= p.r {
                return true;
            }

            match self.direction {
                PlaneDirection::XY => {
                    let x_diff = (p.x - closest_point.x);
                    let y_diff = (p.y - closest_point.y);

                    [
                        Point3D {
                            x: closest_point.x + x_diff,
                            y: closest_point.y + y_diff,
                            z: closest_point.z,
                            r: 1,
                        },
                        Point3D {
                            x: closest_point.x + x_diff,
                            y: closest_point.y - y_diff,
                            z: closest_point.z,
                            r: 1,
                        },
                        Point3D {
                            x: closest_point.x - x_diff,
                            y: closest_point.y + y_diff,
                            z: closest_point.z,
                            r: 1,
                        },
                        Point3D {
                            x: closest_point.x - x_diff,
                            y: closest_point.y - y_diff,
                            z: closest_point.z,
                            r: 1,
                        },
                    ]
                    .iter()
                    .any(|candidate_point| {
                        falls_on_plane(candidate_point, &self.p1, &self.p2)
                            && candidate_point.distance(p) <= p.r
                    })
                }
                PlaneDirection::YZ => {
                    let y_diff = (p.y - closest_point.y);
                    let z_diff = (p.z - closest_point.z);

                    [
                        Point3D {
                            x: closest_point.x,
                            y: closest_point.y + y_diff,
                            z: closest_point.z + z_diff,
                            r: 1,
                        },
                        Point3D {
                            x: closest_point.x,
                            y: closest_point.y + y_diff,
                            z: closest_point.z - z_diff,
                            r: 1,
                        },
                        Point3D {
                            x: closest_point.x,
                            y: closest_point.y - y_diff,
                            z: closest_point.z + z_diff,
                            r: 1,
                        },
                        Point3D {
                            x: closest_point.x,
                            y: closest_point.y - y_diff,
                            z: closest_point.z - z_diff,
                            r: 1,
                        },
                    ]
                    .iter()
                    .any(|candidate_point| {
                        falls_on_plane(candidate_point, &self.p1, &self.p2)
                            && candidate_point.distance(p) <= p.r
                    })
                }
                PlaneDirection::XZ => {
                    let x_diff = (p.x - closest_point.x);
                    let z_diff = (p.z - closest_point.z);

                    [
                        Point3D {
                            x: closest_point.x + x_diff,
                            y: closest_point.y,
                            z: closest_point.z + z_diff,
                            r: 1,
                        },
                        Point3D {
                            x: closest_point.x + x_diff,
                            y: closest_point.y,
                            z: closest_point.z - z_diff,
                            r: 1,
                        },
                        Point3D {
                            x: closest_point.x - x_diff,
                            y: closest_point.y,
                            z: closest_point.z + z_diff,
                            r: 1,
                        },
                        Point3D {
                            x: closest_point.x - x_diff,
                            y: closest_point.y,
                            z: closest_point.z - z_diff,
                            r: 1,
                        },
                    ]
                    .iter()
                    .any(|candidate_point| {
                        falls_on_plane(candidate_point, &self.p1, &self.p2)
                            && candidate_point.distance(p) <= p.r
                    })
                }
            }
        }
    }

    // FIXME: we need cubes to be non-overlapping!  Need to fix the offsets up
    // Test that this is the case.  Also check whether the actual division is right here...
    fn split_cubes(cube: &Cube) -> Vec<Cube> {
        let x_step = (cube.x2 - cube.x1 + 1) / 2 - 1;
        let y_step = (cube.y2 - cube.y1 + 1) / 2 - 1;
        let z_step = (cube.z2 - cube.z1 + 1) / 2 - 1;

        // if x_step == 1 && y_step == 1 && z_step == 1 {
        //     panic!("Hit single position");
        // }

        if x_step < 0 || y_step < 0 || z_step < 0 {
            // panic!("Didn't expect a negative here");
            return Vec::new();
        }

        vec![
            // front top left
            Cube {
                x1: cube.x1,
                y1: cube.y1,
                z1: cube.z1,
                x2: cube.x1 + x_step,
                y2: cube.y1 + y_step,
                z2: cube.z1 + z_step,
            },
            // back top left
            Cube {
                x1: cube.x1,
                y1: cube.y1,
                z1: cube.z1 + z_step + 1,
                x2: cube.x1 + x_step,
                y2: cube.y1 + y_step,
                z2: cube.z2,
            },
            // front top right
            Cube {
                x1: cube.x1 + x_step + 1,
                y1: cube.y1,
                z1: cube.z1,
                x2: cube.x2,
                y2: cube.y1 + y_step,
                z2: cube.z1 + z_step,
            },
            // back top right
            Cube {
                x1: cube.x1 + x_step + 1,
                y1: cube.y1,
                z1: cube.z1 + z_step + 1,
                x2: cube.x2,
                y2: cube.y1 + y_step,
                z2: cube.z2,
            },
            // front bottom left
            Cube {
                x1: cube.x1,
                y1: cube.y1 + y_step + 1,
                z1: cube.z1,
                x2: cube.x1 + x_step,
                y2: cube.y2,
                z2: cube.z1 + z_step,
            },
            // back bottom left
            Cube {
                x1: cube.x1,
                y1: cube.y1 + y_step + 1,
                z1: cube.z1 + z_step + 1,
                x2: cube.x1 + x_step,
                y2: cube.y2,
                z2: cube.z2,
            },
            // front bottom right
            Cube {
                x1: cube.x1 + x_step + 1,
                y1: cube.y1 + y_step + 1,
                z1: cube.z1,
                x2: cube.x2,
                y2: cube.y2,
                z2: cube.z1 + z_step,
            },
            // back bottom right
            Cube {
                x1: cube.x1 + x_step + 1,
                y1: cube.y1 + y_step + 1,
                z1: cube.z1 + z_step + 1,
                x2: cube.x2,
                y2: cube.y2,
                z2: cube.z2,
            },
        ]
    }

    fn manhattan_distance(x1: i64, y1: i64, z1: i64, x2: i64, y2: i64, z2: i64) -> i64 {
        ((x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs())
    }

    // This is still getting skipped over:
    // 906: 17125603,40740992,33461246
    // even better:
    // Best single: Cube { x1: 18374828, y1: 41219173, z1: 34232290, x2: 18374828, y2: 41219173, z2: 34232290 } with 969

    pub fn part2() {
        let nanobots: Vec<Nanobot> = input_lines("input_files/day23.txt")
            .map(|s| Nanobot::from_str(&s))
            .collect();
        let _world_dim = 300_000_000;

        println!(
            "Test point: {}",
            nanobots
                .iter()
                .filter(
                    |n| manhattan_distance(17125603, 40740992, 33461246, n.x, n.y, n.z) <= n.radius
                )
                .count()
        );

        let mut search_queue = LinkedList::new();

        // Best single: Cube { x1: 18374828, y1: 41219173, z1: 34232290, x2: 18374828, y2: 41219173, z2: 34232290 } with 969
        search_queue.push_front(Cube {
            // x1: -world_dim, y1: -world_dim, z1: -world_dim,
            // x2: world_dim, y2: world_dim, z2: world_dim
            x1: 18000000,
            y1: 41000000,
            z1: 34000000,
            x2: 19000000,
            y2: 42000000,
            z2: 35000000,
        });

        let mut iterations = 0;
        while !search_queue.is_empty() {
            iterations += 1;
            let mut new_searches = LinkedList::new();
            let mut best = 0;

            while !search_queue.is_empty() {
                let area = search_queue.pop_front().unwrap();

                let subareas = split_cubes(&area);
                // let subareas = vec!(area);

                for c in subareas {
                    let mut count = 0;

                    for n in &nanobots {
                        let p = Point3D {
                            x: n.x,
                            y: n.y,
                            z: n.z,
                            r: n.radius,
                        };

                        if c.overlaps(&p) {
                            count += 1;
                        } else {
                            if (manhattan_distance(c.x1, c.y1, c.z1, p.x, p.y, p.z) <= p.r) {
                                panic!("Eh?");
                            }
                        }
                    }

                    if iterations < 5 {
                        // In the first few iterations we'll just take whatever.
                        new_searches.push_front(c);
                    } else if count > best {
                        best = count;

                        if c.is_single_position() {
                            println!("Best single: {:?} with {}", c, best);
                        } else {
                            new_searches.clear();
                            new_searches.push_front(c);
                        }
                    } else if count == best && best > 0 {
                        if c.is_single_position() {
                            println!("Best single: {:?} with {}", c, best);
                        } else {
                            new_searches.push_front(c);
                        }
                    } else {
                        if c.overlaps(&Point3D {
                            x: 18374828,
                            y: 41219173,
                            z: 34232290,
                            r: 1,
                        }) {
                            println!("Ignored: {:?} with {}", c, count);
                            println!("THE MONEY ONE ^^^^^^ in iteration {}", iterations);
                        }
                    }
                }
            }

            search_queue = new_searches;
            println!("Best was: {}", best);
            println!("Queue size: {}", search_queue.len());
        }
    }
}

mod day24 {
    use crate::shared::*;

    #[derive(Eq, PartialEq, Hash, Debug)]
    enum Team {
        ImmuneSystem,
        Infection,
    }

    #[derive(Eq, PartialEq, Hash, Debug)]
    enum Attr {
        Fire,
        Cold,
        Slashing,
        Bludgeoning,
        Radiation,
    }

    #[derive(Eq, PartialEq, Hash, Debug)]
    struct Group {
        team_type: Team,
        hp: i64,
        attack_damage: i64,
        attack_type: Attr,
        living_units: i64,
        initiative: i64,
        weaknesses: Vec<Attr>,
        immunities: Vec<Attr>,
    }

    fn load_groups() -> Vec<Group> {
        vec![
            Group {
                team_type: Team::ImmuneSystem,
                living_units: 1117,
                hp: 5042,
                weaknesses: vec![Attr::Slashing],
                immunities: vec![Attr::Fire, Attr::Radiation, Attr::Bludgeoning],
                attack_damage: 44,
                attack_type: Attr::Fire,
                initiative: 15,
            },
            Group {
                team_type: Team::ImmuneSystem,
                living_units: 292,
                hp: 2584,
                attack_damage: 81,
                attack_type: Attr::Bludgeoning,
                immunities: vec![],
                weaknesses: vec![],
                initiative: 18,
            },
            Group {
                team_type: Team::ImmuneSystem,
                living_units: 2299,
                hp: 8194,
                attack_damage: 35,
                attack_type: Attr::Radiation,
                immunities: vec![],
                weaknesses: vec![],
                initiative: 7,
            },
            Group {
                team_type: Team::ImmuneSystem,
                living_units: 1646,
                hp: 6315,
                weaknesses: vec![Attr::Slashing],
                immunities: vec![],
                attack_damage: 37,
                attack_type: Attr::Slashing,
                initiative: 14,
            },
            Group {
                team_type: Team::ImmuneSystem,
                living_units: 2313,
                hp: 6792,
                weaknesses: vec![Attr::Fire, Attr::Radiation],
                immunities: vec![Attr::Cold],
                attack_damage: 29,
                attack_type: Attr::Bludgeoning,
                initiative: 9,
            },
            Group {
                team_type: Team::ImmuneSystem,
                living_units: 2045,
                hp: 8634,
                immunities: vec![],
                weaknesses: vec![Attr::Radiation],
                attack_damage: 36,
                attack_type: Attr::Fire,
                initiative: 13,
            },
            Group {
                team_type: Team::ImmuneSystem,
                living_units: 34,
                hp: 1019,
                immunities: vec![],
                weaknesses: vec![Attr::Bludgeoning],
                attack_damage: 295,
                attack_type: Attr::Cold,
                initiative: 6,
            },
            Group {
                team_type: Team::ImmuneSystem,
                living_units: 157,
                hp: 6487,
                immunities: vec![],
                weaknesses: vec![Attr::Slashing, Attr::Cold],
                attack_damage: 362,
                attack_type: Attr::Radiation,
                initiative: 3,
            },
            Group {
                team_type: Team::ImmuneSystem,
                living_units: 1106,
                hp: 4504,
                immunities: vec![],
                weaknesses: vec![Attr::Cold],
                attack_damage: 39,
                attack_type: Attr::Slashing,
                initiative: 12,
            },
            Group {
                team_type: Team::ImmuneSystem,
                living_units: 5092,
                hp: 8859,
                immunities: vec![Attr::Cold, Attr::Slashing],
                weaknesses: vec![],
                attack_damage: 12,
                attack_type: Attr::Radiation,
                initiative: 16,
            },
            Group {
                team_type: Team::Infection,
                living_units: 3490,
                hp: 20941,
                immunities: vec![Attr::Fire],
                weaknesses: vec![],
                attack_damage: 9,
                attack_type: Attr::Bludgeoning,
                initiative: 5,
            },
            Group {
                team_type: Team::Infection,
                living_units: 566,
                hp: 11571,
                immunities: vec![],
                weaknesses: vec![Attr::Cold, Attr::Bludgeoning],
                attack_damage: 40,
                attack_type: Attr::Bludgeoning,
                initiative: 10,
            },
            Group {
                team_type: Team::Infection,
                living_units: 356,
                hp: 30745,
                immunities: vec![],
                weaknesses: vec![Attr::Radiation],
                attack_damage: 147,
                attack_type: Attr::Slashing,
                initiative: 8,
            },
            Group {
                team_type: Team::Infection,
                living_units: 899,
                hp: 49131,
                weaknesses: vec![Attr::Slashing],
                immunities: vec![Attr::Radiation, Attr::Bludgeoning, Attr::Fire],
                attack_damage: 93,
                attack_type: Attr::Cold,
                initiative: 19,
            },
            Group {
                team_type: Team::Infection,
                living_units: 1203,
                hp: 27730,
                immunities: vec![],
                weaknesses: vec![Attr::Cold],
                attack_damage: 43,
                attack_type: Attr::Slashing,
                initiative: 4,
            },
            Group {
                team_type: Team::Infection,
                living_units: 22,
                hp: 45002,
                immunities: vec![],
                weaknesses: vec![Attr::Bludgeoning],
                attack_damage: 3748,
                attack_type: Attr::Bludgeoning,
                initiative: 17,
            },
            Group {
                team_type: Team::Infection,
                living_units: 3028,
                hp: 35744,
                immunities: vec![],
                weaknesses: vec![Attr::Bludgeoning],
                attack_damage: 18,
                attack_type: Attr::Fire,
                initiative: 11,
            },
            Group {
                team_type: Team::Infection,
                living_units: 778,
                hp: 17656,
                immunities: vec![],
                weaknesses: vec![Attr::Fire],
                attack_damage: 35,
                attack_type: Attr::Bludgeoning,
                initiative: 2,
            },
            Group {
                team_type: Team::Infection,
                living_units: 47,
                hp: 16006,
                immunities: vec![Attr::Bludgeoning],
                weaknesses: vec![Attr::Cold, Attr::Radiation],
                attack_damage: 645,
                attack_type: Attr::Cold,
                initiative: 20,
            },
            Group {
                team_type: Team::Infection,
                living_units: 4431,
                hp: 13632,
                weaknesses: vec![Attr::Fire],
                immunities: vec![Attr::Bludgeoning],
                attack_damage: 6,
                attack_type: Attr::Bludgeoning,
                initiative: 1,
            },
        ]
    }

    impl Group {
        pub fn effective_power(&self) -> i64 {
            self.living_units * self.attack_damage
        }

        pub fn is_alive(&self) -> bool {
            self.living_units >= 0
        }

        pub fn damage_to(&self, other_group: &Group) -> i64 {
            if other_group.immunities.contains(&self.attack_type) {
                // Can't hurt them
                return 0;
            }

            let mut damage = self.effective_power();

            if other_group.weaknesses.contains(&self.attack_type) {
                // Boom!  Double damage.
                damage *= 2;
            }

            damage
        }

        pub fn record_attack_by(&mut self, other_group: &Group) {
        }
    }

    pub fn part1() {
        let mut groups = load_groups();

        for _fight in (0..100) {
            // Bring out your dead!
            groups = groups.into_iter().filter(|g| g.is_alive()).collect();
            groups.sort_by_key(|g| (-g.effective_power(), -g.initiative));

            // Target selection
            let mut attacker_to_target: HashMap<&Group, &Group> = HashMap::new();
            let mut target_to_attacker: HashMap<&Group, &Group> = HashMap::new();

            for group in &groups {
                let candidate_targets = groups
                    .iter()
                    .filter(|other_group| {
                        other_group.team_type != group.team_type
                            && !target_to_attacker.contains_key(other_group)
                    });

                if candidate_targets.clone().count() == 0 {
                    // Can't attack
                    println!("No possible targets!");
                    continue;
                }

                let best_target = candidate_targets.max_by_key(|target| (group.damage_to(target), target.effective_power(), target.initiative)).unwrap();

                if group.damage_to(best_target) > 0 {
                    // Attack it!
                    attacker_to_target.insert(group, best_target);
                    target_to_attacker.insert(best_target, group);
                }
            }

            // Attacking phase
            let mut attackers: Vec<&Group> = attacker_to_target.keys().map(|&k| k).collect();
            attackers.sort_by_key(|g| - g.initiative);

            for attacker in attackers {
                let target = attacker_to_target.get(attacker).unwrap();

                for mut group in groups.iter_mut() {
                    if *group == **target {
                        group.record_attack_by(attacker);
                    }
                }
            }
        }
    }
}

fn main() {
    if false {
        // regex_examples();

        day1::part1();
        day1::part2();

        day2::part1();
        day2::part2();

        day3::part1();
        day3::part2();

        day4::part1();
        day4::part2();

        day5::part1();
        day5::part2();

        day5::part1_alternative();
        day5::part2_alternative();

        day6::part1();
        day6::part2();

        day7::part1();
        day7::part2();

        day8::part1();
        day8::part2();

        day9::part1();
        day9::part2();

        day10::part1();
        day10::part2();

        day11::part1();
        day11::part2();

        day12::part1();
        day12::part2();

        day13::part1();
        day13::part2();

        day14::part1();
        day14::part2();

        day15::part1();
        day15::part2();

        day16::part1();
        day16::part2();

        day17::part1_and_2();

        day18::part1();
        day18::part2();

        day19::part1();
        day19::part2();

        day20::part1();
        day20::part2();

        day21::part1();
        day21::part2();

        day22::part1();
        day22::part2();

        day23::part1();
        day23::part2();
    }

    day24::part1();
}
