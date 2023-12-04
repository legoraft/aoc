pub fn main() {
    let games = include_str!("../inputs/day_02.txt");
    let answer: Vec<_> = vec!["Hello", "World"];

    let games = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    println!("-- Day Two --\nPart 1: {:?}\nPart 2: {:?}\n", answer[0], answer[1]);
    part_one(games);
}

struct Colors {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn part_one(games: &str) {
    let games: Vec<Vec<&str>> = games.lines().map(|line| process_games(line)).collect();

    for game in games {
        let cubes = process_cubes(game);
        dbg!(cubes.blue);
        dbg!(cubes.red);
        dbg!(cubes.green);
    }
}

fn process_games(game: &str) -> Vec<&str> {
    let game: Vec<&str> = game.split(":").collect();
    let rounds: Vec<&str> = game.iter().nth(1).unwrap().split(";").collect();
    rounds
}

fn process_cubes(game: Vec<&str>) -> Colors {
    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;

    for round in game {
        let cubes: Vec<&str> = round.split(",").collect();

        for cube in cubes {
            let num_color: Vec<&str> = cube.split(" ").collect();
            let number: u32 = num_color[0].trim().parse().unwrap();
            let color: &str = num_color[1].trim();

            match color {
                "red" => red = number,
                "green" => green = number,
                "blue" => blue = number,
                _ => eprintln!("That shouldn't be possible...")
            }
        }
    }

    Colors {
        red: red,
        green: green,
        blue: blue,
    }
}