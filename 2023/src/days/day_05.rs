pub fn main() {
    let almanac = include_str!("../inputs/day_05.txt");
    let answer: Vec<i64> = vec![part_one(almanac), 0];

    println!("-- Day Five --\nPart 1: {:?}\nPart 2: {:?}\n", answer[0], answer[1]);
}

#[derive(Debug)]
struct Block {
    maps: Vec<Vec<i64>>,
}

pub fn part_one(almanac: &str) -> i64 {
    let (blocks, seeds) = parse(almanac);
    let mut positions: Vec<i64> = Vec::new();

    for mut seed in seeds {
        for block in &blocks {
            for map in &block.maps {
                let source = map[1];
                let destination = map[0];
                let length = map[2];

                let range = source..source + length;
                let delta = destination - source;

                if range.contains(&seed) {
                    seed = seed + delta;
                    break;
                };
            }
        }
        positions.push(seed);
    }
    let minimum_position = positions.iter().min().unwrap();
    *minimum_position
}

fn parse(almanac: &str) -> (Vec<Block>, Vec<i64>) {
    let sets: Vec<&str> = almanac.split("\n\n").collect();

    let (_, seeds) = sets[0].split_once(": ").unwrap();
    let seeds: Vec<i64> = seeds.split_whitespace().map(|seed| seed.parse().unwrap()).collect();

    let mut blocks: Vec<Block> = Vec::new();

    for set in &sets[1..] {
        let lines: Vec<&str> = set.lines().collect();
        let mut maps: Vec<Vec<i64>> = Vec::new();

        for line in &lines[1..] {
            let ranges: Vec<i64> = line.split_whitespace().map(|num| num.parse().unwrap()).collect();
            maps.push(ranges);
        }

        blocks.push(Block {
            maps,
        });
    }

    (blocks, seeds)
}