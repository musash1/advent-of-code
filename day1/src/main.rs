use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn create_vector_with_calories(lines: Vec<String>) -> Vec<i32> {
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

    vec
}

fn search_sum_of_three_biggest_elves(calories: &mut Vec<i32>) -> i32 {
    let max: i32 = *calories.iter().max().unwrap();
    let mut sum: i32 = 0;
    let mut i = 0;
    while i < 3 {
        sum += *calories.iter().max().unwrap();
        let index: usize = calories.iter().position(|x| *x == *calories.iter().max().unwrap()).unwrap();
        calories.remove(index);
        i += 1;
    } 

    sum
}

fn main() {
    let lines = lines_from_file("src/day1.txt").expect("Could not load lines");
    let mut vec: Vec<i32> = create_vector_with_calories(lines);
    let sum: i32 = search_sum_of_three_biggest_elves(&mut vec);


    println!("{}", sum);
}