use std::vec;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Map {
    source: i64,
    destination: i64,
    range: i64,
}

fn part_one(input: &str) -> i64 {
    let (seeds, maps) = parse(input);

    println!("seeds: {:?}\nmaps: {:?}", seeds, maps);

    0
}

fn parse(file: &str) -> (Vec<i64>, Vec<Vec<Map>>) {
    let (seeds, maps) = file.split_once("\n\n").expect("Couldn't split seeds!");

    let lines: Vec<&str> = maps.split("\n\n")
        .map(|l| {
            let (_, maps) = l
                .split_once(":")
                .expect("Couldn't split maps!"); maps })
        .collect();

    let seeds: Vec<i64> = seeds[7..]
        .split_whitespace()
        .map(|n| n.parse::<i64>().expect("Couldn't parse seed!"))
        .collect();

    let mut maps: Vec<Vec<Map>> = Vec::new();

    for line in lines {
        let mut map: Vec<Map> = Vec::new();

        for locations in line.trim().lines() {
            let locations: Vec<i64> = locations
                .split_whitespace()
                .map(|n| n.parse::<i64>().expect("Couldn't parse map!"))
                .collect();

            map.push(Map{
                source: locations[1],
                destination: locations[0],
                range: locations[2],
            })
        }

        maps.push(map);
    }

    (seeds, maps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input_file: &str = "\
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
56 93 4";

        let answer: i64 = 35;

        assert_eq!(answer, part_one(input_file));
    }
}
