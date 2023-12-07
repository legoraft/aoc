use crate::output_part;

pub fn main() {
    let schematic = include_str!("../inputs/day_03.txt");

    println!("{}", output_part(|| part_one(schematic), || part_two(schematic), "03"))
}

#[derive(Debug)]
struct Symbol {
    symbol: String,
    coordinates: Vec<Vec<i64>>,
}

pub fn part_one(schematic: &str) -> i64 {
    let symbols = parse(schematic);

    0
}

pub fn part_two(schematic: &str) -> i64 {
    0
}

fn parse(schematic: &str) -> Vec<Symbol> {
    let rows: Vec<&str> = schematic.lines().collect();
    let symbols = vec!["*", "#", "$", "+"];
    let mut locations: Vec<Symbol> = Vec::new();

    for (index_y, row) in rows.iter().enumerate() {
        let chars: Vec<char> = row.chars().collect();
        for (index_x, char) in chars.iter().enumerate() {
            if symbols.contains(&&*char.to_string()) {
                let x = index_x as i64;
                let y = index_y as i64;

                locations.push(Symbol{
                    symbol: char.to_string(),
                    coordinates: vec![
                        vec![x - 1, y + 1],
                        vec![x - 1, y],
                        vec![x - 1, y - 1],
                        vec![x, y + 1],
                        vec![x, y - 1],
                        vec![x + 1, y + 1],
                        vec![x + 1, y],
                        vec![x + 1, y - 1]],
                })
            }
        }
    }

    locations
}