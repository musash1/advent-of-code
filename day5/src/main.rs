use std::fs::read_to_string;
use regex::Regex;

fn parse_crates(lines: &String) -> &str {
    let mut first_vec: Vec<&str> = Vec::new();
    for line in lines.lines() {
         
    }
    "hello"
}

fn part_1(lines: &String) -> &str {
    let crates: &str = parse_crates(lines);
    "Hello"
}

fn main() {
    let lines: String = read_to_string("src/day5.txt").unwrap();
    let part1: &str = part_1(&lines);

    println!("{}", part1);
}

