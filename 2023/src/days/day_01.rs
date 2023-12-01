pub fn main() {
    let calibration_value = include_str!("../inputs/day_01.txt");
    println!("{:?}", part_one(calibration_value));
}

pub fn part_one(calibration_value: &str) -> u32 {
    let lines: Vec<&str> = calibration_value.lines().collect();
    let mut values: Vec<Vec<u32>> = Vec::new();
    let mut sum: u32 = 0;

    for line in lines {
        let numbers: Vec<u32> = line.chars().filter_map(|num| num.to_digit(10)).collect();
        values.push(numbers);
    }

    for numbers in values {
        let digit = numbers[0].to_string() + &*numbers[numbers.len() - 1].to_string();
        sum += digit.parse::<u32>().unwrap();
    }

    sum
}

pub fn part_two(calibration_value: &str) -> u32 {


    281
}