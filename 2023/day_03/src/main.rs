use std::collections::HashSet;

fn main() {
    let input_file = include_str!("../../../2023/inputs/day_03.txt");

    let answer_one = part_one(input_file);

    println!("Part one: {}\n", answer_one);
}

fn part_one(input: &str) -> i64 {
    /*
    
    let map = parser(input);

    for (y, line) in map.iter.enumerate() {
        for (x, ch) in line.iter.enumerate() {
            if ch == digit {
                let (number, coords) = extend_digit()

                nums.push(Number {
                    number,
                    coords
                })
            } if ch == symbol {
                let coords = get_coords()
                syms.push(coords)
            } else {
                continue 
            }
        }
    }

    if syms.contains(nums.number.coords) {
        answer += number
    }

    extend_digit() {
        let mut number = vec!
        let mut coords = HashSet
        
        for x in x..map.len() {
            if x.isdigit() {
                number.push map[y][x]
                coords.insert((x, y))
            } else {
                break 
            }
        }

        number.parse, coords
    }

    */
    
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