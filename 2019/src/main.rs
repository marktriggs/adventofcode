// (cd ../; cargo run --release)

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(dead_code)]

extern crate lazy_static;
extern crate regex;

pub mod intcode {
    pub struct IntCode {
        pub pc: usize,
        pub memory: Vec<i64>,

        pub terminated: bool,
        pub waiting_for_input: bool,

        pub input: Vec<i64>,
        pub output: Vec<i64>,
    }

    pub fn new(code: Vec<i64>, input: Vec<i64>, output: Vec<i64>) -> IntCode {
        IntCode {
            memory: code,
            pc: 0,
            terminated: false,
            waiting_for_input: false,
            input,
            output,
        }
    }

    impl IntCode {
        pub fn set_memory(&mut self, addr: usize, value: i64) {
            self.memory[addr] = value;
        }

        // Read a value relative to the program counter
        pub fn read_relative(&self, offset: usize) -> i64 {
            self.read_absolute((self.pc + offset) as i64, ParameterMode::Position)
        }

        // Read a value from an address (Position mode) or just return addr
        // (Immediate mode)
        pub fn read_absolute(&self, addr: i64, mode: ParameterMode) -> i64 {
            match mode {
                ParameterMode::Position => {
                    assert!(addr >= 0);
                    self.memory[addr as usize]
                }
                ParameterMode::Immediate => addr,
            }
        }

        fn next_instruction(&self) -> Box<dyn Instruction> {
            let instruction = self.memory[self.pc];

            // Right two digits
            let opcode = instruction % 100;

            let parameter_modes = vec![
                (instruction / 100) % 10,
                (instruction / 1000) % 10,
                (instruction / 10000) % 10,
            ];

            let parameter_modes = parameter_modes
                .iter()
                .map(|&n| {
                    if n == 0 {
                        ParameterMode::Position
                    } else {
                        ParameterMode::Immediate
                    }
                })
                .collect();

            match opcode {
                1 => Box::new(Addition { parameter_modes }),
                2 => Box::new(Multiplication { parameter_modes }),
                3 => Box::new(Input {}),
                4 => Box::new(Output { parameter_modes }),
                5 => Box::new(JumpIfTrue { parameter_modes }),
                6 => Box::new(JumpIfFalse { parameter_modes }),
                7 => Box::new(LessThan { parameter_modes }),
                8 => Box::new(Equals { parameter_modes }),
                99 => Box::new(Terminate {}),
                _ => panic!(
                    "Invalid instruction at offset {}: {}",
                    &self.pc, &self.memory[self.pc]
                ),
            }
        }

        pub fn evaluate(&mut self) {
            if (!self.input.is_empty()) {
                self.waiting_for_input = false;
            }

            while !self.terminated && !self.waiting_for_input {
                let instruction = self.next_instruction();
                instruction.apply(self);
            }
        }
    }

    pub trait Instruction {
        fn apply(&self, intcode: &mut IntCode);
    }

    #[derive(Copy, Clone)]
    pub enum ParameterMode {
        Position,
        Immediate,
    }

    struct Addition {
        parameter_modes: Vec<ParameterMode>,
    }
    struct Multiplication {
        parameter_modes: Vec<ParameterMode>,
    }
    struct Terminate {}
    struct Input {}
    struct Output {
        parameter_modes: Vec<ParameterMode>,
    }

    // Part 2 extras
    struct JumpIfTrue {
        parameter_modes: Vec<ParameterMode>,
    }
    struct JumpIfFalse {
        parameter_modes: Vec<ParameterMode>,
    }
    struct LessThan {
        parameter_modes: Vec<ParameterMode>,
    }
    struct Equals {
        parameter_modes: Vec<ParameterMode>,
    }

    impl Instruction for Addition {
        fn apply(&self, intcode: &mut IntCode) {
            let op1_addr = intcode.read_relative(1);
            let op2_addr = intcode.read_relative(2);
            let target_addr = intcode.read_relative(3);

            intcode.set_memory(
                target_addr as usize,
                intcode.read_absolute(op1_addr, self.parameter_modes[0])
                    + intcode.read_absolute(op2_addr, self.parameter_modes[1]),
            );
            intcode.pc += 4;
        }
    }

    impl Instruction for Multiplication {
        fn apply(&self, intcode: &mut IntCode) {
            let op1_addr = intcode.read_relative(1);
            let op2_addr = intcode.read_relative(2);
            let target_addr = intcode.read_relative(3);

            intcode.set_memory(
                target_addr as usize,
                intcode.read_absolute(op1_addr, self.parameter_modes[0])
                    * intcode.read_absolute(op2_addr, self.parameter_modes[1]),
            );
            intcode.pc += 4;
        }
    }

    impl Instruction for Terminate {
        fn apply(&self, intcode: &mut IntCode) {
            intcode.terminated = true;
            intcode.pc += 1;
        }
    }

    impl Instruction for Input {
        fn apply(&self, intcode: &mut IntCode) {
            match intcode.input.pop() {
                Some(value) => {
                    intcode.waiting_for_input = false;

                    let target_addr = intcode.read_relative(1);
                    intcode.set_memory(target_addr as usize, value);
                    intcode.pc += 2;
                }
                None => {
                    intcode.waiting_for_input = true;
                }
            }
        }
    }

    impl Instruction for Output {
        fn apply(&self, intcode: &mut IntCode) {
            let source_addr = intcode.read_relative(1);
            intcode
                .output
                .push(intcode.read_absolute(source_addr, self.parameter_modes[0]));
            intcode.pc += 2;
        }
    }

    impl Instruction for JumpIfTrue {
        fn apply(&self, intcode: &mut IntCode) {
            let test = intcode.read_absolute(intcode.read_relative(1), self.parameter_modes[0]);
            let jump_target = intcode.read_relative(2);

            if test != 0 {
                intcode.pc = intcode.read_absolute(jump_target, self.parameter_modes[1]) as usize;
            } else {
                intcode.pc += 3;
            }
        }
    }

    impl Instruction for JumpIfFalse {
        fn apply(&self, intcode: &mut IntCode) {
            let test = intcode.read_absolute(intcode.read_relative(1), self.parameter_modes[0]);
            let jump_target = intcode.read_relative(2);

            if test == 0 {
                intcode.pc = intcode.read_absolute(jump_target, self.parameter_modes[1]) as usize;
            } else {
                intcode.pc += 3;
            }
        }
    }

    impl Instruction for LessThan {
        fn apply(&self, intcode: &mut IntCode) {
            let a = intcode.read_absolute(intcode.read_relative(1), self.parameter_modes[0]);
            let b = intcode.read_absolute(intcode.read_relative(2), self.parameter_modes[1]);

            if a < b {
                intcode.set_memory(intcode.read_relative(3) as usize, 1);
            } else {
                intcode.set_memory(intcode.read_relative(3) as usize, 0);
            }

            intcode.pc += 4;
        }
    }

    impl Instruction for Equals {
        fn apply(&self, intcode: &mut IntCode) {
            let a = intcode.read_absolute(intcode.read_relative(1), self.parameter_modes[0]);
            let b = intcode.read_absolute(intcode.read_relative(2), self.parameter_modes[1]);

            if a == b {
                intcode.set_memory(intcode.read_relative(3) as usize, 1);
            } else {
                intcode.set_memory(intcode.read_relative(3) as usize, 0);
            }

            intcode.pc += 4;
        }
    }
}

mod shared {
    pub use regex::Regex;

    pub use intcode::{self, IntCode};
    pub use std::cmp::{self, Ordering};
    pub use std::collections::BTreeMap;
    pub use std::collections::HashMap;
    pub use std::collections::HashSet;
    pub use std::collections::LinkedList;
    pub use std::fmt::{self, Display};
    pub use std::fs::{self, File};
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

        let replace_regex = Regex::new(r"h.llo").unwrap();
        println!(
            "{}",
            replace_regex.replace_all("hello hello hello", "goodbye")
        );
    }

    pub fn read_file(file: &str) -> String {
        fs::read_to_string(file).unwrap().trim().to_owned()
    }

    pub fn input_lines(file: &str) -> impl Iterator<Item = String> {
        let f = File::open(file).unwrap_or_else(|_|panic!("Failed to open input file: {}", &file));
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
            let elt = inputs.get(0).unwrap().clone();
            let subperms = permutations(inputs.iter().skip(1).cloned().collect());

            subperms
                .iter()
                .flat_map(|subperm: &Vec<T>| {
                    (0..=subperm.len())
                        .map(|idx| {
                            let mut r = subperm.clone();
                            r.insert(idx, elt.clone());
                            r
                        })
                        .collect::<Vec<Vec<T>>>()
                })
                .collect()
        }
    }
}

mod day1 {
    use crate::shared::*;

    pub fn part1() {
        let result: i64 = input_lines("input_files/day1.txt")
            .map(|line| line.parse::<i64>().unwrap())
            .map(|mass| (mass / 3) - 2)
            .sum();

        dbg!(result);
    }

    fn cumulative_mass(mass: i64) -> i64 {
        let mut total_fuel = 0;
        let mut remainder = mass;

        loop {
            let fuel = (remainder / 3) - 2;

            if fuel <= 0 {
                break;
            }

            total_fuel += fuel;
            remainder = fuel;
        }

        total_fuel
    }

    pub fn part2() {
        let result: i64 = input_lines("input_files/day1.txt")
            .map(|line| line.parse::<i64>().unwrap())
            .map(cumulative_mass)
            .sum();

        dbg!(result);
    }
}

mod day2 {
    use crate::shared::*;

    fn intcode_eval(code: &str, noun: i64, verb: i64) -> i64 {
        let mut state: Vec<i64> = code.split(',').map(|s| s.parse().unwrap()).collect();
        let mut pc = 0;

        state[1] = noun;
        state[2] = verb;

        while state[pc] != 99 {
            let op1_addr = state[pc + 1] as usize;
            let op2_addr = state[pc + 2] as usize;
            let target_addr = state[pc + 3] as usize;

            match state[pc] {
                1 => {
                    state[target_addr] = state[op1_addr] + state[op2_addr];
                }
                2 => {
                    state[target_addr] = state[op1_addr] * state[op2_addr];
                }
                _ => panic!("invalid input"),
            };

            pc += 4;
        }

        state[0]
    }

    pub fn part1() {
        println!(
            "{}",
            intcode_eval(
                "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,1,5,19,23,1,13,23,27,1,6,27,31,2,31,\
                 13,35,1,9,35,39,2,39,13,43,1,43,10,47,1,47,13,51,2,13,51,55,1,55,9,59,1,59,5,\
                 63,1,6,63,67,1,13,67,71,2,71,10,75,1,6,75,79,1,79,10,83,1,5,83,87,2,10,87,91,\
                 1,6,91,95,1,9,95,99,1,99,9,103,2,103,10,107,1,5,107,111,1,9,111,115,2,13,115,\
                 119,1,119,10,123,1,123,10,127,2,127,10,131,1,5,131,135,1,10,135,139,1,139,2,\
                 143,1,6,143,0,99,2,14,0,0",
                12,
                2
            )
        );
    }

    pub fn part2() {
        for noun in 0..100 {
            for verb in 0..100 {
                let result = intcode_eval(
                    "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,1,5,19,23,1,13,23,27,1,6,27,31,2,31,\
                     13,35,1,9,35,39,2,39,13,43,1,43,10,47,1,47,13,51,2,13,51,55,1,55,9,59,1,59,5,\
                     63,1,6,63,67,1,13,67,71,2,71,10,75,1,6,75,79,1,79,10,83,1,5,83,87,2,10,87,91,\
                     1,6,91,95,1,9,95,99,1,99,9,103,2,103,10,107,1,5,107,111,1,9,111,115,2,13,115,\
                     119,1,119,10,123,1,123,10,127,2,127,10,131,1,5,131,135,1,10,135,139,1,139,2,\
                     143,1,6,143,0,99,2,14,0,0",
                    noun,
                    verb,
                );

                if result == 19_690_720 {
                    dbg!(100 * noun + verb);
                    return;
                }
            }
        }
    }
}

mod day3 {
    use crate::shared::*;

    #[derive(Debug, PartialEq, Eq, Clone, Hash)]
    struct Point {
        x: i64,
        y: i64,
    }

    #[derive(Debug)]
    enum Move {
        X(i64),
        Y(i64),
    }

    impl Point {
        fn calculate_offsets(pos: i64, offset: i64) -> Vec<i64> {
            (0..=offset.abs())
                .map(|n| pos + (n * if offset < 0 { -1 } else { 1 }))
                .collect()
        }

        fn trace_path(&self, m: &Move) -> Vec<Point> {
            match m {
                Move::X(magnitude) => Point::calculate_offsets(self.x, *magnitude)
                    .iter()
                    .map(|o| Point { x: *o, y: self.y })
                    .collect(),
                Move::Y(magnitude) => Point::calculate_offsets(self.y, *magnitude)
                    .iter()
                    .map(|o| Point { x: self.x, y: *o })
                    .collect(),
            }
        }
    }

    fn parse_wire(input: &str) -> Vec<Move> {
        input
            .split(',')
            .map(|move_desc| {
                let magnitude: i64 = move_desc[1..move_desc.len()].parse().unwrap();
                match &move_desc[0..1] {
                    "U" => Move::Y(magnitude),
                    "D" => Move::Y(-magnitude),
                    "L" => Move::X(-magnitude),
                    "R" => Move::X(magnitude),
                    _ => panic!("Parse error: {}", move_desc),
                }
            })
            .collect()
    }

    fn position_costs(moves: &[Move]) -> HashMap<Point, usize> {
        let mut pos = Point { x: 0, y: 0 };
        let mut cost: usize = 0;
        let mut result = HashMap::new();

        for next_move in moves {
            let path = pos.trace_path(next_move);

            for p in &path {
                result.entry(p.clone()).or_insert(cost);
                cost += 1;
            }

            cost -= 1;
            pos = path.last().unwrap().clone();
        }

        result
    }

    pub fn part1() {
        let input: Vec<String> = input_lines("input_files/day3.txt").collect();

        let wire1 = parse_wire(&input[0]);
        let wire2 = parse_wire(&input[1]);

        let wire1_positions = position_costs(&wire1);
        let wire2_positions = position_costs(&wire2);

        let wire1_points = wire1_positions.keys().cloned().collect::<HashSet<Point>>();
        let wire2_points = wire2_positions.keys().cloned().collect::<HashSet<Point>>();

        let intersections = wire1_points.intersection(&wire2_points);

        dbg!(&intersections
            .filter(|p| p.x != 0 && p.y != 0)
            .map(|point| point.x.abs() + point.y.abs())
            .min()
            .unwrap());
    }

    pub fn part2() {
        let input: Vec<String> = input_lines("input_files/day3.txt").collect();

        let wire1 = parse_wire(&input[0]);
        let wire2 = parse_wire(&input[1]);

        let wire1_costs = position_costs(&wire1);
        let wire2_costs = position_costs(&wire2);

        let wire1_points = wire1_costs.keys().cloned().collect::<HashSet<Point>>();
        let wire2_points = wire2_costs.keys().cloned().collect::<HashSet<Point>>();

        let intersections = wire1_points
            .intersection(&wire2_points)
            .filter(|p| p.x != 0 && p.y != 0);

        dbg!(intersections
            .map(|point| wire1_costs.get(&point).unwrap() + wire2_costs.get(&point).unwrap())
            .min()
            .unwrap());
    }
}

mod day4 {
    use crate::shared::*;

    pub fn part1() {
        let range_start = 128_392;
        let range_end = 643_281;
        let mut count = 0;

        for candidate in range_start..=range_end {
            let chars: Vec<char> = candidate.to_string().chars().collect();

            let mut ok = false;

            for i in 0..chars.len() - 1 {
                if chars[i] > chars[i + 1] {
                    ok = false;
                    break;
                }

                if chars[i] == chars[i + 1] {
                    ok = true;
                }
            }

            if ok {
                count += 1;
            }
        }

        dbg!(count);
    }

    pub fn part2() {
        let range_start = 128_392;
        let range_end = 643_281;
        let mut count = 0;

        for candidate in range_start..=range_end {
            let mut chars: Vec<char> = candidate.to_string().chars().collect();

            // padding so we don't have to think about boundary cases
            chars.insert(0, ' ');
            chars.push(' ');

            let mut got_two = false;
            let mut order_ok = true;

            for i in 1..chars.len() - 2 {
                // instant disqualify
                if chars[i] > chars[i + 1] {
                    order_ok = false;
                    break;
                }

                // Run of exactly two
                if chars[i] == chars[i + 1]
                    && chars[i - 1] != chars[i]
                    && chars[i + 1] != chars[i + 2]
                {
                    got_two = true;
                }
            }

            if order_ok && got_two {
                count += 1;
            }
        }

        dbg!(count);
    }
}

mod day5 {
    use crate::shared::*;

    pub fn part1() {
        let code = read_file("input_files/day5.txt");

        let mut intcode = intcode::new(
            code.split(',').map(|s| s.parse().unwrap()).collect(),
            vec![1],
            Vec::new(),
        );

        intcode.evaluate();
        dbg!(intcode.output);
    }

    pub fn part2() {
        let code = read_file("input_files/day5.txt");

        let mut intcode = intcode::new(
            code.split(',').map(|s| s.parse().unwrap()).collect(),
            vec![5],
            Vec::new(),
        );

        intcode.evaluate();
        dbg!(intcode.output);
    }
}

mod day6 {
    use crate::shared::*;

    pub fn part1() {
        let mut orbiter_to_orbitee_map: HashMap<String, String> =
            input_lines("input_files/day6.txt")
                .map(|line| {
                    let bits: Vec<&str> = line.split(')').collect();
                    (bits[1].to_owned(), bits[0].to_owned())
                })
                .collect();

        orbiter_to_orbitee_map.insert("COM".to_owned(), "COM".to_owned());

        let mut orbit_counts: HashMap<String, usize> = HashMap::new();

        // COM orbits nothing
        orbit_counts.insert("COM".to_owned(), 0);

        while orbit_counts.len() < orbiter_to_orbitee_map.len() {
            // Find entries whose orbitee has been fully calculated
            for orbiter in orbiter_to_orbitee_map.keys() {
                if orbit_counts.contains_key(orbiter) {
                    continue;
                }

                let orbitee = orbiter_to_orbitee_map.get(orbiter).unwrap();
                if orbit_counts.contains_key(orbitee) {
                    orbit_counts
                        .insert(orbiter.to_string(), orbit_counts.get(orbitee).unwrap() + 1);
                }
            }
        }

        dbg!(orbit_counts.values().sum::<usize>());
    }

    // Return a path back to root with a number of steps required to get to each point
    pub fn path_to_com(
        who: &str,
        orbiter_to_orbitee_map: &HashMap<String, String>,
    ) -> Vec<(String, usize)> {
        let mut orbiting = orbiter_to_orbitee_map.get(who).unwrap();
        let mut cost = 0;
        let mut result = Vec::new();

        while orbiting != "COM" {
            orbiting = orbiter_to_orbitee_map.get(orbiting).unwrap();
            cost += 1;
            result.push((orbiting.clone(), cost));
        }

        result
    }

    pub fn part2() {
        let orbiter_to_orbitee_map: HashMap<String, String> = input_lines("input_files/day6.txt")
            .map(|line| {
                let bits: Vec<&str> = line.split(')').collect();
                (bits[1].to_owned(), bits[0].to_owned())
            })
            .collect();

        let me_path = path_to_com("YOU", &orbiter_to_orbitee_map);
        let santa_path = path_to_com("SAN", &orbiter_to_orbitee_map);

        let me_nodes: HashMap<String, usize> = me_path.into_iter().collect();

        // The first node that both our paths meet defines the shortest path
        // between us.
        for (node, cost) in santa_path {
            if me_nodes.contains_key(&node) {
                dbg!("Shortest path:", cost + me_nodes.get(&node).unwrap());
                break;
            }
        }
    }
}

mod day7 {
    use crate::shared::*;

    pub fn part1() {
        let permutations = permutations(vec![0, 1, 2, 3, 4]);

        let code = read_file("input_files/day7.txt");
        let mut best_output = 0;

        for phases in permutations {
            let mut input = 0;

            for p in phases {
                let mut intcode = intcode::new(
                    code.split(',').map(|s| s.parse().unwrap()).collect(),
                    vec![input, p],
                    Vec::new(),
                );

                intcode.evaluate();
                input = intcode.output[0];
            }

            if input > best_output {
                best_output = input;
            }
        }

        dbg!(best_output);
    }

    pub fn part2() {
        let permutations = permutations(vec![5, 6, 7, 8, 9]);
        let code = read_file("input_files/day7.txt");

        let mut best_output = 0;

        for phases in permutations {
            let mut amplifiers: Vec<IntCode> = phases
                .iter()
                .map(|&phase| {
                    intcode::new(
                        code.split(',').map(|s| s.parse().unwrap()).collect(),
                        vec![phase],
                        Vec::new(),
                    )
                })
                .collect();

            let mut round = 0;
            let mut output_from_last_round = vec![0];

            loop {
                let idx = round % amplifiers.len();
                let amplifier = &mut amplifiers[idx];

                while !output_from_last_round.is_empty() {
                    amplifier
                        .input
                        .insert(0, output_from_last_round.pop().unwrap());
                }

                amplifier.evaluate();

                if amplifier.terminated && idx == 4 {
                    // FIRE TORPEDOES
                    let output = amplifier.output.last().unwrap();

                    if *output > best_output {
                        best_output = *output;
                    }

                    break;
                }

                output_from_last_round = amplifier.output.clone();

                amplifier.output.clear();
                round += 1;
            }
        }

        dbg!(best_output);
    }
}

mod day_n {
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

    // day7::part1();
    day7::part2();
}
