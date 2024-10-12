fn main() {
    let input_file = include_str!("../../inputs/day_07.txt");

    let answer_one = part_one(input_file);
    let answer_two = part_two(input_file);

    println!("Part one: {}\nPart two: {}", answer_one, answer_two);
}

fn part_one(file: &str) -> i64 {
    0
}

fn part_two(file: &str) -> i64 {
    0
}

fn parse(file: &str) {
    
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const INPUT: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    
    #[test]
    fn test_part_one() {
        let answer: i64 = 6440;

        assert_eq!(answer, part_one(INPUT));
    }
    
    #[test]
    fn test_part_two() {
        let answer: i64 = 5905;

        assert_eq!(answer, part_one(INPUT));
    }
}