use crate::output_part;

pub fn main() {
    let schematic = include_str!("../inputs/day_03.txt");

    println!("{}", output_part(|| part_one(schematic), || part_two(schematic), "03"))
}

struct Symbol {
    symbol: &'static str,
    coordinates: Vec<i64>,
}

pub fn part_one(schematic: &str) -> i64 {
    0
}

pub fn part_two(schematic: &str) -> i64 {
    0
}

fn parse() {

}