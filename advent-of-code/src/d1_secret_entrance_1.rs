use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INIT_SAFE_VALUE: i16 = 50;
const MAX_SAFE_VALUE: i16 = 100;

/**
 * https://adventofcode.com/2025/day/1
 * Honestly, it's just starting from 50
 * 1. Sum through all the values where L is negative and R is positive
 * 2. And you mod 100, so you can generalize it as a (mod b) = b - (|a| (mod b))
 *
 * But can we parse in a way where we can utilize some trait impl
 * **/
pub fn secret_entrance_1() {
    let input_file = File::open("./src/input/01_input.txt").unwrap();
    let reader = BufReader::new(input_file);
    let mut safe_pointer: i16 = INIT_SAFE_VALUE;
    let mut password = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let (direction, value) = line.split_at(1);
        let value_u8 = value.parse::<i16>().unwrap();

        match direction {
            "L" => safe_pointer -= value_u8,
            "R" => safe_pointer += value_u8,
            _ => panic!("yo this is not allowed"),
        }

        safe_pointer = safe_pointer.rem_euclid(MAX_SAFE_VALUE);
        if safe_pointer == 0 {
            password += 1;
        }
    }

    println!("d1.1 The password is {}", password);
}
