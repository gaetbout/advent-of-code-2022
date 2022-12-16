use std::u128;

use regex::Regex;

const NUMBER_REGEX: &str = r"\d+";

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u128>,
    operation: Operation,
    division_number: u128,
    true_monkey: u128,
    false_monkey: u128,
    items_inspected: u128,
}

impl Monkey {
    fn do_operation(&self, item: u128) -> u128 {
        match self.operation {
            Operation::Mul(op) => match op {
                Some(n) => n * item,
                None => item * item,
            },
            Operation::Sum(op) => match op {
                Some(n) => n + item,
                None => item + item,
            },
        }
    }
}
#[derive(Debug, Clone)]
enum Operation {
    Mul(Option<u128>),
    Sum(Option<u128>),
}

pub fn part_one(input: &str) -> Option<u128> {
    let binding = input.split('\n').into_iter().collect::<Vec<_>>();
    let it = binding.chunks(7);
    let mut monkeys: Vec<Monkey> = vec![];
    for chunks in it {
        monkeys.push(create_monkey(chunks));
    }

    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            play_all_items(idx, &mut monkeys);
        }
    }

    let mut scores = monkeys
        .into_iter()
        .map(|m| m.items_inspected)
        .collect::<Vec<u128>>();
    scores.sort();
    Some(scores[scores.len() - 1] * scores[scores.len() - 2])
}

fn create_monkey(chunks: &[&str]) -> Monkey {
    let items = extract_numbers_from(chunks[1]);
    let operation_numbers: Vec<u128> = extract_numbers_from(chunks[2]);
    let operation_number = if operation_numbers.is_empty() {
        None
    } else {
        Some(operation_numbers[0])
    };
    let operation = if chunks[2].contains('*') {
        Operation::Mul(operation_number)
    } else {
        Operation::Sum(operation_number)
    };
    let division_number = extract_numbers_from(chunks[3])[0];
    let true_monkey = extract_numbers_from(chunks[4])[0];
    let false_monkey = extract_numbers_from(chunks[5])[0];
    Monkey {
        items,
        operation,
        division_number,
        true_monkey,
        false_monkey,
        items_inspected: 0,
    }
}

fn play_all_items(monkey_idx: usize, monkeys: &mut [Monkey]) {
    let mut monkey = monkeys[monkey_idx].clone();
    monkey.items_inspected += monkey.items.len() as u128;
    for item in monkey.items.iter() {
        let operation_new = monkey.do_operation(*item);
        let operation_bored = operation_new / 3;
        let monkey_to_push = if operation_bored % monkey.division_number == 0 {
            &mut monkeys[monkey.true_monkey as usize]
        } else {
            &mut monkeys[monkey.false_monkey as usize]
        };
        monkey_to_push.items.push(operation_bored);
    }
    monkey.items = vec![];
    monkeys[monkey_idx] = monkey;
}

pub fn part_two(input: &str) -> Option<u128> {
    let binding = input.split('\n').into_iter().collect::<Vec<_>>();
    let it = binding.chunks(7);
    let mut monkeys: Vec<Monkey> = vec![];

    let mut modulo = 1;
    for chunks in it {
        let m = create_monkey(chunks);
        modulo *= m.division_number;
        monkeys.push(m);
    }

    for _ in 0..10000 {
        for idx in 0..monkeys.len() {
            play_all_items_2(idx, &mut monkeys, modulo);
        }
    }

    let mut scores = monkeys
        .into_iter()
        .map(|m| m.items_inspected)
        .collect::<Vec<u128>>();
    scores.sort();
    Some(scores[scores.len() - 1] * scores[scores.len() - 2])
}

fn play_all_items_2(monkey_idx: usize, monkeys: &mut [Monkey], modulo: u128) {
    let mut monkey = monkeys[monkey_idx].clone();
    monkey.items_inspected += monkey.items.len() as u128;
    for item in monkey.items.iter() {
        let operation_new = monkey.do_operation(*item);
        let operation_bored = operation_new % modulo; // / 3;

        let monkey_to_push = if operation_bored % monkey.division_number == 0 {
            &mut monkeys[monkey.true_monkey as usize]
        } else {
            &mut monkeys[monkey.false_monkey as usize]
        };
        monkey_to_push.items.push(operation_bored);
    }
    monkey.items = vec![];
    monkeys[monkey_idx] = monkey;
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}

fn extract_numbers_from(line: &str) -> Vec<u128> {
    Regex::new(NUMBER_REGEX)
        .unwrap()
        .find_iter(line)
        .map(|n| n.as_str().parse::<u128>().unwrap())
        .collect::<Vec<u128>>()
}
