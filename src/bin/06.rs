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
        stack_valid(&win, size)
    });
    Some(idx - 1)
}

fn stack_valid(stack: &[u8], size: usize) -> bool {
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
