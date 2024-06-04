#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::env;
use std::io::BufRead;
use std::path::Path;

lazy_static! {
    static ref MASK_REGEX: Regex = Regex::new(r"^mask = (?<mask>[X10]+)$").unwrap();
    static ref ASSIGN_REGEX: Regex = Regex::new(r"^mem\[(?<address>\d+)\] = (?<value>\d+)$").unwrap();
}

fn part1(lines: &Vec<String>) {
    let memory = run_program(lines);
    let memory_sum: u64 = memory.iter()
        .map(|x| x.1)
        .sum();

    println!("{memory_sum}");
}

fn part2(lines: &Vec<String>) {
    println!("Part 2");
}

fn run_program(lines: &Vec<String>) -> HashMap<u32, u64> {
    let mut memory = HashMap::new();
    let mut mask: [char; 36] = (0..36)
        .map(|_| 'X')
        .collect::<Vec<char>>()
        .try_into()
        .unwrap();

    for line in lines.iter() {
        let mask_captures = MASK_REGEX.captures(line);
        if mask_captures.is_some() {
            let mask_captures = mask_captures.unwrap();
            mask = mask_captures["mask"].chars().into_iter()
                .collect::<Vec<char>>()
                .try_into()
                .unwrap();
            continue;
        }

        let assign_captures = ASSIGN_REGEX.captures(line);
        if assign_captures.is_some() {
            let assign_captures = assign_captures.unwrap();
            let address: u32 = assign_captures["address"].parse().unwrap();
            let raw_value:u64 = assign_captures["value"].parse().unwrap();
            let value = apply_mask(raw_value, &mask);
            memory.insert(address, value);
            continue;
        }

        panic!("Could not parse line");
    }

    memory
}


fn apply_mask(value: u64, mask: &[char; 36]) -> u64 {
    let mut result = 0;

    for bit in 0..36 {
        let bit_mask: u64 = 1 << bit;
        let bit_value = (value & bit_mask) >> bit;

        let masked_bit = match mask[35 - bit] {
            '0' => 0,
            '1' => 1,
            'X' => bit_value,
            _ => panic!("Unknown mask char")
        };

        result = result | (masked_bit << bit);
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args[1].parse::<i32>().unwrap();

    let mut filename = "input.txt";
    if args.len() > 2 {
        filename = &args[2];
    }

    let lines: Vec<String> = read_lines(&filename).unwrap()
        .map(|x| x.unwrap()).collect();

    match part {
        1 => part1(&lines),
        2 => part2(&lines),
        _ => println!("Invalid part number"),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}