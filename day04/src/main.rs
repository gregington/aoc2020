use std::collections::HashMap;
use std::env;
use std::io;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

fn part1(passports: &Vec<HashMap<String, String>>) {
    let valid_count = passports.iter().filter(|passport| is_valid(passport)).count();
    println!("{valid_count}");
}

fn part2() {
    println!("Part 2");
}

fn read_passports(filename: &str) -> Vec<HashMap<String, String>> {
    let lines = read_lines(filename).unwrap();
    let mut passports: Vec<HashMap<String, String>> = Vec::new();
    let mut passport: HashMap<String, String> = HashMap::new();

    for line in lines {
        let  line = line.unwrap();
        if line.trim().len() == 0 {
            if passport.len() > 0
            {
                passports.push(passport);
                passport = HashMap::new();
            }
            continue;
        }

        let mappings = line.split(' ');

        for mapping in mappings.into_iter() {
            let mut kvp = mapping.split(':');
            let key = kvp.nth(0).unwrap().to_string();
            let value = kvp.nth(0).unwrap().to_string();

            passport.insert(key, value);
        }
    }

    if passport.len() > 0 {
        passports.push(passport);
    }

    passports
}

fn is_valid(passport: &HashMap<String, String>) -> bool {
    passport.contains_key("byr")
        && passport.contains_key("iyr")
        && passport.contains_key("eyr")
        && passport.contains_key("hgt")
        && passport.contains_key("hcl")
        && passport.contains_key("ecl")
        && passport.contains_key("pid")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args[1].parse::<i32>().unwrap();

    let mut filename = "input.txt";
    if args.len() > 2 {
        filename = &args[2];
    }

    let passports = read_passports(&filename);

    match part {
        1 => part1(&passports),
        2 => part2(),
        _ => println!("Invalid part number"),
    }
}
