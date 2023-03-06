use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> String {
    std::fs::read_to_string(filename).unwrap()
}

fn part_1(lines: &String) -> i32 {
    let mut value: i32 = 0;
    lines
        .lines()
        .map(|line| {
            let splittet_line: Vec<&str> = line.split(',').collect();
            let left_splittet_line: Vec<&str> = splittet_line[0].split('-').collect();
            let right_splittet_line: Vec<&str> = splittet_line[1].split('-').collect();
            let left_vec = vec![left_splittet_line[0].parse::<i32>().unwrap()..left_splittet_line[1].parse::<i32>().unwrap()];
            let right_vec = vec![right_splittet_line[0].parse::<i32>().unwrap()..right_splittet_line[1].parse::<i32>().unwrap()];

            if left_vec.iter().all(|item| right_vec.contains(item)) || right_vec.iter().all(|item| left_vec.contains(item)) {
                value += 1;
            }
        });

    value
}

fn main() {
    let lines: String = lines_from_file("src/day4.txt");
    let value: i32 = part_1(&lines);

    println!("{}", lines);
    println!("{}", value);
}

