// Day 1 of the advent of code challenge
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_calibration_data_from_file(file_path: String) -> Vec<String> {
    let file = File::open(file_path).expect("File not found");
    let reader = BufReader::new(file);
    let mut data = Vec::new();
    for line in reader.lines() {
        data.push(line.unwrap());
    }
    data
}

fn parse_ints_from_string(s: String) -> Vec<i32> {
    let mut ints = Vec::new();
    for c in s.chars() {
        match c.to_digit(10) {
            Some(d) => ints.push(d as i32),
            None => continue,
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

fn combine_ints(i1: i32, i2: i32) -> i32 {
    i1 * 10 + i2
}

fn day1_task(file_path: String) {
    // Read the calibration data from the file
    let data = read_calibration_data_from_file(file_path.to_string());
    let mut result = 0;
    for d in data {
        let first_and_last_ints = get_first_and_last_int_from_string(d);
        let combined_ints = combine_ints(first_and_last_ints.0, first_and_last_ints.1);
        result += combined_ints;
    }
    println!("Result: {}", result);
}

fn main() {
    // Parse the command line arguments
    let args: Vec<String> = env::args().collect();
    // Get the file path from the command line arguments
    let file_path = &args[1];
    day1_task(file_path.to_string());
}
