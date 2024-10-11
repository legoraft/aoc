fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part_one() {
        let input_file: &str = "\
Time:      7  15   30
Distance:  9  40  200";
        
        let answer: i64 = 288;

        assert_eq!(answer, part_one(input_file));
}