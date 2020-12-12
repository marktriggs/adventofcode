// (cd ../; cargo run --release)

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(dead_code)]
#![feature(iterator_fold_self)]

#[macro_use]
extern crate lazy_static;
extern crate regex;

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
    pub use std::str;
    pub use std::sync::{Arc, Mutex};

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
        let amounts: Vec<usize> = input_lines("input_files/day1.txt")
            .map(|s| s.parse().unwrap())
            .collect();

        for i in 0..amounts.len() {
            for j in (i + 1)..amounts.len() {
                if (amounts[i] + amounts[j]) == 2020 {
                    println!(
                        "{} * {} = {}",
                        amounts[i],
                        amounts[j],
                        amounts[i] * amounts[j]
                    );
                }
            }
        }
    }

    pub fn part2() {
        let amounts: Vec<usize> = input_lines("input_files/day1.txt")
            .map(|s| s.parse().unwrap())
            .collect();

        for i in 0..amounts.len() {
            for j in (i + 1)..amounts.len() {
                for k in (j + 1)..amounts.len() {
                    if (amounts[i] + amounts[j] + amounts[k]) == 2020 {
                        println!(
                            "{} * {} * {} = {}",
                            amounts[i],
                            amounts[j],
                            amounts[k],
                            amounts[i] * amounts[j] * amounts[k]
                        );
                    }
                }
            }
        }
    }
}

mod day2 {
    use crate::shared::*;

    #[derive(Debug)]
    struct Password {
        min: usize,
        max: usize,
        letter: char,
        candidate_password: String,
    }

    impl Password {
        fn from_line(line: String) -> Password {
            let pattern = Regex::new(r"^([0-9]+)-([0-9]+) (.): (.+)$").unwrap();

            if let Some(cap) = pattern.captures(&line) {
                return Password {
                    min: cap[1].parse().unwrap(),
                    max: cap[2].parse().unwrap(),
                    letter: cap[3].parse().unwrap(),
                    candidate_password: cap[4].to_owned(),
                };
            }

            panic!("Password parse error for {}", line);
        }

        fn is_valid(&self) -> bool {
            let c = self
                .candidate_password
                .chars()
                .filter(|&ch| ch == self.letter)
                .count();

            (c >= self.min && c <= self.max)
        }

        fn is_valid_new_policy(&self) -> bool {
            let chars: Vec<char> = self.candidate_password.chars().collect();

            (chars[self.min - 1] == self.letter) ^ (chars[self.max - 1] == self.letter)
        }
    }

    pub fn part1() {
        let candidate_passwords: Vec<Password> = input_lines("input_files/day2.txt")
            .map(Password::from_line)
            .collect();

        println!(
            "Valid password count: {}",
            candidate_passwords.iter().filter(|p| p.is_valid()).count()
        )
    }

    pub fn part2() {
        let candidate_passwords: Vec<Password> = input_lines("input_files/day2.txt")
            .map(Password::from_line)
            .collect();

        println!(
            "Valid password count: {}",
            candidate_passwords
                .iter()
                .filter(|p| p.is_valid_new_policy())
                .count()
        )
    }
}

mod day3 {
    use crate::shared::*;

    struct Map {
        width: usize,
        height: usize,
        grid: Vec<Vec<char>>,
    }

    impl Map {
        fn from_input(path: &str) -> Map {
            let grid: Vec<Vec<char>> = input_lines(path)
                .map(|line| line.chars().collect())
                .collect();

            Map {
                width: grid[0].len(),
                height: grid.len(),
                grid,
            }
        }

        fn is_tree(&self, x: usize, y: usize) -> bool {
            self.grid[y][x] == '#'
        }
    }

    fn trees_hit(map: &Map, right: usize, down: usize) -> usize {
        let mut x = 0;
        let mut y = 0;
        let mut trees_hit = 0;

        while y < map.height {
            if map.is_tree(x, y) {
                trees_hit += 1;
            }

            x = (x + right) % map.width;
            y += down;
        }

        trees_hit
    }

    pub fn part1() {
        let map = Map::from_input("input_files/day3.txt");

        println!(
            "Whacked into {} trees on our way down",
            trees_hit(&map, 3, 1)
        );
    }

    pub fn part2() {
        let map = Map::from_input("input_files/day3.txt");

        println!(
            "{}",
            trees_hit(&map, 1, 1)
                * trees_hit(&map, 3, 1)
                * trees_hit(&map, 5, 1)
                * trees_hit(&map, 7, 1)
                * trees_hit(&map, 1, 2)
        );
    }
}

mod day4 {
    use crate::shared::*;

    const REQUIRED_PASSPORT_FIELDS: &[&str] = &[
        "byr", // (Birth Year)
        "iyr", // (Issue Year)
        "eyr", // (Expiration Year)
        "hgt", // (Height)
        "hcl", // (Hair Color)
        "ecl", // (Eye Color)
        "pid", // (Passport ID)
               // "cid", // (Country ID) - ignored due to trickery
    ];

    type PassportRules = HashMap<&'static str, fn(&str) -> bool>;

    fn validation_rules() -> PassportRules {
        let mut rules: PassportRules = HashMap::new();

        rules.insert("byr", |s| {
            if let Ok(n) = s.parse::<usize>() {
                n >= 1920 && n <= 2002
            } else {
                false
            }
        });

        rules.insert("iyr", |s| {
            if let Ok(n) = s.parse::<usize>() {
                n >= 2010 && n <= 2020
            } else {
                false
            }
        });

        rules.insert("eyr", |s| {
            if let Ok(n) = s.parse::<usize>() {
                n >= 2020 && n <= 2030
            } else {
                false
            }
        });

        rules.insert("hgt", |s| {
            if let Ok(n) = s
                .trim_end_matches(|c: char| !c.is_numeric())
                .parse::<usize>()
            {
                if s.ends_with("cm") {
                    return n >= 150 && n <= 193;
                } else if s.ends_with("in") {
                    return n >= 59 && n <= 76;
                }
            }

            false
        });

        rules.insert("hcl", |s| {
            Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(s)
        });

        rules.insert("ecl", |s| {
            Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$")
                .unwrap()
                .is_match(s)
        });

        rules.insert("pid", |s| Regex::new(r"^[0-9]{9}$").unwrap().is_match(s));

        rules
    }

    struct Passport {
        values: HashMap<String, String>,
    }

    impl Passport {
        fn from_lines(lines: &[String]) -> Passport {
            assert!(!lines.is_empty());

            let mut result = Passport {
                values: HashMap::new(),
            };

            for line in lines {
                for chunk in line.split(' ') {
                    let bits: Vec<&str> = chunk.split(':').collect();
                    result.values.insert(bits[0].to_owned(), bits[1].to_owned());
                }
            }

            result
        }

        fn is_valid(&self) -> bool {
            for &field in REQUIRED_PASSPORT_FIELDS {
                if (!self.values.contains_key(field)) {
                    return false;
                }
            }

            true
        }

        fn is_valid_by_rules(&self, rules: &PassportRules) -> bool {
            for &field in REQUIRED_PASSPORT_FIELDS {
                if (!self.values.contains_key(field)) {
                    return false;
                }

                let rule = rules.get(field).unwrap();
                let field_value = self.values.get(field).unwrap();

                if (!rule(field_value)) {
                    return false;
                }
            }

            true
        }
    }

    pub fn part1() {
        let lines: Vec<String> = input_lines("input_files/day4.txt").collect();

        println!(
            "There are {} valid passports",
            lines
                .split(|s| s.is_empty())
                .map(|passport_lines| Passport::from_lines(passport_lines))
                .filter(|passport| passport.is_valid())
                .count()
        );
    }

    pub fn part2() {
        let lines: Vec<String> = input_lines("input_files/day4.txt").collect();

        let rules = validation_rules();

        println!(
            "There are {} valid passports",
            lines
                .split(|s| s.is_empty())
                .map(|passport_lines| Passport::from_lines(passport_lines))
                .filter(|passport| passport.is_valid_by_rules(&rules))
                .count()
        );
    }
}

mod day5 {
    use crate::shared::*;

    #[derive(Debug)]
    enum WhichHalf {
        Lower,
        Upper,
    }

    #[derive(PartialEq, Eq, Debug, Hash)]
    pub struct Seat {
        row: usize,
        column: usize,
    }

    impl Seat {
        fn id(&self) -> usize {
            (self.row * 8) + self.column
        }
    }

    fn search_range(lower_inclusive: usize, upper_inclusive: usize, input: &[WhichHalf]) -> usize {
        let mut low = lower_inclusive;
        let mut high = upper_inclusive;

        for half in input {
            match half {
                WhichHalf::Lower => {
                    high = low + ((high - low) / 2);
                }
                WhichHalf::Upper => {
                    // div by 2 rounding up
                    low = low + ((high - low + 1) / 2);
                }
            }
        }

        assert_eq!(low, high);

        low
    }

    pub fn calculate_seat(instructions: &str) -> Seat {
        let instructions: Vec<WhichHalf> = instructions
            .chars()
            .map(|ch| match ch {
                'F' | 'L' => WhichHalf::Lower,
                'B' | 'R' => WhichHalf::Upper,
                _ => panic!("Bad mojo: {}", ch),
            })
            .collect();

        let (row_instructions, column_instructions) = instructions.split_at(7);

        let row = search_range(0, 127, row_instructions);
        let column = search_range(0, 7, column_instructions);

        Seat { row, column }
    }

    pub fn part1() {
        let max_seat_id = input_lines("input_files/day5.txt")
            .map(|line| {
                let seat = calculate_seat(&line);
                seat.id()
            })
            .max()
            .unwrap();

        println!("Maximum seat ID is {}", max_seat_id);
    }

    pub fn part2() {
        let mut all_seats: HashSet<Seat> = HashSet::new();

        input_lines("input_files/day5.txt").for_each(|line| {
            let seat = calculate_seat(&line);
            all_seats.insert(seat);
        });

        // Printed it out and eyeballed it :)
        for row in 0..=127 {
            for column in 0..=7 {
                if (all_seats.contains(&Seat { row, column })) {
                    print!("X");
                } else {
                    print!("?")
                }
            }
            println!();
        }

        for row in 1..=126 {
            for column in 0..=7 {
                if (!all_seats.contains(&Seat { row, column })
                    && all_seats.contains(&Seat {
                        row: row - 1,
                        column,
                    })
                    && all_seats.contains(&Seat {
                        row: row + 1,
                        column,
                    }))
                {
                    let seat = Seat { row, column };

                    println!("Free seat: {:?} with id {}", seat, seat.id());
                }
            }
        }
    }
}

mod day6 {
    use crate::shared::*;

    pub fn part1() {
        let lines: Vec<String> = input_lines("input_files/day6.txt").collect();

        let sum: usize = lines
            .split(|s| s.is_empty())
            .map(|group_lines| {
                let yes_set = group_lines.join("").chars().collect::<HashSet<char>>();

                yes_set.len()
            })
            .sum();

        println!("Sum of questions where anyone said yes: {}", sum);
    }

    pub fn part2() {
        let lines: Vec<String> = input_lines("input_files/day6.txt").collect();

        let sum: usize = lines
            .split(|s| s.is_empty())
            .map(|group_lines| {
                let yes_set = group_lines
                    .iter()
                    .map(|line| line.chars().collect::<HashSet<char>>())
                    .fold_first(|s1, s2| s1.intersection(&s2).cloned().collect())
                    .unwrap();

                yes_set.len()
            })
            .sum();

        println!("Sum of questions where everyone said yes: {}", sum);
    }
}

mod day7 {
    use crate::shared::*;

    #[derive(Debug, Clone)]
    struct BagRule {
        bag_type: String,
        contains: Vec<BagQuantity>,
    }

    #[derive(Debug, Clone)]
    struct BagQuantity {
        count: usize,
        bag_type: String,
    }

    #[derive(Debug, Clone)]
    struct BagRules {
        rules: Vec<BagRule>,
        rules_by_type: HashMap<String, usize>,
        rules_by_containee_type: HashMap<String, Vec<usize>>,
    }

    impl BagRules {
        fn from_lines(lines: Vec<String>) -> BagRules {
            let mut result = BagRules {
                rules: Vec::with_capacity(lines.len()),
                rules_by_type: HashMap::new(),
                rules_by_containee_type: HashMap::new(),
            };

            for line in lines {
                let (container, containee_descriptions) =
                    line.split(" bags contain ").collect_tuple().unwrap();

                let mut containees: Vec<BagQuantity> = Vec::new();

                if containee_descriptions != "no other bags." {
                    let descriptions: Vec<&str> = containee_descriptions
                        .trim_end_matches('.')
                        .split(", ")
                        .collect();

                    for d in descriptions {
                        let bits: Vec<&str> = d.split(' ').collect();

                        containees.push(BagQuantity {
                            count: bits[0].parse().unwrap(),
                            bag_type: format!("{} {}", bits[1], bits[2]),
                        });
                    }
                }

                result.rules.push(BagRule {
                    bag_type: container.to_owned(),
                    contains: containees.clone(),
                });

                result
                    .rules_by_type
                    .insert(container.to_owned(), result.rules.len() - 1);

                for c in containees {
                    let entry = result
                        .rules_by_containee_type
                        .entry(c.bag_type)
                        .or_insert_with(Vec::new);
                    entry.push(result.rules.len() - 1);
                }
            }

            result
        }

        fn rule_for_type(&self, bag_type: &str) -> Option<BagRule> {
            if let Some(&idx) = self.rules_by_type.get(bag_type) {
                Some(self.rules[idx].clone())
            } else {
                None
            }
        }

        fn bags_that_can_contain(&self, bag_type: &str) -> Vec<&str> {
            if let Some(rule_indexes) = self.rules_by_containee_type.get(bag_type) {
                return rule_indexes
                    .iter()
                    .map(|&rule_idx| self.rules[rule_idx].bag_type.as_str())
                    .collect();
            }

            Vec::new()
        }
    }

    pub fn part1() {
        let lines: Vec<String> = input_lines("input_files/day7.txt").collect();
        let bag_rules = BagRules::from_lines(lines);

        let mut possible_outcomes: HashSet<String> = HashSet::new();
        possible_outcomes.insert("shiny gold".to_owned());

        loop {
            let mut new_outcomes: HashSet<String> = HashSet::new();

            for existing_bag in &possible_outcomes {
                let next_bags = bag_rules.bags_that_can_contain(&existing_bag);

                for next_bag in next_bags {
                    new_outcomes.insert(next_bag.to_owned());
                }
            }

            if new_outcomes.difference(&possible_outcomes).count() == 0 {
                // No further rule expansion is possible.
                break;
            }

            possible_outcomes.extend(new_outcomes);
        }

        println!(
            "There were {} possible options",
            possible_outcomes.len() - 1
        );
    }

    pub fn part2() {
        let lines: Vec<String> = input_lines("input_files/day7.txt").collect();
        let bag_rules = BagRules::from_lines(lines);

        let mut queue: VecDeque<BagQuantity> = VecDeque::new();

        let mut final_count = 0;

        queue.push_back(BagQuantity {
            count: 1,
            bag_type: "shiny gold".to_owned(),
        });

        while !queue.is_empty() {
            let bag_quantity = queue.pop_front().unwrap();

            if let Some(rule) = bag_rules.rule_for_type(&bag_quantity.bag_type) {
                final_count += bag_quantity.count;

                if !rule.contains.is_empty() {
                    // Multiply out the contained bags
                    for contained in &rule.contains {
                        queue.push_back(BagQuantity {
                            count: contained.count * bag_quantity.count,
                            bag_type: contained.bag_type.clone(),
                        });
                    }
                }
            } else {
                panic!("Rule not found?!");
            }
        }

        println!("Needed {} additional bags", final_count - 1);
    }
}

mod day8 {
    use crate::shared::*;

    #[derive(Clone)]
    enum Instruction {
        Nop(i64),
        Acc(i64),
        Jmp(i64),
    }

    #[derive(Eq, PartialEq)]
    enum State {
        Running,
        FinishedSuccess,
        FinishedLooped,
    }

    struct OKComputer {
        acc: i64,
        pc: usize,
        instructions: Vec<Instruction>,
        pc_history: HashSet<usize>,
        state: State,
    }

    impl OKComputer {
        fn parse_program(lines: &[String]) -> Vec<Instruction> {
            lines
                .iter()
                .map(|line| {
                    let (instruction_code, argstr) = line.split(' ').collect_tuple().unwrap();
                    let arg: i64 = argstr.parse().unwrap();

                    match instruction_code {
                        "nop" => Instruction::Nop(arg),
                        "acc" => Instruction::Acc(arg),
                        "jmp" => Instruction::Jmp(arg),
                        _ => panic!("Parse error: {}", line),
                    }
                })
                .collect()
        }

        fn load_program(program: Vec<Instruction>) -> OKComputer {
            OKComputer {
                acc: 0,
                pc: 0,
                pc_history: HashSet::new(),
                state: State::Running,
                instructions: program,
            }
        }

        fn run(&mut self) {
            loop {
                if self.pc_history.contains(&self.pc) {
                    // We've looped
                    self.state = State::FinishedLooped;
                    break;
                }

                self.pc_history.insert(self.pc);

                match self.pc.cmp(&self.instructions.len()) {
                    Ordering::Less => {
                        match *self
                            .instructions
                            .get(self.pc)
                            .expect("instruction fetch invalid")
                        {
                            Instruction::Nop(_) => {
                                self.pc += 1;
                            }
                            Instruction::Acc(arg) => {
                                self.acc += arg;
                                self.pc += 1;
                            }
                            Instruction::Jmp(arg) => {
                                // A fair bit of legwork here just because I didn't want to make self.pc signed.
                                // Maybe not worth it.  We'll see.
                                if arg < 0 {
                                    if arg.abs() as usize > self.pc {
                                        panic!("Jump instruction would set PC negative");
                                    } else {
                                        self.pc -= arg.abs() as usize;
                                    }
                                } else {
                                    // NOTE: We might still go past the end of our program.  Let our next turn
                                    // around the loop pick that up for now.
                                    self.pc += arg as usize;
                                }
                            }
                        }
                    }
                    Ordering::Equal => {
                        // One past the end of the program is a win
                        self.state = State::FinishedSuccess;
                        break;
                    }
                    Ordering::Greater => {
                        panic!("Ran off the end of the program!");
                    }
                }
            }
        }
    }

    pub fn part1() {
        let lines: Vec<String> = input_lines("input_files/day8.txt").collect();

        let mut ok_computer = OKComputer::load_program(OKComputer::parse_program(&lines));

        ok_computer.run();
        println!("Final accumulator value was: {}", ok_computer.acc);
    }

    pub fn part2() {
        let lines: Vec<String> = input_lines("input_files/day8.txt").collect();

        let template_program = OKComputer::parse_program(&lines);

        for i in 0..template_program.len() {
            let mut modified_program = template_program.clone();

            match *template_program.get(i).expect("offset invalid") {
                Instruction::Nop(arg) => {
                    // Try a jump
                    modified_program[i] = Instruction::Jmp(arg);
                }
                Instruction::Jmp(arg) => {
                    // Try a Nop
                    modified_program[i] = Instruction::Nop(arg);
                }
                _ => {
                    // Boring
                    continue;
                }
            }

            // MACHINE LEARNING!
            let mut ok_computer = OKComputer::load_program(modified_program);

            ok_computer.run();
            if ok_computer.state == State::FinishedSuccess {
                println!(
                    "Fixed program with instruction {}!  Final accumulator value was: {}",
                    i, ok_computer.acc
                );
                return;
            }
        }

        panic!("It's hopeless!");
    }
}

mod day9 {
    use crate::shared::*;

    struct OrderPreservingSet {
        ordered_elements: VecDeque<usize>,
        element_set: HashSet<usize>,
        size: usize,
    }

    impl OrderPreservingSet {
        fn new(size: usize) -> OrderPreservingSet {
            OrderPreservingSet {
                ordered_elements: VecDeque::new(),
                element_set: HashSet::new(),
                size,
            }
        }

        fn push(&mut self, elt: usize) {
            self.ordered_elements.push_back(elt);
            self.element_set.insert(elt);

            while self.ordered_elements.len() > self.size {
                let discarded = self.ordered_elements.pop_front().unwrap();
                self.element_set.remove(&discarded);
            }
        }

        fn contains(&self, elt: usize) -> bool {
            self.element_set.contains(&elt)
        }

        fn iter(&self) -> impl Iterator<Item = &usize> {
            self.element_set.iter()
        }
    }

    pub fn part1() {
        let mut numbers: VecDeque<usize> = input_lines("input_files/day9.txt")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let mut set = OrderPreservingSet::new(25);

        // Pre-load our set with the first 25
        for _ in 0..25 {
            set.push(numbers.pop_front().unwrap());
        }

        // Start looking for invalid numbers
        while !numbers.is_empty() {
            let n = numbers.pop_front().unwrap();

            let valid = set.iter().any(|&x| {
                if x >= n {
                    // This can't be part of our sum
                    return false;
                }

                if set.contains(n - x) {
                    // That's good enough for us!
                    return true;
                }

                false
            });

            if valid {
                set.push(n);
            } else {
                println!("Found our invalid number: {}", n);
                break;
            }
        }
    }

    // Found our invalid number: 18272118
    pub fn part2() {
        let numbers: Vec<usize> = input_lines("input_files/day9.txt")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let target = 18272118;

        for i in 0..numbers.len() {
            let mut subtotal = 0;
            let mut offset = 0;

            while subtotal < target && (i + offset) < numbers.len() {
                subtotal += numbers[i + offset];
                offset += 1;
            }

            if subtotal == target {
                println!("Found our run between {} and {}", i, i + offset);

                let smallest = numbers[i..(i + offset)].iter().min().unwrap();
                let largest = numbers[i..(i + offset)].iter().max().unwrap();

                println!("Encryption weakness: {}", smallest + largest);

                break;
            }
        }
    }
}

mod day10 {
    use crate::shared::*;

    fn find_ordering(
        last_voltage: i64,
        remaining_adapters: Vec<i64>,
        ordering: Vec<i64>,
    ) -> Option<Vec<i64>> {
        if remaining_adapters.is_empty() {
            return Some(ordering);
        }

        for a in &remaining_adapters {
            let difference = a - last_voltage;

            if difference >= 0 && difference <= 3 {
                // Remove the candidate adapter.  This works because adapter outputs are unique.
                let next_remaining: Vec<i64> = remaining_adapters
                    .iter()
                    .filter(|&r| r != a)
                    .copied()
                    .collect();
                let mut next_ordering = ordering.clone();
                next_ordering.push(*a);

                if let Some(result) = find_ordering(*a, next_remaining, next_ordering) {
                    return Some(result);
                }
            }
        }

        None
    }

    pub fn part1() {
        let mut adapters: Vec<i64> = input_lines("input_files/day10.txt")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        // Well... I guess they have to go in this order?
        adapters.sort();

        let device_joltage = adapters.iter().max().unwrap() + 3;

        if let Some(adapter_ordering) = find_ordering(0, adapters, Vec::new()) {
            let mut joltages = Vec::with_capacity(adapter_ordering.len() + 2);
            joltages.push(0);
            joltages.extend(adapter_ordering);
            joltages.push(device_joltage);

            let mut frequencies: HashMap<i64, i64> = HashMap::new();

            for i in 1..joltages.len() {
                let difference = joltages[i] - joltages[i - 1];
                let e = frequencies.entry(difference).or_insert(0);
                *e += 1;
            }

            dbg!(frequencies);
        }
    }

    #[derive(Hash, Eq, PartialEq, Clone)]
    struct Key(i64, Vec<i64>);

    fn count_orderings(
        last_voltage: i64,
        remaining_adapters: Vec<i64>,
        target_voltage: i64,
        cache: Rc<RefCell<HashMap<Key, usize>>>,
    ) -> usize {
        let key: Key = Key(last_voltage, remaining_adapters.clone());

        let cache_handle = cache.borrow();
        if cache_handle.contains_key(&key) {
            // Precomputed!
            return *cache_handle.get(&key).unwrap();
        }
        drop(cache_handle);

        let result = if last_voltage > target_voltage {
            0
        } else if (target_voltage - last_voltage) <= 3 {
            if remaining_adapters.is_empty() {
                1
            } else {
                1 + count_orderings(
                    remaining_adapters[0],
                    remaining_adapters.iter().skip(1).copied().collect(),
                    target_voltage,
                    cache.clone(),
                ) + count_orderings(
                    last_voltage,
                    remaining_adapters.iter().skip(1).copied().collect(),
                    target_voltage,
                    cache.clone(),
                )
            }
        } else if remaining_adapters.is_empty() {
            0
        } else {
            let a = remaining_adapters[0];
            let rest: Vec<i64> = remaining_adapters.iter().skip(1).copied().collect();

            if (a - last_voltage) > 3 {
                // Can't bridge this gap with this (or any subsequent) adapter
                0
            } else {
                count_orderings(a, rest.clone(), target_voltage, cache.clone())
                    + count_orderings(last_voltage, rest, target_voltage, cache.clone())
            }
        };

        let mut cache_handle = cache.borrow_mut();
        cache_handle.insert(key, result);
        drop(cache_handle);

        result
    }

    pub fn part2() {
        let mut adapters: Vec<i64> = input_lines("input_files/day10.txt")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        // Still need to be strictly increasing.
        adapters.sort();

        let device_joltage = adapters.iter().max().unwrap() + 3;

        dbg!(count_orderings(
            0,
            adapters,
            device_joltage,
            Rc::new(RefCell::new(HashMap::new()))
        ));
    }
}

mod day11 {
    use crate::shared::*;

    #[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
    enum Tile {
        Floor,
        Empty,
        Occupied,
    }

    #[derive(Eq, PartialEq, Clone, Hash, Debug)]
    struct Grid {
        width: usize,
        height: usize,
        rows: Vec<Vec<Tile>>,
    }

    impl Grid {
        fn from_lines(lines: Vec<String>) -> Grid {
            let rows: Vec<Vec<Tile>> = lines
                .iter()
                .map(|line| {
                    line.chars()
                        .map(|sym| match sym {
                            '#' => Tile::Occupied,
                            'L' => Tile::Empty,
                            '.' => Tile::Floor,
                            _ => panic!("Unexpected symbol during parse: {}", sym),
                        })
                        .collect()
                })
                .collect();

            assert!(!rows.is_empty());

            Grid {
                width: rows[0].len(),
                height: rows.len(),
                rows,
            }
        }

        fn tile_at(&self, row: i64, col: i64) -> Tile {
            if !self.is_in_bounds(row, col) {
                // Out of bounds
                return Tile::Empty;
            }

            self.rows[row as usize][col as usize]
        }

        fn set(&mut self, row: i64, col: i64, tile: Tile) {
            if !self.is_in_bounds(row, col) {
                // Out of bounds
                return;
            }

            self.rows[row as usize][col as usize] = tile;
        }

        fn is_in_bounds(&self, row: i64, col: i64) -> bool {
            !(row < 0 || col < 0 || row as usize >= self.height || col as usize >= self.width)
        }

        fn adjacent_tiles(&self, row: i64, col: i64) -> [Tile; 8] {
            [
                self.tile_at(row - 1, col - 1),
                self.tile_at(row - 1, col),
                self.tile_at(row - 1, col + 1),
                self.tile_at(row, col - 1),
                self.tile_at(row, col + 1),
                self.tile_at(row + 1, col - 1),
                self.tile_at(row + 1, col),
                self.tile_at(row + 1, col + 1),
            ]
        }

        fn next_grid(&self) -> Grid {
            let mut result = self.clone();

            for row in 0..self.height as i64 {
                for col in 0..self.width as i64 {
                    let adjacent = self.adjacent_tiles(row, col);

                    if self.tile_at(row, col) == Tile::Empty
                        && !adjacent.iter().any(|&t| t == Tile::Occupied)
                    {
                        result.set(row, col, Tile::Occupied);
                    } else if self.tile_at(row, col) == Tile::Occupied
                        && adjacent.iter().filter(|&&t| t == Tile::Occupied).count() >= 4
                    {
                        result.set(row, col, Tile::Empty);
                    } else {
                        // No change
                    }
                }
            }

            result
        }

        fn adjacent_tiles_pt2(&self, row: i64, col: i64) -> Vec<Tile> {
            // (row, col)
            let directions = [
                (-1, 0),  // north
                (1, 0),   // south
                (0, -1),  // west
                (0, 1),   // east
                (-1, -1), // north west
                (-1, 1),  // north east
                (1, -1),  // south west
                (1, 1),   // south east
            ];

            directions
                .iter()
                .map(|direction| {
                    let mut r = row;
                    let mut c = col;

                    loop {
                        r += direction.0;
                        c += direction.1;

                        let tile = self.tile_at(r, c);

                        if tile != Tile::Floor || !self.is_in_bounds(r, c) {
                            break tile;
                        }
                    }
                })
                .collect()
        }

        fn next_grid_pt2(&self) -> Grid {
            let mut result = self.clone();

            for row in 0..self.height as i64 {
                for col in 0..self.width as i64 {
                    let adjacent = self.adjacent_tiles_pt2(row, col);

                    if self.tile_at(row, col) == Tile::Empty
                        && !adjacent.iter().any(|&t| t == Tile::Occupied)
                    {
                        result.set(row, col, Tile::Occupied);
                    } else if self.tile_at(row, col) == Tile::Occupied
                        && adjacent.iter().filter(|&&t| t == Tile::Occupied).count() >= 5
                    {
                        result.set(row, col, Tile::Empty);
                    } else {
                        // No change
                    }
                }
            }

            result
        }

        fn count_tiles(&self, tile: Tile) -> usize {
            self.rows
                .iter()
                .map(|row| row.iter().filter(|&&t| t == tile).count())
                .sum()
        }
    }

    pub fn part1() {
        let mut grid = Grid::from_lines(input_lines("input_files/day11.txt").collect());

        loop {
            let next_grid = grid.next_grid();

            if next_grid == grid {
                // We've hit a stable state
                break;
            }

            grid = next_grid;
        }

        println!(
            "There are {} occupied seats",
            grid.count_tiles(Tile::Occupied)
        );
    }

    pub fn part2() {
        let mut grid = Grid::from_lines(input_lines("input_files/day11.txt").collect());

        loop {
            let next_grid = grid.next_grid_pt2();

            if next_grid == grid {
                // We've hit a stable state
                break;
            }

            grid = next_grid;
        }

        println!(
            "There are {} occupied seats",
            grid.count_tiles(Tile::Occupied)
        );
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
