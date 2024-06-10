use std::collections::HashSet;

fn main() {
    let input_file = include_str!("../../../2023/inputs/day_03.txt");

    let answer_one = part_one(input_file);

    println!("Part one: {}\n", answer_one);
}

struct PartNumber {
    value: i64,
    points: HashSet<(i64, i64)>,
}

impl PartNumber {
    fn new(x: i64, y: i64, ch: char) -> Self {
        let points = HashSet::from([
            (x - 1, y -1), (x, y -1), (x + 1, y - 1),
            (x - 1, y), (x + 1, y),
            (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)
        ]);
        Self {
            value: ch as i64,
            points,
        }
    }
    fn add_digit(&mut self, x: i64, y: i64, ch: char) {
        self.value = self.value * 10 + ch as i64;
        self.points.extend([ (x - 1, y + 1), ((x, y + 1)), (x + 1, y + 1) ]);
    }
}

fn part_one(input: &str) -> i64 {
    let map = parser(input);

    let chars: Vec<(usize, usize)> = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
            .enumerate()
            .filter_map(move |(x, &ch)| if !ch.is_digit(10) && ch != '.' {
                Some((x, y))
            } else {
                None
            })
        }).collect();

    

    0
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