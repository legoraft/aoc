use std::collections::HashSet;
use crate::output_part;

pub fn main() {
    let cards = include_str!("../inputs/day_04.txt");

    println!("{}", output_part(|| part_one(cards), || part_two(cards), "04"))
}

struct Card {
    winning_numbers: HashSet<i64>,
    my_numbers: HashSet<i64>,
}

impl Card {
    fn count(&self) -> usize {
        let count = self.winning_numbers.intersection(&self.my_numbers).count();
        count
    }
}

pub fn part_one(cards: &str) -> i64 {
    let cards = parse(cards);
    let mut answer: i64 = 0;

    for card in cards {
        let count = card.count();
        if count > 0 {
            answer += 2_i64.pow((count - 1) as u32);
        } else {
            answer += 0;
        }
    }
    answer
}

pub fn part_two(cards: &str) -> i64 {
    let cards = parse(cards);
    let mut copies = vec![1; cards.len()];

    for (index, card) in cards.iter().enumerate() {
        let count = card.count();

        for i in index + 1..index+1+count {
            copies[i] += copies[index];
        }
    }
    copies.iter().sum()
}

fn parse(cards: &str) -> Vec<Card> {
    let lines: Vec<&str> = cards.lines().collect();
    let mut cards: Vec<Card> = Vec::new();

    for line in lines {
        let (_, numbers) = line.split_once(": ").unwrap();
        let (winning_string, my_string) = numbers.split_once(" | ").unwrap();

        let winning_numbers = winning_string.split_whitespace().map(|num| num.parse::<i64>().unwrap()).collect::<HashSet<_>>();
        let my_numbers = my_string.split_whitespace().map(|num| num.parse::<i64>().unwrap()).collect::<HashSet<_>>();

        cards.push(Card{
            winning_numbers,
            my_numbers,
        })
    }
    cards
}