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

use std::collections::HashSet;

const DAY1_INPUT: &[&str] = &[
    "L1", "L3", "L5", "L3", "R1", "L4", "L5", "R1", "R3", "L5", "R1", "L3", "L2",
    "L3", "R2", "R2", "L3", "L3", "R1", "L2", "R1", "L3", "L2", "R4", "R2", "L5",
    "R4", "L5", "R4", "L2", "R3", "L2", "R4", "R1", "L5", "L4", "R1", "L2", "R3",
    "R1", "R2", "L4", "R1", "L2", "R3", "L2", "L3", "R5", "L192", "R4", "L5", "R4",
    "L1", "R4", "L4", "R2", "L5", "R45", "L2", "L5", "R4", "R5", "L3", "R5", "R77",
    "R2", "R5", "L5", "R1", "R4", "L4", "L4", "R2", "L4", "L1", "R191", "R1", "L1",
    "L2", "L2", "L4", "L3", "R1", "L3", "R1", "R5", "R3", "L1", "L4", "L2", "L3",
    "L1", "L1", "R5", "L4", "R1", "L3", "R1", "L2", "R1", "R4", "R5", "L4", "L2",
    "R4", "R5", "L1", "L2", "R3", "L4", "R2", "R2", "R3", "L2", "L3", "L5", "R3",
    "R1", "L4", "L3", "R4", "R2", "R2", "R2", "R1", "L4", "R4", "R1", "R2", "R1",
    "L2", "L2", "R4", "L1", "L2", "R3", "L3", "L5", "L4", "R4", "L3", "L1", "L5",
    "L3", "L5", "R5", "L5", "L4", "L2", "R1", "L2", "L4", "L2", "L4", "L1", "R4",
    "R4", "R5", "R1", "L4", "R2", "L4", "L2", "L4", "R2", "L4", "L1", "L2", "R1",
    "R4", "R3", "R2", "R2", "R5", "L1", "L2",
];

const _DAY1_SAMPLE_INPUT: &[&str] = &["R8", "R4", "R4", "R8"];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position(i64, i64);

fn position_add(p1: Position, p2: Position) -> Position {
    Position(p1.0 + p2.0, p1.1 + p2.1)
}

fn day1() {
    let compass = vec!(Position(0, 1), Position(1, 0), Position(0, -1), Position(-1, 0));

    let mut visited_locations = HashSet::new();

    let mut direction = 0;
    let mut pos = Position(0, 0);

    for step in DAY1_INPUT {
        direction = if step.starts_with("R") {
            ((direction as i64 + 1) as usize) % compass.len()
        } else {
            ((direction as i64 - 1) as usize) % compass.len()
        };

        let paces = step[1..].parse().unwrap();

        for _ in 0..paces {
            pos = position_add(pos, compass[direction]);
            if visited_locations.contains(&pos) {
                println!("Visited twice: {:?}", pos);
            } else {
                visited_locations.insert(pos.clone());
            }
        }
    }

    println!("Final position: {:?}", pos);
}


///////////////// Day 2

use std::fs::File;
use std::io::{BufReader, BufRead};


fn day2_pt1() {
    let f = File::open("advent-files/day2_input.txt").expect("open file");
    let br = BufReader::new(f);

    let keypad = [['1', '2', '3'],
                  ['4', '5', '6'],
                  ['7', '8', '9']];

    let mut x = 1;
    let mut y = 1;

    let mut result = Vec::new();

    for line in br.lines() {
        for ch in line.unwrap().chars() {
            match ch {
                'U' => {
                    if y > 0 { y -= 1; }
                },
                'D' => {
                    if y < 2 { y += 1; }
               },
                'L' => {
                    if x > 0 { x -= 1; }
                },
                'R' => {
                    if x < 2 { x += 1; }
                },
                _ => { panic!("Invalid input"); }
            }
        }

        result.push(keypad[y][x]);
    }

    println!("{:?}", result);
}

fn day2_pt2() {
    let f = File::open("advent-files/day2_input.txt").expect("open file");
    let br = BufReader::new(f);

    let keypad = [[' ', ' ', '1', ' ', ' '],
                  [' ', '2', '3', '4', ' '],
                  ['5', '6', '7', '8', '9'],
                  [' ', 'A', 'B', 'C', ' '],
                  [' ', ' ', 'D', ' ', ' ']];

    let mut x = 0;
    let mut y = 2;

    let mut result = Vec::new();

    for line in br.lines() {
        for ch in line.unwrap().chars() {
            match ch {
                'U' => {
                    if y > 0 && keypad[y - 1][x] != ' ' { y -= 1; }
                },
                'D' => {
                    if y < 4 && keypad[y + 1][x] != ' ' { y += 1; }
               },
                'L' => {
                    if x > 0 && keypad[y][x - 1] != ' ' { x -= 1; }
                },
                'R' => {
                    if x < 4 && keypad[y][x + 1] != ' ' { x += 1; }
                },
                _ => { panic!("Invalid input"); }
            }
        }

        result.push(keypad[y][x]);
    }

    println!("{:?}", result);
}

fn day2() {
    day2_pt1();
    day2_pt2();
}


///////////////// Day 3

#![feature(iterator_step_by)]

use std::fs::File;
use std::io::{BufReader, BufRead};


struct Triangle {
    a: usize,
    b: usize,
    c: usize,
}

impl Triangle {
    fn parse(line: String) -> Triangle {
        let edges: Vec<usize> = line.split(" ").map(|n| n.parse().unwrap()).collect();

        Triangle { a: edges[0], b: edges[1], c: edges[2] }
    }

    fn is_valid(&self) -> bool {
        self.a + self.b > self.c &&
        self.a + self.c > self.b &&
        self.c + self.b > self.a
    }
}

fn day3_pt1() {
    let f = File::open("advent-files/day3_input.txt").expect("open file");
    let br = BufReader::new(f);

    println!("Number of valid triangles: {}", br.lines().map(Result::ok).filter(|line| Triangle::parse(line.clone().unwrap()).is_valid()).count());
}

fn lines_iter<'a>() -> Box<Iterator<Item=usize>> {
    let iter = BufReader::new(File::open("advent-files/day3_input.txt").expect("open file"))
        .lines()
        .map(Result::unwrap)
        .flat_map(|line| line.split(" ").map(str::to_owned).collect::<Vec<_>>())
        .map(|n| n.parse::<usize>().unwrap());

    Box::new(iter)
}

fn day3_pt2() {
    let mut count = 0;

    for skip in 0..3 {
        let ns: Vec<usize> = lines_iter().skip(skip).step_by(3).collect();

        for edges in ns.chunks(3) {
            let t = Triangle { a: edges[0], b: edges[1], c: edges[2] };

            if t.is_valid() {
                count += 1;
            }
        }
    }

    println!("Count: {}", count);
}

fn day3() {
    day3_pt1();
    day3_pt2();
}


*/

extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;


fn calculate_checksum(s: &str) -> String {
    let mut frequencies = HashMap::new();

    for ch in s.chars() {
        let e = frequencies.entry(ch).or_insert(0);
        *e += 1;
    }

    let mut freq_a: Vec<(&char, &i64)> = frequencies.iter().collect();

    freq_a.sort_by_key(|k| (-k.1, k.0));

    freq_a.iter().take(5).map(|&(ch, _)| ch).collect::<String>()
}

fn day4_pt1() {
    let f = File::open("advent-files/day4_input.txt").expect("open file");
    let br = BufReader::new(f);

    let pat = Regex::new(r"([a-z-]+)-([0-9]+)\[([a-z]+)\]").unwrap();

    let mut total: i64 = 0;

    for line in br.lines() {
        if let Some(bits) = pat.captures(&line.unwrap()) {
            let name = &bits[1].replace("-", "");
            let sector_id = &bits[2];
            let checksum = &bits[3];

            if calculate_checksum(name) == checksum {
                total += sector_id.parse::<i64>().unwrap();
            }
        }
    }

    println!("Total: {}", total);
}

struct Room {
    name: String,
    sector_id: usize,
}

fn day4_pt2() {
    let f = File::open("advent-files/day4_input.txt").expect("open file");
    let br = BufReader::new(f);

    let pat = Regex::new(r"([a-z-]+)-([0-9]+)\[([a-z]+)\]").unwrap();

    let mut total: i64 = 0;

    let mut rooms = Vec::new();

    for line in br.lines() {
        if let Some(bits) = pat.captures(&line.unwrap()) {
            let name = &bits[1].replace("-", "");
            let sector_id = &bits[2];
            let checksum = &bits[3];

            if calculate_checksum(name) == checksum {
                rooms.push(Room { name: name.to_owned(), sector_id: sector_id.parse::<usize>().unwrap() });
            }
        }
    }

    for room in rooms {
        let mut decrypted_name = String::new();

        for ch in room.name.chars() {
            let code = (ch as u8) - 'a' as u8;

            decrypted_name.push(((((code as usize + room.sector_id) % 26) as u8) + 'a' as u8) as char);
        }

        println!("{}: {}", decrypted_name, room.sector_id);
    }
}


fn day4() {
    day4_pt1();
    day4_pt2();
}


fn main() {
    // day1();
    // day2();
    // day3();

    day4();
}
