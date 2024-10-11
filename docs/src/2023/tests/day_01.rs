fn main() {
    let input_file = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

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

    let mut answer: i64 = 0;

    for line in lines {
        let line = line
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3ree")
            .replace("four", "f4ur")
            .replace("five", "f5ve")
            .replace("six", "s6x")
            .replace("seven", "s7ven")
            .replace("eight", "e8ght")
            .replace("nine", "n9ne");

        let nums: Vec<char> = line.chars()
            .filter(|c| c.is_digit(10))
            .collect();
        
        let number_string: String = [nums[0], nums[nums.len() - 1]].iter().collect();
        let number: i64 = number_string.parse::<i64>().expect("Can't parse string!");

        answer += number;
    }

    answer
}

fn parse(file: &str) -> Vec<&str> {
    let lines: Vec<&str> = file.lines().collect();

    lines
}
