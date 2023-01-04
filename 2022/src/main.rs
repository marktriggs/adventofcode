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
extern crate once_cell;
extern crate enum_map;

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


mod day12 {
    use crate::shared::*;

    const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

    #[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
    struct Position {
        x: i64,
        y: i64,
    }


    #[derive(Debug)]
    struct HeightMap {
        grid: Vec<Vec<i64>>,
        width: i64,
        height: i64,
        start: Position,
        goal: Position,
    }

    impl HeightMap {
        fn parse(lines: impl Iterator<Item=String>) -> HeightMap {
            let mut start: Option<Position> = None;
            let mut goal: Option<Position> = None;

            let grid: Vec<Vec<i64>> =
                lines.enumerate().map(|(row, line)| {
                    line.chars().enumerate().map(|(col, ch)| {
                        let depth = match ch {
                            'S' => 0,
                            'E' => 25,
                            _ => ALPHABET.find(ch).unwrap() as i64,
                        };

                        if ch == 'S' {
                            start = Some(Position { x: col as i64, y: row as i64 });
                        } else if ch == 'E' {
                            goal = Some(Position { x: col as i64, y: row as i64 });
                        }

                        depth
                    }).collect::<Vec<i64>>()
                }).collect();

            let width = grid[0].len() as i64;
            let height = grid.len() as i64;

            HeightMap {
                grid,
                width,
                height,
                start: start.unwrap(),
                goal: goal.unwrap(),
            }
        }

        fn neighbours(&self, position: Position) -> Vec<Position> {
            let mut result = Vec::with_capacity(4);

            for (xoff, yoff) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let new_x = position.x + xoff;
                let new_y = position.y + yoff;

                if (new_x >= 0 && new_x < self.width) && (new_y >= 0 && new_y < self.height) {
                    result.push(Position { x: new_x, y: new_y });
                }
            }

            result
        }

        fn height(&self, position: Position) -> i64 {
            self.grid[position.y as usize][position.x as usize]
        }
    }

    struct Path {
        current_position: Position,
        length: usize,
    }

    pub fn part1() {
        let height_map = HeightMap::parse(input_lines("input_files/day12.txt"));

        let mut shortest_path = usize::MAX;
        let mut active_paths = vec!(Path { current_position: height_map.start, length: 0 });

        let mut best_cost_map: HashMap<Position, usize> = HashMap::new();

        while !active_paths.is_empty() {
            let mut new_paths = Vec::new();

            for path in active_paths {
                if path.current_position == height_map.goal {
                    if path.length < shortest_path {
                        shortest_path = path.length;
                    }

                    continue;
                }

                for position in height_map.neighbours(path.current_position) {
                    if height_map.height(position) <= height_map.height(path.current_position) + 1 {
                        // OK to move
                        if best_cost_map.get(&position).copied().unwrap_or(usize::MAX) > path.length + 1 {
                            // It's a good move!
                            best_cost_map.insert(position, path.length + 1);
                            new_paths.push(Path {
                                current_position: position,
                                length: path.length + 1,
                            });
                        }
                    }
                }
            }

            active_paths = new_paths;
        }

        println!("Found shortest path: {}", shortest_path);
    }

    pub fn part2() {
        let base_height_map = HeightMap::parse(input_lines("input_files/day12.txt"));

        let mut shortest_path = usize::MAX;

        let mut candidate_starts: Vec<HeightMap> = Vec::new();

        for row in 0..base_height_map.height {
            for col in 0..base_height_map.width {
                if base_height_map.height(Position { x: col, y: row }) == 0 {
                    candidate_starts.push(HeightMap {
                        grid: base_height_map.grid.clone(),
                        start: Position { x: col, y: row },
                        .. base_height_map
                    });
                }
            }
        }

        for height_map in candidate_starts {
            let mut active_paths = vec!(Path { current_position: height_map.start, length: 0 });

            let mut best_cost_map: HashMap<Position, usize> = HashMap::new();

            while !active_paths.is_empty() {
                let mut new_paths = Vec::new();

                for path in active_paths {
                    if path.current_position == height_map.goal {
                        if path.length < shortest_path {
                            shortest_path = path.length;
                        }

                        continue;
                    }

                    for position in height_map.neighbours(path.current_position) {
                        if height_map.height(position) <= height_map.height(path.current_position) + 1 {
                            // OK to move
                            if best_cost_map.get(&position).copied().unwrap_or(usize::MAX) > path.length + 1 {
                                // It's a good move!
                                best_cost_map.insert(position, path.length + 1);
                                new_paths.push(Path {
                                    current_position: position,
                                    length: path.length + 1,
                                });
                            }
                        }
                    }
                }

                active_paths = new_paths;
            }
        }

        println!("Found shortest path: {}", shortest_path);
    }
}

mod day13 {
    use crate::shared::*;

    #[derive(Debug, Clone)]
    enum Packet {
        Integer(usize),
        List(Vec<Packet>),
    }

    fn parse_packet(s: &str) -> (Packet, &str) {
        if let Some(mut rest) = s.strip_prefix('[') {
            let mut elements = Vec::new();

            while !rest.starts_with(']') {
                if rest.starts_with(',') {
                    rest = &rest[1..];
                }

                let (element, new_rest) = parse_packet(rest);
                rest = new_rest;
                elements.push(element);
            }

            (Packet::List(elements), &rest[1..])
        } else {
            let len = s.chars().take_while(|&c| char::is_digit(c, 10)).count();
            (Packet::Integer(s[0..len].parse().unwrap()), &s[len..])
        }
    }

    #[derive(Eq, PartialEq, Debug)]
    enum PacketOrder {
        Right,
        Wrong,
        Unsure,
    }

    fn determine_order(p1: &Packet, p2: &Packet) -> PacketOrder {
        match (p1, p2) {
            (Packet::Integer(n1), Packet::Integer(n2)) => {
                match n1.cmp(n2) {
                    Ordering::Less => PacketOrder::Right,
                    Ordering::Greater => PacketOrder::Wrong,
                    Ordering::Equal => PacketOrder::Unsure,
                }
            },
            (Packet::List(l1), Packet::List(l2)) => {
                for idx in 0..std::cmp::min(l1.len(), l2.len()) {
                    let order = determine_order(&l1[idx], &l2[idx]);

                    if order != PacketOrder::Unsure {
                        return order;
                    }
                }

                match l1.len().cmp(&l2.len()) {
                    Ordering::Less => PacketOrder::Right,
                    Ordering::Greater => PacketOrder::Wrong,
                    Ordering::Equal => PacketOrder::Unsure,
                }
            },
            (Packet::List(_), Packet::Integer(_)) => {
                let wrapped = Packet::List(vec![p2.clone()]);

                determine_order(p1, &wrapped)
            },
            (Packet::Integer(_), Packet::List(_)) => {
                let wrapped = Packet::List(vec![p1.clone()]);

                determine_order(&wrapped, p2)
            },
        }
    }

    pub fn part1() {
        let lines: Vec<String> = input_lines("input_files/day13.txt").collect();

        let mut total = 0;

        for (idx, line_pair) in lines.split(|s| s.is_empty()).enumerate() {
            assert_eq!(line_pair.len(), 2);

            let packet1 = parse_packet(&line_pair[0]).0;
            let packet2 = parse_packet(&line_pair[1]).0;

            if determine_order(&packet1, &packet2) == PacketOrder::Right {
                total += idx + 1;
            }
        }

        println!("Pt1: {}", total);
    }

    pub fn part2() {
        let mut lines: Vec<String> = input_lines("input_files/day13.txt")
            .filter(|line| !line.is_empty())
            .collect();

        lines.push("[[2]]".to_owned());
        lines.push("[[6]]".to_owned());

        lines.sort_by(|l1, l2| {
            match determine_order(&parse_packet(l1).0, &parse_packet(l2).0) {
                PacketOrder::Right => Ordering::Less,
                PacketOrder::Wrong => Ordering::Greater,
                PacketOrder::Unsure => Ordering::Equal,
            }
        });

        println!("Decoder key: {}",
                 (lines.iter().position(|elt| elt == "[[2]]").unwrap() + 1) *
                 (lines.iter().position(|elt| elt == "[[6]]").unwrap() + 1));
    }
}

mod day14 {
    use crate::shared::*;

    type Point = (usize, usize);

    #[derive(Clone)]
    enum Tile {
        Rock,
        Sand,
    }

    struct CaveWithAbyss {
        grid: HashMap<Point, Tile>,
        min_x: usize,
        max_x: usize,
        min_y: usize,
        max_y: usize,
    }

    impl CaveWithAbyss {
        pub fn new() -> CaveWithAbyss {
            CaveWithAbyss {
                grid: HashMap::new(),
                min_x: usize::MAX,
                min_y: usize::MAX,
                max_x: usize::MIN,
                max_y: usize::MIN,
            }
        }

        pub fn fill_line(&mut self, start: Point, end: Point) {
            self.min_x = std::cmp::min(end.0, std::cmp::min(start.0, self.min_x));
            self.min_y = std::cmp::min(end.1, std::cmp::min(start.1, self.min_y));
            self.max_x = std::cmp::max(end.0, std::cmp::max(start.0, self.max_x));
            self.max_y = std::cmp::max(end.1, std::cmp::max(start.1, self.max_y));

            if start.0 == end.0 {
                // Vertical line
                for y in std::cmp::min(start.1, end.1)..=std::cmp::max(start.1, end.1) {
                    self.grid.insert((start.0, y), Tile::Rock);
                }
            } else {
                // Horizontal line
                for x in std::cmp::min(start.0, end.0)..=std::cmp::max(start.0, end.0) {
                    self.grid.insert((x, start.1), Tile::Rock);
                }
            }
        }

        pub fn draw(&self) {
            for y in self.min_y..=self.max_y {
                for x in self.min_x..=self.max_x {
                    let ch = match self.grid.get(&(x, y)) {
                        Some(&Tile::Rock) => '#',
                        Some(&Tile::Sand) => 'o',
                        None => ' ',
                    };

                    print!("{}", ch);
                }

                println!();
            }
        }

        // Returns true if the sand settled.  False if it fell forever.
        pub fn drop_sand(&mut self, origin: Point) -> bool {
            let mut sand_position = origin;

            loop {
                let next_position = (sand_position.0, sand_position.1 + 1);

                if next_position.1 > self.max_y {
                    // Into the abyss!
                    return false;
                }

                if self.grid.contains_key(&next_position) {
                    if !self.grid.contains_key(&(next_position.0 - 1, next_position.1)) {
                        // Head left
                        sand_position = (next_position.0 - 1, next_position.1);
                    } else if  !self.grid.contains_key(&(next_position.0 + 1, next_position.1)) {
                        // Head right
                        sand_position = (next_position.0 + 1, next_position.1);
                    } else {
                        // We're stuck!
                        self.grid.insert(sand_position, Tile::Sand);
                        return true;
                    }
                } else {
                    // Keep falling
                    sand_position = next_position;
                }
            }

        }
    }

    pub fn part1() {
        let mut cave = CaveWithAbyss::new();

        for line in input_lines("input_files/day14.txt") {
            let points: Vec<(usize, usize)> = line.split(" -> ").map(|s| s.split(',').map(|n| n.parse::<usize>().unwrap()).collect_tuple().unwrap()).collect();

            for i in 0..(points.len() - 1) {
                let start_point = points[i];
                let end_point = points[i + 1];

                cave.fill_line(start_point, end_point);
            }
        }

        let mut sand_count = 0;

        loop {
            if cave.drop_sand((500, 0)) {
                // Neat.
                sand_count += 1;
            } else {
                // Sand tumbling into the abyss
                break;
            }
        }

        println!("Sand dropped: {}", sand_count);
    }

    #[derive(Clone)]
    struct CaveWithFloor {
        grid: HashMap<Point, Tile>,
        min_x: usize,
        max_x: usize,
        min_y: usize,
        max_y: usize,
    }

    impl CaveWithFloor {
        pub fn new() -> CaveWithFloor {
            CaveWithFloor {
                grid: HashMap::new(),
                min_x: usize::MAX,
                min_y: usize::MAX,
                max_x: usize::MIN,
                max_y: usize::MIN,
            }
        }

        pub fn fill_line(&mut self, start: Point, end: Point) {
            self.min_x = std::cmp::min(end.0, std::cmp::min(start.0, self.min_x));
            self.min_y = std::cmp::min(end.1, std::cmp::min(start.1, self.min_y));
            self.max_x = std::cmp::max(end.0, std::cmp::max(start.0, self.max_x));
            self.max_y = std::cmp::max(end.1, std::cmp::max(start.1, self.max_y));

            if start.0 == end.0 {
                // Vertical line
                for y in std::cmp::min(start.1, end.1)..=std::cmp::max(start.1, end.1) {
                    self.grid.insert((start.0, y), Tile::Rock);
                }
            } else {
                // Horizontal line
                for x in std::cmp::min(start.0, end.0)..=std::cmp::max(start.0, end.0) {
                    self.grid.insert((x, start.1), Tile::Rock);
                }
            }
        }

        pub fn draw(&self) {
            // Recompute x bounds for display to show the whole thing
            let min_x = self.grid.keys().map(|p| p.0).min().unwrap();
            let max_x = self.grid.keys().map(|p| p.0).max().unwrap();

            let min_y = 0;

            for y in min_y..=(self.max_y + 2) {
                for x in min_x..=max_x {
                    let ch = match self.grid.get(&(x, y)) {
                        Some(&Tile::Rock) => '#',
                        Some(&Tile::Sand) => 'o',
                        None => ' ',
                    };

                    print!("{}", ch);
                }

                println!();
            }
        }

        // Returns true if the sand settled.  False if it fell forever.
        pub fn drop_sand(&mut self, origin: Point) -> bool {
            let mut sand_position = origin;

            loop {
                let next_position = (sand_position.0, sand_position.1 + 1);

                if next_position.1 == (self.max_y + 2) {
                    // We've hit the floor
                    self.grid.insert(sand_position, Tile::Sand);
                    return true;
                }

                if self.grid.contains_key(&next_position) {
                    if !self.grid.contains_key(&(next_position.0 - 1, next_position.1)) {
                        // Head left
                        sand_position = (next_position.0 - 1, next_position.1);
                    } else if  !self.grid.contains_key(&(next_position.0 + 1, next_position.1)) {
                        // Head right
                        sand_position = (next_position.0 + 1, next_position.1);
                    } else {
                        // We're stuck!
                        self.grid.insert(sand_position, Tile::Sand);

                        return sand_position != origin;
                    }
                } else {
                    // Keep falling
                    sand_position = next_position;
                }
            }

        }
    }

    pub fn part2() {
        let mut cave = CaveWithFloor::new();

        for line in input_lines("input_files/day14.txt") {
            let points: Vec<(usize, usize)> = line.split(" -> ").map(|s| s.split(',').map(|n| n.parse::<usize>().unwrap()).collect_tuple().unwrap()).collect();

            for i in 0..(points.len() - 1) {
                let start_point = points[i];
                let end_point = points[i + 1];

                cave.fill_line(start_point, end_point);
            }
        }

        let mut sand_count = 0;

        loop {
            sand_count += 1;

            if cave.drop_sand((500, 0)) {
                // Neat.
            } else {
                // We're full!
                break;
            }
        }

        println!("Sand dropped: {}", sand_count);
    }

    pub fn dump_pbm() {
        let mut cave = CaveWithFloor::new();

        for line in input_lines("input_files/day14.txt") {
            let points: Vec<(usize, usize)> = line.split(" -> ").map(|s| s.split(',').map(|n| n.parse::<usize>().unwrap()).collect_tuple().unwrap()).collect();

            for i in 0..(points.len() - 1) {
                let start_point = points[i];
                let end_point = points[i + 1];

                cave.fill_line(start_point, end_point);
            }
        }

        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;

        let mut stdout = io::stdout().lock();

        for round in (0..2) {
            let mut cave = cave.clone();

            let mut frame = 0;
            loop {
                frame += 1;
                if cave.drop_sand((500, 0)) {
                    if round == 1 && frame % 32 == 0 {
                        stdout.write_all(format!("P6\n{} {}\n255\n",
                                                 (max_x - min_x) + 1,
                                                 (max_y - min_y) + 1)
                                         .as_bytes())
                            .unwrap();

                        for y in min_y..=max_y {
                            for x in min_x..=max_x {
                                stdout.write_all(match cave.grid.get(&(x, y)) {
                                    Some(&Tile::Rock) => &[0xa5, 0x2a, 0x2a],
                                    Some(&Tile::Sand) => &[0xfb, 0xbf, 0x77],
                                    None =>  &[0x00, 0x00, 0x00],
                                }).unwrap();
                            }
                        }
                    }
                } else {
                    // We're full!
                    break;
                }
            }

            if round == 0 {
                min_x = cave.grid.keys().map(|p| p.0).min().unwrap();
                max_x = cave.grid.keys().map(|p| p.0).max().unwrap();
                min_y = cave.grid.keys().map(|p| p.1).min().unwrap();
                max_y = cave.grid.keys().map(|p| p.1).max().unwrap();
            }
        }
    }

}


mod day15 {
    use crate::shared::*;

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    struct Position {
        x: i64,
        y: i64,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    enum Tile {
        Sensor,
        SensorHighlight,
        Beacon,
        Signal,
    }

    fn draw_world(world: &HashMap<Position, Tile>) {
        let min_x = world.keys().map(|p| p.x).min().unwrap();
        let max_x = world.keys().map(|p| p.x).max().unwrap();
        let min_y = world.keys().map(|p| p.y).min().unwrap();
        let max_y = world.keys().map(|p| p.y).max().unwrap();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let ch = match world.get(&Position { x, y }) {
                    Some(&Tile::Beacon) => 'B',
                    Some(&Tile::Sensor) => 'S',
                    Some(&Tile::Signal) => '#',
                    Some(&Tile::SensorHighlight) => 'X',
                    None => ' ',
                };

                print!("{}", ch);
            }

            println!();
        }
    }

    fn manhattan_distance(p1: Position, p2: Position) -> i64 {
        (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
    }

    pub fn part1() {
        let line_regex = Regex::new(r"^Sensor at x=([-0-9]+), y=([-0-9]+): closest beacon is at x=([-0-9]+), y=([-0-9]+)$").unwrap();

        let mut world: HashMap<Position, Tile> = HashMap::new();

        let target_row = 2_000_000;

        for line in input_lines("input_files/day15.txt") {
            if let Some(caps) = line_regex.captures(&line) {
                let sensor = Position {
                    x: caps[1].parse::<i64>().unwrap(),
                    y: caps[2].parse::<i64>().unwrap(),
                };

                let beacon = Position {
                    x: caps[3].parse::<i64>().unwrap(),
                    y: caps[4].parse::<i64>().unwrap(),
                };

                world.insert(sensor, Tile::Sensor);
                world.insert(beacon, Tile::Beacon);


                let max_distance = manhattan_distance(sensor, beacon);

                if (sensor.y + max_distance) < target_row || (sensor.y - max_distance) > target_row {
                    // Not of interest.  Cull this point.
                } else {
                    for x in (sensor.x - max_distance)..=(sensor.x + max_distance) {
                        if manhattan_distance(Position { x, y: target_row }, sensor) <= max_distance {
                            // In range
                            world.entry(Position { x, y: target_row }).or_insert(Tile::Signal);
                        }
                    }
                }
            }
        }

        let min_x = world.keys().map(|p| p.x).min().unwrap();
        let max_x = world.keys().map(|p| p.x).max().unwrap();

        println!("Impossible positions: {}", (min_x..=max_x).filter(|&x| world.get(&Position { x, y: target_row }) == Some(&Tile::Signal)).count());
    }

    fn inclusive_range(a: i64, b: i64) -> Vec<i64> {
        if a < b {
            (a..=b).collect()
        } else {
            (b..=a).rev().collect()
        }
    }

    pub fn part2() {
        let line_regex = Regex::new(r"^Sensor at x=([-0-9]+), y=([-0-9]+): closest beacon is at x=([-0-9]+), y=([-0-9]+)$").unwrap();

        struct Diamond {
            origin: Position,
            max_distance: i64,
            x_min: i64,
            x_max: i64,
            y_min: i64,
            y_max: i64,
        }

        impl Diamond {
            fn contains(&self, x: i64, y: i64) -> bool {
                ((self.origin.x - x).abs() + (self.origin.y - y).abs()) <= self.max_distance
            }
        }

        let mut diamonds: Vec<Diamond> = Vec::new();

        for line in input_lines("input_files/day15.txt") {
            if let Some(caps) = line_regex.captures(&line) {
                let sensor = Position {
                    x: caps[1].parse::<i64>().unwrap(),
                    y: caps[2].parse::<i64>().unwrap(),
                };

                let beacon = Position {
                    x: caps[3].parse::<i64>().unwrap(),
                    y: caps[4].parse::<i64>().unwrap(),
                };


                let max_distance = manhattan_distance(sensor, beacon);

                diamonds.push(Diamond {
                    origin: sensor,
                    max_distance,
                    x_min: (sensor.x - max_distance),
                    x_max: (sensor.x + max_distance),
                    y_min: (sensor.y - max_distance),
                    y_max: (sensor.y + max_distance),
                });
            }
        }

        let min_coord = 0;
        let max_coord = 4000000;

        // Idea: Our search only contains one position not reached by a sensor.  This
        // must be right on the edge of one of the diamonds, so only check those
        // positions.
        for diamond in &diamonds {
            for (a, b, c, d) in &[
                (diamond.x_min - 1, diamond.origin.x, diamond.origin.y, diamond.y_min - 1),
                (diamond.origin.x, diamond.x_max + 1, diamond.y_min - 1, diamond.origin.y),
                (diamond.x_min - 1, diamond.origin.x, diamond.origin.y, diamond.y_max + 1),
                (diamond.origin.x, diamond.x_max + 1, diamond.y_max + 1, diamond.origin.y)
            ] {
                for (x, y) in inclusive_range(*a, *b).into_iter().zip(inclusive_range(*c, *d)) {
                    if x < min_coord || x > max_coord || y < min_coord || y > max_coord {
                        continue;
                    }

                    if !diamonds.iter().any(|d| d.contains(x, y)) {
                        println!("Found frequency: {}", (x * 4000000) + y);
                        return;
                    }
                }

            }
        }
    }
}

mod day16 {
    use crate::shared::*;

    #[derive(Clone, Eq, PartialEq, Hash, Debug)]
    struct Valve {
        id: usize,
        name: String,
        flow_rate: usize,
        tunnels: Vec<String>,
    }

    struct Valves {
        valves: Vec<Valve>,
        valves_by_id: HashMap<usize, Valve>,
        valves_by_name: HashMap<String, Valve>,
    }

    impl Valves {
        fn all(&self) -> &[Valve] {
            &self.valves
        }

        fn len(&self) -> usize {
            self.valves_by_id.len()
        }

        fn pressure_per_minute(&self, open_valves_mask: usize) -> usize {
            let mut total = 0;

            for (id, valve) in &self.valves_by_id {
                if (open_valves_mask & (1 << id)) > 0 {
                    total += valve.flow_rate;
                }
            }

            total
        }

        fn get(&self, id: usize) -> &Valve {
            self.valves_by_id.get(&id).unwrap()
        }

        fn get_by_name(&self, name: &str) -> &Valve {
            self.valves_by_name.get(name).unwrap()
        }
    }

    fn parse_valves() -> Valves {
        let line_regex = Regex::new(r"^Valve ([A-Z]+) has flow rate=([0-9]+); tunnels? leads? to valves? (.*)$").unwrap();

        let mut valves: Vec<Valve> = Vec::new();
        let mut valves_by_id: HashMap<usize, Valve> = HashMap::new();
        let mut valves_by_name: HashMap<String, Valve> = HashMap::new();

        let mut id = 0;

        for line in input_lines("input_files/day16.txt") {
            if let Some(caps) = line_regex.captures(&line) {
                let valve = Valve {
                    id,
                    name: caps[1].to_string(),
                    flow_rate: caps[2].parse().unwrap(),
                    tunnels: caps[3].split(", ").map(str::to_string).collect(),
                };

                id += 1;

                valves.push(valve.clone());
                valves_by_id.insert(valve.id, valve.clone());
                valves_by_name.insert(valve.name.clone(), valve);
            } else {
                panic!("Input parse error: {}", line);
            }
        }

        Valves { valves, valves_by_id, valves_by_name }
    }

    // Floyd-Warshall
    fn build_dist_map(valves: &Valves) -> Vec<Vec<usize>> {
        let mut dist: Vec<Vec<usize>> = (0..valves.len()).map(|_| vec![usize::MAX; valves.len()]).collect();

        for valve in valves.all() {
            for other_valve_name in &valve.tunnels {
                let other_valve = valves.get_by_name(other_valve_name);

                dist[valve.id][other_valve.id] = 1;
                dist[other_valve.id][valve.id] = 1;
            }

            dist[valve.id][valve.id] = 0;
        }

        for k in valves.all() {
            for i in valves.all() {
                for j in valves.all() {
                    if dist[i.id][j.id] > (dist[i.id][k.id].saturating_add(dist[k.id][j.id])) {
                        dist[i.id][j.id] = (dist[i.id][k.id] + dist[k.id][j.id]);
                    }
                }
            }
        }

        dist
    }

    fn solve(valves: &Valves,
             dist_map: &Vec<Vec<usize>>,
             minutes_remaining: usize,
             current_location: usize,
             open_valves_bitset: usize,
             total_pressure_released: usize,
             scoreboard: &mut HashMap<usize, usize>) -> usize {
        let this_valve = valves.get(current_location);

        let mut max_pressure_released = total_pressure_released + (minutes_remaining * valves.pressure_per_minute(open_valves_bitset));

        // If this is our best score for this set of valves, record it.
        {
            let entry = scoreboard.entry(open_valves_bitset).or_insert(max_pressure_released);

            if max_pressure_released > *entry {
                *entry = max_pressure_released;
            }
        }

        if minutes_remaining == 0 {
            return max_pressure_released;
        }

        for target_valve in valves.all().iter().filter(|v| v.flow_rate > 0 && open_valves_bitset & (1 << v.id) == 0) {
            if (minutes_remaining - 1) < dist_map[this_valve.id][target_valve.id] {
                continue;
            }

            let move_cost = dist_map[this_valve.id][target_valve.id] + 1;

            let pressure = solve(valves, dist_map,
                                 minutes_remaining - move_cost,
                                 target_valve.id,
                                 open_valves_bitset | (1 << target_valve.id),
                                 (valves.pressure_per_minute(open_valves_bitset) * move_cost) + total_pressure_released,
                                 scoreboard);

            if pressure > max_pressure_released {
                max_pressure_released = pressure;
            }
        }

        max_pressure_released
    }

    pub fn part1() {
        let valves = parse_valves();

        let dist_map = build_dist_map(&valves);

        let mut scoreboard = HashMap::new();

        let best_score = solve(&valves, &dist_map, 30, valves.get_by_name("AA").id, 0, 0, &mut scoreboard);

        println!("Max pressure released: {}", best_score);
    }

    pub fn part2() {
        let valves = parse_valves();

        let dist_map = build_dist_map(&valves);

        let mut scoreboard = HashMap::new();

        solve(&valves, &dist_map, 26, valves.get_by_name("AA").id, 0, 0, &mut scoreboard);

        let mut best = 0;

        for state1 in scoreboard.keys() {
            for state2 in scoreboard.keys() {
                if state1 & state2 == 0 {
                    // non-overlapping.  good
                    let combined_score = scoreboard.get(state1).unwrap() + scoreboard.get(state2).unwrap();

                    if combined_score > best {
                        best = combined_score;
                    }
                }
            }
        }

        println!("Best combined score: {}", best);
    }
}

mod day17 {
    use once_cell::sync::OnceCell;

    use crate::shared::*;

    #[derive(Debug)]
    struct Shape {
        pattern: Vec<Vec<char>>,
        rock_coordinates: Vec<Point>,
    }

    impl Shape {
        fn new(rows: &[&str]) -> Shape {
            let pattern: Vec<Vec<char>> = rows.iter().map(|row| row.chars().collect()).collect();

            let row_count = pattern.len();

            let rock_coordinates = (0..pattern.len()).flat_map(|y| {
                pattern[y].iter().enumerate().flat_map(move |(idx, &ch)| {
                    if ch == '#' {
                        Some(Point { x: idx as i64, y: (row_count - y - 1) as i64 })
                    } else {
                        None
                    }
                })
            }).collect();


            Shape { pattern, rock_coordinates }
        }

        fn height(&self) -> usize {
            self.pattern.len()
        }
    }

    fn shapes() -> &'static [Shape] {
        static SHAPES: OnceCell<Vec<Shape>> = OnceCell::new();

        SHAPES.get_or_init(|| {
            vec!(
                Shape::new(&["####"]),

                Shape::new(&[".#.",
                             "###",
                             ".#."]),

                Shape::new(&["..#",
                             "..#",
                             "###"]),

                Shape::new(&["#",
                             "#",
                             "#",
                             "#"]),

                Shape::new(&["##",
                             "##"]),
            )})
    }

    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
    struct Point { x: i64, y: i64 }

    impl Point {
        fn add(&self, adjustment: Point) -> Point {
            Point { x: self.x + adjustment.x, y: self.y + adjustment.y }
        }
    }

    #[derive(Clone)]
    struct Chamber {
        width: usize,
        height: usize,
        grid: HashSet<Point>,
    }

    impl Chamber {
        fn of_width(width: usize) -> Chamber {
            Chamber {
                width,
                height: 0,
                grid: HashSet::new(),
            }
        }

        fn can_place(&self, shape: &Shape, position: Point) -> bool {
            for p in &shape.rock_coordinates {
                let candidate = position.add(*p);

                if candidate.x < 0 || candidate.x >= self.width as i64 || candidate.y < 0 || self.grid.contains(&candidate) {
                    return false;
                }
            }

            true
        }

        fn place_shape(&mut self, shape: &Shape, position: Point) {
            for p in &shape.rock_coordinates {
                let absolute_position = position.add(*p);

                if (absolute_position.y + 1) > self.height.try_into().unwrap() {
                    self.height = (absolute_position.y + 1) as usize;
                }

                self.grid.insert(absolute_position);
            }
        }

        fn print_with_active(&self, shape: &Shape, position: Point) {
            let mut printme = self.clone();

            printme.place_shape(shape, position);

            printme.print();
        }

        fn print(&self) {
            println!();
            for y in (0..=self.height).rev() {
                for x in (0..self.width) {
                    if self.grid.contains(&Point { x: x as i64, y: y as i64 }) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }

                println!();
            }
        }
    }

    pub fn part1() {
        let jets: Vec<char> = read_file("input_files/day17.txt").chars().collect();

        let mut chamber = Chamber::of_width(7);

        let mut shape_count = 0;
        let mut tick_count = 0;

        let all_shapes = shapes();

        loop {
            let falling_shape = &all_shapes[shape_count % all_shapes.len()];
            let mut position = Point { x: 2, y: (chamber.height + 3) as i64 };

            // chamber.print_with_active(falling_shape, position);

            // Iterate this shape until it settles
            loop {
                let jet_adjustment = match &jets[tick_count % jets.len()] {
                    '<' => Point { x: -1, y: 0},
                    '>' => Point { x: 1, y: 0 },
                    _ => unreachable!(),
                };

                // Move left/right
                if chamber.can_place(falling_shape, position.add(jet_adjustment)) {
                    position = position.add(jet_adjustment);
                }

                tick_count += 1;

                // Move down
                let down_adjustment = Point { x: 0, y: -1 };
                if chamber.can_place(falling_shape, position.add(down_adjustment)) {
                    position = position.add(down_adjustment);
                } else {
                    // We're now settled.  Lock it in.
                    chamber.place_shape(falling_shape, position);
                    break;
                }
            }

            shape_count += 1;

            // chamber.print();
            if shape_count == 2022 {
                break;
            }
        }

        println!("Chamber is {} units tall", chamber.height);
    }

    pub fn part2() {
        let jets: Vec<char> = read_file("input_files/day17.txt").chars().collect();

        let mut chamber = Chamber::of_width(7);

        let mut shape_count = 0;
        let mut tick_count = 0;

        let all_shapes = shapes();

        let mut last_height = 0;
        // let mut differences = Vec::new();

        loop {
            if shape_count % 347000 == 0 {
                // println!("Height at {}: {}", shape_count, chamber.height);

                if last_height > 0 {
                    println!("Height: {}", chamber.height);
                    println!("Difference: {}", chamber.height - last_height);
                    // differences.push(chamber.height - last_height);
                }

                last_height = chamber.height;
            }

            let falling_shape = &all_shapes[shape_count % all_shapes.len()];
            let mut position = Point { x: 2, y: (chamber.height + 3) as i64 };

            // chamber.print_with_active(falling_shape, position);

            // Iterate this shape until it settles
            loop {
                let jet_adjustment = match &jets[tick_count % jets.len()] {
                    '<' => Point { x: -1, y: 0},
                    '>' => Point { x: 1, y: 0 },
                    _ => unreachable!(),
                };

                // Move left/right
                if chamber.can_place(falling_shape, position.add(jet_adjustment)) {
                    position = position.add(jet_adjustment);
                }

                tick_count += 1;

                // Move down
                let down_adjustment = Point { x: 0, y: -1 };
                if chamber.can_place(falling_shape, position.add(down_adjustment)) {
                    position = position.add(down_adjustment);
                } else {
                    // We're now settled.  Lock it in.
                    chamber.place_shape(falling_shape, position);
                    break;
                }
            }

            shape_count += 1;

            // chamber.print();
            // if shape_count == 1_200_000 {
            //     break;
            // }

            if shape_count == 132000 {
                break;
            }


        }

        println!("Done at {}", chamber.height);

        // let mut xor = 0;
        // for (idx, t) in differences.iter().enumerate() {
        //     xor ^= t;
        //
        //     if xor == 0 {
        //         if (idx + 1) % 2 == 0 {
        //             if differences[0..(idx + 1) / 2] == differences[(idx + 1) / 2.. idx + 1] {
        //                 println!("Found a cycle: {}", idx);
        //             }
        //         }
        //     }
        // }
        //
        // println!();
        //
        // for n in &differences[0..347] {
        //     println!("{}", n);
        // }
        //
        // println!();
        //
        // for n in &differences[347..694] {
        //     println!("{}", n);
        // }

    }
}

mod day18 {
    use crate::shared::*;

    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
    struct Point { x: i64, y: i64, z: i64 }

    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
    struct CubeFace {
        diag_pt1: Point,
        diag_pt2: Point,
    }

    impl CubeFace {
        fn from(p1: Point, p2: Point) -> CubeFace {
            if p1 <= p2 {
                CubeFace {
                    diag_pt1: p1,
                    diag_pt2: p2,
                }
            } else {
                Self::from(p2, p1)
            }
        }

        fn min_x(&self) -> i64 {
            std::cmp::min(self.diag_pt1.x, self.diag_pt2.x)
        }

        fn min_y(&self) -> i64 {
            std::cmp::min(self.diag_pt1.y, self.diag_pt2.y)
        }

        fn min_z(&self) -> i64 {
            std::cmp::min(self.diag_pt1.z, self.diag_pt2.z)
        }

        fn max_x(&self) -> i64 {
            std::cmp::max(self.diag_pt1.x, self.diag_pt2.x)
        }

        fn max_y(&self) -> i64 {
            std::cmp::max(self.diag_pt1.y, self.diag_pt2.y)
        }

        fn max_z(&self) -> i64 {
            std::cmp::max(self.diag_pt1.z, self.diag_pt2.z)
        }

        fn shift(&self, transform: (i64, i64, i64)) -> CubeFace {
            CubeFace::from(Point {
                x: self.diag_pt1.x + transform.0,
                y: self.diag_pt1.y + transform.1,
                z: self.diag_pt1.z + transform.2,
            }, Point {
                x: self.diag_pt2.x + transform.0,
                y: self.diag_pt2.y + transform.1,
                z: self.diag_pt2.z + transform.2,
            })
        }
    }

    fn cube_faces(x: i64, y: i64, z: i64) -> Vec<CubeFace> {
        vec![
            // left face
            CubeFace::from(Point { x, y, z },
                           Point { x, y: y + 1, z: z + 1 }),
            // front face
            CubeFace::from(Point { x, y, z },
                           Point { x: x + 1, y: y + 1, z }),
            // bottom face
            CubeFace::from(Point { x, y, z },
                           Point { x: x + 1, y, z: z + 1 }),
            // top face
            CubeFace::from(Point { x, y: y + 1, z },
                           Point { x: x + 1, y: y + 1, z: z + 1 }),
            // back face
            CubeFace::from(Point { x, y, z: z + 1 },
                           Point { x: x + 1, y: y + 1, z: z + 1 }),
            // right face
            CubeFace::from(Point { x: x + 1, y, z },
                           Point { x: x + 1, y: y + 1, z: z + 1 }),
        ]

    }

    pub fn part1() {
        let mut face_counts: HashMap<CubeFace, usize> = HashMap::new();

        for line in input_lines("input_files/day18.txt") {
            let (x, y, z) = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect_tuple().unwrap();

            for face in cube_faces(x, y, z) {
                let entry = face_counts.entry(face).or_insert(0);
                *entry += 1;
            }
        }

        let mut total = 0;

        for (_face, count) in face_counts {
            if count == 1 {
                total += 1;
            }
        }

        println!("Total surface area: {}", total);
    }

    // This didn't work!  My idea was to project a 2d plane of "rays" in each of the
    // 6 possible directions, marking which faces were reached.
    //
    // This doesn't work for shapes like:
    //
    //    X X
    //    XXX
    //
    // Since the inner faces between the two two cubes *are* reachable, but won't be
    // hit by a l-to-r or r-to-l scan.
    pub fn failed_part2() {
        let mut face_counts: HashMap<CubeFace, usize> = HashMap::new();

        for line in input_lines("input_files/day18_sample.txt") {
            let (x, y, z) = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect_tuple().unwrap();

            for face in cube_faces(x, y, z) {
                let entry = face_counts.entry(face).or_insert(0);
                *entry += 1;
            }
        }

        let all_faces: HashSet<CubeFace> = face_counts.keys().filter(|k| face_counts.get(k) == Some(&1)).cloned().collect();

        let min_x = all_faces.iter().map(|f| f.min_x()).min().unwrap();
        let min_y = all_faces.iter().map(|f| f.min_y()).min().unwrap();
        let min_z = all_faces.iter().map(|f| f.min_z()).min().unwrap();

        let max_x = all_faces.iter().map(|f| f.max_x()).max().unwrap();
        let max_y = all_faces.iter().map(|f| f.max_y()).max().unwrap();
        let max_z = all_faces.iter().map(|f| f.max_z()).max().unwrap();

        let mut faces_reached: HashSet<CubeFace> = HashSet::new();

        // Sweep from left to right
        {
            let mut face_rays: HashSet<CubeFace> = HashSet::new();
            for x in min_x..=min_x {
                for y in min_y..=max_y {
                    for z in min_z..=max_z {
                        face_rays.insert(CubeFace::from(Point { x, y, z}, Point { x, y: y + 1, z: z + 1 }));
                    }
                }
            }

            for _pass in min_x..=max_x {
                let hits: Vec<CubeFace> = face_rays.intersection(&all_faces).cloned().collect();

                for hit in hits {
                    face_rays.remove(&hit);
                    faces_reached.insert(hit);
                }

                face_rays = face_rays.into_iter().map(|face| face.shift((1, 0, 0))).collect();
            }
        }

        // Sweep from right to left
        {
            let mut face_rays: HashSet<CubeFace> = HashSet::new();
            for x in max_x..=max_x {
                for y in min_y..=max_y {
                    for z in min_z..=max_z {
                        face_rays.insert(CubeFace::from(Point { x, y, z}, Point { x, y: y + 1, z: z + 1 }));
                    }
                }
            }

            for _pass in min_x..=max_x {
                let hits: Vec<CubeFace> = face_rays.intersection(&all_faces).cloned().collect();

                for hit in hits {
                    face_rays.remove(&hit);
                    faces_reached.insert(hit);
                }

                face_rays = face_rays.into_iter().map(|face| face.shift((-1, 0, 0))).collect();
            }
        }

        // Sweep from top to bottom
        {
            let mut face_rays: HashSet<CubeFace> = HashSet::new();
            for x in min_x..=max_x {
                for y in max_y..=max_y {
                    for z in min_z..=max_z {
                        face_rays.insert(CubeFace::from(Point { x, y, z}, Point { x: x + 1, y, z: z + 1 }));
                    }
                }
            }

            for _pass in min_y..=max_y {
                let hits: Vec<CubeFace> = face_rays.intersection(&all_faces).cloned().collect();

                for hit in hits {
                    face_rays.remove(&hit);
                    faces_reached.insert(hit);
                }

                face_rays = face_rays.into_iter().map(|face| face.shift((0, -1, 0))).collect();
            }
        }

        // Sweep from bottom to top
        {
            let mut face_rays: HashSet<CubeFace> = HashSet::new();
            for x in min_x..=max_x {
                for y in min_y..=min_y {
                    for z in min_z..=max_z {
                        face_rays.insert(CubeFace::from(Point { x, y, z}, Point { x: x + 1, y, z: z + 1 }));
                    }
                }
            }

            for _pass in min_y..=max_y {
                let hits: Vec<CubeFace> = face_rays.intersection(&all_faces).cloned().collect();

                for hit in hits {
                    face_rays.remove(&hit);
                    faces_reached.insert(hit);
                }

                face_rays = face_rays.into_iter().map(|face| face.shift((0, 1, 0))).collect();
            }
        }

        // Sweep from front to back
        {
            let mut face_rays: HashSet<CubeFace> = HashSet::new();
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    for z in min_z..=min_z {
                        face_rays.insert(CubeFace::from(Point { x, y, z}, Point { x: x + 1, y: y + 1, z }));
                    }
                }
            }

            for _pass in min_z..=max_z {
                let hits: Vec<CubeFace> = face_rays.intersection(&all_faces).cloned().collect();

                for hit in hits {
                    face_rays.remove(&hit);
                    faces_reached.insert(hit);
                }

                face_rays = face_rays.into_iter().map(|face| face.shift((0, 0, 1))).collect();
            }
        }

        // Sweep from back to front
        {
            let mut face_rays: HashSet<CubeFace> = HashSet::new();
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    for z in max_z..=max_z {
                        face_rays.insert(CubeFace::from(Point { x, y, z}, Point { x: x + 1, y: y + 1, z }));
                    }
                }
            }

            for _pass in min_z..=max_z {
                let hits: Vec<CubeFace> = face_rays.intersection(&all_faces).cloned().collect();

                for hit in hits {
                    face_rays.remove(&hit);
                    faces_reached.insert(hit);
                }

                face_rays = face_rays.into_iter().map(|face| face.shift((0, 0, -1))).collect();
            }
        }

        dbg!(faces_reached.len());
    }



    pub fn part2() {
        let mut face_counts: HashMap<CubeFace, usize> = HashMap::new();
        let mut cubes: HashSet<Point> = HashSet::new();

        for line in input_lines("input_files/day18.txt") {
            let (x, y, z) = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect_tuple().unwrap();

            cubes.insert(Point { x, y, z });

            for face in cube_faces(x, y, z) {
                let entry = face_counts.entry(face).or_insert(0);
                *entry += 1;
            }
        }

        let all_faces: HashSet<CubeFace> = face_counts.keys().filter(|k| face_counts.get(k) == Some(&1)).cloned().collect();

        let min_x = all_faces.iter().map(|f| f.min_x()).min().unwrap();
        let min_y = all_faces.iter().map(|f| f.min_y()).min().unwrap();
        let min_z = all_faces.iter().map(|f| f.min_z()).min().unwrap();

        let max_x = all_faces.iter().map(|f| f.max_x()).max().unwrap();
        let max_y = all_faces.iter().map(|f| f.max_y()).max().unwrap();
        let max_z = all_faces.iter().map(|f| f.max_z()).max().unwrap();

        let mut queue: VecDeque<Point> = VecDeque::new();

        queue.push_back(Point { x: min_x - 1, y: min_y - 1, z: min_z - 1 });

        let mut filled: HashSet<Point> = HashSet::new();
        let mut reached_faces: HashSet<CubeFace> = HashSet::new();

        while !queue.is_empty() {
            let candidate_point = queue.pop_back().unwrap();

            if cubes.contains(&candidate_point) || filled.contains(&candidate_point) {
                // Can't place it here
            } else {
                for face in cube_faces(candidate_point.x, candidate_point.y, candidate_point.z) {
                    if all_faces.contains(&face) {
                        reached_faces.insert(face);
                    }
                }

                for &x_off in &[-1, 0, 1] {
                    for &y_off in &[-1, 0, 1] {
                        for &z_off in &[-1, 0, 1] {
                            if (x_off * x_off + y_off * y_off + z_off * z_off) != 1 {
                                continue;
                            }

                            let x_new = candidate_point.x + x_off as i64;
                            let y_new = candidate_point.y + y_off as i64;
                            let z_new = candidate_point.z + z_off as i64;

                            if (x_new < (min_x - 1) || y_new < (min_y - 1)  || z_new < (min_z - 1)) || (x_new > (max_x + 1) || y_new > (max_y + 1)  || z_new > (max_z + 1)) {
                                // Out of bounds
                            } else {
                                queue.push_back(Point {
                                    x: x_new,
                                    y: y_new,
                                    z: z_new,
                                });
                            }
                        }
                    }
                }

                filled.insert(candidate_point);
            }
        }

        dbg!(&reached_faces.len());
    }

}

mod day19 {
    use crate::shared::*;
    use self::Material::*;
    use enum_map::{enum_map, Enum, EnumMap};

    #[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd, Enum, Hash)]
    enum Material {
        Ore,
        Clay,
        Obsidian,
        Geode
    }

    #[derive(Debug, Eq, PartialEq, Hash, Clone)]
    struct State {
        production_capability: EnumMap<Material, usize>,
        inventory: EnumMap<Material, usize>,
    }

    impl Default for State {
        fn default() -> Self {
            Self {
                production_capability: enum_map!(Ore => 1, _ => 0),
                inventory: Default::default()
            }
        }
    }

    #[derive(Debug)]
    struct Blueprint {
        robot_prices: EnumMap<Material, Vec<(Material, usize)>>,
    }

    impl Blueprint {
        fn max_required_materials(&self) -> EnumMap<Material, usize> {
            let mut result = EnumMap::default();

            for items in self.robot_prices.values() {
                for (material, amount) in items {
                    if *amount > result[*material] {
                        result[*material] = *amount;
                    }
                }
            }

            result
        }
    }


    fn parse_blueprint(s: &str) -> Blueprint {
        let line_regex = Regex::new(r"^Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore. Each clay robot costs ([0-9]+) ore. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian\.$").unwrap();

        if let Some(caps) = line_regex.captures(s) {
            let mut result = Blueprint {
                robot_prices: EnumMap::default(),
            };

            result.robot_prices[Ore] = vec!((Ore, caps[2].parse().unwrap()));
            result.robot_prices[Clay] = vec!((Ore, caps[3].parse().unwrap()));
            result.robot_prices[Obsidian] = vec!((Ore, caps[4].parse().unwrap()),
                                                 (Clay, caps[5].parse().unwrap()));
            result.robot_prices[Geode] = vec!((Ore, caps[6].parse().unwrap()),
                                              (Obsidian, caps[7].parse().unwrap()));

            result
        } else {
            panic!("Parse error: {}", s);
        }

    }

    #[derive(Ord, PartialOrd, PartialEq, Eq, Debug)]
    enum WaitTime {
        Minutes(usize),
        Never,
    }

    impl WaitTime {
        fn minutes(&self) -> usize {
            match self {
                WaitTime::Minutes(n) => *n,
                WaitTime::Never => panic!("Can't get minutes from NEVER"),
            }
        }
    }

    // FIXME: optimise
    #[inline(never)]
    fn wait_time_for_robot(state: &State, blueprint: &Blueprint, robot: Material) -> WaitTime {
        let mut result: usize = 0;

        for requirement in &blueprint.robot_prices[robot] {
            if state.production_capability[requirement.0] == 0 {
                // we'll never make it!
                return WaitTime::Never;
            }

            let amount_needed = requirement.1;
            let amount_on_hand = state.inventory[requirement.0];

            let shortfall = amount_needed.saturating_sub(amount_on_hand);

            result = std::cmp::max(result, (shortfall as f64 / state.production_capability[requirement.0] as f64).ceil() as usize);
        }

        WaitTime::Minutes(result)
    }


    #[derive(Debug, Eq, PartialEq)]
    struct GeodeResult {
        count: usize,
        state: State,
        minutes: usize,
    }

    fn geode_count(robot_to_build: Material, total_minutes: usize, blueprint: &Blueprint, init_minutes: usize, mut state: State) -> GeodeResult {
        let next_robot_time = wait_time_for_robot(&state, blueprint, robot_to_build);

        let mut minute = init_minutes;

        if next_robot_time != WaitTime::Never && (minute + next_robot_time.minutes()) < total_minutes {
            // accrue our inventory, simulating wait time (which is possibly zero)
            for (material, units) in state.production_capability {
                state.inventory[material] += units * (next_robot_time.minutes() + 1);
            }

            // Build our new robot
            for (material, units_required) in &blueprint.robot_prices[robot_to_build] {
                state.inventory[*material] -= units_required;
            }

            state.production_capability[robot_to_build] += 1;

            minute += (next_robot_time.minutes() + 1);
        }

        GeodeResult {
            count: state.inventory[Geode] + (total_minutes - minute) * state.production_capability[Geode],
            minutes: minute,
            state,
        }
    }


    // Spend 2 ore to start building a clay-collecting robot.
    // Spend 2 ore to start building a clay-collecting robot.
    // Spend 2 ore to start building a clay-collecting robot.
    // Spend 3 ore and 14 clay to start building an obsidian-collecting robot.
    // Spend 2 ore to start building a clay-collecting robot.
    // Spend 3 ore and 14 clay to start building an obsidian-collecting robot.
    // Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
    // Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
    // 2 geode-cracking robots crack 2 geodes; you now have 9 open geodes.

    fn best_score(counts: &mut EnumMap<Material, usize>, blueprint: &Blueprint, max_minutes: usize, minutes: usize, state: &State, last_score: usize) -> usize {
        if minutes == max_minutes {
            return last_score;
        }

        let mut max_score = last_score;

        for material in &[Ore, Clay, Obsidian, Geode] {
            let count = counts[*material];

            if count > 0 && wait_time_for_robot(state, blueprint, *material) < WaitTime::Minutes(max_minutes - minutes) {
                counts[*material] -= 1;

                let result = geode_count(*material, max_minutes, blueprint, minutes, state.clone());
                let score = best_score(counts, blueprint, max_minutes, result.minutes, &result.state, result.count);

                if score > max_score {
                    max_score = score;
                }

                counts[*material] += 1;
            }
        }

        max_score
    }


    pub fn part1() {
        for line in input_lines("input_files/day19.txt") {
            let blueprint = parse_blueprint(&line);

            let mut counts = blueprint.max_required_materials();
            counts[Geode] = usize::MAX;

            dbg!(best_score(&mut counts, &blueprint, 24, 0, &State::default(), 0));
        }
    }

    pub fn part2() {
        for line in input_lines("input_files/day19.txt").take(3) {
            let blueprint = parse_blueprint(&line);

            let mut counts = blueprint.max_required_materials();
            counts[Geode] = usize::MAX;

            dbg!(best_score(&mut counts, &blueprint, 32, 0, &State::default(), 0));
        }
    }
}


mod day20 {
    use crate::shared::*;

    #[derive(Debug)]
    struct Element {
        value: i64,
    }

    fn posmod(n: i64, d: usize) -> usize {
        let d = d as i64;
        (((n % d) + d) % d) as usize
    }

    pub fn part1() {
        let mut elements = Vec::new();
        let mut next_links = Vec::new();
        let mut prev_links = Vec::new();

        for line in input_lines("input_files/day20.txt") {
            let n: i64 = line.parse().unwrap();

            elements.push(Element { value: n });
        }

        for i in 0..elements.len() {
            let i = i as i64;
            let len = elements.len();

            next_links.push(posmod((i + 1), len));
            prev_links.push(posmod((i - 1), len));
        }

        for i in 0..elements.len() {
            let offset = elements[i].value % (elements.len() - 1) as i64;

            if offset == 0 {
                // No move
                continue;
            }

            // Work out our new neighbours
            let (new_next, new_prev) = {
                let mut idx = i;

                let negative = offset < 0;

                for _ in 0..offset.abs() {
                    if negative {
                        idx = prev_links[idx];
                    } else {
                        idx = next_links[idx];
                    }

                    if idx == i {
                        if negative {
                            idx = prev_links[idx];
                        } else {
                            idx = next_links[idx];
                        }
                    }
                }

                if negative {
                    (idx, prev_links[idx])
                } else {
                    (next_links[idx], idx)
                }
            };

            // Remove our element from the list
            {
                let old_next = next_links[i];
                let old_prev = prev_links[i];

                next_links[old_prev] = old_next;
                prev_links[old_next] = old_prev;
            }

            // Insert into the right spot
            next_links[i] = new_next;
            prev_links[i] = new_prev;

            next_links[new_prev as usize] = i as usize;
            prev_links[new_next as usize] = i as usize;
        }

        let start_idx = elements.iter().position(|elt| elt.value == 0).unwrap();

        let mut result = 0;
        let mut idx = start_idx;

        for i in 0..=3000 {
            if i == 1000 {
                dbg!(elements[idx].value);
                result += elements[idx].value;
            }
            if i == 2000 {
                dbg!(elements[idx].value);
                result += elements[idx].value;
            }
            if i == 3000 {
                dbg!(elements[idx].value);
                result += elements[idx].value;
            }

            idx = next_links[idx] as usize;
        }

        println!("Coordinates: {}", result);
    }

    pub fn part2() {
        let mut elements = Vec::new();
        let mut next_links = Vec::new();
        let mut prev_links = Vec::new();

        for line in input_lines("input_files/day20.txt") {
            let n: i64 = line.parse().unwrap();

            elements.push(Element { value: n * 811589153 });
        }

        for i in 0..elements.len() {
            let i = i as i64;
            let len = elements.len();

            next_links.push(posmod((i + 1), len));
            prev_links.push(posmod((i - 1), len));
        }

        for _ in 0..10 {
            for i in 0..elements.len() {
                let offset = elements[i].value % (elements.len() - 1) as i64;

                if offset == 0 {
                    // No move
                    continue;
                }

                // Work out our new neighbours
                let (new_next, new_prev) = {
                    let mut idx = i;

                    let negative = offset < 0;

                    for _ in 0..offset.abs() {
                        if negative {
                            idx = prev_links[idx];
                        } else {
                            idx = next_links[idx];
                        }

                        if idx == i {
                            if negative {
                                idx = prev_links[idx];
                            } else {
                                idx = next_links[idx];
                            }
                        }
                    }

                    if negative {
                        (idx, prev_links[idx])
                    } else {
                        (next_links[idx], idx)
                    }
                };

                // Remove our element from the list
                {
                    let old_next = next_links[i];
                    let old_prev = prev_links[i];

                    next_links[old_prev] = old_next;
                    prev_links[old_next] = old_prev;
                }

                // Insert into the right spot
                next_links[i] = new_next;
                prev_links[i] = new_prev;

                next_links[new_prev as usize] = i as usize;
                prev_links[new_next as usize] = i as usize;
            }
        }

        let start_idx = elements.iter().position(|elt| elt.value == 0).unwrap();

        let mut result = 0;
        let mut idx = start_idx;

        for i in 0..=3000 {
            if i == 1000 {
                dbg!(elements[idx].value);
                result += elements[idx].value;
            }
            if i == 2000 {
                dbg!(elements[idx].value);
                result += elements[idx].value;
            }
            if i == 3000 {
                dbg!(elements[idx].value);
                result += elements[idx].value;
            }

            idx = next_links[idx] as usize;
        }

        println!("Coordinates: {}", result);
    }
}


mod day21 {
    use crate::shared::*;

    #[derive(Eq, PartialEq, Hash, Clone, Debug)]
    enum Expression {
        Name(String),
        Value(usize),
    }

    #[derive(Eq, PartialEq, Hash, Clone, Debug)]
    enum Operator {
        Plus,
        Minus,
        Multiply,
        Divide,
        Equal,
    }

    #[derive(Eq, PartialEq, Hash, Clone, Debug)]
    struct Operation {
        lhs: Expression,
        rhs: Expression,
        op: Operator,
        dependencies_remaining: usize,
    }

    impl Operation {
        fn resolve_dependency(&mut self, name: &str, value: usize) {
            if self.lhs == Expression::Name(name.to_owned()) {
                self.lhs = Expression::Value(value);
                self.dependencies_remaining -= 1;
            }

            if self.rhs == Expression::Name(name.to_owned()) {
                self.rhs = Expression::Value(value);
                self.dependencies_remaining -= 1;
            }
        }

        fn evaluate(&self) -> usize {
            if let Expression::Value(lhs) = self.lhs {
                if let Expression::Value(rhs) = self.rhs {
                    if self.op == Operator::Equal {
                        dbg!(lhs, rhs);
                    }

                    match self.op {
                        Operator::Plus => lhs + rhs,
                        Operator::Minus => lhs - rhs,
                        Operator::Multiply => lhs * rhs,
                        Operator::Divide => lhs / rhs,
                        Operator::Equal => (lhs == rhs) as usize,
                    }
                } else {
                    unreachable!();
                }
            } else {
                unreachable!();
            }
        }
    }

    pub fn part1() {
        let mut monkeys: HashMap<String, Operation> = HashMap::new();
        let mut monkey_dependencies: HashMap<String, Vec<String>> = HashMap::new();

        for line in input_lines("input_files/day21.txt") {
            let (monkey, value) = line.split(": ").collect_tuple().unwrap();

            monkeys.insert(monkey.to_owned(),
                           if value.chars().next().unwrap().is_ascii_digit() {
                               // Expression value literal
                               Operation {
                                   lhs: Expression::Value(value.parse::<usize>().unwrap()),
                                   rhs: Expression::Value(1),
                                   op: Operator::Multiply,
                                   dependencies_remaining: 0,
                               }
                           } else {
                               let (lhs, op, rhs) = value.split(' ').collect_tuple().unwrap();
                               Operation {
                                   lhs: Expression::Name(lhs.to_owned()),
                                   rhs: Expression::Name(rhs.to_owned()),
                                   op: match op {
                                       "+" => Operator::Plus,
                                       "-" => Operator::Minus,
                                       "*" => Operator::Multiply,
                                       "/" => Operator::Divide,
                                       "=" => Operator::Equal,
                                       _ => unreachable!(),
                                   },
                                   dependencies_remaining: 2,
                               }
                           }
            );
        }

        // Load dependencies
        for (monkey, operation) in &monkeys {
            if operation.dependencies_remaining == 2 {
                if let Expression::Name(s) = &operation.lhs {
                    let entry = monkey_dependencies.entry(s.clone()).or_default();
                    entry.push(monkey.clone());
                } else {
                    unreachable!();
                }

                if let Expression::Name(s) = &operation.rhs {
                    let entry = monkey_dependencies.entry(s.clone()).or_default();
                    entry.push(monkey.clone());
                } else {
                    unreachable!();
                }
            }
        }


        while monkeys.get("root").unwrap().dependencies_remaining > 0 {
            let mut resolved_monkeys = Vec::new();

            for (name, operation) in &monkeys {
                if operation.dependencies_remaining == 0 {
                    resolved_monkeys.push(name.clone());
                }
            }

            for name in resolved_monkeys {
                let operation = monkeys.remove(&name).unwrap();

                let value = operation.evaluate();

                let waiting_monkeys = monkey_dependencies.remove(&name).unwrap();

                for waiter in waiting_monkeys {
                    monkeys.get_mut(&waiter).unwrap().resolve_dependency(&name, value);
                }
            }
        }

        println!("Root says: {}", monkeys.get("root").unwrap().evaluate());
    }

    fn render_tree(monkey: &Operation, monkeys: &HashMap<String, Operation>) -> String {
        let lhs = match &monkey.lhs {
            Expression::Name(name) => { if name == "humn" { "HUMAN".to_owned() } else { render_tree(monkeys.get(name).unwrap(), monkeys) } },
            Expression::Value(n) => format!("{}", n),
        };

        let rhs = match &monkey.rhs {
            Expression::Name(name) => { if name == "humn" { "HUMAN".to_owned() } else { render_tree(monkeys.get(name).unwrap(), monkeys) } },
            Expression::Value(n) => format!("{}", n),
        };

        format!("({} {} {})", lhs,
                match monkey.op {
                    Operator::Plus => "+",
                    Operator::Minus => "-",
                    Operator::Multiply => "*",
                    Operator::Divide => "/",
                    Operator::Equal => "=",
                },
                rhs)
    }

    pub fn part2() {
        let mut monkeys: HashMap<String, Operation> = HashMap::new();
        let mut monkey_dependencies: HashMap<String, Vec<String>> = HashMap::new();

        for line in input_lines("input_files/day21.txt") {
            let (monkey, value) = line.split(": ").collect_tuple().unwrap();

            monkeys.insert(monkey.to_owned(),
                           if value.chars().next().unwrap().is_ascii_digit() {
                               // Expression value literal
                               Operation {
                                   lhs: Expression::Value(value.parse::<usize>().unwrap()),
                                   rhs: Expression::Value(1),
                                   op: Operator::Multiply,
                                   dependencies_remaining: 0,
                               }
                           } else {
                               let (lhs, mut op, rhs) = value.split(' ').collect_tuple().unwrap();

                               if monkey == "root" {
                                   op = "=";
                               }

                               Operation {
                                   lhs: Expression::Name(lhs.to_owned()),
                                   rhs: Expression::Name(rhs.to_owned()),
                                   op: match op {
                                       "+" => Operator::Plus,
                                       "-" => Operator::Minus,
                                       "*" => Operator::Multiply,
                                       "/" => Operator::Divide,
                                       "=" => Operator::Equal,
                                       _ => unreachable!(),
                                   },
                                   dependencies_remaining: 2,
                               }
                           }
            );
        }

        // Load dependencies
        for (monkey, operation) in &monkeys {
            if operation.dependencies_remaining == 2 {
                if let Expression::Name(s) = &operation.lhs {
                    let entry = monkey_dependencies.entry(s.clone()).or_default();
                    entry.push(monkey.clone());
                } else {
                    unreachable!();
                }

                if let Expression::Name(s) = &operation.rhs {
                    let entry = monkey_dependencies.entry(s.clone()).or_default();
                    entry.push(monkey.clone());
                } else {
                    unreachable!();
                }
            }
        }


        // rhs is always 77247625979730
        // let mut monkeys = monkeys.clone();
        // let mut monkey_dependencies = monkey_dependencies.clone();

        // monkeys.remove("humn");

        loop {
            let mut progressed = false;
            let mut resolved_monkeys = Vec::new();

            for (name, operation) in &monkeys {
                if name != "humn" && operation.dependencies_remaining == 0 {
                    resolved_monkeys.push(name.clone());
                }
            }

            for name in resolved_monkeys {
                progressed = true;
                let operation = monkeys.remove(&name).unwrap();

                let value = operation.evaluate();

                let waiting_monkeys = monkey_dependencies.remove(&name).unwrap();

                for waiter in waiting_monkeys {
                    monkeys.get_mut(&waiter).unwrap().resolve_dependency(&name, value);
                }
            }

            if !progressed {
                break;
            }
        }

        // 3952288690726!  Thanks QALC!
        println!("{}", render_tree(monkeys.get("root").unwrap(), &monkeys));
    }
}


mod day22 {
    use crate::shared::*;

    #[derive(Debug, Eq, PartialEq, Copy, Clone)]
    enum Tile {
        Open,
        Wall,
        Nothing,
    }

    fn next_tile(grid: &Vec<Vec<Tile>>, position: (usize, usize), direction: (i64, i64)) -> ((usize, usize), Tile) {
        let mut new_row = position.0 as i64;
        let mut new_col = position.1 as i64;

        loop {
            new_col += direction.0;
            new_row += direction.1;

            if new_row >= grid.len() as i64 {
                new_row = 0;
            }

            if new_col >= grid[0].len() as i64 {
                new_col = 0;
            }

            if new_row < 0 {
                new_row += grid.len() as i64;
            }

            if new_col < 0 {
                new_col += grid[0].len() as i64;
            }


            if grid[new_row as usize][new_col as usize] == Tile::Nothing {
                // Keep moving until we hit something
            } else {
                return ((new_row as usize, new_col as usize), grid[new_row as usize][new_col as usize]);
            }
        }
    }

    fn draw_grid(grid: &Vec<Vec<Tile>>, position: (usize, usize), direction: (i64, i64)) {
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                let cell = {
                    if (row, col) == position {
                        match direction {
                            (1, 0) => '>',
                            (-1, 0) => '<',
                            (0, 1) => 'v',
                            (0, -1) => '^',
                            _ => panic!("No direction"),
                        }
                    } else {
                        match grid[row][col] {
                            Tile::Open => '.',
                            Tile::Wall => '#',
                            Tile::Nothing => ' ',
                        }
                    }
                };

                print!("{}", cell);
            }
            println!();
        }
    }

    pub fn part1() {
        let (map, description) = read_file_raw("input_files/day22.txt").split("\n\n").map(str::to_owned).collect_tuple().unwrap();

        let grid: Vec<Vec<Tile>> = {
            let map_width = map.split('\n').map(str::len).max().unwrap();

            map.split('\n').map(|line| {
                let mut row = " ".repeat(map_width);
                row.replace_range(0..line.len(), line);
                row.chars().map(|ch| match ch {
                    '.' => Tile::Open,
                    '#' => Tile::Wall,
                    ' ' => Tile::Nothing,
                    _ => panic!("Bad input"),
                }).collect()
            }).collect()
        };

        let mut position = (0, grid[0].iter().position(|&tile| tile == Tile::Open).unwrap());
        let mut direction: (i64, i64) = (1, 0);

        // println!("\nSTART");
        // draw_grid(&grid, position, direction);


        for instruction in description.trim().replace('R', "_R_").replace('L', "_L_").split('_') {
            if instruction == "R" {
                direction = match direction {
                    (1, 0) => (0, 1),
                    (0, 1) => (-1, 0),
                    (-1, 0) => (0, -1),
                    (0, -1) => (1, 0),
                    _ => panic!("Directionless!"),
                };
            } else if instruction == "L" {
                direction = match direction {
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    (-1, 0) => (0, 1),
                    (0, 1) => (1, 0),
                    _ => panic!("Directionless!"),
                };
            } else {
                let steps = instruction.parse::<usize>().unwrap();

                for _step in 0..steps {
                    let (next_tile_position, next_tile_type) = next_tile(&grid, position, direction);

                    if next_tile_type == Tile::Open {
                        position = next_tile_position;
                    } else {
                        break;
                    }
                }
            }

            // println!("\nMove: {}", instruction);
            // draw_grid(&grid, position, direction);
        }

        println!("Final password: {}",
                 (1000 * (position.0 + 1)) +
                 (4 * (position.1 + 1)) +
                 match direction {
                     (1, 0) => 0,
                     (-1, 0) => 2,
                     (0, 1) => 1,
                     (0, -1) => 3,
                     _ => panic!("No direction"),
                 });
    }

    fn draw_image(grid: &Vec<Vec<Tile>>) {
        let mut out = File::create("/home/mst/tmp/grid.pbm").unwrap();

        out.write_all(format!("P6\n{} {}\n255\n",
                                 grid[0].len(),
                                 grid.len())
                         .as_bytes())
            .unwrap();

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                out.write_all(match grid[row][col] {
                    Tile::Open => &[0x00, 0x00, 0x00],
                    Tile::Wall => &[0x00, 0x00, 0x00],
                    Tile::Nothing => &[0xFF, 0xFF, 0xFF],
                }).unwrap();
            }
        }
    }

    fn next_tile_3d(grid: &Vec<Vec<Tile>>, position: (usize, usize), direction: (i64, i64)) -> ((usize, usize), Tile, (i64, i64)) {
        let mut new_row = position.0 as i64;
        let mut new_col = position.1 as i64;
        let mut new_direction = direction;


        new_col += new_direction.0;
        new_row += new_direction.1;

        let mut needs_remap = false;

        needs_remap |= new_row >= grid.len() as i64;
        needs_remap |= new_col >= grid[0].len() as i64;
        needs_remap |= new_row < 0;
        needs_remap |= new_col < 0;
        needs_remap |= !needs_remap && grid[new_row as usize][new_col as usize] == Tile::Nothing;

        if needs_remap {
            let old_col = new_col - new_direction.0;
            let old_row = new_row - new_direction.1;
            let old_direction = new_direction;

            if false {

            } else if old_direction == (-1, 0) && old_col == 50 && (0..50).contains(&(old_row as usize)) {
                // left 10 (upper)
                new_direction = (1, 0);
                new_col = 0;
                new_row = 100 + (49 - old_row);
            } else if old_direction == (0, -1) && (0..50).contains(&old_col) && old_row == 100 {
                // up 11
                new_direction = (1, 0);
                new_col = 50;
                new_row = 50 + old_col;
            } else if old_direction == (-1, 0) && old_col == 50 && (50..100).contains(&(old_row as usize)) {
                // left 11
                new_direction = (0, 1);
                new_col = old_row - 50;
                new_row = 100;
            } else if old_direction == (1, 0) && old_col == 99  && (100..150).contains(&(old_row as usize)) {
                // right 8 (lower)
                new_direction = (-1, 0);
                new_col = 149;
                new_row = (old_row - 149).abs();
            } else if old_direction == (1, 0) && old_col == 149 && (0..50).contains(&(old_row as usize)) {
                // right 8 (upper)
                new_direction = (-1, 0);
                new_col = 99;
                new_row = (old_row - 149).abs();
            } else if old_direction == (1, 0) && old_col == 99 && (50..100).contains(&(old_row as usize)) {
                // right 7
                new_direction = (0, -1);
                new_col = old_row + 50;
                new_row = 49;
            } else if old_direction == (0, 1) && (100..150).contains(&old_col) && old_row == 49 {
                // down 7
                new_direction = (-1, 0);
                new_col = 99;
                new_row = 50 + (old_col - 100);
            } else if old_direction == (0, 1) && (50..100).contains(&old_col) && old_row == 149 {
                // down 6
                new_direction = (-1, 0);
                new_col = 49;
                new_row = 150 + (old_col - 50);
            } else if old_direction == (1, 0) && old_col == 49 && (150..200).contains(&(old_row as usize)) {
                // right 6
                new_direction = (0, -1);
                new_row = 149;
                new_col = (old_row - 150) + 50;
            } else if old_direction == (0, -1) && (50..100).contains(&old_col) && old_row == 0 {
                // up 12
                new_direction = (1, 0);
                new_col = 0;
                new_row = (old_col - 50) + 150;
            } else if old_direction == (0, 1) && (0..50).contains(&old_col) && old_row == 199 {
                // down 9
                new_direction = (0, 1);
                new_col = 100 + old_col;
                new_row = 0;
            } else if old_direction == (0, -1) && (100..150).contains(&old_col) && old_row == 0 {
                // up 9
                new_direction = (0, -1);
                new_col = old_col - 100;
                new_row = 199;
            } else if old_direction == (-1, 0) && old_col == 0 && (150..200).contains(&(old_row as usize)) {
                // left 12
                new_direction = (1, 0);
                new_row = 0;
                new_col = old_row - 100;
            } else if old_direction == (-1, 0) && old_col == 0 && (100..150).contains(&(old_row as usize)) {
                // left 10 (lower)
                new_direction = (1, 0);
                new_col = 50;
                new_row = (old_row - 149).abs();
            } else {
                panic!("Remap row {} col {} direction {:?}", old_row, old_col, old_direction);
            }

            println!("Remapped row {} col {} direction {:?} to row {} col {} direction {:?}",
                     old_row, old_col, old_direction,
                     new_row, new_col, new_direction);

            ((new_row as usize, new_col as usize), grid[new_row as usize][new_col as usize], new_direction)
        } else {
            ((new_row as usize, new_col as usize), grid[new_row as usize][new_col as usize], new_direction)
        }
    }



    pub fn part2() {
        let (map, description) = read_file_raw("input_files/day22.txt").split("\n\n").map(str::to_owned).collect_tuple().unwrap();

        let grid: Vec<Vec<Tile>> = {
            let map_width = map.split('\n').map(str::len).max().unwrap();

            map.split('\n').map(|line| {
                let mut row = " ".repeat(map_width);
                row.replace_range(0..line.len(), line);
                row.chars().map(|ch| match ch {
                    '.' => Tile::Open,
                    '#' => Tile::Wall,
                    ' ' => Tile::Nothing,
                    _ => panic!("Bad input"),
                }).collect()
            }).collect()
        };

        {
            let row = 50;
            let col = 50;
            assert!(grid[row][col] != Tile::Nothing);
            let mut next_row = row;
            let mut next_col = col;
            let mut direction = (-1, 0);

            for _ in 0..(4 * 50) {
                ((next_row, next_col), _, direction) = next_tile_3d(&grid, (next_row, next_col), direction);
                dbg!(&(next_row, next_col));
            }

            assert_eq!((next_row, next_col), (row, col));
        }

        let mut position = (0, grid[0].iter().position(|&tile| tile == Tile::Open).unwrap());
        let mut direction: (i64, i64) = (1, 0);

        for instruction in description.trim().replace('R', "_R_").replace('L', "_L_").split('_') {
            if instruction == "R" {
                direction = match direction {
                    (1, 0) => (0, 1),
                    (0, 1) => (-1, 0),
                    (-1, 0) => (0, -1),
                    (0, -1) => (1, 0),
                    _ => panic!("Directionless!"),
                };
            } else if instruction == "L" {
                direction = match direction {
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    (-1, 0) => (0, 1),
                    (0, 1) => (1, 0),
                    _ => panic!("Directionless!"),
                };
            } else {
                let steps = instruction.parse::<usize>().unwrap();

                for _step in 0..steps {
                    let (next_tile_position, next_tile_type, next_direction) = next_tile_3d(&grid, position, direction);

                    if next_tile_type == Tile::Open {
                        position = next_tile_position;
                        direction = next_direction;
                    } else {
                        break;
                    }
                }
            }

            // println!("\nMove: {}", instruction);
            // draw_grid(&grid, position, direction);
        }

        println!("Final password: {}",
                 (1000 * (position.0 + 1)) +
                 (4 * (position.1 + 1)) +
                 match direction {
                     (1, 0) => 0,
                     (-1, 0) => 2,
                     (0, 1) => 1,
                     (0, -1) => 3,
                     _ => panic!("No direction"),
                 });


    }
}


mod day23 {
    use crate::shared::*;

    #[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
    enum Tile {
        Elf,
        Empty,
    }

    #[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
    struct Point {
        row: i64,
        col: i64,
    }

    impl Point {
        fn adjust(&self, offset: Point) -> Point {
            Point {
                row: self.row + offset.row,
                col: self.col + offset.col,
            }
        }
    }

    #[derive(Debug)]
    enum Direction {
        North,
        South,
        East,
        West,
        Northeast,
        Southeast,
        Northwest,
        Southwest,
    }

    impl Direction {
        fn point_offset(&self) -> Point {
            match self {
                Direction::North => Point { row: -1, col: 0 },
                Direction::South => Point { row: 1, col: 0 },
                Direction::East => Point { row: 0, col: 1 },
                Direction::West => Point { row: 0, col: -1 },
                Direction::Northeast => Point { row: -1, col: 1 },
                Direction::Southeast => Point { row: 1, col: 1 },
                Direction::Northwest => Point { row: -1, col: -1 },
                Direction::Southwest => Point { row: 1, col: -1 },
            }
        }
    }

    #[derive(Debug)]
    enum DirectionGroup {
        North,
        South,
        East,
        West,
        All,
    }

    impl DirectionGroup {
        fn offsets(&self) -> Vec<Point> {
            use self::Direction::*;

            match self {
                DirectionGroup::North => vec!(North.point_offset(), Northeast.point_offset(), Northwest.point_offset()),
                DirectionGroup::South => vec!(South.point_offset(), Southeast.point_offset(), Southwest.point_offset()),
                DirectionGroup::West => vec!(West.point_offset(), Northwest.point_offset(), Southwest.point_offset()),
                DirectionGroup::East => vec!(East.point_offset(), Northeast.point_offset(), Southeast.point_offset()),
                DirectionGroup::All => vec!(
                    North.point_offset(),
                    South.point_offset(),
                    East.point_offset(),
                    West.point_offset(),
                    Northeast.point_offset(),
                    Southeast.point_offset(),
                    Northwest.point_offset(),
                    Southwest.point_offset(),
                ),
            }
        }

        fn canonical_direction(&self) -> Direction {
            use self::Direction::*;

            match self {
                DirectionGroup::North => North,
                DirectionGroup::South => South,
                DirectionGroup::East => East,
                DirectionGroup::West => West,
                DirectionGroup::All => panic!("No canonical direction for All"),
            }
        }
    }


    fn show_grid(grid: &HashMap<Point, Tile>) {
        let min_col = grid.keys().map(|p| p.col).min().unwrap();
        let max_col = grid.keys().map(|p| p.col).max().unwrap();
        let min_row = grid.keys().map(|p| p.row).min().unwrap();
        let max_row = grid.keys().map(|p| p.row).max().unwrap();

        for row in min_row..=max_row {
            for col in min_col..=max_col {
                let ch = match grid.get(&Point { row: row as i64, col: col as i64 }) {
                    Some(&Tile::Elf) => '#',
                    _ => '.',
                };

                print!("{}", ch);
            }

            println!();
        }

        println!();
    }


    pub fn part1() {
        let mut grid: HashMap<Point, Tile> = HashMap::new();

        for (row, line) in input_lines("input_files/day23.txt").enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    grid.insert(Point { row: row as i64, col: col as i64 },
                                Tile::Elf);
                }
            }
        }

        // println!("Initial state");
        // show_grid(&grid);

        let mut round = 0;

        let directions_to_try = &[DirectionGroup::North, DirectionGroup::South, DirectionGroup::West, DirectionGroup::East];

        loop {
            let elf_positions: HashSet<Point> = grid.iter().filter(|(_point, &tile)| tile == Tile::Elf).map(|(&point, _)| point).collect();

            // Elves with nobody near them do nothing
            let slacker_elves: HashSet<Point> = elf_positions.iter().filter(|elf_position| {
                DirectionGroup::All.offsets().iter().all(|&offset| grid.get(&elf_position.adjust(offset)).unwrap_or(&Tile::Empty) == &Tile::Empty)
            }).copied().collect();

            let mut move_proposals: HashMap<Point, Vec<Point>> = HashMap::new();

            for working_elf in (elf_positions.difference(&slacker_elves)) {
                for direction_idx in 0..4 {
                    let next_direction = &directions_to_try[(round + direction_idx) % directions_to_try.len()];

                    if next_direction.offsets().iter().all(|&offset| grid.get(&working_elf.adjust(offset)).unwrap_or(&Tile::Empty) == &Tile::Empty) {
                        move_proposals.entry(working_elf.adjust(next_direction.canonical_direction().point_offset())).or_default().push(*working_elf);
                        break;
                    }
                }
            }

            let mut someone_moved = false;

            for (target_position, interested_elves) in move_proposals {
                if interested_elves.len() > 1 {
                    // Nobody moves
                } else {
                    // This elf moves
                    someone_moved = true;
                    grid.remove(&interested_elves[0]);
                    grid.insert(target_position, Tile::Elf);
                }
            }

            round += 1;

            // println!("End of round {}", round);
            // show_grid(&grid);

            if !someone_moved {
                break;
            }

            if round == 10 {
                break;
            }
        }

        let min_col = grid.keys().map(|p| p.col).min().unwrap();
        let max_col = grid.keys().map(|p| p.col).max().unwrap();
        let min_row = grid.keys().map(|p| p.row).min().unwrap();
        let max_row = grid.keys().map(|p| p.row).max().unwrap();

        println!("Empty tiles: {}", (max_col + 1 - min_col) * (max_row + 1 - min_row) - grid.len() as i64);
    }

    pub fn part2() {
        let mut grid: HashMap<Point, Tile> = HashMap::new();

        for (row, line) in input_lines("input_files/day23.txt").enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    grid.insert(Point { row: row as i64, col: col as i64 },
                                Tile::Elf);
                }
            }
        }

        // println!("Initial state");
        // show_grid(&grid);

        let mut round = 0;

        let directions_to_try = &[DirectionGroup::North, DirectionGroup::South, DirectionGroup::West, DirectionGroup::East];

        loop {
            let elf_positions: HashSet<Point> = grid.iter().filter(|(_point, &tile)| tile == Tile::Elf).map(|(&point, _)| point).collect();

            // Elves with nobody near them do nothing
            let slacker_elves: HashSet<Point> = elf_positions.iter().filter(|elf_position| {
                DirectionGroup::All.offsets().iter().all(|&offset| grid.get(&elf_position.adjust(offset)).unwrap_or(&Tile::Empty) == &Tile::Empty)
            }).copied().collect();

            let mut move_proposals: HashMap<Point, Vec<Point>> = HashMap::new();

            for working_elf in (elf_positions.difference(&slacker_elves)) {
                for direction_idx in 0..4 {
                    let next_direction = &directions_to_try[(round + direction_idx) % directions_to_try.len()];

                    if next_direction.offsets().iter().all(|&offset| grid.get(&working_elf.adjust(offset)).unwrap_or(&Tile::Empty) == &Tile::Empty) {
                        move_proposals.entry(working_elf.adjust(next_direction.canonical_direction().point_offset())).or_default().push(*working_elf);
                        break;
                    }
                }
            }

            let mut someone_moved = false;

            for (target_position, interested_elves) in move_proposals {
                if interested_elves.len() > 1 {
                    // Nobody moves
                } else {
                    // This elf moves
                    someone_moved = true;
                    grid.remove(&interested_elves[0]);
                    grid.insert(target_position, Tile::Elf);
                }
            }

            round += 1;

            // println!("End of round {}", round);
            // show_grid(&grid);

            if !someone_moved {
                println!("Finished at round {}", round);
                break;
            }
        }
    }
}

mod day24 {
    use crate::shared::*;

    #[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
    struct Point {
        row: i64,
        col: i64,
    }

    impl Point {
        fn adjust(&self, offset: Point) -> Point {
            Point {
                row: self.row + offset.row,
                col: self.col + offset.col,
            }
        }
    }


    #[derive(Debug, Eq, PartialEq, Copy, Clone)]
    enum Tile {
        Wall,
        RightBlizzard,
        UpBlizzard,
        DownBlizzard,
        LeftBlizzard,
    }

    #[derive(Clone)]
    struct Grid {
        width: usize,
        height: usize,
        grid: HashMap<Point, Vec<Tile>>,
    }

    impl Grid {
        fn parse(lines: impl Iterator<Item=String>) -> Grid {
            let mut grid: HashMap<Point, Vec<Tile>> = HashMap::new();

            let mut width = 0;
            let mut height = 0;

            for (row, line) in lines.enumerate() {
                height = row + 1;
                for (col, ch) in line.chars().enumerate() {
                    width = col + 1;

                    let tile = match ch {
                        '#' => Tile::Wall,
                        '<' => Tile::LeftBlizzard,
                        '^' => Tile::UpBlizzard,
                        'v' => Tile::DownBlizzard,
                        '>' => Tile::RightBlizzard,
                        _ => continue,
                    };

                    grid.insert(Point { row: row as i64, col: col as i64 },
                                vec!(tile));
                }
            }

            Grid { width, height, grid }
        }

        fn next_grid(&self) -> Grid {
            use self::Tile::*;
            let mut result: HashMap<Point, Vec<Tile>> = HashMap::new();

            for (point, tiles) in &self.grid {
                if tiles == &[Wall] {
                    result.insert(*point, tiles.clone());
                    continue;
                }

                for tile in tiles {
                    let mut next_point = point.adjust(match tile {
                        RightBlizzard => Point { row: 0, col: 1 },
                        LeftBlizzard => Point { row: 0, col: -1 },
                        UpBlizzard => Point { row: -1, col: 0 },
                        DownBlizzard => Point { row: 1, col: 0 },
                        _ => unreachable!(),
                    });

                    if self.grid.get(&next_point).is_some() && self.grid.get(&next_point).unwrap() == &[Wall] {
                        // Wrap around
                        next_point = match tile {
                            RightBlizzard => Point { row: next_point.row, col: 1 },
                            LeftBlizzard => Point { row: next_point.row, col: (self.width - 2) as i64 },
                            UpBlizzard => Point { row: (self.height - 2) as i64, col: next_point.col },
                            DownBlizzard => Point { row: 1, col: next_point.col },
                            _ => unreachable!(),
                        };
                    }

                    let entry = result.entry(next_point).or_default();

                    entry.push(*tile);
                }
            }
            Grid {
                grid: result,
                ..*self
            }
        }

        fn is_empty(&self, position: &Point) -> bool {
            self.grid.get(position).is_none()
        }
    }

    // Idea: floyd-warshall for the 3d space?
    fn solve(grid: &Grid, position: Point, target: Point, minutes: usize) -> usize {
        let next = grid.next_grid();

        if position == target {
            return minutes;
        }

        if minutes > 18 {
            return usize::MAX;
        }

        [
            Point { row: -1, col: 0 },
            Point { row: 1, col: 0 },
            Point { row: 0, col: -1 },
            Point { row: 0, col: 1 },
            Point { row: 0, col: 0 },
        ]
            .iter()
            .map(|offset| position.adjust(*offset))
            .filter(|position| next.is_empty(position))
            .map(|position| solve(&next, position, target, minutes + 1))
            .min()
            .unwrap_or(usize::MAX)
    }

    #[derive(Hash, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
    struct PointInTime {
        row: i64,
        col: i64,
        time: usize,
    }


    pub fn part1() {
        let max_time = 500;
        let grid = Grid::parse(input_lines("input_files/day24.txt"));

        let time_slices: Vec<Grid> = {
            let mut grid = grid;
            let mut r = Vec::new();

            r.push(grid.clone());

            for _t in 0..max_time {
                grid = grid.next_grid();
                r.push(grid.clone());
            }

            r
        };

        let mut all_points = HashSet::new();

        for t in 0..time_slices.len() {
            for row in 0..time_slices[t].height {
                for col in 0..time_slices[t].width {
                    if time_slices[t].is_empty(&Point { row: row as i64, col: col as i64 }) {
                        // Open tile is relevant to our interests!
                        all_points.insert(PointInTime { row: row as i64, col: col as i64, time: t });
                    }
                }
            }
        }

        let mut adjacent_nodes: HashMap<PointInTime, Vec<PointInTime>> = HashMap::new();

        for p in &all_points {
            for offset in [
                Point { row: -1, col: 0 },
                Point { row: 1, col: 0 },
                Point { row: 0, col: -1 },
                Point { row: 0, col: 1 },
                Point { row: 0, col: 0 },
            ] {
                let next_point = offset.adjust(Point { row: p.row, col: p.col });

                let candidate = PointInTime {
                    time: p.time + 1,
                    row: next_point.row,
                    col: next_point.col,
                };

                if all_points.contains(&candidate) {
                    adjacent_nodes.entry(*p).or_default().push(candidate);
                }
            }
        }

        let start_point = PointInTime { row: 0, col: 1, time: 0};

        let mut dist = HashMap::new();
        let mut processed = HashSet::new();
        let mut queue: BinaryHeap<(i64, PointInTime)> = BinaryHeap::new();

        dist.insert(start_point, 0);
        queue.push((0, start_point));

        while !queue.is_empty() {
            let min = queue.pop().unwrap();
            if processed.contains(&min.1) {
                continue;
            }

            processed.insert(min.1);
            if let Some(adjacent) = adjacent_nodes.get(&min.1) {
                for node in adjacent {
                    let new_distance = dist.get(&min.1).unwrap_or(&usize::MAX) + 1;

                    if new_distance < *dist.get(node).unwrap_or(&usize::MAX) {
                        dist.insert(*node, new_distance);
                        queue.push((- (new_distance as i64), *node));
                    }
                }
            }
        }

        for i in 1..time_slices.len() {
            let d =  dist.get(&PointInTime { row: 36, col: 100, time: i});

            if d.is_some() {
                dbg!(i, d);
                break;
            }
        }
    }

    fn build_time_slices(grid: &Grid, minutes: usize) -> Vec<Grid> {
        let mut grid = grid.clone();
        let mut r = Vec::new();

        r.push(grid.clone());

        for _t in 0..minutes {
            grid = grid.next_grid();
            r.push(grid.clone());
        }

        r
    }

    fn help_me_dijkstra(time_slices: &[Grid], start_point: PointInTime) -> HashMap<PointInTime, usize> {
        let mut all_points = HashSet::new();

        for t in 0..time_slices.len() {
            for row in 0..time_slices[t].height {
                for col in 0..time_slices[t].width {
                    if time_slices[t].is_empty(&Point { row: row as i64, col: col as i64 }) {
                        // Open tile is relevant to our interests!
                        all_points.insert(PointInTime { row: row as i64, col: col as i64, time: t });
                    }
                }
            }
        }

        let mut adjacent_nodes: HashMap<PointInTime, Vec<PointInTime>> = HashMap::new();

        for p in &all_points {
            for offset in [
                Point { row: -1, col: 0 },
                Point { row: 1, col: 0 },
                Point { row: 0, col: -1 },
                Point { row: 0, col: 1 },
                Point { row: 0, col: 0 },
            ] {
                let next_point = offset.adjust(Point { row: p.row, col: p.col });

                let candidate = PointInTime {
                    time: p.time + 1,
                    row: next_point.row,
                    col: next_point.col,
                };

                if all_points.contains(&candidate) {
                    adjacent_nodes.entry(*p).or_default().push(candidate);
                }
            }
        }

        let mut dist = HashMap::new();
        let mut processed = HashSet::new();
        let mut queue: BinaryHeap<(i64, PointInTime)> = BinaryHeap::new();

        dist.insert(start_point, 0);
        queue.push((0, start_point));

        while !queue.is_empty() {
            let min = queue.pop().unwrap();
            if processed.contains(&min.1) {
                continue;
            }

            processed.insert(min.1);
            if let Some(adjacent) = adjacent_nodes.get(&min.1) {
                for node in adjacent {
                    let new_distance = dist.get(&min.1).unwrap_or(&usize::MAX) + 1;

                    if new_distance < *dist.get(node).unwrap_or(&usize::MAX) {
                        dist.insert(*node, new_distance);
                        queue.push((- (new_distance as i64), *node));
                    }
                }
            }
        }

        dist
    }

    pub fn part2() {
        let max_time = 500;
        let grid = Grid::parse(input_lines("input_files/day24.txt"));

        let mut time_slices = build_time_slices(&grid, max_time);

        {
            let dist = help_me_dijkstra(&time_slices, PointInTime { row: 0, col: 1, time: 0});

            // First forward pass
            for i in 1..time_slices.len() {
                let d =  dist.get(&PointInTime { row: 36, col: 100, time: i});

                if d.is_some() {
                    dbg!(i, d);
                    break;
                }
            }
        }

        // Back we go
        {
            time_slices = build_time_slices(&time_slices[240], max_time);

            let dist = help_me_dijkstra(&time_slices, PointInTime { row: 36, col: 100, time: 0});

            for i in 1..time_slices.len() {
                let d =  dist.get(&PointInTime { row: 0, col: 1, time: i});

                if d.is_some() {
                    dbg!(i, d);
                    break;
                }
            }
        }

        // (snacks acquired)

        // Second forward pass
        {
            time_slices = build_time_slices(&time_slices[237], max_time);

            let dist = help_me_dijkstra(&time_slices, PointInTime { row: 0, col: 1, time: 0});

            for i in 1..time_slices.len() {
                // let d =  dist.get(&PointInTime { row: 36, col: 100, time: i});
                let d =  dist.get(&PointInTime { row: 36, col: 100, time: i });

                if d.is_some() {
                    dbg!(i, d);
                    break;
                }
            }
        }


    }
}


mod dayn {
    use crate::shared::*;

    pub fn part1() {

    }

    pub fn part2() {

    }
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

        day17::part1();
        day17::part2();

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

    // day24::part1();
    day24::part2();
}
