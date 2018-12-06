// (cd ../; cargo run --release)

#![allow(unused_parens)]
#![allow(dead_code)]

extern crate regex;

use regex::Regex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const ALPHABET: &str = "abcdefghijlkmnopqrstuvwxyz";
const ALPHABET_UPPER: &str = "ABCDEFGHIJLKMNOPQRSTUVWXYZ";

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

fn input_lines(file: &str) -> impl Iterator<Item = String> {
    let f = File::open(file).expect(&format!("Failed to open input file: {}", &file));
    BufReader::new(f).lines().map(Result::unwrap)
}

fn day1_part1() {
    let frequency = input_lines("input_files/day1.txt")
        .map(|s| s.parse().unwrap())
        .fold(0, |acc: i64, n: i64| acc + n);

    println!("Final frequency: {}", frequency);
}

fn day1_part2() {
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

fn day2_part1() {
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

fn day2_part2() {
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

#[derive(Hash, Eq, PartialEq)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn parse_csv(s: &str) -> Point {
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

fn day3_part1() {
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

fn day3_part2() {
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

#[derive(Debug)]
struct Guard {
    id: String,
    sleep_time: usize,
    sleep_minutes: Vec<usize>,
}

fn day4_new_guard(id: String) -> Guard {
    Guard {
        id: id,
        sleep_time: 0,
        sleep_minutes: vec![0; 60],
    }
}

fn day4_part1() {
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
                .or_insert_with(|| day4_new_guard(name.clone()));
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

fn day4_part2() {
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
                .or_insert_with(|| day4_new_guard(name.clone()));
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

fn day5_part1() {
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

fn day5_part2() {
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

fn day5_part1_alternative() {
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

fn day5_part2_alternative() {
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

fn day6_part1() {
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

fn day6_part2() {
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

fn main() {
    if false {
        regex_examples();

        day1_part1();
        day1_part2();

        day2_part1();
        day2_part2();

        day3_part1();
        day3_part2();

        day4_part1();
        day4_part2();

        day5_part1();
        day5_part2();

        day5_part1_alternative();
        day5_part2_alternative();

        day6_part1();
        day6_part2();
    }

    day6_part1();
    day6_part2();
}
