pub fn part_one(input: &str) -> Option<u32> {
    let two_d_array = create_2d_array(input);
    let len = two_d_array.len();
    let mut score: u32 = (len as u32 * 4) - 4;
    for i in 1..len - 1 {
        for j in 1..len - 1 {
            if is_visible(&two_d_array, i, j) {
                score += 1;
            }
        }
    }
    Some(score)
}

fn is_visible(array: &[Vec<u8>], i: usize, j: usize) -> bool {
    is_visible_from_left(array, i, j)
        || is_visible_from_right(array, i, j)
        || is_visible_from_top(array, i, j)
        || is_visible_from_bottom(array, i, j)
}
fn is_visible_from_left(array: &[Vec<u8>], i: usize, j: usize) -> bool {
    let val = array[i][j];
    for k in 0..j {
        if val <= array[i][k] {
            return false;
        }
    }
    true
}

fn is_visible_from_right(array: &[Vec<u8>], i: usize, j: usize) -> bool {
    let val = array[i][j];
    for k in (j + 1..array.len()).rev() {
        if val <= array[i][k] {
            return false;
        }
    }
    true
}

fn is_visible_from_top(array: &[Vec<u8>], i: usize, j: usize) -> bool {
    let val = array[i][j];
    for k in 0..i {
        if val <= array[k][j] {
            return false;
        }
    }
    true
}

fn is_visible_from_bottom(array: &[Vec<u8>], i: usize, j: usize) -> bool {
    let val = array[i][j];
    for k in (i + 1..array.len()).rev() {
        if val <= array[k][j] {
            return false;
        }
    }
    true
}

fn create_2d_array(input: &str) -> Vec<Vec<u8>> {
    input
        .split('\n')
        .map(|line| line.chars().map(|current| current as u8 - b'0').collect())
        .collect::<Vec<Vec<u8>>>()
}

pub fn part_two(input: &str) -> Option<u32> {
    let two_d_array = create_2d_array(input);
    let len = two_d_array.len();
    let mut current_max = 0;
    for i in 1..len - 1 {
        for j in 1..len - 1 {
            let current_top = compute_view(&two_d_array, i, j);
            if current_top > current_max {
                current_max = current_top;
            }
        }
    }
    Some(current_max)
}

fn compute_view(array: &[Vec<u8>], i: usize, j: usize) -> u32 {
    view_left(array, i, j)
        * view_right(array, i, j)
        * view_top(array, i, j)
        * view_bottom(array, i, j)
}
fn view_left(array: &[Vec<u8>], i: usize, j: usize) -> u32 {
    let val = array[i][j];
    let mut view_distance = 1;
    for k in (0..j).rev() {
        if val <= array[i][k] {
            return view_distance;
        }
        view_distance += 1;
    }
    view_distance - 1
}

fn view_right(array: &[Vec<u8>], i: usize, j: usize) -> u32 {
    let val = array[i][j];
    let mut view_distance = 1;
    for k in j + 1..array.len() {
        if val <= array[i][k] {
            return view_distance;
        }
        view_distance += 1;
    }
    view_distance - 1
}

fn view_top(array: &[Vec<u8>], i: usize, j: usize) -> u32 {
    let val = array[i][j];
    let mut view_distance = 1;
    for k in (0..i).rev() {
        if val <= array[k][j] {
            return view_distance;
        }
        view_distance += 1;
    }
    view_distance - 1
}

fn view_bottom(array: &[Vec<u8>], i: usize, j: usize) -> u32 {
    let val = array[i][j];
    let mut view_distance = 1;
    for k in i + 1..array.len() {
        if val <= array[k][j] {
            return view_distance;
        }
        view_distance += 1;
    }
    view_distance - 1
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
