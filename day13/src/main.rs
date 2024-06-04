use std::fs::File;
use std::io;
use std::env;
use std::io::BufRead;
use std::path::Path;

fn part1(earliest_time: i128, bus_ids: &str) {
    let bus_ids = parse_bus_ids(bus_ids);

    let mut earliest_bus_times: Vec<(i128, i128)> = bus_ids.iter()
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

fn part2(_earliest_time: i128, bus_ids: &str) {
    let congruences = parse_congruences(&bus_ids);

    let mut solution: i128 = 0;
    let mut step_size: i128 = 1;

    for c in congruences {
        for timestep in (solution..i128::MAX).step_by(step_size as usize) {
            if (timestep + c.a) % c.m == 0 {
                solution = timestep;
                step_size *= c.m;
                break;
            }
        }
    }

    println!("{solution}");
}

fn parse_bus_ids(bus_ids: &str) -> Vec<i128> {
    bus_ids.split(',')
        .filter(|x| *x != "x")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn parse_congruences(bus_ids: &str) -> Vec<Congruence> {
    let parts: Vec<&str> = bus_ids.split(',').collect();
    let mut congruences: Vec<Congruence> = Vec::new();

    for i in 0..parts.len() {
        if parts[i] == "x" {
            continue;
        }
        congruences.push(Congruence {
            a: i as i128,
            m: parts[i].parse().unwrap()
        });
    }

    congruences
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

    let earliest_time: i128 = lines[0].parse().unwrap();
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

struct Congruence {
    a: i128,
    m: i128
}