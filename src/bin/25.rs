use std::{cmp, ops::Add};

const EXPONENT: i64 = 5;

pub fn part_one(input: &str) -> Option<String> {
    Some(to_snafu(input.split('\n').map(parse_line).sum::<i64>()))
}

fn parse_line(line: &str) -> i64 {
    let mut res = 0_i64;
    for (idx, c) in line.chars().rev().enumerate() {
        let i = idx as u32;
        match c {
            '1' => res += EXPONENT.pow(i) as i64,
            '2' => res += 2 * (EXPONENT.pow(i)) as i64,
            '0' => {}
            '-' => res -= EXPONENT.pow(i) as i64,
            '=' => res -= 2 * (EXPONENT.pow(i)) as i64,
            _ => panic!("Unhandled operation"),
        }
    }
    res as i64
}

fn to_snafu(input: i64) -> String {
    let (mut power, mut val, mut s) = find_starting_power(input);
    while power != 0 {
        power -= 1;
        let (c, d) = get_next_power(val, power);
        s.push_str(&d);
        val = c;
    }
    s
}

fn get_next_power(n: i64, power: u32) -> (i64, String) {
    println!("n: {}, power: {}", n, power);
    let one: i64 = EXPONENT.pow(power) as i64;
    let two = one * 2;

    let one_res = n + one;
    let two_res = n + two;
    let minus_one_res = n - one;
    let minus_two_res = n - two;
    let one_res_abs = one_res.abs();
    let two_res_abs = two_res.abs();
    let minus_one_res_abs = minus_one_res.abs();
    let minus_two_res_abs = minus_two_res.abs();

    let smallest = cmp::min(
        one_res_abs,
        cmp::min(
            two_res_abs,
            cmp::min(minus_one_res_abs, cmp::min(minus_two_res_abs, n.abs())),
        ),
    );
    if smallest == one_res_abs {
        (one_res, "1".to_string())
    } else if smallest == two_res_abs {
        (two_res, "2".to_string())
    } else if smallest == minus_one_res_abs {
        (minus_one_res, "-".to_string())
    } else if smallest == minus_two_res_abs {
        (minus_two_res, "=".to_string())
    } else {
        (n, "0".to_string())
    }
}
fn find_starting_power(n: i64) -> (u32, i64, String) {
    for i in 0..100 {
        let one: i64 = EXPONENT.pow(i) as i64;
        let twice = one * 2;
        if (one..twice).contains(&n) {
            let first_res = n - one;
            let sec_res = twice - n;
            if first_res < sec_res {
                return (i, first_res, String::from("1"));
            } else {
                return (i, sec_res, String::from("2"));
            }
        }
    }
    panic!("This shouldn't happen")
}

pub fn part_two(input: &str) -> Option<i64> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some(String::from("2=-1=0")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_two(&input), None);
    }
}
