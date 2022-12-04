use regex::Regex;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path; // 1.1.8

fn main() {
    let Ok(lines) = read_lines("src/input") else { return };
    let mut overlapping_elve: u32 = 0;
    overlapping_elve += lines
        .into_iter()
        .map(|line| handle_line(line.unwrap()))
        .sum::<u32>();
    println!("Score is: {overlapping_elve}")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Error while reading file");
    Ok(io::BufReader::new(file).lines())
}

fn handle_line(line: String) -> u32 {
    let seperator = Regex::new(r"[,-]").unwrap();

    let mut splits = seperator.split(&line);
    let first_elf_first_number = parse_split_to_u32(&mut splits);
    let first_elf_sec_number = parse_split_to_u32(&mut splits);
    let sec_elf_first_number = parse_split_to_u32(&mut splits);
    let sec_elf_sec_number = parse_split_to_u32(&mut splits);

    match first_elf_first_number.cmp(&sec_elf_first_number) {
        Ordering::Less => return (first_elf_sec_number >= sec_elf_first_number) as u32,
        Ordering::Greater => return (sec_elf_sec_number >= first_elf_first_number) as u32,
        Ordering::Equal => return 1,
    }
}

fn parse_split_to_u32(split: &mut regex::Split) -> u32 {
    return (split.next().unwrap()).parse::<u32>().unwrap();
}
