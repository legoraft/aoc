use crate::output_part;

pub fn main() {
    let hands = include_str!("../inputs/day_03.txt");

    println!("{}", output_part(|| part_one(hands), || part_two(hands), "07"))
}

pub fn part_one(hands: &str) -> i64 {
    parse(hands);

    0
}

pub fn part_two(hands: &str) -> i64 {
    0
}

fn parse(hands: &str) {

}