// (cd ../; cargo run --release)

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(dead_code)]
#![feature(linked_list_cursors)]

#[macro_use]
extern crate lazy_static;
extern crate regex;

mod shared {
    pub use regex::Regex;

    // pub use intcode::{self, IntCode};
    pub use std::cell::RefCell;
    pub use std::cmp::{self, Ordering};
    pub use std::collections::linked_list::CursorMut;
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
                    .reduce(|s1, s2| s1.intersection(&s2).cloned().collect())
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

mod day12 {
    use crate::shared::*;

    pub fn part1() {
        #[derive(Debug)]
        struct Ferry {
            rotation: i64,
            xpos: i64,
            ypos: i64,
        }

        let mut ferry = Ferry {
            rotation: 90, // East
            xpos: 0,
            ypos: 0,
        };

        for instruction in input_lines("input_files/day12.txt") {
            let (mode, n) = instruction.split_at(1);
            let n: i64 = n.parse().unwrap();

            match mode {
                "N" => {
                    ferry.ypos -= n;
                }
                "E" => {
                    ferry.xpos += n;
                }
                "S" => {
                    ferry.ypos += n;
                }
                "W" => {
                    ferry.xpos -= n;
                }
                "L" => {
                    ferry.rotation -= n;
                    if ferry.rotation < 0 {
                        ferry.rotation += 360;
                    }
                }
                "R" => {
                    ferry.rotation += n;
                    if ferry.rotation >= 360 {
                        ferry.rotation -= 360;
                    }
                }
                "F" => match ferry.rotation {
                    0 => {
                        ferry.ypos -= n;
                    }
                    90 => {
                        ferry.xpos += n;
                    }
                    180 => {
                        ferry.ypos += n;
                    }
                    270 => {
                        ferry.xpos -= n;
                    }
                    _ => {
                        panic!("unrecognised rotation: {}", ferry.rotation);
                    }
                },
                _ => panic!("unrecognised instruction"),
            }
        }

        println!(
            "Manhattan distance: {}",
            ferry.xpos.abs() + ferry.ypos.abs()
        );
    }

    // 28479: too low
    pub fn part2() {
        #[derive(Debug)]
        struct Ferry {
            xpos: i64,
            ypos: i64,
        }

        #[derive(Debug)]
        struct Waypoint {
            xpos: i64,
            ypos: i64,
        }

        let mut ferry = Ferry { xpos: 0, ypos: 0 };

        let mut waypoint = Waypoint { xpos: 10, ypos: -1 };

        impl Waypoint {
            fn unit_direction(&self) -> (i64, i64) {
                let xdir = if self.xpos == 0 {
                    1
                } else {
                    self.xpos / self.xpos.abs()
                };

                let ydir = if self.ypos == 0 {
                    1
                } else {
                    self.ypos / self.ypos.abs()
                };

                (xdir, ydir)
            }
        }

        for instruction in input_lines("input_files/day12.txt") {
            let (mode, n) = instruction.split_at(1);
            let n: i64 = n.parse().unwrap();

            match mode {
                "N" => {
                    waypoint.ypos -= n;
                }
                "E" => {
                    waypoint.xpos += n;
                }
                "S" => {
                    waypoint.ypos += n;
                }
                "W" => {
                    waypoint.xpos -= n;
                }
                "L" => {
                    for _ in 0..(n / 90) {
                        let direction = waypoint.unit_direction();

                        let adjustment = match direction {
                            (1, -1) => (-1, -1),
                            (-1, -1) => (-1, 1),
                            (-1, 1) => (1, 1),
                            (1, 1) => (1, -1),
                            _ => panic!("Bad mojo"),
                        };

                        let new_x = waypoint.ypos.abs() * adjustment.0;
                        let new_y = waypoint.xpos.abs() * adjustment.1;
                        waypoint.xpos = new_x;
                        waypoint.ypos = new_y;
                    }
                }
                "R" => {
                    for _ in 0..(n / 90) {
                        let direction = waypoint.unit_direction();

                        let adjustment = match direction {
                            (1, -1) => (1, 1),
                            (1, 1) => (-1, 1),
                            (-1, 1) => (-1, -1),
                            (-1, -1) => (1, -1),
                            _ => panic!("Bad mojo"),
                        };

                        let new_x = waypoint.ypos.abs() * adjustment.0;
                        let new_y = waypoint.xpos.abs() * adjustment.1;
                        waypoint.xpos = new_x;
                        waypoint.ypos = new_y;
                    }
                }
                "F" => {
                    for _ in 0..n {
                        ferry.xpos += waypoint.xpos;
                        ferry.ypos += waypoint.ypos;
                    }
                }
                _ => panic!("unrecognised instruction"),
            }
        }

        println!(
            "Pt 2 Manhattan distance: {}",
            ferry.xpos.abs() + ferry.ypos.abs()
        );
    }
}

mod day13 {
    use crate::shared::*;

    pub fn part1() {
        let (time_str, bus_str) = input_lines("input_files/day13.txt")
            .collect_tuple()
            .unwrap();

        let time: usize = time_str
            .parse()
            .unwrap_or_else(|_| panic!("Time parse error: {}", time_str));
        let buses: Vec<usize> = bus_str
            .split(',')
            .map(|s| {
                if s == "x" {
                    None
                } else {
                    Some(
                        s.parse::<usize>()
                            .unwrap_or_else(|_| panic!("Bus parse error: {}", s)),
                    )
                }
            })
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();

        fn next_time(time: usize, bus: usize) -> usize {
            let m = time % bus;

            if m == 0 {
                time
            } else {
                time + (bus - m)
            }
        }

        let next_bus = buses.iter().min_by_key(|&&b| next_time(time, b)).unwrap();

        println!(
            "Bus id: {}; wait time: {}",
            next_bus,
            next_bus - (time % next_bus)
        );
    }

    // https://en.wikipedia.org/wiki/Chinese_remainder_theorem?
    // https://www.dcode.fr/chinese-remainder
    // http://homepages.math.uic.edu/~leon/mcs425-s08/handouts/chinese_remainder.pdf

    // Might want this to find modular inverses?
    // Implemented from pseudocode here: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm

    // This gets me the modulo inverses from the pdf above:
    // dbg!(((extended_gcd(7, 11).0 % 11) + 11) % 11);
    // dbg!(((extended_gcd(15, 16).0 % 16) + 16) % 16);
    // dbg!(((extended_gcd(11, 21).0 % 21) + 21) % 21);

    #[allow(clippy::many_single_char_names)]
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

        (old_s, old_t)
    }

    fn chinese_remainder(m_vals: Vec<i64>, a_vals: Vec<i64>) -> i64 {
        let m: i64 = m_vals.iter().product();

        let z_vals: Vec<i64> = m_vals.iter().map(|nth_m| m / nth_m).collect();

        let y_vals: Vec<i64> = m_vals
            .iter()
            .zip(z_vals.iter())
            .map(|(&nth_m, &nth_z)| {
                // z^-1 mod nth_m
                ((extended_gcd(nth_z, nth_m).0 % nth_m) + nth_m) % nth_m
            })
            .collect();

        let w_vals: Vec<i64> = y_vals
            .iter()
            .zip(z_vals.iter())
            .map(|(&nth_y, &nth_z)| (nth_y * nth_z) % m)
            .collect();

        let solution: i64 = a_vals
            .iter()
            .zip(w_vals.iter())
            .map(|(&nth_a, &nth_w)| nth_a * nth_w)
            .sum();

        solution % m
    }

    pub fn part2() {
        let (_, bus_str) = input_lines("input_files/day13.txt")
            .collect_tuple()
            .unwrap();

        let buses: Vec<Option<usize>> = bus_str
            .split(',')
            .map(|s| {
                if s == "x" {
                    None
                } else {
                    Some(
                        s.parse::<usize>()
                            .unwrap_or_else(|_| panic!("Bus parse error: {}", s)),
                    )
                }
            })
            .collect();

        let bus_indexes: Vec<usize> = (0..buses.len())
            .filter(|&idx| buses[idx].is_some())
            .collect();

        // Our modulo values are just our bus numbers
        let mods: Vec<i64> = bus_indexes
            .iter()
            .map(|&idx| buses[idx as usize].unwrap() as i64)
            .collect();

        // But our remainders are shifted left to t = 0
        let remainders: Vec<i64> = bus_indexes
            .iter()
            .map(|&idx| {
                let m = buses[idx as usize].unwrap();
                (m - idx) as i64
            })
            .collect();

        // To victory!
        println!(
            "Earliest possible time: {}",
            chinese_remainder(mods, remainders)
        );
    }
}

mod day14 {
    use crate::shared::*;

    fn apply_mask(mask: &str, value: usize) -> usize {
        let mut result = value;

        for (idx, ch) in mask.chars().enumerate() {
            let bit_offset = (mask.len() - idx - 1);
            let bit = match ch {
                '0' => 0,
                '1' => 1,
                'X' => continue,
                _ => panic!("Weird mask: {}", mask),
            };

            if bit == 1 {
                result |= 1 << bit_offset
            } else {
                result &= !(1 << bit_offset)
            }
        }

        result
    }

    pub fn part1() {
        let lines = input_lines("input_files/day14.txt");

        let mut mask: String = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string();
        let mut memory: Vec<usize> = Vec::new();

        let mask_regex = Regex::new("mask = (.+)").unwrap();
        let memory_regex = Regex::new("mem\\[(.+)\\] = (.+)").unwrap();

        for line in lines {
            if let Some(cap) = mask_regex.captures(&line) {
                mask = cap[1].to_string();
            } else if let Some(cap) = memory_regex.captures(&line) {
                let address: usize = cap[1].parse().unwrap();
                let value: usize = cap[2].parse().unwrap();

                while memory.len() <= address {
                    memory.push(0);
                }

                memory[address] = apply_mask(&mask, value)
            } else {
                panic!("Parse error: {}", line);
            }
        }

        println!("Sum of memory: {}", memory.iter().sum::<usize>());
    }

    fn decode_memory_address(mask: &str, address: usize) -> Vec<usize> {
        let mut result: Vec<usize> = vec![0];

        for (idx, ch) in mask.chars().enumerate() {
            let bit_offset = (mask.len() - idx - 1);

            match ch {
                '0' => {
                    // unchanged from `address`
                    result.iter_mut().for_each(|a| {
                        let bit = ((address >> bit_offset) & 1);

                        if bit == 1 {
                            *a |= 1 << bit_offset;
                        } else {
                            *a &= !(1 << bit_offset);
                        }
                    });
                }
                '1' => {
                    // overwrite with 1
                    result.iter_mut().for_each(|a| {
                        *a |= 1 << bit_offset;
                    });
                }
                'X' => {
                    // floating
                    let mut perms: Vec<usize> = Vec::new();
                    for a in &result {
                        perms.push(a | (1 << bit_offset));
                        perms.push(a & !(1 << bit_offset));
                    }

                    result = perms;
                }
                _ => panic!("Weird mask: {}", mask),
            }
        }

        result
    }

    pub fn part2() {
        let lines = input_lines("input_files/day14.txt");

        let mut mask: String = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string();
        let mut memory: HashMap<usize, usize> = HashMap::new();

        let mask_regex = Regex::new("mask = (.+)").unwrap();
        let memory_regex = Regex::new("mem\\[(.+)\\] = (.+)").unwrap();

        for line in lines {
            if let Some(cap) = mask_regex.captures(&line) {
                mask = cap[1].to_string();
            } else if let Some(cap) = memory_regex.captures(&line) {
                let base_address: usize = cap[1].parse().unwrap();
                let value: usize = cap[2].parse().unwrap();

                for address in decode_memory_address(&mask, base_address) {
                    memory.insert(address, value);
                }
            } else {
                panic!("Parse error: {}", line);
            }
        }

        println!("Sum of memory: {}", memory.values().sum::<usize>());
    }
}

mod day15 {
    use crate::shared::*;

    pub fn part1() {
        let input: Vec<usize> = vec![16, 1, 0, 18, 12, 14, 19];
        // let input: Vec<usize> = vec![0,3,6];

        let mut number_last_turn: HashMap<usize, usize> = HashMap::new();

        let mut turn = 0;

        // populate our initial state, excluding the last number
        for &n in &input[0..input.len() - 1] {
            turn += 1;

            number_last_turn.insert(n, turn);
        }

        let mut last_number = *input.last().unwrap();
        turn += 1;

        for _ in 0..(2020 - input.len()) {
            let new_number = if number_last_turn.contains_key(&last_number) {
                turn - number_last_turn.get(&last_number).unwrap()
            } else {
                0
            };

            number_last_turn.insert(last_number, turn);

            turn += 1;
            last_number = new_number;
        }

        println!("Number 2020 is {}", last_number);
    }

    pub fn part2() {
        let input: Vec<usize> = vec![16, 1, 0, 18, 12, 14, 19];
        // let input: Vec<usize> = vec![0,3,6];

        let mut number_last_turn: HashMap<usize, usize> = HashMap::new();

        let mut turn = 0;

        // populate our initial state, excluding the last number
        for &n in &input[0..input.len() - 1] {
            turn += 1;

            number_last_turn.insert(n, turn);
        }

        let mut last_number = *input.last().unwrap();
        turn += 1;

        for _ in 0..(30000000 - input.len()) {
            let new_number = if number_last_turn.contains_key(&last_number) {
                turn - number_last_turn.get(&last_number).unwrap()
            } else {
                0
            };

            number_last_turn.insert(last_number, turn);

            turn += 1;
            last_number = new_number;
        }

        println!("Number 2020 is {}", last_number);
    }
}

mod day16 {
    use crate::shared::*;

    #[derive(Debug, Clone)]
    struct Rule {
        description: String,
        ranges: Vec<std::ops::Range<usize>>,
    }

    impl Rule {
        fn matches(&self, n: usize) -> bool {
            self.ranges.iter().any(|r| r.contains(&n))
        }
    }

    #[derive(Debug, Clone)]
    struct Notes {
        rules: Vec<Rule>,
        ticket: Vec<usize>,
        nearby: Vec<Vec<usize>>,
    }

    fn parse_rule(s: String) -> Rule {
        let (description, rule_s) = s.split(": ").collect_tuple().unwrap();
        Rule {
            description: description.to_string(),
            ranges: rule_s
                .split(" or ")
                .map(|range| {
                    let (start, end) = range
                        .split('-')
                        .map(|r| r.parse().unwrap())
                        .collect_tuple()
                        .unwrap();
                    std::ops::Range {
                        start,
                        end: end + 1,
                    }
                })
                .collect(),
        }
    }

    fn parse_notes(lines: Vec<String>) -> Notes {
        let (rule_lines, ticket_lines, nearby_lines) =
            lines.split(|s| s.is_empty()).collect_tuple().unwrap();

        Notes {
            rules: rule_lines
                .iter()
                .map(|s| parse_rule(s.to_string()))
                .collect(),
            ticket: ticket_lines
                .get(1)
                .unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect(),
            nearby: nearby_lines
                .iter()
                .skip(1)
                .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect())
                .collect(),
        }
    }

    pub fn part1() {
        let lines: Vec<String> = input_lines("input_files/day16.txt").collect();
        let notes = parse_notes(lines);

        let mut inconceivable_numbers = Vec::new();

        for nearby in &notes.nearby {
            for &n in nearby {
                if !notes.rules.iter().any(|r| r.matches(n)) {
                    inconceivable_numbers.push(n);
                }
            }
        }

        println!(
            "Error rate: {}",
            inconceivable_numbers.iter().sum::<usize>()
        );
    }

    pub fn part2() {
        let lines: Vec<String> = input_lines("input_files/day16.txt").collect();
        let notes = parse_notes(lines);

        let mut valid_tickets = Vec::new();

        for nearby in &notes.nearby {
            if nearby
                .iter()
                .any(|&n| !notes.rules.iter().any(|r| r.matches(n)))
            {
                // Invalid ticket
            } else {
                valid_tickets.push(nearby.clone());
            }
        }

        // Our ticket is valid too
        let column_count = notes.ticket.len();
        valid_tickets.push(notes.ticket.clone());

        // Each column has a list of rule indexes corresponding to the rules that
        // satisfy it.
        let mut column_candidates: Vec<HashSet<usize>> =
            (0..column_count).map(|_| HashSet::new()).collect();

        for (rule_idx, r) in notes.rules.iter().enumerate() {
            for ticket_column in 0..column_count {
                if valid_tickets
                    .iter()
                    .all(|ticket| r.matches(ticket[ticket_column]))
                {
                    column_candidates[ticket_column].insert(rule_idx);
                }
            }
        }

        // Iteratively whittle down the candidates by locking in the rules that can only
        // correspond to one column.
        for _ in (0..column_count) {
            let locked_rules: Vec<usize> = column_candidates
                .iter()
                .filter(|&set| set.len() == 1)
                .map(|set| *set.iter().next().unwrap())
                .collect();

            for set in column_candidates.iter_mut() {
                if set.len() > 1 {
                    for r in &locked_rules {
                        set.remove(&r);
                    }
                }
            }
        }

        assert!(column_candidates.iter().all(|c| c.len() == 1));

        let ordered_rules: Vec<usize> = column_candidates
            .iter()
            .map(|c| *c.iter().next().unwrap())
            .collect();

        let mut result = 1;
        for (column_idx, &rule_idx) in ordered_rules.iter().enumerate() {
            if notes.rules[rule_idx].description.starts_with("departure") {
                result *= notes.ticket[column_idx]
            }
        }

        println!("Result: {}", result);
    }
}

mod day17 {
    use crate::shared::*;

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct Point3D {
        x: i64,
        y: i64,
        z: i64,
    }

    impl Point3D {
        fn neighbours(&self) -> Vec<Point3D> {
            let mut result = Vec::with_capacity(26);

            for xoff in (-1..=1) {
                for yoff in (-1..=1) {
                    for zoff in (-1..=1) {
                        if xoff == 0 && yoff == 0 && zoff == 0 {
                            continue;
                        }

                        result.push(Point3D {
                            x: self.x + xoff,
                            y: self.y + yoff,
                            z: self.z + zoff,
                        });
                    }
                }
            }

            result
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    enum State {
        Active,
        Inactive,
    }

    #[derive(Debug, Clone)]
    struct Grid3D {
        grid: HashMap<Point3D, State>,
    }

    impl Grid3D {
        fn parse(lines: Vec<String>) -> Grid3D {
            let mut result = Grid3D::new();

            for (row_idx, row) in lines.iter().enumerate() {
                for (col_idx, ch) in row.chars().enumerate() {
                    let p = Point3D {
                        x: col_idx as i64,
                        y: row_idx as i64,
                        z: 0i64,
                    };

                    let v = if ch == '#' {
                        State::Active
                    } else {
                        State::Inactive
                    };

                    result.grid.insert(p, v);
                }
            }

            result
        }

        fn new() -> Grid3D {
            Grid3D {
                grid: HashMap::new(),
            }
        }

        fn is_active(&self, p: &Point3D) -> bool {
            if let Some(s) = self.grid.get(p) {
                s == &State::Active
            } else {
                false
            }
        }

        fn set_active(&mut self, p: &Point3D) {
            self.grid.insert(p.clone(), State::Active);
        }

        fn set_inactive(&mut self, p: &Point3D) {
            self.grid.insert(p.clone(), State::Inactive);
        }

        fn count_active(&self) -> usize {
            self.grid.keys().filter(|p| self.is_active(p)).count()
        }

        fn bounding_cube(&self) -> (Point3D, Point3D) {
            let min = self
                .grid
                .keys()
                .cloned()
                .reduce(|min, p| Point3D {
                    x: min.x.min(p.x),
                    y: min.y.min(p.y),
                    z: min.z.min(p.z),
                })
                .unwrap();

            let max = self
                .grid
                .keys()
                .cloned()
                .reduce(|max, p| Point3D {
                    x: max.x.max(p.x),
                    y: max.y.max(p.y),
                    z: max.z.max(p.z),
                })
                .unwrap();

            (min, max)
        }
    }

    pub fn part1() {
        let mut grid = Grid3D::parse(input_lines("input_files/day17.txt").collect());

        for _cycle in 0..6 {
            let (min, max) = grid.bounding_cube();

            let mut next_grid = Grid3D::new();

            for z in (min.z - 1)..=(max.z + 1) {
                for y in (min.y - 1)..=(max.y + 1) {
                    for x in (min.x - 1)..=(max.x + 1) {
                        let p = Point3D { x, y, z };

                        if grid.is_active(&p) {
                            let active_neighbour_count = p
                                .neighbours()
                                .iter()
                                .filter(|np| grid.is_active(np))
                                .count();

                            if active_neighbour_count == 2 || active_neighbour_count == 3 {
                                next_grid.set_active(&p);
                            } else {
                                next_grid.set_inactive(&p);
                            }
                        } else {
                            let active_neighbour_count = p
                                .neighbours()
                                .iter()
                                .filter(|np| grid.is_active(np))
                                .count();

                            if active_neighbour_count == 3 {
                                next_grid.set_active(&p);
                            } else {
                                next_grid.set_inactive(&p);
                            }
                        }
                    }
                }
            }

            grid = next_grid;
        }

        println!("Active cubes: {}", grid.count_active())
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct Point4D {
        w: i64,
        x: i64,
        y: i64,
        z: i64,
    }

    impl Point4D {
        fn neighbours(&self) -> Vec<Point4D> {
            let mut result = Vec::with_capacity(26);

            for woff in (-1..=1) {
                for xoff in (-1..=1) {
                    for yoff in (-1..=1) {
                        for zoff in (-1..=1) {
                            if woff == 0 && xoff == 0 && yoff == 0 && zoff == 0 {
                                continue;
                            }

                            result.push(Point4D {
                                w: self.w + woff,
                                x: self.x + xoff,
                                y: self.y + yoff,
                                z: self.z + zoff,
                            });
                        }
                    }
                }
            }

            result
        }
    }

    #[derive(Debug, Clone)]
    struct Grid4D {
        grid: HashMap<Point4D, State>,
    }

    impl Grid4D {
        fn parse(lines: Vec<String>) -> Grid4D {
            let mut result = Grid4D::new();

            for (row_idx, row) in lines.iter().enumerate() {
                for (col_idx, ch) in row.chars().enumerate() {
                    let p = Point4D {
                        w: 0,
                        x: col_idx as i64,
                        y: row_idx as i64,
                        z: 0i64,
                    };

                    let v = if ch == '#' {
                        State::Active
                    } else {
                        State::Inactive
                    };

                    result.grid.insert(p, v);
                }
            }

            result
        }

        fn new() -> Grid4D {
            Grid4D {
                grid: HashMap::new(),
            }
        }

        fn is_active(&self, p: &Point4D) -> bool {
            if let Some(s) = self.grid.get(p) {
                s == &State::Active
            } else {
                false
            }
        }

        fn set_active(&mut self, p: &Point4D) {
            self.grid.insert(p.clone(), State::Active);
        }

        fn set_inactive(&mut self, p: &Point4D) {
            self.grid.insert(p.clone(), State::Inactive);
        }

        fn count_active(&self) -> usize {
            self.grid.keys().filter(|p| self.is_active(p)).count()
        }

        fn bounding_cube(&self) -> (Point4D, Point4D) {
            let min = self
                .grid
                .keys()
                .cloned()
                .reduce(|min, p| Point4D {
                    w: min.w.min(p.w),
                    x: min.x.min(p.x),
                    y: min.y.min(p.y),
                    z: min.z.min(p.z),
                })
                .unwrap();

            let max = self
                .grid
                .keys()
                .cloned()
                .reduce(|max, p| Point4D {
                    w: max.w.max(p.w),
                    x: max.x.max(p.x),
                    y: max.y.max(p.y),
                    z: max.z.max(p.z),
                })
                .unwrap();

            (min, max)
        }
    }

    pub fn part2() {
        let mut grid = Grid4D::parse(input_lines("input_files/day17.txt").collect());

        for _cycle in 0..6 {
            let (min, max) = grid.bounding_cube();

            let mut next_grid = Grid4D::new();

            for z in (min.z - 1)..=(max.z + 1) {
                for y in (min.y - 1)..=(max.y + 1) {
                    for x in (min.x - 1)..=(max.x + 1) {
                        for w in (min.w - 1)..=(max.w + 1) {
                            let p = Point4D { w, x, y, z };

                            if grid.is_active(&p) {
                                let active_neighbour_count = p
                                    .neighbours()
                                    .iter()
                                    .filter(|np| grid.is_active(np))
                                    .count();

                                if active_neighbour_count == 2 || active_neighbour_count == 3 {
                                    next_grid.set_active(&p);
                                } else {
                                    next_grid.set_inactive(&p);
                                }
                            } else {
                                let active_neighbour_count = p
                                    .neighbours()
                                    .iter()
                                    .filter(|np| grid.is_active(np))
                                    .count();

                                if active_neighbour_count == 3 {
                                    next_grid.set_active(&p);
                                } else {
                                    next_grid.set_inactive(&p);
                                }
                            }
                        }
                    }
                }
            }

            grid = next_grid;
        }

        println!("Active cubes: {}", grid.count_active())
    }
}

mod day18 {
    use crate::shared::*;

    #[derive(Debug, Eq, PartialEq)]
    enum Token {
        Number(i64),
        Add,
        Multiply,
        OpenParen,
        CloseParen,
    }

    fn tokenise(line: String) -> VecDeque<Token> {
        let mut result = VecDeque::new();

        let mut chars: VecDeque<char> = line.chars().collect();

        while !chars.is_empty() {
            let ch = chars.pop_front().unwrap();

            if ch.is_digit(10) {
                let mut n: u32 = ch.to_digit(10).unwrap();

                while !chars.is_empty() && chars[0].is_digit(10) {
                    n *= 10;
                    n += chars.pop_front().unwrap().to_digit(10).unwrap();
                }

                result.push_back(Token::Number(n as i64));
            } else if ch == '+' {
                result.push_back(Token::Add);
            } else if ch == '*' {
                result.push_back(Token::Multiply);
            } else if ch == '(' {
                result.push_back(Token::OpenParen);
            } else if ch == ')' {
                result.push_back(Token::CloseParen);
            } else {
                // Ignored
            }
        }

        result
    }

    fn eval_expression(tokens: &mut VecDeque<Token>) -> i64 {
        if tokens.is_empty() {
            return 0;
        }

        let mut lhs = eval_next_expression(tokens);

        loop {
            if tokens.is_empty() {
                return lhs;
            }

            if tokens[0] == Token::CloseParen {
                tokens.pop_front();
                return lhs;
            }

            let op = tokens.pop_front().unwrap();

            if op == Token::Add {
                lhs += eval_next_expression(tokens);
            } else {
                lhs *= eval_next_expression(tokens);
            }
        }
    }

    fn eval_next_expression(tokens: &mut VecDeque<Token>) -> i64 {
        if tokens.is_empty() {
            return 0;
        }

        let token = tokens.pop_front().unwrap();

        if let Token::Number(n) = token {
            n
        } else if token == Token::OpenParen {
            eval_expression(tokens)
        } else {
            panic!("Parse error!");
        }
    }

    pub fn part1() {
        assert_eq!(
            eval_expression(&mut tokenise("2 * 3 + (4 * 5)".to_string())),
            26
        );
        assert_eq!(
            eval_expression(&mut tokenise("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string())),
            437
        );
        assert_eq!(
            eval_expression(&mut tokenise(
                "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()
            )),
            12240
        );
        assert_eq!(
            eval_expression(&mut tokenise(
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()
            )),
            13632
        );

        let expr: i64 = input_lines("input_files/day18.txt")
            .map(|line| {
                let mut tokens = tokenise(line);
                eval_expression(&mut tokens)
            })
            .sum();

        println!("{}", expr);
    }

    #[derive(Debug)]
    enum Expression {
        Number(i64),
        Add {
            lhs: Box<Expression>,
            rhs: Box<Expression>,
        },
        Multiply {
            lhs: Box<Expression>,
            rhs: Box<Expression>,
        },
    }

    const ADD_PRECEDENCE: usize = 10;
    const MULTIPLY_PRECEDENCE: usize = 1;

    fn parse_expression(tokens: &mut VecDeque<Token>, precedence: usize) -> Expression {
        if tokens.is_empty() {
            panic!("EOF");
        }

        let mut lhs = match tokens.pop_front().unwrap() {
            Token::OpenParen => {
                let r = parse_expression(tokens, 0);
                tokens.pop_front();
                r
            }
            Token::Number(n) => Expression::Number(n),
            unexpected_token => panic!("Parse error! {:?}", unexpected_token),
        };

        while !tokens.is_empty() {
            match tokens[0] {
                Token::Add => {
                    if ADD_PRECEDENCE < precedence {
                        break;
                    }

                    tokens.pop_front();
                    let rhs = parse_expression(tokens, ADD_PRECEDENCE + 1);

                    lhs = Expression::Add {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    }
                }
                Token::Multiply => {
                    if MULTIPLY_PRECEDENCE < precedence {
                        break;
                    }

                    tokens.pop_front();
                    let rhs = parse_expression(tokens, MULTIPLY_PRECEDENCE + 1);

                    lhs = Expression::Multiply {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    }
                }
                Token::CloseParen => {
                    break;
                }
                _ => panic!("Parse error"),
            }
        }

        lhs
    }

    fn dump_expression(e: &Expression) -> String {
        match e {
            Expression::Number(n) => format!("{}", n),
            Expression::Add { lhs, rhs } => {
                format!("({} + {})", dump_expression(lhs), dump_expression(rhs))
            }
            Expression::Multiply { lhs, rhs } => {
                format!("({} * {})", dump_expression(lhs), dump_expression(rhs))
            }
        }
    }

    fn eval_prattish_expression(expr: String) -> i64 {
        let mut tokens = tokenise(expr);
        let expr = parse_expression(&mut tokens, 0);

        fn eval(e: Expression) -> i64 {
            match e {
                Expression::Number(n) => n,
                Expression::Add { lhs, rhs } => eval(*lhs) + eval(*rhs),
                Expression::Multiply { lhs, rhs } => eval(*lhs) * eval(*rhs),
            }
        }

        eval(expr)
    }

    pub fn part2() {
        assert_eq!(
            eval_prattish_expression("1 + (2 * 3) + (4 * (5 + 6))".to_string()),
            51
        );
        assert_eq!(eval_prattish_expression("2 * 3 + (4 * 5)".to_string()), 46);
        assert_eq!(
            eval_prattish_expression("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()),
            1445
        );
        assert_eq!(
            eval_prattish_expression("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()),
            669060
        );
        assert_eq!(
            eval_prattish_expression("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()),
            23340
        );

        let expr: i64 = input_lines("input_files/day18.txt")
            .map(eval_prattish_expression)
            .sum();

        println!("Pt2: {}", expr);
    }
}

mod day19 {
    use crate::shared::*;

    type RuleSequence = Vec<usize>;

    #[derive(Clone, Debug)]
    enum Rule {
        Literal(char),
        Alt(Vec<RuleSequence>),
    }

    struct RuleSet {
        rules: Vec<Rule>,
    }

    impl RuleSet {
        fn parse_rules(lines: Vec<String>) -> RuleSet {
            let mut rules: Vec<Option<Rule>> = vec![None; lines.len()];

            for rule_s in lines {
                let (rule_number, rule_expr) = rule_s.split(": ").collect_tuple().unwrap();
                let rule_number: usize = rule_number.parse().unwrap();

                if rule_expr.starts_with('"') {
                    // Literal
                    rules[rule_number] = Some(Rule::Literal(rule_expr.chars().nth(1).unwrap()));
                } else {
                    // Alt
                    rules[rule_number] = Some(Rule::Alt(
                        rule_expr
                            .split(" | ")
                            .map(|s| s.split(' ').map(|n| n.parse::<usize>().unwrap()).collect())
                            .collect(),
                    ));
                }
            }

            RuleSet {
                rules: rules.into_iter().map(|o| o.unwrap()).collect(),
            }
        }

        // Returns a list of indexes into `input` representing the character one past
        // the portion that matched `rule`.
        fn match_positions(&self, input: &[char], start_rule: usize, offset: usize) -> Vec<usize> {
            if offset >= input.len() {
                // Nope
                return vec![];
            }

            match &self.rules[start_rule] {
                Rule::Literal(ch) => {
                    if &input[offset] == ch {
                        vec![offset + 1]
                    } else {
                        vec![]
                    }
                }
                Rule::Alt(patterns) => patterns
                    .iter()
                    .map(|p: &RuleSequence| {
                        let next_rule = p[0];
                        p.iter().skip(1).fold(
                            self.match_positions(input, next_rule, offset),
                            |match_positions, next_rule| {
                                match_positions
                                    .iter()
                                    .map(|idx| self.match_positions(input, *next_rule, *idx))
                                    .flatten()
                                    .collect()
                            },
                        )
                    })
                    .flatten()
                    .collect(),
            }
        }
    }

    pub fn part1() {
        let lines: Vec<String> = input_lines("input_files/day19.txt").collect();

        let (rule_lines, message_lines) = lines.split(|s| s.is_empty()).collect_tuple().unwrap();

        let rule_set = RuleSet::parse_rules(rule_lines.to_vec());

        let mut hits = 0;

        for message in message_lines {
            let input: Vec<char> = message.chars().collect();
            let matches = rule_set.match_positions(&input, 0, 0);
            if matches.iter().any(|&idx| idx == message.len()) {
                hits += 1;
            }
        }

        println!("Matches: {}", hits);
    }

    pub fn part2() {
        let lines: Vec<String> = input_lines("input_files/day19.txt").collect();

        let (rule_lines, message_lines) = lines.split(|s| s.is_empty()).collect_tuple().unwrap();

        let mut rule_set = RuleSet::parse_rules(rule_lines.to_vec());

        rule_set.rules[8] = Rule::Alt(vec![vec![42], vec![42, 8]]);
        rule_set.rules[11] = Rule::Alt(vec![vec![42, 31], vec![42, 11, 31]]);

        let mut hits = 0;

        for message in message_lines {
            let input: Vec<char> = message.chars().collect();
            let matches = rule_set.match_positions(&input, 0, 0);
            if matches.iter().any(|&idx| idx == message.len()) {
                hits += 1;
            }
        }

        println!("Matches: {}", hits);
    }
}

mod day20 {
    use crate::shared::*;

    lazy_static! {
        static ref TITLE_REGEX: regex::Regex = Regex::new("^Tile ([0-9]+):$").unwrap();
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct Tile {
        id: usize,
        top: String,
        left: String,
        right: String,
        bottom: String,

        image: Vec<Vec<char>>,
    }

    impl Tile {
        fn parse(lines: &[String]) -> Tile {
            let id: usize = if let Some(cap) = TITLE_REGEX.captures(&lines[0]) {
                cap[1].parse().unwrap()
            } else {
                panic!("Parse error on title line: {}", lines[0]);
            };

            let top = lines[1].clone();
            let bottom = lines[lines.len() - 1].clone();
            let left = lines
                .iter()
                .skip(1)
                .map(|l| l.chars().next().unwrap())
                .collect();
            let right = lines
                .iter()
                .skip(1)
                .map(|l| l.chars().nth(l.len() - 1).unwrap())
                .collect();

            let image: Vec<Vec<char>> = {
                let char_vec: Vec<Vec<char>> = lines
                    .iter()
                    .skip(1)
                    .map(|row| row.chars().collect())
                    .collect();

                // discard borders
                char_vec[1..char_vec.len() - 1]
                    .iter()
                    .map(|row: &Vec<char>| row[1..row.len() - 1].to_vec())
                    .collect()
            };

            Tile {
                id,
                top,
                bottom,
                left,
                right,
                image,
            }
        }

        fn rotate_right(&self, times: usize) -> Tile {
            let width = self.image[0].len();
            let height = self.image.len();

            let mut result = self.clone();
            for _ in 0..times {
                result = Tile {
                    id: self.id,
                    top: result.left.chars().rev().collect(),
                    right: result.top.clone(),
                    bottom: result.right.chars().rev().collect(),
                    left: result.bottom.clone(),
                    image: (0..width)
                        .map(|x| (0..height).map(|y| result.image[y][x]).rev().collect())
                        .collect(),
                }
            }

            result
        }

        fn flip(&self, times: usize) -> Tile {
            // Flip about the Y axis because why not
            let mut result = self.clone();
            for _ in 0..times {
                result = Tile {
                    id: self.id,
                    top: self.top.chars().rev().collect(),
                    right: self.left.clone(),
                    bottom: self.bottom.chars().rev().collect(),
                    left: self.right.clone(),
                    image: result
                        .image
                        .iter()
                        .map(|row| row.iter().rev().cloned().collect())
                        .collect(),
                }
            }

            result
        }
    }

    pub fn part1() {
        let lines: Vec<String> = input_lines("input_files/day20.txt").collect();
        let tiles: Vec<Tile> = lines
            .split(|s| s.is_empty())
            .map(|tile_lines| Tile::parse(tile_lines))
            .collect();

        let mut edge_map: HashMap<String, HashSet<usize>> = HashMap::new();

        // Load the edges of each tile into a big ol' map
        for t in &tiles {
            for flips in 0..=1 {
                let flipped = t.flip(flips);

                for rotations in 0..4 {
                    let rotated = flipped.rotate_right(rotations);

                    edge_map
                        .entry(rotated.top.clone())
                        .or_insert_with(HashSet::new)
                        .insert(rotated.id);
                    edge_map
                        .entry(rotated.right.clone())
                        .or_insert_with(HashSet::new)
                        .insert(rotated.id);
                    edge_map
                        .entry(rotated.bottom.clone())
                        .or_insert_with(HashSet::new)
                        .insert(rotated.id);
                    edge_map
                        .entry(rotated.left.clone())
                        .or_insert_with(HashSet::new)
                        .insert(rotated.id);
                }
            }
        }

        // Find the corners by picking the tiles where two edges aren't shared
        let corners: Vec<Tile> = tiles
            .iter()
            .filter(|&tile| {
                let mut edge_count = 0;
                edge_count += (edge_map.get(&tile.top).unwrap().len() == 1) as usize;
                edge_count += (edge_map.get(&tile.right).unwrap().len() == 1) as usize;
                edge_count += (edge_map.get(&tile.bottom).unwrap().len() == 1) as usize;
                edge_count += (edge_map.get(&tile.left).unwrap().len() == 1) as usize;

                edge_count == 2
            })
            .cloned()
            .collect();

        dbg!(corners.iter().map(|tile| tile.id).product::<usize>());
    }

    fn stitch_picture(tiles: Vec<Tile>) -> Vec<Vec<char>> {
        let mut tile_permutations = Vec::new();
        for t in &tiles {
            for flips in 0..=1 {
                let flipped = t.flip(flips);

                for rotations in 0..4 {
                    let rotated = flipped.rotate_right(rotations);
                    tile_permutations.push(rotated);
                }
            }
        }

        let mut directional_edge_map: HashMap<String, HashSet<Tile>> = HashMap::new();
        for t in &tile_permutations {
            directional_edge_map
                .entry(format!("TOP:{}", t.top))
                .or_insert_with(HashSet::new)
                .insert(t.clone());
            directional_edge_map
                .entry(format!("RIGHT:{}", t.right))
                .or_insert_with(HashSet::new)
                .insert(t.clone());
            directional_edge_map
                .entry(format!("BOTTOM:{}", t.bottom))
                .or_insert_with(HashSet::new)
                .insert(t.clone());
            directional_edge_map
                .entry(format!("LEFT:{}", t.left))
                .or_insert_with(HashSet::new)
                .insert(t.clone());
        }

        // Find a suitable top-left corner
        let top_left = tile_permutations
            .iter()
            .find(|tile| {
                directional_edge_map
                    .get(&format!("BOTTOM:{}", tile.top))
                    .unwrap()
                    .len()
                    == 1
                    && directional_edge_map
                        .get(&format!("RIGHT:{}", tile.left))
                        .unwrap()
                        .len()
                        == 1
            })
            .unwrap()
            .clone();

        let mut picture: Vec<Vec<Option<Tile>>> = vec![vec![None; 12]; 12];

        let mut placed_tiles_ids: HashSet<usize> = HashSet::new();

        placed_tiles_ids.insert(top_left.id);
        picture[0][0] = Some(top_left);

        // Fill out our first column
        for y in 1..12 {
            let above = picture[y - 1][0].as_ref().unwrap();

            let tile = directional_edge_map
                .get(&format!("TOP:{}", above.bottom))
                .unwrap()
                .iter()
                .find(|t| !placed_tiles_ids.contains(&t.id))
                .unwrap()
                .clone();
            placed_tiles_ids.insert(tile.id);
            picture[y][0] = Some(tile);
        }

        // Fill out the remainder of each row
        #[allow(clippy::needless_range_loop)]
        for y in 0..12 {
            for x in 1..12 {
                let west = picture[y][x - 1].as_ref().unwrap();

                let tile = directional_edge_map
                    .get(&format!("LEFT:{}", west.right))
                    .unwrap()
                    .iter()
                    .find(|t| !placed_tiles_ids.contains(&t.id))
                    .unwrap()
                    .clone();
                placed_tiles_ids.insert(tile.id);
                picture[y][x] = Some(tile);
            }
        }

        // Join the pretty pictures
        let picture_rows = picture[0][0].as_ref().unwrap().image.len();
        let picture_cols = picture[0][0].as_ref().unwrap().image[0].len();

        let picture_width = picture_cols * 12;
        let picture_height = picture_rows * 12;

        let mut result: Vec<Vec<char>> = vec![vec![' '; picture_width]; picture_height];

        #[allow(clippy::needless_range_loop)]
        for y in 0..picture_height {
            for x in 0..picture_width {
                let tile_y = y / picture_rows;
                let tile_x = x / picture_cols;

                let image_y = y % picture_rows;
                let image_x = x % picture_cols;

                result[y][x] = picture[tile_y][tile_x].as_ref().unwrap().image[image_y][image_x];
            }
        }

        result
    }

    pub fn part2() {
        let lines: Vec<String> = input_lines("input_files/day20.txt").collect();
        let tiles: Vec<Tile> = lines
            .split(|s| s.is_empty())
            .map(|tile_lines| Tile::parse(tile_lines))
            .collect();

        let image = Tile {
            id: 0,
            top: "".to_string(),
            right: "".to_string(),
            bottom: "".to_string(),
            left: "".to_string(),

            image: stitch_picture(tiles),
        };

        let nessie: Vec<Vec<char>> = vec![
            "                  # ".chars().collect(),
            "#    ##    ##    ###".chars().collect(),
            " #  #  #  #  #  #   ".chars().collect(),
        ];

        let nessie_width = nessie[0].len();
        let nessie_height = nessie.len();

        for flip in 0..=1 {
            for rotate in 0..4 {
                let img = image.flip(flip).rotate_right(rotate).image.clone();
                let img_width = img[0].len();
                let img_height = img.len();

                let mut nessie_coords: HashSet<(usize, usize)> = HashSet::new();

                for base_y in 0..(img_height - nessie_height) {
                    for base_x in 0..(img_width - nessie_width) {
                        let mut found_nessie = true;

                        let mut maybe_coords: HashSet<(usize, usize)> = HashSet::new();
                        'mainloop: for ny in 0..nessie_height {
                            for nx in 0..nessie_width {
                                if nessie[ny][nx] == '#' {
                                    if img[base_y + ny][base_x + nx] != '#' {
                                        found_nessie = false;
                                        maybe_coords.clear();
                                        break 'mainloop;
                                    } else {
                                        maybe_coords.insert(((base_x + nx), (base_y + ny)));
                                    }
                                }
                            }
                        }

                        if found_nessie {
                            nessie_coords.extend(maybe_coords);
                        }
                    }
                }

                if !nessie_coords.is_empty() {
                    for line in &img {
                        println!("{}", line.iter().collect::<String>());
                    }

                    let total_squares: usize = img
                        .iter()
                        .map(|row| row.iter().filter(|&&ch| ch == '#').count())
                        .sum();
                    println!(
                        "Non-seamonster squares: {}",
                        total_squares - nessie_coords.len()
                    );
                }
            }
        }
    }
}

mod day21 {
    use crate::shared::*;

    pub fn part1() {
        let line_with_allergens = Regex::new(r"^(.+) \(contains (.+)\)").unwrap();

        let mut all_ingredients: Vec<String> = Vec::new();
        let mut allergen_map: HashMap<String, HashSet<String>> = HashMap::new();

        for line in input_lines("input_files/day21.txt") {
            if let Some(cap) = line_with_allergens.captures(&line) {
                let ingredients: HashSet<String> =
                    HashSet::from_iter(cap[1].split(' ').map(str::to_owned));
                let allergens: Vec<String> = cap[2].split(", ").map(str::to_owned).collect();

                all_ingredients.extend(ingredients.clone());

                for a in &allergens {
                    if allergen_map.contains_key(a) {
                        // merge our sets
                        allergen_map
                            .entry(a.to_string())
                            .and_modify(|e| *e = e.intersection(&ingredients).cloned().collect());
                    } else {
                        allergen_map.insert(a.clone(), ingredients.clone());
                    }
                }
            }
        }

        let mut allergen_ingredients = HashSet::new();

        while !allergen_map.is_empty() {
            let mut resolved_ingredients: Vec<String> = Vec::new();

            for (allergen, ingredients) in &allergen_map {
                if ingredients.len() == 1 {
                    let ingredient = ingredients.iter().next().unwrap();
                    println!("ingredient {} contains allergen {}", ingredient, allergen);
                    resolved_ingredients.push(ingredient.clone());
                    allergen_ingredients.insert(ingredient.clone());
                }
            }

            if resolved_ingredients.is_empty() && !allergen_map.is_empty() {
                panic!("Failed to make progress");
            }

            for i in resolved_ingredients {
                for (_, ingredients) in allergen_map.iter_mut() {
                    ingredients.remove(&i);
                }
            }

            let allergens: Vec<String> = allergen_map.keys().cloned().collect();
            for a in allergens {
                if allergen_map.get(&a).unwrap().is_empty() {
                    allergen_map.remove(&a);
                }
            }
        }

        println!(
            "Pt1: {}",
            all_ingredients
                .iter()
                .filter(|&i| !allergen_ingredients.contains(i))
                .count()
        );
    }

    pub fn part2() {
        // Just rearranged my output using Emacs!  xgtj,ztdctgq,bdnrnx,cdvjp,jdggtft,mdbq,rmd,lgllb
    }
}

mod day22 {
    use crate::shared::*;

    fn score_game(hand: &VecDeque<usize>) -> usize {
        hand.iter()
            .rev()
            .enumerate()
            .map(|(idx, value)| (idx + 1) * value)
            .sum::<usize>()
    }

    pub fn part1() {
        let mut p1: VecDeque<usize> = VecDeque::from_iter(
            [
                17, 19, 30, 45, 25, 48, 8, 6, 39, 36, 28, 5, 47, 26, 46, 20, 18, 13, 7, 49, 34, 23,
                43, 22, 4,
            ]
            .iter()
            .copied(),
        );
        let mut p2: VecDeque<usize> = VecDeque::from_iter(
            [
                44, 10, 27, 9, 14, 15, 24, 16, 3, 33, 21, 29, 11, 38, 1, 31, 50, 41, 40, 32, 42,
                35, 37, 2, 12,
            ]
            .iter()
            .copied(),
        );

        loop {
            if p1.is_empty() && p2.is_empty() {
                println!("Draw!");
                break;
            }

            if p1.is_empty() {
                println!("Player 2 wins with score {}", score_game(&p2));
                break;
            }

            if p2.is_empty() {
                println!("Player 1 wins with score {}", score_game(&p1));
                break;
            }

            if p1[0] > p2[0] {
                // p1 wins round
                let p1_card = p1.pop_front().unwrap();
                let p2_card = p2.pop_front().unwrap();
                p1.push_back(p1_card);
                p1.push_back(p2_card);
            } else {
                // p2 wins round
                let p1_card = p1.pop_front().unwrap();
                let p2_card = p2.pop_front().unwrap();
                p2.push_back(p2_card);
                p2.push_back(p1_card);
            }
        }
    }

    #[derive(Hash, Clone, PartialEq, Eq, Debug)]
    struct GameState {
        p1: VecDeque<usize>,
        p2: VecDeque<usize>,
    }

    impl GameState {
        fn new(p1: &[usize], p2: &[usize]) -> GameState {
            GameState {
                p1: VecDeque::from_iter(p1.iter().copied()),
                p2: VecDeque::from_iter(p2.iter().copied()),
            }
        }
    }

    #[derive(Debug)]
    enum RoundResult {
        P1WinsRound,
        P2WinsRound,
    }

    fn play_round(
        mut state: GameState,
        mut seen_states: HashSet<GameState>,
        round: usize,
        game: usize,
    ) -> (RoundResult, GameState) {
        println!("\n-- Round {} (Game {}) --", round, game);
        println!(
            "Player 1's deck: {}",
            state
                .p1
                .iter()
                .map(|n| format!("{}", n))
                .collect::<Vec<String>>()
                .join(", ")
        );
        println!(
            "Player 2's deck: {}",
            state
                .p2
                .iter()
                .map(|n| format!("{}", n))
                .collect::<Vec<String>>()
                .join(", ")
        );

        if state.p1.is_empty() && state.p2.is_empty() {
            panic!("whoops");
        }

        if state.p1.is_empty() {
            return (RoundResult::P2WinsRound, state);
        }

        if state.p2.is_empty() {
            return (RoundResult::P1WinsRound, state);
        }

        if seen_states.contains(&state) {
            return (RoundResult::P1WinsRound, state);
        }

        seen_states.insert(state.clone());

        let p1_card = state.p1.pop_front().unwrap();
        let p2_card = state.p2.pop_front().unwrap();

        println!("Player 1 plays: {}", p1_card);
        println!("Player 2 plays: {}", p2_card);

        let result = if p1_card <= state.p1.len() && p2_card <= state.p2.len() {
            // Each player has enough cards left.  Recursive subgame
            let sub_state = GameState {
                p1: state.p1.iter().take(p1_card).copied().collect(),
                p2: state.p2.iter().take(p2_card).copied().collect(),
            };

            play_round(sub_state, HashSet::new(), 1, game + 1).0
        } else if p1_card > p2_card {
            RoundResult::P1WinsRound
        } else {
            RoundResult::P2WinsRound
        };

        match result {
            RoundResult::P1WinsRound => {
                println!("Player 1 wins round {} of game {}", round, game);

                state.p1.push_back(p1_card);
                state.p1.push_back(p2_card);
                play_round(state, seen_states, round + 1, game)
            }
            RoundResult::P2WinsRound => {
                println!("Player 2 wins round {} of game {}", round, game);

                state.p2.push_back(p2_card);
                state.p2.push_back(p1_card);
                play_round(state, seen_states, round + 1, game)
            }
        }
    }

    // 7595 too low
    // 9222 too low!
    pub fn part2() {
        let p1 = &[
            17, 19, 30, 45, 25, 48, 8, 6, 39, 36, 28, 5, 47, 26, 46, 20, 18, 13, 7, 49, 34, 23, 43,
            22, 4,
        ];
        let p2 = &[
            44, 10, 27, 9, 14, 15, 24, 16, 3, 33, 21, 29, 11, 38, 1, 31, 50, 41, 40, 32, 42, 35,
            37, 2, 12,
        ];

        let initial_state = GameState::new(p1, p2);

        let (result, state) = play_round(initial_state, HashSet::new(), 1, 1);

        match result {
            RoundResult::P1WinsRound => {
                println!("Player 1 wins with score {}", score_game(&state.p1))
            }
            RoundResult::P2WinsRound => {
                println!("Player 2 wins with score {}", score_game(&state.p2))
            }
        }
    }
}

mod day23 {
    use crate::shared::*;

    fn seek_forward_to_cup(cursor: &mut CursorMut<u32>, cup: u32) {
        if cursor.index().is_none() {
            cursor.move_next();
        }

        while *cursor.current().unwrap() != cup {
            cursor.move_next();
            if cursor.index().is_none() {
                // Back to the beginning
                cursor.move_next();
            }
        }
    }

    fn seek_back_to_cup(cursor: &mut CursorMut<u32>, cup: u32) {
        if cursor.index().is_none() {
            cursor.move_prev();
        }

        while *cursor.current().unwrap() != cup {
            cursor.move_prev();
            if cursor.index().is_none() {
                // Back to the beginning
                cursor.move_prev();
            }
        }
    }

    pub fn part1() {
        let mut cups: LinkedList<u32> = "156794823"
            .chars()
            .map(|ch| ch.to_digit(10).unwrap())
            .collect();

        let lowest_cup = cups.iter().min().cloned().unwrap();
        let highest_cup = cups.iter().max().cloned().unwrap();

        let mut current_cup = cups.cursor_front_mut();

        for _m in 0..100 {
            let current_label = current_cup.current().cloned().unwrap();

            current_cup.move_next();
            let mut chosen_cups: Vec<u32> = (0..3)
                .map(|_| {
                    if current_cup.index().is_none() {
                        current_cup.move_next();
                    }
                    current_cup.remove_current().unwrap()
                })
                .collect();

            // The destination cup is the current cup's label minus one, unless that cup is
            // in `chosen_cups`.  Then we repeatedly subtract one until we find on that
            // isn't.
            let mut destination_cup = current_label - 1;
            loop {
                if destination_cup < lowest_cup {
                    destination_cup = highest_cup;
                }

                if !chosen_cups.contains(&destination_cup) {
                    break;
                }

                destination_cup -= 1;
            }

            // move our chosen cups to sit after the destination cup
            seek_forward_to_cup(&mut current_cup, destination_cup);

            while !chosen_cups.is_empty() {
                current_cup.insert_after(chosen_cups.pop().unwrap());
            }

            // the current cup is one past the last current cup
            seek_forward_to_cup(&mut current_cup, current_label);
            loop {
                current_cup.move_next();
                if current_cup.index().is_some() {
                    break;
                }
            }
        }

        seek_forward_to_cup(&mut current_cup, 1);
        current_cup.move_next();
        while current_cup.index().is_none() || current_cup.current().cloned().unwrap() != 1 {
            if let Some(n) = current_cup.current() {
                print!("{}", n);
            }

            current_cup.move_next();
        }
        println!();
    }

    struct Cups {
        cups: HashMap<u64, NextCup>,
        lowest: u64,
        highest: u64,

        current_cup: u64,
    }

    impl Cups {
        fn new(init: &str, max_cup: usize) -> Cups {
            let mut initial_cups: Vec<u64> = init
                .chars()
                .map(|ch| ch.to_digit(10).unwrap() as u64)
                .collect();

            for c in (init.len() + 1)..=max_cup {
                initial_cups.push(c as u64);
            }

            let mut cups: HashMap<u64, NextCup> = HashMap::new();

            for c in 0..(initial_cups.len() - 1) {
                cups.insert(initial_cups[c], NextCup(initial_cups[c + 1]));
            }

            // circular!
            cups.insert(
                initial_cups[initial_cups.len() - 1],
                NextCup(initial_cups[0]),
            );

            Cups {
                cups,
                lowest: 1,
                highest: max_cup as u64,
                current_cup: initial_cups[0],
            }
        }

        fn remove_next(&mut self, n: usize) -> Vec<u64> {
            let mut result = Vec::new();

            let NextCup(mut next) = self.cups.remove(&self.current_cup).unwrap();

            for _ in 0..n {
                let NextCup(nextnext) = self.cups.remove(&next).unwrap();
                result.push(next);
                next = nextnext
            }

            self.cups.insert(self.current_cup, NextCup(next));

            result
        }

        fn insert_after(&mut self, destination: u64, to_insert: Vec<u64>) {
            let last = self.cups.remove(&destination).unwrap();

            let mut c = destination;

            for insert_me in to_insert {
                self.cups.insert(c, NextCup(insert_me));
                c = insert_me;
            }

            self.cups.insert(c, last);
        }

        fn move_next(&mut self) {
            let NextCup(next) = self.cups.get(&self.current_cup).unwrap();

            self.current_cup = *next;
        }

        fn seek_to(&mut self, value: u64) {
            assert!(self.cups.contains_key(&value));
            self.current_cup = value;
        }
    }

    #[derive(Hash, Debug, Eq, PartialEq)]
    struct NextCup(u64);

    pub fn part2() {
        let mut cups = Cups::new("156794823", 1_000_000);

        for _m in 0..10_000_000 {
            // remove the next three cups after this one
            let chosen_cups: Vec<u64> = cups.remove_next(3);

            // The destination cup is the current cup's label minus one, unless that cup is
            // in `chosen_cups`.  Then we repeatedly subtract one until we find on that
            // isn't.

            let mut destination_cup = cups.current_cup - 1;
            loop {
                if destination_cup < cups.lowest {
                    destination_cup = cups.highest;
                }

                if !chosen_cups.contains(&destination_cup) {
                    break;
                }

                destination_cup -= 1;
            }

            // move our chosen cups to sit after the destination cup
            cups.insert_after(destination_cup, chosen_cups);

            // move to the next cup
            cups.move_next();
        }

        cups.seek_to(1);
        let next_two = cups.remove_next(2);
        dbg!(&next_two);
        println!("{}", next_two[0] * next_two[1]);
    }
}

mod day24 {
    use crate::shared::*;

    #[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
    enum Direction {
        East,
        Southeast,
        Southwest,
        West,
        Northwest,
        Northeast,
    }

    #[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
    struct TileId {
        x: u64,
        y: u64,
    }

    impl TileId {
        fn move_to(&self, direction: Direction) -> TileId {
            match direction {
                Direction::East => TileId {
                    x: self.x + 2,
                    y: self.y,
                },
                Direction::West => TileId {
                    x: self.x - 2,
                    y: self.y,
                },
                Direction::Northeast => TileId {
                    x: self.x + 1,
                    y: self.y - 1,
                },
                Direction::Northwest => TileId {
                    x: self.x - 1,
                    y: self.y - 1,
                },
                Direction::Southeast => TileId {
                    x: self.x + 1,
                    y: self.y + 1,
                },
                Direction::Southwest => TileId {
                    x: self.x - 1,
                    y: self.y + 1,
                },
            }
        }
    }

    #[derive(Debug)]
    struct HexTile {
        id: TileId,
        flipped_to_black: bool,
    }

    #[derive(Debug)]
    struct HexGrid {
        tiles: HashMap<TileId, HexTile>,
    }

    impl HexGrid {
        fn new() -> HexGrid {
            let reference_tile = HexTile {
                id: TileId { x: 0, y: 0 },
                flipped_to_black: false,
            };

            let mut r = HexGrid {
                tiles: HashMap::new(),
            };

            r.tiles.insert(reference_tile.id, reference_tile);

            r
        }

        fn parse_path(&self, tile_path: &str) -> Vec<Direction> {
            let mut chars: VecDeque<char> = tile_path.chars().collect();
            let mut result = Vec::new();

            while !chars.is_empty() {
                let ch = chars.pop_front().unwrap();

                let s = if ch == 'n' || ch == 's' {
                    format!("{}{}", ch, chars.pop_front().unwrap())
                } else {
                    format!("{}", ch)
                };

                result.push(match s.as_str() {
                    "e" => Direction::East,
                    "w" => Direction::West,
                    "ne" => Direction::Northeast,
                    "nw" => Direction::Northwest,
                    "se" => Direction::Southeast,
                    "sw" => Direction::Southwest,
                    _ => panic!("parse error"),
                });
            }

            result
        }

        fn flip_tile(&mut self, tile_path: &str) {
            let path = self.parse_path(tile_path);

            // reference tile
            let mut current_tile_id = TileId { x: 0, y: 0 };

            for dir in path {
                let next_tile_id = current_tile_id.move_to(dir);

                self.tiles.entry(next_tile_id).or_insert(HexTile {
                    id: next_tile_id,
                    flipped_to_black: false,
                });

                current_tile_id = next_tile_id;
            }

            // Flip our destination tile
            let destination = self.tiles.get_mut(&current_tile_id).unwrap();
            destination.flipped_to_black = !destination.flipped_to_black;
        }
    }

    pub fn part1() {
        let mut grid = HexGrid::new();

        for line in input_lines("input_files/day24.txt") {
            grid.flip_tile(&line);
        }

        println!(
            "black tiles: {} of {}",
            grid.tiles
                .values()
                .filter(|tile| tile.flipped_to_black)
                .count(),
            grid.tiles.len()
        );
    }

    fn neighbours_of(tile_id: TileId) -> Vec<TileId> {
        vec![
            TileId {
                x: tile_id.x + 2,
                y: tile_id.y,
            },
            TileId {
                x: tile_id.x - 2,
                y: tile_id.y,
            },
            TileId {
                x: tile_id.x + 1,
                y: tile_id.y - 1,
            },
            TileId {
                x: tile_id.x - 1,
                y: tile_id.y - 1,
            },
            TileId {
                x: tile_id.x + 1,
                y: tile_id.y + 1,
            },
            TileId {
                x: tile_id.x - 1,
                y: tile_id.y + 1,
            },
        ]
    }

    pub fn part2() {
        let mut grid = HexGrid::new();

        for line in input_lines("input_files/day24.txt") {
            grid.flip_tile(&line);
        }

        println!(
            "Day 0: {}",
            grid.tiles.values().filter(|v| v.flipped_to_black).count()
        );

        for day in 1..=100 {
            let mut flippers = Vec::new();

            // fill out our grid a bit
            for tile_id in grid
                .tiles
                .values()
                .map(|tile| tile.id)
                .collect::<Vec<TileId>>()
            {
                for neighbour_id in neighbours_of(tile_id) {
                    grid.tiles.entry(neighbour_id).or_insert(HexTile {
                        id: neighbour_id,
                        flipped_to_black: false,
                    });
                }
            }

            for tile in grid.tiles.values() {
                let flipped_to_black_neighbours = neighbours_of(tile.id)
                    .iter()
                    .filter(|neighbour_id| {
                        if let Some(neighbour) = grid.tiles.get(&neighbour_id) {
                            neighbour.flipped_to_black
                        } else {
                            false
                        }
                    })
                    .count();

                if tile.flipped_to_black {
                    // black tile
                    if flipped_to_black_neighbours == 0 || flipped_to_black_neighbours > 2 {
                        flippers.push(tile.id);
                    }
                } else {
                    // white tile
                    if flipped_to_black_neighbours == 2 {
                        flippers.push(tile.id);
                    }
                }
            }

            for flipper in flippers {
                let entry = grid.tiles.get_mut(&flipper).unwrap();
                entry.flipped_to_black = !entry.flipped_to_black;
            }

            println!(
                "Day {}: {}",
                day,
                grid.tiles.values().filter(|v| v.flipped_to_black).count()
            );
        }
    }
}

mod day25 {
    use crate::shared::*;

    fn transform(subject_number: usize, loop_size: usize) -> usize {
        let mut result = 1;

        for _ in 0..loop_size {
            result *= subject_number;
            result %= 20201227;
        }

        result
    }

    fn find_loop_size(key: usize) -> usize {
        let mut loop_size = 1;

        let mut value = 1;
        loop {
            value *= 7;
            value %= 20201227;

            if value == key {
                return loop_size;
            }

            loop_size += 1;
        }
    }

    pub fn part1() {
        let pubkey1 = 12320657;
        let pubkey2 = 9659666;

        println!("Loop size 1: {}", find_loop_size(pubkey1));
        println!("Loop size 2: {}", find_loop_size(pubkey2));

        println!(
            "Encryption key: {}",
            transform(pubkey1, find_loop_size(pubkey2))
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

        day24::part1();
        day24::part2();
    }

    day25::part1();
}
