use regex::Regex;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
enum RockOrSand {
    Rock,
    Sand,
}

trait Add {
    fn add_rock(&mut self, x: u32, y: u32);
    fn add_sand(&mut self, x: u32, y: u32);
    fn contains_something(&self, x: u32, y: u32) -> bool;
}

impl Add for HashMap<Point, RockOrSand> {
    fn add_rock(&mut self, x: u32, y: u32) {
        self.insert(Point { x, y }, RockOrSand::Rock);
    }
    fn add_sand(&mut self, x: u32, y: u32) {
        self.insert(Point { x, y }, RockOrSand::Sand);
    }

    fn contains_something(&self, x: u32, y: u32) -> bool {
        self.contains_key(&Point { x, y })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = HashMap::new();

    let mut max_depth = 0;
    for l in input.split('\n') {
        let depth = populate_map_from_line(l, &mut map);
        if depth > max_depth {
            max_depth = depth;
        }
    }
    // Whenever a grain goes below max_depth ==> BINGO
    Some(make_it_rain(&mut map, max_depth))
}

fn populate_map_from_line(line: &str, map: &mut HashMap<Point, RockOrSand>) -> u32 {
    let seperator = Regex::new(r"\d+").unwrap();
    let mut it = seperator.find_iter(line);

    let mut current_x: u32 = it.next().unwrap().as_str().parse().unwrap();
    let mut current_y: u32 = it.next().unwrap().as_str().parse().unwrap();
    let mut max_depth = 0;

    map.add_rock(current_x, current_y);

    while let Some(match_x) = it.next() {
        let dest_x: u32 = match_x.as_str().parse().unwrap();
        let dest_y: u32 = it.next().unwrap().as_str().parse().unwrap();

        let (x, mut curr_x) = get_max_min(current_x, dest_x);
        let (y, mut curr_y) = get_max_min(current_y, dest_y);
        while curr_x < x {
            map.add_rock(curr_x, curr_y);
            curr_x += 1;
        }
        while curr_y < y {
            map.add_rock(curr_x, curr_y);
            curr_y += 1;
        }
        map.add_rock(dest_x, dest_y);
        if curr_y > max_depth {
            max_depth = curr_y;
        }
        current_x = dest_x;
        current_y = dest_y;
    }
    max_depth
}

fn make_it_rain(map: &mut HashMap<Point, RockOrSand>, max_depth: u32) -> u32 {
    let mut sand_amount = 0;
    while loop_fall(map, max_depth, 500, 0) {
        sand_amount += 1;
    }
    sand_amount
}

fn loop_fall(map: &mut HashMap<Point, RockOrSand>, max_depth: u32, x: u32, y: u32) -> bool {
    if y >= max_depth {
        return false;
    }
    if !map.contains_something(x, y + 1) {
        return loop_fall(map, max_depth, x, y + 1);
    }
    if !map.contains_something(x - 1, y + 1) {
        return loop_fall(map, max_depth, x - 1, y + 1);
    }
    if !map.contains_something(x + 1, y + 1) {
        return loop_fall(map, max_depth, x + 1, y + 1);
    }
    map.add_sand(x, y);
    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = HashMap::new();

    let mut max_depth = 0;
    for l in input.split('\n') {
        let depth = populate_map_from_line(l, &mut map);
        if depth > max_depth {
            max_depth = depth;
        }
    }
    // Whenever a grain goes below max_depth ==> BINGO
    Some((make_it_rain_2(&mut map, max_depth + 1)) + 1)
}

fn make_it_rain_2(map: &mut HashMap<Point, RockOrSand>, max_depth: u32) -> u32 {
    let mut sand_amount = 0;
    while loop_fall_2(map, max_depth, 500, 0) {
        sand_amount += 1;
    }
    sand_amount
}

fn loop_fall_2(map: &mut HashMap<Point, RockOrSand>, max_depth: u32, x: u32, y: u32) -> bool {
    if y >= max_depth {
        map.add_sand(x, y);
        return true;
    }
    if x == 500
        && map.contains_something(499, 1)
        && map.contains_something(500, 1)
        && map.contains_something(501, 1)
    {
        return false;
    }
    if !map.contains_something(x, y + 1) {
        return loop_fall_2(map, max_depth, x, y + 1);
    }
    if !map.contains_something(x - 1, y + 1) {
        return loop_fall_2(map, max_depth, x - 1, y + 1);
    }
    if !map.contains_something(x + 1, y + 1) {
        return loop_fall_2(map, max_depth, x + 1, y + 1);
    }
    map.add_sand(x, y);
    true
}

fn get_max_min(x: u32, y: u32) -> (u32, u32) {
    if x > y {
        (x, y)
    } else {
        (y, x)
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
