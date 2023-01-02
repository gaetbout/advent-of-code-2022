use std::cmp::Ordering;
use std::slice::Iter;

use regex::Regex;

#[derive(Debug, Clone)]
enum NumOrVect {
    Vect(Vec<NumOrVect>),
    Number(u32),
}

pub fn part_one(input: &str) -> Option<u32> {
    let binding = input.split('\n').into_iter().collect::<Vec<_>>();
    let it = binding.chunks(3);
    let mut score = 0;
    let mut idx = 1;
    // can prob do it using some one liner
    for lines in it {
        if handle_lines(lines) {
            score += idx;
        }
        idx += 1;
    }
    Some(score) // 6656
}
fn handle_lines(lines: &[&str]) -> bool {
    let v1 = create_vector(lines[0]);
    let v2 = create_vector(lines[1]);
    println!("{:?}", v1);
    println!("{:?}", v2);
    println!();
    // is_in_right_order(v1, v2)
    is_in_right_order(&mut v1.iter(), &mut v2.iter())
}
fn create_vector(line: &str) -> Vec<NumOrVect> {
    let chars = Regex::new(r"\[|\]|\d+")
        .unwrap()
        .find_iter(line)
        .map(|n| n.as_str())
        .collect::<Vec<&str>>();
    create_vector_loop(&mut chars.iter(), &mut vec![])
}
fn create_vector_loop(chars: &mut Iter<&str>, papa_vec: &mut Vec<NumOrVect>) -> Vec<NumOrVect> {
    let each = match chars.next() {
        Some(e) => e,
        None => return papa_vec.to_vec(),
    };
    match *each {
        "[" => {
            let v = create_vector_loop(chars, &mut vec![]);
            papa_vec.push(NumOrVect::Vect(v));
            create_vector_loop(chars, papa_vec)
        }
        "]" => papa_vec.to_vec(),
        _ => {
            let n = each.parse::<u32>().unwrap();
            papa_vec.push(NumOrVect::Number(n));
            create_vector_loop(chars, papa_vec)
        }
    }
}

// fn is_in_right_order(v1: Vec<NumOrVect>, v2: Vec<NumOrVect>) -> bool {
//   let mut ln1 = 0;
//   let mut ln2 = 0;

//   while ln1 < v1.len() && ln2 < v2.len() {
//       return false;
//   }
//   ln1 == v1.len() - 1
// }

fn is_in_right_order(v1: &mut Iter<NumOrVect>, v2: &mut Iter<NumOrVect>) -> bool {
    let a = match v1.next() {
        Some(a) => a,
        None => return true,
    };
    let b = match v2.next() {
        Some(b) => b,
        None => return false,
    };
    match a {
        NumOrVect::Vect(v) => match b {
            NumOrVect::Vect(v) => is_in_right_order(v1, v2),
            NumOrVect::Number(n2) => false, // TODO
        },
        NumOrVect::Number(n1) => match b {
            NumOrVect::Vect(v) => false, // TODO
            NumOrVect::Number(n2) => match n1.cmp(n2) {
                Ordering::Less => true,
                Ordering::Equal => is_in_right_order(v1, v2),
                Ordering::Greater => false,
            },
        },
    }
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
        assert_eq!(part_two(&input), Some(140));
    }
}
