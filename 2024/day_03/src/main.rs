fn main() {
    let input_file = include_str!("../../inputs/day_03.txt");
    
    let answer_one = part_one(input_file);
    
    println!("Part one: {}", answer_one);
}

fn part_one(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    
    fn test_part_one() {
        let answer = 161;
        
        assert_eq(answer, part_one(INPUT))
    }
}
