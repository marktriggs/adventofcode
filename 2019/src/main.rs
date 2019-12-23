// (cd ../; cargo run --release)

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(dead_code)]

extern crate lazy_static;
extern crate regex;

pub mod intcode {
    use std::collections::HashMap;

    #[derive(Clone)]
    pub struct IntCode {
        pub pc: usize,
        pub memory: HashMap<usize, i64>,

        pub terminated: bool,
        pub waiting_for_input: bool,

        pub input: Vec<i64>,
        pub output: Vec<i64>,

        pub relative_base: i64,
    }

    pub fn new(code: Vec<i64>, input: Vec<i64>, output: Vec<i64>) -> IntCode {
        let mut memory = HashMap::new();

        for (i, &instruction) in code.iter().enumerate() {
            memory.insert(i, instruction);
        }

        IntCode {
            memory,
            pc: 0,
            terminated: false,
            waiting_for_input: false,
            input,
            output,
            relative_base: 0,
        }
    }

    impl IntCode {
        pub fn set_memory(&mut self, addr: i64, value: i64, mode: ParameterMode) {
            let addr = addr as usize;

            match mode {
                ParameterMode::Position => {
                    self.memory.insert(addr, value);
                }
                ParameterMode::Relative => {
                    self.memory
                        .insert((self.relative_base + addr as i64) as usize, value);
                }
                ParameterMode::Immediate => {
                    panic!("Can't set an immediate value");
                }
            };
        }

        // Read a value relative to the program counter
        pub fn read_relative(&self, offset: usize) -> i64 {
            self.read_absolute((self.pc + offset) as i64, ParameterMode::Position)
        }

        fn memory_fetch(&self, addr: usize) -> i64 {
            *(self.memory.get(&addr).unwrap_or(&0))
        }

        // Read a value.  Will either:
        //   * read from an address (Position mode)
        //   * treat addr as an immediate value & return it (Immediate mode)
        //   * read from address + relative_base (Relative mode)
        pub fn read_absolute(&self, addr: i64, mode: ParameterMode) -> i64 {
            match mode {
                ParameterMode::Position => {
                    assert!(addr >= 0);
                    self.memory_fetch(addr as usize)
                }
                ParameterMode::Relative => {
                    assert!(addr + self.relative_base >= 0);
                    self.memory_fetch((addr + self.relative_base) as usize)
                }
                ParameterMode::Immediate => addr,
            }
        }

        fn next_instruction(&self) -> Box<dyn Instruction> {
            let instruction = self.memory_fetch(self.pc);

            // Right two digits
            let opcode = instruction % 100;

            let parameter_modes = vec![
                (instruction / 100) % 10,
                (instruction / 1000) % 10,
                (instruction / 10000) % 10,
            ];

            let parameter_modes = parameter_modes
                .iter()
                .map(|&n| match n {
                    0 => ParameterMode::Position,
                    1 => ParameterMode::Immediate,
                    2 => ParameterMode::Relative,
                    _ => panic!("Unknown parameter mode code: {}", n),
                })
                .collect();

            match opcode {
                1 => Box::new(Addition { parameter_modes }),
                2 => Box::new(Multiplication { parameter_modes }),
                3 => Box::new(Input { parameter_modes }),
                4 => Box::new(Output { parameter_modes }),
                5 => Box::new(JumpIfTrue { parameter_modes }),
                6 => Box::new(JumpIfFalse { parameter_modes }),
                7 => Box::new(LessThan { parameter_modes }),
                8 => Box::new(Equals { parameter_modes }),
                9 => Box::new(AdjustRelativeBase { parameter_modes }),
                99 => Box::new(Terminate {}),
                _ => panic!(
                    "Invalid instruction at offset {}: {}",
                    &self.pc,
                    &self.memory_fetch(self.pc)
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
        Relative,
    }

    struct Addition {
        parameter_modes: Vec<ParameterMode>,
    }
    impl Instruction for Addition {
        fn apply(&self, intcode: &mut IntCode) {
            let op1_addr = intcode.read_relative(1);
            let op2_addr = intcode.read_relative(2);
            let target_addr = intcode.read_relative(3);

            intcode.set_memory(
                target_addr,
                intcode.read_absolute(op1_addr, self.parameter_modes[0])
                    + intcode.read_absolute(op2_addr, self.parameter_modes[1]),
                self.parameter_modes[2],
            );
            intcode.pc += 4;
        }
    }

    struct Multiplication {
        parameter_modes: Vec<ParameterMode>,
    }
    impl Instruction for Multiplication {
        fn apply(&self, intcode: &mut IntCode) {
            let op1_addr = intcode.read_relative(1);
            let op2_addr = intcode.read_relative(2);
            let target_addr = intcode.read_relative(3);

            intcode.set_memory(
                target_addr,
                intcode.read_absolute(op1_addr, self.parameter_modes[0])
                    * intcode.read_absolute(op2_addr, self.parameter_modes[1]),
                self.parameter_modes[2],
            );
            intcode.pc += 4;
        }
    }

    struct Terminate {}
    impl Instruction for Terminate {
        fn apply(&self, intcode: &mut IntCode) {
            intcode.terminated = true;
            intcode.pc += 1;
        }
    }

    struct Input {
        parameter_modes: Vec<ParameterMode>,
    }
    impl Instruction for Input {
        fn apply(&self, intcode: &mut IntCode) {
            match intcode.input.pop() {
                Some(value) => {
                    intcode.waiting_for_input = false;

                    let target_addr = intcode.read_relative(1);

                    intcode.set_memory(target_addr, value, self.parameter_modes[0]);
                    intcode.pc += 2;
                }
                None => {
                    intcode.waiting_for_input = true;
                }
            }
        }
    }

    struct Output {
        parameter_modes: Vec<ParameterMode>,
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

    struct JumpIfTrue {
        parameter_modes: Vec<ParameterMode>,
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

    struct JumpIfFalse {
        parameter_modes: Vec<ParameterMode>,
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

    struct LessThan {
        parameter_modes: Vec<ParameterMode>,
    }
    impl Instruction for LessThan {
        fn apply(&self, intcode: &mut IntCode) {
            let a = intcode.read_absolute(intcode.read_relative(1), self.parameter_modes[0]);
            let b = intcode.read_absolute(intcode.read_relative(2), self.parameter_modes[1]);

            if a < b {
                intcode.set_memory(intcode.read_relative(3), 1, self.parameter_modes[2]);
            } else {
                intcode.set_memory(intcode.read_relative(3), 0, self.parameter_modes[2]);
            }

            intcode.pc += 4;
        }
    }

    struct Equals {
        parameter_modes: Vec<ParameterMode>,
    }
    impl Instruction for Equals {
        fn apply(&self, intcode: &mut IntCode) {
            let a = intcode.read_absolute(intcode.read_relative(1), self.parameter_modes[0]);
            let b = intcode.read_absolute(intcode.read_relative(2), self.parameter_modes[1]);

            if a == b {
                intcode.set_memory(intcode.read_relative(3), 1, self.parameter_modes[2]);
            } else {
                intcode.set_memory(intcode.read_relative(3), 0, self.parameter_modes[2]);
            }

            intcode.pc += 4;
        }
    }

    struct AdjustRelativeBase {
        parameter_modes: Vec<ParameterMode>,
    }
    impl Instruction for AdjustRelativeBase {
        fn apply(&self, intcode: &mut IntCode) {
            let offset = intcode.read_absolute(intcode.read_relative(1), self.parameter_modes[0]);
            intcode.relative_base += offset;
            intcode.pc += 2;
        }
    }
}

mod shared {
    pub use regex::Regex;

    pub use intcode::{self, IntCode};
    pub use std::cell::RefCell;
    pub use std::cmp::{self, Ordering};
    pub use std::collections::BTreeMap;
    pub use std::collections::HashMap;
    pub use std::collections::HashSet;
    pub use std::collections::LinkedList;
    pub use std::collections::VecDeque;
    pub use std::fmt::{self, Display};
    pub use std::fs::{self, File};
    pub use std::io::{self, BufRead, BufReader, Write, Read};
    pub use std::iter::FromIterator;
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
        let f = File::open(file).unwrap_or_else(|_| panic!("Failed to open input file: {}", &file));
        BufReader::new(f).lines().map(Result::unwrap)
    }

    pub fn sample_input(input: &str) -> Vec<String> {
        input.trim().split('\n').map(str::to_owned).collect()
    }

    pub fn permutations<T>(inputs: Vec<T>) -> Vec<Vec<T>>
    where
        T: Clone + Copy + std::fmt::Debug,
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
                        r.insert(idx, *elt);
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

mod day8 {
    use crate::shared::*;

    pub fn part1() {
        let image: Vec<char> = read_file("input_files/day8.txt").chars().collect();
        let width = 25;
        let height = 6;

        let min_zeroes = image
            .chunks(width * height)
            .min_by_key(|layer| layer.iter().filter(|&&pixel| pixel == '0').count())
            .unwrap();

        dbg!(
            min_zeroes.iter().filter(|&&pixel| pixel == '1').count()
                * min_zeroes.iter().filter(|&&pixel| pixel == '2').count()
        );
    }

    pub fn part2() {
        let image: Vec<char> = read_file("input_files/day8.txt").chars().collect();
        let width = 25;
        let height = 6;

        let mut result: Vec<char> = vec!['T'; width * height];

        for layer in image.chunks(width * height).rev() {
            for y in 0..height {
                for x in 0..width {
                    let idx = (y * width) + x;
                    let source_pixel = layer[idx];

                    match source_pixel {
                        '0' => {
                            // black
                            result[idx] = ' ';
                        }
                        '1' => {
                            // white
                            result[idx] = '#';
                        }
                        '2' => {
                            // transparent
                        }
                        _ => panic!("bad pixels, man"),
                    }
                }
            }
        }

        for line in result.chunks(width) {
            println!("{}", line.iter().collect::<String>());
        }
    }
}

mod day9 {
    use crate::shared::*;

    pub fn part1() {
        let code = read_file("input_files/day9.txt");

        let mut intcode = intcode::new(
            code.split(',').map(|s| s.parse().unwrap()).collect(),
            vec![1],
            Vec::new(),
        );

        intcode.evaluate();

        dbg!(intcode.output);
    }

    pub fn part2() {
        let code = read_file("input_files/day9.txt");

        let mut intcode = intcode::new(
            code.split(',').map(|s| s.parse().unwrap()).collect(),
            vec![2],
            Vec::new(),
        );

        intcode.evaluate();

        dbg!(intcode.output);
    }
}

// Going full Rust data modelling madness on this one!  Wheeeee
mod day10 {
    use crate::shared::*;

    /// Grids of stuff
    struct Grid<T: Copy> {
        grid: Vec<Vec<T>>,
        width: usize,
        height: usize,
    }

    impl<T: Copy> Grid<T> {
        fn get(&self, x: usize, y: usize) -> T {
            self.grid[y][x]
        }

        fn set(&mut self, x: usize, y: usize, val: T) {
            self.grid[y][x] = val;
        }

        fn in_range(&self, x: i64, y: i64) -> bool {
            x < self.width as i64 && y < self.height as i64 && x >= 0 && y >= 0
        }

        fn new(width: usize, height: usize, init_value: T) -> Grid<T> {
            let grid: Vec<Vec<T>> = (0..height)
                .map(|_| (0..width).map(|_| init_value).collect())
                .collect();
            Grid {
                grid,
                width,
                height,
            }
        }
    }

    impl<T: std::fmt::Debug + Copy> std::fmt::Debug for Grid<T> {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            formatter.write_str("\n")?;
            for row in &self.grid {
                formatter
                    .write_str(
                        &row.iter()
                            .map(|entry| format!("{:?}", entry))
                            .collect::<String>(),
                    )
                    .unwrap();
                formatter.write_str("\n")?
            }

            Ok(())
        }
    }

    /// Stuff floating in space
    #[derive(Copy, Clone, Eq, PartialEq)]
    enum Space {
        Asteroid,
        Empty,
    }

    impl From<char> for Space {
        fn from(ch: char) -> Space {
            match ch {
                '#' => Space::Asteroid,
                _ => Space::Empty,
            }
        }
    }

    impl std::fmt::Debug for Space {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            formatter.write_str(match self {
                Space::Asteroid => "#",
                Space::Empty => ".",
            })
        }
    }

    /// Marking areas as visible or not
    #[derive(Copy, Clone, Eq, PartialEq)]
    enum Visibility {
        Visible,
        Obscured,
    }

    impl std::fmt::Debug for Visibility {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            formatter
                .write_str(match self {
                    Visibility::Visible => "O",
                    Visibility::Obscured => "?",
                })
                .unwrap();

            Ok(())
        }
    }

    fn parse_grid<T: Copy>(s: &str) -> Grid<T>
    where
        T: From<char>,
    {
        let grid = s
            .split('\n')
            .map(|row| row.chars().map(T::from).collect::<Vec<T>>())
            .collect::<Vec<Vec<T>>>();

        let height = grid.len();
        let width = grid[0].len();

        Grid {
            grid,
            height,
            width,
        }
    }

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    pub fn part1() {
        let mut best_score = 0;
        let mut best_coords = (999, 999);

        let map: Grid<Space> = parse_grid(&read_file("input_files/day10.txt"));

        for ast_y in 0..map.height {
            for ast_x in 0..map.width {
                if map.get(ast_x, ast_y) != Space::Asteroid {
                    continue;
                }

                // Calculate how many asteroids this one can see
                let mut vismap: Grid<Visibility> =
                    Grid::new(map.width, map.height, Visibility::Visible);
                for y in 0..map.height {
                    for x in 0..map.width {
                        if x == ast_x && y == ast_y {
                            // Itsa me!
                            continue;
                        }

                        if map.get(x, y) != Space::Asteroid {
                            // The cold darkness of space
                            continue;
                        }

                        // Three cases to consider:
                        //
                        //   * this asteroid is on the same X axis as our
                        //     source, so will obscure the remainder of that
                        //     axis; or
                        //
                        //   * this asteroid is on the same Y axis as our
                        //     source, so will obscure the remainder of that
                        //     axis; or
                        //
                        //   * this asteroid is diagonal relative to our source
                        //     and obscures the remainder of that line

                        let xdir = if x > ast_x { 1 } else { -1 };
                        let ydir = if y > ast_y { 1 } else { -1 };

                        if ast_x == x {
                            // Both on X axis.  Block out remaining Y coordinates
                            let mut y = y as i64 + ydir;
                            while vismap.in_range(x as i64, y as i64) {
                                vismap.set(x as usize, y as usize, Visibility::Obscured);
                                y += ydir;
                            }
                        } else if ast_y == y {
                            // Both on Y axis.  Block out remaining X coordinates
                            let mut x = x as i64 + xdir;
                            while vismap.in_range(x as i64, y as i64) {
                                vismap.set(x as usize, y as usize, Visibility::Obscured);
                                x += xdir;
                            }
                        } else {
                            // Diagonal.  Project the line formed between our
                            // two asteroids out to the edge of the map, marking
                            // off the squares it crosses.
                            //
                            // If our two asteroids form a square with width W
                            // and height H, we find subsequent points the line
                            // crosses by finding successive W & H values that
                            // have the same ratio.  To find what we should add
                            // to our initial W & H to preserve their ratio, we
                            // take their GCD and divide by that.  W/GCD and
                            // H/GCD can be repeatedly added to each value to
                            // get a new pair on the same line.

                            let width = (x as i64 - ast_x as i64).abs() as usize;
                            let height = (y as i64 - ast_y as i64).abs() as usize;

                            let gcd = gcd(width, height);

                            let unit_width_adjustment = (width / gcd) as i64 * xdir;
                            let unit_height_adjustment = (height / gcd) as i64 * ydir;

                            let mut x = x as i64 + unit_width_adjustment;
                            let mut y = y as i64 + unit_height_adjustment;

                            while vismap.in_range(x, y) {
                                vismap.set(x as usize, y as usize, Visibility::Obscured);
                                x += unit_width_adjustment;
                                y += unit_height_adjustment;
                            }
                        }
                    }
                }

                let mut score = 0;
                for y in 0..map.height {
                    for x in 0..map.width {
                        if x == ast_x && y == ast_y {
                            continue;
                        }

                        if map.get(x, y) == Space::Asteroid
                            && vismap.get(x, y) == Visibility::Visible
                        {
                            score += 1;
                        }
                    }
                }

                if score > best_score {
                    best_score = score;
                    best_coords = (ast_x, ast_y);
                }

                println!("{}, {}: {}", ast_x, ast_y, score);
            }
        }

        dbg!(best_score, best_coords);
    }

    #[derive(Debug)]
    struct Target {
        x: i64,
        y: i64,
        angle: f64,
        distance: f64,
        exploded: bool,
    }

    pub fn part2() {
        let origin_x = 26i64;
        let origin_y = 36i64;

        let map: Grid<Space> = parse_grid(&read_file("input_files/day10.txt"));
        let mut targets: Vec<Target> = Vec::new();

        for ast_y in 0..map.height {
            for ast_x in 0..map.width {
                if map.get(ast_x, ast_y) != Space::Asteroid {
                    continue;
                }

                let ast_x = ast_x as i64;
                let ast_y = ast_y as i64;

                if ast_x == origin_x && ast_y == origin_y {
                    continue;
                }

                let x_off = ast_x as i64 - origin_x;
                let y_off = ast_y as i64 - origin_y;

                let distance: f64 = ((x_off * x_off) as f64 + (y_off * y_off) as f64).sqrt();

                let angle = if x_off == 0 || y_off == 0 {
                    if x_off == 0 {
                        if y_off > 0 {
                            180f64
                        } else {
                            0f64
                        }
                    } else {
                        if x_off > 0 {
                            90f64
                        } else {
                            270f64
                        }
                    }
                } else {
                    let angle_degrees =
                        (x_off as f64 / y_off as f64).atan() * (180.0 / std::f64::consts::PI);

                    if x_off > 0 && y_off > 0 {
                        // Quadrant 1
                        180f64 - angle_degrees
                    } else if x_off > 0 && y_off < 0 {
                        // Quadrant 0
                        angle_degrees.abs()
                    } else if x_off < 0 && y_off > 0 {
                        // Quadrant 2
                        270f64 + (-90f64 - angle_degrees)
                    } else if x_off < 0 && y_off < 0 {
                        // Quadrant 3
                        360f64 - angle_degrees
                    } else {
                        // Lines up with us
                        angle_degrees
                    }
                };

                targets.push(Target {
                    x: ast_x,
                    y: ast_y,
                    angle,
                    distance,
                    exploded: false,
                });
            }
        }

        targets.sort_by(|t1, t2| {
            t1.angle
                .partial_cmp(&t2.angle)
                .unwrap()
                .then(t1.distance.partial_cmp(&t2.distance).unwrap())
        });

        let mut exploded_count = 0;
        let mut last_angle_hit: f64 = -1.0;

        for t in targets.iter_mut() {
            if t.exploded || t.angle == last_angle_hit {
                continue;
            }

            exploded_count += 1;

            t.exploded = true;
            last_angle_hit = t.angle;

            if exploded_count == 200 {
                dbg!("200th victim", t);
                break;
            }
        }
    }
}

mod day11 {
    use crate::shared::*;

    #[derive(Hash, Debug, Eq, PartialEq, Copy, Clone)]
    struct Point(i64, i64);

    impl Point {
        fn move_direction(&self, dir: &Direction) -> Point {
            Point(self.0 + dir.0, self.1 + dir.1)
        }
    }

    struct Direction(i64, i64);

    impl Direction {
        fn turn_left(&self) -> Direction {
            match self {
                Direction(0, 1) => Direction(-1, 0),
                Direction(-1, 0) => Direction(0, -1),
                Direction(0, -1) => Direction(1, 0),
                Direction(1, 0) => Direction(0, 1),
                _ => unreachable!(),
            }
        }

        fn turn_right(&self) -> Direction {
            match self {
                Direction(0, 1) => Direction(1, 0),
                Direction(-1, 0) => Direction(0, 1),
                Direction(0, -1) => Direction(-1, 0),
                Direction(1, 0) => Direction(0, -1),
                _ => unreachable!(),
            }
        }
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    enum Colour {
        Black,
        White,
    }

    struct PanelPainter {
        panel: HashMap<Point, Colour>,
        position: Point,
        direction: Direction,
    }

    impl PanelPainter {
        fn new() -> PanelPainter {
            PanelPainter {
                panel: HashMap::new(),
                position: Point(0, 0),
                direction: Direction(0, 1),
            }
        }

        fn current_colour(&self) -> Colour {
            *(self.panel.get(&self.position).unwrap_or(&Colour::Black))
        }

        fn paint(&mut self, colour: Colour) {
            self.panel.insert(self.position, colour);
        }

        fn step(&mut self) {
            self.position = self.position.move_direction(&self.direction);
        }
    }

    pub fn part1() {
        let mut painter = PanelPainter::new();
        let code = read_file("input_files/day11.txt");

        let mut intcode = intcode::new(
            code.split(',').map(|s| s.parse().unwrap()).collect(),
            vec![0],
            Vec::new(),
        );

        loop {
            intcode.evaluate();

            let turn_direction = intcode.output.pop().unwrap();
            let paint_colour = if intcode.output.pop().unwrap() == 0 {
                Colour::Black
            } else {
                Colour::White
            };

            painter.paint(paint_colour);

            if turn_direction == 0 {
                painter.direction = painter.direction.turn_left();
            } else {
                painter.direction = painter.direction.turn_right();
            }

            painter.step();

            if intcode.terminated {
                break;
            }

            intcode.input.push(painter.current_colour() as i64);
        }

        dbg!(painter.panel.len());
    }

    pub fn part2() {
        let mut painter = PanelPainter::new();
        let code = read_file("input_files/day11.txt");

        let mut intcode = intcode::new(
            code.split(',').map(|s| s.parse().unwrap()).collect(),
            vec![1],
            Vec::new(),
        );

        loop {
            intcode.evaluate();

            let turn_direction = intcode.output.pop().unwrap();
            let paint_colour = if intcode.output.pop().unwrap() == 0 {
                Colour::Black
            } else {
                Colour::White
            };

            painter.paint(paint_colour);

            if turn_direction == 0 {
                painter.direction = painter.direction.turn_left();
            } else {
                painter.direction = painter.direction.turn_right();
            }

            painter.step();

            if intcode.terminated {
                break;
            }

            intcode.input.push(painter.current_colour() as i64);
        }

        let min_x = painter.panel.keys().map(|point| point.0).min().unwrap();
        let max_x = painter.panel.keys().map(|point| point.0).max().unwrap();
        let min_y = painter.panel.keys().map(|point| point.1).min().unwrap();
        let max_y = painter.panel.keys().map(|point| point.1).max().unwrap();

        for y in (min_y..=max_y).rev() {
            for x in (min_x..=max_x) {
                let colour = painter.panel.get(&Point(x, y)).unwrap_or(&Colour::Black);
                let ch = match colour {
                    Colour::Black => ' ',
                    Colour::White => '#',
                };

                print!("{}", ch);
            }
            println!();
        }
    }
}

mod day12 {
    use crate::shared::*;

    #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
    struct Point3D {
        x: i64,
        y: i64,
        z: i64,
    }

    impl std::ops::Add for Point3D {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }

    #[derive(Clone, Hash, Eq, PartialEq)]
    struct Moon {
        position: Point3D,
        velocity: Point3D,
    }

    impl Moon {
        fn new(x: i64, y: i64, z: i64) -> Moon {
            Moon {
                position: Point3D { x, y, z },
                velocity: Point3D { x: 0, y: 0, z: 0 },
            }
        }

        fn step(&mut self) {
            self.position = self.position + self.velocity;
        }

        fn step_x(&mut self) {
            self.position.x += self.velocity.x;
        }
    }

    impl std::fmt::Debug for Moon {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            formatter.write_str(&format!(
                "pos=<x={}, y={}, z={}>, vel=<x={}, y={}, z={}>",
                self.position.x,
                self.position.y,
                self.position.z,
                self.velocity.x,
                self.velocity.y,
                self.velocity.z
            ))
        }
    }

    pub fn part1() {
        let moons = vec![
            RefCell::new(Moon::new(1, 2, -9)),
            RefCell::new(Moon::new(-1, -9, -4)),
            RefCell::new(Moon::new(17, 6, 8)),
            RefCell::new(Moon::new(12, 4, 2)),
        ];

        for _step in 0..1000 {
            let moons_len = moons.len();
            // Apply gravity
            for i in 0..moons_len - 1 {
                let mut m1 = moons[i].borrow_mut();

                for j in i + 1..moons_len {
                    let mut m2 = moons[j].borrow_mut();

                    if m1.position.x > m2.position.x {
                        m1.velocity.x -= 1;
                        m2.velocity.x += 1;
                    } else if m1.position.x < m2.position.x {
                        m1.velocity.x += 1;
                        m2.velocity.x -= 1;
                    }

                    if m1.position.y > m2.position.y {
                        m1.velocity.y -= 1;
                        m2.velocity.y += 1;
                    } else if m1.position.y < m2.position.y {
                        m1.velocity.y += 1;
                        m2.velocity.y -= 1;
                    }

                    if m1.position.z > m2.position.z {
                        m1.velocity.z -= 1;
                        m2.velocity.z += 1;
                    } else if m1.position.z < m2.position.z {
                        m1.velocity.z += 1;
                        m2.velocity.z -= 1;
                    }
                }
            }

            // Apply velocity
            for moon in &moons {
                moon.borrow_mut().step();
            }

            // println!("After step {}", step + 1);
            // for moon in &moons {
            //     dbg!(moon.borrow());
            // }
        }

        let mut total_energy = 0;

        for moon in &moons {
            let m = moon.borrow();

            let potential = m.position.x.abs() + m.position.y.abs() + m.position.z.abs();
            let kinetic = m.velocity.x.abs() + m.velocity.y.abs() + m.velocity.z.abs();

            total_energy += potential * kinetic;
        }

        println!("Total energy: {}", total_energy);
    }

    #[derive(Eq, PartialEq, Hash)]
    struct Key {
        positions: Vec<i64>,
        velocities: Vec<i64>,
    }

    // Returns (first_step, repeat_step)
    fn steps_to_cycle(positions: &mut Vec<i64>, velocities: &mut Vec<i64>) -> (usize, usize) {
        let mut seen: HashMap<Key, usize> = HashMap::new();
        assert!(positions.len() == velocities.len());

        for step in 0..1000000usize {
            // Apply gravity
            for i in 0..positions.len() {
                let this_x = &positions[i];

                let mut lt_x = 0;
                let mut gt_x = 0;

                for x in positions.iter() {
                    match (*x).cmp(this_x) {
                        std::cmp::Ordering::Less => lt_x += 1,
                        std::cmp::Ordering::Greater => gt_x += 1,
                        _ => {}
                    }
                }

                velocities[i] += (gt_x - lt_x);
            }

            // Apply velocity
            for i in 0..positions.len() {
                positions[i] += velocities[i];
            }

            let key = Key {
                positions: positions.clone(),
                velocities: velocities.clone(),
            };

            match seen.insert(key, step) {
                Some(first_step) => {
                    return (first_step, step);
                }
                None => {}
            }
        }

        panic!("Didn't find a repeat!");
    }

    pub fn part2() {
        let mut xs: Vec<i64> = vec![1, -1, 17, 12];
        let mut xvels: Vec<i64> = vec![0, 0, 0, 0];

        let mut ys: Vec<i64> = vec![2, -9, 6, 4];
        let mut yvels: Vec<i64> = vec![0, 0, 0, 0];

        let mut zs: Vec<i64> = vec![-9, -4, 8, 2];
        let mut zvels: Vec<i64> = vec![0, 0, 0, 0];

        // Turns out the cycle begins immediately, so I didn't end up needing the first value here
        let (_, x_step) = steps_to_cycle(&mut xs, &mut xvels);
        let (_, y_step) = steps_to_cycle(&mut ys, &mut yvels);
        let (_, z_step) = steps_to_cycle(&mut zs, &mut zvels);

        // Sayz the internet
        // lcm(a, b, c) = lcm(a, lcm(b, c))
        dbg!(lcm(lcm(x_step, y_step), z_step));
    }

    fn lcm(a: usize, b: usize) -> usize {
        (a * b) / gcd(a, b)
    }

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }
}

mod day13 {
    use crate::shared::*;

    pub fn part1() {
        let code = read_file("input_files/day13.txt");

        let mut intcode = intcode::new(
            code.split(',').map(|s| s.parse().unwrap()).collect(),
            Vec::new(),
            Vec::new(),
        );

        intcode.evaluate();

        let mut coords: Vec<(i64, i64)> = intcode
            .output
            .chunks(3)
            .filter(|chunk| chunk[2] == 2)
            .map(|chunk| (chunk[0], chunk[1]))
            .collect();
        coords.dedup();

        dbg!(coords.len());
    }

    #[derive(Clone, Copy, Eq, PartialEq)]
    enum Tile {
        Empty,
        Wall,
        Block,
        Paddle,
        Ball,
    }

    impl From<usize> for Tile {
        fn from(tile_type: usize) -> Tile {
            match tile_type {
                0 => Tile::Empty,
                1 => Tile::Wall,
                2 => Tile::Block,
                3 => Tile::Paddle,
                4 => Tile::Ball,
                _ => panic!("Not a valid tile number: {}", tile_type),
            }
        }
    }

    pub fn part2() {
        let mut code: Vec<i64> = read_file("input_files/day13.txt").split(',').map(|s| s.parse().unwrap()).collect();
        // FREEEEE
        code[0] = 2;

        let mut intcode = intcode::new(
            code,
            Vec::new(),
            Vec::new(),
        );

        let width = 42;
        let height = 26;
        let mut framebuffer: Vec<Vec<Tile>> = (0..height).map(|_| vec![Tile::Empty; width]).collect();
        let mut current_score = 0;

        let mut last_paddle_x = 0;
        let mut last_ball_x = 0;

        for _ in (0..500) {
            println!();
        }

        loop {
            intcode.evaluate();

            for directive in intcode.output.chunks(3) {
                if directive[0] == -1 && directive[1] == 0 {
                    // Score update
                    current_score = directive[2];
                } else {
                    let x = directive[0];
                    let y = directive[1];
                    let tile = Tile::from(directive[2] as usize);

                    if tile == Tile::Ball {
                        last_ball_x = x;
                    }

                    if tile == Tile::Paddle {
                        last_paddle_x = x;
                    }

                    framebuffer[y as usize][x as usize] = tile;
                }
            }

            intcode.output.clear();

            print!("{}[2J", 27 as char);
            print!("{}[3J", 27 as char);
            println!("{:>42}", format!("Score: {}", current_score));
            for row in &framebuffer {
                for col in row {
                    let ch = match col {
                        Tile::Empty => ' ',
                        Tile::Wall => '#',
                        Tile::Block => 'X',
                        Tile::Paddle => '=',
                        Tile::Ball => 'o',
                    };

                    print!("{}", ch);
                }

                println!();
            }


            if intcode.terminated {
                break;
            }

            assert!(intcode.waiting_for_input);

            if last_paddle_x < last_ball_x {
                intcode.input.push(1);
            } else if last_paddle_x > last_ball_x {
                intcode.input.push(-1);
            } else {
                intcode.input.push(0);
            }

            std::thread::sleep(std::time::Duration::from_millis(5));
        }

        println!("FINAL SCORE: {}", current_score);
    }

    fn write_frame(grid: &Vec<Vec<Tile>>, out: &mut impl Write) {
        const PIXEL_SIZE: usize = 25;

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
                let val = match cell {
                    Tile::Empty => (255, 255, 255),
                    Tile::Wall => (188, 143, 143),
                    Tile::Block => (32, 32, 32),
                    Tile::Paddle => (178, 34, 34),
                    Tile::Ball => (255, 0, 0),
                };

                for _ in 0..PIXEL_SIZE {
                    // RGB
                    output_row.push(val.0);
                    output_row.push(val.1);
                    output_row.push(val.2);
                }
            }

            // Repeat the row PIXEL_SIZE to make square pixels.
            for _ in 0..PIXEL_SIZE {
                out.write_all(&output_row).unwrap();
            }
        }
    }

    pub fn part2_deluxe_mode() {
        let mut code: Vec<i64> = read_file("input_files/day13.txt").split(',').map(|s| s.parse().unwrap()).collect();
        // FREEEEE
        code[0] = 2;

        let mut intcode = intcode::new(
            code,
            Vec::new(),
            Vec::new(),
        );

        let width = 42;
        let height = 26;
        let mut framebuffer: Vec<Vec<Tile>> = (0..height).map(|_| vec![Tile::Empty; width]).collect();
        let mut current_score = 0;

        let mut last_paddle_x = 0;
        let mut last_ball_x = 0;

        let mut ffmpeg = std::process::Command::new("ffmpeg")
            .args(&["-vcodec", "ppm", "-y", "-f", "image2pipe", "-framerate", "30", "-i", "-", "day13.mp4"])
            .stdin(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to start ffmpeg");

        let to_ffmpeg = ffmpeg.stdin.as_mut().expect("Failed to open ffmpeg stdin");

        loop {
            intcode.evaluate();

            for directive in intcode.output.chunks(3) {
                if directive[0] == -1 && directive[1] == 0 {
                    // Score update
                    current_score = directive[2];
                } else {
                    let x = directive[0];
                    let y = directive[1];
                    let tile = Tile::from(directive[2] as usize);

                    if tile == Tile::Ball {
                        last_ball_x = x;
                    }

                    if tile == Tile::Paddle {
                        last_paddle_x = x;
                    }

                    framebuffer[y as usize][x as usize] = tile;
                }
            }

            intcode.output.clear();


            // PPM image away!
            write_frame(&framebuffer, to_ffmpeg);

            if intcode.terminated {
                break;
            }

            assert!(intcode.waiting_for_input);

            if last_paddle_x < last_ball_x {
                intcode.input.push(1);
            } else if last_paddle_x > last_ball_x {
                intcode.input.push(-1);
            } else {
                intcode.input.push(0);
            }
        }

        ffmpeg.wait().unwrap();

        println!("FINAL SCORE: {}", current_score);
    }
}

mod day14 {
    use crate::shared::*;

    #[derive(Debug)]
    struct ChemQuant {
        qty: i64,
        chem: String,
    }

    impl From<&str> for ChemQuant {
        fn from(s: &str) -> ChemQuant {
            let bits: Vec<&str> = s.split(" ").collect();

            ChemQuant {
                qty: bits[0].parse().unwrap(),
                chem: bits[1].to_owned(),
            }
        }
    }

    #[derive(Debug)]
    struct Reaction {
        inputs: Vec<ChemQuant>,
        output: ChemQuant,
    }

    pub fn part1() {
        let reactions: Vec<Reaction> = input_lines("input_files/day14.txt").map(|line| {
            let bits: Vec<String> = line.split(" => ").map(str::to_owned).collect();

            let inputs: Vec<ChemQuant> = bits[0].split(", ").map(ChemQuant::from).collect();
            let output: ChemQuant = ChemQuant::from(&bits[1] as &str);

            Reaction { inputs, output }
        }).collect();

        let mut goal_amounts: HashMap<String, i64> = HashMap::new();

        goal_amounts.insert("FUEL".to_owned(), 1);

        loop {
            let mut done = true;

            for (chem, &amount_needed) in &goal_amounts.clone() {
                if chem == "ORE" {
                    // Nothing more to be done
                    continue;
                }

                if amount_needed <= 0 {
                    continue;
                }

                done = false;
                let reaction = reactions.iter().find(|r| &(r.output.chem) == chem).unwrap();
                let reactions_needed = (amount_needed as f64 / reaction.output.qty as f64).ceil() as i64;

                for input in &reaction.inputs {
                    let e = goal_amounts.entry(input.chem.clone()).or_insert(0);
                    *e += (reactions_needed * input.qty);
                }

                goal_amounts.insert(chem.clone(), amount_needed - (reaction.output.qty * reactions_needed));
                break;
            }

            if done {
                break;
            }
        }

        dbg!(goal_amounts);
    }

    pub fn part2() {
        let reactions: Vec<Reaction> = input_lines("input_files/day14.txt").map(|line| {
            let bits: Vec<String> = line.split(" => ").map(str::to_owned).collect();

            let inputs: Vec<ChemQuant> = bits[0].split(", ").map(ChemQuant::from).collect();
            let output: ChemQuant = ChemQuant::from(&bits[1] as &str);

            Reaction { inputs, output }
        }).collect();

        let mut goal_amounts: HashMap<String, i64> = HashMap::new();

        // 1_000_000_000_000

        // Trial and error & binary searched to find this value!
        goal_amounts.insert("FUEL".to_owned(), 2269325);

        loop {
            let mut done = true;

            for (chem, &amount_needed) in &goal_amounts.clone() {
                if chem == "ORE" {
                    // Nothing more to be done
                    continue;
                }

                if amount_needed <= 0 {
                    continue;
                }

                done = false;
                let reaction = reactions.iter().find(|r| &(r.output.chem) == chem).unwrap();
                let reactions_needed = (amount_needed as f64 / reaction.output.qty as f64).ceil() as i64;

                for input in &reaction.inputs {
                    let e = goal_amounts.entry(input.chem.clone()).or_insert(0);
                    *e += (reactions_needed * input.qty);
                }

                goal_amounts.insert(chem.clone(), amount_needed - (reaction.output.qty * reactions_needed));
                break;
            }

            if done {
                break;
            }
        }

        dbg!(goal_amounts);
    }


}

mod day15 {
    use crate::shared::*;

    struct Exploration {
        state: intcode::IntCode,
        position: (i64, i64),
        move_count: usize,
        try_direction: usize,
    }

    #[derive(Eq, PartialEq, Copy, Clone, Debug)]
    enum Tile {
        Open,
        Wall,
        OxygenSystem,
    }

    pub fn move_position(pos: (i64, i64), direction: usize) -> (i64, i64) {
        match direction {
            1 => (pos.0, pos.1 + 1),
            2 => (pos.0, pos.1 - 1),
            3 => (pos.0 - 1, pos.1),
            4 => (pos.0 + 1, pos.1),
            _ => panic!("Bad direction: {}", direction),
        }
    }

    fn map_world() -> HashMap<(i64, i64), Tile> {
        let code: Vec<i64> = read_file("input_files/day15.txt").split(',').map(|s| s.parse().unwrap()).collect();

        let intcode = intcode::new(
            code,
            Vec::new(),
            Vec::new(),
        );

        let mut world: HashMap<(i64, i64), Tile> = HashMap::new();

        let mut queue: VecDeque<Exploration> = VecDeque::from(
            vec!(
                Exploration { state: intcode.clone(), position: (0, 0), try_direction: 1, move_count: 1 },
                Exploration { state: intcode.clone(), position: (0, 0), try_direction: 2, move_count: 1 },
                Exploration { state: intcode.clone(), position: (0, 0), try_direction: 3, move_count: 1 },
                Exploration { state: intcode.clone(), position: (0, 0), try_direction: 4, move_count: 1 },
            ));

        while (!queue.is_empty()) {
            let mut exploration = queue.pop_front().unwrap();

            let tested_position = move_position(exploration.position, exploration.try_direction);

            if !world.contains_key(&tested_position) {
                if !exploration.state.input.is_empty() {
                    dbg!(&exploration.state.input);
                }

                exploration.state.input.push(exploration.try_direction as i64);
                exploration.state.evaluate();

                let output = exploration.state.output.pop().unwrap();

                let tile = match output {
                    0 => Tile::Wall,
                    1 => Tile::Open,
                    2 => Tile::OxygenSystem,
                    _ => panic!("Unexpected response: {}", output),
                };

                if tile == Tile::OxygenSystem {
                    dbg!(exploration.move_count);
                }

                world.insert(tested_position, tile);

                if tile != Tile::Wall {
                    queue.push_front(Exploration { state: exploration.state.clone(), position: tested_position, try_direction: 1, move_count: exploration.move_count + 1 });
                    queue.push_front(Exploration { state: exploration.state.clone(), position: tested_position, try_direction: 2, move_count: exploration.move_count + 1 });
                    queue.push_front(Exploration { state: exploration.state.clone(), position: tested_position, try_direction: 3, move_count: exploration.move_count + 1 });
                    queue.push_front(Exploration { state: exploration.state.clone(), position: tested_position, try_direction: 4, move_count: exploration.move_count + 1 });
                }

            }
        }

        world
    }

    fn dump_world(world: &HashMap<(i64, i64), Tile>) {
        let min_x = *(world.iter().map(|((x, _y), _v)| x).min().unwrap());
        let max_x = *(world.iter().map(|((x, _y), _v)| x).max().unwrap());
        let min_y = *(world.iter().map(|((_x, y), _v)| y).min().unwrap());
        let max_y = *(world.iter().map(|((_x, y), _v)| y).max().unwrap());

        for y in (min_y..=max_y) {
            for x in (min_x..=max_x) {
                print!("{}",
                       match world.get(&(x, y)).unwrap_or(&Tile::Open) {
                           Tile::Wall => '#',
                           Tile::Open => ' ',
                           Tile::OxygenSystem => 'O',
                       });
            }
            println!();
        }
    }

    pub fn part1() {
        let world = map_world();

        dump_world(&world);
    }

    pub fn part2() {
        let mut world = map_world();

        let oxygen_system = world.iter().find(|&(_, v)| v == &Tile::OxygenSystem).map(|(k, _)| k).unwrap();

        let mut queue: VecDeque<(i64, i64)> = VecDeque::from(vec!(*oxygen_system));

        let mut minutes = 0;
        loop {
            let mut next_queue: VecDeque<(i64, i64)> = VecDeque::new();

            while !queue.is_empty() {
                let pos = queue.pop_front().unwrap();

                // If there are adjacent squares to fill in, do it...
                for direction in 1..=4 {
                    let adjacent_pos = move_position(pos, direction);

                    if world.get(&adjacent_pos).unwrap_or(&Tile::Open) == &Tile::Open {
                        world.insert(adjacent_pos, Tile::OxygenSystem);
                        next_queue.push_front(adjacent_pos);
                    }
                }
            }

            if next_queue.is_empty() {
                break;
            }

            minutes += 1;
            queue = next_queue;
        }

        println!("Filled in {} minutes", minutes);
    }
}


mod day16 {
    use crate::shared::*;

    fn repeat_for_phase<T>(base_pattern: &Vec<T>, phase: usize) -> impl Iterator<Item=T> + '_
        where T: Clone
    {
        base_pattern.iter().map(move |elt| (0..phase).map(move |_| elt.clone())).flatten().cycle()
    }


    pub fn part1() {
        let pattern: Vec<i64> = vec!(0, 1, 0, -1);
        let input_s = read_file("input_files/day16.txt");
        let mut input: Vec<i64> = input_s.chars().map(|ch| ch.to_digit(10).unwrap() as i64).collect();

        for _phase in (0..100) {
            let mut result: Vec<i64> = vec![0; input.len()];

            for repeat in 0..input.len() {
                let output: i64 = input
                    .iter()
                    .zip(repeat_for_phase(&pattern, repeat + 1).skip(1))
                    .map(|(digit, pattern)| ((digit * pattern)))
                    .sum();

                result[repeat] = (output % 10).abs();
            }

            input = result;
        }

        dbg!(input.iter().take(8).cloned().collect::<Vec<i64>>());
    }

    pub fn part2() {
        // let pattern: Vec<i64> = vec!(0, 1, 0, -1);
        let input_s = read_file("input_files/day16.txt");

        let repeated_input: String = (0..10_000).map(|_| input_s.clone()).collect();

        // input len: 6,500,000 
        let offset = 5975589;

        // 0 - 5975588 (inclusive) all zero
        // -- then add to the end & mod 8 times

        let mut input: Vec<i64> = repeated_input.chars().map(|ch| ch.to_digit(10).unwrap() as i64).collect();

        for _phase in (0..100) {
            // dbg!(_phase);
            let mut result: Vec<i64> = vec![0; input.len()];

            let mut total: i64 = input.iter().skip(offset).sum();
            result[offset] = (total % 10).abs();

            for repeat in (offset + 1)..input.len() {
                let adjusted = total - input[repeat - 1];
                total = adjusted;
                result[repeat] = (total % 10).abs();
            }

            input = result;
        }

        dbg!(input.iter().skip(offset).take(8).cloned().map(|n| format!("{}", n)).collect::<String>());

    }
}


mod day17 {
    use crate::shared::*;
    extern crate termion;
    use self::termion::raw::IntoRawMode;


    pub fn part1() {
        let code: Vec<i64> = read_file("input_files/day17.txt").split(',').map(|s| s.parse().unwrap()).collect();

        let mut intcode = intcode::new(
            code,
            Vec::new(),
            Vec::new(),
        );

        intcode.evaluate();

        let mut world: HashMap<(usize, usize), char> = HashMap::new();

        let mut col = 0;
        let mut row = 0;

        let mut width = 0;
        let mut height = 0;

        for ch in intcode.output {
            if (ch as u8 as char) == '\n' {
                height = row;
                row += 1;
                col = 0;
            } else {
                world.insert((row, col), ch as u8 as char);
                col += 1;
                width = col;
            }
        }

        let mut total = 0;

        for row in 1..(height - 1) {
            for col in 1..(width - 1) {
                if world.get(&(row, col)).unwrap() == &'#' &&
                    world.get(&(row + 1, col)).unwrap() == &'#' &&
                    world.get(&(row - 1, col)).unwrap() == &'#' &&
                    world.get(&(row, col + 1)).unwrap() == &'#' &&
                    world.get(&(row, col - 1)).unwrap() == &'#' {
                        total += row * col;
                        world.insert((row, col), 'O');
                    }
            }
        }

        for row in 0..height {
            for col in 0..width {
                print!("{}", world.get(&(row, col)).unwrap_or(&'.'));
            }
            println!();
        }

        println!("Total was {}", total);
    }

    enum Facing {
        Up,
        Down,
        Left,
        Right,
    }

    fn apply_adjustment(position: (usize, usize), adjustment: (i64, i64)) -> (usize, usize) {
        ((position.0 as i64 + adjustment.0) as usize, (position.1 as i64 + adjustment.1) as usize)
    }

    pub fn part2_simulator() {
        let code: Vec<i64> = read_file("input_files/day17.txt").split(',').map(|s| s.parse().unwrap()).collect();

        let mut intcode = intcode::new(
            code,
            Vec::new(),
            Vec::new(),
        );

        intcode.evaluate();

        let mut world: HashMap<(usize, usize), char> = HashMap::new();

        let mut col = 0;
        let mut row = 0;

        let mut width = 0;
        let mut height = 0;

        for ch in intcode.output {
            if (ch as u8 as char) == '\n' {
                height = row;
                row += 1;
                col = 0;
            } else {
                world.insert((row, col), ch as u8 as char);
                col += 1;
                width = col;
            }
        }

        let mut vacuum_facing = Facing::Up;
        let mut vacuum_position = (0, 0);

        let mut movements: Vec<char> = Vec::new();

        loop {
            print!("{}[2J", 27 as char);
            print!("{}[3J", 27 as char);

            for row in 0..height {
                for col in 0..width {
                    if world.get(&(row, col)).unwrap_or(&'.') == &'^' {
                        // Our little guy
                        let ch = match vacuum_facing {
                            Facing::Up => '^',
                            Facing::Down => 'v',
                            Facing::Right => '>',
                            Facing::Left => '<',
                        };

                        vacuum_position = (row, col);
                        print!("{}", ch);
                    } else {
                        print!("{}",  world.get(&(row, col)).unwrap_or(&'.'));
                    }
                }
                println!();
            }

            let mut _stdout = std::io::stdout().into_raw_mode().unwrap();
            let stdin = std::io::stdin();
            let stdin = stdin.lock();
            let input = stdin.bytes().next().unwrap().unwrap();

            if input as char == 'q' {
                break;
            }

            match input {
                65 => {
                    // up - move forward
                    let adjustment = match vacuum_facing {
                        Facing::Up => (-1, 0),
                        Facing::Down => (1, 0),
                        Facing::Right => (0, 1),
                        Facing::Left => (0, -1),
                    };

                    let new_position = apply_adjustment(vacuum_position, adjustment);
                    let &target_tile_type = world.get(&new_position).unwrap_or(&'.');

                    if target_tile_type == '#' || target_tile_type == 'X' {
                        // OK!
                        // Old square is visited
                        world.insert(vacuum_position, 'X');

                        // New square is us
                        vacuum_position = new_position;
                        world.insert(vacuum_position, '^');

                        movements.push('1');
                    }
                },
                66 => {
                    // down
                },
                67 => {
                    // turn right
                    vacuum_facing = match vacuum_facing {
                        Facing::Up => Facing::Right,
                        Facing::Down => Facing::Left,
                        Facing::Right => Facing::Down,
                        Facing::Left => Facing::Up,
                    };

                    movements.push('R');
                },
                68 => {
                    // left
                    vacuum_facing = match vacuum_facing {
                        Facing::Up => Facing::Left,
                        Facing::Down => Facing::Right,
                        Facing::Right => Facing::Up,
                        Facing::Left => Facing::Down,
                    };
                    movements.push('L');
                },
                _ => {
                    // dbg!("\n\nNO IDEA", input);
                    // break;
                },
            }
        }

        println!("{}", movements.iter().collect::<String>());
    }

    fn collapse_movements(movements: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut chars = movements.chars();
        let mut run_length = 0;

        loop {
            match chars.next() {
                Some(ch) => {
                    if ch == 'L' || ch == 'R' {
                        if run_length > 0 {
                            result.push(format!("{}", run_length));
                        }

                        result.push(format!("{}", ch));
                        run_length = 0;
                    } else {
                        run_length += 1;
                    }
                },
                None => {
                    if run_length > 0 {
                        result.push(format!("{}", run_length));
                    }

                    break;
                }
            };
        }

        result
    }


    fn matches(moves: &[String], a: &[String], b: &[String], c: &[String]) -> bool {
        if moves.len() == 0 {
            true
        } else {
            (&moves[0..a.len()] == a && matches(&moves[a.len()..], a, b, c)) ||
                (&moves[0..b.len()] == b && matches(&moves[b.len()..], a, b, c)) ||
                (&moves[0..c.len()] == c && matches(&moves[c.len()..], a, b, c))
        }
    }

    fn find_groups(moves: &str) -> Option<(Vec<String>, Vec<String>, Vec<String>)> {
        let moves: Vec<String> = moves.split(",").map(str::to_owned).collect();

        for a_len in 1..20 {
            for b_len in 1..20 {
                for c_len in 1..20 {
                    for b_gap in 0..(moves.len() - a_len - b_len - c_len) {
                        for c_gap in 0..(moves.len() - a_len - b_len - c_len - b_gap) {
                            if (a_len + b_len + c_len + b_gap + c_gap) >= moves.len() {
                                continue;
                            }

                            let a_start = 0;
                            let b_start = a_len + b_gap;
                            let c_start = b_start + b_len + c_gap;

                            let a = &moves[a_start..a_len];
                            let b = &moves[b_start..(b_start + b_len)];
                            let c = &moves[c_start..(c_start + c_len)];

                            if matches(&moves, a, b, c) {
                                return Some((a.to_vec(), b.to_vec(), c.to_vec()));
                            }
                        }
                    }
                }
            }
        }

        None
    }

    pub fn part2() {
        // Path strategy: always go as far as you can without stopping.  This string output from the simulator.
        let movements = "L111111L1111R11111111R11111111L111111L1111L1111111111R11111111L111111L1111R11111111L1111R1111L1111R11111111R11111111L111111L1111L1111111111R11111111L1111R1111L1111R11111111R11111111L111111L1111L1111111111R11111111L1111R1111L1111R11111111L111111L1111R11111111R11111111L111111L1111L1111111111R11111111";
        let collapsed = collapse_movements(movements).join(",");

        if let Some((a, b, c)) = find_groups(&collapsed) {
            let a = a.join(",");
            let b = b.join(",");
            let c = c.join(",");

            let main_program = collapsed.replace(&a, "A").replace(&b, "B").replace(&c, "C");

            let mut code: Vec<i64> = read_file("input_files/day17.txt").split(',').map(|s| s.parse().unwrap()).collect();

            // Enter the right mode
            assert_eq!(code[0], 1);
            code[0] = 2;

            let mut input: Vec<char> = Vec::new();

            input.extend(main_program.chars()); input.push('\n');
            input.extend(a.chars()); input.push('\n');
            input.extend(b.chars()); input.push('\n');
            input.extend(c.chars()); input.push('\n');
            input.push('n'); input.push('\n');

            // intcode wants input in reverse order and as i64s
            let input: Vec<i64> = input.iter().map(|&ch| ch as i64).rev().collect();

            let mut intcode = intcode::new(
                code,
                input,
                Vec::new(),
            );

            intcode.evaluate();

            assert!(intcode.terminated);
            dbg!(intcode.output.pop().unwrap());
        } else {
            panic!("No solution found");
        }
    }
}


mod day18 {
    use crate::shared::*;

    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
    struct Point (i64, i64);

    impl Point {
        fn neighbours(&self) -> Vec<Point> {
            vec!(Point(self.0 + 1, self.1),
                 Point(self.0 - 1, self.1),
                 Point(self.0, self.1 + 1),
                 Point(self.0, self.1 - 1))
        }
    }

    type KeyType = char;

    #[derive(Debug, Clone, Copy)]
    enum Tile {
        Open,
        Key(KeyType),
        Door(KeyType),
        Wall,
    }

    #[derive(Debug, Clone)]
    struct World {
        grid: Vec<Vec<Tile>>,
        starting_positions: Vec<Point>,
        key_count: usize,
    }

    impl World {
        fn parse(s: &str) -> World {
            let mut key_count = 0;
            let mut starting_positions = Vec::new();

            let grid = s.split("\n").enumerate().map(|(y, row)| {
                row.chars().enumerate().map(|(x, ch)| {
                    match ch {
                        'A'..='Z' => { Tile::Door(ch.to_ascii_lowercase()) },
                        'a'..='z' => {
                            key_count += 1;
                            Tile::Key(ch.to_ascii_lowercase())
                        },
                        '@' => {
                            starting_positions.push(Point(x as i64, y as i64));
                            Tile::Open
                        },
                        '#' => Tile::Wall,
                        '.' => Tile::Open,
                        _ => panic!("Unrecognised tile: {}", ch),
                    }
                }).collect()
            }).collect();

            World { grid, key_count, starting_positions }
        }

        fn width(&self) -> usize {
            self.grid[0].len()
        }

        fn height(&self) -> usize {
            self.grid.len()
        }

        fn tile_at(&self, p: Point) -> Tile {
            self.grid[p.1 as usize][p.0 as usize]
        }

        fn count_keys(&self) -> usize {
            self.key_count
        }
    }

    #[derive(Eq, PartialEq, Hash, Clone)]
    struct PlayerState {
        position: Point,
        key_set: KeySet,
    }

    #[derive(Eq, PartialEq)]
    struct State {
        player: PlayerState,
        accumulated_cost: usize,
    }

    impl std::cmp::Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            self.accumulated_cost.cmp(&other.accumulated_cost)
        }
    }

    impl std::cmp::PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(Eq, PartialEq, Hash, Clone, Copy)]
    struct KeySet {
        bitset: usize,
        count: usize,
    }

    impl KeySet {
        fn new() -> KeySet {
            KeySet { bitset: 0, count: 0 }
        }

        fn has_key(&self, key: KeyType) -> bool {
            let flag = 1 << (key as usize - 'a' as usize);
            (self.bitset & flag) != 0
        }

        fn add_key(&self, key: KeyType) -> KeySet {
            let flag = 1 << (key as usize - 'a' as usize);
            KeySet {
                bitset: self.bitset | flag,
                count: self.count + 1,
            }
        }

        fn contains_all(&self, other: &KeySet) -> bool {
            (self.bitset & other.bitset) == other.bitset
        }
    }

    impl State {
        fn new(p: Point) -> State {
            State {
                player: PlayerState {
                    position: p,
                    key_set: KeySet::new(),
                },
                accumulated_cost: 0,
            }
        }

        fn has_key(&self, key: KeyType) -> bool {
            self.player.key_set.has_key(key)
        }

        fn add_key(&self, key: KeyType, cost: usize, new_position: Point) -> State {
            State {
                player: PlayerState {
                    position: new_position,
                    key_set: self.player.key_set.add_key(key),
                },
                accumulated_cost: self.accumulated_cost + cost,
            }
        }

        fn count_keys(&self) -> usize {
            self.player.key_set.count
        }
    }

    struct Reachability {
        keys_required: KeySet,
        cost: usize,
        target_key: KeyType,
        target_position: Point,
    }

    struct ReachabilityMap {
        map: HashMap<Point, Vec<Reachability>>,
    }

    impl ReachabilityMap {
        fn build(world: &World) -> ReachabilityMap {
            let mut result = ReachabilityMap {
                map: HashMap::new(),
            };

            let mut points_of_interest = world.starting_positions.clone();

            for y in 0..world.height() {
                for x in 0..world.width() {
                    let p = Point(x as i64, y as i64);
                    if let Tile::Key(_) = world.tile_at(p) {
                        points_of_interest.push(p);
                    }
                }
            }

            for p in points_of_interest {
                let mut reachable_keys = Vec::new();

                let mut queue: VecDeque<(Point, usize, KeySet)> = VecDeque::from(vec!((p, 0, KeySet::new())));
                let mut visited: HashSet<Point> = HashSet::with_capacity(1024);

                while !queue.is_empty() {
                    let (point, cost, key_set) = queue.pop_front().unwrap();

                    for next_p in point.neighbours() {
                        if visited.contains(&next_p) {
                            continue;
                        }

                        visited.insert(next_p);

                        match world.tile_at(next_p) {
                            Tile::Key(found_key) => {
                                // Found a key... hooray!
                                reachable_keys.push(Reachability {
                                    keys_required: key_set,
                                    cost: cost + 1,
                                    target_key: found_key,
                                    target_position: next_p,
                                });

                                queue.push_back((next_p, cost + 1, key_set));
                            },
                            Tile::Open => {
                                queue.push_back((next_p, cost + 1, key_set));
                            },
                            Tile::Door(door_key) => {
                                queue.push_back((next_p, cost + 1, key_set.add_key(door_key)));
                            },
                            _ => {
                                // Anything else and we're stuck
                            }
                        }
                    }
                }

                result.map.insert(p, reachable_keys);
            }

            result
        }

        fn find_reachable_keys(&self, state: &State) -> Vec<(KeyType, usize, Point)> {
            self.map.get(&state.player.position).unwrap().iter().filter(|reachability| {
                // We're OK if we have all the right keys
                state.player.key_set.contains_all(&reachability.keys_required) && !state.player.key_set.has_key(reachability.target_key)
            })
                .map(|reachability| (reachability.target_key, reachability.cost, reachability.target_position))
                .collect()
        }
    }

    pub fn part1() {
        let world = World::parse(&read_file("input_files/day18.txt"));

        let reachability_map = ReachabilityMap::build(&world);

        let mut queue = std::collections::BinaryHeap::new();

        assert!(world.starting_positions.len() == 1);

        queue.push(State::new(world.starting_positions[0]));

        let mut best = std::usize::MAX;

        let mut seen_player_states: HashMap<PlayerState, usize> = HashMap::new();

        while !queue.is_empty() {
            let state = queue.pop().unwrap();

            if state.accumulated_cost > best {
                // No point continuing this path
                continue;
            }

            if let Some(cost) = seen_player_states.get(&state.player) {
                if state.accumulated_cost >= *cost {
                    continue;
                }
            }

            seen_player_states.insert(state.player.clone(), state.accumulated_cost);

            if state.count_keys() == world.count_keys() {
                if state.accumulated_cost < best {
                    best = state.accumulated_cost;
                    println!("Finished in {}", state.accumulated_cost);
                }
            }

            for (key, cost, point) in reachability_map.find_reachable_keys(&state) {
                let next_state = state.add_key(key, cost, point);
                queue.push(next_state);
            }
        }
    }

    struct RobotStates {
        robots: Vec<State>,
    }

    impl RobotStates {
        fn new(starting_positions: Vec<Point>) -> RobotStates {
            RobotStates {
                robots: starting_positions.iter().map(|&position| State::new(position)).collect(),
            }
        }
    }

    pub fn part2() {
        let world = World::parse(&read_file("input_files/day18_pt2.txt"));

        let reachability_map = ReachabilityMap::build(&world);

        // let mut queue = std::collections::BinaryHeap::new();
        //
        // queue.push(State::new(world.starting_position));
        //
        // let mut best = std::usize::MAX;
        //
        // let mut seen_player_states: HashMap<PlayerState, usize> = HashMap::new();
        //
        // while !queue.is_empty() {
        //     let state = queue.pop().unwrap();
        //
        //     if state.accumulated_cost > best {
        //         // No point continuing this path
        //         continue;
        //     }
        //
        //     if let Some(cost) = seen_player_states.get(&state.player) {
        //         if state.accumulated_cost >= *cost {
        //             continue;
        //         }
        //     }
        //
        //     seen_player_states.insert(state.player.clone(), state.accumulated_cost);
        //
        //     if state.count_keys() == world.count_keys() {
        //         if state.accumulated_cost < best {
        //             best = state.accumulated_cost;
        //             println!("Finished in {}", state.accumulated_cost);
        //         }
        //     }
        //
        //     for (key, cost, point) in reachability_map.find_reachable_keys(&state) {
        //         let next_state = state.add_key(key, cost, point);
        //         queue.push(next_state);
        //     }
        // }
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
        day13::part2_deluxe_mode();

        day14::part1();
        day14::part2();

        day15::part1();
        day15::part2();

        day16::part1();
        day16::part2();

        day17::part1();
        day17::part2_simulator();
        day17::part2();
    }

    day18::part1();
    // day18::part2();
}
