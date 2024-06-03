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
    println!("Part 2");
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
