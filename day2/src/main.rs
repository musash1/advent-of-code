use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn create_vector_with_moves(lines: Vec<String>) -> Vec<Vec<char>> {
    let mut vec: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let moves: Vec<char> = line.chars().collect();
        vec.push(moves);
    }

    vec
}

fn check_game_result(game: &Vec<char>) -> i16 {
    match game[0] {
        'A' => match game[2] {
            'X' => return 0,
            'Y' => return 1,
            'Z' => return 2,
            _ => return -1,
        },
        'B' => match game[2] {
            'X' => return 2,
            'Y' => return 0,
            'Z' => return 1,
            _ => return -1,
        },
        'C' => match game[2] {
            'X' => return 1,
            'Y' => return 2,
            'Z' => return 0,
            _ => return -1,
        },
        _ => return -1,
    }
} 

fn check_game_points(game_moves: Vec<Vec<char>>) -> i32 {
    let mut points: i32 = 0;
    for game in game_moves {
        match check_game_result(&game) {
            0 => match game[2] {
                'X' => points += 4,
                'Y' => points += 5,
                'Z' => points += 6,
                _ => return -1,
            },
            1 => match game[2] {
                'X' => points += 7,
                'Y' => points += 8,
                'Z' => points += 9,
                _ => return -1,
            },
            2 => match game[2] {
                'X' => points += 1,
                'Y' => points += 2,
                'Z' => points += 3,
                _ => return -1,
            },
            _ => return -1,
        }
    }

    points
}

fn main() {
    let lines: Vec<String> = lines_from_file("src/day2.txt").expect("Could not load lines.");
    let game_moves: Vec<Vec<char>> = create_vector_with_moves(lines);
    let points: i32 = check_game_points(game_moves);

    println!("{:?}", points);
}