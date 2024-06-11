use std::collections::HashSet;

fn main() {
    let input_file = include_str!("../../../2023/inputs/day_03.txt");

    let answer_one = part_one(input_file);
    let answer_two = part_two(input_file);

    println!("Part one: {}\nPart two: {}", answer_one, answer_two);
}

#[derive(Debug)]
struct Number {
    value: i64,
    coords: HashSet<(i64, i64)>,
}

impl Number {
    fn new(x: usize, y: usize, map: &Vec<Vec<char>>) -> Self {
        let mut number: String = String::new();
        let mut coords: HashSet<(i64, i64)> = HashSet::new();

        for x in x..map.len() {
            if map[y][x].is_digit(10) {
                number.push(map[y][x]);

                let x = x as i64;
                let y = y as i64;

                coords.extend(HashSet::from([
                    (x - 1, y - 1), (x - 1, y), (x - 1, y + 1),
                    (x, y - 1), (x, y + 1),
                    (x + 1, y - 1), (x + 1, y), (x + 1, y + 1),
                ]));
            } else {
                break;
            }
        }

        Number {
            value: number.parse::<i64>().expect("Couldn't parse number!"),
            coords,
        }
    }
}

fn part_one(input: &str) -> i64 {
    let map = parser(input);
    let mut answer = 0;

    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: HashSet<(i64, i64)> = HashSet::new();

    for (y, line) in map.iter().enumerate() {
        let mut n = 0;

        for (x, &ch) in line.iter().enumerate() {
            if n > 0 {
                n -= 1;
                continue;
            }
            
            if ch.is_digit(10) {
                let num = Number::new(x, y, &map);
                n += (num.value.to_string()).len() - 1;
                numbers.push(num);
            } else if ch != '.' {
                let coords = [
                    (x as i64, y as i64),
                ];
                symbols.extend(coords);
            } else {
                continue 
            }
        }
    }

    for number in numbers {
        if number.coords.intersection(&symbols).next().is_some() {
            answer += number.value;
        }
    }
    
    answer
}

fn part_two(input: &str) -> i64 {
    let map = parser(input);

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

    #[test]
    fn test_part_two() {
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

        let answer: i64 = 467835;

        assert_eq!(answer, part_two(input_file));
    }
}