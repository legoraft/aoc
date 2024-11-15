fn main() {
    let input_file = include_str!("../../inputs/day_09.txt");

    let answer_one = part_one(input_file);
    
    println!("Part one: {}", answer_one);
}

fn part_one(file: &str) -> i64 {
    let histories = parse(file);
    let mut extrapolated: Vec<i64> = Vec::new();
    
    for history in histories {
        let mut edges: Vec<i64> = Vec::new();
        edges.push(history.last().unwrap().clone());
        
        let mut old_projections: Vec<i64> = history;
        let mut projections: Vec<i64>;
        let mut sum: i64 = old_projections.iter().sum();
        
        while sum != 0 {    
            projections = Vec::new();
            sum = old_projections.iter().sum();
            
            for (index, value) in old_projections.iter().enumerate() {
                if index != 0 {
                    let projection = value - old_projections[index - 1];
                    projections.push(projection);
                }
            }

            edges.push(projections.last().unwrap().clone());
            old_projections = projections;
        }
        
        extrapolated.push(edges.iter().sum());
    }
    
    extrapolated.iter().sum()
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
10 13 16 21 30 45
6 1 -4 -9 -14 -19";
        
        let answer: i64 = 90;

        assert_eq!(answer, part_one(input));
    }
}