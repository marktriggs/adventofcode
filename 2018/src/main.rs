// (cd ../; cargo run --release)

#![allow(unused_parens)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate regex;

mod shared {
    pub use regex::Regex;

    pub use std::collections::HashMap;
    pub use std::collections::HashSet;
    pub use std::fmt::{self, Display};
    pub use std::fs::File;
    pub use std::io::{self, BufRead, BufReader, Write};
    pub use std::str;
    pub use std::cmp;

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
    where T: Display
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
    use crate::shared::*;

    type Registers = Vec<usize>;

    trait Operation {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize);
    }

    // Addition
    struct OpAddr;
    impl Operation for OpAddr {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = regs[a] + regs[b];
        }
    }

    struct OpAddi;
    impl Operation for OpAddi {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = regs[a] + b;
        }
    }

    // Multiplication
    struct OpMulr;
    impl Operation for OpMulr {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = regs[a] * regs[b];
        }
    }

    struct OpMuli;
    impl Operation for OpMuli {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = regs[a] * b;
        }
    }

    // Bitwise AND
    struct OpBanr;
    impl Operation for OpBanr {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = regs[a] & regs[b];
        }
    }

    struct OpBani;
    impl Operation for OpBani {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = regs[a] & b;
        }
    }

    // Bitwise OR
    struct OpBorr;
    impl Operation for OpBorr {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = regs[a] | regs[b];
        }
    }

    struct OpBori;
    impl Operation for OpBori {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = regs[a] | b;
        }
    }

    // Assignment
    struct OpSetr;
    impl Operation for OpSetr {
        fn invoke(&self, regs: &mut Registers, a: usize, _b: usize, c: usize) {
            regs[c] = regs[a];
        }
    }

    struct OpSeti;
    impl Operation for OpSeti {
        fn invoke(&self, regs: &mut Registers, a: usize, _b: usize, c: usize) {
            regs[c] = a
        }
    }

    // Greater-than testing
    struct OpGtir;
    impl Operation for OpGtir {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = if a > regs[b] { 1 } else { 0 };
        }
    }

    struct OpGtri;
    impl Operation for OpGtri {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = if regs[a] > b { 1 } else { 0 };
        }
    }

    struct OpGtrr;
    impl Operation for OpGtrr {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = if regs[a] > regs[b] { 1 } else { 0 };
        }
    }

    // Equality testing
    struct OpEqir;
    impl Operation for OpEqir {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = if a == regs[b] { 1 } else { 0 };
        }
    }

    struct OpEqri;
    impl Operation for OpEqri {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = if regs[a] == b { 1 } else { 0 };
        }
    }

    struct OpEqrr;
    impl Operation for OpEqrr {
        fn invoke(&self, regs: &mut Registers, a: usize, b: usize, c: usize) {
            regs[c] = if regs[a] == regs[b] { 1 } else { 0 };
        }
    }

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
    struct Bounds {
        x_min: usize,
        x_max: usize,
        y_min: usize,
        y_max: usize,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct Droplet {
        position: Point,
        direction: (i64, i64),
        stuck: bool,
    }

    impl Droplet {
        pub fn new(x: u64, y: u64) -> Droplet {
            Droplet {
                position: Point { x, y },
                direction: (0, 1), // down
                stuck: false,
            }
        }
    }

    impl Bounds {
        fn new() -> Bounds {
            Bounds {
                x_min: std::usize::MAX,
                x_max: 0,
                y_min: std::usize::MAX,
                y_max: 0,
            }
        }

        fn update_from(&mut self, reading: &ClayReading) {
            self.x_min = cmp::min(self.x_min, reading.x_start);
            self.x_max = cmp::max(self.x_max, reading.x_end);
            self.y_min = cmp::min(self.y_min, reading.y_start);
            self.y_max = cmp::max(self.y_max, reading.y_end);
        }

        fn from_readings(readings: &Vec<ClayReading>) -> Bounds {
            let mut result = Bounds::new();

            for clay in readings {
                result.update_from(&clay);
            }

            result
        }

        fn width(&self) -> usize { self.x_max - self.x_min + 1 }
        fn height(&self) -> usize { self.y_max - self.y_min + 1 }
        fn yval(&self, val: usize) -> usize { val - self.y_min }
        fn xval(&self, val: usize) -> usize { val - self.x_min + 1 } // + 1 for the LHS void
    }

    #[derive(PartialEq, Eq, Clone)]
    enum Cell {
        Sand,
        Clay,
        Occupied,
        TheMagnificentVoid,
        Wet,
    }

    impl Display for Cell {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let s = match self {
                Cell::Sand => ".",
                Cell::Clay => "#",
                Cell::Occupied => "~",
                Cell::TheMagnificentVoid => "X",
                Cell::Wet => "W",
            };

            write!(f, "{}", s)
        }
    }

    // FIXME: simulating too much stuff here to be fast.  Need to think about
    // more efficient fill methods.
    //
    // New idea:
    //
    //   Start with the drip point
    //   If we can move down into sand, that's what we do.  Add 1 to our cell count
    //   When we can't go down anymore:
    //
    //     - project left while we have clay underneath us and sand in front of us, counting as we go
    //
    //     - project right in the same way
    //
    //   If we hit clay on the left and right, we're in an enclosed space.  Move up a square and repeat the process
    //
    //   Otherwise, we're in an open space: open either on one side or both.  Add points to explore to the openings and continue from those.

    pub fn part1() {
        let clay_readings: Vec<ClayReading> = input_lines("input_files/day17.txt")
            .map(|line| ClayReading::parse(&line))
            .collect();

        let mut bounds = Bounds::from_readings(&clay_readings);

        bounds.y_min = 0;

        println!("{:?}", bounds);

        // Fill the world with sand
        let mut world: Vec<Vec<Cell>> = (0..bounds.height()).map(|_| {
            (0..bounds.width()).map (|_| Cell::Sand).collect()
        }).collect();

        // Add an extra VOID row on the bottom and sides
        world.push((0..bounds.width()).map (|_| Cell::TheMagnificentVoid).collect());
        for row in world.iter_mut() {
            row.insert(0, Cell::TheMagnificentVoid);
            row.push(Cell::TheMagnificentVoid);
        }

        // Then apply our readings to fill out the clay
        for reading in &clay_readings {
            for y in reading.y_start..=reading.y_end {
                for x in reading.x_start..=reading.x_end {
                    world[bounds.yval(y)][bounds.xval(x)] = Cell::Clay;
                }
            }
        }

        // Droplets in descending order of age
        let mut droplets: Vec<Option<Droplet>> = Vec::new();

        let mut all_positions = HashSet::new();

        for frame in 0..std::usize::MAX {
            let position_count = all_positions.len();

            if frame % 100 == 0 {
                println!("{}", frame);

                // GC!
                droplets = droplets.into_iter().filter(|o| o.is_some()).collect();
                println!("{} droplets in play", droplets.len());
            }

            // if frame % 2000 == 0 || frame % 2000 == 1 {
            //     println!("{}", frame);
            //     println!("{} droplets in play", droplets.iter().filter(|d| !d.stuck).count());
            // 
            //     let mut f = File::create(format!("frame_{}.txt", frame)).unwrap();
            //     f.write_all(format!("{}", format_grid(&world)).as_bytes());
            // }

            // Drip...
            if world[bounds.yval(0)][bounds.xval(500)] == Cell::Sand {
                world[bounds.yval(0)][bounds.xval(500)] = Cell::Occupied;

                droplets.push(Some(Droplet::new(bounds.xval(500) as u64,
                                                bounds.yval(0) as u64)));
            }

            // if frame == 0 {
            //     println!("{:?}", droplets);
            // 
            //     println!("\n{}", format_grid(&world));
            // }



            let mut i = 0;
            while i < droplets.len() {
                if droplets[i].is_none() || droplets[i].as_ref().unwrap().stuck {
                    i += 1;
                    continue;
                }

                droplets.push(None);
                let mut droplet = droplets.swap_remove(i).unwrap();

                // If we hit the void that's the end of this droplet
                if world[(droplet.position.y + 1) as usize][droplet.position.x as usize] == Cell::TheMagnificentVoid {
                    world[droplet.position.y as usize][droplet.position.x as usize] = Cell::Sand;
                    continue;
                }

                // If we can move down we always do
                if world[(droplet.position.y + 1) as usize][droplet.position.x as usize] == Cell::Sand {
                    world[droplet.position.y as usize][droplet.position.x as usize] = Cell::Sand;
                    world[(droplet.position.y + 1) as usize][droplet.position.x as usize] = Cell::Occupied;
                    droplet.position = Point { x: droplet.position.x, y: droplet.position.y + 1 };
                    droplet.direction = (0, 1);
                    droplets.push(Some(droplet));
                    droplets.swap_remove(i);
                    i += 1;
                    continue;
                }

                // Otherwise, keep moving in the direction we were headed
                let new_x = (droplet.position.x as i64 + droplet.direction.0) as usize;
                let new_y = (droplet.position.y as i64 + droplet.direction.1) as usize;

                if world[new_y][new_x] == Cell::TheMagnificentVoid {
                    world[droplet.position.y as usize][droplet.position.x as usize] = Cell::Sand;
                    continue;
                }

                if world[new_y][new_x] == Cell::Sand {
                    // We can move.  Free the old spot and update our position.
                    world[droplet.position.y as usize][droplet.position.x as usize] = Cell::Sand;
                    world[new_y][new_x] = Cell::Occupied;
                    droplet.position = Point { x: new_x as u64, y: new_y as u64 };
                    droplets.push(Some(droplet));
                    droplets.swap_remove(i);
                    i += 1;
                } else {
                    if droplet.direction.1 != 1 {
                        // We were already moving left/right, can't move down, can't move further.  We're stuck.
                        droplet.stuck = true;
                        droplets.push(Some(droplet));
                        droplets.swap_remove(i);
                        i += 1;
                        continue;
                    }

                    let mut leftward = droplet.clone();
                    let mut rightward = droplet.clone();

                    leftward.position = Point { x: droplet.position.x - 1, y: droplet.position.y };
                    rightward.position = Point { x: droplet.position.x + 1, y: droplet.position.y };

                    leftward.direction = (-1, 0);
                    rightward.direction = (1, 0);

                    let mut inserted = 0;

                    if world[leftward.position.y as usize][leftward.position.x as usize] == Cell::Sand {
                        world[leftward.position.y as usize][leftward.position.x as usize] = Cell::Occupied;
                        world[droplet.position.y as usize][droplet.position.x as usize] = Cell::Sand;
                        droplets.insert(i, Some(leftward));
                        inserted += 1;
                    }

                    if world[rightward.position.y as usize][rightward.position.x as usize] == Cell::Sand {
                        // Clear current...
                        world[rightward.position.y as usize][rightward.position.x as usize] = Cell::Occupied;
                        world[droplet.position.y as usize][droplet.position.x as usize] = Cell::Sand;
                        droplets.insert(i, Some(rightward));
                        inserted += 1;
                    }

                    if inserted == 0 {
                        // Couldn't move
                        droplet.stuck = true;
                        droplets.push(Some(droplet));
                        droplets.swap_remove(i);
                        i += 1;
                    } else {
                        i += inserted;
                    }
                }
            }

            let new_positions: HashSet<Point> = droplets.iter().map(|cell| {
                if let Some(d) = cell {
                    Some(d.position.clone())
                } else {
                    None
                }
            }).filter(Option::is_some).map(Option::unwrap).collect();

            all_positions = all_positions.union(&new_positions).cloned().collect();

            if all_positions.len() == position_count {
                // Done!
                println!("Finished in {} frames", frame);
                break;
            }
        }

        println!("Wet cells: {}", all_positions.len());
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
    }

    day17::part1();
}
