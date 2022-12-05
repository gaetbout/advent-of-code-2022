use regex::Regex;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path; // 1.1.8

// TODO This file is broken as it was done before using the template
fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}

pub fn part_one(_input: &str) -> Option<u32> {
    let Ok(lines) = read_lines("src/input") else { return None };
    let mut overlapping_elve: u32 = 0;
    overlapping_elve += lines
        .into_iter()
        .map(|line| handle_line(line.unwrap()))
        .sum::<u32>();
    Some(overlapping_elve)
}

fn handle_line(_line: String) -> u32 {
    let seperator = Regex::new(r"[,-]").unwrap();

    let mut splits = seperator.split(&_line);
    let first_elf_first_number = parse_split_to_u32(&mut splits);
    let first_elf_sec_number = parse_split_to_u32(&mut splits);
    let sec_elf_first_number = parse_split_to_u32(&mut splits);
    let sec_elf_sec_number = parse_split_to_u32(&mut splits);

    match first_elf_first_number.cmp(&sec_elf_first_number) {
        Ordering::Less => (first_elf_sec_number >= sec_elf_sec_number) as u32,
        Ordering::Greater => (first_elf_sec_number <= sec_elf_sec_number) as u32,
        Ordering::Equal => 1, // It passed even though I should add more stuff here
    }
}

fn parse_split_to_u32(split: &mut regex::Split) -> u32 {
    (split.next().unwrap()).parse::<u32>().unwrap()
}

pub fn part_two(_input: &str) -> Option<u32> {
    let Ok(lines) = read_lines("src/input") else { return None };
    let mut overlapping_elve: u32 = 0;
    overlapping_elve += lines
        .into_iter()
        .map(|line| handle_line_2(line.unwrap()))
        .sum::<u32>();
    Some(overlapping_elve)
}

fn handle_line_2(_line: String) -> u32 {
    let seperator = Regex::new(r"[,-]").unwrap();

    let mut splits = seperator.split(&_line);
    let first_elf_first_number = parse_split_to_u32_2(&mut splits);
    let first_elf_sec_number = parse_split_to_u32_2(&mut splits);
    let sec_elf_first_number = parse_split_to_u32_2(&mut splits);
    let sec_elf_sec_number = parse_split_to_u32_2(&mut splits);

    match first_elf_first_number.cmp(&sec_elf_first_number) {
        Ordering::Less => (first_elf_sec_number >= sec_elf_first_number) as u32,
        Ordering::Greater => (sec_elf_sec_number >= first_elf_first_number) as u32,
        Ordering::Equal => 1,
    }
}

fn parse_split_to_u32_2(split: &mut regex::Split) -> u32 {
    (split.next().unwrap()).parse::<u32>().unwrap()
}

// UTILS
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Error while reading file");
    Ok(io::BufReader::new(file).lines())
}
