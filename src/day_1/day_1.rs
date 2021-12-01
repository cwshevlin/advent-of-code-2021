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
    let mut increases: i32 = 0;
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut c: i32 = 0;
    let mut d: i32 = 0;

    for (index, line) in lines.enumerate() {
        if let Ok(depth) = line {
            let mut sum = 0;
            let new_depth = depth.parse::<i32>().ok();
            match new_depth {
                Some(new_depth_unwrapped) => {
                    if index < 3 {
                        a += new_depth_unwrapped
                    } else if index % 4 == 3 {
                        b += new_depth_unwrapped;
                        c += new_depth_unwrapped;
                        d += new_depth_unwrapped;

                        sum = a;
                        println!("A: {}", sum);
                        a = 0;
                    } else if index % 4 == 0 {
                        a += new_depth_unwrapped;
                        c += new_depth_unwrapped;
                        d += new_depth_unwrapped;

                        sum = b; 
                        println!("B: {}", sum);
                        b = 0;
                    } else if index % 4 == 1 {
                        a += new_depth_unwrapped;
                        b += new_depth_unwrapped;
                        d += new_depth_unwrapped;

                        sum = c; 
                        println!("C: {}", sum);
                        c = 0;
                    } else if index % 4 == 2 {
                        a += new_depth_unwrapped;
                        b += new_depth_unwrapped;
                        c += new_depth_unwrapped;

                        sum = d; 
                        println!("D: {}", sum);
                        d = 0;
                    }
                    if previous_sum != 0 && sum > previous_sum {
                        increases += 1
                    }
                    previous_sum = sum;
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

