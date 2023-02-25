use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn check_for_same_letter(lines: &Vec<String>) -> Vec<&String> {
    let mut return_value: Vec<&String> = vec![];
    for line in lines {
        let (arr1, arr2) = lines.split_at(line.len() / 2);
        for i in 0..arr1.len() {
            for j in 0..arr2.len() {
                if arr1[i] == arr2[j] {
                    return_value.push(&arr1[i]);
                }
            }
        }
    }

    return_value
} 

fn get_value(letters: &Vec<&String>) -> i32 {
    let mut value = 0;
    for letter in letters {
        value = value + letter.parse::<i32>().unwrap() - 64; 
    }
    value
}

fn main() {
    let lines: Vec<String> = lines_from_file("src/day3.txt").expect("Could not load lines");
    let letters: Vec<&String> = check_for_same_letter(&lines);
    let value: i32 = get_value(&letters);

    println!("{}", value);
}


