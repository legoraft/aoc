use core::num;
use std::collections::HashSet;

fn main() {
    let input_file = include_str!("../../../2023/inputs/day_03.txt");

    let answer_one = part_one(input_file);

    println!("Part one: {}\n", answer_one);
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
            } if ch != '.' {
                let x = x as i64;
                let y = y as i64;

                let coords = [
                    (x - 1, y - 1), (x - 1, y), (x - 1, y + 1),
                    (x, y - 1), (x, y + 1),
                    (x + 1, y - 1), (x + 1, y), (x + 1, y + 1),
                ];
                symbols.extend(coords);
            } else {
                continue 
            }
        }
    }

    println!("{:?}", symbols);

    for number in numbers {
        if number.coords.intersection(&symbols).next().is_some() {
            answer += number.value;
        }
    }
    
    answer
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