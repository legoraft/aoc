pub fn main() {
    let calibration_value = include_str!("../inputs/day_01.txt");
    println!("Part 1: {:?}\nPart 2: {:?}", part_one(calibration_value), part_two(calibration_value));
}

pub fn part_one(calibration_value: &str) -> u32 {
    let lines: Vec<&str> = calibration_value.lines().collect();
    let mut values: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        let numbers: Vec<u32> = line.chars().filter_map(|num| num.to_digit(10)).collect();
        values.push(numbers);
    }

    let sum = calculate_sum(values);
    sum
}

pub fn part_two(calibration_value: &str) -> u32 {
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
    println!("{sum}");
    sum
}

fn calculate_sum(values: Vec<Vec<u32>>) -> u32 {
    let mut sum: u32 = 0;

    for numbers in values {
        let digit = numbers[0].to_string() + &*numbers[numbers.len() - 1].to_string();
        sum += digit.parse::<u32>().unwrap();
    }
    sum
}