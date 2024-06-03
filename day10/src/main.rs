use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io;
use std::path::Path;

fn part1(lines: &Vec<String>) {
    let mut numbers = read_numbers(lines);
    numbers.push(0);
    numbers.sort_unstable();
    numbers.push(numbers.last().unwrap() + 3);

    let (diff_1, diff_3) = count_differences(&numbers);
    let mul = diff_1 * diff_3;
    println!("{mul}");
}

fn part2(lines: &Vec<String>) {
    let mut numbers = read_numbers(lines);
    numbers.push(0);
    numbers.sort_unstable();
    numbers.push(numbers.last().unwrap() + 3);

    let num_set: HashSet<i64> = HashSet::from_iter(numbers.iter().cloned());

    let target = numbers.last().unwrap() + 3;
    let mut cache: HashMap<i64, i64> = HashMap::new();

    let num_combinations = find_num_combinations(&num_set, &mut cache, target, 0);
    println!("{num_combinations}");
}

fn find_num_combinations(numbers: &HashSet<i64>, cache: &mut HashMap<i64, i64>, target: i64, num: i64) -> i64 {
    if cache.contains_key(&num) {
        return cache[&num];
    }

    let mut count: i64 = 0;

    if num == target {
        count = 1;
    } else if num < target {
        for next in num + 1..=num + 3 {
            if numbers.contains(&num) {
                count += find_num_combinations(numbers, cache, target, next);
            }
        }
    }

    cache.insert(num, count);
    count
}

fn count_differences(numbers: &[i64]) -> (i64, i64) {
    let it = numbers.iter();
    let lagged_it = numbers.iter().skip(1);
    let zipped = lagged_it.zip(it);
    let differences = zipped.map(|x| x.0 - x.1).collect::<Vec<i64>>();

    let diff_1_count = differences.iter().filter(|x| **x == 1).count() as i64;
    let diff_2_count = differences.iter().filter(|x| **x == 3).count() as i64;

    (diff_1_count, diff_2_count)
}

fn read_numbers(lines: &Vec<String>) -> Vec<i64> {
    lines.iter().map(|line| line.parse().unwrap()).collect()
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
