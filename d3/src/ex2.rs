use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let Ok(lines) = read_lines("src/input") else { return };
    let mut score: u32 = 0;
    let mut round = 0;
    let mut letters = HashSet::new();
    for res_line in lines {
        match res_line {
            Ok(line) => {
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
            _ => (),
        }
    }
    println!("Score is: {score}")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Error while reading file");
    Ok(io::BufReader::new(file).lines())
}

fn get_score_equivalent(c: char) -> u32 {
    let char_as_u32 = c as u32;
    if char_as_u32 > 96 {
        return char_as_u32 - 96;
    }
    return char_as_u32 - 38;
}

fn filter_set(line: String, letters: HashSet<char>) -> HashSet<char> {
    let mut new_letters = HashSet::new();
    line.chars().for_each(|c| {
        if letters.contains(&c) {
            new_letters.insert(c);
        };
    });
    return new_letters;
}

fn find_first(line: String, letters: HashSet<char>) -> char {
    for c in line.chars() {
        if letters.contains(&c) {
            return c;
        }
    }
    return '0';
}
