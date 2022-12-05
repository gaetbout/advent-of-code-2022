use std::collections::VecDeque;
fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut arr = setup_stack();

    input
        .split('\n')
        .into_iter()
        .for_each(|f| handle_line(f, &mut arr));
    arr.into_iter()
        .for_each(|mut stack| print!("{}", stack.pop_back().unwrap()));
    println!();
    None
}

fn handle_line(line: &str, arrs: &mut [VecDeque<char>; 9]) {
    let mut line_splitted = line.split(' ');
    let amount = line_splitted.nth(1).unwrap().parse::<u32>().unwrap();
    let from = line_splitted.nth(1).unwrap().parse::<u32>().unwrap() - 1;
    let to = line_splitted.nth(1).unwrap().parse::<u32>().unwrap() - 1;
    for _ in 1..(amount + 1) {
        let char_from = arrs.get_mut(from as usize).unwrap().pop_back().unwrap();
        arrs.get_mut(to as usize).unwrap().push_back(char_from);
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut arr = setup_stack();

    input
        .split('\n')
        .into_iter()
        .for_each(|f| handle_line_2(f, &mut arr));
    arr.into_iter()
        .for_each(|mut stack| print!("{}", stack.pop_back().unwrap()));
    println!();
    None
}

fn handle_line_2(line: &str, arrs: &mut [VecDeque<char>; 9]) {
    let mut line_splitted = line.split(' ');
    let amount = line_splitted.nth(1).unwrap().parse::<u32>().unwrap();
    let from = line_splitted.nth(1).unwrap().parse::<u32>().unwrap() - 1;
    let to = line_splitted.nth(1).unwrap().parse::<u32>().unwrap() - 1;

    let mut tmp = VecDeque::new();
    let arr_from = arrs.get_mut(from as usize).unwrap();
    for _ in 1..(amount + 1) {
        tmp.push_front(arr_from.pop_back().unwrap());
    }
    let arr_to = arrs.get_mut(to as usize).unwrap();
    for _ in 1..(amount + 1) {
        arr_to.push_back(tmp.pop_front().unwrap());
    }
}

fn setup_stack() -> [VecDeque<char>; 9] {
    let mut vec1 = VecDeque::new();
    let mut vec2 = VecDeque::new();
    let mut vec3 = VecDeque::new();
    let mut vec4 = VecDeque::new();
    let mut vec5 = VecDeque::new();
    let mut vec6 = VecDeque::new();
    let mut vec7 = VecDeque::new();
    let mut vec8 = VecDeque::new();
    let mut vec9 = VecDeque::new();

    // 1
    vec1.push_back('W');
    vec1.push_back('B');
    vec1.push_back('D');
    vec1.push_back('N');
    vec1.push_back('C');
    vec1.push_back('F');
    vec1.push_back('J');

    // 2
    vec2.push_back('P');
    vec2.push_back('Z');
    vec2.push_back('V');
    vec2.push_back('Q');
    vec2.push_back('L');
    vec2.push_back('S');
    vec2.push_back('T');

    // 3
    vec3.push_back('P');
    vec3.push_back('Z');
    vec3.push_back('B');
    vec3.push_back('G');
    vec3.push_back('J');
    vec3.push_back('T');

    // 4
    vec4.push_back('D');
    vec4.push_back('T');
    vec4.push_back('L');
    vec4.push_back('J');
    vec4.push_back('Z');
    vec4.push_back('B');
    vec4.push_back('H');
    vec4.push_back('C');

    // 5
    vec5.push_back('G');
    vec5.push_back('V');
    vec5.push_back('B');
    vec5.push_back('J');
    vec5.push_back('S');

    // 6
    vec6.push_back('P');
    vec6.push_back('S');
    vec6.push_back('Q');

    // 7
    vec7.push_back('B');
    vec7.push_back('V');
    vec7.push_back('D');
    vec7.push_back('F');
    vec7.push_back('L');
    vec7.push_back('M');
    vec7.push_back('P');
    vec7.push_back('N');

    // 8
    vec8.push_back('P');
    vec8.push_back('S');
    vec8.push_back('M');
    vec8.push_back('F');
    vec8.push_back('B');
    vec8.push_back('D');
    vec8.push_back('L');
    vec8.push_back('R');

    // 9
    vec9.push_back('V');
    vec9.push_back('D');
    vec9.push_back('T');
    vec9.push_back('R');

    [vec1, vec2, vec3, vec4, vec5, vec6, vec7, vec8, vec9]
}
