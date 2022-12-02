use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // part1()
    part2()
}

// PART 1
// fn part1() {
//     let mut score: i32 = 0;
//     if let Ok(lines) = read_lines("src/input") {
//         for res_line in lines {
//             if let Ok(line) = res_line {
//                 score += compute_points(line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap());
//             }
//         }
//     }
//     println!("Score: {score}")
// }

// fn compute_points(opponent: char, me: char) -> i32 {
//     match opponent {
//         'A' => return handle_rock(me),
//         'B' => return handle_paper(me),
//         _ => return handle_scisor(me),
//     }
// }

// fn handle_rock(me: char) -> i32 {
//     match me {
//         'X' => return 4,
//         'Y' => return 8,
//         _ => return 3,
//     }
// }

// fn handle_paper(me: char) -> i32 {
//     match me {
//         'X' => return 1,
//         'Y' => return 5,
//         _ => return 9,
//     }
// }

// fn handle_scisor(me: char) -> i32 {
//     match me {
//         'X' => return 7,
//         'Y' => return 2,
//         _ => return 6,
//     }
// }

// PART 2

fn part2() {
    let mut score: i32 = 0;
    if let Ok(lines) = read_lines("src/input") {
        for res_line in lines {
            if let Ok(line) = res_line {
                score += compute_points(line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap());
            }
        }
    }
    println!("Score: {score}")
}

fn compute_points(opponent: char, me: char) -> i32 {
    match opponent {
        'A' => return handle_rock(me),
        'B' => return handle_paper(me),
        _ => return handle_scisor(me),
    }
}

fn handle_rock(me: char) -> i32 {
    match me {
        'X' => return 3,
        'Y' => return 4,
        _ => return 8,
    }
}

fn handle_paper(me: char) -> i32 {
    match me {
        'X' => return 1,
        'Y' => return 5,
        _ => return 9,
    }
}

fn handle_scisor(me: char) -> i32 {
    match me {
        'X' => return 2,
        'Y' => return 6,
        _ => return 7,
    }
}

// UTILS
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Error while reading file");
    Ok(io::BufReader::new(file).lines())
}
