use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// TODO This file is broken as it was done before using the template
fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut score: u32 = 0;
    if let Ok(lines) = read_lines("src/input") {
        for res_line in lines {
            if let Ok(line) = res_line {
                score += compute_points(line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap());
            }
        }
    }
    return Some(score);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score: u32 = 0;
    if let Ok(lines) = read_lines("src/input") {
        for res_line in lines {
            if let Ok(line) = res_line {
                score +=
                    compute_points_2(line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap());
            }
        }
    }
    return Some(score);
}

// PART 1
fn part1() {}

fn compute_points(opponent: char, me: char) -> u32 {
    match opponent {
        'A' => return handle_rock(me),
        'B' => return handle_paper(me),
        _ => return handle_scisor(me),
    }
}

fn handle_rock(me: char) -> u32 {
    match me {
        'X' => return 4,
        'Y' => return 8,
        _ => return 3,
    }
}

fn handle_paper(me: char) -> u32 {
    match me {
        'X' => return 1,
        'Y' => return 5,
        _ => return 9,
    }
}

fn handle_scisor(me: char) -> u32 {
    match me {
        'X' => return 7,
        'Y' => return 2,
        _ => return 6,
    }
}

// PART 2

fn compute_points_2(opponent: char, me: char) -> u32 {
    match opponent {
        'A' => return handle_rock_2(me),
        'B' => return handle_paper_2(me),
        _ => return handle_scisor_2(me),
    }
}

fn handle_rock_2(me: char) -> u32 {
    match me {
        'X' => return 3,
        'Y' => return 4,
        _ => return 8,
    }
}

fn handle_paper_2(me: char) -> u32 {
    match me {
        'X' => return 1,
        'Y' => return 5,
        _ => return 9,
    }
}

fn handle_scisor_2(me: char) -> u32 {
    match me {
        'X' => return 2,
        'Y' => return 6,
        _ => return 7,
    }
}

// UTILS
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Error while reading file");
    Ok(io::BufReader::new(file).lines())
}
