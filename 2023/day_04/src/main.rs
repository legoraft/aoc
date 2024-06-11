fn main() {
    let input_file = include_str!("../../../2023/inputs/day_04.txt");

    let answer_one = part_one(input_file);

    println!("Part one: {}\n", answer_one);
}

fn part_one(input: &str) -> i64 {
    
    
    0
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
}
