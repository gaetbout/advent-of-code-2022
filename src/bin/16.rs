use regex::Regex;
#[derive(Debug, Clone)]
struct Valve {
    name: String,
    rate: u8,
    leads_to: Vec<String>,
}

fn print_valves(valves: &Vec<Valve>) {
    for v in valves {
        println!("{:?}", v);
    }
    println!();
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut time = 30;
    let all_valves: Vec<Valve> = input.split('\n').map(get_valves).collect();
    let mut current_position = find_valve_named("AA".to_string(), &all_valves);
    let rate_valves = get_all_valves_with_rate(&all_valves);
    let mut closed_valves: Vec<Valve> = rate_valves.clone();
    println!("{:?}", current_position);
    println!();
    print_valves(&all_valves);
    print_valves(&rate_valves);
    while !closed_valves.is_empty() && time != 0 {
        println!("{}", time);
        time -= 1;
    }
    None
}

fn find_valve_named(name: String, valves: &[Valve]) -> &Valve {
    valves.iter().find(|e| e.name.eq(&name)).unwrap()
}

fn get_next_valve(
    current_position: Valve,
    all_valves: &[Valve],
    rate_valves: &[Valve],
    limit: u32,
) -> Option<u32> {
    None
}
fn get_valves(input: &str) -> Valve {
    let seperator = Regex::new(r"[A-Z][A-Z]|\d+").unwrap();
    let mut it = seperator.find_iter(input);
    let name: String = it.next().unwrap().as_str().to_string();
    let rate = it.next().unwrap().as_str().parse::<u8>().unwrap();
    let leads_to = it.map(|e| e.as_str().to_string()).collect();
    Valve {
        name,
        rate,
        leads_to,
    }
}

fn get_all_valves_with_rate(valves: &[Valve]) -> Vec<Valve> {
    valves.iter().filter(|e| e.rate > 0).cloned().collect()
}
pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}
