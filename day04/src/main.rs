use std::collections::HashMap;
use std::env;
use std::io;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

fn part1(passports: &Vec<HashMap<String, String>>) {
    let valid_count = passports.iter().filter(|passport| contains_required_fields(passport)).count();
    println!("{valid_count}");
}

fn part2(passports: &Vec<HashMap<String, String>>) {
    let valid_count = passports.iter().filter(|passport| is_valid(passport)).count();
    println!("{valid_count}");
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

fn contains_required_fields(passport: &HashMap<String, String>) -> bool {
    passport.contains_key("byr")
        && passport.contains_key("iyr")
        && passport.contains_key("eyr")
        && passport.contains_key("hgt")
        && passport.contains_key("hcl")
        && passport.contains_key("ecl")
        && passport.contains_key("pid")
}

fn is_valid(passport: &HashMap<String, String>) -> bool {
    contains_required_fields(passport)
        && valid_year(&passport["byr"], 1920, 2002)
        && valid_year(&passport["iyr"], 2010, 2020)
        && valid_year(&passport["eyr"], 2020, 2030)
        && valid_height(&passport["hgt"])
        && valid_hair_color(&passport["hcl"])
        && valid_eye_color(&passport["ecl"])
        && valid_passport_id(&passport["pid"])
}

fn valid_year(year: &str, min: i32, max: i32) -> bool {
    if year.len() != 4 {
        return false;
    }

    let parse_result: Result<i32, _> = year.parse();
    if parse_result.is_err() {
        return false;
    }

    let year = parse_result.ok().unwrap();
    year >=min && year <= max
}

fn valid_height(height: &str) -> bool {
    let unit = &height[height.len() - 2..];

    if !(unit == "in" || unit == "cm") {
        return false;
    }

    let value_result: Result<i32, _> = height[..height.len() - 2].parse();
    if value_result.is_err() {
        return false;
    }

    let value = value_result.ok().unwrap();

    if unit == "cm" {
        return value >= 150 && value <= 193;
    }

    value >= 59 && value <= 76
}

fn valid_hair_color(color: &str) -> bool {
    if color.len() != 7 {
        return false;
    }

    if !color.starts_with("#") {
        return false;
    }

    color[1..].chars().all(|c| (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f'))
}

fn valid_eye_color(color: &str) -> bool {
    color == "amb"
        || color == "blu"
        || color == "brn"
        || color == "gry"
        || color == "grn"
        || color == "hzl"
        || color == "oth"
}

fn valid_passport_id(id: &str) -> bool {
    id.len() == 9 && id.chars().all(|c| c.is_ascii_digit())
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
        2 => part2(&passports),
        _ => println!("Invalid part number"),
    }
}
