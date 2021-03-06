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

        pub instructions_executed: usize,

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
            instructions_executed: 0,
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
                self.instructions_executed += 1;
                let instruction = self.next_instruction();
                instruction.apply(self);
            }
        }

        pub fn step(&mut self) {
            if (!self.input.is_empty()) {
                self.waiting_for_input = false;
            }

            if !self.terminated && !self.waiting_for_input {
                self.instructions_executed += 1;
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
    pub use std::collections::BTreeSet;
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
    pub use std::convert::TryInto;

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
        fs::read_to_string(file).unwrap().to_owned()
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

        fn accumulated_cost(&self) -> usize {
            self.accumulated_cost
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

        fn find_reachable_keys(&self, state: &PlayerState) -> Vec<(KeyType, usize, Point)> {
            self.map.get(&state.position).unwrap().iter().filter(|reachability| {
                // We're OK if we have all the right keys
                state.key_set.contains_all(&reachability.keys_required) && !state.key_set.has_key(reachability.target_key)
            })
                .map(|reachability| (reachability.target_key, reachability.cost, reachability.target_position))
                .collect()
        }
    }

    pub fn part1() {
        let world = World::parse(&read_file("input_files/day18.txt"));

        let reachability_map = ReachabilityMap::build(&world);

        let mut queue = std::collections::BinaryHeap::new();

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

            for (key, cost, point) in reachability_map.find_reachable_keys(&state.player) {
                let next_state = state.add_key(key, cost, point);
                queue.push(next_state);
            }
        }
    }

    #[derive(Eq, PartialEq, Hash, Clone)]
    struct RobotStates {
        robots: Vec<PlayerState>,
        accumulated_cost: usize,
    }

    impl RobotStates {
        fn new(starting_positions: Vec<Point>) -> RobotStates {
            RobotStates {
                robots: starting_positions.iter().map(|&p| PlayerState {
                    position: p,
                    key_set: KeySet::new(),
                }).collect(),
                accumulated_cost: 0,
            }
        }

        fn accumulated_cost(&self) -> usize {
            self.accumulated_cost
        }

        fn count_keys(&self) -> usize {
            // All robots will be given all keys
            self.robots[0].key_set.count
        }

        fn add_key(&self, robot: usize, key: KeyType, cost: usize, new_position: Point) -> RobotStates {
            let mut robots = self.robots.clone();

            // Target robot moves
            robots[robot].position = new_position;

            // All robots get the key
            for mut r in robots.iter_mut() {
                r.key_set = r.key_set.add_key(key);
            }

            RobotStates {
                robots,
                accumulated_cost: self.accumulated_cost + cost,
            }
        }
    }

    impl std::cmp::Ord for RobotStates {
        fn cmp(&self, other: &Self) -> Ordering {
            self.accumulated_cost.cmp(&other.accumulated_cost)
        }
    }

    impl std::cmp::PartialOrd for RobotStates {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }


    pub fn part2() {
        let world = World::parse(&read_file("input_files/day18_pt2.txt"));

        let reachability_map = ReachabilityMap::build(&world);

        let mut queue = std::collections::BinaryHeap::new();

        queue.push(RobotStates::new(world.starting_positions.clone()));

        let mut best = std::usize::MAX;

        let mut seen_states: HashMap<RobotStates, usize> = HashMap::new();

        while !queue.is_empty() {
            let state = queue.pop().unwrap();

            if state.accumulated_cost > best {
                // No point continuing this path
                continue;
            }

            if let Some(cost) = seen_states.get(&state) {
                if state.accumulated_cost >= *cost {
                    continue;
                }
            }

            seen_states.insert(state.clone(), state.accumulated_cost);

            if state.count_keys() == world.count_keys() {
                if state.accumulated_cost < best {
                    best = state.accumulated_cost;
                    println!("Finished in {}", state.accumulated_cost);
                }
            }

            for robot in 0..4 {
                let robot_state = &state.robots[robot];

                for (key, cost, point) in reachability_map.find_reachable_keys(robot_state) {
                    let next_state = state.add_key(robot, key, cost, point);
                    queue.push(next_state);
                }
            }
        }
    }
}


mod day19 {
    use crate::shared::*;

    pub fn part1() {
        let code: Vec<i64> = read_file("input_files/day19.txt").split(',').map(|s| s.parse().unwrap()).collect();

        let mut count = 0;

        for y in 0..50 {
            for x in 0..50 {

                let mut intcode = intcode::new(
                    code.clone(),
                    vec!(y, x),
                    Vec::new(),
                );

                intcode.evaluate();

                assert_eq!(intcode.output.len(), 1);
                if intcode.output.pop().unwrap() == 1 {
                    count += 1;
                }
            }
        }

        println!("{} points affected", count);
    }

    pub fn part2() {
        let code: Vec<i64> = read_file("input_files/day19.txt").split(',').map(|s| s.parse().unwrap()).collect();

        let point_in_beam = |x, y| {
            if x < 0 || y < 0 {
                return false;
            }

            let mut intcode = intcode::new(
                code.clone(),
                vec!(y, x),
                Vec::new(),
            );

            intcode.evaluate();

            intcode.output.pop().unwrap() == 1
        };

        for y in 0..2000 {
            match (0..2000).find(|&x| point_in_beam(x, y)) {
                Some(x) => {
                    if point_in_beam(x + 99, y) &&
                        point_in_beam(x + 99, y - 99) &&
                        point_in_beam(x, y - 99) {
                            println!("x={}; y={}", x, y - 99);
                            return;
                        }
                },
                None => {},
            }
        }
    }
}


mod day20 {
    use crate::shared::*;

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    struct Point { x: usize, y: usize }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Tile {
        Open,
        Wall,
        Teleporter(Teleporter)
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    struct Teleporter {
        sends_to: Point,
        depth_adjustment: i64,
    }

    #[derive(Debug)]
    struct DonutWorld {
        grid: Vec<Vec<Tile>>,
        teleporter_locations: HashMap<Point, Teleporter>,
    }

    impl DonutWorld {
        fn parse(s: &str) -> DonutWorld {
            let input: Vec<Vec<char>> = s.split("\n").map(|line| {
                line.chars().collect()
            }).filter(|line: &Vec<_>| !line.is_empty())
                .collect();

            let input_width = input[0].len();
            let input_height = input.len();

            // Assume our input has a margin of 2 cells around it
            let margin = 2;
            let grid_width = input_width - (margin * 2);
            let grid_height = input_height - (margin * 2);

            let mut grid: Vec<Vec<Tile>> = (0..grid_height).map(|_| vec![Tile::Wall; grid_width]).collect();
            let mut teleporters = HashMap::new();

            // Teleporters that are waiting until we discover their corresponding location
            let mut pending_teleporters: HashMap<String, Point> = HashMap::new();

            let mut found_teleporter = |point: Point, ch1, ch2, depth_adjustment| {
                let label = format!("{}{}", ch1, ch2);
                if pending_teleporters.contains_key(&label) {
                    let other_point = pending_teleporters.remove(&label).unwrap();

                    teleporters.insert(point.clone(),
                                       Teleporter {
                                           sends_to: other_point.clone(),
                                           depth_adjustment: depth_adjustment,
                                       });

                    teleporters.insert(other_point.clone(),
                                       Teleporter {
                                           sends_to: point.clone(),
                                           depth_adjustment: depth_adjustment * -1,
                                       });
                } else {
                    pending_teleporters.insert(label, point);
                }
            };

            for grid_y in 0..grid_height {
                for grid_x in 0..grid_width {
                    let input_y = grid_y + margin;
                    let input_x = grid_x + margin;

                    grid[grid_y][grid_x] = match input[input_y][input_x] {
                        '.' => {
                            // There might be a teleporter here too.  Look around for that.
                            if input[input_y][input_x - 1].is_ascii_uppercase() {
                                let outer = (input_x == margin);
                                found_teleporter(Point {x: grid_x, y: grid_y},
                                                 input[input_y][input_x - 2], input[input_y][input_x - 1],
                                                 if outer { -1 } else { 1 });
                            } else if input[input_y][input_x + 1].is_ascii_uppercase() {
                                let outer = (grid_x == (grid_width - 1));
                                found_teleporter(Point {x: grid_x, y: grid_y},
                                                 input[input_y][input_x + 1], input[input_y][input_x + 2],
                                                 if outer { -1 } else { 1 });
                            } else if input[input_y - 1][input_x].is_ascii_uppercase() {
                                let outer = (input_y == margin);
                                found_teleporter(Point {x: grid_x, y: grid_y},
                                                 input[input_y - 2][input_x], input[input_y - 1][input_x],
                                                 if outer { -1 } else { 1 });
                            } else if input[input_y + 1][input_x].is_ascii_uppercase() {
                                let outer = (grid_y == (grid_height - 1));
                                found_teleporter(Point {x: grid_x, y: grid_y},
                                                 input[input_y + 1][input_x], input[input_y + 2][input_x],
                                                 if outer { -1 } else { 1 });
                            }

                            Tile::Open
                        },
                        _ => Tile::Wall,
                    }
                }
            }

            DonutWorld { grid, teleporter_locations: teleporters }
        }

        fn width(&self) -> i64 {
            self.grid[0].len() as i64
        }

        fn height(&self) -> i64 {
            self.grid.len() as i64
        }

        fn reachable_positions(&self, p: Point) -> Vec<(Point, i64)> {
            let mut result = Vec::with_capacity(4);

            // If we're on a teleporter, we can teleport if we like
            match self.teleporter_locations.get(&p) {
                Some(teleporter) => {
                    result.push((teleporter.sends_to.clone(), 
                                 teleporter.depth_adjustment));
                },
                _ => {}
            }

            // We can move to neighbours on the same level
            for (x_off, y_off) in &[(-1, 0), (1, 0), (0, 1), (0, -1)] {
                let new_x = (p.x as i64 + x_off);
                let new_y = (p.y as i64 + y_off);

                if new_x < 0 || new_y < 0 || new_x >= self.width() || new_y >= self.height() {
                    continue;
                }

                match self.grid[new_y as usize][new_x as usize] {
                    Tile::Open => {
                        result.push((Point {
                            x: new_x as usize,
                            y: new_y as usize,
                        }, 0));
                    },
                    _ => {}
                }
            }

            result
        }
    }


    #[derive(Hash, Eq, PartialEq, Debug, Clone)]
    struct State {
        position: Point,
        accumulated_cost: usize,
        level: i64,
    }

    impl std::cmp::Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            self.level.cmp(&other.level).then(self.accumulated_cost.cmp(&other.accumulated_cost))
        }
    }

    impl std::cmp::PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }


    pub fn part1() {
        let world = DonutWorld::parse(&read_file_raw("input_files/day20.txt"));

        let mut queue: VecDeque<State> = VecDeque::from(vec!(State {
            position: Point { x: 0, y: 61 }, 
            accumulated_cost: 0,
            level: 0,
        }));
        let target = Point { x: 57, y: 110 };

        let mut seen_states: HashSet<State> = HashSet::new();

        while !queue.is_empty() {
            let state = queue.pop_front().unwrap();

            if seen_states.contains(&state) {
                continue;
            }

            seen_states.insert(state.clone());

            if state.position == target {
                println!("Out in {} steps", state.accumulated_cost);
                break;
            }

            for (next_p, depth_adjustment) in world.reachable_positions(state.position) {
                queue.push_back(State {
                    position: next_p,
                    accumulated_cost: state.accumulated_cost + 1,
                    level: state.level + depth_adjustment,
                });
            }
        }

    }

    pub fn part2() {
        let world = DonutWorld::parse(&read_file_raw("input_files/day20.txt"));

        // Realsies
        let start_pos = Point { x: 0, y: 61 };
        let target = Point { x: 57, y: 110 };

        // SAMPLE 2
        // let start_pos = Point { x: 13, y: 32 };
        // let target = Point { x: 11, y: 0 };

        let max_depth = 200;

        let mut queue = std::collections::BinaryHeap::new();
        queue.push(State {
            position: start_pos,
            accumulated_cost: 0,
            level: 0,
        });

        let mut seen_states: HashMap<(Point, i64), usize> = HashMap::new();

        while !queue.is_empty() {
            let state = queue.pop().unwrap();

            match seen_states.get(&(state.position, state.level)) {
                Some(&cost) => {
                    if cost <= state.accumulated_cost {
                        continue;
                    }
                },
                _ => {}
            }

            if state.level > max_depth {
                continue;
            }

            seen_states.insert((state.position.clone(), state.level), state.accumulated_cost);

            if state.position == target && state.level == 0 {
                println!("Out in {} steps", state.accumulated_cost);
            }

            for (next_p, depth_adjustment) in world.reachable_positions(state.position) {
                if state.level + depth_adjustment >= 0 {
                    queue.push(State {
                        position: next_p,
                        accumulated_cost: state.accumulated_cost + 1,
                        level: state.level + depth_adjustment,
                    });
                }
            }
        }
    }
}


mod day21 {
    use crate::shared::*;

    extern crate rand;
    use day21::rand::prelude::*;


    fn ascii_code(lines: &Vec<String>) -> Vec<i64> {
        let mut result = Vec::new();

        for line in lines.iter().rev() {
            result.push('\n' as char as i64);
            result.extend(line.chars().rev().map (|ch| ch as i64));
        }

        result
    }

    fn random_program_pt1() -> Vec<String> {
        let instructions = &["AND", "OR", "NOT"];
        let inputs = &["A", "B", "C", "D", "T", "J"];
        let outputs = &["T", "J"];

        let mut rng = rand::thread_rng();
        (0..15).map(|_| {
            let instruction = rng.gen_range(0, instructions.len());
            let input = rng.gen_range(0, inputs.len());
            let output = rng.gen_range(0, outputs.len());

            format!("{} {} {}", instructions[instruction], inputs[input], outputs[output])
        }).collect()
    }

    pub fn part1() {
        let code: Vec<i64> = read_file("input_files/day21.txt").split(',').map(|s| s.parse().unwrap()).collect();

        loop {
            let mut program = random_program_pt1();
            program.push("WALK".to_string());

            let mut intcode = intcode::new(
                code.clone(),
                ascii_code(&program),
                Vec::new(),
            );

            intcode.evaluate();

            if intcode.output.iter().all(|&ch| ch >= 0 && ch <= 255) {
                // print!("{}", intcode.output.iter().map(|&ch| ch as u8 as char).collect::<String>());
            } else {
                // Made it!
                dbg!(intcode.output);
                break
            }
        }
    }

    fn random_instruction(rng: &mut impl Rng) -> String {
        let instructions = &["AND", "OR", "NOT"];
        let inputs = &["A", "B", "C", "D", "E", "F", "G", "H", "I", "T", "J"];
        let outputs = &["T", "J"];

        let instruction = rng.gen_range(0, instructions.len());
        let input = rng.gen_range(0, inputs.len());
        let output = rng.gen_range(0, outputs.len());

        format!("{} {} {}", instructions[instruction], inputs[input], outputs[output])
    }


    fn random_program_pt2(rng: &mut impl Rng) -> Vec<String> {
        let mut result: Vec<String> = (0..15).map(|_| {
            random_instruction(rng)
        }).collect();

        result.push("RUN".to_owned());
        result
    }

    fn mutate_program(program: &mut Vec<String>, rng: &mut impl Rng) {
        let idx = rng.gen_range(0, 15);
        program[idx] = random_instruction(rng);
    }

    // MACHINE LEARNING BABY
    pub fn part2() {
        let code: Vec<i64> = read_file("input_files/day21.txt").split(',').map(|s| s.parse().unwrap()).collect();
        let mut rng = rand::thread_rng();

        let min_score = 50000;
        let mut best: Option<Vec<String>> = None;
        let mut best_score = 0;

        let mut spins = 0;

        while best == None || spins < 1000 {
            spins += 1;
            let program = match best.clone() {
                Some(p) => {
                    let mut new_program = p.clone();
                    mutate_program(&mut new_program, &mut rng);
                    new_program
                },
                None => random_program_pt2(&mut rng)
            };

            let mut intcode = intcode::new(
                code.clone(),
                ascii_code(&program),
                Vec::new(),
            );

            intcode.evaluate();

            let last_output = intcode.output.last().unwrap();

            if *last_output > 255 {
                println!("RESULT: {}", last_output);
                break
            } else {
                // print!("{}", intcode.output.iter().map(|&ch| ch as u8 as char).collect::<String>());
                // break;
            }

            if intcode.instructions_executed > min_score && intcode.instructions_executed > best_score {
                println!("Set best! - score was {}", intcode.instructions_executed);
                best = Some(program);
                best_score = intcode.instructions_executed;
                spins = 0
            }
        }
    }
}


mod day22 {
    /*

    I'm going to leave all evidence of my previous aborted attempts here, but
    the "refired" versions are what eventually got me over the line here.  The
    hint I got from Reddit was part of it:

    > Indeed, each of the shuffling functions is of the general form:
    >
    > "x" goes to "(A*x + B) mod N"
    >
    > Where sometimes B is zero or A is -1. The reverse shuffling functions are then also of this form.

    Applying each step in the forward direction fell out pretty easily: either
    you were adjusting the coefficient, or the offset, or both.  Using pencil
    and paper shenanigans I worked out how to apply the steps twice, and that's
    what `double_transform` does.

    Once I had that, I needed to work out how to do it all in reverse, since I'm
    trying to start with position 2020 and work backwards.  The trick, then, was
    to come up with the same transformations in reverse.

    The hard part here was reversing the deal function, which was p * n % C.  I
    knew C was prime, so intuitively I felt like I should be able to work
    backwards to the original position, but my maths skills were failing me.
    Googling for inverse modular arithmetic suggested that Euclid's Extended
    Algorithm could help, but I was having trouble applying it.

    Salvation came from here:

    https://math.stackexchange.com/questions/684550/how-to-reverse-modulo-of-a-multiplication

    and that showed how to apply the coefficient from Euclid's Extended
    Algorithm to what I had to derive the original position.

    Once I had that, I needed to apply my reversed steps an insane number of
    times.  To save on work, I broke my shuffle count into sums of powers of
    two, and used my transform doubling function to fast-path those power of two
    repetitions.  I was still getting the wrong answer--and I noticed the answer
    I got changed when I applied the transforms in varying orders, which seemed
    wrong.

    Running in debug mode flagged the error: I was overflowing on
    multiplications.  I had a hazy memory from Project Euler that there was an
    algorithm for doing modular multiplication without overflow, and I found a
    function that worked for that.  And that was it!  Two weeks later I had my
    answer :)

     */


    use crate::shared::*;

    type Deck = VecDeque<usize>;

    fn deal_into_new_stack(deck: &mut Deck) {
        let mut i = 0;
        let mut j = deck.len() - 1;

        while i < j {
            deck.swap(i, j);

            i += 1;
            j -= 1;
        }
    }

    fn cut(deck: &mut Deck, n: i64) {
        let mut size = n.abs();

        if n >= 0 {
            while size > 0 {
                let elt = deck.pop_front().unwrap();
                deck.push_back(elt);
                size -= 1;
            }
        } else {
            while size > 0 {
                let elt = deck.pop_back().unwrap();
                deck.push_front(elt);
                size -= 1;
            }
        }
    }

    fn deal(deck: &mut Deck, n: usize) {
        let mut table: Vec<Option<usize>> = vec![None; deck.len()];
        let mut i = 0;

        while !deck.is_empty() {
            let card = deck.pop_front().unwrap();
            assert!(table[i] == None);

            table[i] = Some(card);

            i = (i + n) % table.len();
        }

        while !table.is_empty() {
            let card = table.pop().unwrap();

            deck.push_front(card.unwrap());
        }
    }

    pub fn part1() {
        let mut deck: Deck = (0..10007).collect();

        for instruction in input_lines("input_files/day22.txt") {
            if instruction == "deal into new stack" {
                deal_into_new_stack(&mut deck);
            } else {
                // Otherwise, we're an instruction with a numeric bit
                let idx = instruction.rfind(" ").unwrap();
                let operation = &instruction[0..idx];
                let n: i64 = instruction[idx + 1..instruction.len()].parse().expect(&format!("Bad mojo: {}", &instruction));

                match operation {
                    "cut" => { cut(&mut deck, n) },
                    "deal with increment" => { deal(&mut deck, n as usize) },
                    _ => panic!("Dunno: {}", instruction),
                }
            }
        }

        // for (new_pos, old_pos) in deck.iter().enumerate() {
        //     println!("{} => {}", old_pos, new_pos);
        // }

        let pos = deck.iter().position(|&card| card == 2019).unwrap();
        println!("Card 2019 is at position {}", pos);
    }


    // const CARD_COUNT: u64 = 10007;


    fn position_before_deal_into_new_stack(position: u64) -> u64 {
        CARD_COUNT - position - 1
    }

    fn position_before_cut(position: u64, n: i64) -> u64 {
        let abs_n: u64 = n.abs().try_into().unwrap();

        if n > 0 {
            if position >= (CARD_COUNT - abs_n) {
                // We took from the left and added to the right

                // these were originally on the left
                return position - (CARD_COUNT - abs_n);
            } else {
                // shifted left
                return position + abs_n;
            }
        } else if n < 0 {
            if position < abs_n {
                // We took from the right and added to the left

                // These were originally on the right
                return CARD_COUNT - abs_n + position;
                // return position - (CARD_COUNT - abs_n);
            } else {
                // shifted right
                return position - abs_n;
            }
        } else {
            return position;
        }
    }


    // https://en.wikipedia.org/wiki/Modular_multiplicative_inverse ?
    //
    // Need extended euclidian algorithm?

    fn position_before_deal(position: u64, n: u64) -> u64 {
        let mut idx = 0;
        let mut orig_idx = 0;

        if position == 0 {
            // Zero never moves
            return 0;
        }

        loop {
            if (idx % CARD_COUNT) == position % n {
                // we'll find our target this loop.  Iterate!
                let offset = (position - idx) / n;
                return orig_idx + offset;
            }

            // Otherwise, we can skip up to the end of the current cycle
            let positions_to_skip = ((CARD_COUNT - idx) / n);
            if positions_to_skip > 0 {
                orig_idx += positions_to_skip;
                idx += positions_to_skip * n;
            }


            // and move to the next one
            orig_idx += 1;
            idx = (idx + n) % CARD_COUNT;
        }
    }


    // Hint from reddit:

    // Indeed, each of the shuffling functions is of the general form:
    //
    // "x" goes to "(A*x + B) mod N"
    //
    // Where sometimes B is zero or A is -1. The reverse shuffling functions are then also of this form.

    pub fn part2() {
        let mut position = 2020;
        println!("{}", position);

        for _round in (0..1000000) {
            position = position_before_deal_into_new_stack(position);
            position = position_before_cut(position, 7411);
            position = position_before_deal_into_new_stack(position);
            position = position_before_cut(position, -8660);
            position = position_before_deal(position, 63);
            position = position_before_cut(position, -4407);
            position = position_before_deal(position, 29);
            position = position_before_deal_into_new_stack(position);
            position = position_before_cut(position, -7243);
            position = position_before_deal_into_new_stack(position);
            position = position_before_cut(position, -896);
            position = position_before_deal_into_new_stack(position);
            position = position_before_deal(position, 67);
            position = position_before_deal_into_new_stack(position);
            position = position_before_cut(position, 7005);
            position = position_before_deal(position, 75);
            position = position_before_cut(position, 8417);
            position = position_before_deal(position, 38);
            position = position_before_cut(position, -8065);
            position = position_before_deal(position, 75);
            position = position_before_cut(position, -8491);
            position = position_before_deal(position, 35);
            position = position_before_cut(position, 2255);
            position = position_before_deal(position, 26);
            position = position_before_cut(position, 5823);
            position = position_before_deal(position, 60);
            position = position_before_cut(position, -9915);
            position = position_before_deal(position, 13);
            position = position_before_cut(position, 3203);
            position = position_before_deal(position, 64);
            position = position_before_cut(position, -2973);
            position = position_before_deal(position, 59);
            position = position_before_cut(position, -2963);
            position = position_before_deal(position, 5);
            position = position_before_cut(position, 3019);
            position = position_before_deal(position, 75);
            position = position_before_deal_into_new_stack(position);
            position = position_before_cut(position, -9068);
            position = position_before_deal(position, 33);
            position = position_before_cut(position, 8430);
            position = position_before_deal(position, 61);
            position = position_before_cut(position, -3460);
            position = position_before_deal_into_new_stack(position);
            position = position_before_deal(position, 46);
            position = position_before_cut(position, -3600);
            position = position_before_deal(position, 70);
            position = position_before_cut(position, -5937);
            position = position_before_deal_into_new_stack(position);
            position = position_before_cut(position, -7962);
            position = position_before_deal(position, 34);
            position = position_before_cut(position, 6364);
            position = position_before_deal(position, 18);
            position = position_before_cut(position, -3388);
            position = position_before_deal(position, 63);
            position = position_before_deal_into_new_stack(position);
            position = position_before_cut(position, 4783);
            position = position_before_deal(position, 32);
            position = position_before_cut(position, 9777);
            position = position_before_deal(position, 73);
            position = position_before_cut(position, 5945);
            position = position_before_deal(position, 20);
            position = position_before_deal_into_new_stack(position);
            position = position_before_deal(position, 42);
            position = position_before_cut(position, 4665);
            position = position_before_deal(position, 40);
            position = position_before_cut(position, -1405);
            position = position_before_deal(position, 14);
            position = position_before_deal_into_new_stack(position);
            position = position_before_cut(position, -5074);
            position = position_before_deal_into_new_stack(position);
            position = position_before_deal(position, 32);
            position = position_before_cut(position, -5387);
            position = position_before_deal(position, 70);
            position = position_before_deal_into_new_stack(position);
            position = position_before_cut(position, -4299);
            position = position_before_deal(position, 32);
            position = position_before_cut(position, -6954);
            position = position_before_deal(position, 8);
            position = position_before_cut(position, 3555);
            position = position_before_deal(position, 34);
            position = position_before_deal_into_new_stack(position);
            position = position_before_cut(position, 4879);
            position = position_before_deal(position, 62);
            position = position_before_deal_into_new_stack(position);
            position = position_before_deal(position, 62);
            position = position_before_deal_into_new_stack(position);
            position = position_before_cut(position, -1543);
            position = position_before_deal(position, 20);
            position = position_before_cut(position, 7577);
            position = position_before_deal(position, 3);
            position = position_before_cut(position, -4724);
            position = position_before_deal(position, 7);
            position = position_before_cut(position, 8507);
            position = position_before_deal(position, 12);
            position = position_before_cut(position, -4758);
            position = position_before_deal(position, 64);
            position = position_before_cut(position, -9977);
            position = position_before_deal(position, 22);
            position = position_before_cut(position, 9569);
            position = position_before_deal(position, 5);
        }

        println!("{}", position);
    }


    pub fn part2_orig() {
        // Vague idea, based on the observation that the middle of the deck doesn't matter all that much:
        //
        // New representation of the deck that only tracks the N first and N last cards
        //
        // Reversing the deck can be carried out easily
        //
        // Cutting too
        //
        // Dealing is more tricky.  Can we cheaply figure out which cards end up where based on modulo arithmetic?
        //
        // Then we need to repeat this many times.  Hopefully once we have the
        // above, we'll find that it cycles after some number of applications
        // anyway.  Then we just need to figure out 101741582076661 % CYCLE_LEN
        // repeats.
        //
        // ALTERNATVE APPROCACH: Can we work backwards in the instructions?  If
        // the card I want is at 2020, and that's after a reverse, then prior to
        // that step it was at MAX - 2020.  Can we apply this reverse thing
        // repeatedly to work out where the card would have to start out?

        // let position = 0;

        // dbg!(position_before_deal_into_new_stack(position));


        //     0123456789
        //  2: 2345678901

        //     0123456789
        // -2: 8901234567


        let mut instructions: Vec<String> = input_lines("input_files/day22_simplified.txt").collect();
        instructions.reverse();

        let mut position = 2020;
        println!("{}", position);
        for _ in (0..100) {
            for instruction in &instructions {
                if instruction == "deal into new stack" {
                    position = position_before_deal_into_new_stack(position);
                } else {
                    // Otherwise, we're an instruction with a numeric bit
                    let idx = instruction.rfind(" ").unwrap();
                    let operation = &instruction[0..idx];
                    let n: i64 = instruction[idx + 1..instruction.len()].parse().expect(&format!("Bad mojo: {}", &instruction));

                    match operation {
                        "cut" => {
                            position = position_before_cut(position, n);
                        },
                        "deal with increment" => {
                            position = position_before_deal(position, n as u64);
                        },
                        _ => panic!("Dunno: {}", instruction),
                    }
                }
            }

            println!("{}", position);
        }
    }


    // p' = coeff * p + offset
    #[derive(Debug, Copy, Clone)]
    struct Transform {
        coeff: i64,
        offset: i64,
    }

    pub fn part1_refired() {
        let mut transform = Transform { coeff: 1, offset: 0 };

        for instruction in input_lines("input_files/day22.txt") {
            if instruction == "deal into new stack" {
                // new_p = (-p - 1) % C
                transform.coeff *= -1;
                transform.offset *= -1;
                transform.offset -= 1;
            } else {
                // Otherwise, we're an instruction with a numeric bit
                let idx = instruction.rfind(" ").unwrap();
                let operation = &instruction[0..idx];
                let n: i64 = instruction[idx + 1..instruction.len()].parse().expect(&format!("Bad mojo: {}", &instruction));

                match operation {
                    "cut" => {
                        // cut(n, p) = (p - n) % C
                        transform.offset -= n;
                    },
                    "deal with increment" => {
                        // deal_inc(i, p) = p * i % C
                        transform.coeff = negmod((transform.coeff * n), CARD_COUNT as i64);
                        transform.offset = negmod((transform.offset * n), CARD_COUNT as i64);
                    },
                    _ => panic!("Dunno: {}", instruction),
                }
            }
        }

        dbg!(transform);

        // let pos = deck.iter().position(|&card| card == 2019).unwrap();
        // println!("Card 2019 is at position {}", pos);
    }

    // Implemented from pseudocode here: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
    fn extended_gcd(a: i64, b: i64) -> (i64, i64) {
        let mut s = 0;
        let mut old_s = 1;

        let mut t = 0;
        let mut old_t = 1;

        let mut r = b;
        let mut old_r = a;

        while r != 0 {
            let quotient = old_r / r;

            // (old_r, r) := (r, old_r − quotient × r)
            let tmp = r;
            r = old_r - quotient * tmp;
            old_r = tmp;

            // (old_s, s) := (s, old_s − quotient × s)
            let tmp = s;
            s = old_s - quotient * tmp;
            old_s = tmp;

            // (old_t, t) := (t, old_t − quotient × t)
            let tmp = t;
            t = old_t - quotient * tmp;
            old_t = tmp;
        }

        return (old_s, old_t);
    }


    const CARD_COUNT: u64 = 119315717514047;
    const SHUFFLES: u64 = 101741582076661;
    const POSITION_OF_INTEREST: i64 = 2020;

    // const CARD_COUNT: u64 = 10007;
    // const SHUFFLES: u64 = 2;
    // const POSITION_OF_INTEREST: i64 = 2235;

    fn mult_mod(a: i64, b: i64, modulo: i64) -> i64 {
        if b < 0 {
            return mult_mod(a, b * -1, modulo) * -1;
        }

        let mut res = 0;
        let mut a = a;
        let mut b = b;

        while b > 0 {
            if b % 2 == 1 {
                res = negmod((res + a), modulo);
            }

            a = negmod((a * 2), modulo);
            b = b / 2;
        }

        res
    }

    pub fn part2_refired() {
        let mut transform = Transform { coeff: 1, offset: 0 };

        let mut instructions: Vec<String> = input_lines("input_files/day22.txt").collect();
        instructions.reverse();

        for instruction in &instructions {
            // dbg!(instruction);

            if instruction == "deal into new stack" {
                // Already its own reverse!
                // new_p = (-p - 1) % C
                transform.coeff *= -1;
                transform.offset *= -1;
                transform.offset -= 1;
            } else {
                // Otherwise, we're an instruction with a numeric bit
                let idx = instruction.rfind(" ").unwrap();
                let operation = &instruction[0..idx];
                let n: i64 = instruction[idx + 1..instruction.len()].parse().expect(&format!("Bad mojo: {}", &instruction));

                match operation {
                    "cut" => {
                        // cut(n, p) = (p - n) % C
                        // rev_cut(n, p) = (p + n) % C
                        transform.offset += n;
                    },
                    "deal with increment" => {
                        // deal_inc(i, p) = p * i % C
                        // rev_deal_inc(i, p) = ?
                        // Ah!  This is where I need some smarts?

                        // https://math.stackexchange.com/questions/684550/how-to-reverse-modulo-of-a-multiplication
                        //
                        // Given: (x * a) % b = c
                        // Where we know a, b, c but not x (our original position)
                        //
                        // Since a and b are coprime, the Euclidean algorithm lets you find s and t such that a⋅s+t⋅b=1 and it does it fairly quickly. This means that
                        //
                        // a*s=1 mod b
                        //
                        // So then you can multiply the sides of your original equation (x * a = c mod b) by the s that is output from the Euclidean algorithm, and:
                        //
                        // x * a * s = c * s mod b
                        // x * 1 = c * s mod b
                        // x = c * s mod b

                        let egcd = extended_gcd(n, CARD_COUNT.try_into().unwrap());

                        transform.coeff = mult_mod(transform.coeff, egcd.0, CARD_COUNT as i64);
                        transform.offset = mult_mod(transform.offset, egcd.0, CARD_COUNT as i64);
                    },
                    _ => panic!("Dunno: {}", instruction),
                }
            }
            // dbg!(&transform);
        }

        let mut transforms: Vec<Transform> = Vec::new();

        for i in 0..64 {
            if ((1 << i) & SHUFFLES) != 0 {
                transforms.push(transform.clone());
            }
            transform = double_transform(&transform);
        }

        let mut position = POSITION_OF_INTEREST;

        for transform in transforms {
            position = negmod(mult_mod(position, transform.coeff, CARD_COUNT as i64) + transform.offset,
                              CARD_COUNT as i64);
        }

        dbg!(&position);
    }

    fn negmod(n: i64, m: i64) -> i64 {
        ((n % m as i64) + m as i64) % m as i64
    }

    fn double_transform(t: &Transform) -> Transform {
        Transform {
            coeff: mult_mod(t.coeff, t.coeff, CARD_COUNT as i64),
            offset: (mult_mod(t.coeff, t.offset, CARD_COUNT as i64) + t.offset) % CARD_COUNT as i64,
        }
    }
}


mod day23 {
    use crate::shared::*;

    #[derive(Clone, Debug)]
    struct Packet {
        x: i64,
        y: i64,
    }

    struct IntCodeForWorkgroups {
        computer: IntCode,
        address: i64,
        inbox: VecDeque<Packet>,
    }

    pub fn part1() {
        let code: Vec<i64> = read_file("input_files/day23.txt").split(',').map(|s| s.parse().unwrap()).collect();

        let mut computers: HashMap<_, IntCodeForWorkgroups> = (0..50).map(|address| {
            (address, IntCodeForWorkgroups {
                computer: intcode::new(code.clone(), vec!(address), Vec::new()),
                address: address,
                inbox: VecDeque::new(),
            })
        }).collect();

        loop {
            // Run a step
            for (_, c) in &mut computers {
                c.computer.step();
            }

            // Deliver any packets that are ready to go out
            let mut deliver_me: Vec<(i64, Packet)> = Vec::new();

            for (_, c) in &mut computers {
                if c.computer.output.len() == 3 {
                    // Packet ready to go out
                    let y = c.computer.output.pop().unwrap();
                    let x = c.computer.output.pop().unwrap();
                    let target = c.computer.output.pop().unwrap();

                    deliver_me.push((target, Packet { x, y }));
                }
            }

            for (address, packet) in deliver_me {
                if address == 255 {
                    // That's our target for part 1
                    println!("Packet addressed to 255 with Y value: {}", packet.y);
                    return
                }

                computers.get_mut(&address).unwrap().inbox.push_front(packet);
            }

            // Fulfill any pending input requests.  If we have a packet for you
            // we'll send it along.  Otherwise have a -1 for no additional
            // charge.
            for (_, c) in &mut computers {
                if c.computer.waiting_for_input {
                    if c.inbox.is_empty() {
                        // No message
                        c.computer.input.push(-1);
                    } else {
                        // Got a message
                        let packet = c.inbox.pop_back().unwrap();

                        // Reversing the order here due to my bad API again :(
                        c.computer.input.push(packet.y);
                        c.computer.input.push(packet.x);
                    }
                }
            }

        }
    }

    pub fn part2() {
        let code: Vec<i64> = read_file("input_files/day23.txt").split(',').map(|s| s.parse().unwrap()).collect();

        let mut computers: HashMap<_, IntCodeForWorkgroups> = (0..50).map(|address| {
            (address, IntCodeForWorkgroups {
                computer: intcode::new(code.clone(), vec!(address), Vec::new()),
                address: address,
                inbox: VecDeque::new(),
            })
        }).collect();

        let mut nat: Option<Packet> = None;

        let mut last_y_value: Option<i64> = None;

        let mut _round = -1;
        let mut consecutive_receive_failures = 0;
        loop {
            _round += 1;
            // Run a step
            for (_, c) in &mut computers {
                c.computer.step();
            }

            // Deliver any packets that are ready to go out
            let mut deliver_me: Vec<(i64, Packet)> = Vec::new();

            for (_, c) in &mut computers {
                if c.computer.output.len() == 3 {
                    // Packet ready to go out
                    let y = c.computer.output.pop().unwrap();
                    let x = c.computer.output.pop().unwrap();
                    let target = c.computer.output.pop().unwrap();

                    deliver_me.push((target, Packet { x, y }));
                }
            }

            for (address, packet) in deliver_me {
                if address == 255 {
                    // Store our NAT packet
                    nat = Some(packet);
                } else {
                    computers.get_mut(&address).unwrap().inbox.push_front(packet);
                }
            }

            // Fulfill any pending input requests.  If we have a packet for you
            // we'll send it along.  Otherwise have a -1 for no additional
            // charge.
            for (_, c) in &mut computers {
                if c.computer.waiting_for_input {
                    if c.inbox.is_empty() {
                        // No message
                        consecutive_receive_failures += 1;
                        c.computer.input.push(-1);
                    } else {
                        // Got a message.  Network no longer idle.
                        consecutive_receive_failures = 0;
                        let packet = c.inbox.pop_back().unwrap();

                        // Reversing the order here due to my bad API again :(
                        c.computer.input.push(packet.y);
                        c.computer.input.push(packet.x);
                    }
                }
            }

            if consecutive_receive_failures >= (computers.len() * 1000) {
                match nat.clone() {
                    Some(packet) => {
                        if last_y_value.is_some() && last_y_value.unwrap() == packet.y {
                            println!("Saw repeated Y value: {}", packet.y);
                            return;
                        }

                        let c = computers.get_mut(&0).unwrap();

                        last_y_value = Some(packet.y);

                        // println!("Round {} failures {} packet y {}", _round, consecutive_receive_failures, packet.y);
                        c.computer.input.push(packet.y);
                        c.computer.input.push(packet.x);

                        consecutive_receive_failures = 0;
                        nat = None;
                    },
                    None => {
                    }
                }
            }

        }
    }
}


mod day24 {
    use crate::shared::*;

    #[derive(Hash, Eq, PartialEq, Clone)]
    enum Tile {
        Bug,
        Empty,
    }

    enum Action {
        Die,
        Infest,
    }

    pub fn part1() {
        let mut grid: Vec<Vec<Tile>> = read_file("input_files/day24.txt").split('\n').map(|s| s.chars().map(|ch| {
            match ch {
                '#' => Tile::Bug,
                '.' => Tile::Empty,
                _ => panic!("Unrecognised input: {}", ch),
            }
        }).collect()
        ).collect();

        let rows = grid.len();
        let cols = grid[0].len();

        // Adding a margin around our counts so I don't have to deal with edge cases.
        let mut neighbour_counts: Vec<Vec<usize>> = (0..(rows + 2)).map(|_| {
            vec![0; cols + 2]
        }).collect();

        // Calculate our initial neighbour counts
        for row in 0..rows {
            for col in 0..cols {
                match grid[row][col] {
                    Tile::Bug => {
                        let n_row = row + 1;
                        let n_col = col + 1;

                        neighbour_counts[n_row - 1][n_col] += 1;
                        neighbour_counts[n_row + 1][n_col] += 1;
                        neighbour_counts[n_row][n_col - 1] += 1;
                        neighbour_counts[n_row][n_col + 1] += 1;
                    },
                    _ => {}
                }
            }
        }

        // Iterate!

        let mut seen_grids: HashSet<Vec<Vec<Tile>>> = HashSet::new();
        seen_grids.insert(grid.clone());

        loop {
            let mut actions = Vec::new();

            for row in 0..rows {
                for col in 0..cols {
                    match grid[row][col] {
                        Tile::Bug => {
                            if neighbour_counts[row + 1][col + 1] != 1 {
                                actions.push(((row, col), Action::Die));
                            }
                        },
                        Tile::Empty => {
                            if neighbour_counts[row + 1][col + 1] == 1 || neighbour_counts[row + 1][col + 1] == 2 {
                                actions.push(((row, col), Action::Infest));
                            }
                        }
                    }
                }
            }

            for ((row, col), action) in actions {
                match action {
                    Action::Die => {
                        grid[row][col] = Tile::Empty;

                        let n_row = row + 1;
                        let n_col = col + 1;

                        neighbour_counts[n_row - 1][n_col] -= 1;
                        neighbour_counts[n_row + 1][n_col] -= 1;
                        neighbour_counts[n_row][n_col - 1] -= 1;
                        neighbour_counts[n_row][n_col + 1] -= 1;
                    },
                    Action::Infest => {
                        grid[row][col] = Tile::Bug;

                        let n_row = row + 1;
                        let n_col = col + 1;

                        neighbour_counts[n_row - 1][n_col] += 1;
                        neighbour_counts[n_row + 1][n_col] += 1;
                        neighbour_counts[n_row][n_col - 1] += 1;
                        neighbour_counts[n_row][n_col + 1] += 1;
                    }
                }
            }

            if seen_grids.contains(&grid) {
                let mut rating = 0u64;

                for row in 0..rows {
                    for col in 0..cols {
                        if grid[row][col] == Tile::Bug {
                            rating += 2_u64.pow((((row * rows) + col) as u32));
                        }

                        print!("{}", match grid[row][col] {
                            Tile::Bug => '#',
                            Tile::Empty => '.',
                        });
                    }

                    println!();
                }

                println!("Biodiversity rating: {}", rating);

                return;
            } else {
                seen_grids.insert(grid.clone());
            }
        }
    }

    struct NeighbourCounts {
        counts: HashMap<(i64, i64), usize>,
    }

    impl NeighbourCounts {
        fn new() -> NeighbourCounts {
            NeighbourCounts {
                counts: HashMap::new(),
            }
        }

        fn incr(&mut self, x: i64, y: i64, offset: i64) {
            let entry = self.counts.entry((x, y)).or_insert(0);
            *entry = (*entry as i64 + offset) as usize;
        }

        fn get(&self, x: usize, y: usize) -> usize {
            *self.counts.get(&(x as i64, y as i64)).unwrap_or(&0)
        }
    }

    pub fn part1_sparse() {
        let mut grid: Vec<Vec<Tile>> = read_file("input_files/day24.txt").split('\n').map(|s| s.chars().map(|ch| {
            match ch {
                '#' => Tile::Bug,
                '.' => Tile::Empty,
                _ => panic!("Unrecognised input: {}", ch),
            }
        }).collect()
        ).collect();

        let rows = grid.len();
        let cols = grid[0].len();

        let mut neighbour_counts = NeighbourCounts::new();

        // Calculate our initial neighbour counts
        for row in 0..rows {
            for col in 0..cols {
                match grid[row][col] {
                    Tile::Bug => {
                        let row = row as i64;
                        let col = col as i64;

                        neighbour_counts.incr(row - 1, col, 1);
                        neighbour_counts.incr(row + 1, col, 1);
                        neighbour_counts.incr(row, col - 1, 1);
                        neighbour_counts.incr(row, col + 1, 1);
                    },
                    _ => {}
                }
            }
        }

        // Iterate!
        let mut seen_grids: HashSet<Vec<Vec<Tile>>> = HashSet::new();
        seen_grids.insert(grid.clone());

        loop {
            let mut actions = Vec::new();

            for row in 0..rows {
                for col in 0..cols {
                    match grid[row][col] {
                        Tile::Bug => {
                            if neighbour_counts.get(row, col) != 1 {
                                actions.push(((row, col), Action::Die));
                            }
                        },
                        Tile::Empty => {
                            let neighbour_count = neighbour_counts.get(row, col);
                            if neighbour_count == 1 || neighbour_count == 2 {
                                actions.push(((row, col), Action::Infest));
                            }
                        }
                    }
                }
            }

            for ((row, col), action) in actions {
                match action {
                    Action::Die => {
                        grid[row][col] = Tile::Empty;

                        let row = row as i64;
                        let col = col as i64;

                        neighbour_counts.incr(row - 1, col, -1);
                        neighbour_counts.incr(row + 1, col, -1);
                        neighbour_counts.incr(row, col - 1, -1);
                        neighbour_counts.incr(row, col + 1, -1);
                    },
                    Action::Infest => {
                        grid[row][col] = Tile::Bug;

                        let row = row as i64;
                        let col = col as i64;

                        neighbour_counts.incr(row - 1, col, 1);
                        neighbour_counts.incr(row + 1, col, 1);
                        neighbour_counts.incr(row, col - 1, 1);
                        neighbour_counts.incr(row, col + 1, 1);
                    }
                }
            }

            if seen_grids.contains(&grid) {
                let mut rating = 0u64;

                for row in 0..rows {
                    for col in 0..cols {
                        if grid[row][col] == Tile::Bug {
                            rating += 2_u64.pow((((row * rows) + col) as u32));
                        }

                        print!("{}", match grid[row][col] {
                            Tile::Bug => '#',
                            Tile::Empty => '.',
                        });
                    }

                    println!();
                }

                println!("Biodiversity rating: {}", rating);

                return;
            } else {
                seen_grids.insert(grid.clone());
            }
        }
    }

    struct Neighbour {
        row: i64,
        col: i64,
        zoff: i64,
    }

    fn load_neighbour_map() -> HashMap<(i64, i64), Vec<Neighbour>> {
        let mut result = HashMap::new();

        for row in 0..5 {
            for col in 0..5 {
                let mut neighbours = Vec::new();

                // Boring cells
                if row > 0 {
                    neighbours.push(Neighbour { row: row - 1, col, zoff: 0 });
                }

                if col > 0 {
                    neighbours.push(Neighbour { row: row, col: col - 1, zoff: 0 });
                }

                if row < 4 {
                    neighbours.push(Neighbour { row: row + 1, col, zoff: 0 });
                }

                if col < 4 {
                    neighbours.push(Neighbour { row: row, col: col + 1, zoff: 0 });
                }

                // Outer edges
                if row == 0 {
                    neighbours.push(Neighbour { row: 1, col: 2, zoff: -1 });
                }

                if col == 0 {
                    neighbours.push(Neighbour { row: 2, col: 1, zoff: -1 });
                }

                if row == 4 {
                    neighbours.push(Neighbour { row: 3, col: 2, zoff: -1 });
                }

                if col == 4 {
                    neighbours.push(Neighbour { row: 2, col: 3, zoff: -1 });
                }

                // Inner edges

                // 8
                if row == 1 && col == 2 {
                    for subcol in 0..5 {
                        neighbours.push(Neighbour { row: 0, col: subcol, zoff: 1 });
                    }
                }

                // 12
                if row == 2 && col == 1 {
                    for subrow in 0..5 {
                        neighbours.push(Neighbour { row: subrow, col: 0, zoff: 1 });
                    }
                }

                // 18
                if row == 3 && col == 2 {
                    for subcol in 0..5 {
                        neighbours.push(Neighbour { row: 4, col: subcol, zoff: 1 });
                    }
                }

                // 14
                if row == 2 && col == 3 {
                    for subrow in 0..5 {
                        neighbours.push(Neighbour { row: subrow, col: 4, zoff: 1 });
                    }
                }

                // The square in the middle is never anybody's neighbour.
                result.insert((row, col), neighbours.into_iter().filter(|n| !(n.row == 2 && n.col == 2)).collect());
            }
        }

        result
    }

    #[derive(Debug)]
    struct NeighbourCounts3d {
        counts: HashMap<(i64, i64, i64), i64>,
    }

    impl NeighbourCounts3d {
        fn new() -> Self {
            Self {
                counts: HashMap::new(),
            }
        }

        fn incr(&mut self, x: i64, y: i64, z: i64, offset: i64) {
            assert!(!(x == 2 && y == 2));

            let entry = self.counts.entry((x, y, z)).or_insert(0);
            *entry = (*entry as i64 + offset) as i64;
        }

        fn get(&self, x: i64, y: i64, z: i64) -> i64 {
            assert!(!(x == 2 && y == 2));

            *self.counts.get(&(x as i64, y as i64, z)).unwrap_or(&0)
        }
    }


    pub fn part2() {
        let mut grid: HashMap<(i64, i64, i64), Tile> = HashMap::new();

        for (idx_row, row) in read_file("input_files/day24.txt").split('\n').enumerate() {
            for (idx_col, ch) in row.chars().enumerate() {
                let tile = match ch {
                    '#' => Tile::Bug,
                    '.' => Tile::Empty,
                    _ => panic!("Unrecognised input: {}", ch),
                };

                if tile == Tile::Bug {
                    grid.insert((idx_row as i64, idx_col as i64, 0), tile);
                }
            }
        }

        let neighbour_map = load_neighbour_map();

        let mut neighbour_counts = NeighbourCounts3d::new();

        // Calculate our initial neighbour counts
        for ((row, col, level), tile) in &grid {
            match tile {
                Tile::Bug => {
                    for neighbour in neighbour_map.get(&(*row, *col)).unwrap() {
                        let n_row = neighbour.row as i64;
                        let n_col = neighbour.col as i64;
                        let n_level = neighbour.zoff + level;

                        neighbour_counts.incr(n_row, n_col, n_level, 1);
                    }
                },
                _ => {}
            }
        }

        // Iterate!
        for _round in 0..200 {
            // Can't just walk the bugs... sometimes we resurrect a blank
            // square!  Need to come up with a set of positions to check.  The
            // set of all bug locations in 3d space plus the bugs neighbouring
            // cells.  Then check those.
            let positions_of_interest: HashSet<(i64, i64, i64)> = grid.keys().flat_map(|(row, col, level)| {
                let mut positions = vec!((*row, *col, *level));

                for neighbour in neighbour_map.get(&(*row, *col)).unwrap() {
                    let n_row = neighbour.row as i64;
                    let n_col = neighbour.col as i64;
                    let n_level = neighbour.zoff + level;

                    positions.push((n_row, n_col, n_level));
                }

                positions
            }).collect();

            // Accumulate actions
            let mut actions = Vec::new();
            for (row, col, level) in positions_of_interest {
                match grid.get(&(row, col, level)).unwrap_or(&Tile::Empty) {
                    Tile::Bug => {
                        if neighbour_counts.get(row, col, level) != 1 {
                            actions.push(((row, col, level), Action::Die));
                        }
                    },
                    Tile::Empty => {
                        let neighbour_count = neighbour_counts.get(row, col, level);
                        if neighbour_count == 1 || neighbour_count == 2 {
                            actions.push(((row, col, level), Action::Infest));
                        }
                    }
                }
            }

            for ((row, col, level), action) in actions {
                match action {
                    Action::Die => {
                        grid.remove(&(row, col, level));

                        for neighbour in neighbour_map.get(&(row, col)).unwrap() {
                            let n_row = neighbour.row as i64;
                            let n_col = neighbour.col as i64;
                            let n_level = neighbour.zoff + level;

                            neighbour_counts.incr(n_row, n_col, n_level, -1);
                        }
                    },
                    Action::Infest => {
                        grid.insert((row, col, level), Tile::Bug);

                        for neighbour in neighbour_map.get(&(row, col)).unwrap() {
                            let n_row = neighbour.row as i64;
                            let n_col = neighbour.col as i64;
                            let n_level = neighbour.zoff + level;

                            neighbour_counts.incr(n_row, n_col, n_level, 1);
                        }
                    }
                }
            }
        }

        println!("Bug count after 200 minutes: {}", grid.len());
    }
}


// Warning: This uses gigabytes of memory but eventually gets the answer.  Would
// have been better to locate and grab all the inventory items, move to the
// pressure sensor and then start dropping permutations.  Ah well!
mod day25 {
    use crate::shared::*;

    #[derive(Clone)]
    struct AsciiCode {
        intcode: IntCode,
    }

    impl AsciiCode {
        fn new(code: Vec<i64>) -> AsciiCode {
            AsciiCode {
                intcode: intcode::new(code, vec!(), Vec::new()),
            }
        }

        fn readline(&mut self) -> String {
            loop {
                self.intcode.step();


                if let Some(n) = self.intcode.output.last() {
                    if *n == 10 {
                        let result = self.intcode.output.iter().map(|&i| i as u8 as char).collect();
                        self.intcode.output.clear();
                        return result;
                    }
                }
            }
        }

        fn sendline(&mut self, s: &str) {
            for ch in s.chars() {
                self.intcode.input.insert(0, ch as i64);
            }
        }
    }


    #[derive(Debug)]
    struct Room {
        title: String,
        description: Vec<String>,
        exits: Vec<String>,
        items: Vec<String>,
    }

    fn parse_room(lines: &Vec<String>) -> Room {
        let mut result = Room {
            title: String::new(),
            description: Vec::new(),
            exits: Vec::new(),
            items: Vec::new(),
        };

        let mut iter = lines.iter();

        while let Some(line) = iter.next() {
            if line == "\n" {
                continue;
            }

            if line.starts_with("== ") && line.ends_with(" ==\n") {
                result.title = line.trim().to_string();
            } else if line == "Doors here lead:\n" {
                // Read exits
                while let Some(line) = iter.next() {
                    if line == "\n" {
                        break;
                    }

                    result.exits.push(line[2..line.len() - 1].to_string());
                }
            } else if line == "Items here:\n" {
                // Read items
                while let Some(line) = iter.next() {
                    if line == "\n" {
                        break;
                    }

                    result.items.push(line[2..line.len() - 1].to_string());
                }

            } else {
                result.description.push(line.to_string());
            }
        }

        result
    }

    #[derive(Clone, Hash, Eq, PartialEq, Debug)]
    struct State {
        room: String,
        items: BTreeSet<String>,
    }

    #[derive(Clone)]
    struct Path {
        state: State,
        droid: AsciiCode,
        steps: usize,
        trace: String,
        skip_prompts: usize,
        seen_states: HashSet<State>,
    }


    pub fn part1_manual() {
        let code: Vec<i64> = read_file("input_files/day25.txt").split(',').map(|s| s.parse().unwrap()).collect();

        let mut droid = AsciiCode::new(code);
        let stdin = std::io::stdin();

        loop {
            loop {
                let line = droid.readline();

                print!("{}", line);

                if line == "Command?\n" {
                    break;
                }
            }


            let mut command = String::new();
            stdin.read_line(&mut command).unwrap();

            droid.sendline(&command);
        }
    }


    pub fn part1() {
        let code: Vec<i64> = read_file("input_files/day25.txt").split(',').map(|s| s.parse().unwrap()).collect();

        let mut queue: VecDeque<Path> = VecDeque::new();

        queue.push_back(Path {
            state: State {
                room: "".to_owned(),
                items: BTreeSet::new(),
            },
            droid: AsciiCode::new(code),
            steps: 0,
            trace: String::new(),
            seen_states: HashSet::new(),
            skip_prompts: 0,
        });

        let mut messages = HashSet::new();

        while !queue.is_empty() {
            let mut path = queue.pop_front().unwrap();

            // Print loop
            let mut lines = Vec::new();
            let mut dead = false;
            loop {
                let line = path.droid.readline();

                if !messages.contains(&line) {
                    print!("{}", &line);
                    messages.insert(line.clone());
                }

                // if line == "== Pressure-Sensitive Floor ==\n" {
                //     dbg!(path.steps);
                //     dbg!(path.state);
                //     dbg!(path.trace);
                //     panic!();
                // }

                if line == "Command?\n" {
                    if path.skip_prompts == 0 {
                        break;
                    } else {
                        path.skip_prompts -= 1;
                    }
                }

                if (line == "The molten lava is way too hot! You melt!\n" ||
                    line == "You're launched into space! Bye!\n" ||
                    line == "It is suddenly completely dark! You are eaten by a Grue!\n" ||
                    line == "The giant electromagnet is stuck to you.  You can't move!!\n" ||
                    line == "A loud, robotic voice says \"Alert! Droids on this ship are heavier than the detected value!\" and you are ejected back to the checkpoint.\n" ||
                    line == "You take the infinite loop.\n") {
                    dead = true;
                    break;
                }

                lines.push(line);
            }

            if dead {
                continue;
            }

            let room = parse_room(&lines);

            let mut this_state = path.state.clone();
            this_state.room = room.title.clone();

            if path.seen_states.contains(&this_state) {
                continue;
            }

            path.seen_states.insert(this_state);

            // Take every possible combination of items
            let mask_upper = 2_usize.pow(room.items.len() as u32);
            for item_mask in 0..mask_upper {
                let mut droid_with_taken_items = path.droid.clone();

                let mut took_items = Vec::new();

                for (idx, item) in room.items.iter().enumerate() {
                    // // Banned items!  No touchy!
                    // if item == "escape pod" || item == "molten lava" || item == "infinite loop" || item == "photons" {
                    //     continue;
                    // }

                    if (item_mask & (1 << idx)) != 0 {
                        droid_with_taken_items.sendline(&format!("take {}\n", item));

                        took_items.push(item.clone());
                    }
                }

                // Now take every possible direction with this combination of items
                for direction in &room.exits {
                    let mut moved_droid = droid_with_taken_items.clone();
                    moved_droid.sendline(&format!("{}\n", direction));

                    let mut new_path = path.clone();

                    // What we learned: the maze isn't based on a grid.  A
                    // sequence of steps like NESW doesn't necessarily land you
                    // back to where you started.  Use the names of the room to
                    // identify places we've already visited instead of
                    // coordinates.

                    // match direction.as_str() {
                    //     "north" => { new_path.state.y += 1 },
                    //     "south" => { new_path.state.y -= 1 },
                    //     "east" => { new_path.state.x += 1 },
                    //     "west" => { new_path.state.x -= 1 },
                    //     _ => { panic!("Weird direction: {}", direction) },
                    // }

                    for item in &took_items {
                        new_path.state.items.insert(item.clone());
                    }

                    new_path.skip_prompts = took_items.len();
                    new_path.steps += 1;
                    new_path.trace.push(direction.chars().nth(0).unwrap());

                    new_path.droid = moved_droid;

                    queue.push_back(new_path);
                }
            }
        }
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
        day22::part1_refired();
        day22::part2_refired();

        day23::part1();
        day23::part2();

        day24::part1();
        day24::part1_sparse();
        day24::part2();

        day25::part1_manual();
        day25::part1();
    }
}
