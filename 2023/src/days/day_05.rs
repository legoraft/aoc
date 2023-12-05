pub fn main() {
    let almanac = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    part_one(almanac);
}

#[derive(Debug)]
struct Map {
    destination : i64,
    source: i64,
    length: i64,
}

pub fn part_one(almanac: &str) {
    let (maps, mut seeds) = parse(almanac);

    for map in maps {
        let destination = map[0];
        let source = map[1];
        let length = map[2];

        let mut locations: Vec<i64> = Vec::new();

        for seed in &seeds {
            let range = source..source + length;
            if range.contains(seed) {
                locations.push(seed - source + destination);
            } else {
                locations.push(*seed);
            }
        }
        seeds = locations.clone();
    }
    dbg!(&seeds);
}

fn parse(almanac: &str) -> (Vec<Vec<i64>>, Vec<i64>) {
    let sets: Vec<&str> = almanac.split("\n\n").collect();

    let (_, seeds) = sets[0].split_once(": ").unwrap();
    let seeds: Vec<i64> = seeds.split_whitespace().map(|seed| seed.parse().unwrap()).collect();

    let mut maps: Vec<Vec<i64>> = Vec::new();

    for set in &sets[1..] {
        let lines: Vec<&str> = set.lines().collect();

        for line in &lines[1..] {
            let ranges: Vec<i64> = line.split_whitespace().map(|num| num.parse().unwrap()).collect();
            maps.push(ranges);
        }
    }

    (maps, seeds)
}