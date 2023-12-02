// Day 1 (part b) of the advent of code challenge
use std::env;
mod day1_a;
extern crate fancy_regex;

fn parse_ints_from_string(s: String) -> Vec<i32> {
    // Use regular expressions to parse the ints from the string.
    // The ints could be spelled (i.e. "zero", "one", etc.) or as digits.

    let re_str: &str = r"(?=(one|two|three|four|five|six|seven|eight|nine|zero|\d))";
    let re = fancy_regex::Regex::new(re_str).unwrap();
    let mut ints = Vec::new();
    let captures: Vec<_> = re.captures_iter(&s).collect();

    for c in captures {
        let cap = c.unwrap();
        let cap_str = cap.get(1).unwrap().as_str();
        match cap_str {
            "one" => ints.push(1),
            "two" => ints.push(2),
            "three" => ints.push(3),
            "four" => ints.push(4),
            "five" => ints.push(5),
            "six" => ints.push(6),
            "seven" => ints.push(7),
            "eight" => ints.push(8),
            "nine" => ints.push(9),
            "zero" => ints.push(0),
            _ => {
                let d = cap_str.parse::<i32>().unwrap();
                ints.push(d);
            }
        }
    }
    ints
}

fn get_first_and_last_int_from_string(s: String) -> (i32, i32) {
    let mut ints = parse_ints_from_string(s);
    let first = ints.remove(0);
    // If there is only one int, then the first and last are the same
    if ints.len() == 0 {
        return (first, first);
    }
    let last = ints.pop().unwrap();
    (first, last)
}

fn day1_b_task(file_path: String) {
    // Read the calibration data from the file
    let data = day1_a::read_calibration_data_from_file(file_path.to_string());
    let mut result = 0;
    for d in data {
        let first_and_last_ints = get_first_and_last_int_from_string(d);
        let combined_ints = day1_a::combine_ints(first_and_last_ints.0, first_and_last_ints.1);
        result += combined_ints;
    }
    println!("Result: {}", result);
}

fn main() {
    // Parse the command line arguments
    let args: Vec<String> = env::args().collect();
    // Get the file path from the command line arguments
    let file_path = &args[1];
    day1_b_task(file_path.to_string());
}
