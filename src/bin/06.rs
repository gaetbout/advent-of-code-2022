use std::collections::HashSet;
use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<u32> {
    do_logic(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    do_logic(input, 14)
}

fn do_logic(input: &str, size: usize) -> Option<u32> {
    let mut input_as_iter = input.chars();
    let mut stack = VecDeque::new();
    for _ in 1..(size + 1) {
        stack.push_back(input_as_iter.next().unwrap());
    }
    let mut idx = size;
    for c in input_as_iter {
        if stack_valid(&stack, size) {
            return Some(idx as u32);
        }
        idx += 1;
        stack.pop_front();
        stack.push_back(c);
    }
    None
}

fn stack_valid(stack: &VecDeque<char>, size: usize) -> bool {
    let mut set = HashSet::new();
    stack.into_iter().for_each(|c| {
        set.insert(c);
    });
    set.len() == size
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("inputs", 6);
        assert_eq!(part_one(&input), Some(1356));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(2564));
    }
}
