fn main() {
    let input_file = include_str!("../../inputs/day_02.txt");
    
    let answer_one = part_one(input_file);
    let answer_two = part_two(input_file);
    
    println!("Part one: {}\nPart two: {}", answer_one, answer_two);
}

fn part_one(input: &str) -> i64 {
    let reports = parse(input);
    let mut safe_reports = 0;
    
    for report in reports {
        let differences: Vec<i64> = report.windows(2).map(|n| n[1] - n[0]).collect();
        
        if differences.iter().all(|&n| n > 0) || differences.iter().all(|&n| n < 0) {
            if differences.iter().all(|n| n.abs() > 0 && n.abs() < 4) {
                safe_reports += 1;
            }
        }
    }
    
    safe_reports
}

fn part_two(input: &str) -> i64 {
    let reports = parse(input);
    let mut safe_reports = 0;
    
    for report in reports {
        let differences: Vec<i64> = report.windows(2).map(|n| n[1] - n[0]).collect();
        
        if differences.iter().all(|&n| n > 0) || differences.iter().all(|&n| n < 0) {
            if differences.iter().all(|n| n.abs() > 0 && n.abs() < 4) {
                safe_reports += 1;
            }
        }
    }
    
    safe_reports
}

fn parse(file: &str) -> Vec<Vec<i64>> {
    let reports: Vec<Vec<i64>> = file.lines()
        .map(|l| l.split_whitespace()
            .map(|n| n.parse().expect("Couldn't parse number!"))
            .collect())
        .collect();
    
    reports
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
    
    #[test]
    fn test_part_one() {
        let answer = 4;
        
        assert_eq!(answer, part_two(INPUT))
    }
}