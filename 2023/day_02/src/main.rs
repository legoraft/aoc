fn main() {
    let input_file = include_str!("../../inputs/day_02.txt");

    let answer_one = part_one(input_file);
    let answer_two = part_two(input_file);

    println!("Part one: {}\nPart two: {}", answer_one, answer_two);
}

fn part_one(input: &str) -> i64 {
    let games: Vec<&str> = parse(input);
    let mut answer: i64 = 0;

    'outer: for (id, game ) in games.iter().enumerate() {
        let id = id + 1;

        let draws: Vec<&str> = game
            .split([',', ';'])
            .map(|s| s.trim())
            .collect();

        for draw in draws {
            let (count, color) = draw
                .split_once(" ")
                .expect("Can't split draw!");

            let count = count.parse::<i64>().expect("Can't parse count!");

            let possible = match color {
                "red" =>  count <= 12,
                "green" =>  count <= 13,
                "blue" =>  count <= 14,
                _ => panic!("That shouldn't happen..."),
            };

            if !possible { continue 'outer; }
        }

        answer += id as i64;

    }

    answer
}

fn part_two(input: &str) -> i64 {
    let games: Vec<&str> = parse(input);
    let mut answer: i64 = 0;

    for game in games {
        let mut red: Vec<i64> = Vec::new();
        let mut green: Vec<i64> = Vec::new();
        let mut blue: Vec<i64> = Vec::new();

        let draws: Vec<&str> = game
            .split([',', ';'])
            .map(|s| s.trim())
            .collect();

        for draw in draws {
            let (count, color) = draw
                .split_once(" ")
                .expect("Can't split draw!");

            let count = count.parse::<i64>().expect("Can't parse count!");

            match color {
                "red" =>  red.push(count),
                "green" =>  green.push(count),
                "blue" =>  blue.push(count),
                _ => panic!("That shouldn't happen..."),
            };
        }

        let red = red.iter().max().expect("Couldn't find minimum value!");
        let green = green.iter().max().expect("Couldn't find minimum value!");
        let blue = blue.iter().max().expect("Couldn't find minimum value!");

        let power: i64 = red * green * blue;

        answer += power;

    }

    answer
}

fn parse(file: &str) -> Vec<&str> {
    let lines: Vec<&str> = file.lines()
        .map(|l| { 
            let (_, cubes) = l
                .split_once(": ")
                .expect("Couldn't split line!"); cubes })
        .collect();

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input_file: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let answer: i64 = 8;

        assert_eq!(answer, part_one(input_file));
    }

    #[test]
    fn test_part_two() {
        let input_file: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let answer: i64 = 2286;

        assert_eq!(answer, part_two(input_file));
    }
}
