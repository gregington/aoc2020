use std::collections::HashSet;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn part1(rules: &Vec<String>, messages: &Vec<String>) {
    let possible_messages: HashSet<String> = generate_messages(&rules, "0");
    let matches = messages.iter().filter(|message| possible_messages.contains(*message)).count();
    println!("{matches}");
}

fn part2(rules: &Vec<String>, messages: &Vec<String>) {
    println!("Part 2");
}

fn generate_messages(rules: &Vec<String>, rule_expr: &str) -> HashSet<String> {
    if rule_expr.starts_with('"') {
        let val = rule_expr[1..rule_expr.len() - 1].to_owned();
        return HashSet::from([val]);
    } else if rule_expr.contains('|') {
        let mut iter = rule_expr.split('|');
        let lhs = iter.next().unwrap().trim();
        let rhs = iter.next().unwrap().trim();

        let mut results = HashSet::new();
        results.extend(generate_messages(rules, lhs));
        results.extend(generate_messages(rules, rhs));

        return results;
    } else if rule_expr.contains(' ') {
        let mut results: HashSet<String> = HashSet::from(["".to_owned()]);

        for subrule in rule_expr.split(' ') {
            let part_results = generate_messages(rules, subrule);

            let mut new_results = HashSet::new();
            for prev_result in results {
                for part_result in &part_results {
                    new_results.insert(format!("{prev_result}{part_result}"));
                }
            }
            results = new_results;
        }
        return results;
    } else {
        let new_rule_expr = &rules[rule_expr.parse::<usize>().unwrap()];
        return generate_messages(rules, new_rule_expr);
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

fn parse_input(lines: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let rule_lines: Vec<String> = lines.iter().take_while(|s| s.contains(':'))
        .map(|x| x.to_owned())
        .collect();

    let rules = parse_rules(&rule_lines);

    let message_lines = lines.iter().skip_while(|x| !x.starts_with('a') && !x.starts_with('b'))
        .map(|x| (*x).to_owned())
        .collect();

    (rules, message_lines)
}

fn parse_rules(lines: &Vec<String>) -> Vec<String> {
    let rule_hashmap: HashMap<u32, &str> = lines.iter()
        .map(|x| {
            let idx = x.find(':').unwrap();
            let rule_num: u32 = x[..idx].parse().unwrap();
            let rule = &x[idx+2..];
            (rule_num, rule)
        })
        .collect();

    let max_rule_num = rule_hashmap.keys().max().unwrap();
    let mut results = Vec::new();
    for i in 0..=*max_rule_num {
        results.push(rule_hashmap[&i].to_owned());
    }
    results
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}