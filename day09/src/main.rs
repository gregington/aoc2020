use std::{collections::HashSet, env, fs::File, io::{self, BufRead}, path::Path};

fn part1(lines: &Vec<String>, memory: i32) {
    let numbers = read_numbers(lines);
    let invalid_number = find_invalid_number(&numbers, memory);
    println!("{invalid_number}");
}

fn part2(lines: &Vec<String>) {
    println!("Part 2");
}

fn find_invalid_number(numbers: &[i64], memory: i32) -> i64 {
    let memory = memory as usize;
    for i in memory..numbers.len() {
        let mem: HashSet<i64> = HashSet::from_iter(numbers[i - memory..i].iter().cloned());
        let num = numbers[i];

        if !contains_sum(&mem, num) {
            return num;
        }
    }
    panic!("All valid")
}

fn contains_sum(mem: &HashSet<i64>, num: i64) -> bool {
    for a in mem.iter() {
        let target = num - a;
        if target == *a {
            continue;
        }

        if mem.contains(&target) {
            return true;
        }
    }
    false
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

    let memory = if filename == "input.txt" { 25 } else { 5 };

    match part {
        1 => part1(&lines, memory),
        2 => part2(&lines),
        _ => println!("Invalid part number"),
    }
}

fn read_numbers(lines: &Vec<String>) -> Vec<i64> {
    lines.iter().map(|line| line.parse().unwrap()).collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
