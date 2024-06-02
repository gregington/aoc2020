use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::vec::Vec;

fn part1(map: &Vec<Vec<char>>) {
    let tree_count = count_trees(map, 3, 1);
    println!("{tree_count}");
}

fn part2(map: &Vec<Vec<char>>) {
    let tree_count = count_trees(map, 1, 1)
        * count_trees(map, 3, 1)
        * count_trees(map, 5, 1)
        * count_trees(map, 7, 1)
        * count_trees(map, 1, 2);
        
    println!("{tree_count}");
}

fn count_trees(map: &Vec<Vec<char>>, col_stride: usize, row_stride: usize) -> i64
{
    let mut col = 0;
    let mut row = 0;

    let max_cols = map.get(0).unwrap().len();
    let max_row = map.len();
    let mut tree_count = 0;

    while row < max_row {
        let c = *map.get(row).unwrap().get(col % max_cols).unwrap();

        if c == '#' {
            tree_count += 1;
        }

        col += col_stride;
        row += row_stride;
    }

    tree_count    
}

fn read_map(filename: &str) -> Vec<Vec<char>> {
    let lines = read_lines(filename).unwrap();
    Vec::from_iter(lines.map(|line| Vec::from_iter(line.unwrap().chars())))
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args[1].parse::<i32>().unwrap();

    let mut filename = "input.txt";
    if args.len() > 2 {
        filename = &args[2];
    }

    let map: Vec<Vec<char>> = read_map(filename); 

    match part {
        1 => part1(&map),
        2 => part2(&map),
        _ => println!("Invalid part number"),
    }
}
