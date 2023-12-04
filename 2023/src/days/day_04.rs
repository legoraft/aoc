use std::ops::Index;

pub fn main() {
    let cards = include_str!("../inputs/day_04.txt");
    let answer: Vec<u32> = vec![part_one(cards), 0];

    println!("-- Day Four --\nPart 1: {:?}\nPart 2: {:?}\n", answer[0], answer[1]);
}

pub fn part_one(cards: &str) -> u32 {
    let cards: Vec<&str> = cards.lines().collect();
    let mut total_score: u32 = 0;

    for card in cards {
        let mut score: u32 = 0;
        let (winning_numbers, my_numbers) = process_cards(card);

        for number in my_numbers {
            let win = winning_numbers.contains(&number);

            if win && score == 0 {
                score += 1;
            } else if win {
                score += score;
            }
        }
        total_score += score;
    }

    total_score
}

pub fn part_two(cards: &str) {
    let mut cards: Vec<&str> = cards.lines().collect();

    for card in &cards {
        let (winning_numbers, my_numbers) = process_cards(card);
    }
}

fn process_cards(card: &str) -> (Vec<&str>, Vec<&str>) {
    let card = &card[10..];
    let card: Vec<&str> = card.split("|").collect();

    let winning_numbers: Vec<&str> = card[0].trim().split(" ").collect();
    let mut my_numbers: Vec<&str> = card[1].trim().split(" ").collect();
    my_numbers.retain(|value| *value != "");

    (winning_numbers, my_numbers)
}