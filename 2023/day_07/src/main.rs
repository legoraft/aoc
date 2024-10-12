use std::collections::HashMap;

fn main() {
    let input_file = include_str!("../../inputs/day_07.txt");

    let answer_one = part_one(input_file);
    let answer_two = part_two(input_file);

    println!("Part one: {}\nPart two: {}", answer_one, answer_two);
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    score: [i64; 2],
    bid: i64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(card: char) -> Self {
        match card {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("That shouldn't be possible!")
        }
    }
}

fn part_one(file: &str) -> i64 {
    let games = parse(file);
    let mut hands: Vec<Hand> = Vec::new();
    
    for mut game in games {
        let mut score: HashMap<&Card, i64> = HashMap::new();
        
        for card in &game.cards {
            score.entry(card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        
        let mut values: Vec<i64> = score.values().cloned().collect();
        values.sort();
        values.reverse();
        values.push(0);
        
        let score: [i64; 2] = [values[0], values[1]];
        game.score = score;
        
        hands.push(game);
    }
    
    hands.sort_by(|a, b| {
        let score_cmp = a.score.cmp(&b.score);
        
        if score_cmp == std::cmp::Ordering::Equal {
            a.cards.cmp(&b.cards)
        } else {
            score_cmp
        }
    });
    
    let mut answer = 0;
    
    for (rank, hand) in hands.iter().enumerate() {
        let score = hand.bid * (rank as i64 + 1);
        answer += score;
    }
    
    answer
}

fn part_two(file: &str) -> i64 {
    0
}

fn parse(file: &str) -> Vec<Hand> {
    let hands: Vec<(&str, &str)> = file.lines().map(|l| l.split_once(" ").expect("Couldn't split line!")).collect();
    let mut games: Vec<Hand> = Vec::new();
    
    for (cards, bid) in hands {
        let bid: i64 = bid.parse().expect("Couldn't parse bid!");
        let cards: Vec<Card> = cards.chars().map(|c| Card::from_char(c)).collect();
        
        games.push(Hand {
            cards,
            score: [0, 0],
            bid,
        });
    }
    
    games
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const INPUT: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    
    #[test]
    fn test_part_one() {
        let answer: i64 = 6440;

        assert_eq!(answer, part_one(INPUT));
    }
    
    #[test]
    fn test_part_two() {
        let answer: i64 = 5905;

        assert_eq!(answer, part_two(INPUT));
    }
}