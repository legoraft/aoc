fn main() {
    let input_file = include_str!("../../inputs/day_01.txt");
    
    let answer_one = part_one(input_file);
    
    println!("Part one: {}", answer_one);
}

fn part_one(input: &str) -> i64 {
    let (mut list_one, mut list_two) = parse(input);
    let mut answer: i64 = 0;
    
    list_one.sort();
    list_two.sort();
    
    for (index, item) in list_one.iter().enumerate() {
        let difference = item - list_two[index];
        
        answer += difference.abs();
    }
    
    answer
}

fn parse(file:  &str) -> (Vec<i64>, Vec<i64>) {
    let input: Vec<i64> = file.split_whitespace().map(|n| n.parse().expect("Couldn't parse number!")).collect();
    
    let list_one: Vec<i64> = input.clone().into_iter().step_by(2).collect();
    let list_two: Vec<i64> = input.clone().into_iter().skip(1).step_by(2).collect();

    (list_one, list_two)
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
