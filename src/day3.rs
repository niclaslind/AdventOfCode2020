use std::io::{BufReader, BufRead};
use std::fs::File;
use std::fs;
use itertools::cons_tuples;

fn file_to_vec() -> Vec<String> {
    let file_in = fs::File::open("src/day3/input.txt".to_string()).expect("Could not found file");
    let file_reader = BufReader::new(file_in);
    let lines: Vec<String> = file_reader.lines().into_iter().map(|l| {
        l.unwrap()
    }).collect();
    lines
}

fn part1() {
    let map = file_to_vec();

    let hits = ride(&map, 3, 1);
    println!("Part1 - {}", hits);
}

fn part2() {
    let map = file_to_vec();

    let hits = (ride(&map, 3, 1) *
        ride(&map, 1, 1) *
        ride(&map, 5, 1) *
        ride(&map, 7, 1) *
        ride(&map, 1, 2));
    println!("Part2 - {}", hits);
}

fn ride(map: &Vec<String>, right_move: usize, down_mode: usize) -> u64 {
    let mut current_position = 0;
    let mut trees = 0;

    for (pos, line) in map.iter().enumerate() {
        if pos % down_mode == 0 {
            let index = current_position % line.chars().count();
            if line.chars().nth(index).unwrap() == '#' {
                trees += 1;
            }
            current_position += right_move;
        }
    }

    trees
}

pub fn main() {
    part1();
    part2();
}