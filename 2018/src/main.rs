// (cd ../; cargo run)

#![allow(unused_parens)]
#![allow(dead_code)]

extern crate regex;

use regex::Regex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
            let mut entry = freqs.entry(ch).or_insert(0);
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

            let mut entry = tokens.entry(key).or_insert(HashSet::new());
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
struct Point(u64, u64);

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
                let entry = used.entry(Point(x + left, y + top)).or_insert(0);
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
                let entry = used.entry(Point(x + left, y + top)).or_insert(0);
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
                let entry = used.get(&Point(x + left, y + top)).unwrap();
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

fn main() {
    if false {
        regex_examples();

        day1_part1();
        day1_part2();

        day2_part1();
        day2_part2();

        day3_part1();
        day3_part2();
    }

    day4_part1();
    day4_part2();
}
