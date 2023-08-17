use std::fs::read_to_string;

fn part_1(lines: &String) -> i32 {
    let mut value: i32 = 0;
    for line in lines.lines() {
            let splittet_line: Vec<&str> = line.split(',').collect();
            let left_splittet_line: Vec<&str> = splittet_line[0].split('-').collect();
            let right_splittet_line: Vec<&str> = splittet_line[1].split('-').collect();
            let left_range = left_splittet_line[0].parse::<i32>().unwrap()..=left_splittet_line[1].parse::<i32>().unwrap();
            let right_range = right_splittet_line[0].parse::<i32>().unwrap()..=right_splittet_line[1].parse::<i32>().unwrap();
            
            if (left_range.contains(&right_range.start()) && left_range.contains(&right_range.end())) ||
               (right_range.contains(&left_range.start()) && right_range.contains(&left_range.end())) {
                value += 1;
            }
        }
    value
}

fn main() {
    let lines: String = read_to_string("src/day4.txt").unwrap();
    let value: i32 = part_1(&lines);


    println!("{}", value);
}

