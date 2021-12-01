use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

pub fn count_increases(smoothed: bool) -> i32 {
    let filepath = Path::new("./day_1/input.txt");

    if let Ok(lines) = read_lines(filepath) {
        if smoothed {
            return count_increases_smooth(lines);
        } else {
            return count_increases_rough(lines);
        }
    }
    return -1;
}

fn count_increases_smooth(lines: Vec<i32>) -> i32 {
    let mut increases: i32 = 0;
    let mut last_depth: i32 = 0;
    let mut second_last_depth: i32 = 0;
    let mut third_last_depth: i32 = 0;

    for depth in lines {
        let current_sum = depth + last_depth + second_last_depth;
        let previous_sum = last_depth + second_last_depth + third_last_depth;

        if third_last_depth != 0 && current_sum > previous_sum {
            increases += 1
        }

        third_last_depth = second_last_depth;
        second_last_depth = last_depth;
        last_depth = depth;
    }
    return increases;
}

fn count_increases_rough(lines: Vec<i32>) -> i32 {
    let mut previous_depth: Option<i32> = None;
    let mut increases: i32 = 0;
    for new_depth in lines {
        match previous_depth {
            Some(previous_depth_unwrapped) => {
                if previous_depth_unwrapped < new_depth {
                    increases += 1;
                }
            }
            None => {}
        } 
        previous_depth = Some(new_depth)
    }
    return increases;
}

fn read_lines<P>(filename: P) -> io::Result<Vec<i32>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().filter_map(|depth| depth.ok().unwrap().parse::<i32>().ok()).collect())
}

