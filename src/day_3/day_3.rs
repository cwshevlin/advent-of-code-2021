#[path = "../utils/utils.rs"]
mod utils;
use std::collections::hash_map::HashMap;
use std::path::Path;

pub fn get_power_consumption() -> i32 {
    let filepath = Path::new("./day_3/input.txt");
    let mut frequency_map: HashMap<usize, i32> = HashMap::new();
    let mut gamma_rate = "000000000000".to_string();
    let mut epsilon_rate= "000000000000".to_string();

    if let Ok(binary_numbers) = utils::read_lines_as_strings(filepath) {
        for binary_number in binary_numbers {
            for (index, char) in binary_number.chars().enumerate() {
                match char {
                    '1' => *frequency_map.entry(index).or_insert(0) += 1,
                    '0' => *frequency_map.entry(index).or_insert(0) -= 1,
                    _ => {}
                }
            }
        }
        for (key, value) in frequency_map.iter() {
            if *value > 0 {
                gamma_rate.replace_range(*key..*key+1, "1");
                epsilon_rate.replace_range(*key..*key+1, "0");
            } else {
                gamma_rate.replace_range(*key..*key+1, "0");
                epsilon_rate.replace_range(*key..*key+1, "1");
            }
        }

        let gamma_rate_int = i32::from_str_radix(gamma_rate.as_str(), 2).unwrap();
        let epsilon_rate_int = i32::from_str_radix(epsilon_rate.as_str(), 2).unwrap();

        return gamma_rate_int * epsilon_rate_int;
    }
    return -1;
}