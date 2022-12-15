use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn find_biggest_number(lines: Vec<String>) -> i32 {
    let mut vec = Vec::new();
    let mut temp: i32 = 0;
    for line in lines {
        if line.is_empty() {
            vec.push(temp);
            temp = 0;
            continue;
        }
        temp += line.parse::<i32>().unwrap();
    } 

    *vec.iter().max().unwrap()
}

fn main() {
    let lines = lines_from_file("src/day1.txt").expect("Could not load lines");
    let max: i32 = find_biggest_number(lines);

    println!("{}", max);
}