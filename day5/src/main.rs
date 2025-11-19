use std::fs::read_to_string;

fn parse_crates(lines: &str) -> Vec<Vec<&str>> {
    let brackets_trim = |c| c == '[' || c == ']';
    let split_lines: Vec<&str> = lines.split("\n\n").collect();
    let mut crate_levels: Vec<Vec<&str>> = split_lines[0]
        .lines()
        .map(|line| {
            line.as_bytes()
                .chunks(4)
                .map(|chunk| {
                    std::str::from_utf8(chunk)
                        .unwrap()
                        .trim()
                        .trim_matches(brackets_trim)
                })
                .collect()
        })
        .collect();

    crate_levels.pop();
    crate_levels.reverse();
    transpose(crate_levels)
}

fn parse_moves(lines: &str) -> Vec<Vec<usize>> {
    let split_lines: Vec<&str> = lines.split("\n\n").collect();
    split_lines[1]
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|e| e.parse::<usize>().ok())
                .collect()
        })
        .collect()
}

fn transpose(mut matrix: Vec<Vec<&str>>) -> Vec<Vec<&str>> {
    if matrix.is_empty() {
        return Vec::new();
    }
    let num_cols = matrix[0].len();
    let mut result: Vec<Vec<&str>> = (0..num_cols).map(|_| Vec::new()).collect();

    for row in &mut matrix {
        for (col, val) in row.drain(..).enumerate() {
            if !val.trim().is_empty() {
                result[col].push(val);
            }
        }
    }
    result
}

fn part_1(lines: &str) -> String {
    let mut crates: Vec<Vec<&str>> = parse_crates(lines);
    let moves: Vec<Vec<usize>> = parse_moves(lines);

    for mov in moves {
        for _ in 0..mov[0] {
            let val = crates[mov[1] - 1].pop().unwrap();
            crates[mov[2] - 1].push(val);
        }
    }

    let mut top_string = "".to_string();
    for stack in crates {
        top_string += stack[stack.len() - 1];
    }

    top_string
}

fn part_2(lines: &str) -> String {
    let mut crates: Vec<Vec<&str>> = parse_crates(lines);
    let moves: Vec<Vec<usize>> = parse_moves(lines);

    for mov in moves {
        let idx = mov[1] - 1;
        let dest_idx = mov[2] - 1;
        let len = crates[idx].len();

        let to_move: Vec<&str> = crates[idx][(len - mov[0])..].to_vec();

        crates[idx].truncate(len - mov[0]);

        crates[dest_idx].extend(to_move);
    }

    let mut top_string = "".to_string();
    for stack in crates {
        top_string += stack[stack.len() - 1];
    }

    top_string
}

fn main() {
    let lines: String = read_to_string("src/day5.txt").unwrap();
    let part1 = part_1(&lines);
    let part2 = part_2(&lines);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
