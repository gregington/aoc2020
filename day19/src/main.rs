use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

use regex::Regex;

fn part1(rules: &HashMap<u32, String>, messages: &Vec<String>) {
    let pattern = format!("^{}$", &generate_regex_pattern(rules, "0"));
    let regex = Regex::new(&pattern).unwrap();

    let matches = messages.iter().filter(|message| regex.is_match(*message)).count();
    println!("{matches}");
}

fn part2(rules: &HashMap<u32, String>, messages: &Vec<String>) {
    let mut new_rules = rules.clone();
    
    let rule8_reps = 5;
    let mut rule8 = String::new();
    for i in 0..rule8_reps {
        for _ in 0..=i {
            rule8.push_str("42 ")
        }
        if i != rule8_reps - 1 {
            rule8.push_str("| ");
        }
    }
    new_rules.insert(8, rule8);

    let rule11_reps = 4;
    let mut rule11 = String::new();
    for i in 0..rule11_reps {
        for _ in 0..=i {
            rule11.push_str("42 ")
        }
        for _ in 0..=i {
            rule11.push_str("31 ")
        }
        if i != rule11_reps - 1 {
            rule11.push_str("| ")
        }
    }
    new_rules.insert(11, rule11);

    let pattern = format!("^{}$", &generate_regex_pattern(&new_rules, "0"));
    let regex = Regex::new(&pattern).unwrap();

    let matches = messages.iter().filter(|message| regex.is_match(*message)).count();
    println!("{matches}");
}

fn generate_regex_pattern(rules: &HashMap<u32, String>, rule_expr: &str) -> String {
    let rule_expr = rule_expr.trim();
    if rule_expr.starts_with('"') {
        return rule_expr[1..rule_expr.len() - 1].to_owned();
    } else if rule_expr.contains('|') {
        let sub_patterns = rule_expr.split("|")
            .map(|x| generate_regex_pattern(rules, x))
            .collect::<Vec<String>>()
            .join("|");
        return format!("({sub_patterns})");
    } else if rule_expr.contains(' ') {
        let mut results = String::from("");
        for subrule in rule_expr.split(' ') {
            let part_results = generate_regex_pattern(&rules, subrule);
            results = format!("{results}{part_results}");
        }        
        return results;
    } else {
        let new_rule_expr = &rules[&rule_expr.parse::<u32>().unwrap()];
        return generate_regex_pattern(rules, new_rule_expr);
    }
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

    let (rules, messages) = parse_input(&lines);

    match part {
        1 => part1(&rules, &messages),
        2 => part2(&rules, &messages),
        _ => println!("Invalid part number"),
    }
}

fn parse_input(lines: &Vec<String>) -> (HashMap<u32, String>, Vec<String>) {
    let rule_lines: Vec<String> = lines.iter().take_while(|s| s.contains(':'))
        .map(|x| x.to_owned())
        .collect();

    let rules = parse_rules(&rule_lines);

    let message_lines = lines.iter().skip_while(|x| !x.starts_with('a') && !x.starts_with('b'))
        .map(|x| (*x).to_owned())
        .collect();

    (rules, message_lines)
}

fn parse_rules(lines: &Vec<String>) -> HashMap<u32, String> {
    lines.iter()
        .map(|x| {
            let idx = x.find(':').unwrap();
            let rule_num: u32 = x[..idx].parse().unwrap();
            let rule = x[idx+2..].to_owned();
            (rule_num, rule)
        })
        .collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}