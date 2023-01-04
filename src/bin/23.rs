use std::{
    cmp,
    collections::{HashMap, HashSet},
};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    fn new(row: i32, col: i32) -> Self {
        Position { row, col }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut elves = generate_set_of_elves(input);

    for round in 0..10 {
        let moving_elves = generate_set_of_elves_that_will_move(&elves);
        let mut elves_not_moving: HashSet<Position> =
            HashSet::from_iter(elves.iter().copied().filter(|e| !moving_elves.contains(e)));
        let move_considration = consider_moving(&moving_elves, round);

        for (pos, set) in move_considration {
            if set.len() > 1 {
                for e in set {
                    elves_not_moving.insert(e);
                }
            } else {
                elves_not_moving.insert(pos);
            }
        }
        elves = elves_not_moving;
    }
    Some(compute_size(&elves))
}

fn generate_set_of_elves(input: &str) -> HashSet<Position> {
    let mut elves = HashSet::new();
    for (row, line) in input.split('\n').enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert(Position::new(row as i32, col as i32));
            }
        }
    }
    elves
}

fn generate_set_of_elves_that_will_move(elves: &HashSet<Position>) -> HashSet<Position> {
    let mut moving_elves: HashSet<Position> = HashSet::new();
    for elf in elves {
        if can_move_anywhere(elf, elves) {
            moving_elves.insert(*elf);
        }
    }
    moving_elves
}

fn can_move_anywhere(elf: &Position, elves: &HashSet<Position>) -> bool {
    !(can_move_north(elf, elves).is_some()
        && can_move_south(elf, elves).is_some()
        && can_move_east(elf, elves).is_some()
        && can_move_west(elf, elves).is_some())
}
fn consider_moving(elves: &HashSet<Position>, round: u32) -> HashMap<Position, HashSet<Position>> {
    let mut position_elves: HashMap<Position, HashSet<Position>> = HashMap::new();
    for elf in elves {
        match round % 4 {
            0 => check_move(
                elf,
                elves,
                &mut position_elves,
                can_move_north,
                can_move_south,
                can_move_west,
                can_move_east,
            ),
            1 => check_move(
                elf,
                elves,
                &mut position_elves,
                can_move_south,
                can_move_west,
                can_move_east,
                can_move_north,
            ),
            2 => check_move(
                elf,
                elves,
                &mut position_elves,
                can_move_west,
                can_move_east,
                can_move_north,
                can_move_south,
            ),
            3 => check_move(
                elf,
                elves,
                &mut position_elves,
                can_move_east,
                can_move_north,
                can_move_south,
                can_move_west,
            ),
            _ => panic!("Is this even possible"),
        }
    }
    position_elves
}

fn check_move(
    elf: &Position,
    elves: &HashSet<Position>,
    position_elves: &mut HashMap<Position, HashSet<Position>>,
    f1: fn(&Position, &HashSet<Position>) -> Option<Position>,
    f2: fn(&Position, &HashSet<Position>) -> Option<Position>,
    f3: fn(&Position, &HashSet<Position>) -> Option<Position>,
    f4: fn(&Position, &HashSet<Position>) -> Option<Position>,
) {
    if let Some(key) = f1(elf, elves) {
        update(position_elves, key, *elf);
        return;
    }
    if let Some(key) = f2(elf, elves) {
        update(position_elves, key, *elf);
        return;
    }
    if let Some(key) = f3(elf, elves) {
        update(position_elves, key, *elf);
        return;
    }
    if let Some(key) = f4(elf, elves) {
        update(position_elves, key, *elf);
        return;
    }
    update(position_elves, *elf, *elf);
}
fn update(elves: &mut HashMap<Position, HashSet<Position>>, key: Position, elf: Position) {
    elves.entry(key).or_default().insert(elf);
}
fn can_move_north(elf: &Position, elves: &HashSet<Position>) -> Option<Position> {
    let positions = [
        (elf.row - 1, elf.col),
        (elf.row - 1, elf.col - 1),
        (elf.row - 1, elf.col + 1),
    ];
    if contains_any(positions, elves) {
        None
    } else {
        Some(Position::new(elf.row - 1, elf.col))
    }
}

fn can_move_south(elf: &Position, elves: &HashSet<Position>) -> Option<Position> {
    let positions = [
        (elf.row + 1, elf.col),
        (elf.row + 1, elf.col - 1),
        (elf.row + 1, elf.col + 1),
    ];
    if contains_any(positions, elves) {
        None
    } else {
        Some(Position::new(elf.row + 1, elf.col))
    }
}

fn can_move_west(elf: &Position, elves: &HashSet<Position>) -> Option<Position> {
    let positions = [
        (elf.row, elf.col - 1),
        (elf.row - 1, elf.col - 1),
        (elf.row + 1, elf.col - 1),
    ];
    if contains_any(positions, elves) {
        None
    } else {
        Some(Position::new(elf.row, elf.col - 1))
    }
}

fn can_move_east(elf: &Position, elves: &HashSet<Position>) -> Option<Position> {
    let positions = [
        (elf.row, elf.col + 1),
        (elf.row - 1, elf.col + 1),
        (elf.row + 1, elf.col + 1),
    ];
    if contains_any(positions, elves) {
        None
    } else {
        Some(Position::new(elf.row, elf.col + 1))
    }
}

fn contains_any(positions: [(i32, i32); 3], elves: &HashSet<Position>) -> bool {
    positions
        .iter()
        .any(|(row, col)| elves.contains(&Position::new(*row, *col)))
}

fn compute_size(elves: &HashSet<Position>) -> u32 {
    let mut min_row = i32::MAX;
    let mut max_row = i32::MIN;
    let mut min_col = i32::MAX;
    let mut max_col = i32::MIN;
    for e in elves {
        min_row = cmp::min(e.row, min_row);
        max_row = cmp::max(e.row, max_row);
        min_col = cmp::min(e.col, min_col);
        max_col = cmp::max(e.col, max_col);
    }
    (((max_row - min_row) + 1) * ((max_col - min_col) + 1)) as u32 - elves.len() as u32
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut elves = generate_set_of_elves(input);
    let tot_elves = elves.len();
    for round in 0..1000000 {
        let moving_elves = generate_set_of_elves_that_will_move(&elves);
        let mut elves_not_moving: HashSet<Position> =
            HashSet::from_iter(elves.iter().copied().filter(|e| !moving_elves.contains(e)));
        if elves_not_moving.len() == tot_elves {
            return Some(round + 1);
        }
        let move_considration = consider_moving(&moving_elves, round);

        for (pos, set) in move_considration {
            if set.len() > 1 {
                for e in set {
                    elves_not_moving.insert(e);
                }
            } else {
                elves_not_moving.insert(pos);
            }
        }
        elves = elves_not_moving;
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
