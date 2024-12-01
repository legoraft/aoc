fn main() {
    let input_file = include_str!("../../inputs/day_01.txt");
    
    let answer_one = part_one(input_file);
    
    println!("Part one: {}", answer_one);
}

fn part_one(input: &str) -> i64 {
    parse(input);
    
    0
}

fn parse(file:  &str) {
    let input: Vec<&str> = file.split_whitespace().collect();
    dbg!(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "\
3   4
4   3
2   5
1   3
3   9
3   3";

        let answer = 11;

        assert_eq!(answer, part_one(input))
    }
}
