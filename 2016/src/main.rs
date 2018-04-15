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

extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;


fn day5_pt1() {
    let input = "abbhdwsy";
    let mut i: usize = 0;

    let mut md5 = Md5::new();
    let mut out = vec![0; md5.output_bytes()];

    let mut password = Vec::new();

    loop {
        md5.reset();
        md5.input_str(input);
        md5.input_str(&i.to_string());

        md5.result(&mut out);

        if out[0] == 0 && out[1] == 0 && out[2] >> 4 == 0 {
            password.push(format!("{:x}", out[2] & 0xF));
            if password.len() == 8 {
                break;
            }
        }

        i += 1;
    }

    println!("The password is {}", password.join(""));
}

fn day5_pt2() {
    let input = "abbhdwsy";
    let mut i: usize = 0;

    let mut md5 = Md5::new();
    let mut out = vec![0; md5.output_bytes()];

    let mut password = vec!["".to_owned(); 8];
    let mut chars_found = 0;

    loop {
        md5.reset();
        md5.input_str(input);
        md5.input_str(&i.to_string());

        md5.result(&mut out);

        if out[0] == 0 && out[1] == 0 && out[2] >> 4 == 0 {
            let position = out[2] & 0xF;

            if position < 8 && password[position as usize] == "" {
                password[position as usize] = format!("{:x}", out[3] >> 4);
                chars_found += 1;

                if chars_found == 8 {
                    break;
                }
            }
        }

        i += 1;
    }

    println!("The password is {}", password.join(""));
}



fn day5() {
    day5_pt1();
    day5_pt2();
}

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;


fn day6_pt1() {
    let mut frequencies: Vec<HashMap<char, usize>> = (0..8).map(|_| HashMap::new()).collect();

    let f = File::open("advent-files/day6_input.txt").expect("open file");
    let br = BufReader::new(f);

    for line in br.lines().map(Result::unwrap) {
        for (i, ch) in line.chars().enumerate() {
            let e = frequencies[i].entry(ch).or_insert(0);
            *e += 1
        }
    }

    for i in 0..8 {
        let mut char_counts = frequencies[i].iter().collect::<Vec<(&char, &usize)>>();

        char_counts.sort_by_key(|&(_, count)| -(*count as i64));
        print!("{}", char_counts[0].0);
    }

    println!("");
}

fn day6_pt2() {
    let mut frequencies: Vec<HashMap<char, usize>> = (0..8).map(|_| HashMap::new()).collect();

    let f = File::open("advent-files/day6_input.txt").expect("open file");
    let br = BufReader::new(f);

    for line in br.lines().map(Result::unwrap) {
        for (i, ch) in line.chars().enumerate() {
            let e = frequencies[i].entry(ch).or_insert(0);
            *e += 1
        }
    }

    for i in 0..8 {
        let mut char_counts = frequencies[i].iter().collect::<Vec<(&char, &usize)>>();

        char_counts.sort_by_key(|&(_, count)| *count as i64);
        print!("{}", char_counts[0].0);
    }

    println!("");
}

fn day6() {
    day6_pt1();
    day6_pt2();
}

///////////////// Day 7

extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn outer_bit(addr: String) -> String {
    let pattern = Regex::new(r"\[.*?\]").unwrap();

    pattern.replace_all(&addr, " ").to_string()
}

fn inner_bit(addr: String) -> String {
    let pattern_start = Regex::new(r"^.*?\[").unwrap();
    let pattern_middle = Regex::new(r"\].*?\[").unwrap();
    let pattern_end = Regex::new(r"\].*?$").unwrap();

    [pattern_start, pattern_middle, pattern_end].iter().fold(addr, |a, pattern| {
        pattern.replace_all(&a, " ").into_owned()
    })
}


fn has_abba(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();

    for i in 0..((chars.len() - 4) + 1) {
        if chars[i] == chars[i + 3] && chars[i + 1] == chars[i + 2] && chars[i] != chars[i + 1] {
            return true;
        }
    }

    false
}

fn supports_tls(addr: String) -> bool {
    has_abba(&outer_bit(addr.clone())) && !has_abba(&inner_bit(addr.clone()))
}

fn supports_ssl(addr: String) -> bool {
    let outer = outer_bit(addr.clone()).chars().collect::<Vec<char>>();
    let inner = inner_bit(addr.clone()).chars().collect::<Vec<char>>();

    // N^2 yay!
    for i in 0..((outer.len() - 3) + 1) {
        if outer[i] != outer[i + 1] && outer[i] == outer[i + 2] {
            for j in 0..((inner.len() - 3) + 1) {
                if inner[j] == outer[i + 1] && inner[j + 2] == outer[i + 1] && inner[j + 1] == outer[i] {
                    return true;
                }
            }
        }
    }

    false
}

fn day7_pt1() {
    let mut count = 0;
    let f = File::open("advent-files/day7_input.txt").expect("open file");
    let br = BufReader::new(f);

    for line in br.lines().map(Result::unwrap) {
        if supports_tls(line.clone()) {
            count += 1;
        }
    }

    println!("Count was: {}", count);

}

fn day7_pt2() {
    let mut count = 0;
    let f = File::open("advent-files/day7_input.txt").expect("open file");
    let br = BufReader::new(f);

    for line in br.lines().map(Result::unwrap) {
        if supports_ssl(line.clone()) {
            count += 1;
        }
    }

    println!("Count was: {}", count);

}

fn day7() {
    day7_pt1();
    day7_pt2();
}

///////////////// Day 8

extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

const SCREEN_WIDTH: usize = 50;
const SCREEN_HEIGHT: usize = 6;

type Screen = Vec<Vec<char>>;

fn fill_rect(screen: &mut Screen, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            screen[y][x] = '#';
        }
    }
}

fn rotate_column(screen: &mut Screen, target_column: usize, offset: usize) {
    let values = (0..SCREEN_HEIGHT).map(|y| screen[y][target_column]).collect::<Vec<char>>();
    let mut rotated = values.iter().chain(values.iter()).skip(SCREEN_HEIGHT - offset).take(SCREEN_HEIGHT).cloned().collect::<Vec<char>>();

    for y in 0..SCREEN_HEIGHT {
        screen[y][target_column] = rotated.remove(0);
    }
}

fn rotate_row(screen: &mut Screen, target_row: usize, offset: usize) {
    let values = (0..SCREEN_WIDTH).map(|x| screen[target_row][x]).collect::<Vec<char>>();
    let mut rotated = values.iter().chain(values.iter()).skip(SCREEN_WIDTH - offset).take(SCREEN_WIDTH).cloned().collect::<Vec<char>>();

    for x in 0..SCREEN_WIDTH {
        screen[target_row][x] = rotated.remove(0);
    }
}

fn show(screen: &Screen) {
    for row in screen {
        println!("{}", row.iter().map(|&ch| if ch == '#' { '#' } else { ' '}).collect::<String>());
    }
}

fn day8() {
    let mut screen: Screen = (0..SCREEN_HEIGHT).map(|_| vec!['.'; SCREEN_WIDTH]).collect();

    let rect_command = Regex::new(r"rect (\d+)x(\d+)").unwrap();
    let row_rotate_command = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
    let column_rotate_command = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();

    let f = File::open("advent-files/day8_input.txt").expect("open file");
    let br = BufReader::new(f);

    for line in br.lines().map(Result::unwrap) {
        if let Some(args) = rect_command.captures(&line) {
            fill_rect(&mut screen, args[1].parse().unwrap(), args[2].parse().unwrap())
        } else if let Some(args) = row_rotate_command.captures(&line) {
            rotate_row(&mut screen, args[1].parse().unwrap(), args[2].parse().unwrap());
        } else if let Some(args) = column_rotate_command.captures(&line) {
            rotate_column(&mut screen, args[1].parse().unwrap(), args[2].parse().unwrap());
        }
    }

    show(&screen);

    println!("Lit pixels: {}", screen.iter().flat_map(|row| row.iter().filter(|&&ch| ch == '#')).count());
}


///////////////// Day 9

use std::fs::File;
use std::io::{Read, BufReader};

fn read_number(input: &mut Vec<char>) -> usize {
    let mut s = String::new();

    while !input.is_empty() {
        let ch = input[0];

        if ch.is_digit(10) {
            s.push(input.remove(0));
        } else {
            break;
        }
    }

    s.parse::<usize>().unwrap()
}

fn read_marker(input: &mut Vec<char>) -> (usize, usize) {
    input.remove(0);            // Skip (
    let len = read_number(input);
    input.remove(0);            // Skip x
    let repeats = read_number(input);
    input.remove(0);            // Skip )

    (len, repeats)
}

fn decompress_pt1(s: String) -> String {
    let mut result: Vec<char> = Vec::new();
    let mut input = s.chars().collect::<Vec<char>>();

    while !input.is_empty() {
        let ch = input[0];

        if ch == '(' {
            let (len, repeats) = read_marker(&mut input);

            for _ in 0..repeats {
                for i in 0..len {
                    result.push(input[i]);
                }
            }

            for _ in 0..len {
                input.remove(0);
            }
        } else {
            result.push(input.remove(0));
        }
    }

    result.iter().collect::<String>()
}

fn decompressed_length(s: String) -> usize {
    let mut input = s.chars().collect::<Vec<char>>();
    let mut result = 0;

    while !input.is_empty() {
        let ch = input[0];

        if ch == '(' {
            let (len, repeats) = read_marker(&mut input);

            let decompressed_len = decompressed_length(input[0..len].iter().collect::<String>());

            for _ in 0..len {
                input.remove(0);
            }

            result += repeats * decompressed_len
        } else {
            input.remove(0);
            result += 1;
        }
    }

    result
}


fn day9_pt1() {
    let f = File::open("advent-files/day9_input.txt").expect("open file");
    let mut br = BufReader::new(f);
    let mut input = String::new();
    br.read_to_string(&mut input).unwrap();

    input = input.trim().to_owned();
    println!("input length: {}", input.len());
    println!("Decompressed length: {}", decompress_pt1(input).len());
}

fn day9_pt2() {
    let f = File::open("advent-files/day9_input.txt").expect("open file");
    let mut br = BufReader::new(f);
    let mut input = String::new();
    br.read_to_string(&mut input).unwrap();

    input = input.trim().to_owned();

    println!("input length: {}", input.len());
    println!("Decompressed length: {}", decompressed_length(input));
}

fn day9() {
    day9_pt1();
    // 26606220610: too high
    day9_pt2();
}

///////////////// Day 10

extern crate regex;
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::collections::HashMap;


type BucketValues = HashMap<String, Vec<String>>;

#[derive(PartialEq, Eq, Debug)]
enum BotOrOutput {
    BOT,
    OUTPUT,
}

#[derive(Debug)]
struct Rule {
    low_to: String,
    low_type: BotOrOutput,
    high_to: String,
    high_type: BotOrOutput,
    hit: bool,
}

fn day10() {
    let simple_assignment = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
    let rule = Regex::new(r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)").unwrap();

    let mut bot_values: BucketValues = HashMap::new();
    let mut output_values: BucketValues = HashMap::new();
    let mut rules: HashMap<String, Rule> = HashMap::new();

    let f = File::open("advent-files/day10_input.txt").expect("open file");
    let br = BufReader::new(f);

    for line in br.lines().map(Result::unwrap) {
        if let Some(args) = simple_assignment.captures(&line) {
            let mut e = bot_values.entry(args[2].to_owned()).or_insert(Vec::new());
            e.push(args[1].to_owned());
        } else if let Some(args) = rule.captures(&line) {
            bot_values.entry(args[3].to_owned()).or_insert(Vec::new());
            bot_values.entry(args[5].to_owned()).or_insert(Vec::new());
            output_values.entry(args[3].to_owned()).or_insert(Vec::new());
            output_values.entry(args[5].to_owned()).or_insert(Vec::new());


            rules.insert(args[1].to_owned(),
                         Rule {
                             low_to: args[3].to_owned(),
                             low_type: if &args[2] == "bot" { BotOrOutput::BOT } else { BotOrOutput::OUTPUT },
                             high_to: args[5].to_owned(),
                             high_type: if &args[4] == "bot" { BotOrOutput::BOT } else { BotOrOutput::OUTPUT },
                             hit: false,
                         });
        }
    }

    println!("Rules: {:#?}", rules);

    let bots: Vec<String> = bot_values.keys().cloned().collect();

    loop {
        let mut progressed = false;

        for bot in &bots {
            if bot_values[bot].len() > 2 {
                panic!("Eh?");
            }

            if bot_values[bot].len() == 2 {
                let rule = rules.get_mut(bot).unwrap();

                if rule.hit {
                    continue;
                }

                progressed = true;

                // Apply our rule
                bot_values.get_mut(bot).unwrap().sort_by_key(|s| s.parse::<usize>().unwrap());
                let lower = &bot_values[bot][0].clone();
                let higher = &bot_values[bot][1].clone();


                if lower == "17" && higher == "61" {
                    println!("Target microchips handled by bot: {}", bot);
                }

                if rule.low_type == BotOrOutput::BOT {
                    bot_values.get_mut(&rule.low_to).unwrap().push(lower.clone());
                } else {
                    output_values.get_mut(&rule.low_to).unwrap().push(lower.clone());
                }

                if rule.high_type == BotOrOutput::BOT {
                    bot_values.get_mut(&rule.high_to).unwrap().push(higher.clone());
                } else {
                    output_values.get_mut(&rule.high_to).unwrap().push(higher.clone());
                }

                bot_values.get_mut(bot).unwrap().clear();

                rule.hit = true;

                break;
            }
        }

        if !progressed {
            break;
        }
    }

    println!("OUTPUT BINS: {:#?}", output_values);
    // println!("BOTS: {:#?}", bot_values);
}


///////////////// Day 11

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Hash)]
enum GizmoType {
    GENERATOR,
    MICROCHIP,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Hash)]
enum Chemical {
    THULIUM,
    PLUTONIUM,
    STRONTIUM,
    PROMETHIUM,
    RUTHENIUM,
    ELERIUM,
    DILITHIUM,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Hash)]
struct Gizmo {
    chemical: Chemical,
    gizmo_type: GizmoType
}

type Floor = HashSet<Gizmo>;

#[derive(Clone)]
struct State {
    elevator_position: usize,
    floors: Vec<Floor>,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "State {{ at_floor: {}, floors:\n{} }}",
               self.elevator_position,
               self.floors.iter().map(|floor| {
                   format!("[{}]",
                           floor.iter().map(|gizmo| format!("{:?}", gizmo)).collect::<Vec<String>>().join(", "))
               }).collect::<Vec<String>>().join("\n"))
    }
}


fn floor_is_valid(floor: &Floor) -> bool {
    for m in floor {
        if m.gizmo_type != GizmoType::MICROCHIP {
            continue;
        }

        let mut protected = false;
        let mut irradiated = false;

        for g in floor {
            if g.gizmo_type != GizmoType::GENERATOR {
                continue;
            }

            if g.chemical == m.chemical {
                protected = true;
            } else {
                irradiated = true;
            }
        }

        if !protected && irradiated {
            return false;
        }
    }

    true
}

fn next_states(state: &State, seen_states: &mut HashSet<String>) -> Vec<State> {
    let mut result: Vec<State> = Vec::new();

    for move_direction in [-1, 1].iter() {
        let current_elevator_position = state.elevator_position;
        let new_elevator_position = (current_elevator_position as i32) + move_direction;

        if new_elevator_position < 0 || new_elevator_position > 3 {
            continue;
        }

        let new_elevator_position = new_elevator_position as usize;

        for g1 in &state.floors[current_elevator_position] {
            for g2 in &state.floors[current_elevator_position] {

                if g2 < g1 {
                    continue;
                }

                let mut old_floor = state.floors[current_elevator_position].clone();
                let mut new_floor = state.floors[new_elevator_position].clone();

                if g1 == g2 {
                    old_floor.remove(g1);
                    new_floor.insert(g1.clone());
                } else {
                    old_floor.remove(g1);
                    old_floor.remove(g2);
                    new_floor.insert(g1.clone());
                    new_floor.insert(g2.clone());
                }

                if floor_is_valid(&old_floor) && floor_is_valid(&new_floor) {
                    // Possible new state!
                    let mut new_state = state.clone();

                    new_state.floors[current_elevator_position] = old_floor;
                    new_state.floors[new_elevator_position] = new_floor;
                    new_state = new_state;

                    new_state.elevator_position = new_elevator_position as usize;

                    let c = canonicalize(&new_state);

                    if !seen_states.contains(&c) {
                        seen_states.insert(c);
                        result.push(new_state);
                    }
                }
            }
        }
    }

    result
}

fn state_solved(state: &State) -> bool {
    (state.floors[0].len() + state.floors[1].len() + state.floors[2].len()) == 0
}

fn canonicalize(state: &State) -> String {
    let mut result = String::with_capacity(64);

    result.push_str("[");
    result.push_str(&state.elevator_position.to_string());
    result.push_str("]");

    let mut mapping: HashMap<&Chemical, usize> = HashMap::new();
    let mut count = 0;

    for floor in &state.floors {
        let mut codes: Vec<String> = Vec::new();

        for gizmo in floor {
            if !mapping.contains_key(&gizmo.chemical) {
                count += 1;
                mapping.insert(&gizmo.chemical, count);
            }

            if gizmo.gizmo_type == GizmoType::GENERATOR {
                codes.push(mapping.get(&gizmo.chemical).unwrap().to_string() + "G");
            } else {
                codes.push(mapping.get(&gizmo.chemical).unwrap().to_string() + "M");
            }
        }
        result.push_str("[");
        codes.sort();
        for c in codes.iter() {
            result.push_str(c);
            result.push_str(" ");
        }
        result.push_str("]");
    }

    result
}

fn day11_pt1_state() -> State {
    let mut state = State {
        elevator_position: 0,
        floors: Vec::new()
    };

    for _ in 0..4 {
        state.floors.push(Floor::new());
    }

    state.floors[0].insert(Gizmo { chemical: Chemical::THULIUM, gizmo_type: GizmoType::GENERATOR});
    state.floors[0].insert(Gizmo { chemical: Chemical::THULIUM, gizmo_type: GizmoType::MICROCHIP});
    state.floors[0].insert(Gizmo { chemical: Chemical::PLUTONIUM, gizmo_type: GizmoType::GENERATOR});
    state.floors[0].insert(Gizmo { chemical: Chemical::STRONTIUM, gizmo_type: GizmoType::GENERATOR});

    state.floors[1].insert(Gizmo { chemical: Chemical::PLUTONIUM, gizmo_type: GizmoType::MICROCHIP});
    state.floors[1].insert(Gizmo { chemical: Chemical::STRONTIUM, gizmo_type: GizmoType::MICROCHIP});

    state.floors[2].insert(Gizmo { chemical: Chemical::PROMETHIUM, gizmo_type: GizmoType::GENERATOR});
    state.floors[2].insert(Gizmo { chemical: Chemical::PROMETHIUM, gizmo_type: GizmoType::MICROCHIP});
    state.floors[2].insert(Gizmo { chemical: Chemical::RUTHENIUM, gizmo_type: GizmoType::GENERATOR});
    state.floors[2].insert(Gizmo { chemical: Chemical::RUTHENIUM, gizmo_type: GizmoType::MICROCHIP});

    state
}

fn day11_pt2_state() -> State {
    let mut state = State {
        elevator_position: 0,
        floors: Vec::new()
    };

    for _ in 0..4 {
        state.floors.push(Floor::new());
    }

    state.floors[0].insert(Gizmo { chemical: Chemical::THULIUM, gizmo_type: GizmoType::GENERATOR});
    state.floors[0].insert(Gizmo { chemical: Chemical::THULIUM, gizmo_type: GizmoType::MICROCHIP});
    state.floors[0].insert(Gizmo { chemical: Chemical::PLUTONIUM, gizmo_type: GizmoType::GENERATOR});
    state.floors[0].insert(Gizmo { chemical: Chemical::STRONTIUM, gizmo_type: GizmoType::GENERATOR});
    state.floors[0].insert(Gizmo { chemical: Chemical::ELERIUM, gizmo_type: GizmoType::GENERATOR});
    state.floors[0].insert(Gizmo { chemical: Chemical::ELERIUM, gizmo_type: GizmoType::MICROCHIP});
    state.floors[0].insert(Gizmo { chemical: Chemical::DILITHIUM, gizmo_type: GizmoType::GENERATOR});
    state.floors[0].insert(Gizmo { chemical: Chemical::DILITHIUM, gizmo_type: GizmoType::MICROCHIP});


    state.floors[1].insert(Gizmo { chemical: Chemical::PLUTONIUM, gizmo_type: GizmoType::MICROCHIP});
    state.floors[1].insert(Gizmo { chemical: Chemical::STRONTIUM, gizmo_type: GizmoType::MICROCHIP});

    state.floors[2].insert(Gizmo { chemical: Chemical::PROMETHIUM, gizmo_type: GizmoType::GENERATOR});
    state.floors[2].insert(Gizmo { chemical: Chemical::PROMETHIUM, gizmo_type: GizmoType::MICROCHIP});
    state.floors[2].insert(Gizmo { chemical: Chemical::RUTHENIUM, gizmo_type: GizmoType::GENERATOR});
    state.floors[2].insert(Gizmo { chemical: Chemical::RUTHENIUM, gizmo_type: GizmoType::MICROCHIP});

    state
}


fn day11_run(state: State) {
    let mut states: Vec<State> = vec!(state);
    let mut steps = 0;

    let mut seen_states: HashSet<String> = HashSet::new();

    while !states.iter().any(|state| state_solved(state)) {
        steps += 1;

        println!("Up to {} possible states", states.len());

        let mut new_states = Vec::new();

        for state in &states {
            new_states.extend(next_states(&state, &mut seen_states));
        }

        states = new_states;
    }

    println!("That took {} steps", steps);
    println!("");

}

fn day11() {
    day11_run(day11_pt1_state());
    day11_run(day11_pt2_state());
}

///////////////// Day 12

use std::collections::HashMap;

// todo: NOP?
const DAY12_INPUT: &str = "
cpy 1 a
cpy 1 b
cpy 26 d
jnz c 2
jnz 1 5
cpy 7 c
inc d
dec c
jnz c -2
cpy a c
inc a
dec b
jnz b -2
cpy c b
dec d
jnz d -6
cpy 19 c
cpy 11 d
inc a
dec d
jnz d -2
dec c
jnz c -5
";


fn to_register(name: &str) -> char {
    name.chars().nth(0).unwrap()
}

fn deref_value(value: &str, registers: &HashMap<char, i64>) -> i64 {
    match to_register(value) {
        r @ 'a'...'z' => {
            *registers.get(&r).unwrap()
        },
        _ => { value.parse().unwrap() }
    }
}


fn day12_pt1() {
    let instructions: Vec<&str> = DAY12_INPUT.trim().split("\n").map(|s| {
        if &s[0..1] == "#" {
            "nop"
        } else {
            s
        }}).collect();

    let mut registers = "abcd".chars().fold(HashMap::new(), |mut acc, register| {
        if register == 'c' {
            acc.insert(register, 1);
        } else {
            acc.insert(register, 0);
        }
        acc
    });

    let mut pc: i64 = 0;

    loop {
        if pc < 0 || pc >= (instructions.len() as i64) {
            break;
        }

        let instruction = instructions[pc as usize];

        if pc == 31 {
            println!("{}: {:?}", pc, instruction);
            println!("{:?}", &registers);
        }


        let bits: Vec<&str> = instruction.split(" ").collect();

        match bits[0] {
            "cpy" => {
                let value = deref_value(bits[1], &registers);
                registers.insert(to_register(bits[2]), value);
            },
            "nop" => {},
            "inc" => {
                let new_value = deref_value(bits[1], &registers) + 1;
                registers.insert(to_register(bits[1]), new_value);
            },
            "dec" => {
                let new_value = deref_value(bits[1], &registers) - 1;
                registers.insert(to_register(bits[1]), new_value);
            },
            "jnz" => {
                let x = deref_value(bits[1], &registers);
                let y = deref_value(bits[2], &registers);

                if x != 0 {
                    // Compensate for the increment we're going to get anyway.
                    pc -= 1;
                    pc += y;

                //println!("Post jump: {:?}", &registers);

                }
            },
            _ => { panic!("WTF?!"); },
        }

        pc += 1;
    }

    println!("Final state: {:?}", &registers);
}


fn day12() {
    day12_pt1();
}


///////////////// Day 13

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

// Build a lookup table for our bytes
lazy_static! {
    static ref BYTE_COUNTS: HashMap<u8, usize> = {
        let mut m = HashMap::new();

        for i in 0..256 {
            let mut byte = i as u8;

            let mut count = 0;

            while byte > 0 {
                if (byte & 1) == 1 {
                    count += 1;
                }

                byte = byte >> 1
            }

            m.insert(i as u8, count);
        }

        m
    };
}


fn count_set_bits(byte: u8) -> usize {
    *BYTE_COUNTS.get(&byte).unwrap()
}

fn is_wall(x: usize, y: usize) -> bool {
    let n = x*x + 3*x + 2*x*y + y + y*y + 1362;

    let bit_count = count_set_bits(n as u8) +
        count_set_bits((n >> 8) as u8) +
        count_set_bits((n >> 16) as u8) +
        count_set_bits((n >> 24) as u8);

    (bit_count % 2 != 0)
}

#[derive(Copy, Clone, Debug)]
struct Node(usize, usize);

fn surrounding_nodes(node: Node, walls: &Vec<Vec<bool>>) -> Vec<Node> {
    let mut result: Vec<Node> = Vec::new();

    for &(xoff, yoff) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
        let new_x = node.0 as i64 + xoff;
        let new_y = node.1 as i64 + yoff;

        if new_x >= 0 && new_y >= 0 &&
            (new_y as usize) < walls.len() && (new_x as usize) < walls[0].len() &&
            !walls[new_x as usize][new_y as usize] {
            result.push(Node(new_x as usize, new_y as usize));
        }
    }

    result
}

fn day13_pt1() {
    let size = 1000;

    let mut walls: Vec<Vec<bool>> = (0..size).map(|_row| vec!(false; size)).collect();
    let mut path_lengths: Vec<Vec<usize>> = (0..size).map(|_row| vec!(std::usize::MAX; size)).collect();

    for x in 0..size {
        for y in 0..size {
            walls[x][y] = is_wall(x, y);
        }
    }

    // We can get to our target from our target trivially
    path_lengths[31][39] = 0;

    let mut nodes_to_expand = vec!(Node(31, 39));

    while !nodes_to_expand.is_empty() {
        let node = nodes_to_expand.pop().unwrap();

        let surrounding = surrounding_nodes(node, &walls);
        let my_length = path_lengths[node.0][node.1];

        for Node(x, y) in surrounding {
            if my_length + 1 < path_lengths[x][y] {
                // println!("{}x{} was {} now {}", x, y, path_lengths[x][y], length);
                path_lengths[x][y] = my_length + 1;
                nodes_to_expand.push(Node(x, y));
            }
        }
    }

    println!("Shortest path: {}", path_lengths[1][1]);
}


fn day13_pt2() {
    let size = 1000;

    let mut walls: Vec<Vec<bool>> = (0..size).map(|_row| vec!(false; size)).collect();
    let mut path_lengths: Vec<Vec<usize>> = (0..size).map(|_row| vec!(std::usize::MAX; size)).collect();

    for x in 0..size {
        for y in 0..size {
            walls[x][y] = is_wall(x, y);
        }
    }

    // We can get to our target from our target trivially
    path_lengths[1][1] = 0;

    let mut nodes_to_expand = vec!(Node(1, 1));

    while !nodes_to_expand.is_empty() {
        let node = nodes_to_expand.pop().unwrap();

        let surrounding = surrounding_nodes(node, &walls);
        let my_length = path_lengths[node.0][node.1];

        for Node(x, y) in surrounding {
            if my_length + 1 < path_lengths[x][y] {
                // println!("{}x{} was {} now {}", x, y, path_lengths[x][y], length);
                path_lengths[x][y] = my_length + 1;
                nodes_to_expand.push(Node(x, y));
            }
        }
    }

    let mut count = 0;

    for row in path_lengths {
        for steps in row {
            if steps <= 50 {
                count += 1;
            }
        }
    }

    // 325 too high!
    println!("Reachable within 50 steps: {}", count);
}

fn day13() {
    day13_pt1();
    day13_pt2();
}

///////////////// Day 14

extern crate regex;
extern crate crypto;

#[macro_use]
extern crate lazy_static;


use std::collections::HashMap;
use crypto::md5::Md5;
use crypto::digest::Digest;
use regex::Regex;

lazy_static! {
    static ref TRIPLES_PATTERN: Regex = {
        Regex::new(r"(000|111|222|333|444|555|666|777|888|999|aaa|bbb|ccc|ddd|eee|fff)").unwrap()
    };

    static ref QUINTUPLET_PATTERNS: HashMap<String, Regex> = {
        let mut map = HashMap::new();

        map.insert("000".to_owned(), Regex::new(r"00000").unwrap());
        map.insert("111".to_owned(), Regex::new(r"11111").unwrap());
        map.insert("222".to_owned(), Regex::new(r"22222").unwrap());
        map.insert("333".to_owned(), Regex::new(r"33333").unwrap());
        map.insert("444".to_owned(), Regex::new(r"44444").unwrap());
        map.insert("555".to_owned(), Regex::new(r"55555").unwrap());
        map.insert("666".to_owned(), Regex::new(r"66666").unwrap());
        map.insert("777".to_owned(), Regex::new(r"77777").unwrap());
        map.insert("888".to_owned(), Regex::new(r"88888").unwrap());
        map.insert("999".to_owned(), Regex::new(r"99999").unwrap());
        map.insert("aaa".to_owned(), Regex::new(r"aaaaa").unwrap());
        map.insert("bbb".to_owned(), Regex::new(r"bbbbb").unwrap());
        map.insert("ccc".to_owned(), Regex::new(r"ccccc").unwrap());
        map.insert("ddd".to_owned(), Regex::new(r"ddddd").unwrap());
        map.insert("eee".to_owned(), Regex::new(r"eeeee").unwrap());
        map.insert("fff".to_owned(), Regex::new(r"fffff").unwrap());

        map
    };
}

fn triples(hash: &str) -> Vec<String> {
    let mut triples = TRIPLES_PATTERN.captures_iter(hash).map(|c| c[0].to_string()).collect::<Vec<String>>();
    triples.dedup();

    triples
}

#[derive(Debug)]
struct Key {
    hash: String,
    index: usize,
}

fn stretch_hash(hash: String, md5: &mut Md5) -> String {
    let mut next = hash;
    let mut out = vec![0; md5.output_bytes()];

    for _ in 0..2016 {
        md5.reset();
        md5.input_str(&next);

        md5.result(&mut out);
        next = out.iter().map(|b| format!("{:02x}", b)).collect();
    }

    next
}

fn day14() {
    let salt = "yjdafjpo";
    // let salt = "abc";

    let mut md5 = Md5::new();
    let mut out = vec![0; md5.output_bytes()];

    let mut result = Vec::new();
    let target_keys = 64;
    let lookahead = 1001;
    let mut indexes: Vec<usize> = Vec::with_capacity(lookahead);
    let mut buffer: Vec<String> = Vec::with_capacity(lookahead);
    let mut i = 0;

    'outer: loop {
        if result.len() == target_keys {
            println!("Finished at index: {}", i - 1);
            break;
        }

        if buffer.len() == lookahead {
            let hash_to_check = buffer.remove(0);
            let hash_index = indexes.remove(0);

            for triple in triples(&hash_to_check) {
                // If the remainder of the buffer contains the same digit
                // repeated 5 times, win.

                let p = QUINTUPLET_PATTERNS.get(&triple).unwrap();
                for line in &buffer {
                    if p.is_match(&line) {
                        result.push(Key { hash: hash_to_check, index: hash_index });
                        continue 'outer;
                    }
                }

                // We actually only want the first triple!
                break;
            }

        }

        md5.reset();
        md5.input_str(salt);
        md5.input_str(&i.to_string());

        md5.result(&mut out);

        let hash: String = stretch_hash(out.iter().map(|b| format!("{:02x}", b)).collect(), &mut md5);

        indexes.push(i);
        buffer.push(hash);

        i += 1;
    }

    for key in result {
        println!("{:?}", key);
    }
}

#[derive(Debug, Copy, Clone)]
struct Disc {
    position_count: usize,
    current_position: usize,
}

impl Disc {
    fn tick(&self) -> Disc {
        Disc { position_count: self.position_count, current_position: ((self.current_position + 1) % self.position_count) }
    }
}

fn day15() {
    let discs_at_time_zero = vec!(
        Disc { position_count: 13, current_position: 10 },
        Disc { position_count: 17, current_position: 15 },
        Disc { position_count: 19, current_position: 17 },
        Disc { position_count: 7, current_position: 1 },
        Disc { position_count: 5, current_position: 0 },
        Disc { position_count: 3, current_position: 1 },
        Disc { position_count: 11, current_position: 0 },
    );

    let mut logical_discs = discs_at_time_zero.iter().cloned().enumerate().map(|(i, disc)| {
        let mut result: Disc = disc;
        for _ in 0..i+1 {
            result = result.tick();
        }

        result
    }).collect::<Vec<Disc>>();

    let mut time = 0;

    loop {
        if logical_discs.iter().all (|disc| disc.current_position == 0) {
            break;
        }

        logical_discs = logical_discs.iter().map (|disc| disc.tick()).collect();

        time += 1;
    }

    println!("Earliest time: {}", time);

}


use std::collections::HashMap;


fn checksum_lookup_table() -> HashMap<String, String> {
    let mut result = HashMap::new();

    result.insert("00000000".to_owned(), "1111".to_owned());
    result.insert("00000001".to_owned(), "1110".to_owned());
    result.insert("00000010".to_owned(), "1110".to_owned());
    result.insert("00000011".to_owned(), "1111".to_owned());
    result.insert("00000100".to_owned(), "1101".to_owned());
    result.insert("00000101".to_owned(), "1100".to_owned());
    result.insert("00000110".to_owned(), "1100".to_owned());
    result.insert("00000111".to_owned(), "1101".to_owned());
    result.insert("00001000".to_owned(), "1101".to_owned());
    result.insert("00001001".to_owned(), "1100".to_owned());
    result.insert("00001010".to_owned(), "1100".to_owned());
    result.insert("00001011".to_owned(), "1101".to_owned());
    result.insert("00001100".to_owned(), "1111".to_owned());
    result.insert("00001101".to_owned(), "1110".to_owned());
    result.insert("00001110".to_owned(), "1110".to_owned());
    result.insert("00001111".to_owned(), "1111".to_owned());
    result.insert("00010000".to_owned(), "1011".to_owned());
    result.insert("00010001".to_owned(), "1010".to_owned());
    result.insert("00010010".to_owned(), "1010".to_owned());
    result.insert("00010011".to_owned(), "1011".to_owned());
    result.insert("00010100".to_owned(), "1001".to_owned());
    result.insert("00010101".to_owned(), "1000".to_owned());
    result.insert("00010110".to_owned(), "1000".to_owned());
    result.insert("00010111".to_owned(), "1001".to_owned());
    result.insert("00011000".to_owned(), "1001".to_owned());
    result.insert("00011001".to_owned(), "1000".to_owned());
    result.insert("00011010".to_owned(), "1000".to_owned());
    result.insert("00011011".to_owned(), "1001".to_owned());
    result.insert("00011100".to_owned(), "1011".to_owned());
    result.insert("00011101".to_owned(), "1010".to_owned());
    result.insert("00011110".to_owned(), "1010".to_owned());
    result.insert("00011111".to_owned(), "1011".to_owned());
    result.insert("00100000".to_owned(), "1011".to_owned());
    result.insert("00100001".to_owned(), "1010".to_owned());
    result.insert("00100010".to_owned(), "1010".to_owned());
    result.insert("00100011".to_owned(), "1011".to_owned());
    result.insert("00100100".to_owned(), "1001".to_owned());
    result.insert("00100101".to_owned(), "1000".to_owned());
    result.insert("00100110".to_owned(), "1000".to_owned());
    result.insert("00100111".to_owned(), "1001".to_owned());
    result.insert("00101000".to_owned(), "1001".to_owned());
    result.insert("00101001".to_owned(), "1000".to_owned());
    result.insert("00101010".to_owned(), "1000".to_owned());
    result.insert("00101011".to_owned(), "1001".to_owned());
    result.insert("00101100".to_owned(), "1011".to_owned());
    result.insert("00101101".to_owned(), "1010".to_owned());
    result.insert("00101110".to_owned(), "1010".to_owned());
    result.insert("00101111".to_owned(), "1011".to_owned());
    result.insert("00110000".to_owned(), "1111".to_owned());
    result.insert("00110001".to_owned(), "1110".to_owned());
    result.insert("00110010".to_owned(), "1110".to_owned());
    result.insert("00110011".to_owned(), "1111".to_owned());
    result.insert("00110100".to_owned(), "1101".to_owned());
    result.insert("00110101".to_owned(), "1100".to_owned());
    result.insert("00110110".to_owned(), "1100".to_owned());
    result.insert("00110111".to_owned(), "1101".to_owned());
    result.insert("00111000".to_owned(), "1101".to_owned());
    result.insert("00111001".to_owned(), "1100".to_owned());
    result.insert("00111010".to_owned(), "1100".to_owned());
    result.insert("00111011".to_owned(), "1101".to_owned());
    result.insert("00111100".to_owned(), "1111".to_owned());
    result.insert("00111101".to_owned(), "1110".to_owned());
    result.insert("00111110".to_owned(), "1110".to_owned());
    result.insert("00111111".to_owned(), "1111".to_owned());
    result.insert("01000000".to_owned(), "0111".to_owned());
    result.insert("01000001".to_owned(), "0110".to_owned());
    result.insert("01000010".to_owned(), "0110".to_owned());
    result.insert("01000011".to_owned(), "0111".to_owned());
    result.insert("01000100".to_owned(), "0101".to_owned());
    result.insert("01000101".to_owned(), "0100".to_owned());
    result.insert("01000110".to_owned(), "0100".to_owned());
    result.insert("01000111".to_owned(), "0101".to_owned());
    result.insert("01001000".to_owned(), "0101".to_owned());
    result.insert("01001001".to_owned(), "0100".to_owned());
    result.insert("01001010".to_owned(), "0100".to_owned());
    result.insert("01001011".to_owned(), "0101".to_owned());
    result.insert("01001100".to_owned(), "0111".to_owned());
    result.insert("01001101".to_owned(), "0110".to_owned());
    result.insert("01001110".to_owned(), "0110".to_owned());
    result.insert("01001111".to_owned(), "0111".to_owned());
    result.insert("01010000".to_owned(), "0011".to_owned());
    result.insert("01010001".to_owned(), "0010".to_owned());
    result.insert("01010010".to_owned(), "0010".to_owned());
    result.insert("01010011".to_owned(), "0011".to_owned());
    result.insert("01010100".to_owned(), "0001".to_owned());
    result.insert("01010101".to_owned(), "0000".to_owned());
    result.insert("01010110".to_owned(), "0000".to_owned());
    result.insert("01010111".to_owned(), "0001".to_owned());
    result.insert("01011000".to_owned(), "0001".to_owned());
    result.insert("01011001".to_owned(), "0000".to_owned());
    result.insert("01011010".to_owned(), "0000".to_owned());
    result.insert("01011011".to_owned(), "0001".to_owned());
    result.insert("01011100".to_owned(), "0011".to_owned());
    result.insert("01011101".to_owned(), "0010".to_owned());
    result.insert("01011110".to_owned(), "0010".to_owned());
    result.insert("01011111".to_owned(), "0011".to_owned());
    result.insert("01100000".to_owned(), "0011".to_owned());
    result.insert("01100001".to_owned(), "0010".to_owned());
    result.insert("01100010".to_owned(), "0010".to_owned());
    result.insert("01100011".to_owned(), "0011".to_owned());
    result.insert("01100100".to_owned(), "0001".to_owned());
    result.insert("01100101".to_owned(), "0000".to_owned());
    result.insert("01100110".to_owned(), "0000".to_owned());
    result.insert("01100111".to_owned(), "0001".to_owned());
    result.insert("01101000".to_owned(), "0001".to_owned());
    result.insert("01101001".to_owned(), "0000".to_owned());
    result.insert("01101010".to_owned(), "0000".to_owned());
    result.insert("01101011".to_owned(), "0001".to_owned());
    result.insert("01101100".to_owned(), "0011".to_owned());
    result.insert("01101101".to_owned(), "0010".to_owned());
    result.insert("01101110".to_owned(), "0010".to_owned());
    result.insert("01101111".to_owned(), "0011".to_owned());
    result.insert("01110000".to_owned(), "0111".to_owned());
    result.insert("01110001".to_owned(), "0110".to_owned());
    result.insert("01110010".to_owned(), "0110".to_owned());
    result.insert("01110011".to_owned(), "0111".to_owned());
    result.insert("01110100".to_owned(), "0101".to_owned());
    result.insert("01110101".to_owned(), "0100".to_owned());
    result.insert("01110110".to_owned(), "0100".to_owned());
    result.insert("01110111".to_owned(), "0101".to_owned());
    result.insert("01111000".to_owned(), "0101".to_owned());
    result.insert("01111001".to_owned(), "0100".to_owned());
    result.insert("01111010".to_owned(), "0100".to_owned());
    result.insert("01111011".to_owned(), "0101".to_owned());
    result.insert("01111100".to_owned(), "0111".to_owned());
    result.insert("01111101".to_owned(), "0110".to_owned());
    result.insert("01111110".to_owned(), "0110".to_owned());
    result.insert("01111111".to_owned(), "0111".to_owned());
    result.insert("10000000".to_owned(), "0111".to_owned());
    result.insert("10000001".to_owned(), "0110".to_owned());
    result.insert("10000010".to_owned(), "0110".to_owned());
    result.insert("10000011".to_owned(), "0111".to_owned());
    result.insert("10000100".to_owned(), "0101".to_owned());
    result.insert("10000101".to_owned(), "0100".to_owned());
    result.insert("10000110".to_owned(), "0100".to_owned());
    result.insert("10000111".to_owned(), "0101".to_owned());
    result.insert("10001000".to_owned(), "0101".to_owned());
    result.insert("10001001".to_owned(), "0100".to_owned());
    result.insert("10001010".to_owned(), "0100".to_owned());
    result.insert("10001011".to_owned(), "0101".to_owned());
    result.insert("10001100".to_owned(), "0111".to_owned());
    result.insert("10001101".to_owned(), "0110".to_owned());
    result.insert("10001110".to_owned(), "0110".to_owned());
    result.insert("10001111".to_owned(), "0111".to_owned());
    result.insert("10010000".to_owned(), "0011".to_owned());
    result.insert("10010001".to_owned(), "0010".to_owned());
    result.insert("10010010".to_owned(), "0010".to_owned());
    result.insert("10010011".to_owned(), "0011".to_owned());
    result.insert("10010100".to_owned(), "0001".to_owned());
    result.insert("10010101".to_owned(), "0000".to_owned());
    result.insert("10010110".to_owned(), "0000".to_owned());
    result.insert("10010111".to_owned(), "0001".to_owned());
    result.insert("10011000".to_owned(), "0001".to_owned());
    result.insert("10011001".to_owned(), "0000".to_owned());
    result.insert("10011010".to_owned(), "0000".to_owned());
    result.insert("10011011".to_owned(), "0001".to_owned());
    result.insert("10011100".to_owned(), "0011".to_owned());
    result.insert("10011101".to_owned(), "0010".to_owned());
    result.insert("10011110".to_owned(), "0010".to_owned());
    result.insert("10011111".to_owned(), "0011".to_owned());
    result.insert("10100000".to_owned(), "0011".to_owned());
    result.insert("10100001".to_owned(), "0010".to_owned());
    result.insert("10100010".to_owned(), "0010".to_owned());
    result.insert("10100011".to_owned(), "0011".to_owned());
    result.insert("10100100".to_owned(), "0001".to_owned());
    result.insert("10100101".to_owned(), "0000".to_owned());
    result.insert("10100110".to_owned(), "0000".to_owned());
    result.insert("10100111".to_owned(), "0001".to_owned());
    result.insert("10101000".to_owned(), "0001".to_owned());
    result.insert("10101001".to_owned(), "0000".to_owned());
    result.insert("10101010".to_owned(), "0000".to_owned());
    result.insert("10101011".to_owned(), "0001".to_owned());
    result.insert("10101100".to_owned(), "0011".to_owned());
    result.insert("10101101".to_owned(), "0010".to_owned());
    result.insert("10101110".to_owned(), "0010".to_owned());
    result.insert("10101111".to_owned(), "0011".to_owned());
    result.insert("10110000".to_owned(), "0111".to_owned());
    result.insert("10110001".to_owned(), "0110".to_owned());
    result.insert("10110010".to_owned(), "0110".to_owned());
    result.insert("10110011".to_owned(), "0111".to_owned());
    result.insert("10110100".to_owned(), "0101".to_owned());
    result.insert("10110101".to_owned(), "0100".to_owned());
    result.insert("10110110".to_owned(), "0100".to_owned());
    result.insert("10110111".to_owned(), "0101".to_owned());
    result.insert("10111000".to_owned(), "0101".to_owned());
    result.insert("10111001".to_owned(), "0100".to_owned());
    result.insert("10111010".to_owned(), "0100".to_owned());
    result.insert("10111011".to_owned(), "0101".to_owned());
    result.insert("10111100".to_owned(), "0111".to_owned());
    result.insert("10111101".to_owned(), "0110".to_owned());
    result.insert("10111110".to_owned(), "0110".to_owned());
    result.insert("10111111".to_owned(), "0111".to_owned());
    result.insert("11000000".to_owned(), "1111".to_owned());
    result.insert("11000001".to_owned(), "1110".to_owned());
    result.insert("11000010".to_owned(), "1110".to_owned());
    result.insert("11000011".to_owned(), "1111".to_owned());
    result.insert("11000100".to_owned(), "1101".to_owned());
    result.insert("11000101".to_owned(), "1100".to_owned());
    result.insert("11000110".to_owned(), "1100".to_owned());
    result.insert("11000111".to_owned(), "1101".to_owned());
    result.insert("11001000".to_owned(), "1101".to_owned());
    result.insert("11001001".to_owned(), "1100".to_owned());
    result.insert("11001010".to_owned(), "1100".to_owned());
    result.insert("11001011".to_owned(), "1101".to_owned());
    result.insert("11001100".to_owned(), "1111".to_owned());
    result.insert("11001101".to_owned(), "1110".to_owned());
    result.insert("11001110".to_owned(), "1110".to_owned());
    result.insert("11001111".to_owned(), "1111".to_owned());
    result.insert("11010000".to_owned(), "1011".to_owned());
    result.insert("11010001".to_owned(), "1010".to_owned());
    result.insert("11010010".to_owned(), "1010".to_owned());
    result.insert("11010011".to_owned(), "1011".to_owned());
    result.insert("11010100".to_owned(), "1001".to_owned());
    result.insert("11010101".to_owned(), "1000".to_owned());
    result.insert("11010110".to_owned(), "1000".to_owned());
    result.insert("11010111".to_owned(), "1001".to_owned());
    result.insert("11011000".to_owned(), "1001".to_owned());
    result.insert("11011001".to_owned(), "1000".to_owned());
    result.insert("11011010".to_owned(), "1000".to_owned());
    result.insert("11011011".to_owned(), "1001".to_owned());
    result.insert("11011100".to_owned(), "1011".to_owned());
    result.insert("11011101".to_owned(), "1010".to_owned());
    result.insert("11011110".to_owned(), "1010".to_owned());
    result.insert("11011111".to_owned(), "1011".to_owned());
    result.insert("11100000".to_owned(), "1011".to_owned());
    result.insert("11100001".to_owned(), "1010".to_owned());
    result.insert("11100010".to_owned(), "1010".to_owned());
    result.insert("11100011".to_owned(), "1011".to_owned());
    result.insert("11100100".to_owned(), "1001".to_owned());
    result.insert("11100101".to_owned(), "1000".to_owned());
    result.insert("11100110".to_owned(), "1000".to_owned());
    result.insert("11100111".to_owned(), "1001".to_owned());
    result.insert("11101000".to_owned(), "1001".to_owned());
    result.insert("11101001".to_owned(), "1000".to_owned());
    result.insert("11101010".to_owned(), "1000".to_owned());
    result.insert("11101011".to_owned(), "1001".to_owned());
    result.insert("11101100".to_owned(), "1011".to_owned());
    result.insert("11101101".to_owned(), "1010".to_owned());
    result.insert("11101110".to_owned(), "1010".to_owned());
    result.insert("11101111".to_owned(), "1011".to_owned());
    result.insert("11110000".to_owned(), "1111".to_owned());
    result.insert("11110001".to_owned(), "1110".to_owned());
    result.insert("11110010".to_owned(), "1110".to_owned());
    result.insert("11110011".to_owned(), "1111".to_owned());
    result.insert("11110100".to_owned(), "1101".to_owned());
    result.insert("11110101".to_owned(), "1100".to_owned());
    result.insert("11110110".to_owned(), "1100".to_owned());
    result.insert("11110111".to_owned(), "1101".to_owned());
    result.insert("11111000".to_owned(), "1101".to_owned());
    result.insert("11111001".to_owned(), "1100".to_owned());
    result.insert("11111010".to_owned(), "1100".to_owned());
    result.insert("11111011".to_owned(), "1101".to_owned());
    result.insert("11111100".to_owned(), "1111".to_owned());
    result.insert("11111101".to_owned(), "1110".to_owned());
    result.insert("11111110".to_owned(), "1110".to_owned());
    result.insert("11111111".to_owned(), "1111".to_owned());

    result
}


fn expand_to_size(initial_state: String, min_size: usize) -> String {
    let mut state = initial_state;

    while state.len() < min_size {
        let flipped: String = state.chars().map(|ch| {
            match ch {
                '0' => '1',
                _ => '0'
            }
        }).rev().collect();

        state = state + "0" + &flipped;
    }

    state
}


fn checksum_round(s: String, lookup: &HashMap<String, String>) -> String {
    let mut result = "".to_owned();

    // Use our lookup table as much as possible
    let mut i = 0;
    while (i + 8) < (s.len()) {
        result += lookup.get(&(s[i..(i + 8)]).to_owned()).unwrap();
        i += 8;
    }

    // and manually handle the remainder...
    while i < (s.len() - 1) {
        if s.chars().nth(i).unwrap() == s.chars().nth(i+1).unwrap() {
            result += "1";
        } else {
            result += "0";
        }

        i += 2;
    }

    result
}


fn checksum(s: String, lookup: &HashMap<String, String>) -> String {
    let mut c = checksum_round(s, lookup);

    while (c.len() % 2) == 0 {
        c = checksum_round(c, lookup);
    }

    return c;
}

fn day16() {
    // Part 1
    // let initial = "01111010110010011";
    // let n = 272;

    // Part 2
    let initial = "01111010110010011";
    let n = 35651584;

    let lookup = checksum_lookup_table();

    let expanded = expand_to_size(initial.to_owned(), n);

    println!("{}", checksum((&expanded[0..n]).to_owned(), &lookup));
}


extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

#[derive(Eq, PartialEq, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn add(&self, other: &Position) -> Position {
        Position { x: (self.x + other.x), y: (self.y + other.y) }
    }
}

struct Candidate<'a> {
    position: Position,
    path: Vec<&'a str>,
}

fn hash(passcode: &str, path: &Vec<&str>) -> String {
    let mut md5 = Md5::new();
    let mut out = vec![0; md5.output_bytes()];

    md5.input_str(passcode);

    for char in path {
        md5.input(char.as_bytes());
    }

    md5.result(&mut out);

    out.iter().take(2).map(|b| format!("{:02x}", b)).collect()
}

struct Direction<'a> (&'a str, Position);

fn day17() {
    let directions = vec!(Direction("U", Position { x: 0, y: -1 }),
                          Direction("D", Position { x: 0, y: 1 }),
                          Direction("L", Position { x: -1, y: 0 }),
                          Direction("R", Position { x: 1, y: 0 }));

    let passcode = "mmsxrhfx";

    let target = Position { x: 3, y: 3 };
    let mut candidates: Vec<Candidate> = vec!(Candidate { position: Position { x: 0, y: 0 }, path: Vec::new() });
    let mut result: Vec<Candidate> = Vec::new();

    while candidates.len() > 0 {
        let candidate = candidates.remove(0);

        if candidate.position == target {
            result.push(candidate);
            continue;
        }

        let doors = hash(passcode, &candidate.path);
        // println!("{}{} -> {}", passcode, candidate.path.join(""), doors);


        for d in 0..directions.len() {
            if doors.chars().nth(d).unwrap() >= 'b' && doors.chars().nth(d).unwrap() <= 'f' {
                // The door is open
                let mut new_path = candidate.path.clone();
                new_path.push(directions[d].0);

                let new_position = candidate.position.add(&directions[d].1);

                if (new_position.x >= 0 && new_position.y >= 0) && (new_position.x <= 3 && new_position.y <= 3) {
                    // println!("Moving {} from {:?}", directions[d].0, candidate.position);
                    // println!("New position {:?}", new_position);

                    candidates.push(Candidate { position: new_position,
                                                path: new_path });
                }
            }
        }
    }

    result.sort_by_key(|candidate| candidate.path.len());
    println!("Shortest: {}", result[0].path.join(""));
    println!("Longest: {}", result[result.len() - 1].path.join(""));

}

// Too low: 501
fn day18() {
    // . = safe; ^ = TRAP
    let mut last_row = ".^^..^...^..^^.^^^.^^^.^^^^^^.^.^^^^.^^.^^^^^^.^...^......^...^^^..^^^.....^^^^^^^^^....^^...^^^^..^".to_owned();
    let mut padded = String::new();
    let mut safe_count = 0;

    for _ in 0..400000 {
        safe_count += last_row.chars().filter(|&ch| ch == '.').count();
        // println!("{}", last_row);

        padded.clear();

        padded.push('.');
        padded.push_str(&last_row);
        padded.push('.');

        last_row.clear();

        for i in 1..(padded.len() - 1) {
            let left = padded.chars().nth(i - 1).unwrap();
            let centre = padded.chars().nth(i).unwrap();
            let right = padded.chars().nth(i + 1).unwrap();

            if (left == '^' && centre == '^' && right == '.') ||
                (left == '.' && centre == '^' && right == '^') ||
                (left == '^' && centre == '.' && right == '.') ||
                (left == '.' && centre == '.' && right == '^') {
                    last_row.push('^');
                } else {
                    last_row.push('.');
                }
        }
    }

    println!("Number of safe tiles: {}", safe_count);
}


fn day19_pt1() {
    let count: usize = 3014603;
    let mut elves: Vec<i32> = (1..(count + 1) as i32).collect();

    let mut next_pos = 0;
    let mut remaining_elves = count;
    let mut keep = true;

    loop {
        if remaining_elves == 1 {
            break
        }

        if elves[next_pos] >= 0 {
            if !keep {
                elves[next_pos] = -1;
                remaining_elves -= 1;
            }

            keep = !keep;
        }

        next_pos += 1;

        if next_pos >= count {
            next_pos = 0;
        }
    }

    println!("{:?}", elves.iter().find(|&&n| n >= 0).unwrap());
}

// Nuttiness :)
fn day19_pt2() {
    let count: usize = 3014603;
    let mut elves: Vec<i32> = (1..(count + 1) as i32).collect();

    let mut next_pos = 0;
    let mut next_victim = count / 2;
    let mut remaining_elves = count;

    loop {
        if remaining_elves == 1 {
            break
        }

        if elves[next_pos] >= 0 {
            while elves[next_victim] < 0 {
                next_victim += 1;

                if next_victim >= count { next_victim = 0; }
            }

            elves[next_victim] = -1;

            while elves[next_victim] < 0 {
                next_victim += 1;
                if next_victim >= count { next_victim = 0; }
            }

            if (remaining_elves % 2) == 1 {
                next_victim += 1;
                if next_victim >= count { next_victim = 0; }
            }

            remaining_elves -= 1;
        }

        next_pos += 1;

        if next_pos >= count { next_pos = 0; }
    }

    println!("{:?}", elves.iter().find(|&&n| n >= 0).unwrap());
}
fn day19() {
    day19_pt1();
    day19_pt2();
}

// NOTE: Better way to do this would be sort the ranges by lower, merge
// overlapping ranges, then just look for the gaps between them.  But I felt
// like loading it all into memory just because I could :)

use std::fs::File;
use std::io::{BufRead, BufReader};

fn first_set_bit(bitvec: &Vec<u32>) -> Option<u32> {
    for (count, &segment) in bitvec.iter().enumerate() {
        if segment != 0 {
            for offset in 0..32 {
                if (segment >> (32 - offset - 1) & 1) == 1 {
                    println!("Found a thing in segment: {}", count);
                    return Some((count as u32 * 32) + offset);
                }
            }
        }
    }

    None
}

fn count_set_bits(bitvec: &Vec<u32>) -> usize {
    let mut result = 0;

    for &segment in bitvec {
        if segment != 0 {
            for offset in 0..32 {
                if (segment >> (32 - offset - 1) & 1) == 1 {
                    result += 1;
                }
            }
        }
    }

    result
}

fn day20() {
    let max: u64 = 4294967296;

    // Could just use a bitset, but for giggles...
    let mut valid_addresses: Vec<u32> = vec!(0xFFFFFFFF; (max / 32) as usize);

    let f = File::open("advent-files/day20_input.txt").expect("open file");
    let br = BufReader::new(f);

    for range in br.lines().map(Result::unwrap) {
        let range: Vec<usize> = range.split("-").map(str::parse).map(Result::unwrap).collect();

        for bit_to_clear in range[0]..(range[1] + 1) {
            let segment = bit_to_clear / 32;
            let offset = bit_to_clear % 32;

            valid_addresses[segment] &= !(1 << (32 - offset - 1));
        }
    }

    println!("Scanning for valid address...");
    println!("First valid address: {}", first_set_bit(&valid_addresses).unwrap());
    println!("Number of valid addresses: {}", count_set_bits(&valid_addresses));
}

extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;


fn day21_pt1() {
    let f = File::open("advent-files/day21_input.txt").expect("open file");
    let br = BufReader::new(f);

    let swap_position = Regex::new(r"swap position (\d+) with position (\d+)").unwrap();
    let swap_letter = Regex::new(r"swap letter (\w) with letter (\w)").unwrap();
    let rotate = Regex::new(r"rotate (left|right) (\d+) steps?").unwrap();
    let rotate_by_letter = Regex::new(r"rotate based on position of letter (\w)").unwrap();
    let reverse_by_position = Regex::new(r"reverse positions (\d+) through (\d+)").unwrap();
    let move_position = Regex::new(r"move position (\d+) to position (\d+)").unwrap();

    let mut input: Vec<String> = "abcdefgh".chars().map(|c| c.to_string()).collect();

    for instruction in br.lines().map(Result::unwrap) {
        // println!("Partial result: {}", input.join(""));

        if let Some(args) = swap_position.captures(&instruction) {
            let positions: Vec<usize> = args.iter().map(Option::unwrap).skip(1).map(|m| m.as_str().parse().unwrap()).collect();

            let tmp = input[positions[0]].clone();
            input[positions[0]] = input[positions[1]].clone();
            input[positions[1]] = tmp;
        } else if let Some(args) = swap_letter.captures(&instruction) {
            let letters: Vec<&str> = args.iter().map(Option::unwrap).skip(1).map(|m| m.as_str()).collect();

            let pos_a = input.iter().position(|c| c == letters[0]).unwrap();
            let pos_b = input.iter().position(|c| c == letters[1]).unwrap();

            let tmp = input[pos_a].clone();
            input[pos_a] = input[pos_b].clone();
            input[pos_b] = tmp;
        } else if let Some(args) = rotate.captures(&instruction) {
            let direction = &args[1];
            let places: usize = args[2].parse().unwrap();

            if places > 0 {
                let mut pos = if direction == "left" {
                    places % input.len()
                } else {
                    input.len() - (places % input.len())
                };


                let mut rotated = Vec::with_capacity(input.len());

                for _ in 0..input.len() {
                    rotated.push(input[pos].clone());
                    pos = (pos + 1 ) % input.len();
                }

                input = rotated;
            }
        } else if let Some(args) = rotate_by_letter.captures(&instruction) {
            let letter = &args[1];
            let letter_idx = input.iter().position(|c| c == letter).unwrap();

            let mut rotations = letter_idx + 1;
            if letter_idx >= 4 {
                rotations += 1;
            }

            if (rotations % input.len()) > 0 {
                let mut pos = input.len() - (rotations % input.len());
                let mut rotated = Vec::with_capacity(input.len());

                for _ in 0..input.len() {
                    rotated.push(input[pos].clone());
                    pos = (pos + 1) % input.len();
                }

                input = rotated;
            }
        } else if let Some(args) = reverse_by_position.captures(&instruction) {
            let positions: Vec<usize> = args.iter().map(Option::unwrap).skip(1).map(|m| m.as_str().parse().unwrap()).collect();

            let mut pos_a = positions[0];
            let mut pos_b = positions[1];

            while pos_a < pos_b {
                let tmp = input[pos_a].clone();
                input[pos_a] = input[pos_b].clone();
                input[pos_b] = tmp;

                pos_a += 1;
                pos_b -= 1;
            }
        } else if let Some(args) = move_position.captures(&instruction) {
            let positions: Vec<usize> = args.iter().map(Option::unwrap).skip(1).map(|m| m.as_str().parse().unwrap()).collect();

            let pos_a = positions[0];
            let pos_b = positions[1];

            let elt = input.remove(pos_a);
            input.insert(pos_b, elt);
        } else {
            panic!(format!("Parse error for instruction: {}", instruction));
        }
    }

    println!("Result: {}", input.join(""));
}

fn day21_pt2() {
    let f = File::open("advent-files/day21_input_backwards.txt").expect("open file");

    let br = BufReader::new(f);

    let swap_position = Regex::new(r"^swap position (\d+) with position (\d+)").unwrap();
    let swap_letter = Regex::new(r"^swap letter (\w) with letter (\w)").unwrap();
    let rotate = Regex::new(r"^rotate (left|right) (\d+) steps?").unwrap();
    let rotate_by_letter = Regex::new(r"^rotate based on position of letter (\w)").unwrap();
    let left_rotate_by_letter = Regex::new(r"^rotate left based on position of letter (\w)").unwrap();
    let reverse_by_position = Regex::new(r"^reverse positions (\d+) through (\d+)").unwrap();
    let undo_move_position = Regex::new(r"^undo move position (\d+) to position (\d+)").unwrap();
    let move_position = Regex::new(r"^move position (\d+) to position (\d+)").unwrap();

    let mut input: Vec<String> = "fbgdceah".chars().map(|c| c.to_string()).collect();

    for instruction in br.lines().map(Result::unwrap) {
        // println!("Partial result: {}", input.join(""));

        if let Some(args) = swap_position.captures(&instruction) {
            let positions: Vec<usize> = args.iter().map(Option::unwrap).skip(1).map(|m| m.as_str().parse().unwrap()).collect();

            let tmp = input[positions[0]].clone();
            input[positions[0]] = input[positions[1]].clone();
            input[positions[1]] = tmp;
        } else if let Some(args) = swap_letter.captures(&instruction) {
            let letters: Vec<&str> = args.iter().map(Option::unwrap).skip(1).map(|m| m.as_str()).collect();

            let pos_a = input.iter().position(|c| c == letters[0]).unwrap();
            let pos_b = input.iter().position(|c| c == letters[1]).unwrap();

            let tmp = input[pos_a].clone();
            input[pos_a] = input[pos_b].clone();
            input[pos_b] = tmp;
        } else if let Some(args) = rotate.captures(&instruction) {
            let direction = &args[1];
            let places: usize = args[2].parse().unwrap();

            if places > 0 {
                let mut pos = if direction == "left" {
                    places % input.len()
                } else {
                    input.len() - (places % input.len())
                };


                let mut rotated = Vec::with_capacity(input.len());

                for _ in 0..input.len() {
                    rotated.push(input[pos].clone());
                    pos = (pos + 1 ) % input.len();
                }

                input = rotated;
            }
        } else if let Some(args) = rotate_by_letter.captures(&instruction) {
            let letter = &args[1];
            let letter_idx = input.iter().position(|c| c == letter).unwrap();

            let mut rotations = letter_idx + 1;
            if letter_idx >= 4 {
                rotations += 1;
            }

            if (rotations % input.len()) > 0 {
                let mut pos = input.len() - (rotations % input.len());
                let mut rotated = Vec::with_capacity(input.len());

                for _ in 0..input.len() {
                    rotated.push(input[pos].clone());
                    pos = (pos + 1) % input.len();
                }

                input = rotated;
            }
        } else if let Some(args) = left_rotate_by_letter.captures(&instruction) {
            let letter = &args[1];
            let letter_idx = input.iter().position(|c| c == letter).unwrap();

            let mut pos = match letter_idx {
                1 => 1,
                3 => 2,
                5 => 3,
                7 => 4,
                2 => 6,
                4 => 7,
                6 => 0,
                0 => 1,
                _ => panic!("Bugger")
            };


            let mut rotated = Vec::with_capacity(input.len());

            for _ in 0..input.len() {
                rotated.push(input[pos].clone());
                pos = (pos + 1) % input.len();
            }

            input = rotated;
        } else if let Some(args) = reverse_by_position.captures(&instruction) {
            let positions: Vec<usize> = args.iter().map(Option::unwrap).skip(1).map(|m| m.as_str().parse().unwrap()).collect();

            let mut pos_a = positions[0];
            let mut pos_b = positions[1];

            while pos_a < pos_b {
                let tmp = input[pos_a].clone();
                input[pos_a] = input[pos_b].clone();
                input[pos_b] = tmp;

                pos_a += 1;
                pos_b -= 1;
            }
        } else if let Some(args) = move_position.captures(&instruction) {
            let positions: Vec<usize> = args.iter().map(Option::unwrap).skip(1).map(|m| m.as_str().parse().unwrap()).collect();

            let pos_a = positions[0];
            let pos_b = positions[1];

            let elt = input.remove(pos_a);
            input.insert(pos_b, elt);
        } else if let Some(args) = undo_move_position.captures(&instruction) {
            let positions: Vec<usize> = args.iter().map(Option::unwrap).skip(1).map(|m| m.as_str().parse().unwrap()).collect();

            let pos_a = positions[0];
            let pos_b = positions[1];

            let elt = input.remove(pos_b);
            input.insert(pos_a, elt);
        } else {
            panic!(format!("Parse error for instruction: {}", instruction));
        }
    }

    println!("Result: {}", input.join(""));
}


fn day21() {
    day21_pt1();
    day21_pt2();
}


extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Node {
    x: usize,
    y: usize,
    used_tb: usize,
    avail_tb: usize,
}

fn day22_pt1() {
    let f = File::open("advent-files/day22_input.txt").expect("open file");
    let br = BufReader::new(f);

    let delim = Regex::new(" +").unwrap();

    let mut nodes = Vec::new();
    for line in br.lines().skip(2).map(Result::unwrap) {
        let row: Vec<&str> = delim.split(&line).collect();

        let node = Node {
            x: 0,               // unused
            y: 0,               // unused
            used_tb: row[2].replace("T", "").parse().unwrap(),
            avail_tb: row[3].replace("T", "").parse().unwrap(),
        };

        nodes.push(node);
    }

    let mut matching_pairs_count = 0;
    for i in 0..nodes.len() {
        for j in (i+1)..nodes.len() {
            if (nodes[i].used_tb > 0 && nodes[i].used_tb <= nodes[j].avail_tb) ||
                (nodes[j].used_tb > 0 && nodes[j].used_tb <= nodes[i].avail_tb) {
                    // println!("Match: [{}: {}/{}] -- [{}: {}/{}]",
                    //          i, nodes[i].used_tb, nodes[i].used_tb + nodes[i].avail_tb,
                    //          j, nodes[j].used_tb, nodes[j].used_tb + nodes[j].avail_tb)
                        // ;
                    matching_pairs_count += 1;
                }
        }
    }

    println!("Found {} matching pairs", matching_pairs_count);
}

use std::collections::HashSet;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct State {
    grid: Vec<Vec<Option<Node>>>,
    target_data_x: usize,
    target_data_y: usize,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct PackedState (usize, usize, Vec<usize>);

fn packed_state(state: &State) -> PackedState {
    let mut result = vec!(0 ;state.grid.len() * state.grid.len());

    for y in 0..state.grid.len() {
        for x in 0..state.grid[0].len() {
            let node = state.grid[y][x].clone().unwrap();
            result[(y * state.grid.len()) + x] = node.used_tb;
        }
    }

    PackedState(state.target_data_x, state.target_data_y, result)
}



impl State {
    fn from_nodes(nodes: Vec<Node>) -> State {
        let max_x = nodes.iter().map(|node| node.x).max().unwrap();
        let max_y = nodes.iter().map(|node| node.y).max().unwrap();

        let mut grid: Vec<Vec<Option<Node>>> = (0..(max_y + 1)).map(|_| {
            (0..(max_x + 1)).map(|_| { None }).collect()
        }).collect();

        for node in nodes {
            grid[node.y][node.x] = Some(node.clone())
        }

        State {
            grid: grid,
            target_data_x: max_x,
            target_data_y: 0,
        }
    }

    fn move_data(&self, source: &Node, target: &Node) -> State {
        let mut new_grid: Vec<Vec<Option<Node>>> = self.grid.iter().map(|row| row.iter().cloned().collect()).collect();

        new_grid[source.y][source.x] = Some(Node {
            avail_tb: source.used_tb + source.avail_tb,
            used_tb: 0,
            x: source.x,
            y: source.y,
        });

        new_grid[target.y][target.x] = Some(Node {
            avail_tb: (target.avail_tb - source.used_tb),
            used_tb: (target.used_tb + source.used_tb),
            x: target.x,
            y: target.y,
        });

        let mut result = State {
            grid: new_grid,
            target_data_x: self.target_data_x,
            target_data_y: self.target_data_y,
        };

        if self.target_data_x == source.x && self.target_data_y == source.y {
            result.target_data_x = target.x;
            result.target_data_y = target.y;
        }

        result
    }

    fn already_seen(&self, state: &State, seen_states: &HashSet<PackedState>) -> bool {
        return seen_states.contains(&packed_state(&state))
    }

    fn record_state(&self, state: &State, seen_states: &mut HashSet<PackedState>) {
        seen_states.insert(packed_state(state));
    }

    fn next_states(&self, seen_states: &mut HashSet<PackedState>) -> Vec<State> {
        let mut next_states = Vec::new();

        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                let node = self.grid[y][x].clone().unwrap();

                for offset in &[[-1, 0], [1, 0], [0, -1], [0, 1]] {
                    let new_x = x as i32 + offset[0];
                    let new_y = y as i32 + offset[1];

                    if new_y >= 0 && new_y < self.grid.len() as i32 && new_x >= 0 && new_x < self.grid[new_y as usize].len() as i32 {
                        let target_node = self.grid[new_y as usize][new_x as usize].clone().unwrap();

                        if target_node.avail_tb >= node.used_tb && node.used_tb > 0 {
                            let mut new_state = self.move_data(&node, &target_node);

                            if !self.already_seen(&new_state, seen_states) {
                                self.record_state(&new_state, seen_states);
                                next_states.push(new_state);
                            }
                        }
                    }
                }
            }
        }

        next_states
    }
}

struct Chain {
    state: State,
    steps: usize
}

// Naive brute-force solution (doesn't work :)
// THINKME: A better version of state?  What if each node just knew how much space it used, how much it had left, and which nodes data it held?
fn day22_pt2() {
    let f = File::open("advent-files/day22_input.txt").expect("open file");
    let br = BufReader::new(f);

    let delim = Regex::new(" +").unwrap();
    let device_pattern = Regex::new("/dev/grid/node-x([0-9]+)-y([0-9]+)").unwrap();

    let mut nodes = Vec::new();
    for line in br.lines().skip(2).map(Result::unwrap) {
        let row: Vec<&str> = delim.split(&line).collect();

        let device = device_pattern.captures(row[0]).unwrap();

        let mut node = Node {
            x: device[1].parse().unwrap(),
            y: device[2].parse().unwrap(),
            used_tb: row[2].replace("T", "").parse().unwrap(),
            avail_tb: row[3].replace("T", "").parse().unwrap(),
        };

        // Bucket our nodes to save on state space.
        if node.used_tb > 400 {
            node.used_tb = 1000;
            node.avail_tb = 0;
        } else if node.used_tb > 0 {
            node.used_tb = 80;
            node.avail_tb = 20;
        } else {
            node.avail_tb = 100
        }

        nodes.push(node);
    }

    let mut seen_states = HashSet::new();

    let mut queue = vec!(Chain {
        state: State::from_nodes(nodes),
        steps: 0,
    });

    seen_states.insert(packed_state(&queue[0].state));


    let mut loopcount = 0;

    'outer: while queue.len() > 0 {
        loopcount += 1;

        let chain = queue.remove(0);

        let next_states = chain.state.next_states(&mut seen_states);

        for next_state in next_states {
            if next_state.target_data_x == 0 && next_state.target_data_y == 0 {
                println!("Win! {} steps", chain.steps + 1);
                break 'outer
            }

            queue.push(Chain {
                state: next_state,
                steps: chain.steps + 1
            });
        }

        if loopcount > 1000000 {
            break;
        }
    }

    println!("Finished...");
}


fn day22() {
    day22_pt1();
    day22_pt2();
}

use std::collections::HashMap;


const DAY23_SAMPLE_INPUT: &str = "
cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a
";

const DAY23_INPUT: &str = "
cpy a b
dec b
cpy a d
cpy 0 a
cpy b c
inc a
dec c
jnz c -2
dec d
jnz d -5
dec b
cpy b c
cpy c d
dec d
inc c
jnz d -2
tgl c
cpy -16 c
jnz 1 c
cpy 79 c
jnz 74 d
inc a
inc d
jnz d -2
inc c
jnz c -5
";

const DAY23_INPUT_OPTIMIZED: &str = "
cpy a b
dec b
nop
nop
nop
mul b a
cpy 0 c
cpy 0 d
nop
nop
nop
dec b
cpy b c
cpy c d
dec d
inc c
jnz d -2
tgl c
cpy -16 c
jnz 1 c
cpy 79 c
jnz 74 d
inc a
inc d
jnz d -2
inc c
jnz c -5
";


const MAGIC_START_NUMBER: i64 = 12;


fn to_register(name: &str) -> char {
    name.chars().nth(0).unwrap()
}

fn deref_value(value: &str, registers: &HashMap<char, i64>) -> i64 {
    match to_register(value) {
        r @ 'a'...'z' => {
            *registers.get(&r).unwrap()
        },
        _ => { value.parse().unwrap() }
    }
}

fn day23() {
    let mut instructions: Vec<String> = DAY23_INPUT_OPTIMIZED.trim().split("\n").map(|s| {
        if &s[0..1] == "#" {
            "nop".to_string()
        } else {
            s.to_string()
        }}).collect();

    let mut registers = "abcd".chars().fold(HashMap::new(), |mut acc, register| {
        if register == 'a' {
            acc.insert(register, MAGIC_START_NUMBER);
        } else {
            acc.insert(register, 0);
        }
        acc
    });

    let mut pc: i64 = 0;

    loop {
        // println!("PC: {}; REGS: {:?}", pc, registers);

        if pc < 0 || pc >= (instructions.len() as i64) {
            break;
        }

        let instruction = (&instructions[pc as usize]).clone();
        let bits: Vec<&str> = instruction.split(" ").collect();

        match bits[0] {
            "tgl" => {
                let offset = deref_value(bits[1], &registers);

                if (pc + offset) >= 0 && ((pc + offset) as usize) < instructions.len() {
                    println!{"Toggle: {} ({:?})", (pc + offset), instructions[(pc + offset) as usize]};
                    let target_instruction = (pc + offset) as usize;
                    let target_bits: Vec<String> = instructions[target_instruction].clone().split(" ").map(str::to_string).collect();

                    match target_bits.len() {
                        3 => {
                            if target_bits[0] == "jnz" {
                                instructions[target_instruction] = format!("{} {} {}", "cpy", target_bits[1], target_bits[2]);
                            } else {
                                instructions[target_instruction] = format!("{} {} {}", "jnz", target_bits[1], target_bits[2]);
                            }
                        },
                        2 => {
                            if target_bits[0] == "inc" {
                                instructions[target_instruction] = format!("{} {}", "dec", target_bits[1]);
                            } else {
                                instructions[target_instruction] = format!("{} {}", "inc", target_bits[1]);
                            }
                        },
                        _ => {
                            panic!("Invalid instruction: {:?}", target_bits);
                        }
                    };
                }
            },
            "cpy" => {
                let value = deref_value(bits[1], &registers);
                registers.insert(to_register(bits[2]), value);
            },
            "mul" => {
                let value1 = deref_value(bits[1], &registers);
                let value2 = deref_value(bits[2], &registers);
                registers.insert(to_register(bits[2]), value1 * value2);
            },
            "nop" => {},
            "inc" => {
                let new_value = deref_value(bits[1], &registers) + 1;
                registers.insert(to_register(bits[1]), new_value);
            },
            "dec" => {
                let new_value = deref_value(bits[1], &registers) - 1;
                registers.insert(to_register(bits[1]), new_value);
            },
            "jnz" => {
                let x = deref_value(bits[1], &registers);
                let y = deref_value(bits[2], &registers);

                if x != 0 {
                    // Compensate for the increment we're going to get anyway.
                    pc -= 1;
                    pc += y;
                }
            },
            _ => { panic!("WTF?! {}", bits[0]); },
        }

        pc += 1;
    }

    println!("Final state: {:?}", &registers);
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Location = char;

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<Location>>,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Position {
    x: usize,
    y: usize,
}


impl Grid {
    fn new() -> Grid {
        Grid { data: Vec::new() }
    }

    fn load(input: &mut BufRead) -> Grid {
        let mut result = Grid::new();

        for line in input.lines().map(Result::unwrap) {
            result.data.push(line.chars().collect());
        }

        result
    }

    fn position_of(&self, location: Location) -> Option<Position> {
        for y in 0..self.data.len() {
            for x in 0..self.data[y].len() {
                if self.data[y][x] == location {
                    return Some(Position { x, y })
                }
            }
        }

        return None;
    }

    fn get(&self, position: &Position) -> Option<Location> {
        if position.y < self.data.len() &&
            position.x < self.data[position.y].len() {
                return Some(self.data[position.y][position.x])
            }

        None
    }
}

struct Candidate {
    position: Position,
    cost: usize,
}

struct ShortestPath {
    paths: HashMap<(Position, Position), usize>,
}

impl ShortestPath {
    fn new() -> ShortestPath {
        ShortestPath { paths: HashMap::new() }
    }

    fn add_location(&mut self, start: &Position, end: &Position, cost: usize) {
        self.paths.insert((start.clone(), end.clone()), cost);
    }

    fn cost(&self, start: &Position, end: &Position) -> usize {
        match self.paths.get(&(start.clone(), end.clone())) {
            Some(&cost) => { cost },
            None => { std::usize::MAX }
        }
    }
}

fn surrounding_positions(pos: Position) -> Vec<Position> {
    let mut result = Vec::new();

    for &(x_offset, y_offset) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
        if ((pos.x as i64) + x_offset >= 0) && ((pos.y as i64) + y_offset >= 0) {
            result.push(Position {
                x: ((pos.x as i64) + x_offset) as usize,
                y: ((pos.y as i64) + y_offset) as usize,
            });
        }
    }

    result
}

use std::fmt::Debug;

fn all_permutations<T: Clone+Debug>(elts: Vec<T>) -> Vec<Vec<T>> {
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

fn day24() {
    let f = File::open("advent-files/day24_input.txt").expect("open file");
    let mut br = BufReader::new(f);

    let grid = Grid::load(&mut br);

    let mut paths = ShortestPath::new();
    let locations = ['0', '1', '2', '3', '4', '5', '6', '7'];

    for &start in locations.iter() {
        let start_position = grid.position_of(start).unwrap();

        paths.add_location(&start_position, &start_position, 0);

        let mut queue: Vec<Candidate> = Vec::new();
        queue.push(Candidate {
            position: start_position.clone(),
            cost: 0,
        });

        while !queue.is_empty() {
            let candidate = queue.remove(0);

            for new_position in surrounding_positions(candidate.position) {
                match grid.get(&new_position) {
                    Some(location) => {
                        if location == '#' {
                            // No good!
                        } else {
                            if paths.cost(&start_position, &new_position) > (candidate.cost + 1) {
                                // We've got a better one...
                                paths.add_location(&start_position, &new_position, candidate.cost + 1);
                                queue.push(Candidate {
                                    position: new_position,
                                    cost: candidate.cost + 1,
                                });
                            }
                        }
                    },
                    None => {},
                }
            }
        }
    }

    let mut best_cost = std::usize::MAX;

    for permutation in all_permutations(['1', '2', '3', '4', '5', '6', '7'].to_vec()) {
        let mut start = '0';
        let mut cost = 0;

        for end in permutation {
            cost += paths.cost(&grid.position_of(start).unwrap(), &grid.position_of(end).unwrap());
            start = end;
        }

        // Part 2: return to start!
        cost += paths.cost(&grid.position_of(start).unwrap(), &grid.position_of('0').unwrap());

        if cost < best_cost {
            best_cost = cost;
        }
    }

    println!("Shortest path: {}", best_cost);
}

*/

use std::collections::HashMap;

const DAY25_INPUT : &str = "
cpy a d
cpy 7 c
cpy 365 b
inc d
dec b
jnz b -2
dec c
jnz c -5
cpy d a
jnz 0 0
cpy a b
cpy 0 a
cpy 2 c
jnz b 2
jnz 1 6
dec b
dec c
jnz c -4
inc a
jnz 1 -7
cpy 2 b
jnz c 2
jnz 1 4
dec b
dec c
jnz 1 -4
jnz 0 0
out b
jnz a -19
jnz 1 -21
";

fn to_register(name: &str) -> char {
    name.chars().nth(0).unwrap()
}

fn deref_value(value: &str, registers: &HashMap<char, i64>) -> i64 {
    match to_register(value) {
        r @ 'a'...'z' => {
            *registers.get(&r).unwrap()
        },
        _ => { value.parse().unwrap() }
    }
}

fn run_program(init_value: i64) -> Option<Vec<i64>> {
    let mut instructions: Vec<String> = DAY25_INPUT.trim().split("\n").map(|s| {
        if &s[0..1] == "#" {
            "nop".to_string()
        } else {
            s.to_string()
        }}).collect();

    let mut registers = "abcd".chars().fold(HashMap::new(), |mut acc, register| {
        acc.insert(register, 0);
        acc
    });

    registers.insert('a', init_value);

    let mut pc: i64 = 0;

    let mut transmitted : Vec<i64> = Vec::new();

    loop {
        // println!("PC: {}; REGS: {:?}", pc, registers);

        if pc < 0 || pc >= (instructions.len() as i64) {
            break;
        }

        let instruction = (&instructions[pc as usize]).clone();
        let bits: Vec<&str> = instruction.split(" ").collect();

        match bits[0] {
            "out" => {
                let value = deref_value(bits[1], &registers);

                if value != 1 && value != 0 {
                    // No good.
                    return None;
                }

                if transmitted.len() > 0 && transmitted[transmitted.len() - 1] == value {
                    return None;
                }

                transmitted.push(value);

                if transmitted.len() > 1000 {
                    return Some(transmitted);
                }
            }

            "tgl" => {
                let offset = deref_value(bits[1], &registers);

                if (pc + offset) >= 0 && ((pc + offset) as usize) < instructions.len() {
                    println!{"Toggle: {} ({:?})", (pc + offset), instructions[(pc + offset) as usize]};
                    let target_instruction = (pc + offset) as usize;
                    let target_bits: Vec<String> = instructions[target_instruction].clone().split(" ").map(str::to_string).collect();

                    match target_bits.len() {
                        3 => {
                            if target_bits[0] == "jnz" {
                                instructions[target_instruction] = format!("{} {} {}", "cpy", target_bits[1], target_bits[2]);
                            } else {
                                instructions[target_instruction] = format!("{} {} {}", "jnz", target_bits[1], target_bits[2]);
                            }
                        },
                        2 => {
                            if target_bits[0] == "inc" {
                                instructions[target_instruction] = format!("{} {}", "dec", target_bits[1]);
                            } else {
                                instructions[target_instruction] = format!("{} {}", "inc", target_bits[1]);
                            }
                        },
                        _ => {
                            panic!("Invalid instruction: {:?}", target_bits);
                        }
                    };
                }
            },
            "cpy" => {
                let value = deref_value(bits[1], &registers);
                registers.insert(to_register(bits[2]), value);
            },
            "mul" => {
                let value1 = deref_value(bits[1], &registers);
                let value2 = deref_value(bits[2], &registers);
                registers.insert(to_register(bits[2]), value1 * value2);
            },
            "nop" => {},
            "inc" => {
                let new_value = deref_value(bits[1], &registers) + 1;
                registers.insert(to_register(bits[1]), new_value);
            },
            "dec" => {
                let new_value = deref_value(bits[1], &registers) - 1;
                registers.insert(to_register(bits[1]), new_value);
            },
            "jnz" => {
                let x = deref_value(bits[1], &registers);
                let y = deref_value(bits[2], &registers);

                if x != 0 {
                    // Compensate for the increment we're going to get anyway.
                    pc -= 1;
                    pc += y;
                }
            },
            _ => { panic!("WTF?! {}", bits[0]); },
        }

        pc += 1;
    }

    return None;
}


fn day25() {
    let mut i = 0;

    loop {
        let transmitted = run_program(i);

        match transmitted {
            Some(_) => {
                println!("Woot: {}", i);
            },
            None => {}
        }

        i += 1;
    }
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
    // day18();
    // day19();
    // day20();
    // day21();
    // day22();
    // day23();
    // day24();

    day25();

}




