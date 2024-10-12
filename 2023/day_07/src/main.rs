use std::collections::HashMap;

fn main() {
    let input_file = include_str!("../../inputs/day_07.txt");

    let answer_one = part_one(input_file);
    let answer_two = part_two(input_file);

    println!("Part one: {}\nPart two: {}", answer_one, answer_two);
}

#[derive(Debug)]
struct Hand {
    cards: String,
    score: [i64; 2],
    bid: i64,
}

fn part_one(file: &str) -> i64 {
    let hands = parse(file);
    
    for mut hand in hands {
        let mut score: HashMap<char, i64> = HashMap::new();
        
        for card in hand.cards.chars() {
            score.entry(card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        
        let mut values: Vec<i64> = score.values().cloned().collect();
        values.sort();
        values.reverse();
        values.push(0);
        
        let score: [i64; 2] = [values[0], values[1]];
        hand.score = score;
    }
    
    0
}

fn part_two(file: &str) -> i64 {
    0
}

fn parse(file: &str) -> Vec<Hand> {
    let hands: Vec<(&str, &str)> = file.lines().map(|l| l.split_once(" ").expect("Couldn't split line!")).collect();
    let mut games: Vec<Hand> = Vec::new();
    
    for (cards, bid) in hands {
        let bid: i64 = bid.parse().expect("Couldn't parse bid!");
        let cards: String = cards.to_string();
        
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

        assert_eq!(answer, part_one(INPUT));
    }
}