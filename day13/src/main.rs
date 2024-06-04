use std::fs::File;
use std::io;
use std::env;
use std::io::BufRead;
use std::path::Path;

fn part1(earliest_time: i64, bus_ids: &str) {
    let bus_ids = parse_bus_ids(bus_ids);

    let mut earliest_bus_times: Vec<(i64, i64)> = bus_ids.iter()
        .map(|id| {
            let time_before = (earliest_time / id) * id;
            let earliest_bus_time = if time_before == earliest_time { earliest_time } else { time_before + id };
            (*id, earliest_bus_time)
        })
        .collect();
    earliest_bus_times.sort_by_key(|x| x.1);
    let (id, earliest_bus_time) = earliest_bus_times[0];
    let product = id * (earliest_bus_time - earliest_time);
    println!("{product}");
}

fn part2(earliest_time: i64, bus_ids: &str) {
    println!("Part 2");
}

fn parse_bus_ids(bus_ids: &str) -> Vec<i64> {
    bus_ids.split(',')
        .filter(|x| *x != "x")
        .map(|x| x.parse().unwrap())
        .collect()
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

    let earliest_time: i64 = lines[0].parse().unwrap();
    let bus_ids = &lines[1];

    match part {
        1 => part1(earliest_time, &bus_ids),
        2 => part2(earliest_time, &bus_ids),
        _ => println!("Invalid part number"),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}