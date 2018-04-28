// (cd ../ && cargo build && scp target/debug/adventofcode2016 mozart:tmp/ && ssh mozart '(cd /home/mst/tmp; RUST_BACKTRACE=1 ~/tmp/adventofcode2016)')

// use regex::Regex;
// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::fs::File;
// use std::io::{BufRead, BufReader};
// use std::io::{BufReader, BufRead};
// use std::io::{BufReader, Read};
// use std::iter::FromIterator;
// use std::rc::Rc;
//
// extern crate regex;

/*

///////////////// Day 1

use std::fs::File;
use std::io::Read;
use std::io::BufReader;

fn day1() {
let f = File::open("advent-files/day1-input.txt").expect("open file");
let mut br = BufReader::new(f);

let mut input = String::new();
br.read_to_string(&mut input).unwrap();

let result = input.trim().chars().enumerate().fold(0, |floor, (idx, ch)| {
let new_floor = match ch {
'(' => { floor + 1 }
')' => { floor - 1 }
_ => { panic!("Invalid input: {}", ch) }
        };

        if floor == 0 && new_floor == -1 {
            println!("Entered the basement at position: {}", idx + 1);
        }

        new_floor
    });

    println!("Final floor: {}", result);
}

///////////////// Day 2

use std::fs::File;
use std::io::{BufRead, BufReader};

fn day2_pt1() {
    let f = File::open("advent-files/day2-input.txt").expect("open file");
    let br = BufReader::new(f);

    let mut total = 0;

    for line in br.lines().map(Result::unwrap) {
        let dimensions: Vec<usize> = line.split("x").map(str::parse).map(Result::unwrap).collect();
        let mut sides = Vec::new();

        let w = dimensions[0];
        let h = dimensions[1];
        let d = dimensions[2];

        sides.push(w * h);
        sides.push(w * d);
        sides.push(h * d);

        total += sides.iter().fold(0, |total, area| total + (area * 2)) + sides.iter().min().unwrap()
    }

    println!("{}", total);
}

fn day2_pt2() {
    let f = File::open("advent-files/day2-input.txt").expect("open file");
    let br = BufReader::new(f);

    let mut total = 0;

    for line in br.lines().map(Result::unwrap) {
        let mut dimensions: Vec<usize> = line.split("x").map(str::parse).map(Result::unwrap).collect();
        dimensions.sort();

        total += (dimensions[0] * 2 + dimensions[1] * 2) + (dimensions[0] * dimensions[1] * dimensions[2])
    }

    println!("Ribbon: {}", total);
}

fn day2() {
    day2_pt1();
    day2_pt2();
}

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::{BufReader};


#[derive(Eq, PartialEq, Hash, Clone)]
struct Position {
    x: i64,
    y: i64,
}

fn day3_pt1() {
    let f = File::open("advent-files/day3-input.txt").expect("open file");
    let mut br = BufReader::new(f);

    let mut input = String::new();
    br.read_to_string(&mut input).unwrap();

    let mut houses: HashMap<Position, i64> = HashMap::new();

    let mut current_position = Position { x: 0, y: 0 };

    houses.insert(current_position.clone(), 1);

    for ch in input.trim().chars() {
        match ch {
            '^' => { current_position.y -= 1 },
            '>' => { current_position.x += 1 },
            'v' => { current_position.y += 1},
            '<' => { current_position.x -= 1 },
            _ => { panic!("Parse error") },
        }

        let mut entry = houses.entry(current_position.clone()).or_insert(0);
        *entry += 1;
    }

    println!("Houses: {}", houses.keys().len());
}

fn day3_pt2() {
    let f = File::open("advent-files/day3-input.txt").expect("open file");
    let mut br = BufReader::new(f);

    let mut input = String::new();
    br.read_to_string(&mut input).unwrap();

    let mut houses: HashMap<Position, i64> = HashMap::new();

    for &santa in &[0, 1] {
        let mut current_position = Position { x: 0, y: 0 };

        {
            let mut entry = houses.entry(current_position.clone()).or_insert(0);
            *entry += 1;
        }

        for (idx, ch) in input.trim().chars().enumerate() {
            if (idx % 2) != santa {
                continue;
            }

            match ch {
                '^' => { current_position.y -= 1 },
                '>' => { current_position.x += 1 },
                'v' => { current_position.y += 1},
                '<' => { current_position.x -= 1 },
                _ => { panic!("Parse error") },
            }

            let mut entry = houses.entry(current_position.clone()).or_insert(0);
            *entry += 1;
        }
    }

    println!("Houses: {}", houses.keys().len());
}

fn day3() {
    day3_pt1();
    day3_pt2();
}


extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;


fn day4() {

    let key = "bgvyzdsv";

    let mut i = 1;

    let mut md5 = Md5::new();
    let mut out = vec![0; md5.output_bytes()];

    loop {
        md5.reset();
        md5.input_str(key);
        md5.input_str(&i.to_string());

        md5.result(&mut out);

        // Six leading zeroes...
        if out[0] == 0 && out[1] == 0 && out[2] == 0 {
            println!("Key is {}", i);
            break;
        }

        i += 1;
    }
}

use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_nice(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let mut has_pair = false;
    let mut has_triple = false;

    // Crazy N^3 but inputs are small...
    for i in 0..chars.len() {
        for j in i..chars.len() - 1 {
            if !has_pair {
                for k in j+2..chars.len() - 1 {
                    if chars[j] == chars[k] && chars[j + 1] == chars[k + 1] {
                        has_pair = true;
                    }
                }
            }

            if !has_triple && j + 2 < chars.len() {
                if chars[j] == chars[j + 2] {
                    has_triple = true;
                }
            }
        }
    }

    has_pair && has_triple
}

fn day5() {
    let f = File::open("advent-files/day5-input.txt").expect("open file");
    let br = BufReader::new(f);

    let mut nice = 0;

    for line in br.lines().map(Result::unwrap) {
        if is_nice(&line) {
            nice += 1;
        }
    }

    println!("Nice strings: {}", nice);
}

extern crate regex;
use regex::{Regex, Captures};
use std::fs::File;
use std::io::{BufRead, BufReader};


const GRID_SIZE : usize = 1000;

fn parse_coords(captures: Captures) -> Vec<usize> {
    captures.iter().skip(1).map(|s| {
        s.unwrap().as_str().parse().unwrap()
    }).collect()
}

fn day6() {
    let mut grid : Vec<Vec<i32>> = (0..GRID_SIZE).map(|_| vec![0; GRID_SIZE]).collect();

    let f = File::open("advent-files/day6-input.txt").expect("open file");
    let br = BufReader::new(f);

    let turn_on = Regex::new("turn on ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)").unwrap();
    let turn_off = Regex::new("turn off ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)").unwrap();
    let toggle = Regex::new("toggle ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)").unwrap();

    for line in br.lines().map(Result::unwrap) {
        if let Some(captures) = turn_on.captures(&line) {
            let coords = parse_coords(captures);
            let (x1, y1) = (coords[0], coords[1]);
            let (x2, y2) = (coords[2], coords[3]);

            for x in x1..x2+1 {
                for y in y1..y2+1 {
                    grid[y][x] += 1;
                }
            }
        } else if let Some(captures) = turn_off.captures(&line) {
            let coords = parse_coords(captures);
            let (x1, y1) = (coords[0], coords[1]);
            let (x2, y2) = (coords[2], coords[3]);

            for x in x1..x2+1 {
                for y in y1..y2+1 {
                    grid[y][x] -= 1;
                    if grid[y][x] < 0 {
                        grid[y][x] = 0;
                    }
                }
            }
        } else if let Some(captures) = toggle.captures(&line) {
            let coords = parse_coords(captures);
            let (x1, y1) = (coords[0], coords[1]);
            let (x2, y2) = (coords[2], coords[3]);

            for x in x1..x2+1 {
                for y in y1..y2+1 {
                    grid[y][x] += 2;
                }
            }

        } else {
            panic!("Parse error: {}", line);
        }
    }

    println!("Brightness: {}",
             grid.iter().map(|row| row.iter().sum::<i32>()).sum::<i32>());
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn day7_pt1() {
    let mut wires: HashMap<String, i64> = HashMap::new();

    // Load numbers as self-values
    for i in 0..65536 {
        wires.insert(i.to_string(), i);
    }

    loop {
        let f = File::open("advent-files/day7-input.txt").expect("open file");
        let br = BufReader::new(f);
        let mut progressed = false;

        for line in br.lines().map(Result::unwrap) {
            let bits: Vec<String> = line.split(" ").map(str::to_string).collect();

            match bits.len() {
                3 => {
                    let (source, target) = (bits[0].clone(), bits[2].clone());

                    if wires.contains_key(&target) {
                        continue;
                    }

                    match source.parse() {
                        Ok(number) => {
                            // Simple assignment like 123 -> x
                            wires.insert(target.clone(), number);
                            progressed = true;
                        },
                        _ => {
                            // Variable assignment like y -> x.  If y is available...
                            if wires.contains_key(&source) {
                                let value = *(wires.entry(source.clone()).or_insert(0));
                                wires.insert(target.clone(), value);
                                progressed = true;
                            }
                        }
                    }
                },
                4 => {
                    let (operator, source, target) = (bits[0].clone(), bits[1].clone(), bits[3].clone());

                    if wires.contains_key(&target) {
                        continue;
                    }

                    // Unary
                    assert_eq!(operator, "NOT");
                    if wires.contains_key(&source.clone()) {
                        let mut source_value = *(wires.entry(source.clone()).or_insert(0));
                        wires.insert(target.clone(), !source_value);
                        progressed = true;
                    }
                },
                5 => {
                    if wires.contains_key(&bits[4]) {
                        continue;
                    }

                    let (source1, operator, source2, target) = (bits[0].clone(), bits[1].clone(), bits[2].clone(), bits[4].clone());

                    // Binary
                    match operator.as_ref() {
                        "AND" => {
                            if wires.contains_key(&source1) && wires.contains_key(&source2) {
                                let wire1 = *(wires.entry(source1.clone()).or_insert(0));
                                let wire2 = *(wires.entry(source2.clone()).or_insert(0));
                                let target = wires.entry(target.clone()).or_insert(0);

                                *target = wire1 & wire2;
                                progressed = true;
                            }
                        },
                        "OR" => {
                            if wires.contains_key(&source1) && wires.contains_key(&source2) {
                                let wire1 = *(wires.entry(source1.clone()).or_insert(0));
                                let wire2 = *(wires.entry(source2.clone()).or_insert(0));
                                let target = wires.entry(target.clone()).or_insert(0);

                                *target = wire1 | wire2;
                                progressed = true;
                            }
                        },
                        "LSHIFT" => {
                            if wires.contains_key(&source1) {
                                let wire = *(wires.entry(source1.clone()).or_insert(0));
                                let places: usize = source2.parse().unwrap();
                                let target = wires.entry(target.clone()).or_insert(0);

                                *target = wire << places;
                                progressed = true;
                            }
                        },
                        "RSHIFT" => {
                            if wires.contains_key(&source1) {
                                let wire = *(wires.entry(source1.clone()).or_insert(0));
                                let places: usize = source2.parse().unwrap();
                                let target = wires.entry(target.clone()).or_insert(0);

                                *target = wire >> places;
                                progressed = true;
                            }
                        },
                        _ => { panic!("Invalid binary operator: {}", line); },
                    }
                },
                _ => { panic!("Unrecognized line: {}", line) }
            }
        }

        if !progressed {
            break;
        }
    }

    println!("Part 1: {:?}", wires.get("a").unwrap());
}

fn day7_pt2(){
    let mut wires: HashMap<String, i64> = HashMap::new();

    // Load numbers as self-values
    for i in 0..65536 {
        wires.insert(i.to_string(), i);
    }

    wires.insert("b".to_string(), 956);

    loop {
        let f = File::open("advent-files/day7-input.txt").expect("open file");
        let br = BufReader::new(f);
        let mut progressed = false;

        for line in br.lines().map(Result::unwrap) {
            let bits: Vec<String> = line.split(" ").map(str::to_string).collect();

            if bits[bits.len() - 1] == "b" {
                // We're overriding this.  Skip it.
                continue;
            }

            match bits.len() {
                3 => {
                    let (source, target) = (bits[0].clone(), bits[2].clone());

                    if wires.contains_key(&target) {
                        continue;
                    }

                    match source.parse() {
                        Ok(number) => {
                            // Simple assignment like 123 -> x
                            wires.insert(target.clone(), number);
                            progressed = true;
                        },
                        _ => {
                            // Variable assignment like y -> x.  If y is available...
                            if wires.contains_key(&source) {
                                let value = *(wires.entry(source.clone()).or_insert(0));
                                wires.insert(target.clone(), value);
                                progressed = true;
                            }
                        }
                    }
                },
                4 => {
                    let (operator, source, target) = (bits[0].clone(), bits[1].clone(), bits[3].clone());

                    if wires.contains_key(&target) {
                        continue;
                    }

                    // Unary
                    assert_eq!(operator, "NOT");
                    if wires.contains_key(&source.clone()) {
                        let mut source_value = *(wires.entry(source.clone()).or_insert(0));
                        wires.insert(target.clone(), !source_value);
                        progressed = true;
                    }
                },
                5 => {
                    if wires.contains_key(&bits[4]) {
                        continue;
                    }

                    let (source1, operator, source2, target) = (bits[0].clone(), bits[1].clone(), bits[2].clone(), bits[4].clone());

                    // Binary
                    match operator.as_ref() {
                        "AND" => {
                            if wires.contains_key(&source1) && wires.contains_key(&source2) {
                                let wire1 = *(wires.entry(source1.clone()).or_insert(0));
                                let wire2 = *(wires.entry(source2.clone()).or_insert(0));
                                let target = wires.entry(target.clone()).or_insert(0);

                                *target = wire1 & wire2;
                                progressed = true;
                            }
                        },
                        "OR" => {
                            if wires.contains_key(&source1) && wires.contains_key(&source2) {
                                let wire1 = *(wires.entry(source1.clone()).or_insert(0));
                                let wire2 = *(wires.entry(source2.clone()).or_insert(0));
                                let target = wires.entry(target.clone()).or_insert(0);

                                *target = wire1 | wire2;
                                progressed = true;
                            }
                        },
                        "LSHIFT" => {
                            if wires.contains_key(&source1) {
                                let wire = *(wires.entry(source1.clone()).or_insert(0));
                                let places: usize = source2.parse().unwrap();
                                let target = wires.entry(target.clone()).or_insert(0);

                                *target = wire << places;
                                progressed = true;
                            }
                        },
                        "RSHIFT" => {
                            if wires.contains_key(&source1) {
                                let wire = *(wires.entry(source1.clone()).or_insert(0));
                                let places: usize = source2.parse().unwrap();
                                let target = wires.entry(target.clone()).or_insert(0);

                                *target = wire >> places;
                                progressed = true;
                            }
                        },
                        _ => { panic!("Invalid binary operator: {}", line); },
                    }
                },
                _ => { panic!("Unrecognized line: {}", line) }
            }
        }

        if !progressed {
            break;
        }
    }

    println!("Part 1: {:?}", wires.get("a").unwrap());
}


fn day7() {
    day7_pt1();
    day7_pt2();
}


use std::fs::File;
use std::io::{BufRead, BufReader};

fn encode(s: String) -> String {
    let mut result = String::with_capacity(s.len());

    result.push('"');

    for ch in s.chars() {
        match ch {
            '"' => {
                result.push('\\');
                result.push('"');
            },
            '\\' => {
                result.push('\\');
                result.push('\\');
            },
            _ => {
                result.push(ch);
            }
        }
    }

    result.push('"');

    result
}

fn day8() {
    let f = File::open("advent-files/day8-input.txt").expect("open file");
    let br = BufReader::new(f);

    let mut original_length = 0;
    let mut encoded_length = 0;

    for line in br.lines().map(Result::unwrap) {
        original_length += line.len();
        encoded_length += encode(line).len();
    }

    println!("{}", encoded_length - original_length);
}


use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Hash, Eq, PartialEq)]
struct Route {
    from: String,
    to: String,
}

fn all_permutations<T: Clone>(elts: Vec<T>) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = Vec::new();
    result.push(Vec::new());

    for i in (0..elts.len()).rev() {
        let elt = elts.get(i).unwrap();
        let mut expanded = Vec::new();

        while !result.is_empty() {
            let candidate = result.remove(0);

            for pos in 0..candidate.len() + 1 {
                // Insert 'elt' at every possible position
                let mut permutation = candidate.clone();
                permutation.insert(pos, elt.clone());
                expanded.push(permutation);
            }
        }

        result = expanded;
    }

    result
}

fn day9() {
    let mut distances: HashMap<Route, usize> = HashMap::new();
    let mut locations: HashSet<String> = HashSet::new();

    let f = File::open("advent-files/day9-input.txt").expect("open file");
    let br = BufReader::new(f);

    for line in br.lines().map(Result::unwrap) {
        let bits: Vec<String> = line.split(' ').map(str::to_owned).collect();

        let (from, to, distance): (_, _, usize) = (bits[0].clone(), bits[2].clone(), bits[4].parse().unwrap());

        locations.insert(from.clone());
        locations.insert(to.clone());
        distances.insert(Route { from: from.clone(), to: to.clone() }, distance);
        distances.insert(Route { to: from.clone(), from: to.clone() }, distance);
    }

    let mut best = std::i64::MIN;

    for permutation in all_permutations(locations.iter().collect()) {
        let mut distance_travelled: i64 = 0;

        for i in 0..permutation.len() - 1 {
            let route = Route { from: permutation[i].clone(), to: permutation[i + 1].clone() };
            distance_travelled += *distances.get(&route).unwrap() as i64;
        }

        if distance_travelled > best {
            best = distance_travelled;
        }
    }

    println!("Best distance: {}", best);
}

fn look_and_say(s: String) -> String {
    let mut result = String::with_capacity(s.len());

    let chars = s.chars().collect::<Vec<char>>();

    let mut i = 0;
    while i < chars.len() {
        let mut run_length = 1;
        let ch = chars[i];

        let mut j = i + 1;
        while j < chars.len() && chars[j] == ch {
            run_length += 1;
            j += 1
        }

        i += run_length;
        result.push_str(&run_length.to_string());
        result.push(ch);
    }

    result
}

fn day10() {
    let mut input = "3113322113".to_owned();

    for _ in 0..50 {
        input = look_and_say(input);
    }

    println!("{}", input.len());
}


use std::collections::HashSet;

const PASSWORD_LETTERS: &str = "abcdefghjkmnpqrstuvwxyz";

fn is_good_password(password: &Vec<char>) -> bool {
    let codes: Vec<i64> = password.iter().map(|&ch| ch as i64).collect();
    let mut has_run = false;
    let mut pairs: HashSet<char> = HashSet::new();

    for i in 0..codes.len() - 2 {
        if (codes[i + 1] - codes[i]) == 1 && (codes[i + 2] - codes[i + 1]) == 1 {
            has_run = true;
            break
        }
    }

    for i in 0..codes.len() - 1 {
        if codes[i] == codes[i + 1] {
            pairs.insert(password[i]);

            if pairs.len() == 2 {
                break;
            }
        }
    }

    has_run && pairs.len() == 2
}

fn tick_password(password: &mut Vec<char>, password_chars: &Vec<char>) {
    let mut i: i64 = (password.len() - 1) as i64;
    while i >= 0 {
        let old_ch = password[i as usize];
        let pos = password_chars.iter().position(|&ch| ch == old_ch).unwrap();

        password[i as usize] = password_chars[(pos + 1) % password_chars.len()];

        if pos < password_chars.len() - 1 {
            break;
        }

        i -= 1;
    }
}


fn next_password(start_password: &str) -> String {
    let password_chars: Vec<char> = PASSWORD_LETTERS.chars().collect();
    let mut password: Vec<char> = start_password.chars().collect();

    loop {
        tick_password(&mut password, &password_chars);

        if is_good_password(&password) {
            break;
        }
    }

    password.iter().collect()
}

fn day11() {
    println!("Next password: {}", next_password("hxbxwxba"));
    println!("Next next password: {}", next_password("hxbxxyzz"));
}

use std::fs::File;
use std::io::{BufReader, Read};

extern crate serde_json;
use serde_json::Value;

fn sum_numbers_pt1(initial_value: Value) -> i64 {
    let mut result: i64 = 0;
    let mut queue: Vec<Value> = vec![initial_value];

    while let Some(v) = queue.pop() {
        match v {
            Value::Number(n) => result += n.as_i64().unwrap(),
            Value::Array(values) => queue.extend(values),
            Value::Object(obj) => queue.extend(obj.values().cloned()),
            _ => {}
        }
    }

    result
}

fn sum_numbers_pt2(initial_value: Value) -> i64 {
    let mut total: i64 = 0;
    let mut queue: Vec<Value> = vec![initial_value];

    while let Some(v) = queue.pop() {
        match v {
            Value::Number(n) => total += n.as_i64().unwrap(),
            Value::Array(values) => queue.extend(values),
            Value::Object(obj) => {
                if (obj.values()
                    .find(|&value| value.is_string() && value.as_str().unwrap() == "red"))
                    .is_none()
                {
                    queue.extend(obj.values().cloned().collect::<Vec<Value>>())
                }
            }
            _ => {}
        }
    }

    total
}

fn day12() {
    let f = File::open("advent-files/day12-input.txt").expect("open file");
    let mut br = BufReader::new(f);

    let mut input = String::new();
    br.read_to_string(&mut input).unwrap();

    let v: Value = serde_json::from_str(&input).unwrap();

    println!("Summed numbers: {}", sum_numbers_pt1(v.clone()));
    println!("Summed numbers (pt2): {}", sum_numbers_pt2(v.clone()));
}


use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn all_permutations<T: Clone>(elts: Vec<T>) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = Vec::new();
    result.push(Vec::new());

    for i in (0..elts.len()).rev() {
        let elt = elts.get(i).unwrap();
        let mut expanded = Vec::new();

        while !result.is_empty() {
            let candidate = result.remove(0);

            for pos in 0..candidate.len() + 1 {
                // Insert 'elt' at every possible position
                let mut permutation = candidate.clone();
                permutation.insert(pos, elt.clone());
                expanded.push(permutation);
            }
        }

        result = expanded;
    }

    result
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Pairing {
    subject_person: String,
    object_person: String,
}

fn day13() {
    let f = File::open("advent-files/day13-input.txt").expect("open file");
    let br = BufReader::new(f);

    let mut pairing_costs: HashMap<Pairing, i64> = HashMap::new();

    for line in br.lines().map(Result::unwrap) {
        let bits: Vec<String> = line.split(" ").map(str::to_owned).collect();

        let pairing = Pairing { subject_person: bits[0].clone(), object_person: bits[10].clone() };
        let magnitude: i64 = bits[3].parse().unwrap();

        let multiplier = match bits[2].as_ref() {
            "gain" => { 1 },
            "lose" => { -1 },
            _ => panic!("Parse error: {}", bits[2]),
        };

        pairing_costs.insert(pairing, multiplier * magnitude);
    }

    // 751: too high!
    let mut all_people: Vec<String> = pairing_costs
        .keys()
        .map(|pairing| pairing.subject_person.clone())
        .collect();

    for person in &all_people {
        pairing_costs.insert(Pairing { subject_person: "MarkTriggs".to_owned(),
                                       object_person: person.clone() },
                             0);

        pairing_costs.insert(Pairing { subject_person: person.clone(),
                                       object_person: "MarkTriggs".to_owned() },
                             0);
    }

    all_people.push("MarkTriggs".to_owned());

    all_people.sort();
    all_people.dedup();

    let mut happiest: i64 = std::i64::MIN;

    let people_count = all_people.len() as i64;

    for permutation in all_permutations(all_people) {
        let mut happiness = 0;

        for i in 0..permutation.len() {
            let subject = permutation[i].clone();
            let left_neighbour = permutation[((((i as i64 - 1) % people_count) + people_count) % people_count) as usize].clone();
            let right_neighbour = permutation[((((i as i64 + 1) % people_count) + people_count) % people_count) as usize].clone();

            happiness += *(pairing_costs.get(&Pairing { subject_person: subject.clone(), object_person: left_neighbour }).unwrap());
            happiness += *(pairing_costs.get(&Pairing { subject_person: subject.clone(), object_person: right_neighbour }).unwrap());
        }

        if happiness > happiest {
            happiest = happiness;
        }
    }

    println!("MOST HAPPY: {}", happiest);
}

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Reindeer {
    name: String,
    top_speed: usize,
    sprint_time: usize,
    rest_time: usize,

    distance_travelled: usize,
    points: usize,
}

fn day14_pt1() {
    let f = File::open("advent-files/day14-input.txt").expect("open file");
    let br = BufReader::new(f);

    let mut reindeer: Vec<Reindeer> = Vec::new();

    for line in br.lines().map(Result::unwrap) {
        let bits: Vec<String> = line.split(" ").map(str::to_owned).collect();

        reindeer.push(Reindeer {
            name: bits[0].clone(),
            top_speed: bits[3].parse().unwrap(),
            sprint_time: bits[6].parse().unwrap(),
            rest_time: bits[13].parse().unwrap(),

            distance_travelled: 0,
            points: 0,
        });
    }

    for i in 0..2503 {
        for mut r in &mut reindeer {
            if i % (r.sprint_time + r.rest_time) < r.sprint_time {
                r.distance_travelled += r.top_speed
            }
        }
    }

    println!("Winner: {:?}", reindeer.iter().max_by_key(|r| r.distance_travelled));
}

fn day14_pt2() {
    let f = File::open("advent-files/day14-input.txt").expect("open file");
    let br = BufReader::new(f);

    let mut reindeer: Vec<Reindeer> = Vec::new();

    for line in br.lines().map(Result::unwrap) {
        let bits: Vec<String> = line.split(" ").map(str::to_owned).collect();

        reindeer.push(Reindeer {
            name: bits[0].clone(),
            top_speed: bits[3].parse().unwrap(),
            sprint_time: bits[6].parse().unwrap(),
            rest_time: bits[13].parse().unwrap(),

            distance_travelled: 0,
            points: 0,
        });
    }

    for i in 0..2503 {
        for mut r in &mut reindeer {
            if i % (r.sprint_time + r.rest_time) < r.sprint_time {
                r.distance_travelled += r.top_speed;
            }
        }

        let max = reindeer.iter().map(|r| r.distance_travelled).max().unwrap();

        for r in &mut reindeer {
            if r.distance_travelled == max {
                r.points += 1;;
            }
        }
    }

    println!("Winner: {:?}", reindeer.iter().max_by_key(|r| r.points));
}

fn day14() {
    day14_pt1();
    day14_pt2();
}

use std::collections::HashSet;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Properties {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

#[derive(Clone, Debug)]
struct Ingredient {
    name: String,
    properties: Properties,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct IngredientSet {
    ingredients: Vec<String>,
}

impl IngredientSet {
    fn new() -> IngredientSet {
        IngredientSet { ingredients: Vec::new() }
    }

    fn push(&self, ingredient: String) -> IngredientSet {
        let mut result = self.ingredients.clone();
        result.push(ingredient);
        result.sort();

        IngredientSet { ingredients: result }
    }
}


#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Bowl {
    ingredients: IngredientSet,
    properties: Properties,
}

fn score(bowl: &Bowl) -> i64 {
    if bowl.properties.capacity < 0 ||
        bowl.properties.durability < 0 ||
        bowl.properties.flavor < 0 ||
        bowl.properties.texture < 0 {
            return 0;
        }

    bowl.properties.capacity * bowl.properties.durability * bowl.properties.flavor * bowl.properties.texture
}

fn add_to_bowl(bowl: &Bowl, ingredient: &Ingredient) -> Bowl {
    Bowl {
        ingredients: bowl.ingredients.push(ingredient.name.clone()),
        properties: properties_combine(&bowl.properties, &ingredient.properties),
    }
}

fn properties_combine(a: &Properties, b: &Properties) -> Properties {
    Properties {
        capacity: a.capacity + b.capacity,
        durability: a.durability + b.durability,
        flavor: a.flavor + b.flavor,
        texture: a.texture + b.texture,
        calories: a.calories + b.calories,
    }
}

fn day15_pt1() {
    let mut ingredients: Vec<Ingredient> = Vec::new();

    // ingredients.push(Ingredient {
    //     name: "Butterscotch".to_owned(),
    //     properties: Properties {
    //         capacity: -1,
    //         durability: -2,
    //         flavor: 6,
    //         texture: 3,
    //         calories: 8,
    //     },
    // });
    //
    // ingredients.push(Ingredient {
    //     name: "Cinnamon".to_owned(),
    //     properties: Properties {
    //         capacity: 2,
    //         durability: 3,
    //         flavor: -2,
    //         texture: -1,
    //         calories: 3,
    //     },
    // });


    ingredients.push(Ingredient {
        name: "Sprinkles".to_owned(),
        properties: Properties {
            capacity: 5,
            durability: -1,
            flavor: 0,
            texture: 0,
            calories: 5
        },
    });
    ingredients.push(Ingredient {
        name: "PeanutButter".to_owned(),
        properties: Properties {
            capacity: -1,
            durability: 3,
            flavor: 0,
            texture: 0,
            calories: 1
        },
    });
    ingredients.push(Ingredient {
        name: "Frosting".to_owned(),
        properties: Properties {
            capacity: 0,
            durability: -1,
            flavor: 4,
            texture: 0,
            calories: 6
        },
    });
    ingredients.push(Ingredient {
        name: "Sugar".to_owned(),
        properties: Properties {
            capacity: -1,
            durability: 0,
            flavor: 0,
            texture: 2,
            calories: 8
        },
    });

    let mut candidates: HashSet<Bowl> = ingredients
        .iter()
        .map(|ingredient| Bowl {
            ingredients: {
                IngredientSet::new().push(ingredient.name.clone())
            },
            properties: ingredient.properties.clone()
        }).collect();

    for i in 0..99 {
        println!("Round {}", i);
        let mut new_candidates = Vec::new();
        for candidate in &candidates {
            let mut potential_bowls: Vec<(i64, Bowl)> = ingredients
                .iter()
                .map(|ingredient| {
                    let new_bowl = add_to_bowl(&candidate, &ingredient);
                    (score(&new_bowl), new_bowl)
                })
                .collect();

            potential_bowls.sort_by_key(|&(score, _)| -score);

            let best_score = potential_bowls[0].0;

            for &(_, ref bowl) in potential_bowls.iter().take_while(|&&(score, _)| score == best_score) {
                new_candidates.push(bowl.clone())
            }
        }

        new_candidates.sort_by_key(|candidate| -score(&candidate));
        let best_score = score(&new_candidates[0]);
        candidates = new_candidates.iter().take_while(|&candidate| score(&candidate) == best_score).cloned().collect();

        println!("{} candidates", candidates.len());
    }

    println!("Best score was: {}", score(candidates.iter().nth(0).unwrap()));
    println!("Recipe: {:?}", candidates.iter().nth(0).unwrap());
}

fn day15_pt2() {
    let mut ingredients: Vec<Ingredient> = Vec::new();

    ingredients.push(Ingredient {
        name: "Sprinkles".to_owned(),
        properties: Properties {
            capacity: 5,
            durability: -1,
            flavor: 0,
            texture: 0,
            calories: 5
        },
    });
    ingredients.push(Ingredient {
        name: "PeanutButter".to_owned(),
        properties: Properties {
            capacity: -1,
            durability: 3,
            flavor: 0,
            texture: 0,
            calories: 1
        },
    });
    ingredients.push(Ingredient {
        name: "Frosting".to_owned(),
        properties: Properties {
            capacity: 0,
            durability: -1,
            flavor: 4,
            texture: 0,
            calories: 6
        },
    });
    ingredients.push(Ingredient {
        name: "Sugar".to_owned(),
        properties: Properties {
            capacity: -1,
            durability: 0,
            flavor: 0,
            texture: 2,
            calories: 8
        },
    });

    let mut candidates: HashSet<Bowl> = ingredients
        .iter()
        .map(|ingredient| Bowl {
            ingredients: {
                IngredientSet::new().push(ingredient.name.clone())
            },
            properties: ingredient.properties.clone()
        }).collect();

    for i in 0..99 {
        println!("Round {}", i);
        let mut new_candidates = Vec::new();
        for candidate in &candidates {
            for ingredient in &ingredients {
                let new_bowl = add_to_bowl(&candidate, &ingredient);

                if new_bowl.properties.calories <= 500 {
                    new_candidates.push(new_bowl)
                }
            }
        }

        candidates = new_candidates.iter().cloned().collect::<HashSet<Bowl>>();

        println!("{} candidates", candidates.len());
    }

    println!("Best score was: {}", candidates.iter().filter(|bowl| bowl.properties.calories == 500).map(score).max().unwrap());
}

fn day15() {
    day15_pt1();
    day15_pt2();
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};


    // children: usize,
    // cats: usize,
    // samoyeds: usize,
    // pomeranians: usize,
    // akitas: usize,
    // vizslas: usize,
    // goldfish: usize,
    // trees: usize,
    // cars: usize,
    // perfumes: usize,

#[derive(Debug)]
struct Sue {
    name: String,
    properties: HashMap<String, usize>,
}

fn day16_pt1() {
    let f = File::open("advent-files/day16-input.txt").expect("open file");
    let br = BufReader::new(f);

    let mut sues: Vec<Sue> = Vec::new();

    for line in br.lines().map(Result::unwrap) {
        let mut bits: Vec<String> = line.replace(":", "").replace(",", "").split(" ").map(str::to_owned).collect();

        bits.remove(0);
        let sue_name = bits.remove(0);
        let mut sue =  Sue { name: sue_name, properties: HashMap::new() };

        while !bits.is_empty() {
            let prop = bits.remove(0);
            let value = bits.remove(0);
            sue.properties.insert(prop, value.parse().unwrap());
        }

        sues.push(sue)
    }

    let mut target_sue = Sue { name: "target".to_owned(), properties: HashMap::new() };

    target_sue.properties.insert("children".to_owned(), 3);
    target_sue.properties.insert("cats".to_owned(), 7);
    target_sue.properties.insert("samoyeds".to_owned(), 2);
    target_sue.properties.insert("pomeranians".to_owned(), 3);
    target_sue.properties.insert("akitas".to_owned(), 0);
    target_sue.properties.insert("vizslas".to_owned(), 0);
    target_sue.properties.insert("goldfish".to_owned(), 5);
    target_sue.properties.insert("trees".to_owned(), 3);
    target_sue.properties.insert("cars".to_owned(), 2);
    target_sue.properties.insert("perfumes".to_owned(), 1);

    let candidates: Vec<&Sue> = sues.iter().filter(|&sue| {
        sue.properties.iter().all(|(key, value)| {
            match target_sue.properties.get(key) {
                Some(v) => { v == value }
                None => { true }
            }
        })
    }).collect();

    println!("Candidates: {:?}", candidates);

}

fn day16_pt2() {
    let f = File::open("advent-files/day16-input.txt").expect("open file");
    let br = BufReader::new(f);

    let mut sues: Vec<Sue> = Vec::new();

    for line in br.lines().map(Result::unwrap) {
        let mut bits: Vec<String> = line.replace(":", "").replace(",", "").split(" ").map(str::to_owned).collect();

        bits.remove(0);
        let sue_name = bits.remove(0);
        let mut sue =  Sue { name: sue_name, properties: HashMap::new() };

        while !bits.is_empty() {
            let prop = bits.remove(0);
            let value = bits.remove(0);
            sue.properties.insert(prop, value.parse().unwrap());
        }

        sues.push(sue)
    }

    let mut target_sue = Sue { name: "target".to_owned(), properties: HashMap::new() };

    target_sue.properties.insert("children".to_owned(), 3);
    target_sue.properties.insert("cats".to_owned(), 7);
    target_sue.properties.insert("samoyeds".to_owned(), 2);
    target_sue.properties.insert("pomeranians".to_owned(), 3);
    target_sue.properties.insert("akitas".to_owned(), 0);
    target_sue.properties.insert("vizslas".to_owned(), 0);
    target_sue.properties.insert("goldfish".to_owned(), 5);
    target_sue.properties.insert("trees".to_owned(), 3);
    target_sue.properties.insert("cars".to_owned(), 2);
    target_sue.properties.insert("perfumes".to_owned(), 1);

    let candidates: Vec<&Sue> = sues.iter().filter(|&sue| {
        sue.properties.iter().all(|(key, value)| {
            if key == "cats" || key == "trees" {
                match target_sue.properties.get(key) {
                    Some(v) => { v < value }
                    None => { true }
                }
            } else if key == "pomeranians" || key == "goldfish" {
                match target_sue.properties.get(key) {
                    Some(v) => { v > value }
                    None => { true }
                }

            } else {
                match target_sue.properties.get(key) {
                    Some(v) => { v == value }
                    None => { true }
                }
            }
        })
    }).collect();

    println!("Candidates: {:?}", candidates);
}


fn day16() {
    day16_pt1();
    day16_pt2();
}

fn combinations_pt1(target: usize, containers: &mut Vec<usize>) -> usize {
    if target == 0 {
        return 1;
    }

    let mut count = 0;

    for (idx, &container) in containers.iter().enumerate() {
        if container <= target {
            let mut new_containers: Vec<usize> = containers.iter().skip(idx + 1).cloned().collect();
            count += combinations_pt1(target - container, &mut new_containers);
        }
    }

    count
}

fn combinations_pt2(target: usize, containers: &mut Vec<usize>, container_count: usize) -> Vec<usize> {
    if target == 0 {
        return vec!(container_count);
    }

    let mut result: Vec<usize> = Vec::new();

    for (idx, &container) in containers.iter().enumerate() {
        if container <= target {
            let mut new_containers: Vec<usize> = containers.iter().skip(idx + 1).cloned().collect();
            result.append(&mut combinations_pt2(target - container, &mut new_containers, container_count + 1));
        }
    }

    result
}

fn day17() {
    let mut containers = vec!(43, 3, 4, 10, 21, 44, 4, 6, 47, 41, 34, 17, 17, 44, 36, 31, 46, 9, 27, 38);
    println!("pt1 {}", combinations_pt1(150, &mut containers));

    let mut containers = vec!(43, 3, 4, 10, 21, 44, 4, 6, 47, 41, 34, 17, 17, 44, 36, 31, 46, 9, 27, 38);
    let mut container_counts = combinations_pt2(150, &mut containers, 0);
    container_counts.sort();

    println!("pt2 {:?}", container_counts.iter().take_while(|&&elt| elt == container_counts[0]).count());

}


*/

use std::fs::File;
use std::io::{BufRead, BufReader};

type Row = Vec<bool>;
type Grid = Vec<Row>;

fn next_grid_pt1(lights: &Grid) -> Grid {
    let mut result = lights.clone();

    for y in 0..lights.len() as i64 {
        let row = &lights[y as usize];
        for x in 0..row.len() as i64 {
            let mut neighbours_on_count = 0;

            for &xoffset in &[-1, 0, 1] {
                for &yoffset in &[-1, 0, 1] {
                    if xoffset == 0 && yoffset == 0 {
                        continue;
                    }

                    if ((x + xoffset) >= 0 && (x + xoffset) < row.len() as i64) &&
                        ((y + yoffset) >= 0 && (y + yoffset) < lights.len() as i64) {
                            if lights[(y + yoffset) as usize][(x + xoffset) as usize] {
                                neighbours_on_count += 1;
                            }
                        }
                }
            }

            if lights[y as usize][x as usize] {
                if neighbours_on_count != 2 && neighbours_on_count != 3 {
                    result[y as usize][x as usize] = false
                }
            } else {
                if neighbours_on_count == 3 {
                    result[y as usize][x as usize] = true
                }
            }
        }
    }

    result
}

fn next_grid_pt2(lights: &Grid) -> Grid {
    let mut result = lights.clone();

    for y in 0..lights.len() as i64 {
        let row = &lights[y as usize];
        for x in 0..row.len() as i64 {
            let mut neighbours_on_count = 0;

            for &xoffset in &[-1, 0, 1] {
                for &yoffset in &[-1, 0, 1] {
                    if xoffset == 0 && yoffset == 0 {
                        continue;
                    }

                    if ((x + xoffset) >= 0 && (x + xoffset) < row.len() as i64) &&
                        ((y + yoffset) >= 0 && (y + yoffset) < lights.len() as i64) {
                            if lights[(y + yoffset) as usize][(x + xoffset) as usize] {
                                neighbours_on_count += 1;
                            }
                        }
                }
            }

            if lights[y as usize][x as usize] {
                if neighbours_on_count != 2 && neighbours_on_count != 3 {
                    result[y as usize][x as usize] = false
                }
            } else {
                if neighbours_on_count == 3 {
                    result[y as usize][x as usize] = true
                }
            }
        }
    }

    // Corners always on
    result[0][0] = true;
    result[0][lights.len() - 1] = true;
    result[lights.len() - 1][0] = true;
    result[lights.len() - 1][lights.len() - 1] = true;

    result
}

fn day18_solve(next_grid: fn(&Grid) -> Grid) {
    let f = File::open("advent-files/day18-input.txt").expect("open file");
    let br = BufReader::new(f);

    let mut lights: Grid = br.lines().map(Result::unwrap).map(|line| {
        line.chars().map(|ch| ch == '#').collect()
    }).collect();

    for _ in 0..100 {
        lights = next_grid(&lights);
    }

    println!("There are {} lights on", lights.iter().map(|row| row.iter().filter(|&&ch| ch).count()).sum::<usize>());
}

fn day18() {
    day18_solve(next_grid_pt1);
    day18_solve(next_grid_pt2);
}

fn main() {
    // day1();
    // day2();
    // day3();
    // day4();
    // day5();
    // day6();
    // day7();
    // day8();
    // day9();
    // day10();
    // day11();
    // day12();
    // day13();
    // day14();
    // day15();
    // day16();
    // day17();

    day18();
}
