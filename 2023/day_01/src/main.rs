fn main() {
    println!("Hello, first day!");
}

fn part_one(file: &str) {
    
}

#[cfg(test)]
mod tests {
    fn test_part_one() {
        let input_file: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"

        let answer: i64 = 142

        assert_eq!(answer, part_one(input_file));
    }
}
