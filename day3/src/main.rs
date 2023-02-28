use std::{
    path::Path,
};

use itertools::Itertools;

fn lines_from_file(filename: impl AsRef<Path>) -> String {
    std::fs::read_to_string(filename).unwrap()
}

fn part1(lines: &String) -> u32 {
    lines
        .lines()
        // calculate the priority of the single unique item for each line
        .filter_map(|line| {
            let line = line.as_bytes();
            let (left, right) = line.split_at(line.len() / 2);

            left.iter()
                .find(|item| right.contains(item))
                .map(|item| match item {
                    b'a'..=b'z' => (item - b'a') + 1,
                    _ => (item - b'A') + 1 + 26,
                } as u32)
        })
        .sum()
}

pub fn part2(lines: &String) -> u32 {
    lines
        .lines()
        .map(|line| line.as_bytes())
        .tuples()
        .filter_map(|(sack1, sack2, sack3)| {
           sack1
            .iter()
            .find(|item| sack2.contains(item) && sack3.contains(item))
            .map(|item| match item {
                    b'a'..=b'z' => (item - b'a') + 1,
                    _ => (item - b'A') + 1 + 26,
                } as u32)
        })
            .sum()
} 

fn main() {
    let lines: String = lines_from_file("src/day3.txt");
    let value: u32 = part1(&lines);
    let value2: u32 = part2(&lines);

    println!("{}", value);
    println!("{}", value2);
}

