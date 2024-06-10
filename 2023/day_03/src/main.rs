fn main() {
    let input_file = include_str!("../../../2023/inputs/day_03.txt");

    let answer_one = part_one(input_file);

    println!("Part one: {}\n", answer_one);
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

    for char in chars {
        
    }

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