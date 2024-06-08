use std::fs::File;
use std::io;
use std::env;
use std::io::BufRead;
use std::path::Path;

fn part1(lines: &Vec<String>) {
    let sum: u64 = lines.iter().map(|expr| evaluate(expr, parse_operator)).sum();
    println!("{sum}");
}

fn part2(lines: &Vec<String>) {
    let sum: u64 = lines.iter().map(|expr| evaluate(expr, parse_operator_precedence)).sum();
    println!("{sum}");
}

fn evaluate(expr: &str, parse_operator_fn: fn(&str) -> (&str, char, &str)) -> u64 {
    let expr = expr.trim();

    let parsed = expr.parse::<u64>();
    if parsed.is_ok() {
        return parsed.unwrap();
    }

    if expr.starts_with('(') {
        let closing_brace_idx = find_closing_brace_idx(&expr);
        let lhs = evaluate(&expr[1..closing_brace_idx], parse_operator_fn);        
        return evaluate(&format!("{} {}", lhs, &expr[closing_brace_idx + 1..]), parse_operator_fn)
    }

    let (lhs, operator, rhs) = parse_operator_fn(expr);

    let lhs = evaluate(lhs, parse_operator_fn);
    let rhs = evaluate(rhs, parse_operator_fn);

    if operator == '+' {
        return lhs + rhs;
    }
    
    return lhs * rhs
}

fn find_closing_brace_idx(expr: &str) -> usize {
    let mut iter = expr.char_indices();
    iter.next().unwrap();

    let mut idx: usize = 0;
    let mut count = 1;
    while count > 0 {
        let val = iter.next().unwrap();
        idx = val.0;
        let c = val.1;
        if c == '(' {
            count += 1;
        } else if c == ')' {
            count -= 1;
        }
    }

    idx
}

fn parse_operator(expr: &str) -> (&str, char, &str) {
    let chars: Vec<char> = expr.chars().collect();
    let mut brace_count = 0;
    for idx in (0..chars.len()).rev() {
        let c = chars[idx];
        if c == '(' {
            brace_count += 1;
        } else if c == ')' {
            brace_count -= 1;
        } else if brace_count == 0 && (c == '+' || c == '*') {
            return (&expr[..idx], expr[idx..=idx].chars().next().unwrap(), &expr[idx+2..])
        }
    }

    panic!("Reached end of string");
}

fn parse_operator_precedence(expr: &str) -> (&str, char, &str) {
    let chars: Vec<char> = expr.chars().collect();
    let mut brace_count = 0;
    for idx in (0..chars.len()).rev() {
        let c = chars[idx];
        if c == '(' {
            brace_count += 1;
        } else if c == ')' {
            brace_count -= 1;
        } else if brace_count == 0 && c == '*' {
            return (&expr[..idx], expr[idx..=idx].chars().next().unwrap(), &expr[idx+2..])
        }
    }

    brace_count = 0;
    for idx in (0..chars.len()).rev() {
        let c = chars[idx];
        if c == '(' {
            brace_count += 1;
        } else if c == ')' {
            brace_count -= 1;
        } else if brace_count == 0 && c == '+' {
            return (&expr[..idx], expr[idx..=idx].chars().next().unwrap(), &expr[idx+2..])
        }
    }

    panic!("Reached end of string");
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