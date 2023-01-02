#[derive(Debug, Clone)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}
const NAME: &str = "humn";

#[derive(Debug, Clone)]
enum Monkey {
    Value(String, i64),
    Redirect(String, String, Operation, String),
}

impl Monkey {
    fn get_name(&self) -> String {
        match self {
            Monkey::Value(n, _) => n.to_string(),
            Monkey::Redirect(n, _, _, _) => n.to_string(),
        }
    }

    fn get_value(&self, monkeys: &[Monkey]) -> i64 {
        match self {
            Monkey::Value(_, n) => *n,
            Monkey::Redirect(_, left, op, right) => {
                let left = Monkey::find_monkey_by_name(left, monkeys)
                    .unwrap()
                    .get_value(monkeys);
                let right = Monkey::find_monkey_by_name(right, monkeys)
                    .unwrap()
                    .get_value(monkeys);
                op.do_operation(left, right)
            }
        }
    }

    fn compute_missing_value(&self, monkeys: &[Monkey], eq: i64) -> Option<i64> {
        match self {
            Monkey::Value(_, _) => Some(eq),
            Monkey::Redirect(_, left, op, right) => {
                let left = Monkey::find_monkey_by_name(left, monkeys).unwrap();
                let right = Monkey::find_monkey_by_name(right, monkeys).unwrap();
                if left.contains_name(monkeys) {
                    let val = right.get_value(monkeys);
                    let res = match op {
                        Operation::Add => eq - val,
                        Operation::Sub => eq + val,
                        Operation::Mul => eq / val,
                        Operation::Div => eq * val,
                    };
                    left.compute_missing_value(monkeys, res)
                } else {
                    let val = left.get_value(monkeys);
                    let res = match op {
                        Operation::Add => eq - val,
                        Operation::Sub => val - eq,
                        Operation::Mul => eq / val,
                        Operation::Div => val / eq,
                    };
                    right.compute_missing_value(monkeys, res)
                }
            }
        }
    }

    fn contains_name(&self, monkeys: &[Monkey]) -> bool {
        match self {
            Monkey::Value(n, _) => n.eq(NAME),
            Monkey::Redirect(_, l, _, r) => {
                let left = Monkey::find_monkey_by_name(l, monkeys).unwrap();
                let right = Monkey::find_monkey_by_name(r, monkeys).unwrap();
                left.contains_name(monkeys) || right.contains_name(monkeys)
            }
        }
    }
    fn find_monkey_by_name(name: &str, monkeys: &[Monkey]) -> Option<Monkey> {
        monkeys.iter().find(|m| m.get_name() == name).cloned()
    }
}

impl Operation {
    fn do_operation(&self, n1: i64, n2: i64) -> i64 {
        match self {
            Operation::Add => n1 + n2,
            Operation::Sub => n1 - n2,
            Operation::Mul => n1 * n2,
            Operation::Div => n1 / n2,
        }
    }

    fn new(input: &str) -> Self {
        match input {
            "+" => Operation::Add,
            "-" => Operation::Sub,
            "*" => Operation::Mul,
            "/" => Operation::Div,
            _ => panic!("Unrecognized operation"),
        }
    }
}
pub fn part_one(input: &str) -> Option<i64> {
    let monkeys = input
        .split('\n')
        .collect::<Vec<_>>()
        .iter()
        .map(parse_line)
        .collect::<Vec<Monkey>>();
    let root = Monkey::find_monkey_by_name("root", &monkeys);
    Some(root?.get_value(&monkeys))
}

fn parse_line(line: &&str) -> Monkey {
    let mut it = line.split(':');
    let name = it.next().unwrap();
    let next = it.next().unwrap().trim();
    let number = next.parse::<i64>();
    match number {
        Ok(ok) => Monkey::Value(String::from(name), ok),
        Err(_) => create_redirect_monkey_named(name, next),
    }
}

fn create_redirect_monkey_named(name: &str, input: &str) -> Monkey {
    let mut it = input.split_ascii_whitespace();
    let name1 = it.next().unwrap();
    let operation = Operation::new(it.next().unwrap());
    let name2 = it.next().unwrap();
    Monkey::Redirect(
        String::from(name),
        String::from(name1),
        operation,
        String::from(name2),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let monkeys = input
        .split('\n')
        .collect::<Vec<_>>()
        .iter()
        .map(parse_line)
        .collect::<Vec<Monkey>>();

    let root = Monkey::find_monkey_by_name("root", &monkeys).unwrap();
    if let Monkey::Redirect(_, left, _, right) = root {
        let left_root = Monkey::find_monkey_by_name(&left, &monkeys).unwrap();
        let right_root = Monkey::find_monkey_by_name(&right, &monkeys).unwrap();
        return Some(compute_result(left_root, right_root, &monkeys));
    }
    None
}
// If both are not static, it'll be way more complex, and we'll have to manually check each value.
fn compute_result(left_monkey: Monkey, right_monkey: Monkey, monkeys: &[Monkey]) -> i64 {
    let static_number = if right_monkey.contains_name(monkeys) {
        left_monkey.get_value(monkeys)
    } else {
        right_monkey.get_value(monkeys)
    };
    left_monkey
        .compute_missing_value(monkeys, static_number)
        .unwrap()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
