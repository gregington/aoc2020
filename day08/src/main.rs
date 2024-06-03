use std::{collections::HashSet, env, fs::File, io::{self, BufRead}, path::Path};

fn part1(lines: &Vec<String>) {
    let instructions = parse_instructions(&lines);
    let acc = find_acc_at_loop(&instructions);
    println!("{acc}");
}

fn part2(lines: &Vec<String>) {
    println!("Part 2")
}

fn parse_instructions(lines: &Vec<String>) -> Vec<Instruction> {
    lines.iter().map(|line| parse_instruction(line)).collect::<Vec<Instruction>>()
}

fn find_acc_at_loop(instructions: &Vec<Instruction>) -> i64 {
    let mut acc: i64 = 0;
    let mut ip: i16 = 0;
    let mut executed_instructions: HashSet<i16> = HashSet::new();
    
    loop {
        if executed_instructions.contains(&ip) {
            break;
        }
        let instruction = &instructions[ip as usize];
        
        executed_instructions.insert(ip);

        match instruction.operation {
            Operation::Acc => {
                acc += instruction.argument as i64;
                ip += 1;
            },
            Operation::Jmp => {
                ip += instruction.argument;
            },
            Operation::Nop => {
                ip += 1;
            }
        }
    }

    acc
}

fn parse_instruction(line: &str) -> Instruction {
    let idx = line.find(' ').unwrap();
    let operation = match &line[..idx] {
        "acc" => Operation::Acc,
        "jmp" => Operation::Jmp,
        "nop" => Operation::Nop,
        _ => panic!("Unknown operation")
    };

    let argument = line[idx + 1..].parse::<i16>().unwrap();

    Instruction {
        operation,
        argument
    }
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
    operation: Operation,
    argument: i16
}

enum Operation {
    Acc,
    Jmp,
    Nop
}