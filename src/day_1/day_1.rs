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

fn count_increases_smooth(lines: io::Lines<io::BufReader<File>>) -> i32 {
    let mut previous_sum: i32 = 0;
    let mut current_sum: i32 = 0;
    let mut increases: i32 = 0;
    let mut last_depth: i32 = 0;
    let mut second_last_depth: i32 = 0;
    let mut third_last_depth: i32 = 0;


    for (index, line) in lines.enumerate() {
        if let Ok(depth) = line {
            let new_depth = depth.parse::<i32>().ok();
            match new_depth {
                Some(new_depth_unwrapped) => {
                    if index > 3 {
                        current_sum = new_depth_unwrapped + last_depth + second_last_depth;
                        previous_sum = last_depth + second_last_depth + third_last_depth;
                    }
                    if previous_sum != 0 && current_sum > previous_sum {
                        increases += 1
                    }
                    third_last_depth = second_last_depth;
                    second_last_depth = last_depth;
                    last_depth = new_depth_unwrapped;
                }
                None => {
                    break
                }
            }
        }
    }
    return increases;
}

fn count_increases_rough(lines: io::Lines<io::BufReader<File>>) -> i32 {
    let mut previous_depth: Option<i32> = None;
    let mut increases: i32 = 0;
    for line in lines {
        if let Ok(depth) = line {
            match previous_depth {
                Some(previous_depth_unwrapped) => {
                    // There is a value, compare to last.
                    let new_depth = depth.parse::<i32>().ok();
                    match new_depth {
                        Some(new_depth_unwrapped) => {
                            if previous_depth_unwrapped < new_depth_unwrapped {
                                increases += 1;
                            }
                            previous_depth = new_depth
                        }
                        None => {
                            break
                        }
                    }
                }
                None => {
                    // There's not a value in previous_depth yet, assign it to this depth.
                    previous_depth = depth.parse::<i32>().ok();
                }
            } 
        }
    }
    return increases;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

