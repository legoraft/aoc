use crate::output_part;

pub fn main() {
    let hands = include_str!("../inputs/day_07.txt");

    println!("{}", output_part(|| part_one(hands), || part_two(hands), "07"))
}

#[derive(Debug)]
struct Hand {
    bid: i64,
    cards: Vec<char>,
    values: Vec<i64>,
}

pub fn part_one(hands: &str) -> i64 {
    let games = parse(hands);
    dbg!(games);
    0
}

pub fn part_two(hands: &str) -> i64 {
    0
}

fn parse(hands: &str) -> Vec<Hand> {
    let hands: Vec<&str> = hands.lines().collect();
    let mut games: Vec<Hand> = Vec::new();

    for hand in hands {
        let (cards, bid) = hand.split_once(" ").unwrap();

        let bid: i64 = bid.parse().unwrap();
        let cards: Vec<char> = cards.chars().collect();

        let values: Vec<String> = cards.iter().map(|char| char.to_string()).collect();
        let values: Vec<String> = values.iter().map(|card| card
                .replace("A", "14")
                .replace("K", "13")
                .replace("Q", "12")
                .replace("J", "11")
                .replace("T", "10")
        ).collect();

        games.push(Hand{
            bid,
            cards,
            values: values.iter().map(|s| s.parse::<i64>().unwrap()).collect(),
        })
    }

    games
}