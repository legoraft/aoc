fn main() {
    let input_file = include_str!("../../inputs/day_01.txt");

    let answer_one = part_one(input_file);
    let answer_two = part_two(input_file);

    println!("Part one: {}\nPart two: {}", answer_one, answer_two);
}

fn part_one(input: &str) -> i64 {
    let lines = parse(input);

    let mut answer: i64 = 0;

    for line in lines {
        let nums: Vec<char> = line.chars()
            .filter(|c| c.is_digit(10))
            .collect();
        
        let number_string: String = [nums[0], nums[nums.len() - 1]].iter().collect();
        let number: i64 = number_string.parse::<i64>().expect("Can't parse string!");

        answer += number;
    }

    answer
}

fn part_two(input: &str) -> i64 {
    let lines = parse(input);

    281
}

fn parse(file: &str) -> Vec<&str> {
    let lines: Vec<&str> = file.lines().collect();

    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part_one() {
        let input_file: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let answer: i64 = 142;

        assert_eq!(answer, part_one(input_file));
    }

    #[test]
    fn test_part_two() {
        let input_file: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let answer: i64 = 281;

        assert_eq!(answer, part_two(input_file));
    }
}
