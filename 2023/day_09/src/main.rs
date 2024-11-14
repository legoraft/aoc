fn main() {
    let input_file = include_str!("../../inputs/day_09.txt");

    let answer_one = part_one(input_file);
    
    println!("Part one: {}", answer_one);
}

fn part_one(file: &str) -> i64 {
    let input = parse(file);
    dbg!(input);
    
    0
}

fn parse(file: &str) -> Vec<Vec<i64>> {
    let lines: Vec<&str> = file.lines().collect();
    let histories: Vec<Vec<i64>> = lines.iter().map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect()).collect();
    
    histories
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part_one() {
        let input: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        
        let answer: i64 = 114;

        assert_eq!(answer, part_one(input));
    }
}