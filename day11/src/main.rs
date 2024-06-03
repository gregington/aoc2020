use std::cmp::max;
use std::cmp::min;
use std::fs::File;
use std::io;
use std::env;
use std::io::BufRead;
use std::path::Path;

fn part1(grid: &Vec<Vec<char>>) {
    let mut current_grid = grid.clone();
    let mut prev_grid: Vec<Vec<char>>;

    loop {
        prev_grid = current_grid.clone();
        current_grid = step_grid(&prev_grid);

        if grids_equal(&prev_grid, &current_grid) {
            break;
        }
    }

    let num_occupied: i32 = current_grid.iter().map(|row| row.iter().filter(|x| **x == '#').count() as i32).sum();
    println!("{num_occupied}");
}

fn part2(grid: &Vec<Vec<char>>) {
    println!("Part 2");
}

fn step_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid: Vec<Vec<char>> = grid.clone();
    let num_cols = new_grid[0].len() as i32;
    let num_rows = new_grid.len() as i32;

    for row in 0..num_rows {
        for col in 0..num_cols {
            let neighbors = get_neighbors(&grid, row, col);

            let seat = grid[row as usize][col as usize];
            let new_seat: char;

            if seat == 'L' {
                if neighbors.iter().all(|x| *x != '#') {
                    new_seat = '#';
                } else {
                    new_seat = 'L'
                }
            } else if seat == '#' {
                if neighbors.iter().filter(|x| **x == '#').count() >= 4 {
                    new_seat = 'L';
                } else {
                    new_seat = '#';
                }
            } else {
                new_seat = seat;
            }

            new_grid[row as usize][col as usize] = new_seat;
        }
    }

    new_grid
}

fn get_neighbors(grid: &Vec<Vec<char>>, row: i32, col: i32) -> Vec<char> {
    let mut neighbors: Vec<char> = Vec::with_capacity(8);

    let num_cols = grid[0].len() as i32;
    let num_rows = grid.len() as i32;
    let min_col = max(0, col - 1);
    let max_col = min(num_cols - 1, col + 1);
    let min_row = max(0, row - 1);
    let max_row = min(num_rows - 1, row + 1);

    for r in min_row..=max_row {
        for c in min_col..=max_col {
            if col == c && row == r {
                continue;
            }
            neighbors.push(grid[r as usize][c as usize]);
        }
    }

    neighbors
}

fn grids_equal(grid1: &Vec<Vec<char>>, grid2: &Vec<Vec<char>>) -> bool {
    let num_cols = grid1[0].len();
    let num_rows = grid1.len();

    for row in 0..num_rows {
        for col in 0..num_cols {
            if grid1[row][col] != grid2[row][col] {
                return false;
            }
        }
    }

    true
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

    let grid = create_grid(lines);

    match part {
        1 => part1(&grid),
        2 => part2(&grid),
        _ => println!("Invalid part number"),
    }
}

fn create_grid(lines: Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
