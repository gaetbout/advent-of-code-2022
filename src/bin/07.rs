use std::str::Split;

#[derive(Debug, Clone)]
struct Folder {
    size: u32,
    parent_folder: Option<Box<Folder>>,
}

fn new_root_folder() -> Folder {
    new_empty_folder_named(None)
}

fn new_empty_folder_named(parent_folder: Option<Box<Folder>>) -> Folder {
    Folder {
        size: 0,
        parent_folder,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut current_folder = new_root_folder();
    let mut score = 0;
    input.split('\n').skip(1).for_each(|line| {
        let mut it = line.split(' ');
        let first_part = it.next().unwrap();
        if first_part.eq("$") {
            (score, current_folder) = handle_command(&mut it, current_folder.clone(), score);
        } else if first_part.eq("dir") {
            // Do nothing
        } else {
            handle_file(first_part, &mut current_folder);
        }
    });
    let (score, _) = get_root(current_folder, score);
    println!("{:?}", score);
    Some(score)
}

fn get_root(folder: Folder, score: u32) -> (u32, Folder) {
    if folder.parent_folder.is_some() {
        let mut parent_folder = *folder.parent_folder.unwrap();
        parent_folder.size += folder.size;
        return get_root(parent_folder, get_new_score(folder.size, score));
    }
    (score, folder)
}

fn get_new_score(size: u32, score: u32) -> u32 {
    if size < 100000 {
        return size + score;
    }
    score
}
fn handle_command(it: &mut Split<char>, folder: Folder, score: u32) -> (u32, Folder) {
    let command = it.next().unwrap();
    if command.eq("ls") {
        return (score, folder);
    }
    let directory_to_move_to = it.next().unwrap();
    if directory_to_move_to.eq("..") {
        let mut parent_folder = folder.parent_folder.clone().unwrap();
        let new_score = get_new_score(folder.size, score);
        parent_folder.size += folder.size;
        return (new_score, *parent_folder);
    }
    (score, new_empty_folder_named(Some(Box::new(folder))))
}

fn handle_file(filesize: &str, folder: &mut Folder) {
    let size = filesize.parse::<u32>().unwrap();
    folder.size += size;
}
const UPDATE_LIMIT: u32 = 208860; // < 215710

pub fn part_two(input: &str) -> Option<u32> {
    let mut current_folder = new_root_folder();
    let mut score = 3000000000;
    input.split('\n').skip(1).for_each(|line| {
        let mut it = line.split(' ');
        let first_part = it.next().unwrap();
        if first_part.eq("$") {
            let tmp;
            (tmp, current_folder) = handle_command_2(&mut it, current_folder.clone(), score);
            if tmp >= UPDATE_LIMIT && tmp < score {
                score = tmp;
            }
        } else if first_part.eq("dir") {
            // Do nothing
        } else {
            handle_file(first_part, &mut current_folder);
        }
    });
    let (score, current_folder) = get_root_2(current_folder, score);
    println!("{:?}", current_folder);
    println!("{:?}", score);
    Some(score)
}

fn get_root_2(folder: Folder, score: u32) -> (u32, Folder) {
    if folder.parent_folder.is_some() {
        let mut parent_folder = *folder.parent_folder.unwrap();
        parent_folder.size += folder.size;
        if parent_folder.size >= UPDATE_LIMIT && parent_folder.size < score {
            return get_root(parent_folder, folder.size);
        }
        return get_root(parent_folder, score);
    }
    (score, folder)
}

fn handle_command_2(it: &mut Split<char>, folder: Folder, score: u32) -> (u32, Folder) {
    let command = it.next().unwrap();
    if command.eq("ls") {
        return (score, folder);
    }
    let directory_to_move_to = it.next().unwrap();
    if directory_to_move_to.eq("..") {
        let mut parent_folder = folder.parent_folder.clone().unwrap();
        parent_folder.size += folder.size;
        return (folder.size, *parent_folder);
    }
    (score, new_empty_folder_named(Some(Box::new(folder))))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
