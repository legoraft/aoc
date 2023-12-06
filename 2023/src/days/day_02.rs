use crate::output_part;

pub fn main() {
    let games = include_str!("../inputs/day_02.txt");

    println!("{}", output_part(|| part_one(games), || part_two(games), "02"))
}

struct Colors {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn part_one(games: &str) -> i64 {
    let games: Vec<&str> = games.lines().collect();
    let mut score: i64 = 0;

    for game in games {
        let (cubes, id) = parse(game);

        if cubes.red <= 12 && cubes.green <= 13 && cubes.blue <= 14 {
            score += id;
        }
    }
    score
}

pub fn part_two(games: &str) -> i64 {
    let games: Vec<&str> = games.lines().collect();
    let mut score: i64 = 0;

    for game in games {
        let (cubes, _id) = parse(game);

        let power = cubes.red * cubes.green * cubes.blue;
        score += power;
    }
    score
}

fn parse(game: &str) -> (Colors, u32) {
    let mut red: Vec<u32> = Vec::new();
    let mut green: Vec<u32> = Vec::new();
    let mut blue: Vec<u32> = Vec::new();

    let game: Vec<&str> = game.split(|c| c == ':' || c == ';').collect();
    let id: u32 = game[0].replace("Game ", "").parse().unwrap();

    for round in &game [1..] {
        let colors: Vec<Vec<&str>> = round.split(",").map(|color| color.split(" ").collect()).collect();

        for cube in colors {
            let amount: u32 = cube[1].parse().unwrap();
            let color: &str = cube[2];

            match color {
                "red" => red.push(amount),
                "green" => green.push(amount),
                "blue" => blue.push(amount),
                _ => eprintln!("That isn't possible...")
            }
        }
    }

    let red = *red.iter().max().unwrap();
    let green = *green.iter().max().unwrap();
    let blue = *blue.iter().max().unwrap();

    ( Colors {
        red,
        green,
        blue,
    },
    id )
}