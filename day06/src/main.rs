use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::vec::Vec;

fn part1(lines: &Vec<String>) {
    let answer_groups = create_answer_groups(lines);
    let sum: u64 = answer_groups.iter().map(|x| x.len() as u64).sum();
    println!("{sum}");
}

fn part2(lines: &Vec<String>) {
    println!("Part 2");
}

fn create_answer_groups(lines: &Vec<String>) -> Vec<HashSet<char>> {
    let mut groups: Vec<HashSet<char>> = Vec::new();
    let mut answers: HashSet<char> = HashSet::new();

    for line in lines {
        if line.trim().is_empty() && !answers.is_empty() {
            groups.push(answers);
            answers = HashSet::new();
            continue;
        }

        for c in line.chars() {
            answers.insert(c);
        }
    }

    if !&answers.is_empty() {
        groups.push(answers);
    }

    groups
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
