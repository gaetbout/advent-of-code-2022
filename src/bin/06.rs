use std::collections::HashSet;
pub fn part_one(input: &str) -> Option<u32> {
    do_logic(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    do_logic(input, 14)
}

fn do_logic(input: &str, size: usize) -> Option<u32> {
    let mut idx: u32 = size as u32;
    input.as_bytes().windows(size).into_iter().any(|win| {
        idx += 1;
        stack_valid(win)
    });
    Some(idx - 1)
}

fn stack_valid(stack: &[u8]) -> bool {
    let mut set = HashSet::new();
    let mut is_valid = true;
    stack.iter().for_each(|c| {
        if set.contains(c) {
            is_valid = false;
            return;
        }
        set.insert(c);
    });
    is_valid
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
