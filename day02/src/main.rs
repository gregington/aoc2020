use regex::Regex;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::vec::Vec;

fn part1() {
    let lines = read_lines("input.txt").unwrap();
    let regex = create_regex();
    let policies_and_passwords = Vec::from_iter(lines.map(|line| read_line(&regex, &line.unwrap())));
    let num_valid = policies_and_passwords.iter().filter(|x| is_valid(&x.0, &x.1)).count();
    println!("{num_valid}");
}

fn part2() {
    println!("Part 2");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args[1].parse::<i32>().unwrap();

    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Invalid part number"),
    }
}


fn create_regex() -> Regex {
    Regex::new(r"^(?<min>\d+)-(?<max>\d+) (?<char>.): (?<password>.+)$").unwrap()
}

fn read_line(re: &Regex, line: &str) -> (PasswordPolicy, String) {
    let caps = re.captures(line).unwrap();
    let policy = PasswordPolicy {
        min: caps["min"].parse::<i32>().unwrap(),
        max: caps["max"].parse::<i32>().unwrap(),
        character: caps["char"].chars().next().unwrap()
    };
    let mut password = String::new();
    caps["password"].clone_into(&mut password);

    (policy, password)
}

fn is_valid(policy: &PasswordPolicy, password: &str) -> bool {
    let count = password.chars().filter(|c| *c == policy.character).count() as i32;
    count >= policy.min && count <= policy.max
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
struct PasswordPolicy {
    min: i32,
    max: i32,
    character: char 
}