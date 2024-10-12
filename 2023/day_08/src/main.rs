fn main() {
    let input_file = include_str!("../../inputs/day_08.txt");

    let answer_one = part_one(input_file);
//    let answer_two = part_two(input_file);

    println!("Part one: {}\nPart two: {}", answer_one, /* answer_two */);
}

fn part_one(file: &str) -> i64 {
    0
}

fn parse(file: &str) {
    
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const INPUT: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    
    #[test]
    fn test_part_one() {
        let answer: i64 = 6;

        assert_eq!(answer, part_one(INPUT));
    }
}