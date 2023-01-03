use std::{cmp, collections::HashSet};

use regex::Regex;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Point {
    row: u32,
    col: u32,
}

impl Point {
    fn new(row: u32, col: u32) -> Self {
        Point { row, col }
    }

    fn get_pt_up(&self, height: u32) -> Point {
        let new_row = if self.row == 0 { height } else { self.row - 1 };
        Point {
            row: new_row,
            col: self.col,
        }
    }

    fn get_pt_down(&self, height: u32) -> Point {
        let new_row = if self.row == height { 1 } else { self.row + 1 };
        Point {
            row: new_row,
            col: self.col,
        }
    }

    fn get_pt_left(&self, width: u32) -> Point {
        let new_col = if self.col == 0 { width } else { self.col - 1 };
        Point {
            row: self.row,
            col: new_col,
        }
    }

    fn get_pt_right(&self, width: u32) -> Point {
        let new_col = if self.col == width { 1 } else { self.col + 1 };
        Point {
            row: self.row,
            col: new_col,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Orientation {
    Right = 0,
    Down = 1,
    Left = 2,
    Top = 3,
}

impl Orientation {
    fn get_orientation(n: u32) -> Orientation {
        match n {
            0 => Orientation::Right,
            1 => Orientation::Down,
            2 => Orientation::Left,
            3 => Orientation::Top,
            _ => panic!("Invalid orientation"),
        }
    }
    fn turn(&self, c: &str) -> Self {
        if c.eq("R") {
            Orientation::get_orientation((*self as u32 + 1) % 4)
        } else {
            Orientation::get_orientation((*self as u32 + 3) % 4)
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut it = input.split("\n\n");
    let (map_points, wall_points, width, height) = parse_map(it.next().unwrap());

    let (final_pt, orientation) = walk(it.next().unwrap(), map_points, wall_points, width, height);

    Some((1000 * final_pt.row) + (4 * final_pt.col) + orientation as u32)
}

// Will return the map of points, then the map of walls, then the width andthen the height
fn parse_map(map: &str) -> (HashSet<Point>, HashSet<Point>, u32, u32) {
    let mut map_points = HashSet::new();
    let mut wall_points = HashSet::new();
    let mut width = 0_u32;
    let it = map.split('\n');
    let mut height = 0_u32;
    for (row, line) in it.enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '.' {
                map_points.insert(Point::new(row as u32 + 1, col as u32 + 1));
            } else if c == '#' {
                wall_points.insert(Point::new(row as u32 + 1, col as u32 + 1));
            }
        }
        width = cmp::max(width, line.len() as u32);
        height = row as u32 + 1
    }
    (map_points, wall_points, width, height)
}

fn get_most_top_left_point(map_points: &HashSet<Point>, width: u32) -> Point {
    for i in 1..width {
        if let Some(pt) = map_points.get(&Point::new(1, i)) {
            return *pt;
        }
    }
    panic!("This should happen")
}

fn walk(
    path: &str,
    map_points: HashSet<Point>,
    wall_points: HashSet<Point>,
    width: u32,
    height: u32,
) -> (Point, Orientation) {
    let mut current_pt = get_most_top_left_point(&map_points, width);
    let mut orientation: Orientation = Orientation::Right;
    let seperator = Regex::new(r"\d+|R|L").unwrap();
    let it = seperator.find_iter(path).map(|tmp| tmp.as_str());
    for each in it {
        // println!("{:?}", current_pt);
        let number = each.parse::<u32>();
        match number {
            Ok(steps) => {
                current_pt = do_walk(
                    current_pt,
                    orientation,
                    steps,
                    &map_points,
                    &wall_points,
                    width,
                    height,
                );
                // println!("Ending on: {:?}", current_pt);
                // println!();
            }
            Err(_) => orientation = orientation.turn(each),
        }
    }
    (current_pt, orientation)
}

fn do_walk(
    starting_point: Point,
    orientation: Orientation,
    steps: u32,
    map_points: &HashSet<Point>,
    wall_points: &HashSet<Point>,
    width: u32,
    height: u32,
) -> Point {
    // println!(
    //     "do_walk: {:?}:{:?} {:?}",
    //     orientation, steps, starting_point
    // );
    let mut pt = starting_point;
    let mut current_pt = starting_point;
    for _ in 0..steps {
        // println!("STEP");
        loop {
            // println!("current_pt: {:?}", current_pt);
            current_pt = match orientation {
                Orientation::Right => current_pt.get_pt_right(width),
                Orientation::Left => current_pt.get_pt_left(width),
                Orientation::Down => current_pt.get_pt_down(height),
                Orientation::Top => current_pt.get_pt_up(height),
            };
            if wall_points.contains(&current_pt) {
                return pt;
            }
            if map_points.contains(&current_pt) {
                break;
            }
        }
        // println!("OUT");
        pt = current_pt
    }
    pt // 154038 Too low 101070
       // 200038 too high
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut it = input.split("\n\n");
    let (map_points, wall_points, width, height) = parse_map(it.next().unwrap());

    let (final_pt, orientation) = walk(it.next().unwrap(), map_points, wall_points, width, height);

    Some((1000 * final_pt.row) + (4 * final_pt.col) + orientation as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), Some(5031));
    }
}
