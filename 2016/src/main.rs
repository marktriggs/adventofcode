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

*/

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

    day15();
}
