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

pub fn part_one(_input: &str) -> Option<u32> {
    let mut score: u32 = 0;
    if let Ok(lines) = read_lines("src/input") {
        for line in lines.into_iter().flatten() {
            score += compute_points(line.chars().next().unwrap(), line.chars().nth(2).unwrap());
        }
    }
    Some(score)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let mut score: u32 = 0;
    if let Ok(lines) = read_lines("src/input") {
        for line in lines.into_iter().flatten() {
            score += compute_points_2(line.chars().next().unwrap(), line.chars().nth(2).unwrap());
        }
    }
    Some(score)
}

// PART 1

fn compute_points(opponent: char, me: char) -> u32 {
    match opponent {
        'A' => handle_rock(me),
        'B' => handle_paper(me),
        _ => handle_scisor(me),
    }
}

fn handle_rock(me: char) -> u32 {
    match me {
        'X' => 4,
        'Y' => 8,
        _ => 3,
    }
}

fn handle_paper(me: char) -> u32 {
    match me {
        'X' => 1,
        'Y' => 5,
        _ => 9,
    }
}

fn handle_scisor(me: char) -> u32 {
    match me {
        'X' => 7,
        'Y' => 2,
        _ => 6,
    }
}

// PART 2

fn compute_points_2(opponent: char, me: char) -> u32 {
    match opponent {
        'A' => handle_rock_2(me),
        'B' => handle_paper_2(me),
        _ => handle_scisor_2(me),
    }
}

fn handle_rock_2(me: char) -> u32 {
    match me {
        'X' => 3,
        'Y' => 4,
        _ => 8,
    }
}

fn handle_paper_2(me: char) -> u32 {
    match me {
        'X' => 1,
        'Y' => 5,
        _ => 9,
    }
}

fn handle_scisor_2(me: char) -> u32 {
    match me {
        'X' => 2,
        'Y' => 6,
        _ => 7,
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
