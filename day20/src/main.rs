use std::collections::{HashMap, HashSet};
use std::env;
use std::path::Path;
use std::io::BufRead;
use std::io;
use std::fs::File;

use regex::Regex;

fn part1(tiles: &HashMap<u32, [[char; 10]; 10]>) {
    let tile_edges = find_tile_edges(tiles);
    let corner_tiles = find_corner_tiles(&tile_edges);

    let product: u64 = corner_tiles.iter().map(|x| *x as u64).product();
    println!("{product}");
}

fn part2(tiles: &HashMap<u32, [[char; 10]; 10]>) {
    println!("Part 2");
}

fn find_tile_edges(tiles: &HashMap<u32, [[char; 10]; 10]>) -> HashMap<[char; 10], HashSet<u32>> {
    let mut results: HashMap<[char; 10], HashSet<u32>> = HashMap::new();

    for (tile_id, image) in tiles.iter() {
        let tile_edges = get_image_edges(&image);


        for edge in tile_edges.iter() {
            let tile_set_option = results.get(edge);
            let mut tile_set: HashSet<u32>;
            if tile_set_option.is_some() { 
                tile_set = tile_set_option.unwrap().to_owned();
            } else { 
                tile_set = HashSet::new()
            };
    
            tile_set.insert(*tile_id);

            results.insert(*edge, tile_set);
        }
    }

    results
}

fn find_corner_tiles(tile_edges: &HashMap<[char; 10], HashSet<u32>>) -> Vec<u32> {
    let edges: HashMap<[char; 10], u32> = tile_edges.iter()
        .filter(|kv| kv.1.len() == 1)
        .map(|kv| (kv.0.to_owned(), *kv.1.iter().next().unwrap()))
        .collect();

    let edge_tiles: Vec<u32> = edges.iter().map(|kv| *kv.1).collect();

    let mut tile_edge_counts: HashMap<u32, u32> = HashMap::new();
    for edge_tile in edge_tiles.iter() {
        if (tile_edge_counts.contains_key(edge_tile)) {
            tile_edge_counts.insert(*edge_tile, tile_edge_counts[edge_tile] + 1);
        } else {
            tile_edge_counts.insert(*edge_tile, 1);
        }
    }

    tile_edge_counts.iter()
        .filter(|kv| *kv.1 == 4)
        .map(|kv| kv.0.to_owned())
        .collect()
}

fn get_image_edges(image: &[[char; 10]; 10]) -> [[char; 10]; 8] {
    let mut results: [[char; 10]; 8] = [[' '; 10]; 8];

    for i in 0..10 {
        // top
        results[0][i] = image[0][i];
        results[4][i] = image[0][i];

        // bottom
        results[1][i] = image[9][i];
        results[5][i] = image[9][i];

        // left
        results[2][i] = image[i][0];
        results[6][i] = image[i][0];

        // right
        results[3][i] = image[i][9];
        results[7][i] = image[i][9];
    }

    results[4].reverse();
    results[5].reverse();
    results[6].reverse();
    results[7].reverse();

    results
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

    let tiles = parse_tiles(&lines);

    match part {
        1 => part1(&tiles),
        2 => part2(&tiles),
        _ => println!("Invalid part number"),
    }
}

fn parse_tiles(lines: &Vec<String>) -> HashMap<u32, [[char; 10]; 10]> {
    let mut results = HashMap::new();

    let tile_id_regex = Regex::new(r"^Tile (?<tile_id>\d+):$").unwrap();
    let mut tile_id = 0;
    let mut tile_lines: Vec<String> = Vec::new();

    for line in lines.iter() {
        if line.is_empty() && !tile_lines.is_empty() {
            if tile_lines.len() != 10 {
                panic!("Unexpected number of lines");
            }
            let image_array: [[char; 10]; 10] = tile_lines.iter()
                .map(|line| line.chars().collect::<Vec<char>>().try_into().unwrap())
                .collect::<Vec<[char; 10]>>().try_into().unwrap();

            results.insert(tile_id, image_array);
            tile_lines.clear();

            continue;
        }

        let captures = tile_id_regex.captures(line);
        if captures.is_some() {
            tile_id = captures.unwrap()["tile_id"].parse::<u32>().unwrap();
            continue;
        }

        tile_lines.push(line.to_owned());
    }

    if !tile_lines.is_empty() {
        if tile_lines.len() != 10 {
            panic!("Unexpected number of lines");
        }
        let image_array: [[char; 10]; 10] = tile_lines.iter()
            .map(|line| line.chars().collect::<Vec<char>>().try_into().unwrap())
            .collect::<Vec<[char; 10]>>().try_into().unwrap();

        results.insert(tile_id, image_array);
        tile_lines.clear();
    }

    results
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}