use crate::output_part;

pub fn main() {
    let games = include_str!("../inputs/day_07.txt");

    println!("{}", output_part(|| part_one(games), || part_two(games), "07"))
}

#[derive(Debug)]
struct Game {
    cards: String,
    bid: i64,
    strength: i64,
}

pub fn part_one(games: &str) -> i64 {
    let games = parse(games);
    dbg!(games);

    0
}

pub fn part_two(_games: &str) -> i64 {
    0
}

fn parse(games: &str) -> Vec<Game> {
    let lines: Vec<&str> = games.lines().collect();
    let mut games: Vec<Game> = Vec::new();

    for line in lines {
        let (cards, bid) = line.split_once(" ").unwrap();
        let strength = calculate_strength(cards);

        games.push(Game{
            cards: cards.to_string(),
            bid: bid.parse::<i64>().unwrap(),
            strength,
        })
    }
    games
}

fn calculate_strength(cards: &str) -> i64 {
    let cards: Vec<char> = cards.chars().collect();
    let mut counts: Vec<i64> = Vec::new();
    let mut strength: i64 = 0;

    for card in &cards {
        let count = cards.iter().filter(|char| *char == card).count();
        counts.push(count as i64);
    }

    let counts_max: i64 = *counts.iter().max().unwrap();

    if counts_max > 3 {
        strength = counts_max + 1;
    } else if counts_max == 3 {
        if counts.contains(&2) {
            strength = counts_max + 1
        }
        strength = counts_max
    } else if counts_max == 2 {
        if counts.iter().filter(|card| *card == &2_i64).count() > 2 {
            strength = counts_max;
        } else {
            strength = counts_max - 1
        }
    } else {
        strength = 0
    }

    strength
}