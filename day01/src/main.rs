use std::collections::HashSet;
use std::io::BufRead;
use std::{env, io};
use std::fs::File;
use std::path::Path;

fn part1() {
    let lines = read_lines("input.txt").unwrap();
    let nums_iter = lines.map(|line| line.unwrap().parse::<i32>().unwrap());
    let nums_set: HashSet<i32> = HashSet::from_iter(nums_iter);

    for num in nums_set.iter()  {
        if nums_set.contains(&(2020 - num)) {
            println!("{}", num * (2020 - num));
            break;
        }
    }
}

fn part2() {
    let lines = read_lines("input.txt").unwrap();
    let nums_iter = lines.map(|line| line.unwrap().parse::<i32>().unwrap());
    let nums_set: HashSet<i32> = HashSet::from_iter(nums_iter);

    for num1 in nums_set.iter() {
        for num2 in nums_set.iter() {
            if num1 == num2 {
                continue;
            }
            let target = 2020 - num1 - num2;
            if target == *num1 || target == *num2 {
                continue;
            }
            if nums_set.contains(&target) {
                println!("{}", num1 * num2 * target);
                return;
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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