// (cd ~/projects/adventofcode/2021 && cargo run)

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(dead_code)]

mod shared {
    pub use regex::Regex;

    // pub use intcode::{self, IntCode};
    pub use std::cell::RefCell;
    pub use std::cell::RefMut;
    pub use std::cmp::{self, Ordering, Reverse};
    pub use std::collections::BinaryHeap;
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
    pub use std::str::{self, FromStr};
    pub use std::sync::{Arc, Mutex};
    pub use std::ops::RangeInclusive;

    pub use anyhow::{anyhow, bail, Error};

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

        let window_sums: Vec<usize> = readings
            .iter()
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .collect();

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
            let mut it = s.split(' ');

            Ok(Command {
                op: match it.next().ok_or_else(|| anyhow!("empty line"))? {
                    "forward" => Op::Forward,
                    "down" => Op::Down,
                    "up" => Op::Up,
                    _ => bail!("unknown command"),
                },
                n: it
                    .next()
                    .ok_or_else(|| anyhow!("missing second arg"))?
                    .parse()?,
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

        println!(
            "Horizontal: {}, Depth: {}.  Product: {}",
            horizontal,
            depth,
            horizontal * depth
        );
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
                }
                Op::Down => aim += command.n,
                Op::Up => aim -= command.n,
            }
        }

        println!(
            "Horizontal: {}, Depth: {}.  Product: {}",
            horizontal,
            depth,
            horizontal * depth
        );
    }
}

mod day3 {
    use crate::shared::*;

    fn to_decimal(bits: &[u32]) -> u64 {
        let mut result: u64 = 0;

        for bit in bits {
            result *= 2;
            result += (*bit as u64);
        }

        result
    }

    pub fn part1() {
        let numbers: Vec<Vec<u32>> = input_lines("input_files/day3.txt")
            .map(|s| {
                s.chars()
                    .map(|ch| ch.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect();

        let number_width = numbers[0].len();

        let gamma: Vec<u32> = (0..number_width)
            .map(|idx| {
                let ones_count = numbers.iter().map(|ns| ns[idx]).filter(|&n| n == 1).count();

                if ones_count > numbers.len() / 2 {
                    1
                } else {
                    0
                }
            })
            .collect();

        let epsilon: Vec<u32> = gamma.iter().map(|&n| n ^ 1).collect();

        println!(
            "Gamma: {}; Epsilon: {}, Result: {}",
            to_decimal(&gamma),
            to_decimal(&epsilon),
            to_decimal(&gamma) * to_decimal(&epsilon)
        );
    }

    pub fn part2() {
        fn best_value(
            numbers: Vec<Vec<u32>>,
            target_value_from_highest_frequence: impl Fn(Option<u32>) -> u32,
        ) -> u64 {
            let number_width = numbers[0].len();

            let mut remaining = numbers;
            for idx in 0..number_width {
                if remaining.len() == 1 {
                    break;
                }

                let ones_count = remaining
                    .iter()
                    .map(|ns| ns[idx])
                    .filter(|&n| n == 1)
                    .count();
                let zeroes_count = remaining.len() - ones_count;

                let highest_frequency = match ones_count.cmp(&zeroes_count) {
                    Ordering::Greater => Some(1),
                    Ordering::Less => Some(0),
                    Ordering::Equal => None,
                };

                let target_value = target_value_from_highest_frequence(highest_frequency);

                remaining = remaining
                    .into_iter()
                    .filter(|n| n[idx] == target_value)
                    .collect();
            }

            to_decimal(&remaining[0])
        }

        let numbers: Vec<Vec<u32>> = input_lines("input_files/day3.txt")
            .map(|s| {
                s.chars()
                    .map(|ch| ch.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect();

        let oxygen_value = best_value(numbers.clone(), |most_frequent| most_frequent.unwrap_or(1));
        let co2_value = best_value(numbers, |most_frequent| {
            most_frequent.map(|n| n ^ 1).unwrap_or(0)
        });

        println!("{}", oxygen_value * co2_value);
    }
}

mod day4 {
    use crate::shared::*;

    #[derive(Debug)]
    struct Game {
        called_numbers: Vec<usize>,
        boards: Vec<Board>,
    }

    impl Game {
        fn from_lines(mut lines: impl Iterator<Item = String>) -> Game {
            let called_numbers: Vec<usize> = lines
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            // Eat blank
            let _ = lines.next().unwrap();

            let mut boards = Vec::new();
            while let Some(board) = Board::from_lines(&mut lines) {
                boards.push(board);
            }

            Game {
                called_numbers,
                boards,
            }
        }
    }

    #[derive(Debug)]
    struct Board {
        rows: Vec<Vec<Cell>>,
    }

    impl Board {
        fn from_lines(lines: &mut impl Iterator<Item = String>) -> Option<Board> {
            let board_delim = Regex::new(r" +").unwrap();

            let mut rows = Vec::new();

            for line in lines {
                if line.is_empty() {
                    break;
                }

                rows.push(
                    board_delim
                        .split(&line)
                        .filter(|s| !s.is_empty())
                        .map(|s| Cell {
                            value: s.parse().unwrap(),
                            marked: false,
                        })
                        .collect(),
                );
            }

            if rows.is_empty() {
                None
            } else {
                Some(Board { rows })
            }
        }

        fn mark_cells_with_value(&mut self, value: usize) {
            for row in self.rows.iter_mut() {
                for cell in row.iter_mut() {
                    if cell.value == value {
                        cell.marked = true;
                    }
                }
            }
        }

        fn is_winner(&self) -> bool {
            // Full row
            for row in &self.rows {
                if row.iter().all(|cell| cell.marked) {
                    return true;
                }
            }

            // Full column
            for col_idx in 0..self.rows[0].len() {
                if self.rows.iter().map(|row| row[col_idx].marked).all(|m| m) {
                    return true;
                }
            }

            // Bupkis
            false
        }

        fn score(&self, last_number: usize) -> usize {
            let mut sum = 0;
            for row in &self.rows {
                for cell in row {
                    if !cell.marked {
                        sum += cell.value;
                    }
                }
            }

            sum * last_number
        }
    }

    #[derive(Debug)]
    struct Cell {
        value: usize,
        marked: bool,
    }

    pub fn part1() {
        let mut game = Game::from_lines(input_lines("input_files/day4.txt"));

        for n in game.called_numbers {
            for b in game.boards.iter_mut() {
                b.mark_cells_with_value(n);
            }

            for b in &game.boards {
                if b.is_winner() {
                    println!("Winner with score: {}", b.score(n));
                    return;
                }
            }
        }
    }

    pub fn part2() {
        let mut game = Game::from_lines(input_lines("input_files/day4.txt"));

        let mut remaining_loser_positions: Vec<usize>;
        let mut loser_idx = None;
        let mut last_number = None;

        for n in game.called_numbers {
            for b in game.boards.iter_mut() {
                b.mark_cells_with_value(n);
            }

            remaining_loser_positions = game
                .boards
                .iter()
                .enumerate()
                .filter(|(_, b)| !b.is_winner())
                .map(|(idx, _)| idx)
                .collect();

            if remaining_loser_positions.len() == 1 {
                loser_idx = Some(remaining_loser_positions[0]);
            } else if remaining_loser_positions.is_empty() {
                // All done
                last_number = Some(n);
                break;
            }
        }

        assert!(loser_idx.is_some());
        assert!(last_number.is_some());

        println!(
            "Final winning board had a score of {}",
            game.boards[loser_idx.unwrap()].score(last_number.unwrap())
        );
    }
}

mod day5 {
    use crate::shared::*;

    #[derive(Debug)]
    struct Line {
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
    }

    impl std::str::FromStr for Line {
        type Err = Error;

        fn from_str(s: &str) -> Result<Line, Error> {
            let mut it = s.split(" -> ");

            fn parse_point(s: &str) -> Result<(usize, usize), Error> {
                let mut it = s.split(',');
                Ok((it.next().unwrap().parse()?, it.next().unwrap().parse()?))
            }

            let (x1, y1) = parse_point(it.next().unwrap())?;
            let (x2, y2) = parse_point(it.next().unwrap())?;

            Ok(Line { x1, y1, x2, y2 })
        }
    }

    fn range(a: usize, b: usize) -> Vec<usize> {
        if a <= b {
            (a..=b).collect()
        } else {
            let mut r = range(b, a);
            r.reverse();
            r
        }
    }

    fn line_points(line: &Line) -> Vec<(usize, usize)> {
        if line.x1 != line.x2 && line.y1 != line.y2 {
            // not horizontal/vertical
            return vec![];
        }

        let mut xs = range(line.x1, line.x2);
        let mut ys = range(line.y1, line.y2);

        let len = cmp::max(xs.len(), ys.len());

        xs = xs.iter().cycle().take(len).copied().collect();
        ys = ys.iter().cycle().take(len).copied().collect();

        xs.into_iter().zip(ys.into_iter()).collect()
    }

    pub fn part1() {
        let mut diagram: HashMap<(usize, usize), usize> = HashMap::new();

        for linespec in input_lines("input_files/day5.txt") {
            let line = Line::from_str(&linespec).expect("parse error");

            for (x, y) in line_points(&line) {
                let entry = diagram.entry((x, y)).or_insert(0);
                *entry += 1;
            }
        }

        let mut count = 0;
        for (&k, &v) in diagram.iter() {
            if v >= 2 {
                println!("{:?}", k);
                count += 1;
            }
        }

        println!("Total overlaps: {}", count);
    }

    fn line_points_with_diagonals(line: &Line) -> Vec<(usize, usize)> {
        let mut xs = range(line.x1, line.x2);
        let mut ys = range(line.y1, line.y2);

        if (line.x1 != line.x2 && line.y1 != line.y2) && xs.len() != ys.len() {
            // not horizontal/vertical/45 degrees
            return vec![];
        }

        let len = cmp::max(xs.len(), ys.len());

        xs = xs.iter().cycle().take(len).copied().collect();
        ys = ys.iter().cycle().take(len).copied().collect();

        xs.into_iter().zip(ys.into_iter()).collect()
    }

    pub fn part2() {
        let mut diagram: HashMap<(usize, usize), usize> = HashMap::new();

        for linespec in input_lines("input_files/day5.txt") {
            let line = Line::from_str(&linespec).expect("parse error");

            for (x, y) in line_points_with_diagonals(&line) {
                let entry = diagram.entry((x, y)).or_insert(0);
                *entry += 1;
            }
        }

        let mut count = 0;
        for (&k, &v) in diagram.iter() {
            if v >= 2 {
                println!("{:?}", k);
                count += 1;
            }
        }

        println!("Total overlaps: {}", count);
    }
}

mod day6 {
    use crate::shared::*;

    pub fn part1() {
        let mut lantern_fish: Vec<i64> = input_lines("input_files/day6.txt")
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        for _day in 1..=80 {
            let mut spawn_count = 0;

            for fish in lantern_fish.iter_mut() {
                *fish -= 1;

                if *fish < 0 {
                    *fish = 6;
                    spawn_count += 1;
                }
            }

            if spawn_count > 0 {
                lantern_fish.resize(spawn_count + lantern_fish.len(), 8);
            }
        }

        println!("After 80 days: {}", lantern_fish.len());
    }

    pub fn part2() {
        let mut lantern_fish: HashMap<u8, usize> = HashMap::new();

        for fish in input_lines("input_files/day6.txt")
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
        {
            let entry = lantern_fish.entry(fish).or_insert(0);
            *entry += 1;
        }

        for _day in 1..=256 {
            // Spawners
            let spawners = *(lantern_fish.get(&0).unwrap_or(&0));

            for gen in 1..=8 {
                let count = *(lantern_fish.get(&gen).unwrap_or(&0));
                lantern_fish.insert(gen - 1, count);
            }

            // Spawners are reset
            let entry = lantern_fish.entry(6).or_insert(0);
            *entry += spawners;

            // Spawners also spawn babbies
            let entry = lantern_fish.entry(8).or_insert(0);
            *entry = spawners;
        }

        // not 5846967653643182790
        println!("After 256 days: {}", lantern_fish.values().sum::<usize>());
    }
}

mod day7 {
    use crate::shared::*;

    pub fn part1() {
        let crab_positions: Vec<i64> = input_lines("input_files/day7.txt")
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        let min_crab = *crab_positions.iter().min().unwrap();
        let max_crab = *crab_positions.iter().max().unwrap();

        let mut best_position = -1;
        let mut fuel_spend = i64::MAX;

        for candidate_position in min_crab..=max_crab {
            let cost = crab_positions.iter().map(|p| (p - candidate_position).abs()).sum();

            if cost < fuel_spend {
                best_position = candidate_position;
                fuel_spend = cost;
            }
        }

        println!("Position: {}; Cost: {}", best_position, fuel_spend);
    }

    fn weight(n: i64) -> i64 {
        (1..=n).sum()
    }

    pub fn part2() {
        let crab_positions: Vec<i64> = input_lines("input_files/day7.txt")
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        let min_crab = *crab_positions.iter().min().unwrap();
        let max_crab = *crab_positions.iter().max().unwrap();

        let mut best_position = -1;
        let mut fuel_spend = i64::MAX;

        for candidate_position in min_crab..=max_crab {
            let cost = crab_positions.iter().map(|p| weight((p - candidate_position).abs())).sum();

            if cost < fuel_spend {
                best_position = candidate_position;
                fuel_spend = cost;
            }
        }

        println!("Position: {}; Cost: {}", best_position, fuel_spend);
    }
}


mod day8 {
    use crate::shared::*;

    pub fn part1() {
        let count: usize = input_lines("input_files/day8.txt")
            .map(|line| {
                let words = line.split(" | ").nth(1).unwrap().split(' ');
                words
                    .filter(|w| matches!(w.len(), 2 | 3 | 4 | 7))
                    .count()
            }).sum();

        println!("Output count: {}", count);
    }

    const WIRES: &[char] = &['a', 'b', 'c', 'd', 'e', 'f', 'g'];



    //   0:      1:      2:      3:      4:
    //  aaaa    ....    aaaa    aaaa    ....
    // b    c  .    c  .    c  .    c  b    c
    // b    c  .    c  .    c  .    c  b    c
    //  ....    ....    dddd    dddd    dddd
    // e    f  .    f  e    .  .    f  .    f
    // e    f  .    f  e    .  .    f  .    f
    //  gggg    ....    gggg    gggg    ....
    //
    //   5:      6:      7:      8:      9:
    //  aaaa    aaaa    aaaa    aaaa    aaaa
    // b    .  b    .  .    c  b    c  b    c
    // b    .  b    .  .    c  b    c  b    c
    //  dddd    dddd    ....    dddd    dddd
    // .    f  e    f  .    f  e    f  .    f
    // .    f  e    f  .    f  e    f  .    f
    //  gggg    gggg    ....    gggg    gggg


    fn merge_into_candidates(candidates: &mut HashMap<char, HashSet<char>>,
                             letters: &[char],
                             input: &str) {
        let letter_set: HashSet<char> = letters.iter().copied().collect();
        let input_set: HashSet<char> = input.chars().collect();

        // Remove our input chars from other mappings
        for (k, v) in candidates.iter_mut() {
            if !letter_set.contains(k) {
                *v = v.difference(&input_set).copied().collect();
            }
        }

        // and intersect with the letters of interest
        for l in letters.iter() {
            let c = candidates.get_mut(l).unwrap();
            *c = c.intersection(&input_set).copied().collect();
        }
    }

    fn all_mappings(mut candidates: HashMap<char, HashSet<char>>) -> Vec<HashMap<char, char>> {
        fn aux(remaining_candidates: &mut HashMap<char, HashSet<char>>,
               current_mapping: &mut HashMap<char, char>,
               used_values: &mut HashSet<char>,
               results: &mut Vec<HashMap<char, char>>,
               level: usize) {

            if remaining_candidates.is_empty() {
                results.push(current_mapping.clone());
                return;
            }

            let next_key = *remaining_candidates.keys().next().unwrap();
            let next_values = remaining_candidates.remove(&next_key).unwrap();

            let possible_values: Vec<char> = next_values.difference(used_values).copied().collect();

            for v in possible_values {
                used_values.insert(v);
                current_mapping.insert(next_key, v);
                aux(remaining_candidates, current_mapping, used_values, results, level + 1);
                used_values.remove(&v);
            }

            remaining_candidates.insert(next_key, next_values);
        }

        let mut results = Vec::new();
        aux(&mut candidates, &mut HashMap::new(), &mut HashSet::new(), &mut results, 0);
        results
    }

    fn unscramble(inputs: &[String], mapping: &HashMap<char, char>) -> Vec<String> {
        let reverse_mapping: HashMap<char, char> = mapping.iter().map(|(&k, &v)| (v, k)).collect();

        inputs.iter().map(|input| {
            let mut chars: Vec<char> = input.chars().map(|ch| reverse_mapping.get(&ch).unwrap()).copied().collect();
            chars.sort_unstable();
            chars.into_iter().collect::<String>()
        }).collect()
    }

    pub fn part2() {
        let mut total = 0;

        let mut correct_inputs_map: HashMap<String, usize> = HashMap::new();

        correct_inputs_map.insert("abcefg".to_string(),  0);
        correct_inputs_map.insert("cf".to_string(),      1);
        correct_inputs_map.insert("acdeg".to_string(),   2);
        correct_inputs_map.insert("acdfg".to_string(),   3);
        correct_inputs_map.insert("bcdf".to_string(),    4);
        correct_inputs_map.insert("abdfg".to_string(),   5);
        correct_inputs_map.insert("abdefg".to_string(),  6);
        correct_inputs_map.insert("acf".to_string(),     7);
        correct_inputs_map.insert("abcdefg".to_string(), 8);
        correct_inputs_map.insert("abcdfg".to_string(),  9);

        let correct_inputs: HashSet<String> = correct_inputs_map.keys().cloned().collect();

        for line in input_lines("input_files/day8.txt") {
            let mut it = line.split(" | ");
            let inputs: Vec<String> = it.next().unwrap().split(' ').map(|s| s.to_string()).collect();
            let outputs: Vec<String> = it.next().unwrap().split(' ').map(|s| s.to_string()).collect();

            // 'a' -> ['a', 'b', 'c'] reads "segment a might be represented by a, b or c"
            let mut candidates: HashMap<char, HashSet<char>> = HashMap::new();

            // Initially, anything is possible
            for &ch in WIRES {
                candidates.insert(ch, WIRES.iter().copied().collect::<HashSet<char>>());
            }

            // Cull the candidates for the unique-lengthed outputs
            for input in inputs.iter() {
                match input.len() {
                    2 => {
                        merge_into_candidates(&mut candidates, &['c', 'f'], input);
                    }
                    3 => {
                        merge_into_candidates(&mut candidates, &['a', 'c', 'f'], input);
                    }
                    4 => {
                        merge_into_candidates(&mut candidates, &['b', 'c', 'd', 'f'], input);
                    }
                    _ => {}
                }
            }

            // Combinations should now be small enough to brute-force
            for mapping in all_mappings(candidates) {
                // Unscramble our incoming string and see if we match our correct inputs.
                let candidate = unscramble(&inputs, &mapping);

                if candidate.into_iter().collect::<HashSet<String>>() == correct_inputs {
                    let mut subtotal = 0;
                    for output in unscramble(&outputs, &mapping) {
                        subtotal = (subtotal * 10) + correct_inputs_map.get(&output).unwrap();
                    }

                    total += subtotal;

                    break;
                }
            }
        }

        println!("Output total: {}", total);
    }
}


mod day9 {
    use crate::shared::*;

    pub fn part1() {
        let height_map: Vec<Vec<usize>> =
            input_lines("input_files/day9.txt")
            .map(|s| s.chars().map(|ch| ch.to_digit(10).unwrap() as usize).collect())
            .collect();

        let mut total_risk = 0;

        for row in 0..height_map.len() {
            for col in 0..height_map[0].len() {
                let value = height_map[row][col];

                let mut neighbours = [(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .into_iter()
                    .map(|(x_off, y_off)| {
                        let n_row = (row as i64 + y_off);
                        let n_col = (col as i64 + x_off);

                        if n_row >= 0 && n_row < height_map.len() as i64 && n_col >= 0 && n_col < height_map[0].len() as i64 {
                            Some(height_map[n_row as usize][n_col as usize])
                        } else {
                            None
                        }
                    })
                    .flatten();

                if neighbours.all(|v| v > value) {
                    total_risk += (1 + value);
                }
            }
        }

        println!("Total risk: {}", total_risk);
    }

    #[derive(Debug)]
    struct Basin {
        size: usize,
        extents: HashMap<usize, Vec<(usize, usize)>>,
    }

    fn extract_basins(row_idx: usize, row: &[usize]) -> Vec<Basin> {
        let mut result = Vec::new();
        let mut last_start_idx = None;

        for (idx, &cell) in row.iter().enumerate() {
            if cell == 9 {
                if let Some(start_idx) = last_start_idx {
                    // end of a basin
                    result.push(Basin {
                        size: (idx - start_idx),
                        extents: [(row_idx, vec![(start_idx, idx - 1)])].into_iter().collect(),
                    });

                    last_start_idx = None;
                }
            } else if last_start_idx.is_none() {
                // Start of the next basin
                last_start_idx = Some(idx);
            }
        }

        if let Some(last_start_idx) = last_start_idx {
            let end_idx = row.len() - 1;

            result.push(Basin {
                size: (end_idx - last_start_idx + 1),
                extents: [(row_idx, vec![(last_start_idx, end_idx)])].into_iter().collect(),
            });
        }

        result
    }

    impl Basin {
        fn overlaps(&self, target_idx: usize, other: &Basin) -> bool {
            if let Some(last_row_extents) = self.extents.get(&target_idx) {
                last_row_extents.iter().any(|(start_idx, end_idx)| {
                    other.extents.get(&(target_idx + 1)).unwrap().iter().any(|(other_start_idx, other_end_idx)| {
                        !(other_end_idx < start_idx || other_start_idx > end_idx)
                    })
                })
            } else {
                false
            }
        }

        fn merge_row_basin(&mut self, row_idx: usize, other: &Basin) {
            self.size += other.size;
            let row_extents = self.extents.entry(row_idx).or_insert_with(Vec::new);
            row_extents.extend(other.extents.get(&row_idx).unwrap());
        }
    }

    pub fn part2() {
        let height_map: Vec<Vec<usize>> =
            input_lines("input_files/day9.txt")
            .map(|s| s.chars().map(|ch| ch.to_digit(10).unwrap() as usize).collect())
            .collect();

        let mut basins: Vec<Basin> = Vec::new();

        for (row_idx, row) in height_map.iter().enumerate() {
            for row_basin in extract_basins(row_idx, row) {
                let mut overlapping_basin_indexes: Vec<usize> = (0..basins.len()).filter(|&i| {
                    row_idx > 0 && basins[i].overlaps(row_idx - 1, &row_basin)
                }).collect();

                overlapping_basin_indexes.reverse();

                if overlapping_basin_indexes.is_empty() {
                    // If we didn't merge into an existing basin, we're the start of a new one
                    basins.push(row_basin);
                } else {
                    // Merge our overlapping basins into one
                    let mut merged_basin = basins.swap_remove(overlapping_basin_indexes[0]);

                    for &remove_idx in &overlapping_basin_indexes[1..] {
                        merged_basin.merge_row_basin(row_idx - 1, &basins.swap_remove(remove_idx));
                    }

                    // Merge our row basin too
                    merged_basin.merge_row_basin(row_idx, &row_basin);

                    basins.push(merged_basin);
                }
            }
        }

        basins.sort_by_key(|b| 0 - b.size as i64);
        println!("Basin sizes multiplied: {}",
                 basins.iter().take(3).map(|b| b.size).product::<usize>());
    }
}

mod day10 {
    use crate::shared::*;

    pub fn part1() {
        let mut error_score = 0;

        'input_loop:
        for line in input_lines("input_files/day10.txt") {
            let mut expected_closers = Vec::new();

            for ch in line.chars() {
                match ch {
                    '[' => expected_closers.push(']'),
                    '{' => expected_closers.push('}'),
                    '<' => expected_closers.push('>'),
                    '(' => expected_closers.push(')'),

                    ']' | '}' | '>' | ')' => {
                        let expected_closer = expected_closers.pop();

                        if Some(ch) != expected_closer {
                            println!("Unexpected closer: {} (expected {:?}", ch, expected_closer);

                            error_score += match ch {
                                ')' => 3,
                                ']' => 57,
                                '}' => 1197,
                                '>' => 25137,
                                _ => unreachable!()
                            };

                            continue 'input_loop
                        }
                    },
                    _ => panic!("malformed input")
                }
            }

            if !expected_closers.is_empty() {
                println!("Warning: line truncated...");
            }
        }

        println!("Total error score: {}", error_score);
    }

    pub fn part2() {
        let mut line_scores = Vec::new();

        // WRONG 312966863
        'input_loop:
        for line in input_lines("input_files/day10.txt") {
            let mut expected_closers = Vec::new();

            for ch in line.chars() {
                match ch {
                    '[' => expected_closers.push(']'),
                    '{' => expected_closers.push('}'),
                    '<' => expected_closers.push('>'),
                    '(' => expected_closers.push(')'),

                    ']' | '}' | '>' | ')' => {
                        let expected_closer = expected_closers.pop();

                        if Some(ch) != expected_closer {
                            continue 'input_loop
                        }
                    },
                    _ => panic!("malformed input")
                }
            }

            if !expected_closers.is_empty() {
                // An incomplete line... score it
                let mut score = 0;

                for closer in expected_closers.iter().rev() {
                    score *= 5;
                    score += match closer {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!("unexpected char"),
                    }
                }

                line_scores.push(score);
            }
        }

        line_scores.sort_unstable();
        println!("Median score: {}", line_scores[line_scores.len() / 2]);
    }
}


mod day11 {
    use crate::shared::*;

    #[derive(Debug)]
    struct Grid {
        grid: Vec<Vec<usize>>,
        flashed_positions: Vec<Vec<bool>>,
    }

    impl Grid {
        fn new(grid: Vec<Vec<usize>>) -> Grid {
            let width = grid[0].len();
            let height = grid.len();

            Grid {
                grid,
                flashed_positions: (0..height).map(|_| vec![false; width]).collect(),
            }
        }

        fn len(&self) -> usize {
            self.grid.len() * self.grid[0].len()
        }

        fn increment_all(&mut self) {
            for row in self.grid.iter_mut() {
                for cell in row.iter_mut() {
                    *cell += 1
                }
            }
        }

        fn find_flashes(&mut self) -> Vec<(usize, usize)> {
            let mut result = Vec::new();

            for row in 0..self.grid.len() {
                for col in 0..self.grid[0].len() {
                    if self.grid[row][col] > 9 && !self.flashed_positions[row][col] {
                        result.push((row, col));
                        self.flashed_positions[row][col] = true;
                    }
                }
            }

            result
        }

        fn increment_neighbours(&mut self, row: usize, col: usize) {
            let width = self.grid[0].len() as i64;
            let height = self.grid.len() as i64;

            for &xoff in &[-1, 0, 1] {
                for &yoff in &[-1, 0, 1] {
                    let neighbour_row = row as i64 + xoff;
                    let neighbour_col = col as i64 + yoff;

                    if (neighbour_row >= 0 && neighbour_row < height) && (neighbour_col >= 0 && neighbour_col < width) {
                        self.grid[neighbour_row as usize][neighbour_col as usize] += 1;
                    }
                }
            }
        }

        fn adjust_energy_levels(&mut self) {
            for row in self.grid.iter_mut() {
                for cell in row.iter_mut() {
                    if *cell > 9 {
                        *cell = 0;
                    }
                }
            }

            for row in self.flashed_positions.iter_mut() {
                for cell in row.iter_mut() {
                    *cell = false;
                }
            }

        }

    }


    pub fn part1() {
        let mut grid = Grid::new(input_lines("input_files/day11.txt")
                                 .map(|row| row.chars().map(|ch| ch.to_digit(10).unwrap() as usize).collect())
                                 .collect());

        let mut flash_count = 0;

        for _step in 0..100 {
            grid.increment_all();

            loop {
                let flashed_positions = grid.find_flashes();

                if flashed_positions.is_empty() {
                    break;
                }

                flash_count += flashed_positions.len();

                for (row, col) in flashed_positions {
                    grid.increment_neighbours(row, col);
                }
            }

            grid.adjust_energy_levels();
        }

        println!("Flash count: {}", flash_count);
    }

    pub fn part2() {
        let mut grid = Grid::new(input_lines("input_files/day11.txt")
                                 .map(|row| row.chars().map(|ch| ch.to_digit(10).unwrap() as usize).collect())
                                 .collect());

        // 1000: the largest known number
        for step in 0..1000 {
            let mut flash_count = 0;

            grid.increment_all();

            loop {
                let flashed_positions = grid.find_flashes();

                if flashed_positions.is_empty() {
                    break;
                }

                flash_count += flashed_positions.len();

                for (row, col) in flashed_positions {
                    grid.increment_neighbours(row, col);
                }
            }

            grid.adjust_energy_levels();

            if flash_count == grid.len() {
                println!("All flashed at step {}", step + 1);
                break;
            }
        }
    }
}


mod day12 {
    use crate::shared::*;

    #[derive(Hash, Eq, PartialEq, Debug, Clone)]
    struct Cave {
        name: String,
        is_big: bool,
    }

    impl Cave {
        fn from_str(s: &str) -> Cave {
            Cave {
                name: s.to_owned(),
                is_big: s.chars().next().unwrap().is_ascii_uppercase(),
            }
        }
    }

    fn count_paths(connections: &HashMap<Cave, Vec<Cave>>) -> usize {
        fn aux(connections: &HashMap<Cave, Vec<Cave>>,
               visited_caves: &mut HashSet<Cave>,
               path: &mut Vec<Cave>,
               current_cave: Cave) -> usize {

            if current_cave.name == "end" {
                return 1;
            }

            visited_caves.insert(current_cave.clone());

            let subcount =
                if let Some(candidates) = connections.get(&current_cave) {
                    let mut total = 0;

                    for candidate in candidates {
                        if !candidate.is_big && visited_caves.contains(candidate) {
                            continue;
                        }

                        path.push(current_cave.clone());
                        total += aux(connections, visited_caves, path, candidate.clone());
                        path.pop();
                    }

                    total
                } else {
                    0
                };

            visited_caves.remove(&current_cave);

            subcount
        }

        let mut visited_caves = HashSet::new();
        let mut path = Vec::new();

        aux(connections,
            &mut visited_caves,
            &mut path,
            Cave::from_str("start"))
    }


    pub fn part1() {
        let mut cave_connections: HashMap<Cave, Vec<Cave>> = HashMap::new();

        for line in input_lines("input_files/day12.txt") {
            let mut it = line.split('-');
            let from = Cave::from_str(it.next().unwrap());
            let to = Cave::from_str(it.next().unwrap());

            // a -> b
            {
                let entry = cave_connections.entry(from.clone()).or_insert_with(Vec::new);
                entry.push(to.clone());
            }

            // b -> a
            {
                let entry = cave_connections.entry(to.clone()).or_insert_with(Vec::new);
                entry.push(from.clone());
            }
        }

        println!("Unique paths: {}", count_paths(&cave_connections));
    }

    fn count_paths_pt2(connections: &HashMap<Cave, Vec<Cave>>, blessed_small_cave: Cave, all_paths: &mut HashSet<Vec<Cave>>) -> usize {
        fn aux(connections: &HashMap<Cave, Vec<Cave>>,
               visited_caves: &mut HashSet<Cave>,
               path: &mut Vec<Cave>,
               current_cave: Cave,
               blessed_small_cave: &mut Option<Cave>,
               all_paths: &mut HashSet<Vec<Cave>>,
        ) -> usize {

            if current_cave.name == "end" {
                all_paths.insert(path.clone());
                return 1;
            }

            visited_caves.insert(current_cave.clone());

            let subcount =
                if let Some(candidates) = connections.get(&current_cave) {
                    let mut total = 0;

                    for candidate in candidates {
                        let mut used_blessed = false;

                        if !candidate.is_big && visited_caves.contains(candidate) {
                            if Some(candidate) == blessed_small_cave.as_ref() {
                                // OK, you get one repeat
                                blessed_small_cave.take();
                                used_blessed = true;
                            } else {
                                continue;
                            }
                        }

                        path.push(current_cave.clone());
                        total += aux(connections, visited_caves, path, candidate.clone(), blessed_small_cave, all_paths);
                        path.pop();

                        if used_blessed {
                            let _ = blessed_small_cave.insert(candidate.clone());
                            visited_caves.insert(candidate.clone());
                        }
                    }

                    total
                } else {
                    0
                };

            visited_caves.remove(&current_cave);

            subcount
        }

        let mut visited_caves = HashSet::new();
        let mut path = Vec::new();
        let mut blessed_small_cave = Some(blessed_small_cave);

        aux(connections,
            &mut visited_caves,
            &mut path,
            Cave::from_str("start"),
            &mut blessed_small_cave,
            all_paths);

        all_paths.len()
    }


    pub fn part2() {
        let mut cave_connections: HashMap<Cave, Vec<Cave>> = HashMap::new();

        for line in input_lines("input_files/day12.txt") {
            let mut it = line.split('-');
            let from = Cave::from_str(it.next().unwrap());
            let to = Cave::from_str(it.next().unwrap());

            // a -> b
            {
                let entry = cave_connections.entry(from.clone()).or_insert_with(Vec::new);
                entry.push(to.clone());
            }

            // b -> a
            {
                let entry = cave_connections.entry(to.clone()).or_insert_with(Vec::new);
                entry.push(from.clone());
            }
        }

        let mut all_paths = HashSet::new();

        for blessed_cave in cave_connections.keys().filter(|&cave| !cave.is_big && (cave.name != "start" && cave.name != "end")) {
            count_paths_pt2(&cave_connections, blessed_cave.clone(), &mut all_paths);
        }

        println!("Total: {}", all_paths.len());
    }
}

mod day13 {
    use crate::shared::*;

    fn print_grid(grid: &HashSet<(usize, usize)>) {
        let max_row = *grid.iter().map(|(_, row)| row).max().unwrap();
        let max_col = *grid.iter().map(|(col, _)| col).max().unwrap();

        for row in 0..=max_row {
            for col in 0..=max_col {
                if grid.contains(&(col, row)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    pub fn part1() {
        let mut lines = input_lines("input_files/day13.txt");

        let mut grid: HashSet<(usize, usize)> = HashSet::new();

        // Parse the grid
        for line in &mut lines {
            if line.is_empty() {
                break;
            }

            let mut it = line.split(',');
            let x = it.next().unwrap().parse().unwrap();
            let y = it.next().unwrap().parse().unwrap();

            grid.insert((x, y));
        }

        let fold_instruction = Regex::new(r"fold along ([xy])=([0-9]+)").unwrap();

        // apply our first fold
        if let Some(line) = lines.next() {
            let mut it = fold_instruction.captures_iter(&line);
            let caps = it.next().unwrap();

            let fold_orientation = &caps[1];
            let fold_offset: usize = caps[2].parse().unwrap();

            let mut new_grid: HashSet<(usize, usize)> = HashSet::new();

            if fold_orientation == "y" {
                for (x, y) in grid {
                    if y > fold_offset {
                        let new_y = fold_offset - (y - fold_offset);
                        new_grid.insert((x, new_y));
                    } else {
                        new_grid.insert((x, y));
                    }
                }
            } else {
                for (x, y) in grid {
                    if x > fold_offset {
                        let new_x = fold_offset - (x - fold_offset);
                        new_grid.insert((new_x, y));
                    } else {
                        new_grid.insert((x, y));
                    }
                }
            }

            grid = new_grid;
        }

        println!("There are {} visible dots", grid.len());
    }

    pub fn part2() {
        let mut lines = input_lines("input_files/day13.txt");

        let mut grid: HashSet<(usize, usize)> = HashSet::new();

        // Parse the grid
        for line in &mut lines {
            if line.is_empty() {
                break;
            }

            let mut it = line.split(',');
            let x = it.next().unwrap().parse().unwrap();
            let y = it.next().unwrap().parse().unwrap();

            grid.insert((x, y));
        }

        let fold_instruction = Regex::new(r"fold along ([xy])=([0-9]+)").unwrap();

        // apply our first fold
        for line in &mut lines {
            let mut it = fold_instruction.captures_iter(&line);
            let caps = it.next().unwrap();

            let fold_orientation = &caps[1];
            let fold_offset: usize = caps[2].parse().unwrap();

            let mut new_grid: HashSet<(usize, usize)> = HashSet::new();

            if fold_orientation == "y" {
                for (x, y) in grid {
                    if y > fold_offset {
                        let new_y = fold_offset - (y - fold_offset);
                        new_grid.insert((x, new_y));
                    } else {
                        new_grid.insert((x, y));
                    }
                }
            } else {
                for (x, y) in grid {
                    if x > fold_offset {
                        let new_x = fold_offset - (x - fold_offset);
                        new_grid.insert((new_x, y));
                    } else {
                        new_grid.insert((x, y));
                    }
                }
            }

            grid = new_grid;
        }

        print_grid(&grid);
    }


}

mod day14 {
    use crate::shared::*;

    struct Insertion {
        s: String,
        idx: usize,
    }

    pub fn part1() {
        let mut lines = input_lines("input_files/day14.txt");

        let mut template: String = lines.next().unwrap();

        // Eat the blank line
        let _ = lines.next().unwrap();

        let rule_pattern = Regex::new(r"(.+) -> (.+)").unwrap();
        let mut substitutions: HashMap<String, String> = HashMap::new();

        for rule in &mut lines {
            let mut it = rule_pattern.captures_iter(&rule);
            let caps = it.next().unwrap();

            let from = &caps[1];
            let to = &caps[2];

            substitutions.insert(from.to_string(), to.to_string());
        }

        for _ in 0..10 {
            // Find the chars to insert
            let mut to_insert = Vec::new();
            for idx in 0..(template.len() - 1) {
                if let Some(s) = substitutions.get(&template[idx..idx+2]) {
                    to_insert.push(Insertion { s: s.clone(), idx: idx + 1});
                }
            }

            if to_insert.is_empty() {
                break;
            }

            for insert in to_insert.into_iter().rev() {
                template.insert_str(insert.idx, &insert.s);
            }
        }

        let mut freqs: HashMap<char, usize> = HashMap::new();
        for ch in template.chars() {
            *freqs.entry(ch).or_insert(0) += 1;
        }

        let min = freqs.iter().map(|(_, v)| v).min().unwrap();
        let max = freqs.iter().map(|(_, v)| v).max().unwrap();

        println!("Difference: {}", max - min);
    }

    pub fn part2() {
        let mut lines = input_lines("input_files/day14.txt");

        let template: String = lines.next().unwrap();

        // Eat the blank line
        let _ = lines.next().unwrap();

        let rule_pattern = Regex::new(r"(.+) -> (.+)").unwrap();
        let mut substitutions: HashMap<String, String> = HashMap::new();

        for rule in &mut lines {
            let mut it = rule_pattern.captures_iter(&rule);
            let caps = it.next().unwrap();

            let from = &caps[1];
            let to = &caps[2];

            substitutions.insert(from.to_string(), to.to_string());
        }

        let mut pairs: HashMap<String, usize> = HashMap::new();

        for idx in 0..(template.len() - 1) {
            *pairs.entry(template[idx..idx + 2].to_string()).or_insert(0) += 1;
        }

        for _ in 0..40 {
            let mut new_pairs: HashMap<String, usize> = HashMap::new();

            for (pair, to_insert) in substitutions.iter() {
                if let Some(freq) = pairs.remove(pair) {
                    *new_pairs.entry(format!("{}{}", pair.chars().next().unwrap(), to_insert)).or_insert(0) += freq;
                    *new_pairs.entry(format!("{}{}", to_insert, pair.chars().nth(1).unwrap())).or_insert(0) += freq;
                }
            }

            new_pairs.extend(pairs);
            pairs = new_pairs;
        }


        let mut freqs: HashMap<char, usize> = HashMap::new();
        for k in pairs.keys() {
            for ch in k.chars().take(1) {
                *freqs.entry(ch).or_insert(0) += pairs[k];
            }
        }

        // Since we only counted the first of each pair (to avoid duplication), we need
        // to manually add the last character from the original template.  Fortunately,
        // the first and last characters of the template cannot be changed by our rules.
        *freqs.entry(template.chars().last().unwrap()).or_insert(0) += 1;

        let min = freqs.iter().map(|(_, v)| v).min().unwrap();
        let max = freqs.iter().map(|(_, v)| v).max().unwrap();

        println!("Difference: {}", max - min);
    }
}

mod day15 {
    use crate::shared::*;

    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, PartialOrd, Ord)]
    struct Vertex {
        row: usize,
        col: usize,
    }

    impl Vertex {
        fn neighbours(&self, width: usize, height: usize) -> Vec<Vertex> {
            let mut result = Vec::with_capacity(4);

            if self.col > 0 {
                result.push(Vertex { row: self.row, col: self.col - 1 });
            }

            if self.col < width - 1 {
                result.push(Vertex { row: self.row, col: self.col + 1 });
            }

            if self.row > 0 {
                result.push(Vertex { row: self.row - 1, col: self.col });
            }

            if self.row < height - 1 {
                result.push(Vertex { row: self.row + 1, col: self.col });
            }

            result
        }
    }

    pub fn part1() {
        let lines = input_lines("input_files/day15.txt");
        let grid: Vec<Vec<_>> = lines.map(|row| row.chars().map(|ch| ch.to_digit(10).unwrap() as usize).collect()).collect();

        let source = Vertex { row: 0, col: 0 };

        let width = grid[0].len();
        let height = grid.len();

        let mut queue: HashSet<Vertex> = HashSet::new();
        let mut dist: HashMap<Vertex, usize> = HashMap::new();
        let mut prev: HashMap<Vertex, Option<Vertex>> = HashMap::new();

        for row in 0..height {
            for col in 0..width {
                let v = Vertex { row, col };
                queue.insert(v);
                prev.insert(v, None);
                dist.insert(v, usize::MAX);
            }
        }

        dist.insert(source, 0);

        while !queue.is_empty() {
            let u = *queue.iter().min_by_key(|v| dist.get(v)).unwrap();

            queue.remove(&u);

            for neighbour in u.neighbours(width, height) {
                if !queue.contains(&neighbour) {
                    continue;
                }

                let alt = dist.get(&u).unwrap().saturating_add(grid[neighbour.row][neighbour.col]);
                if alt < *dist.get(&neighbour).unwrap() {
                    dist.insert(neighbour, alt);
                    prev.insert(neighbour, Some(u));
                }
            }
        }

        let mut total_risk: usize = 0;

        let mut u = Vertex { row: height - 1, col: width - 1 };
        if prev.get(&u).is_some() || u == source {
            loop {
                total_risk += grid[u.row][u.col];
                if let Some(p) = prev.get(&u).unwrap() {
                    u = *p;
                } else {
                    break;
                }
            }
        }

        println!("Risk: {}", total_risk - grid[0][0]);
    }

    fn project_grid(grid: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let width = grid[0].len();
        let height = grid.len();

        let projected_width = width * 5;
        let projected_height = height * 5;

        let mut result: Vec<Vec<usize>> = (0..projected_height).map(|_| vec![0; projected_width]).collect();

        for row in 0..projected_height {
            for col in 0..projected_width {
                let adjustment = (row / height) + (col / width);

                result[row][col] = (((grid[row % height][col % width] - 1) + adjustment) % 9) + 1;
            }
        }

        result
    }

    #[derive(Eq, PartialEq, Copy, Clone)]
    struct WeightedVertex {
        dist: usize,
        vertex: Vertex,
    }

    impl PartialOrd for WeightedVertex {
        fn partial_cmp(&self, other: &WeightedVertex) -> Option<std::cmp::Ordering> {
            if self.dist == other.dist {
                self.vertex.partial_cmp(&other.vertex)
            } else {
                self.dist.partial_cmp(&other.dist)
            }
        }
    }

    impl Ord for WeightedVertex {
        fn cmp(&self, other: &WeightedVertex) -> std::cmp::Ordering {
            if self.dist == other.dist {
                self.vertex.cmp(&other.vertex)
            } else {
                self.dist.cmp(&other.dist)
            }
        }
    }



    pub fn part2() {
        let lines = input_lines("input_files/day15.txt");
        let grid: Vec<Vec<_>> = project_grid(lines.map(|row| row.chars().map(|ch| ch.to_digit(10).unwrap() as usize).collect()).collect());

        let width = grid[0].len();
        let height = grid.len();

        let source = Vertex { row: 0, col: 0 };
        let target = Vertex { row: height - 1, col: width - 1 };

        let mut queue: HashSet<Vertex> = HashSet::new();
        let mut heap: BTreeMap<WeightedVertex, usize> = BTreeMap::new();

        let mut dist: HashMap<Vertex, usize> = HashMap::new();
        let mut prev: HashMap<Vertex, Option<Vertex>> = HashMap::new();

        for row in 0..height {
            for col in 0..width {
                let v = Vertex { row, col };
                prev.insert(v, None);

                queue.insert(v);

                if v == source {
                    heap.insert(WeightedVertex { dist: 0, vertex: v }, 0);
                    dist.insert(v, 0);
                } else {
                    heap.insert(WeightedVertex { dist: usize::MAX, vertex: v }, usize::MAX);
                    dist.insert(v, usize::MAX);
                }
            }
        }

        while !queue.is_empty() {
            let entry = *heap.keys().next().unwrap();
            let u = entry.vertex;

            heap.remove(&entry);
            queue.remove(&u);

            if u == target {
                break;
            }

            for neighbour in u.neighbours(width, height) {
                if !queue.contains(&neighbour) {
                    continue;
                }

                let alt = dist.get(&u).unwrap().saturating_add(grid[neighbour.row][neighbour.col]);
                let old_dist = *dist.get(&neighbour).unwrap();
                if alt < old_dist {
                    heap.remove(&WeightedVertex { vertex: neighbour, dist: old_dist });
                    dist.insert(neighbour, alt);
                    prev.insert(neighbour, Some(u));
                    heap.insert(WeightedVertex { vertex: neighbour, dist: alt }, alt);
                }
            }
        }

        let mut total_risk: usize = 0;

        let mut u = target;
        if prev.get(&u).is_some() || u == source {
            loop {
                total_risk += grid[u.row][u.col];
                if let Some(p) = prev.get(&u).unwrap() {
                    u = *p;
                } else {
                    break;
                }
            }
        }

        println!("Risk: {}", total_risk - grid[0][0]);
    }
}


mod day16 {
    use crate::shared::*;

    struct BitStream {
        bytes: Vec<u8>,
        bit_offset: usize,
    }

    impl BitStream {
        fn from_hex(s: &str) -> BitStream {
            BitStream {
                bytes: (0..s.len()).step_by(2).map(|idx| u8::from_str_radix(&s[idx..idx+2], 16).unwrap()).collect(),
                bit_offset: 0,
            }
        }

        fn read_usize(&mut self, bitcount: usize) -> usize {
            let mut result: usize = 0;

            for _ in 0..bitcount {
                let current_byte_offset = self.bit_offset / 8;

                result *= 2;
                result |= ((self.bytes[current_byte_offset] >> (7 - (self.bit_offset % 8))) & 0x01) as usize;
                self.bit_offset += 1;
            }

            result
        }
    }


    #[derive(Debug)]
    enum Packet {
        Literal {
            version: usize,
            type_id: usize,
            value: usize,
        },
        Operator {
            version: usize,
            type_id: usize,
            subpackets: Vec<Packet>,
        }
    }

    fn read_packet(stream: &mut BitStream) -> Packet {
        let version = stream.read_usize(3);
        let type_id = stream.read_usize(3);

        if type_id == 4 {
            // literal value
            let mut value = 0;

            loop {
                let segment = stream.read_usize(5);
                value <<= 4;
                value |= (segment & 0xF);

                if (segment >> 4) & 0x1 == 0x0 {
                    break;
                }
            }

            Packet::Literal {
                version,
                type_id,
                value
            }
        } else {
            // Operator packet
            let length_type_id = stream.read_usize(1);

            match length_type_id {
                0 => {
                    let length_of_subpackets_in_bits = stream.read_usize(15);

                    let starting_position = stream.bit_offset;

                    let mut subpackets = Vec::new();

                    while (stream.bit_offset - starting_position) < length_of_subpackets_in_bits {
                        subpackets.push(read_packet(stream));
                    }

                    Packet::Operator {
                        version,
                        type_id,
                        subpackets,
                    }
                },
                1 => {
                    let number_of_subpackets = stream.read_usize(11);

                    Packet::Operator {
                        version,
                        type_id,
                        subpackets: (0..number_of_subpackets).map(|_| read_packet(stream)).collect(),
                    }
                },
                _ => panic!("Invalid length type id: {}", length_type_id),
            }
        }
    }

    fn sum_versions(packet: &Packet) -> usize {
        match packet {
            Packet::Literal { version, .. } => *version,
            Packet::Operator { version, subpackets, .. } => {
                *version + subpackets.iter().map(|p| sum_versions(p)).sum::<usize>()
            }
        }
    }

    pub fn part1() {
        let mut lines = input_lines("input_files/day16.txt");
        let mut stream = BitStream::from_hex(&lines.next().unwrap());

        let packet = read_packet(&mut stream);

        println!("Sum of version numbers: {}", sum_versions(&packet));
    }

    fn evaluate(packet: &Packet) -> usize {
        match packet {
            Packet::Literal { value, .. } => *value,
            Packet::Operator { type_id, subpackets, .. } => {
                match type_id {
                    0 => {
                        // Sum packet
                        subpackets.iter().map(|p| evaluate(p)).sum::<usize>()
                    }
                    1 => {
                        // Product packet
                        subpackets.iter().map(|p| evaluate(p)).product::<usize>()
                    }
                    2 => {
                        // Minimum packet
                        subpackets.iter().map(|p| evaluate(p)).min().unwrap()
                    }
                    3 => {
                        // Maximum packet
                        subpackets.iter().map(|p| evaluate(p)).max().unwrap()
                    }
                    5 => {
                        // Greater than packet
                        if evaluate(&subpackets[0]) > evaluate(&subpackets[1]) {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        // Less than packet
                        if evaluate(&subpackets[0]) < evaluate(&subpackets[1]) {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        // Equal packet
                        if evaluate(&subpackets[0]) == evaluate(&subpackets[1]) {
                            1
                        } else {
                            0
                        }

                    }
                    _ => panic!("Invalid packet type: {}", type_id),
                }
            }
        }

    }

    pub fn part2() {
        let mut lines = input_lines("input_files/day16.txt");
        let mut stream = BitStream::from_hex(&lines.next().unwrap());

        let packet = read_packet(&mut stream);

        println!("Packet result: {}", evaluate(&packet));
    }

}


mod day17 {
    use crate::shared::*;

    pub fn part1() {
        // let target_xmin: i64 = 20;
        // let target_xmax: i64 = 30;

        // let target_ymin: i64 = -10;
        // let target_ymax: i64 = -5;

        let target_xmin: i64 = 185;
        let target_xmax: i64 = 221;

        let target_ymin: i64 = -122;
        let target_ymax: i64 = -74;

        let mut best_y: i64 = i64::MIN;

        for start_x_velocity in -1000..1000 {
            for start_y_velocity in -1000..1000 {

                let mut x_velocity = start_x_velocity;
                let mut y_velocity = start_y_velocity;

                let mut x: i64 = 0;
                let mut y: i64 = 0;

                let mut best_y_this_round: i64 = i64::MIN;

                // let mut step: usize = 0;
                loop {
                    if (x >= target_xmin && x <= target_xmax) && (y >= target_ymin && y <= target_ymax) {
                        // println!("Hit on step {}", step);

                        if best_y_this_round > best_y {
                            best_y = best_y_this_round;
                        }

                        break;
                    }

                    if x_velocity == 0 && y_velocity < 0 && y < target_ymin {
                        break;
                    }

                    x += x_velocity;
                    y += y_velocity;

                    if y > best_y_this_round {
                        best_y_this_round = y;
                    }

                    if x_velocity != 0 {
                        x_velocity -= (x_velocity.abs() / x_velocity);
                    }

                    y_velocity -= 1;

                    // step += 1;
                    // println!("After step {}: x = {}; y = {}", step, x, y);
                }
            }
        }

        println!("Best y: {}", best_y);
    }

    pub fn part2() {
        // let target_xmin: i64 = 20;
        // let target_xmax: i64 = 30;
        //
        // let target_ymin: i64 = -10;
        // let target_ymax: i64 = -5;

        let target_xmin: i64 = 185;
        let target_xmax: i64 = 221;

        let target_ymin: i64 = -122;
        let target_ymax: i64 = -74;

        let mut target_hit: HashSet<(i64, i64)> = HashSet::new();

        for start_x_velocity in -1000..1000 {
            for start_y_velocity in -1000..1000 {

                let mut x_velocity = start_x_velocity;
                let mut y_velocity = start_y_velocity;

                let mut x: i64 = 0;
                let mut y: i64 = 0;

                // let mut step: usize = 0;
                loop {
                    if (x >= target_xmin && x <= target_xmax) && (y >= target_ymin && y <= target_ymax) {
                        // println!("Hit on step {}", step);
                        target_hit.insert((start_x_velocity, start_y_velocity));
                        break;
                    }

                    if x_velocity == 0 && y_velocity < 0 && y < target_ymin {
                        break;
                    }

                    x += x_velocity;
                    y += y_velocity;

                    if x_velocity != 0 {
                        x_velocity -= (x_velocity.abs() / x_velocity);
                    }

                    y_velocity -= 1;

                    // step += 1;
                    // println!("After step {}: x = {}; y = {}", step, x, y);
                }
            }
        }

        println!("Unique values: {}", target_hit.len());
    }

}

mod day18 {
    use crate::shared::*;

    #[derive(Debug, Eq, PartialEq, Clone)]
    enum Token {
        StartPair,
        EndPair,
        Number(usize),
    }

    fn tokenise_tree(s: &str) -> Vec<Token> {
        let mut result = Vec::new();
        let mut it = s.chars().peekable();

        while let Some(ch) = it.next() {
            match ch {
                '[' => result.push(Token::StartPair),
                ']' => result.push(Token::EndPair),
                '0'..='9' => {
                    let mut n = ch.to_digit(10).unwrap() as usize;
                    while let Some(&next_ch) = it.peek() {
                        if let Some(digit) = next_ch.to_digit(10) {
                            let _ = it.next();
                            n *= 10;
                            n += digit as usize;
                        } else {
                            break;
                        }
                    }

                    result.push(Token::Number(n))
                },
                _ => {},
            }
        }

        result
    }

    fn explode(tokens: &mut Vec<Token>) -> bool {
        let mut depth = 0;

        for idx in 0..tokens.len() {
            let token = &tokens[idx];

            match token {
                Token::StartPair => {
                    if depth == 4 {
                        // This pair should always consist of two regular numbers, so sayeth the
                        // problem.
                        assert_eq!(tokens[idx], Token::StartPair);
                        assert!(matches!(tokens[idx + 1], Token::Number(_n)));
                        assert!(matches!(tokens[idx + 2], Token::Number(_n)));
                        assert_eq!(tokens[idx + 3], Token::EndPair);

                        // Scan left for our number to add to
                        if let Token::Number(lhs) = tokens[idx + 1] {
                            for n_idx in (0..idx).rev() {
                                if let Token::Number(n) = tokens[n_idx] {
                                    tokens[n_idx] = Token::Number(n + lhs);
                                    break;
                                }
                            }
                        } else {
                            unreachable!();
                        }

                        // Scan right for our number to add to
                        if let Token::Number(rhs) = tokens[idx + 2] {
                            for token in tokens.iter_mut().skip(idx + 4) {
                                if let Token::Number(n) = token {
                                    *token = Token::Number(*n + rhs);
                                    break;
                                }
                            }
                        } else {
                            unreachable!();
                        }

                        // Replace our pair with a zero
                        for _ in 0..4 {
                            tokens.remove(idx);
                        }

                        tokens.insert(idx, Token::Number(0));

                        return true;
                    } else {
                        depth += 1;
                    }
                }
                Token::EndPair => depth -= 1,
                _ => {}
            }
        }

        false
    }

    fn split(tokens: &mut Vec<Token>) -> bool {
        for idx in 0..tokens.len() {
            if let Token::Number(n) = tokens[idx] {
                if n >= 10 {
                    tokens.remove(idx);
                    tokens.insert(idx, Token::EndPair);
                    tokens.insert(idx, Token::Number((n as f64 / 2.0_f64).ceil() as usize));
                    tokens.insert(idx, Token::Number((n as f64 / 2.0_f64).floor() as usize));
                    tokens.insert(idx, Token::StartPair);

                    return true;
                }
            }
        }

        false
    }


    #[allow(clippy::vec_init_then_push)]
    fn sum_snailfish(n1: &[Token], n2: &[Token]) -> Vec<Token> {
        let mut result: Vec<Token> = Vec::new();

        result.push(Token::StartPair);
        result.extend(n1.iter().cloned());
        result.extend(n2.iter().cloned());
        result.push(Token::EndPair);

        loop {
            if explode(&mut result) || split(&mut result) {
                continue;
            } else {
                break;
            }
        }

        result
    }

    #[derive(Debug)]
    enum SnailfishNumber {
        Pair {
            lhs: Box<SnailfishNumber>,
            rhs: Box<SnailfishNumber>,
        },
        Number(usize)
    }

    fn parse_tree(tokens: Vec<Token>) -> SnailfishNumber {
        fn parse_next(tokens: &[Token], offset: usize) -> (SnailfishNumber, usize) {
            match tokens[offset] {
                Token::StartPair => {
                    let (lhs, next_offset) = parse_next(tokens, offset + 1);
                    let (rhs, next_offset) = parse_next(tokens, next_offset);

                    // Eat the EndPair
                    assert_eq!(tokens[next_offset], Token::EndPair);

                    (SnailfishNumber::Pair { lhs: Box::new(lhs), rhs: Box::new(rhs) }, next_offset + 1)
                }
                Token::Number(n) => (SnailfishNumber::Number(n), offset + 1),
                _ => panic!("Parse error"),
            }
        }

        parse_next(&tokens, 0).0
    }

    fn magnitude(tree: &SnailfishNumber) -> usize {
        match tree {
            SnailfishNumber::Number(n) => *n,
            SnailfishNumber::Pair { lhs, rhs } => {
                3 * magnitude(lhs) + 2 * magnitude(rhs)
            }
        }
    }

    pub fn part1() {
        let numbers = input_lines("input_files/day18.txt").map(|s| tokenise_tree(&s));
        let sum = numbers.reduce(|result, n| sum_snailfish(&result, &n)).unwrap();

        println!("Magnitude: {}", magnitude(&parse_tree(sum)));
    }

    pub fn part2() {
        let numbers: Vec<Vec<Token>> = input_lines("input_files/day18.txt").map(|s| tokenise_tree(&s)).collect();

        let mut best_magnitude = usize::MIN;

        for i in 0..numbers.len() {
            for j in 0..numbers.len() {
                if i == j {
                    continue;
                }

                let sum = sum_snailfish(&numbers[i], &numbers[j]);
                best_magnitude = std::cmp::max(best_magnitude, magnitude(&parse_tree(sum)));
            }
        }

        println!("Best magnitude: {}", best_magnitude);
    }

}


mod day19 {
    use crate::shared::*;

    // Well, this was a bit of a maths refresher...
    //
    // There are 24 possible rotations of our collection of beacon coordinates,
    // corresponding to the sensor facing one of six ways, while being rotated
    // (rolled) in one of four orientations.
    //
    // A straightforward way of generating rotations for our points is to enumerate
    // every possible permutation of (x, y, z) along with every possible sign flip
    // of each coordinate.  That would yield (6 * 4 * 2) = 48 different rotations.
    //
    // However, half of those are reflections rather than pure rotations, so half of
    // those aren't applicable.  But which ones?!
    //
    // Instead of thinking in terms of permutations, we can use a rotation matrices
    // (https://en.wikipedia.org/wiki/Rotation_matrix).  For rotations 90 degree
    // multiples of three dimensions, our rotation matrix will consist of three rows
    // and three columns, with a single 1 or -1) in each row and each column.
    //
    // As with the permutations approach, this gives you 48 possible rotation
    // matrices, with 24 reflections.  But One Weird Trick the Internet tells me is
    // that a "proper rotation" (i.e. non-reflected) will have a rotation matrix
    // whose determinant is 1.
    //
    // So, the plan is: build our 24 proper rotation matrices.  Apply every rotation
    // to every set of beacons, then brute force to find where the sensors overlap.

    fn determinant_3x3(matrix: &[[i64; 3]]) -> i64 {
        (matrix[0][0] * (matrix[1][1] * matrix[2][2] - matrix[1][2] * matrix[2][1]) -
         matrix[0][1] * (matrix[1][0] * matrix[2][2] - matrix[1][2] * matrix[2][0]) +
         matrix[0][2] * (matrix[1][0] * matrix[2][1] - matrix[1][1] * matrix[2][0]))
    }


    fn build_rotation_matrices() -> Vec<Vec<[i64; 3]>> {
        // Starting with a nice trick from Knuth TAOCP 4a.  You can generate all
        // possible rotation matrices (with exactly one 1 in each row and column) by
        // taking all permutations of (e.g.) 012, writing the permutation alongside the
        // ordered digits like:
        //
        //    012
        //    210  <- permutation
        //
        // then pairing them off and setting the bits at those positions (0,2), (1,1),
        // (2,0).
        //
        // Since I want all permutations of 1's and -1's, I'm combining this with a
        // bit-wrangling trick to generate those variants too.

        let mut matrices = Vec::new();

        for perm in (0..=2).permutations(3) {
            for sign_mask in (0..=7) {
                let mut matrix = vec![[0, 0, 0],
                                      [0, 0, 0],
                                      [0, 0, 0]];

                for (x, &y) in (0..=3).zip(&perm) {
                    if sign_mask & (1 << x) == 0 {
                        matrix[x][y] = 1;
                    } else {
                        matrix[x][y] = -1;
                    }
                }

                if (determinant_3x3(&matrix) == 1) {
                    matrices.push(matrix);
                }
            }
        }

        matrices
    }


    #[derive(Debug, Hash, Eq, PartialEq, Clone)]
    #[allow(clippy::upper_case_acronyms)]
    struct XYZ {
        x: i64,
        y: i64,
        z: i64,
    }

    impl XYZ {
        fn subtract(&self, other: &XYZ) -> XYZ {
            XYZ {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }

        fn manhattan_distance(&self, other: &XYZ) -> i64 {
            (other.x - self.x).abs() +
                (other.y - self.y).abs() +
                (other.z - self.z).abs()
        }
    }

    #[derive(Debug, Clone)]
    struct Scanner {
        number: usize,
        beacons: HashSet<XYZ>,
    }

    struct ScannerRotations {
        scanner: Scanner,
        rotations: Vec<Scanner>,
    }

    fn parse_scanners(it: impl Iterator<Item=String>) -> Vec<Scanner> {
        let mut result: Vec<Scanner> = Vec::new();

        let mut current_scanner = None;

        for line in it {
            if line.starts_with("---") {
                if current_scanner.is_some() { result.push(current_scanner.take().unwrap()); }

                let number = line.split(' ').nth(2).map(|s| s.parse::<usize>().ok()).flatten().unwrap();
                current_scanner = Some(Scanner { number, beacons: HashSet::new() });
            } else if line.is_empty() {
                continue;
            } else {
                let coords: Vec<i64> = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect();

                if let Some(s) = current_scanner.as_mut() {
                    s.beacons.insert(XYZ {
                        x: coords[0],
                        y: coords[1],
                        z: coords[2],
                    });
                }
            }
        }

        if current_scanner.is_some() { result.push(current_scanner.take().unwrap()); }

        result
    }

    fn matrix_multiply(coord: &XYZ, rotation: &[[i64; 3]]) -> XYZ {
        XYZ {
            x: (coord.x * rotation[0][0]) + (coord.y * rotation[1][0]) + (coord.z * rotation[2][0]),
            y: (coord.x * rotation[0][1]) + (coord.y * rotation[1][1]) + (coord.z * rotation[2][1]),
            z: (coord.x * rotation[0][2]) + (coord.y * rotation[1][2]) + (coord.z * rotation[2][2]),
        }
    }

    fn generate_rotations(scanners: Vec<Scanner>, rotations: &[Vec<[i64; 3]>]) -> Vec<ScannerRotations> {
        let mut result = Vec::new();

        for scanner in scanners {
            let base = scanner.clone();
            let mut scanner_rotations = ScannerRotations {
                scanner,
                rotations: Vec::new(),
            };

            for rotation in rotations {
                scanner_rotations.rotations.push(Scanner {
                    number: base.number,
                    beacons: base.beacons.iter().map(|coord| matrix_multiply(coord, rotation)).collect()
                });
            }

            result.push(scanner_rotations);
        }

        result
    }

    struct AlignmentResult {
        mapped_beacons: HashSet<XYZ>,
        offset: XYZ,
    }

    fn attempt_scanner_alignment(target: &Scanner, candidate: &Scanner) -> Option<AlignmentResult> {
        for target_point in target.beacons.iter() {
            for candidate_point in candidate.beacons.iter() {
                let offset = candidate_point.subtract(target_point);

                let offset_candidates: HashSet<XYZ> = candidate.beacons.iter().map(|XYZ { x, y, z }| XYZ {
                    x: x - offset.x,
                    y: y - offset.y,
                    z: z - offset.z,
                }).collect();

                let len = offset_candidates.intersection(&target.beacons).count();

                if len >= 12 {
                    return Some(AlignmentResult {
                        mapped_beacons: offset_candidates,
                        offset,
                    });
                }
            }
        }

        None
    }

    pub fn part1() {
        let three_by_three_rotation_matrices = build_rotation_matrices();
        let scanners: Vec<Scanner> = parse_scanners(input_lines("input_files/day19.txt"));

        let mut all_rotations: VecDeque<ScannerRotations> = generate_rotations(scanners, &three_by_three_rotation_matrices).into_iter().collect();

        // Arbitrarily pick a scanner as our starting point and required orientation
        let mut merged_scanner = all_rotations.pop_front().unwrap().scanner;

        // Find a scanner that overlaps with our requisite 12 beacons in one of its orientations
        while !all_rotations.is_empty() {
            let candidate_scanner_rotations = all_rotations.pop_front().unwrap();

            let mut success = false;
            for rotation in &candidate_scanner_rotations.rotations {
                if let Some(alignment_result) = attempt_scanner_alignment(&merged_scanner, rotation) {
                    success = true;
                    merged_scanner.beacons.extend(alignment_result.mapped_beacons);
                    break;
                }
            }

            if !success {
                // Back you go for next time
                all_rotations.push_back(candidate_scanner_rotations);
            }
        }

        println!("We found {} beacons", merged_scanner.beacons.len());
    }

    pub fn part2() {
        let three_by_three_rotation_matrices = build_rotation_matrices();
        let scanners: Vec<Scanner> = parse_scanners(input_lines("input_files/day19.txt"));

        let mut all_rotations: VecDeque<ScannerRotations> = generate_rotations(scanners, &three_by_three_rotation_matrices).into_iter().collect();

        // Arbitrarily pick a scanner as our starting point and required orientation
        let mut merged_scanner = all_rotations.pop_front().unwrap().scanner;

        let mut offsets = Vec::new();

        // Find a scanner that overlaps with our requisite 12 beacons in one of its orientations
        while !all_rotations.is_empty() {
            let candidate_scanner_rotations = all_rotations.pop_front().unwrap();

            let mut success = false;
            for rotation in &candidate_scanner_rotations.rotations {
                if let Some(alignment_result) = attempt_scanner_alignment(&merged_scanner, rotation) {
                    success = true;
                    merged_scanner.beacons.extend(alignment_result.mapped_beacons);
                    offsets.push(alignment_result.offset);
                    break;
                }
            }

            if !success {
                // Back you go for next time
                all_rotations.push_back(candidate_scanner_rotations);
            }
        }

        let mut max_distance: i64 = 0;

        for i in 0..offsets.len() {
            for j in (i + 1)..offsets.len() {
                let offset_a = &offsets[i];
                let offset_b = &offsets[j];

                let distance = offset_a.manhattan_distance(offset_b);

                if distance > max_distance {
                    max_distance = distance;
                }
            }
        }

        println!("Max distance between scanners: {}", max_distance);
    }
}

mod day20 {
    use crate::shared::*;

    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
    struct Position (i64, i64);

    #[derive(Debug, Default)]
    struct Image {
        pixels: HashMap<Position, char>,
        top_left_row: i64,
        top_left_col: i64,
        bottom_right_row: i64,
        bottom_right_col: i64,
    }

    impl Image {
        fn new() -> Image {
            Default::default()
        }

        fn set_pixel(&mut self,
                     position: Position,
                     ch: char) {
            self.top_left_row = std::cmp::min(self.top_left_row, position.0);
            self.top_left_col = std::cmp::min(self.top_left_col, position.1);
            self.bottom_right_row = std::cmp::max(self.bottom_right_row, position.0);
            self.bottom_right_col = std::cmp::max(self.bottom_right_col, position.1);

            self.pixels.insert(position, ch);
        }

        fn positions(&self) -> Vec<Position> {
            let mut result = Vec::new();

            let margin = 10;

            for row in ((self.top_left_row - margin)..=(self.bottom_right_row + margin)) {
                for col in ((self.top_left_col - margin)..=(self.bottom_right_col + margin)) {
                    result.push(Position(row, col));
                }
            }

            result
        }

        fn count_lit(&self) -> usize {
            self.pixels.values().filter(|&&ch| ch == '#').count()
        }

        fn read_value(&self, position: Position, default_value: usize) -> usize {
            let mut result = 0;

            for &(offset_row, offset_col) in &[
                (-1, -1),  (-1, 0),  (-1, 1),
                (0,  -1),  (0,  0),  (0,  1),
                (1,  -1),  (1,  0),  (1,  1),
            ] {
                let p = Position((position.0 + offset_row), (position.1 + offset_col));

                let bit = match self.pixels.get(&p) {
                    Some('#') => 1,
                    Some('.') => 0,
                    _ => default_value,
                };


                result *= 2;
                result += bit;
            }

            result
        }

        fn print(&self) {
            let margin = 0;

            for row in ((self.top_left_row - margin)..=(self.bottom_right_row + margin)) {
                for col in ((self.top_left_col - margin)..=(self.bottom_right_col + margin)) {
                    let p = Position(row, col);

                    print!("{}",
                           match self.pixels.get(&p) {
                               Some(ch) => *ch,
                               _ => '.'
                           });
                }
                println!();
            }
        }
    }

    fn parse_image(lines: Vec<String>) -> Image {
        let mut result = Image::new();

        for row in 0..lines.len() {
            for (col, ch) in lines[row].chars().enumerate() {
                result.set_pixel(Position(row as i64, col as i64), ch)
            }
        }

        result
    }

    pub fn part1() {
        let mut it = input_lines("input_files/day20.txt");

        let enhancement_algorithm: Vec<char> = it.next().unwrap().chars().collect();
        let _ = it.next();

        let mut image = parse_image(it.collect());

        for round in 0..2 {
            let default_value = (round % 2) as usize;

            let mut next_image = Image::new();

            for position in image.positions() {
                let ch = enhancement_algorithm[image.read_value(position, default_value)];

                next_image.set_pixel(position, ch);
            }

            image = next_image;
        }

        image.print();

        println!("Pixels are LIT: {}", image.count_lit());
    }

    pub fn part2() {
        let mut it = input_lines("input_files/day20.txt");

        let enhancement_algorithm: Vec<char> = it.next().unwrap().chars().collect();
        let _ = it.next();

        let mut image = parse_image(it.collect());

        for round in 0..50 {
            let default_value = (round % 2) as usize;

            let mut next_image = Image::new();

            for position in image.positions() {
                let ch = enhancement_algorithm[image.read_value(position, default_value)];

                next_image.set_pixel(position, ch);
            }

            image = next_image;
        }

        image.print();

        println!("Pixels are LIT: {}", image.count_lit());
    }
}

mod day21 {
    use crate::shared::*;

    #[derive(Debug, Default, Clone)]
    struct Player {
        position: usize,
        score: usize
    }

    pub fn part1() {
        let mut player: Vec<Player> = vec![Default::default(); 2];

        player[0].position = 10;
        player[1].position = 6;

        let mut dice = 0;

        let mut rolls = 0;

        loop {
            for &i in &[0, 1] {
                let roll1 = { dice += 1; dice};
                rolls += 1;
                let roll2 = { dice += 1; dice};
                rolls += 1;
                let roll3 = { dice += 1; dice};
                rolls += 1;

                player[i].position += (roll1 + roll2 + roll3);

                if player[i].position > 10 {
                    player[i].position = ((player[i].position - 1) % 10) + 1;
                }

                player[i].score += player[i].position;

                if player[i].score >= 1000 {
                    println!("Player {} wins after {} rolls", i + 1, rolls);
                    println!("Result: {}", player[(i + 1) % 2].score * rolls);
                    return;
                }
            }
        }
    }

    #[derive(Debug, Clone)]
    struct Universe {
        positions: [usize; 2],
        scores: [usize; 2],
        universes_created: usize,
        next_player: usize,
    }

    pub fn part2() {
        let target_score = 21;

        let mut p1_win_count: usize = 0;
        let mut p2_win_count: usize = 0;

        let roll_sum_frequencies = {
            let mut map = HashMap::new();

            for roll1 in (1..=3) {
                for roll2 in (1..=3) {
                    for roll3 in (1..=3) {
                        *map.entry(roll1 + roll2 + roll3).or_insert(0) += 1
                    }
                }
            }

            map
        };

        let mut active_universes = vec!(Universe {
            positions: [10, 6],
            scores: [0, 0],
            universes_created: 1,
            next_player: 0,
        });

        while !active_universes.is_empty() {
            let mut next_universes = Vec::new();

            for universe in active_universes {
                let player = universe.next_player;

                for roll_total in (3..=9) {
                    let mut next_universe = universe.clone();
                    next_universe.positions[player] += roll_total;

                    if next_universe.positions[player] > 10 {
                        next_universe.positions[player] -= 10;
                    }

                    next_universe.scores[player] += next_universe.positions[player];

                    next_universe.universes_created *= *roll_sum_frequencies.get(&roll_total).unwrap();

                    next_universe.next_player = (player + 1) % 2;

                    if next_universe.scores[player] >= target_score {
                        if player == 0 {
                            p1_win_count += next_universe.universes_created;
                        } else {
                            p2_win_count += next_universe.universes_created;
                        }
                    } else {
                        next_universes.push(next_universe);
                    }
                }
            }

            active_universes = next_universes;
        }

        println!("Player 1: {}; Player 2: {}", p1_win_count, p2_win_count);
    }
}


mod day22 {
    use crate::shared::*;

    fn parse_range(start: &str, end: &str) -> std::ops::RangeInclusive<i64> {
        start.parse().unwrap()..=end.parse().unwrap()
    }

    pub fn part1() {
        let mut world: HashSet<(i64, i64, i64)> = HashSet::new();

        let rule_pattern = Regex::new(r"^x=(.+?)\.\.(.+?),y=(.+?)\.\.(.+?),z=(.+?)\.\.(.+?)$").unwrap();

        for line in input_lines("input_files/day22.txt") {
            let mut it = line.split(' ');

            let cmd = it.next();

            let mut it = rule_pattern.captures_iter(it.next().unwrap());
            let caps = it.next().unwrap();

            let xrange = parse_range(&caps[1], &caps[2]);
            let yrange = parse_range(&caps[3], &caps[4]);
            let zrange = parse_range(&caps[5], &caps[6]);

            if cmd == Some("on") {
                for x in xrange.clone() {
                    if x >= -50 && x <= 50 {
                        for y in yrange.clone() {
                            if y >= -50 && y <= 50 {
                                for z in zrange.clone() {
                                    if z >= -50 && z <= 50 {
                                        world.insert((x, y, z));
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                for x in xrange.clone() {
                    if x >= -50 && x <= 50 {
                        for y in yrange.clone() {
                            if y >= -50 && y <= 50 {
                                for z in zrange.clone() {
                                    if z >= -50 && z <= 50 {
                                        world.remove(&(x, y, z));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("Active cubes: {}", world.len());
    }

    #[derive(Debug, Clone)]
    struct Cube {
        top_x: i64,
        top_y: i64,
        top_z: i64,
        bot_x: i64,
        bot_y: i64,
        bot_z: i64,
        split_type: SplitType,
    }

    #[derive(Debug, Clone)]
    enum SplitType {
        None,
        Left,
        Right,
        Front,
        Back,
        Top,
        Bottom,
    }

    impl Cube {

        // Coordinate system I'm using here
        //
        //        z+
        //      /
        //    /
        //  /
        // ----------x+
        // |
        // |
        // |
        // |
        // y+

        fn clip_to(&self, other: &Cube) -> Cube {
            let mut result = self.clone();

            if result.top_x < other.top_x { result.top_x = other.top_x; }
            if result.bot_x > other.bot_x { result.bot_x = other.bot_x; }

            if result.top_y < other.top_y { result.top_y = other.top_y; }
            if result.bot_y > other.bot_y { result.bot_y = other.bot_y; }

            if result.top_z < other.top_z { result.top_z = other.top_z; }
            if result.bot_z > other.bot_z { result.bot_z = other.bot_z; }

            result
        }

        fn split(&self, other: &Cube) -> Vec<Cube> {
            let other = other.clip_to(self);

            let result: Vec<Cube> = vec!(
                // Left of other
                Cube {
                    top_x: self.top_x,
                    top_y: self.top_y,
                    top_z: self.top_z,
                    bot_x: other.top_x - 1,
                    bot_y: self.bot_y,
                    bot_z: self.bot_z,
                    split_type: SplitType::Left,
                },

                // Right of other
                Cube {
                    top_x: other.bot_x + 1,
                    top_y: self.top_y,
                    top_z: self.top_z,
                    bot_x: self.bot_x,
                    bot_y: self.bot_y,
                    bot_z: self.bot_z,
                    split_type: SplitType::Right,
                },

                // Front of other
                Cube {
                    top_x: other.top_x,
                    top_y: self.top_y,
                    top_z: self.top_z,
                    bot_x: other.bot_x,
                    bot_y: self.bot_y,
                    bot_z: other.top_z - 1,
                    split_type: SplitType::Front,
                },

                // Back of other
                Cube {
                    top_x: other.top_x,
                    top_y: self.top_y,
                    top_z: other.bot_z + 1,
                    bot_x: other.bot_x,
                    bot_y: self.bot_y,
                    bot_z: self.bot_z,
                    split_type: SplitType::Back,
                },

                // Top of other
                Cube {
                    top_x: other.top_x,
                    top_y: self.top_y,
                    top_z: other.top_z,
                    bot_x: other.bot_x,
                    bot_y: other.top_y - 1,
                    bot_z: other.bot_z,
                    split_type: SplitType::Top,
                },

                // Bottom of other
                Cube {
                    top_x: other.top_x,
                    top_y: other.bot_y + 1,
                    top_z: other.top_z,
                    bot_x: other.bot_x,
                    bot_y: self.bot_y,
                    bot_z: other.bot_z,
                    split_type: SplitType::Bottom,
                },
            ).into_iter().filter(|c| !c.is_empty()).collect();

            for i in 0..result.len() {
                for j in i+1..result.len() {
                    assert!(!result[i].overlaps(&result[j]));
                }
            }

            result
        }

        fn is_empty(&self) -> bool {
            self.top_x > self.bot_x || self.top_y > self.bot_y || self.top_z > self.bot_z
        }

        fn overlaps(&self, other: &Cube) -> bool {
            !(self.bot_x < other.top_x ||
              self.top_x > other.bot_x ||
              self.bot_y < other.top_y ||
              self.top_y > other.bot_y ||
              self.bot_z < other.top_z ||
              self.top_z > other.bot_z)
        }

        fn area(&self) -> i64 {
            (((self.bot_x + 1) - self.top_x) * ((self.bot_y + 1) - self.top_y) * ((self.bot_z + 1) - self.top_z))
        }
    }

    pub fn part2() {
        let rule_pattern = Regex::new(r"^x=(.+?)\.\.(.+?),y=(.+?)\.\.(.+?),z=(.+?)\.\.(.+?)$").unwrap();

        let mut world = Vec::new();

        for line in input_lines("input_files/day22.txt") {
            let mut it = line.split(' ');

            let cmd = it.next().unwrap();

            let mut it = rule_pattern.captures_iter(it.next().unwrap());
            let caps = it.next().unwrap();

            let mut new_world: Vec<Cube> = Vec::new();

            let rule_cube = Cube {
                top_x: caps[1].parse().unwrap(),
                top_y: caps[3].parse().unwrap(),
                top_z: caps[5].parse().unwrap(),
                bot_x: caps[2].parse().unwrap(),
                bot_y: caps[4].parse().unwrap(),
                bot_z: caps[6].parse().unwrap(),
                split_type: SplitType::None,
            };

            assert!(!rule_cube.is_empty());

            for cube in world.into_iter() {
                if rule_cube.overlaps(&cube) {
                    let splits = cube.split(&rule_cube);

                    if (cube.area() < splits.iter().map(|s| s.area()).sum()) {
                        dbg!(&cube);
                        dbg!(&rule_cube);

                        dbg!(cube.area(), splits.iter().map(|s| s.area()).sum::<i64>());

                        dbg!(&splits);
                    }


                    assert!(cube.area() > splits.iter().map(|s| s.area()).sum());

                    new_world.extend(cube.split(&rule_cube));
                } else {
                    new_world.push(cube);
                }
            }

            if cmd == "on" {
                new_world.push(rule_cube);
            }

            world = new_world;
        }

        println!("{} cubes are on", world.iter().map(|cube| cube.area()).sum::<i64>());
    }
}

mod day23 {
    use crate::shared::*;
    use Amphipod::*;
    use Tile::*;


    #[derive(Clone, Eq, PartialEq, Hash, Debug)]
    enum Tile {
        Hallway {
            idx: usize,
            occupant: Option<Amphipod>,
        },
        Room {
            idx: usize,
            upper_occupant: Option<Amphipod>,
            lower_occupant: Option<Amphipod>,
        }
    }

    #[derive(Eq, PartialEq, Debug, Clone, Hash, Copy)]
    enum Amphipod {
        A,
        B,
        C,
        D
    }

    impl Amphipod {
        fn energy(&self) -> usize {
            match *self {
                A => 1,
                B => 10,
                C => 100,
                D => 1000,
            }
        }

        fn target_room(&self) -> usize {
            match *self {
                A => 0,
                B => 1,
                C => 2,
                D => 3,
            }
        }
    }

    type World = Vec<Tile>;

    fn is_complete(world: &World) -> bool {
        world.iter().all(|t| {
            match t {
                Hallway { .. } => true,
                Room { upper_occupant, lower_occupant, .. } => {
                    upper_occupant == lower_occupant && upper_occupant.is_some()
                }
            }
        })
    }

    fn possible_moves(world: &World) -> Vec<(World, usize)> {
        if is_complete(&world) {
            return vec!();
        }

        let mut result = Vec::new();

        // If any amphipod is on a transitory square, it must move.  This constrains the
        // possible moves.
        let in_transitory = world.iter().any(|t| {
            match t {
                Hallway { idx, occupant: Some(_) } => (*idx == 2 || *idx == 4 || *idx == 6 || *idx == 8),
                _ => false,
            }
        });

        for world_idx in 0..world.len() {
            let tile = &world[world_idx];

            match tile {
                Hallway { occupant: None, .. } => {}
                Hallway { occupant: Some(amphipod), idx } => {
                    // FIXME: missed a rule here.  An amphipod that stops moving in a hallway is
                    // frozen until it can move into its room.

                    if idx > &0 && (!in_transitory || (*idx == 2 || *idx == 4 || *idx == 6 || *idx == 8)) {
                        if let Some(target_idx) = world.iter().position(|t| {
                            match t {
                                Hallway { occupant: None, idx: tile_idx } => *tile_idx == idx - 1,
                                _ => false,
                            }
                        }) {
                            // move it left
                            let mut new_world = world.clone();
                            new_world[world_idx] = Hallway { occupant: None, idx: *idx };
                            new_world[target_idx] = Hallway { occupant: Some(*amphipod), idx: idx - 1 };
                            result.push((new_world, amphipod.energy()));
                        }
                    }

                    if idx < &10 && (!in_transitory || (*idx == 2 || *idx == 4 || *idx == 6 || *idx == 8)) {
                        if let Some(target_idx) = world.iter().position(|t| {
                            match t {
                                Hallway { occupant: None, idx: tile_idx } => *tile_idx == idx + 1,
                                _ => false,
                            }
                        }) {
                            // move it right
                            let mut new_world = world.clone();
                            new_world[world_idx] = Hallway { occupant: None, idx: *idx };
                            new_world[target_idx] = Hallway { occupant: Some(*amphipod), idx: idx + 1 };
                            result.push((new_world, amphipod.energy()));
                        }
                    }

                    if *idx == 2 || *idx == 4 || *idx == 6 || *idx == 8 {
                        // Outside of a room.  Can enter if we're brave.
                        let room_idx = (idx / 2) - 1;

                        if amphipod.target_room() == room_idx {
                            if let Some(target_idx) = world.iter().position(|t| {
                                match t {
                                    Room { upper_occupant: None, idx: tile_idx, .. } => *tile_idx == room_idx,
                                    _ => false,
                                }
                            }) {
                                if let Room { upper_occupant: None, lower_occupant, .. } = &world[target_idx] {
                                    if lower_occupant.is_none() || lower_occupant == &Some(*amphipod) {
                                        let mut new_world = world.clone();
                                        new_world[world_idx] = Hallway { occupant: None, idx: *idx };
                                        new_world[target_idx] = Room { upper_occupant: Some(*amphipod), lower_occupant: lower_occupant.clone(), idx: room_idx };
                                        result.push((new_world, amphipod.energy()));
                                    }
                                }
                            }
                        }
                    }
                }
                Room { .. } => {
                    if !in_transitory {
                        if let Room { idx, lower_occupant: Some(lower_occupant), upper_occupant: None } = tile {
                            // Move lower to upper
                            let mut new_world = world.clone();
                            new_world[world_idx] = Room { idx: *idx, lower_occupant: None, upper_occupant: Some(*lower_occupant) };
                            result.push((new_world, lower_occupant.energy()));
                        }

                        if let Room { idx, lower_occupant: None, upper_occupant: Some(upper_occupant) } = tile {
                            // Move upper to lower
                            let mut new_world = world.clone();
                            new_world[world_idx] = Room { idx: *idx, lower_occupant: Some(*upper_occupant), upper_occupant: None };
                            result.push((new_world, upper_occupant.energy()));
                        }

                        if let Room { idx, upper_occupant: Some(upper_occupant), lower_occupant } = tile {
                            let hallway_idx = (idx * 2) + 2;

                            if let Some(target_hallway_idx) =  world.iter().position(|t| {
                                match t {
                                    Hallway { occupant: None, idx: tile_idx } => *tile_idx == hallway_idx,
                                    _ => false,
                                }
                            }) {
                                // Move them up into the hallway
                                let mut new_world = world.clone();
                                new_world[target_hallway_idx] = Hallway { occupant: Some(*upper_occupant), idx: hallway_idx };
                                new_world[world_idx] = Room { idx: *idx, upper_occupant: None, lower_occupant: lower_occupant.clone() };
                                result.push((new_world, upper_occupant.energy()));
                            }
                        }
                    }
                }
            }
        }

        result
    }

    #[derive(Clone, Eq, PartialEq)]
    struct Entry {
        world: World,
        cost: usize,
        position: usize,
    }

    impl Ord for Entry {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&(self.cost))
                .then_with(|| self.position.cmp(&other.position))
        }
    }

    impl PartialOrd for Entry {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    fn print(world: &World) {
        let hallways: Vec<String> = (0..=10).map(|idx| {
            world.iter().find(|t| match t {
                Hallway { idx: hallway_idx, .. } => *hallway_idx == idx,
                _ => false,
            }).unwrap()
        }).map(|hallway| {
            match hallway {
                Hallway { occupant: Some(a), .. } => format!("{:?}", a),
                _ => ".".to_string(),
            }
        }).collect();

        let upper_room: Vec<String> = (0..=3).map(|idx| {
            world.iter().find(|t| match t {
                Room { idx: room_idx, .. } => *room_idx == idx,
                _ => false,
            }).unwrap()
        }).map(|room| {
            match room {
                Room { upper_occupant: Some(a), .. } => format!("{:?}", a),
                _ => ".".to_string(),
            }
        }).collect();

        let lower_room: Vec<String> = (0..=3).map(|idx| {
            world.iter().find(|t| match t {
                Room { idx: room_idx, .. } => *room_idx == idx,
                _ => false,
            }).unwrap()
        }).map(|room| {
            match room {
                Room { lower_occupant: Some(a), .. } => format!("{:?}", a),
                _ => ".".to_string(),
            }
        }).collect();


        println!("
#############
#{}{}{}{}{}{}{}{}{}{}{}#
###{}#{}#{}#{}###
  #{}#{}#{}#{}#
  #########
",
                 hallways[0],
                 hallways[1],
                 hallways[2],
                 hallways[3],
                 hallways[4],
                 hallways[5],
                 hallways[6],
                 hallways[7],
                 hallways[8],
                 hallways[9],
                 hallways[10],
                 upper_room[0],
                 upper_room[1],
                 upper_room[2],
                 upper_room[3],
                 lower_room[0],
                 lower_room[1],
                 lower_room[2],
                 lower_room[3],
                 );
    }

    // The horror!
    pub fn part1() {
        let world = vec!(
            Hallway { idx: 0, occupant: None },
            Hallway { idx: 1, occupant: None },
            Hallway { idx: 2, occupant: None },
            Hallway { idx: 3, occupant: None },
            Hallway { idx: 4, occupant: None },
            Hallway { idx: 5, occupant: None },
            Hallway { idx: 6, occupant: None },
            Hallway { idx: 7, occupant: None },
            Hallway { idx: 8, occupant: None },
            Hallway { idx: 9, occupant: None },
            Hallway { idx: 10, occupant: None },
            Room { idx: 0, upper_occupant: Some(C), lower_occupant: Some(B) },
            Room { idx: 1, upper_occupant: Some(D), lower_occupant: Some(A) },
            Room { idx: 2, upper_occupant: Some(A), lower_occupant: Some(D) },
            Room { idx: 3, upper_occupant: Some(B), lower_occupant: Some(C) },
        );

        let mut queue = BinaryHeap::new();

        queue.push(Entry {
            world: world.clone(),
            cost: 0,
            position: 0,
        });

        let mut seen_states: HashMap<World, usize> = HashMap::new();
        seen_states.insert(world, 0);

        let mut position = 1;
        let mut checked: usize = 0;

        while let Some(entry) = queue.pop() {
            checked += 1;
            if is_complete(&entry.world) {
                println!("Completed with cost: {}", seen_states.get(&entry.world).unwrap());
                break;
            }

            let next_moves = possible_moves(&entry.world);

            for (next_move, cost) in next_moves {
                if let Some(best_cost) = seen_states.get(&next_move) {
                    if *best_cost <= entry.cost + cost {
                        continue;
                    }
                }

                seen_states.insert(next_move.clone(), entry.cost + cost);

                queue.push(Entry {
                    world: next_move,
                    cost: entry.cost + cost,
                    position,
                });

                position += 1;
            }
        }

        println!("Checked {} combinations", checked);
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
    }

    day23::part1();
}

