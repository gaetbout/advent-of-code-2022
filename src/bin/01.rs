use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// TODO This file is broken as it was done before using the template
fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }
}
pub fn part_one(_input: &str) -> Option<u32> {
    let mut maxes: [u32; 3] = [0; 3];
    let mut local_max = 0;
    if let Ok(lines) = read_lines("src/input") {
        for line in lines.into_iter().flatten() {
            if line.is_empty() {
                update_max(&mut maxes, local_max);
                local_max = 0;
            } else {
                local_max += line.parse::<u32>().unwrap();
            }
        }
    }
    let max: u32 = maxes.iter().sum();
    Some(max)
}

fn update_max(arr: &mut [u32], local_max: u32) {
    for max in arr {
        if local_max > *max {
            *max = local_max;
            return;
        }
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Error while reading file");
    Ok(io::BufReader::new(file).lines())
}
