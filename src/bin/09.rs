use std::collections::HashSet;

#[derive(PartialEq, Hash, Eq, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn move_to(&mut self, direction: &str) {
        match direction {
            "D" => self.move_down(),
            "U" => self.move_up(),
            "L" => self.move_left(),
            "R" => self.move_right(),
            _ => panic!("Unknown direction"),
        };
    }
    fn move_up(&mut self) {
        self.y += 1;
    }
    fn move_down(&mut self) {
        self.y -= 1;
    }
    fn move_left(&mut self) {
        self.x -= 1;
    }
    fn move_right(&mut self) {
        self.x += 1;
    }

    fn distance(&self, other_point: Point) -> u32 {
        (self.x - other_point.x)
            .abs()
            .max((self.y - other_point.y).abs()) as u32
    }
    fn update_x(&mut self, other_point: &Point) {
        if self.x > other_point.x {
            self.move_left()
        } else {
            self.move_right()
        }
    }

    fn update_y(&mut self, other_point: &Point) {
        if self.y > other_point.y {
            self.move_down()
        } else {
            self.move_up()
        }
    }
    fn update_position(&mut self, other_point: &Point) {
        if self.distance(*other_point) < 2 {
            return;
        }
        if (self.x - other_point.x).abs() >= 2 {
            self.update_x(other_point);
            if self.y != other_point.y {
                self.update_y(other_point);
            }
        } else if (self.y - other_point.y).abs() >= 2 {
            self.update_y(other_point);
            if self.x != other_point.x {
                self.update_x(other_point);
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut points: HashSet<Point> = HashSet::new();
    let mut head_point = Point { x: 0, y: 0 };
    let mut tail_point = Point { x: 0, y: 0 };
    input.split('\n').for_each(|chars| {
        let mut splitted_str = chars.split_whitespace();
        let direction = splitted_str.next().unwrap();
        let moves: u32 = splitted_str.next().unwrap().parse().unwrap();
        for _ in 0..moves {
            head_point.move_to(direction);
            tail_point.update_position(&head_point);
            points.insert(tail_point);
        }
    });
    Some(points.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut points: HashSet<Point> = HashSet::new();
    let mut head_point = Point { x: 0, y: 0 };
    let mut tail_points = vec![];
    for _ in 0..9 {
        tail_points.push(Point { x: 0, y: 0 });
    }
    input.split('\n').for_each(|chars| {
        let mut splitted_str = chars.split_whitespace();
        let direction = splitted_str.next().unwrap();
        let moves: u32 = splitted_str.next().unwrap().parse().unwrap();
        for _ in 0..moves {
            head_point.move_to(direction);
            let mut prev_point = head_point;
            for tail_point in tail_points.iter_mut() {
                tail_point.update_position(&prev_point);
                prev_point = *tail_point;
            }
            points.insert(prev_point);
        }
    });
    // 2434
    Some(points.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
