use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::vec::Vec;

fn part1(lines: &Vec<String>) {
    let boarding_passes = lines.into_iter().map(|line| create_boarding_pass_from_line(line));
    let max_id = boarding_passes.map(|x| x.id).max().unwrap();
    println!("{max_id}");
}

fn part2(lines: &Vec<String>) {
    let boarding_passes = &lines.into_iter().map(|line| create_boarding_pass_from_line(line)).collect::<Vec<_>>();
    let min_row = boarding_passes.into_iter().map(|bp| bp.row).min().unwrap();
    let max_row = boarding_passes.into_iter().map(|bp| bp.row).max().unwrap();

    let rows = min_row+1..max_row;
    let columns = 0 as u8..8;

    let mut boarding_passes_by_id = rows.cartesian_product(columns)
        .map(|x| create_boarding_pass(x.0, x.1))
        .map(|bp| (bp.id, bp))
        .collect::<HashMap<u16, BoardingPass>>();

    boarding_passes.into_iter().for_each(|bp| {
        boarding_passes_by_id.remove(&bp.id);
    });

    let seat_id = boarding_passes_by_id.iter().next().unwrap().0;
    println!("{seat_id}");
}

fn create_boarding_pass_from_line(line: &str) -> BoardingPass {
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

    create_boarding_pass(row, column)
}

fn create_boarding_pass(row: u8, column: u8) -> BoardingPass {
    let id: u16 = (row as u16 * 8) + column as u16;

    BoardingPass {
        row, 
        column,
        id
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

struct BoardingPass {
    row: u8,
    column: u8,
    id: u16
}