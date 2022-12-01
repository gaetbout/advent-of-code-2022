use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut maxes: [i32; 3] = [0; 3];
    let mut local_max = 0;
    if let Ok(lines) = read_lines("src/input") {
        for res_line in lines {
            if let Ok(line) = res_line {
                if line == "" {
                    update_max(&mut maxes, local_max);
                    local_max = 0;
                } else {
                    local_max += line.parse::<i32>().unwrap();
                }
            }
        }
    }
    let max: i32 = maxes.iter().sum();
    println!("{max}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Error while reading file");
    Ok(io::BufReader::new(file).lines())
}

fn update_max(arr: &mut [i32], local_max: i32) {
    for max in arr {
        if local_max > *max {
            *max = local_max;
            return;
        }
    }
}

// Useful while dev
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
