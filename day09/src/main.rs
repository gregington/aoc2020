use std::{cmp::{max, min}, collections::HashSet, env, fs::File, io::{self, BufRead}, path::Path};

fn part1(lines: &Vec<String>, memory: i32) {
    let numbers = read_numbers(lines);
    let invalid_number = find_invalid_number(&numbers, memory);
    println!("{invalid_number}");
}

fn part2(lines: &Vec<String>, memory: i32) {
    let numbers = read_numbers(lines);
    let invalid_number = find_invalid_number(&numbers, memory);
    let (min, max) = find_contiguous_sum(&numbers, invalid_number);
    let sum = min + max;
    println!("{sum}");
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

fn find_contiguous_sum(numbers: &[i64], invalid_number: i64) -> (i64, i64) {
    for i in 0..numbers.len() {
        let slice = &numbers[i..];

        let result = find_contiguous_slice(&slice, invalid_number);
        if result.is_none() {
            continue;
        }

        return result.unwrap();
    }

    panic!("No contiguous sum found");
}

fn find_contiguous_slice(numbers: &[i64], invalid_number: i64) -> Option<(i64, i64)> {
    let mut sum:i64 = 0;

    let mut min_num = i64::MAX;
    let mut max_num = i64::MIN;

    for num in numbers.iter() {
        min_num = min(*num, min_num);
        max_num = max(*num, max_num);
        sum += *num;

        if sum == invalid_number {
            return Some((min_num, max_num));
        }

        if sum > invalid_number {
            return None;
        }
    }

    None
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
        2 => part2(&lines, memory),
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
