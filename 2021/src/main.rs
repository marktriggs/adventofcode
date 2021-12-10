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
    pub use std::str::{self, FromStr};
    pub use std::sync::{Arc, Mutex};

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
                let mut score: u64 = 0;
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
    }

    // day10::part1();
    day10::part2();

}
