
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

enum Direction {
    Up,
    Down, 
    Forward
}

pub fn find_position(use_aim: bool) -> (i32, i32) {
    let filepath = Path::new("./day_2/input.txt");
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;

    if let Ok(movements) = read_lines(filepath) {
        for movement in movements {
            let direction_tuple = parse_movement(movement);
            match direction_tuple.0 {
                Direction::Up => if use_aim {aim -= direction_tuple.1} else { vertical -= direction_tuple.1 },
                Direction::Down => if use_aim {aim += direction_tuple.1} else {vertical += direction_tuple.1},
                Direction::Forward => {
                    if use_aim {
                        horizontal += direction_tuple.1;
                        vertical += aim * direction_tuple.1;
                    } else {
                        horizontal += direction_tuple.1
                    }
                }
            }
        }
    }
    return (horizontal, vertical);
}

fn parse_movement(movement: String) -> (Direction, i32) {
    let pair: Vec<&str> = movement.split(' ').collect();
    let distance = parse_number(pair.get(1).unwrap());
    match pair.get(0) {
        Some(&"up") => return (Direction::Up, distance),
        Some(&"down") => return (Direction::Down, distance),
        Some(&"forward") => return (Direction::Forward, distance),
        _ => (Direction::Forward, 0)
    }
}

fn parse_number(number: &str) -> i32 {
    return number.parse::<i32>().ok().unwrap();
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().filter_map(|line| line.ok()).collect())
}