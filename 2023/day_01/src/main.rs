fn main() {
    println!("Hello, first day!");
    let input_file: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    part_one(input_file);
}

fn part_one(input: &str) -> i64 {
    let lines = parse(input);

    for line in lines {
        let nums: Vec<u32> = line.chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).expect("Can't parse character!"))
            .collect();
        

    }

    1
}

fn parse(file: &str) -> Vec<&str> {
    let lines: Vec<&str> = file.lines().collect();

    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part_one() {
        let input_file: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let answer: i64 = 142;

        assert_eq!(answer, part_one(input_file));
    }
}
