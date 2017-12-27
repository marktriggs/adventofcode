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


fn main() {
    day1();
}
