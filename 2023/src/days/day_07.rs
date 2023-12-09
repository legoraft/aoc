use crate::output_part;

pub fn main() {
    let games = include_str!("../inputs/day_07.txt");

    println!("{}", output_part(|| part_one(games), || part_two(games), "07"))
}

#[derive(Debug)]
struct Game {
    hand: Vec<Card>,
    score: usize,
    bet: i64,
}

#[derive(Debug)]
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
    fn from_char(char: char) -> Self {
        match char {
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
            _ => panic!("That isn't possible"),
        }
    }
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
    let mut hands: Vec<Game> = Vec::new();

    for line in lines {
        let (hand, bet) = line.split_once(" ").unwrap();

        let hand: Vec<Card> = hand.chars().map(|c| Card::from_char(c)).collect();
        hands.push(Game{
            hand,
            score: 0,
            bet: bet.parse::<i64>().unwrap(),
        })
    }

    hands
}