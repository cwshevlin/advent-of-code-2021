use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

pub fn count_increases() {
    let filepath = Path::new("./day_1/input.txt");
    let mut previous_depth: Option<i32> = None;
    let mut increases: i32 = 0;
    let mut decreases: i32 = 0;

    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(depth) = line {
                match previous_depth {
                    Some(previous_depth_unwrapped) => {
                        // There is a value, compare to last
                        let new_depth = depth.parse::<i32>().ok();
                        match new_depth {
                            Some(new_depth_unwrapped) => {
                                if previous_depth_unwrapped < new_depth_unwrapped {
                                    increases += 1;
                                } else {
                                    decreases += 1;
                                }
                                previous_depth = new_depth
                            }
                            None => {
                                break
                            }
                        }
                    }
                    None => {
                        previous_depth = depth.parse::<i32>().ok();
                    }
                } 
            }
        }
    }

    println!("Increases: {}", increases);
    println!("Decreases: {}", decreases);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

