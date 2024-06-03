use regex::Regex;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::vec::Vec;

fn part1(lines: &Vec<String>) {
    let rules = parse_rules(lines);
    let mut queue = VecDeque::from(["shiny gold"]);
    let mut results: HashSet<&String> = HashSet::new();

    while !queue.is_empty() {
        let bag = queue.pop_back().unwrap();

        let iter = rules.iter()
            .filter(|x| x.1.contains_key(bag))
            .map(|x| x.0);

        for containing_bag in iter {
            queue.push_front(&containing_bag);
            results.insert(&containing_bag);
        }
    }

    let count = results.len();
    println!("{count}");
}

fn part2(lines: &Vec<String>) {
    println!("Part 2");
}

fn parse_rules(lines: &Vec<String>) -> HashMap<String, HashMap<String, u8>> {
    let rule_regex = Regex::new(r"^(?<description>.+) bags contain (?<contents>.+)\.$").unwrap();
    let contents_regex = Regex::new(r"\d+ [a-z ]+").unwrap();
    let mut rules: HashMap<String, HashMap<String, u8>> = HashMap::new();

    for line in lines {
        let capture = rule_regex.captures(line);
        if capture.is_none() {
            continue;
        }
        let capture = capture.unwrap();
        let description = capture["description"].to_string();
        let contents = capture["contents"].borrow();
        let contents = parse_contents(&contents_regex, &contents);
        rules.insert(description, contents);
    }

    rules
}

fn parse_contents(re: &Regex, contents: &str) -> HashMap<String, u8> {
    if contents == "no other bags" {
        return HashMap::new();
    }

    let map = re.find_iter(contents)
        .map(|x| {
            let m = x.as_str();
            let idx = m.find(' ').unwrap();
            let count = m[..idx].parse::<u8>().unwrap();
            let description = &m[idx + 1..];
            let idx2 = description.rfind(' ').unwrap();
            let description = description[..idx2].to_string();
            (description, count)
        })
        .collect::<HashMap<String, u8>>();

    map
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args[1].parse::<i32>().unwrap();

    let mut filename = "input.txt";
    if args.len() > 2 {
        filename = &args[2];
    }

    let lines = read_lines(&filename).unwrap()
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
