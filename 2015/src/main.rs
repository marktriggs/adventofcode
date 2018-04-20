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

*/

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

fn main() {
    // day1();
    // day2();
    // day3();
    // day4();
    // day5();
    day6();
}
