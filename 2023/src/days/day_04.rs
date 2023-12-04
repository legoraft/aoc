pub fn main() {
    let cards = include_str!("../inputs/day_04.txt");
    let answer: Vec<u32> = vec![part_one(cards), 0];

    println!("-- Day Four --\nPart 1: {:?}\nPart 2: {:?}\n", answer[0], answer[1]);
}

pub fn part_one(cards: &str) -> u32 {
    let cards: Vec<&str> = cards.lines().collect();
    let mut total_score: u32 = 0;

    for card in cards {
        let card = &card[10..];
        let card: Vec<&str> = card.split("|").collect();
        let mut score = 0;

        let winning_numbers: Vec<&str> = card[0].trim().split(" ").collect();
        let mut my_numbers: Vec<&str> = card[1].trim().split(" ").collect();
        my_numbers.retain(|value| *value != "");

        for number in my_numbers {
            let test = winning_numbers.contains(&number);

            if test && score == 0 {
                score += 1;
            } else if test {
                score += score;
            }
        }
        dbg!(score);
        total_score += score;
    }
    total_score
}