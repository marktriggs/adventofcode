// (cd ~/projects/adventofcode/2022 && cargo run)

// I like 'em!
#![allow(clippy::needless_range_loop)]

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
    pub use std::convert::TryFrom;
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

mod day6 {
    use crate::shared::*;

    pub fn part1() {
        let input = read_file("input_files/day6.txt");

        let mut position = 0;
        while position < input.len() {
            let last_four: HashSet<char> = input.chars().skip(position).take(4).collect();

            if last_four.len() == 4 {
                println!("First position: {}", position + 4);
                break;
            }

            position += 1;
        }
    }


    pub fn part2() {
        let input = read_file("input_files/day6.txt");

        let mut buffer: VecDeque<char> = VecDeque::new();

        for (idx, ch) in input.chars().enumerate() {
            if buffer.iter().copied().collect::<HashSet<char>>().len() == 14 {
                println!("First position: {}", idx);
                break;
            }

            buffer.push_back(ch);

            if buffer.len() > 14 {
                let _ = buffer.pop_front().unwrap();
            }
        }
    }
}

mod day7 {
    use crate::shared::*;

    #[derive(Debug)]
    struct Filesystem {
        files: HashMap<String, usize>,
        known_dirs: HashSet<String>,
    }

    impl Filesystem {
        pub fn new() -> Filesystem {
            let mut result = Filesystem {
                files: HashMap::new(),
                known_dirs: HashSet::new(),
            };

            result.known_dirs.insert("/".to_string());

            result
        }

        pub fn record_file(&mut self, parent_path: &[String], name: &str, size: usize) {
            for i in 1..parent_path.len() {
                let mut dir = parent_path[0..i].join("/");
                dir.push('/');

                self.known_dirs.insert(dir);
            }

            let mut file_path = parent_path.join("/");
            file_path.push('/');
            file_path.push_str(name);

            self.files.insert(file_path, size);
        }

        pub fn record_dir(&mut self, parent_path: &[String], name: &str) {
            for i in 1..parent_path.len() {
                let mut dir = parent_path[0..i].join("/");
                dir.push('/');

                self.known_dirs.insert(dir);
            }


            let mut dir_path = parent_path.join("/");
            dir_path.push('/');
            dir_path.push_str(name);
            dir_path.push('/');

            self.known_dirs.insert(dir_path);
        }
    }


    fn parse_fs_output(path: &str) -> Filesystem {
        let cd_regex = Regex::new(r"^\$ cd (.+?)$").unwrap();
        let ls_regex = Regex::new(r"^\$ ls$").unwrap();
        let dirent_file_regex = Regex::new(r"^([0-9]+) (.+)$").unwrap();
        let dirent_dir_regex = Regex::new(r"^dir (.+)$").unwrap();
        let eof_regex = Regex::new(r"^__EOF__$").unwrap();

        let mut filesystem = Filesystem::new();
        let mut cwd = vec!["".to_owned()];

        let mut input = input_lines(path).collect::<VecDeque<String>>();
        input.push_back("__EOF__".to_owned());

        loop {
            let line = input.pop_front().unwrap();

            if eof_regex.is_match(&line) {
                break;
            }

            if let Some(caps) = cd_regex.captures(&line) {
                let dir = caps.get(1).unwrap().as_str();

                if dir == ".." {
                    let _ = cwd.pop().unwrap();
                } else if dir.starts_with('/') {
                    cwd = dir.split('/').map(str::to_string).collect();

                    // Knocking out trailing slashes
                    if (cwd.len() > 1 && cwd[cwd.len() - 1].is_empty()) {
                        cwd.pop();
                    }
                } else {
                    cwd.push(dir.to_owned());
                }
            } else if ls_regex.is_match(&line) {
                loop {
                    let entry = input.pop_front().unwrap();

                    if eof_regex.is_match(&entry) || entry.starts_with('$') {
                        input.push_front(entry);
                        break;
                    }

                    if let Some(caps) = dirent_file_regex.captures(&entry) {
                        filesystem.record_file(&cwd,
                                               caps.get(2).unwrap().as_str(),
                                               caps.get(1).unwrap().as_str().parse().unwrap());
                    } else if let Some(caps) = dirent_dir_regex.captures(&entry) {
                        filesystem.record_dir(&cwd,
                                              caps.get(1).unwrap().as_str());
                    } else {
                        unreachable!();
                    }
                }
            } else {
                unreachable!();
            }
        }

        filesystem
    }


    pub fn part1() {
        let filesystem = parse_fs_output("input_files/day7.txt");

        let mut grand_total = 0;

        for dir in filesystem.known_dirs {
            let mut total_size = 0;
            for (path, size) in &filesystem.files {
                if path.starts_with(&dir) {
                    total_size += size;
                }
            }

            println!("{}: {}", dir, total_size);

            if total_size <= 100000 {
                grand_total += total_size;
            }
        }

        println!("Grand total: {}", grand_total);
    }

    pub fn part2() {
        let filesystem = parse_fs_output("input_files/day7.txt");

        let volume_size = 70_000_000;
        let space_needed = 30_000_000;

        let space_used: usize = filesystem.files.iter().map(|(_path, size)| size).sum();

        let space_to_free = space_needed - (volume_size - space_used);

        let mut dir_sizes: Vec<(String, usize)> = filesystem.known_dirs.iter().map(|dir| {
            let mut total_size = 0;
            for (path, size) in &filesystem.files {
                if path.starts_with(dir) {
                    total_size += size;
                }
            }

            (dir.to_owned(), total_size)
        }).collect();

        dir_sizes.sort_by_key(|(_path, size)| *size);

        for (path, size) in dir_sizes {
            if size >= space_to_free {
                println!("delete {} with size {}", path, size);
                break
            }
        }
    }
}

mod day8 {
    use crate::shared::*;

    pub fn part1() {
        let tree_heights = input_lines("input_files/day8.txt").map(|line| {
            line.chars().map(|ch| i32::try_from(ch.to_digit(10).unwrap()).unwrap()).collect()
        }).collect::<Vec<Vec<i32>>>();

        let grid_height = tree_heights.len();
        let grid_width = tree_heights[0].len();

        let mut visibility_map = (0..grid_height).map(|_| vec![false; grid_width]).collect::<Vec<Vec<bool>>>();

        for row in 0..grid_height {
            for colrange in &[(0..grid_width).collect::<Vec<usize>>(), (0..grid_width).rev().collect::<Vec<usize>>()] {
                let mut last_height: i32 = -1;
                for &col in colrange {
                    if tree_heights[row][col] > last_height {
                        visibility_map[row][col] = true;
                    }

                    last_height = std::cmp::max(last_height, tree_heights[row][col]);
                }
            }
        }

        for col in 0..grid_width {
            for rowrange in &[(0..grid_height).collect::<Vec<usize>>(), (0..grid_height).rev().collect::<Vec<usize>>()] {
                let mut last_height: i32 = -1;
                for &row in rowrange {
                    if tree_heights[row][col] > last_height {
                        visibility_map[row][col] = true;
                    }

                    last_height = std::cmp::max(last_height, tree_heights[row][col]);
                }
            }
        }

        println!("Visible trees: {}", visibility_map.iter().map(|row| row.iter().filter(|&&v| v).count()).sum::<usize>());
    }

    pub fn part2() {
        let tree_heights = input_lines("input_files/day8.txt").map(|line| {
            line.chars().map(|ch| usize::try_from(ch.to_digit(10).unwrap()).unwrap()).collect()
        }).collect::<Vec<Vec<usize>>>();

        let grid_height = tree_heights.len();
        let grid_width = tree_heights[0].len();

        let mut tree_scores = (0..grid_height).map(|_| vec![1; grid_width]).collect::<Vec<Vec<usize>>>();

        for row in 0..grid_height {
            for colrange in &[(0..grid_width).collect::<Vec<usize>>(), (0..grid_width).rev().collect::<Vec<usize>>()] {
                let mut seen_heights: VecDeque<usize> = VecDeque::new();
                for &col in colrange {

                    let mut tree_count = 0;
                    for &h in seen_heights.iter() {
                        tree_count += 1;

                        if h >= tree_heights[row][col] {
                            break;
                        }
                    }

                    tree_scores[row][col] *= tree_count;
                    seen_heights.push_front(tree_heights[row][col]);
                }
            }
        }

        for col in 0..grid_width {
            for rowrange in &[(0..grid_height).collect::<Vec<usize>>(), (0..grid_height).rev().collect::<Vec<usize>>()] {
                let mut seen_heights: VecDeque<usize> = VecDeque::new();
                for &row in rowrange {
                    let mut tree_count = 0;
                    for &h in seen_heights.iter() {
                        tree_count += 1;

                        if h >= tree_heights[row][col] {
                            break;
                        }
                    }

                    tree_scores[row][col] *= tree_count;
                    seen_heights.push_front(tree_heights[row][col]);
                }
            }
        }

        println!("Best score: {}", tree_scores.iter().map(|row| row.iter().max().unwrap()).max().unwrap());
    }
}

mod day9 {
    use crate::shared::*;

    #[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
    struct Position {
        x: i64,
        y: i64,
    }

    fn move_tail(head_position: Position, tail_position: Position) -> Position {
        let difference = Position {
            x: (head_position.x - tail_position.x),
            y: (head_position.y - tail_position.y),
        };

        if difference.x.abs() > 1 || difference.y.abs() > 1 {
            let adjustment = Position {
                x: if difference.x == 0 { 0 } else { difference.x / difference.x.abs() },
                y: if difference.y == 0 { 0 } else { difference.y / difference.y.abs() },
            };

            Position {
                x: tail_position.x + adjustment.x,
                y: tail_position.y + adjustment.y,
            }
        } else {
            tail_position
        }
    }

    pub fn part1() {
        let mut head_position = Position {x: 0, y: 0};
        let mut tail_position = Position {x: 0, y: 0};

        let mut tail_positions_reached: HashSet<Position> = HashSet::new();

        tail_positions_reached.insert(tail_position);

        for line in input_lines("input_files/day9.txt") {
            if let Some((direction, count)) = line.split(' ').collect_tuple() {
                let count = count.parse::<usize>().unwrap();

                for _ in (0..count) {
                    let adjustment = match direction {
                        "U" => Position { x: 0, y: 1},
                        "D" => Position { x: 0, y: -1},
                        "L" => Position { x: -1, y: 0},
                        "R" => Position { x: 1, y: 0},
                        _ => panic!("Parse error"),
                    };

                    head_position.x += adjustment.x;
                    head_position.y += adjustment.y;


                    tail_position = move_tail(head_position, tail_position);
                    tail_positions_reached.insert(tail_position);
                }
            }
        }

        dbg!(tail_positions_reached.len());
    }

    pub fn part2() {
        // head -> tail
        let mut knots = vec!(
            Position {x: 0, y: 0},
            Position {x: 0, y: 0},
            Position {x: 0, y: 0},
            Position {x: 0, y: 0},
            Position {x: 0, y: 0},
            Position {x: 0, y: 0},
            Position {x: 0, y: 0},
            Position {x: 0, y: 0},
            Position {x: 0, y: 0},
            Position {x: 0, y: 0},
        );

        let mut tail_positions_reached: HashSet<Position> = HashSet::new();

        tail_positions_reached.insert(knots[knots.len() - 1]);

        for line in input_lines("input_files/day9.txt") {
            if let Some((direction, count)) = line.split(' ').collect_tuple() {
                let count = count.parse::<usize>().unwrap();

                for _ in (0..count) {
                    let adjustment = match direction {
                        "U" => Position { x: 0, y: 1},
                        "D" => Position { x: 0, y: -1},
                        "L" => Position { x: -1, y: 0},
                        "R" => Position { x: 1, y: 0},
                        _ => panic!("Parse error"),
                    };

                    knots[0].x += adjustment.x;
                    knots[0].y += adjustment.y;

                    for i in 0..(knots.len() - 1) {
                        let new_position = move_tail(knots[i], knots[i + 1]);

                        knots[i + 1] = new_position
                    }

                    tail_positions_reached.insert(knots[knots.len() - 1]);
                }
            }
        }

        dbg!(tail_positions_reached.len());
    }
}


mod day10 {
    use crate::shared::*;

    fn run_program(lines: impl Iterator<Item=String>) -> Vec<i64> {
        let mut x = 1;

        let mut cycle_values: Vec<i64> = Vec::new();

        for line in lines {
            if line == "noop" {
                cycle_values.push(x);
            } else {
                let adjustment = line.split(' ').last().unwrap().parse::<i64>().unwrap();
                cycle_values.push(x);
                cycle_values.push(x);

                x += adjustment;
            }
        }

        cycle_values.push(x);
        cycle_values
    }

    pub fn part1() {
        let mut signal_strength = 0;

        let cycle_values = run_program(input_lines("input_files/day10.txt"));

        for cycle_input in &[20, 60, 100, 140, 180, 220] {
            signal_strength += (num_traits::cast::<_, i64>(*cycle_input).unwrap() * cycle_values[cycle_input - 1]);
        }

        println!("Final signal strength: {}", signal_strength);
    }

    pub fn part2() {
        let cycle_values = run_program(input_lines("input_files/day10.txt"));

        let screen_width = 40;
        let screen_height = 6;

        let mut crt: Vec<Vec<char>> = (0..screen_height).map(|_| vec!['.'; screen_width]).collect();

        let mut cycle = 0;
        for row in 0..screen_height {
            for col in 0..screen_width {
                let x_position = cycle_values[cycle];

                if ((x_position - 1)..=(x_position + 1)).contains(&i64::try_from(col).unwrap()) {
                    crt[row][col] = '#';
                }

                cycle += 1;
            }
        }

        for row in 0..screen_height {
            for col in 0..screen_width {
                print!("{}", crt[row][col]);
            }

            println!();
        }
    }
}

mod day11 {
    use crate::shared::*;

    struct Monkey {
        items: VecDeque<usize>,
        operation: Box<dyn Fn(usize) -> usize>,
        prime: usize,
        true_target: usize,
        false_target: usize,
        monkey_business: usize,
    }

    pub fn part1() {
        let mut _sample_monkeys: Vec<Option<Monkey>> = vec![
            Some(Monkey {
                items: [79, 98].iter().copied().collect(),
                operation: Box::new(|old| old * 19),
                prime: 23,
                true_target: 2,
                false_target: 3,
                monkey_business: 0,
            }),
            Some(Monkey {
                items: [54, 65, 75, 74].iter().copied().collect(),
                operation: Box::new(|old| old + 6),
                prime: 19,
                true_target: 2,
                false_target: 0,
                monkey_business: 0,
            }),
            Some(Monkey {
                items: [79, 60, 97].iter().copied().collect(),
                operation: Box::new(|old| old * old),
                prime: 13,
                true_target: 1,
                false_target: 3,
                monkey_business: 0,
            }),
            Some(Monkey {
                items: [74].iter().copied().collect(),
                operation: Box::new(|old| old + 3),
                prime: 17,
                true_target: 0,
                false_target: 1,
                monkey_business: 0,
            })
        ];

        let mut monkeys: Vec<Option<Monkey>> = vec![
            Some(Monkey {
                items: [98, 89, 52].iter().copied().collect(),
                operation: Box::new(|old| old * 2),
                prime: 5,
                true_target: 6,
                false_target: 1,
                monkey_business: 0,
            }),

            Some(Monkey {
                items: [57, 95, 80, 92, 57, 78].iter().copied().collect(),
                operation: Box::new(|old| old * 13),
                prime: 2,
                true_target: 2,
                false_target: 6,
                monkey_business: 0,
            }),

            Some(Monkey {
                items: [82, 74, 97, 75, 51, 92, 83].iter().copied().collect(),
                operation: Box::new(|old| old + 5),
                prime: 19,
                true_target: 7,
                false_target: 5,
                monkey_business: 0,
            }),

            Some(Monkey {
                items: [97, 88, 51, 68, 76].iter().copied().collect(),
                operation: Box::new(|old| old + 6),
                prime: 7,
                true_target: 0,
                false_target: 4,
                monkey_business: 0,
            }),

            Some(Monkey {
                items: [63].iter().copied().collect(),
                operation: Box::new(|old| old + 1),
                prime: 17,
                true_target: 0,
                false_target: 1,
                monkey_business: 0,
            }),

            Some(Monkey {
                items: [94, 91, 51, 63].iter().copied().collect(),
                operation: Box::new(|old| old + 4),
                prime: 13,
                true_target: 4,
                false_target: 3,
                monkey_business: 0,
            }),

            Some(Monkey {
                items: [61, 54, 94, 71, 74, 68, 98, 83].iter().copied().collect(),
                operation: Box::new(|old| old + 2),
                prime: 3,
                true_target: 2,
                false_target: 7,
                monkey_business: 0,
            }),

            Some(Monkey {
                items: [90, 56].iter().copied().collect(),
                operation: Box::new(|old| old * old),
                prime: 11,
                true_target: 3,
                false_target: 5,
                monkey_business: 0,
            }),
        ];


        let prime_multiple: usize = monkeys.iter().map(|m| m.as_ref().unwrap().prime).product();

        for _round in 0..20 {
            for monkey_idx in 0..monkeys.len() {
                let mut monkey = std::mem::replace(&mut monkeys[monkey_idx], None).unwrap();

                while !monkey.items.is_empty() {
                    monkey.monkey_business += 1;

                    let mut worry_level = monkey.items.pop_front().unwrap();

                    worry_level = (monkey.operation)(worry_level);
                    worry_level /= 3;

                    // CALM DOWN
                    while worry_level > prime_multiple {
                        worry_level -= prime_multiple;
                    }

                    let target_monkey_idx = if worry_level % monkey.prime == 0 {
                        monkey.true_target
                    } else {
                        monkey.false_target
                    };

                    let mut target_monkey = std::mem::replace(&mut monkeys[target_monkey_idx], None).unwrap();
                    target_monkey.items.push_back(worry_level);
                    monkeys[target_monkey_idx] = Some(target_monkey);
                }

                monkeys[monkey_idx] = Some(monkey);
            }
        }

        let mut monkey_scores: Vec<usize> = monkeys.iter().map(|m| m.as_ref().unwrap().monkey_business).collect();
        monkey_scores.sort_by_key(|score| *score);
        monkey_scores.reverse();

        println!("Top two monkeys: {}", monkey_scores[0] * monkey_scores[1]);
    }

    pub fn part2() {
        let mut _sample_monkeys: Vec<Option<Monkey>> = vec![
            Some(Monkey {
                items: [79, 98].iter().copied().collect(),
                operation: Box::new(|old| old * 19),
                prime: 23,
                true_target: 2,
                false_target: 3,
                monkey_business: 0,
            }),
            Some(Monkey {
                items: [54, 65, 75, 74].iter().copied().collect(),
                operation: Box::new(|old| old + 6),
                prime: 19,
                true_target: 2,
                false_target: 0,
                monkey_business: 0,
            }),
            Some(Monkey {
                items: [79, 60, 97].iter().copied().collect(),
                operation: Box::new(|old| old * old),
                prime: 13,
                true_target: 1,
                false_target: 3,
                monkey_business: 0,
            }),
            Some(Monkey {
                items: [74].iter().copied().collect(),
                operation: Box::new(|old| old + 3),
                prime: 17,
                true_target: 0,
                false_target: 1,
                monkey_business: 0,
            })
        ];

        let mut monkeys: Vec<Option<Monkey>> = vec![
            Some(Monkey {
                items: [98, 89, 52].iter().copied().collect(),
                operation: Box::new(|old| old * 2),
                prime: 5,
                true_target: 6,
                false_target: 1,
                monkey_business: 0,
            }),

            Some(Monkey {
                items: [57, 95, 80, 92, 57, 78].iter().copied().collect(),
                operation: Box::new(|old| old * 13),
                prime: 2,
                true_target: 2,
                false_target: 6,
                monkey_business: 0,
            }),

            Some(Monkey {
                items: [82, 74, 97, 75, 51, 92, 83].iter().copied().collect(),
                operation: Box::new(|old| old + 5),
                prime: 19,
                true_target: 7,
                false_target: 5,
                monkey_business: 0,
            }),

            Some(Monkey {
                items: [97, 88, 51, 68, 76].iter().copied().collect(),
                operation: Box::new(|old| old + 6),
                prime: 7,
                true_target: 0,
                false_target: 4,
                monkey_business: 0,
            }),

            Some(Monkey {
                items: [63].iter().copied().collect(),
                operation: Box::new(|old| old + 1),
                prime: 17,
                true_target: 0,
                false_target: 1,
                monkey_business: 0,
            }),

            Some(Monkey {
                items: [94, 91, 51, 63].iter().copied().collect(),
                operation: Box::new(|old| old + 4),
                prime: 13,
                true_target: 4,
                false_target: 3,
                monkey_business: 0,
            }),

            Some(Monkey {
                items: [61, 54, 94, 71, 74, 68, 98, 83].iter().copied().collect(),
                operation: Box::new(|old| old + 2),
                prime: 3,
                true_target: 2,
                false_target: 7,
                monkey_business: 0,
            }),

            Some(Monkey {
                items: [90, 56].iter().copied().collect(),
                operation: Box::new(|old| old * old),
                prime: 11,
                true_target: 3,
                false_target: 5,
                monkey_business: 0,
            }),
        ];


        let prime_multiple: usize = monkeys.iter().map(|m| m.as_ref().unwrap().prime).product();

        for _round in 0..10000 {
            for monkey_idx in 0..monkeys.len() {
                let mut monkey = std::mem::replace(&mut monkeys[monkey_idx], None).unwrap();

                while !monkey.items.is_empty() {
                    monkey.monkey_business += 1;

                    let mut worry_level = monkey.items.pop_front().unwrap();

                    worry_level = (monkey.operation)(worry_level);

                    // CALM DOWN
                    while worry_level >= prime_multiple {
                        worry_level -= prime_multiple;
                    }

                    let target_monkey_idx = if worry_level % monkey.prime == 0 {
                        monkey.true_target
                    } else {
                        monkey.false_target
                    };

                    let mut target_monkey = std::mem::replace(&mut monkeys[target_monkey_idx], None).unwrap();
                    target_monkey.items.push_back(worry_level);
                    monkeys[target_monkey_idx] = Some(target_monkey);
                }

                monkeys[monkey_idx] = Some(monkey);
            }
        }

        let mut monkey_scores: Vec<usize> = monkeys.iter().map(|m| m.as_ref().unwrap().monkey_business).collect();
        monkey_scores.sort_by_key(|score| *score);
        monkey_scores.reverse();

        println!("Top two monkeys: {}", monkey_scores[0] * monkey_scores[1]);
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

        day5::part1();
        day5::part2();

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
    }

    day11::part1();
    day11::part2();

}
