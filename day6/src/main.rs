use std::fs::read_to_string;

fn part_1(lines: &str) -> usize {
    let mut temp = String::from("");
    for (i, c) in lines.chars().enumerate() {
        if let Some(pos) = temp.find(c) {
            temp.drain(..=pos);
        }
        temp.push(c);
        if temp.len() == 4 {
            return i + 1;
        }
    }
    0
}

fn part_2(lines: &str) -> usize {
    let mut temp = String::from("");
    for (i, c) in lines.chars().enumerate() {
        if let Some(pos) = temp.find(c) {
            temp.drain(..=pos);
        }
        temp.push(c);
        if temp.len() == 14 {
            return i + 1;
        }
    }
    0
}

fn main() {
    let lines: String = read_to_string("src/test.txt").unwrap();
    let part1 = part_1(&lines);
    let part2 = part_2(&lines);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
