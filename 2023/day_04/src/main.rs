use std::collections::HashSet;

fn main() {
    let input_file = include_str!("../../../2023/inputs/day_04.txt");

    let answer_one = part_one(input_file);
    let answer_two = part_two(input_file);

    println!("Part one: {}\nPart two: {}", answer_one, answer_two);
}

fn part_one(input: &str) -> i64 {
    let lines = parse(input);
    let mut answer: i64 = 0;

    for line in lines {
        let (winning, card) = line
            .split_once(" | ")
            .expect("Couldn't split numbers!");

        let winning_numbers: HashSet<i64> = winning
            .split_whitespace()
            .map(|num| num.parse::<i64>().expect("Can't parse number!"))
            .collect();

        let card_numbers: HashSet<i64> = card
            .split_whitespace()
            .map(|num| num.parse::<i64>().expect("Can't parse number!"))
            .collect();

        let power = winning_numbers.intersection(&card_numbers).count();

        if power > 0 {
            answer += 2_i64.pow((power - 1) as u32);
        }
    }
    
    answer
}

fn part_two(input: &str) -> i64 {
    let lines = parse(input);
    let mut answer: Vec<i64> = vec![1; lines.len()];

    for (index, line) in lines.iter().enumerate() {
        let (winning, card) = line
            .split_once(" | ")
            .expect("Couldn't split numbers!");

        let winning_numbers: HashSet<i64> = winning
            .split_whitespace()
            .map(|num| num.parse::<i64>().expect("Can't parse number!"))
            .collect();

        let card_numbers: HashSet<i64> = card
        .split_whitespace()
        .map(|num| num.parse::<i64>().expect("Can't parse number!"))
        .collect();

        let count = winning_numbers.intersection(&card_numbers).count();

        for i in index + 1..index + count + 1 {
            answer[i] += answer[index];
        }
    }
    answer.iter().sum()
}

fn parse(file: &str) -> Vec<&str> {
    let lines: Vec<&str> = file.lines()
        .map(|l| { 
            let (_, cards) = l
                .split_once(": ")
                .expect("Couldn't split line!"); cards })
        .collect();

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input_file: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

        let answer: i64 = 13;

        assert_eq!(answer, part_one(input_file));
    }

    #[test]
    fn test_part_two() {
        let input_file: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

        let answer: i64 = 30;

        assert_eq!(answer, part_two(input_file));
    }
}
