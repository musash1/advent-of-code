use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];

static ASCII_HIGHER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 
    'F', 'G', 'H', 'I', 'J', 
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 
    'U', 'V', 'W', 'X', 'Y', 
    'Z',
];

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn check_for_same_letter(lines: &Vec<String>) -> Vec<char> {
    let mut return_value: Vec<char> = vec![];
    for line in lines {
       let (str1, str2) = line.split_at(line.len() / 2);
        let arr1: Vec<char> = str1.chars().collect();
        let arr2: Vec<char> = str2.chars().collect();
        for i in 0..arr1.len() {
            for j in 0..arr2.len() {
                if arr1[i] == arr2[j] {
                    return_value.push(arr1[i]);
                }
            }
        }
    }

    return_value
} 

fn get_value(letters: &Vec<char>) -> usize {
    let mut value = 0;
    for letter in letters {
        for i in 0..25 {
            if *letter == ASCII_LOWER[i] {
                value += i;
            }
            if *letter == ASCII_HIGHER[i] {
                value += i + 26;
            }
        }
    }
    value
}

fn main() {
    let lines: Vec<String> = lines_from_file("src/day3.txt").expect("Could not load lines");
    let letters: Vec<char> = check_for_same_letter(&lines);
    let value: usize = get_value(&letters);

    println!("{}", value);
}

