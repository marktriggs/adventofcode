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
    }

    day7::part1();
    day7::part2();

}
