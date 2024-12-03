use regex::Regex;

fn main() {
    let input_file = include_str!("../../inputs/day_03.txt");
    
    let answer_one = part_one(input_file);
    
    println!("Part one: {}", answer_one);
}

fn part_one(input: &str) -> i64 {
    let instructions = parse(input);
    let mut answer = 0;
    
    for instruction in instructions {
        let multiplication = instruction[0] * instruction[1];
        answer += multiplication
    }
    
    answer
}

fn parse(file: &str) -> Vec<Vec<i64>> {
    let re = Regex::new(r"mul\(([0-9]+,[0-9]+)\)").unwrap();
    
    let numbers: Vec<Vec<i64>> = re.captures_iter(file)
        .map(|c| c.get(1).unwrap().as_str()
            .split(",").map(|n| n.parse().unwrap())
            .collect())
        .collect();

    numbers
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    
    #[test]
    fn test_part_one() {
        let answer = 161;
        
        assert_eq!(answer, part_one(INPUT))
    }
}
