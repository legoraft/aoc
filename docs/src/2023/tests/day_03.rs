use std::collections::HashSet;

fn main() {
    let input_file = "\
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
    let map = parse(input);
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
    let map = parse(input);
    let mut answer = 0;

    let mut numbers: Vec<Number> = Vec::new();
    let mut gears: HashSet<(i64, i64)> = HashSet::new();

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
            } else if ch == '*' {
                let coords = [
                    (x as i64, y as i64),
                ];
                gears.extend(coords);
            } else {
                continue 
            }
        }
    }

    for gear in gears {
        let mut hits: Vec<i64> = Vec::new(); 
        for number in &numbers {
            if number.coords.contains(&gear) {
                hits.push(number.value);
            }
        }
        if hits.len() == 2 {
            answer += hits[0] * hits[1];
        }
    }
    
    answer
}

fn parse(file: &str) -> Vec<Vec<char>> {
    let map: Vec<Vec<char>> = file
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    map
}
