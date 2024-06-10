use std::collections::HashSet;

fn main() {
    let input_file = include_str!("../../../2023/inputs/day_03.txt");

    let answer_one = part_one(input_file);

    println!("Part one: {}\n", answer_one);
}

#[derive(Debug)]
struct PartNumber {
    value: i64,
    points: HashSet<(i64, i64)>,
}

impl PartNumber {
    fn new(x: i64, y: i64, ch: char) -> Self {
        let points = HashSet::from([
            (x - 1, y - 1), (x - 1, y), (x - 1, y + 1),
            (x, y - 1), (x, y + 1),
            (x + 1, y - 1), (x + 1, y), (x + 1, y + 1)
        ]);
        Self {
            value: (ch as u8 - b'0') as i64,
            points,
        }
    }
    fn add_digit(&mut self, x: i64, y: i64, ch: char) {
        self.value = self.value * 10 + (ch as u8 - b'0') as i64;
        self.points.extend([ (x + 1, y - 1), ((x + 1, y)), (x + 1, y + 1) ]);
    }
}

fn part_one(input: &str) -> i64 {
    let map = parser(input);

    let mut nums: Vec<PartNumber> = Vec::new();
    let mut syms: HashSet<(i64, i64)> = HashSet::new();

    let mut cur_number: Option<PartNumber> = None;

    for (y, lines) in map.iter().enumerate() {
        for (x, &ch) in lines.iter().enumerate() {
            if ch.is_digit(10) {
                if let Some(ref mut num) = cur_number {
                    num.add_digit(x as i64, y as i64, ch);
                } else {
                    cur_number = Some(PartNumber::new(x as i64, y as i64, ch));
                }
            } else {
                if let Some(num ) = cur_number.take() {
                    nums.push(num);
                } if ch != '.' {
                    syms.insert((x as i64, y as i64));
                }
            }
        }
    }

    let total: i64 = nums
        .iter()
        .filter(|num| num.points.intersection(&syms).next().is_some())
        .map(|num| num.value)
        .sum::<i64>();

    total
}

fn parser(file: &str) -> Vec<Vec<char>> {
    let map: Vec<Vec<char>> = file
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input_file: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let answer: i64 = 4361;

        assert_eq!(answer, part_one(input_file));
    }
}