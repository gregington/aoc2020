use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::path::Path;
use std::io;
use std::io::BufRead;
use std::fs::File;
use std::env;

fn part1(lines: &Vec<String>) {
    let mut active = parse_active_cubes(lines);
    active = run_simulation(&active, 6);
    // print_board(&active);
    println!("{}", active.len());
}

fn part2(lines: &Vec<String>) {
    println!("Part 2");
}

fn run_simulation(active: &HashSet<Cube>, count: u32) -> HashSet<Cube> {
    let mut prev = (*active).clone();
    let mut current: HashSet<Cube> = HashSet::new();

    for _ in 0..count {
        current = HashSet::new();

        let bounds = find_bounds(&prev);

        for x in bounds.x.to_owned() {
            for y in bounds.y.to_owned() {
                for z in bounds.z.to_owned() {
                    let cube = Cube { x, y, z };
                    let neighbors_active = count_active_neighbors(&prev, &cube);

                    if prev.contains(&cube) {
                        if neighbors_active == 2 || neighbors_active == 3 {
                            current.insert(cube);
                        }
                    } else {
                        if neighbors_active == 3 {
                            current.insert(cube);
                        }
                    }
                }
            }
        }

        prev = current.to_owned();
    }

    current
}

fn find_bounds(active: &HashSet<Cube>) -> Bounds {
    let min_x = active.iter().map(|cube| cube.x).min().unwrap();
    let max_x = active.iter().map(|cube| cube.x).max().unwrap();
    let min_y = active.iter().map(|cube| cube.y).min().unwrap();
    let max_y = active.iter().map(|cube| cube.y).max().unwrap();
    let min_z = active.iter().map(|cube| cube.z).min().unwrap();
    let max_z = active.iter().map(|cube| cube.z).max().unwrap();

    Bounds {
        x: min_x - 1..=max_x + 1,
        y: min_y - 1..=max_y + 1,
        z: min_z - 1..=max_z + 1
    }
}

fn count_active_neighbors(active: &HashSet<Cube>, cube: &Cube) -> i32 {
    let mut active_count = 0;

    for x in cube.x -1..=cube.x + 1 {
        for y in cube.y -1..=cube.y + 1 {
            for z in cube.z -1..=cube.z + 1 {
                let neighbor = Cube {x, y, z};
                if *cube == neighbor {
                    continue;
                }
                if active.contains(&neighbor) {
                    active_count += 1;
                }
            }
        }
    }

    active_count
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

fn print_board(active: &HashSet<Cube>) {
    let min_x = active.iter().map(|cube| cube.x).min().unwrap();
    let max_x = active.iter().map(|cube| cube.x).max().unwrap();
    let min_y = active.iter().map(|cube| cube.y).min().unwrap();
    let max_y = active.iter().map(|cube| cube.y).max().unwrap();
    let min_z = active.iter().map(|cube| cube.z).min().unwrap();
    let max_z = active.iter().map(|cube| cube.z).max().unwrap();

    for z in min_z..=max_z {
        println!();
        println!("z={z}");

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let cube = Cube{x, y, z};
                let c = if active.contains(&cube) { '#' } else { '.' };
                print!("{c}");
            }
            println!();
        }
    }
}

fn parse_active_cubes(lines: &Vec<String>) -> HashSet<Cube> {
    let mut active = HashSet::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active.insert(Cube {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(), 
                    z: 0
                });
            }
        }
    }

    active
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Cube {
    x: i32,
    y: i32,
    z: i32
}

struct Bounds {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>
}