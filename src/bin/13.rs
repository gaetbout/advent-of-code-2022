use std::cmp::Ordering;
use std::slice::Iter;

use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
enum NumOrVect {
    Vect(Vec<NumOrVect>),
    Number(u32),
}

enum LoopTrueFalse {
    Loop,
    True,
    False,
}

pub fn part_one(input: &str) -> Option<u32> {
    let binding = input.split('\n').into_iter().collect::<Vec<_>>();
    let it = binding.chunks(3);
    let mut score = 0;
    let mut idx = 1;
    // can prob do it using some one liner
    for lines in it {
        let v1 = create_vector(lines[0]);
        let v2 = create_vector(lines[1]);
        match is_in_right_order(v1.to_vec(), v2.to_vec()) {
            LoopTrueFalse::Loop | LoopTrueFalse::True => score += idx,
            LoopTrueFalse::False => (),
        }
        idx += 1;
    }
    Some(score) // 6656
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

fn is_in_right_order(v1: Vec<NumOrVect>, v2: Vec<NumOrVect>) -> LoopTrueFalse {
    let mut idx1 = 0;
    let mut idx2 = 0;
    while idx1 < v1.len() && idx2 < v2.len() {
        let a = &v1[idx1];
        let b = &v2[idx2];
        match a {
            NumOrVect::Vect(local_v1) => return handle_vector(local_v1, b),
            NumOrVect::Number(n1) => match handle_number(n1, b) {
                LoopTrueFalse::Loop => {}
                LoopTrueFalse::True => return LoopTrueFalse::True,
                LoopTrueFalse::False => return LoopTrueFalse::False,
            },
        };
        idx1 += 1;
        idx2 += 1;
    }
    if idx1 == v1.len() && idx2 == v2.len() {
        LoopTrueFalse::Loop
    } else if idx1 == v1.len() {
        LoopTrueFalse::True
    } else {
        LoopTrueFalse::False
    }
}

fn handle_vector(v1: &Vec<NumOrVect>, n2: &NumOrVect) -> LoopTrueFalse {
    if v1.is_empty() {
        return LoopTrueFalse::True;
    }
    match n2 {
        NumOrVect::Vect(local_v2) => is_in_right_order(v1.to_vec(), local_v2.to_vec()),
        NumOrVect::Number(n2) => is_in_right_order(v1.to_vec(), vec![NumOrVect::Number(*n2)]),
    }
}

fn handle_number(n1: &u32, n2: &NumOrVect) -> LoopTrueFalse {
    match n2 {
        NumOrVect::Vect(local_v2) => {
            if local_v2.is_empty() {
                LoopTrueFalse::False
            } else {
                is_in_right_order(vec![NumOrVect::Number(*n1)], local_v2.to_vec())
            }
        }
        NumOrVect::Number(n3) => match n1.cmp(n3) {
            Ordering::Less => LoopTrueFalse::True,
            Ordering::Equal => LoopTrueFalse::Loop,
            Ordering::Greater => LoopTrueFalse::False,
        },
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let binding = input.split('\n').into_iter().collect::<Vec<_>>();
    let it = binding.chunks(3);
    let mut vectors = vec![];
    for lines in it {
        vectors.push(create_vector(lines[0]));
        vectors.push(create_vector(lines[1]));
    }
    let vec_2 = NumOrVect::Vect(vec![NumOrVect::Vect(vec![NumOrVect::Number(2)])]);
    let vec_6 = NumOrVect::Vect(vec![NumOrVect::Vect(vec![NumOrVect::Number(6)])]);
    vectors.push(vec![vec_2.clone()]);
    vectors.push(vec![vec_6.clone()]);

    vectors.sort_by(|v1, v2| compare_vecs(v1, v2));
    // for v in vectors.iter().skip(1) {
    //     println!("v{:?}", v);
    // }
    let p1 = vectors.iter().position(|v| v[0] == vec_2).unwrap() + 1;
    let p2 = vectors.iter().position(|v| v[0] == vec_6).unwrap() + 1;
    Some((p1 * p2) as u32) // 19716
}

fn compare_vecs(v1: &[NumOrVect], v2: &[NumOrVect]) -> Ordering {
    match is_in_right_order(v1.to_vec(), v2.to_vec()) {
        LoopTrueFalse::Loop => Ordering::Less,
        LoopTrueFalse::True => Ordering::Less,
        LoopTrueFalse::False => Ordering::Greater,
    }
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
