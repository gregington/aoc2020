use std::fs::File;
use std::io;
use std::env;
use std::io::BufRead;
use std::path::Path;

fn part1(lines: &Vec<String>) {
    let instructions = parse_instructions(lines);
    let (x, y) = navigate(&instructions, (0, 0), 90);
    let sum = x.abs() + y.abs();
    println!("{sum}");
}

fn part2(lines: &Vec<String>) {
    println!("Part 2");
}

fn navigate(instructions: &[Instruction], start_pos: (i32, i32), start_dir: i32) -> (i32, i32) {
    let mut x;
    let mut y;
    let mut direction = start_dir;
    (x, y) = start_pos;


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
