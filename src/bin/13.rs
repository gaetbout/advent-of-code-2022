use std::cmp::Ordering;
use std::str::Chars;

pub fn part_one(input: &str) -> Option<u32> {
    let mut it = input.split('\n');
    let mut idx = 1;
    let mut score = 0u32;
    while let Some(first_line) = it.next() {
        let sec_line = it.next().unwrap();

        let first_vec = create_number_depth_array(&mut first_line.chars());
        let sec_vec = create_number_depth_array(&mut sec_line.chars());

        it.next();
        // println!("{:?}", first_vec);
        // println!("{:?}", sec_vec);
        if is_valid(first_vec, sec_vec) {
            score += idx;
        }
        // println!();
        idx += 1;
    }
    Some(score)
    // 6656
}
#[derive(Debug)]
struct NumberDepth {
    c: char,
    depth: u32,
}

fn create_number_depth_array(line: &mut Chars<'_>) -> Vec<NumberDepth> {
    let mut vec = vec![];
    let mut current_depth = 1;
    for c in line {
        match c {
            '[' => current_depth += 1,
            ']' => {
                vec.push(NumberDepth {
                    c,
                    depth: current_depth,
                });
                current_depth -= 1
            }
            ',' => {}
            _ => vec.push(NumberDepth {
                c,
                depth: current_depth,
            }),
        }
    }
    vec
}

fn is_valid(v1: Vec<NumberDepth>, v2: Vec<NumberDepth>) -> bool {
    for (i, n1) in v1.iter().enumerate() {
        if v2.len() == i {
            return false;
        }
        let n2 = &v2[i];

        if n2.c.eq(&']') && n1.c.is_alphanumeric() {
            return false;
        }
        if n1.c.eq(&']') {
            match n1.depth.cmp(&n2.depth) {
                Ordering::Less => return true,
                Ordering::Equal => return true,
                Ordering::Greater => return false,
            }
        }
        match n1.depth.cmp(&n2.depth) {
            Ordering::Less => match n1.c.cmp(&n2.c) {
                Ordering::Less => return true,
                Ordering::Equal => return false,
                Ordering::Greater => return false,
            },
            Ordering::Greater => match n1.c.cmp(&n2.c) {
                Ordering::Less => return true,
                Ordering::Equal => continue,
                Ordering::Greater => return false,
            },
            Ordering::Equal => match n1.c.cmp(&n2.c) {
                // OK
                Ordering::Less => return true,
                Ordering::Equal => continue,
                Ordering::Greater => return false,
            },
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}
