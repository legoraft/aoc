use crate::output_part;

pub fn main() {
    let calibration_value = include_str!("../inputs/day_01.txt");

    println!("{}", output_part(|| part_one(calibration_value), || part_two(calibration_value), "01"))
}

pub fn part_one(calibration_value: &str) -> i64 {
    let lines: Vec<&str> = calibration_value.lines().collect();
    let mut values: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        let numbers: Vec<u32> = line.chars().filter_map(|num| num.to_digit(10)).collect();
        values.push(numbers);
    }

    let sum = calculate_sum(values);
    sum
}

pub fn part_two(calibration_value: &str) -> i64 {
    let lines: Vec<&str> = calibration_value.lines().collect();
    let mut values: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        let numbers: Vec<u32> = line.to_string()
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
            .chars().filter_map(|num| num.to_digit(10)).collect();
        values.push(numbers);
    }

    let sum = calculate_sum(values);
    sum
}

fn calculate_sum(values: Vec<Vec<u32>>) -> i64 {
    let mut sum: i64 = 0;

    for numbers in values {
        let digit = numbers[0].to_string() + &*numbers[numbers.len() - 1].to_string();
        sum += digit.parse::<i64>().unwrap();
    }
    sum
}