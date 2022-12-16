use range_union_find::IntRangeUnionFind;
use regex::Regex;
use std::{collections::HashSet, ops::Range};

// CHANGE THIS TO WORK WITH THE ACTUAL INPUT FOR P1
const AIM: i32 = 20;
// const AIM: i32 = 2000000;

const VALID_RANGE: Range<i64> = 0..21;
// const VALID_RANGE: Range<i64> = 0..4000001;

pub fn part_one(input: &str) -> Option<u32> {
    let mut ranges = IntRangeUnionFind::<i32>::new();
    let mut b_set = HashSet::new();
    input
        .split('\n')
        .for_each(|l| handle_line(l, &mut ranges, &mut b_set));

    let mut score = 0;
    for range in ranges.to_collection::<Vec<_>>() {
        score += range.end() - range.start();
    }
    Some(score as u32)
}

fn handle_line(line: &str, ranges: &mut IntRangeUnionFind<i32>, b_set: &mut HashSet<i32>) {
    let seperator = Regex::new(r"\d+|-\d+").unwrap();
    let mut it = seperator.find_iter(line);
    let sensor_x: i32 = it.next().unwrap().as_str().parse().unwrap();
    let sensor_y: i32 = it.next().unwrap().as_str().parse().unwrap();
    let beacon_x: i32 = it.next().unwrap().as_str().parse().unwrap();
    let beacon_y: i32 = it.next().unwrap().as_str().parse().unwrap();

    let manhattan_distance = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();
    if !is_aim_within_sensor_range(sensor_y, manhattan_distance) {
        return;
    }
    let distance = ((sensor_y - AIM).abs() - manhattan_distance).abs();

    if beacon_y == AIM {
        b_set.insert(beacon_x);
    }
    match ranges.insert_range(&((sensor_x - distance)..=(sensor_x + distance))) {
        Ok(_) => {}
        Err(_) => panic!("Duh"),
    }
}

fn is_aim_within_sensor_range(y: i32, distance: i32) -> bool {
    (y - distance..y + distance).contains(&AIM)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut array_of_ranges = vec![];
    for _ in VALID_RANGE {
        array_of_ranges.push(IntRangeUnionFind::<i64>::new());
    }
    input
        .split('\n')
        .for_each(|l| handle_line_2(l, &mut array_of_ranges));

    for (idx, ranges) in array_of_ranges.iter().enumerate() {
        let r_coll = ranges.to_collection::<Vec<_>>();
        if r_coll.len() > 1 {
            return Some(((r_coll[0].end() + 1) as u64 * 4000000) as u64 + (idx as u64));
        }
    }
    None
}

fn handle_line_2(line: &str, array_of_range: &mut [IntRangeUnionFind<i64>]) {
    let seperator = Regex::new(r"\d+|-\d+").unwrap();
    let mut it = seperator.find_iter(line);
    let sensor_x: i64 = it.next().unwrap().as_str().parse().unwrap();
    let sensor_y: i64 = it.next().unwrap().as_str().parse().unwrap();
    let beacon_x: i64 = it.next().unwrap().as_str().parse().unwrap();
    let beacon_y: i64 = it.next().unwrap().as_str().parse().unwrap();

    let manhattan_distance = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();
    for y in (sensor_y - manhattan_distance)..(sensor_y + manhattan_distance) {
        if VALID_RANGE.contains(&y) {
            let distance = ((sensor_y - y).abs() - manhattan_distance).abs();

            let min = min_or_start_of_range(sensor_x, distance);
            let max = max_or_end_of_range(sensor_x, distance);
            match array_of_range[y as usize].insert_range(&(min..=max)) {
                Ok(_) => {}
                Err(_) => panic!("Duh"),
            }
        }
    }
}

fn min_or_start_of_range(x: i64, distance: i64) -> i64 {
    let sol = x - distance;
    if VALID_RANGE.contains(&(sol)) {
        sol
    } else {
        VALID_RANGE.start
    }
}

fn max_or_end_of_range(x: i64, distance: i64) -> i64 {
    let sol = x + distance;
    if VALID_RANGE.contains(&(sol)) {
        sol
    } else {
        VALID_RANGE.end
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
