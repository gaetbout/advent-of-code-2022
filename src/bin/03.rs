use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// TODO This file is broken as it was done before using the template
fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}

pub fn part_one(_input: &str) -> Option<u32> {
    let Ok(lines) = read_lines("src/input") else { return None };
    let mut score: u32 = 0;
    for line in lines.into_iter().flatten() {
        score += get_score_equivalent(get_most_used_letter_for(line));
    }
    Some(score)
}
fn get_most_used_letter_for(line: String) -> char {
    let line_len_divided = (line.len() / 2) - 1;

    let mut letters = HashSet::new();
    for (i, c) in line.chars().enumerate() {
        if i > line_len_divided {
            if letters.contains(&c) {
                return c;
            }
        } else {
            letters.insert(c);
        }
    }
    '0'
}

fn get_score_equivalent(c: char) -> u32 {
    let char_as_u32 = c as u32;
    if char_as_u32 > 96 {
        return char_as_u32 - 96;
    }
    char_as_u32 - 38
}

pub fn part_two(_input: &str) -> Option<u32> {
    let Ok(lines) = read_lines("src/input") else { return None };
    let mut score: u32 = 0;
    let mut round = 0;
    let mut letters = HashSet::new();
    for line in lines.into_iter().flatten() {
        if round == 0 {
            line.chars().for_each(|c| {
                letters.insert(c);
            });
        } else if round == 1 {
            letters = filter_set(line, letters);
        } else {
            score += get_score_equivalent(find_first(line, letters));
            letters = HashSet::new();
        }
        round = (round + 1) % 3;
    }
    Some(score)
}

fn filter_set(line: String, letters: HashSet<char>) -> HashSet<char> {
    let mut new_letters = HashSet::new();
    line.chars().for_each(|c| {
        if letters.contains(&c) {
            new_letters.insert(c);
        };
    });
    new_letters
}

fn find_first(line: String, letters: HashSet<char>) -> char {
    for c in line.chars() {
        if letters.contains(&c) {
            return c;
        }
    }
    '0'
}
// UTILS
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Error while reading file");
    Ok(io::BufReader::new(file).lines())
}
