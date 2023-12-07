use std::collections::HashSet;
use std::thread::panicking;
use crate::output_part;

pub fn main() {
    let games = include_str!("../inputs/day_07.txt");

    println!("{}", output_part(|| part_one(games), || part_two(games), "07"))
}

#[derive(Debug)]
struct Hand {
    bid: i64,
    cards: HashSet<Card>,
    values: Vec<i64>,
    rank: i64,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Card {
    id: char,
    amount: i64,
}

pub fn part_one(games: &str) -> i64 {
    let games: Vec<Hand> = parse(games);
    for game in games {
        // combos are numbered as follows:
        // 1 = high card
        // 2 = one pair
        // 3 = two pair
        // 4 = three of a kind
        // 5 = full house
        // 6 = four of a kind
        // 7 = five of a kind
        let mut combo = 0;
        let mut nothing = 0;
        let mut pairs = 0;
        let mut threes = 0;

        for card in &game.cards {
            match card.amount {
                1 => nothing += 1,
                2 => pairs += 1,
                3 => threes += 1,
                4 => combo = 6,
                5 => combo = 7,
                _ => println!("Prob something wrong...")
            }
        }

        if pairs == 1 && threes == 1 {
            combo = 5;
        } else if pairs == 2 {
            combo = 3;
        } else if nothing == game.cards.len() + 1 {
            combo = 1;
        } else {
            combo = threes * 4 + pairs * 2;
        }
    }

    0
}

pub fn part_two(games: &str) -> i64 {
    0
}

fn parse(games: &str) -> Vec<Hand> {
    let games: Vec<&str> = games.lines().collect();
    let mut rounds: Vec<Hand> = Vec::new();

    for game in games {
        let (hand, bid) = game.split_once(" ").unwrap();
        let mut cards: Vec<Card> = Vec::new();

        let bid: i64 = bid.parse().unwrap();
        let hand: Vec<char> = hand.chars().collect();

        let values: Vec<String> = hand.iter().map(|char| char.to_string()).collect();
        let values: Vec<String> = values.iter().map(|card| card
            .replace("A", "14")
            .replace("K", "13")
            .replace("Q", "12")
            .replace("J", "11")
            .replace("T", "10")
        ).collect();

        for card in &hand {
            let card_count = hand.iter().filter(|count| *count == card).count();
            cards.push(Card {
                id: *card,
                amount: card_count as i64,
            });
        }

        let cards: HashSet<Card> = cards.drain(..).collect();

        rounds.push(Hand{
            bid,
            cards,
            values: values.iter().map(|s| s.parse::<i64>().unwrap()).collect(),
            rank: 1,
        })
    }

    rounds
}