use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::ops::Range;
use std::env;
use std::io::BufRead;
use std::path::Path;

use regex::Regex;

fn part1(input: &Input) {
    let invalid_fields = find_invalid_fields(input);
    println!("{}", invalid_fields.iter().sum::<u32>());
}

fn part2(input: &Input) {
    let valid_nearby_tickets = find_valid_nearby_tickets(input);
    let field_mappings = find_field_mappings(&input.fields, valid_nearby_tickets);

    let departure_cols: Vec<u32> = field_mappings.iter()
        .filter(|x| x.0.starts_with("departure"))
        .map(|x| *x.1)
        .collect();

    let val: u64 = departure_cols.iter()
        .map(|c| input.my_ticket[*c as usize] as u64)
        .product();

    println!("{val}");
}

fn find_invalid_fields(input: &Input) -> Vec<u32> {
    let mut invalid = Vec::new();

    for ticket in &input.nearby_tickets {
        let inv = ticket.iter().filter(|x| input.fields.values().all(|f| !f[0].contains(*x) && !f[1].contains(*x)));
        for x in inv {
            invalid.push(*x);
        }
    }
    invalid
}

fn find_valid_nearby_tickets(input: &Input) -> Vec<Vec<u32>> {
    let mut valid = Vec::new();

    for ticket in &input.nearby_tickets {
        let invalid_values: Vec<&u32> = ticket.iter().filter(|x| input.fields.values().all(|f| !f[0].contains(*x) && !f[1].contains(*x))).collect();
        if invalid_values.is_empty() {
            valid.push(ticket.clone());
        }
    }

    valid
}

fn find_field_mappings(fields: &HashMap<String, [Range<u32>; 2]>, tickets: Vec<Vec<u32>>) -> HashMap<String, u32> {
    let mut results = HashMap::new();
    let field_vals = transpose(&tickets);

    let mut fields_remaining: HashSet<&String> = HashSet::from_iter(fields.keys());
    let mut cols_remaining: HashSet<u32> = (0..field_vals.len() as u32).collect();

    while !fields_remaining.is_empty() {
        let mut all_candidates: HashMap<String, Vec<u32>> = HashMap::new();

        for field in fields_remaining.clone() {
            let mut candidates: Vec<u32> = Vec::new();
            let range0 = &fields[field][0];
            let range1 = &fields[field][1];

            for col in cols_remaining.iter() {
                if field_vals[*col as usize].iter().all(|v| range0.contains(v) || range1.contains(v)) {
                    candidates.push(*col);
                }
            }

            all_candidates.insert(field.clone(), candidates);
        }

        let matched_candidates: HashMap<String, Vec<u32>> = all_candidates.iter()
            .filter(|x| x.1.len() == 1)
            .map(|x| (x.0.clone(), x.1.clone()))
            .collect();

            for matched in matched_candidates.iter() {
            fields_remaining.remove(matched.0);
            cols_remaining.remove(&matched.1[0]);
            results.insert((*matched.0).clone(), matched.1[0]);
        }
    }

    results
}

fn transpose(input: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut results = Vec::new();

    for i in 0..input[0].len() {
        let mut row = Vec::new();
        for j in 0..input.len() {
            row.push(input[j][i])
        }
        results.push(row);
    }

    results
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

    let input = parse_input(&lines);

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part number"),
    }
}

fn parse_input(lines: &[String]) -> Input {
    let fields_lines = lines.iter().take_while(|line| line.contains(':')).collect::<Vec<&String>>();
    let fields = parse_fields(&fields_lines);

    let my_ticket = lines.iter().skip_while(|line| !line.starts_with("your ticket")).skip(1).next().unwrap();
    let my_ticket = parse_ticket(my_ticket);

    let nearby_tickets_iter = lines.iter().skip_while(|line| !line.starts_with("nearby tickets")).skip(1);
    let nearby_tickets = nearby_tickets_iter.map(|line| parse_ticket(line)).collect();

    Input {
        fields,
        my_ticket,
        nearby_tickets
    }
}

fn parse_fields(lines: &Vec<&String>) -> HashMap<String, [Range<u32>; 2]> {
    let regex = Regex::new(r"^(?<key>.+): (?<r1_from>\d+)-(?<r1_to>\d+) or (?<r2_from>\d+)-(?<r2_to>\d+)").unwrap();
    let mut fields = HashMap::new();
    
    for line in lines.iter() {
        let captures = regex.captures(line).unwrap();
        let key = captures["key"].to_owned();
        let range1 = captures["r1_from"].parse::<u32>().unwrap()..captures["r1_to"].parse::<u32>().unwrap() + 1;
        let range2 = captures["r2_from"].parse::<u32>().unwrap()..captures["r2_to"].parse::<u32>().unwrap() + 1;

        fields.insert(key, [range1, range2]);
    }

    fields
}

fn parse_ticket(line: &str) -> Vec<u32> {
    line.split(',').map(|x| x.parse().unwrap()).collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Input {
    fields: HashMap<String, [Range<u32>; 2]>,
    my_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>
}