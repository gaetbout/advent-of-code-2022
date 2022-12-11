use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let seperator = Regex::new(r"[ \n]").unwrap();
    let mut cycle = 1;
    let mut current_value = 1;
    let mut total = 0;
    seperator.split(input).into_iter().for_each(|instruction| {
        cycle += 1;
        current_value += get_register(instruction);
        if is_cycle(cycle) {
            total += cycle * (current_value as u32);
        }
    });
    Some(total)
}

fn get_register(_instruction: &str) -> i32 {
    _instruction.parse::<i32>().unwrap_or(0)
}

fn is_cycle(cycle: u32) -> bool {
    matches!(cycle, 20 | 60 | 100 | 140 | 180 | 220)
}

pub fn part_two(input: &str) -> Option<String> {
    let seperator = Regex::new(r"[ \n]").unwrap();
    let mut cycle = 0;
    let mut sprite_position = 0;
    let mut response = String::new();
    seperator.split(input).into_iter().for_each(|instruction| {
        if (sprite_position..sprite_position + 3).contains(&cycle) {
            response.push('#');
        } else {
            response.push('.');
        }
        cycle += 1;
        sprite_position += get_register(instruction);
        if cycle > 39 {
            response.push('\n');
            cycle = 0;
        }
    });
    Some(response)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(
            part_two(&input),
            Some(String::from(
                "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
            ))
        );
    }
}
