use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::vec::Vec;

fn part1(lines: &Vec<String>) {
    let boarding_passes = lines.into_iter().map(|line| create_boarding_pass(line));
    let max_id = boarding_passes.map(|x| calculate_seat_id(&x)).max().unwrap();
    println!("{max_id}");
}

fn part2(lines: &Vec<String>) {

}

fn create_boarding_pass(line: &str) -> BoardingPass {
    let row = &line[..7];
    let column = &line[7..];

    let row = row.chars()
        .map(|c| if c == 'F' { 0 as u8 } else { 1 })
        .reduce(|acc, a| (acc << 1) | a)
        .unwrap();

    let column = column.chars()
        .map(|c| if c == 'L' { 0 as u8 } else { 1 })
        .reduce(|acc, a| (acc << 1) | a)
        .unwrap();

    BoardingPass {
        row, 
        column 
    }
}

fn calculate_seat_id(boarding_pass: &BoardingPass) -> u64
{
    (boarding_pass.row as u64 * 8) + boarding_pass.column as u64
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

struct BoardingPass {
    row: u8,
    column: u8
}