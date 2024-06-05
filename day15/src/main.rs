use std::collections::HashMap;
use std::path::Path;
use std::io::BufRead;
use std::io;
use std::fs::File;
use std::env;

fn part1(numbers: &[i32]) {
    let num_spoken = play(numbers, 2020);
    println!("{num_spoken}");
}

fn part2(numbers: &[i32]) {
    let num_spoken = play(numbers, 30000000);
    println!("{num_spoken}");
}

fn play(numbers: &[i32], count: i32) -> i32 {
    let mut number_turns: HashMap<i32, [i32; 2]> = HashMap::new();

    for (pos, num) in numbers.iter().enumerate() {
        number_turns.insert(*num, [pos as i32, -1]);
    }

    let mut last_num = *numbers.last().unwrap();
    for turn in numbers.len() as i32..count {
        let times_spoken = &number_turns[&last_num];
        let next_num = if times_spoken[1] == -1 {
            0
        } else {
            times_spoken[0] - times_spoken[1]
        };

        if !number_turns.contains_key(&next_num) {
            number_turns.insert(next_num, [turn as i32, -1]);
        } else {
            let mut times_spoken = number_turns[&next_num];
            times_spoken[1] = times_spoken[0];
            times_spoken[0] = turn as i32;
            number_turns.insert(next_num, times_spoken);
        }

        last_num = next_num;
    }

    last_num
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

    let numbers = lines[0].split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    match part {
        1 => part1(&numbers),
        2 => part2(&numbers),
        _ => println!("Invalid part number"),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}