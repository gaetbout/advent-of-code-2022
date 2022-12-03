use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let Ok(lines) = read_lines("src/input") else { return };
    let mut score: u32 = 0;
    for res_line in lines {
        match res_line {
            Ok(line) => {
                score += get_score_equivalent(get_most_used_letter_for(line));
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
    return '0';
}

fn get_score_equivalent(c: char) -> u32 {
    let char_as_u32 = c as u32;
    if char_as_u32 > 96 {
        return char_as_u32 - 96;
    }
    return char_as_u32 - 38;
}
