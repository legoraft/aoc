use crate::output_part;

pub fn main() {
    let games = include_str!("../inputs/day_07.txt");

    println!("{}", output_part(|| part_one(games), || part_two(games), "07"))
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Game {
    cards: String,
    bid: i64,
    strength: i64,
}

pub fn part_one(games: &str) -> i64 {
    let mut games = parse(games);
    let mut total_winnings: i64 = 0;
    games.sort_by(|a, b| b.cards.cmp(&a.cards));
    games.sort_by(|a, b| b.strength.cmp(&a.strength));

    for (index, game) in games.iter().enumerate() {
        let rank: i64 = (games.len() - index) as i64;
        total_winnings += rank * game.bid;
    }

    total_winnings
}

pub fn part_two(_games: &str) -> i64 {
    0
}

fn parse(games: &str) -> Vec<Game> {
    let lines: Vec<&str> = games.lines().collect();
    let mut games: Vec<Game> = Vec::new();

    for line in lines {
        let (cards, bid) = line.split_once(" ").unwrap();
        let cards = cards
            .replace("A", "E")
            .replace("K", "D")
            .replace("Q", "C")
            .replace("J", "B")
            .replace("T", "A");

        let strength = calculate_strength(&cards);

        games.push(Game{
            cards,
            bid: bid.parse::<i64>().unwrap(),
            strength,
        })
    }
    games
}

fn calculate_strength(cards: &String) -> i64 {
    let cards: Vec<char> = cards.chars().collect();
    let mut counts: Vec<i64> = Vec::new();
    let strength: i64;

    for card in &cards {
        let count = cards.iter().filter(|char| *char == card).count();
        counts.push(count as i64);
    }

    let counts_max: i64 = *counts.iter().max().unwrap();

    if counts_max > 3 {
        strength = counts_max + 1;
    } else if counts_max == 3 {
        if counts.contains(&2) {
            strength = counts_max + 1;
        } else {
            strength = counts_max
        }
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