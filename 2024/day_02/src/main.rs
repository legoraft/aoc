fn main() {
    let input_file = include_str!("../../inputs/day_02.txt");
    
    let answer_one = part_one(input_file);
    
    println!("Part one: {}", answer_one);
}

fn part_one(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_part_one() {
        let answer = 2;
        
        assert_eq!(answer, part_one(INPUT))
    }
}