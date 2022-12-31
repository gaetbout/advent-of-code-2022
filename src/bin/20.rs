use regex::Regex;

#[derive(Debug, Clone)]
struct Node {
    val: i64,
    visited: bool,
    idx: usize,
}

impl Node {
    fn new(val: i64, idx: usize) -> Node {
        Node {
            val,
            visited: false,
            idx,
        }
    }
}

fn input_to_nodes(input: &str, coeff: i64) -> Vec<Node> {
    let seperator = Regex::new(r"\d+|-\d+").unwrap();
    let numbers: Vec<i64> = seperator
        .find_iter(input)
        .map(|s| s.as_str().parse::<i64>().unwrap())
        .collect();
    let mut nodes: Vec<Node> = vec![];
    for (idx, n) in numbers.into_iter().enumerate() {
        let pt = Node::new(n * coeff, idx);
        nodes.push(pt);
    }
    nodes
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut nodes = input_to_nodes(input, 1);
    let ln = nodes.len();
    let mut solved = 0;
    while solved < ln {
        let next_idx = nodes.iter().position(|n| !n.visited).unwrap();
        let mut node = nodes.remove(next_idx);
        node.visited = true;
        let idx_to_add = find_idx_to_add(node.val, next_idx as i64, ln as i64);
        nodes.insert(idx_to_add, node);
        solved += 1;
    }
    Some(make_sum(nodes)) // 13183
}

fn make_sum(nodes: Vec<Node>) -> u64 {
    let ln = nodes.len();
    let idx_of_zero = nodes.iter().position(|n| n.val == 0).unwrap();
    let first = nodes[(1000 + idx_of_zero) % ln].val;
    let sec = nodes[(2000 + idx_of_zero) % ln].val;
    let third = nodes[(3000 + idx_of_zero) % ln].val;
    (first + sec + third) as u64
}

fn find_idx_to_add(val: i64, idx: i64, array_len: i64) -> usize {
    if val == 0 {
        return idx as usize;
    }
    let resp = idx + (val % (array_len - 1));
    if resp >= array_len {
        (resp - array_len + 1) as usize
    } else if resp <= 0 {
        (resp + array_len - 1) as usize
    } else {
        resp as usize
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut nodes = input_to_nodes(input, 811589153);
    let ori_list = input_to_nodes(input, 811589153);
    let mut nodes_to_copy_from = ori_list.to_vec();
    let ln = nodes.len();
    let mut solved: usize;
    for _ in 0..10 {
        solved = 0;
        while solved < ln {
            let next_idx_to_copy_from = nodes_to_copy_from.iter().position(|n| !n.visited).unwrap();
            let node_to_copy_from = nodes_to_copy_from.remove(next_idx_to_copy_from);

            let next_idx = nodes
                .iter()
                .position(|n| n.idx == node_to_copy_from.idx)
                .unwrap();
            let node = nodes.remove(next_idx);
            let idx_to_add = find_idx_to_add(node.val, next_idx as i64, ln as i64);
            nodes.insert(idx_to_add, node);
            solved += 1;
        }
        nodes_to_copy_from = ori_list.to_vec();
    }
    Some(make_sum(nodes)) // 6676132372578
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
