use std::fs::File;
use std::io;
use std::env;
use std::io::BufRead;
use std::path::Path;

fn part1(lines: &Vec<String>) {
    let instructions = parse_instructions(lines);
    let start = (0, 0);
    let end = navigate(&instructions, start);
    let distance = manhattan_distance(end, start);
    println!("{distance}");
}

fn part2(lines: &Vec<String>) {
    let instructions = parse_instructions(lines);
    let start = (0, 0);
    let end = navigate_with_waypoint(&instructions, start);
    let distance = manhattan_distance(end, start);
    println!("{distance}");
}

fn navigate(instructions: &[Instruction], start_pos: (i32, i32)) -> (i32, i32) {
    let mut x = start_pos.0;
    let mut y = start_pos.1;
    let mut direction = 90;

    for instruction in instructions {
        match instruction.action {
            'N' => y += instruction.value,
            'S' => y -= instruction.value,
            'E' => x += instruction.value,
            'W' => x -= instruction.value,
            'L' => direction = (direction - instruction.value + 360) % 360,
            'R' => direction = (direction + instruction.value + 360) % 360,
            'F' => {
                match direction {
                    0 => y += instruction.value,
                    90 => x += instruction.value,
                    180 => y -= instruction.value,
                    270 => x -= instruction.value,
                    _ => panic!("Unknown direction")
                }
            },
            _ => panic!("Unknown action")
        }
    }

    (x, y)
}

fn navigate_with_waypoint(instructions: &[Instruction], start_pos: (i32, i32)) -> (i32, i32) {
    let mut x = start_pos.0;
    let mut y = start_pos.1;
    let mut waypoint_x = 10;
    let mut waypoint_y = 1;

    for instruction in instructions {
        match instruction.action {
            'N' => waypoint_y += instruction.value,
            'S' => waypoint_y -= instruction.value,
            'E' => waypoint_x += instruction.value,
            'W' => waypoint_x -= instruction.value,
            'L' => {
                let num_rotations = instruction.value / 90;
                for _ in 0..num_rotations {
                    (waypoint_x, waypoint_y) = (-waypoint_y, waypoint_x)
                }
            },
            'R' => {
                let num_rotations = instruction.value / 90;
                for _ in 0..num_rotations {
                    (waypoint_x, waypoint_y) = (waypoint_y, -waypoint_x)
                }
            },
            'F' => {
                x += waypoint_x * instruction.value;
                y += waypoint_y * instruction.value;
            },
            _ => panic!("Unknown action")
        }
    }

    (x, y)
}

fn manhattan_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn parse_instructions(lines: &Vec<String>) -> Vec<Instruction> {
    lines.iter().map(|line| {
        Instruction {
            action: line.chars().next().unwrap(),
            value: line.chars().skip(1).collect::<String>().parse().unwrap()
        }
    }).collect()
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

struct Instruction {
    action: char,
    value: i32
}
