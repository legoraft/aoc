fn main() {
    let input_file = "\
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

    let answer_one = part_one(input_file);
    let answer_two = part_two(input_file);

    println!("Part one: {}\nPart two: {}", answer_one, answer_two);
}

#[derive(Debug)]
struct Map {
    source: i64,
    destination: i64,
    range: i64,
}

fn part_one(input: &str) -> i64 {
    let (seeds, blocks) = parse(input);

    let mut positions: Vec<i64> = Vec::new();
    
    for mut seed in seeds {
        for block in &blocks {
            for map in block {
                if (map.source..map.source + map.range).contains(&seed) {
                    seed = seed + (map.destination - map.source);
                    break;
                }
            }
        }
        
        positions.push(seed);
    }

    let &answer = positions
        .iter()
        .min()
        .expect("Couldn't find minimal value!");

    answer
}

fn part_two(input: &str) -> i64 {
    let (seeds, blocks) = parse(input);
    
    let ranges: Vec<&i64> = seeds.iter().skip(1).step_by(2).collect();
    let seeds: Vec<&i64> = seeds.iter().step_by(2).collect();
    
    let seeds: Vec<(&i64, &i64)> = seeds.into_iter().zip(ranges.into_iter()).collect();
    let blocks: Vec<Vec<Map>> = blocks.into_iter().rev().collect();

    for i in 0.. {
        let mut seed = i;
        
        for block in &blocks {
            for map in block {
                if (map.destination..map.destination + map.range).contains(&seed) {
                    seed = &seed - (map.destination - map.source);
                    break;
                }
            }
        }
        
        for (source, range) in &seeds {
            let range = *source..&(*source + *range - 1);
            if range.contains(&&seed) {
                return i;
            }
        }
    }
    
    0
}

fn parse(file: &str) -> (Vec<i64>, Vec<Vec<Map>>) {
    // Replace all unnecessary text
    let file = file
        .replace("seeds: ", "")
        .replace("seed-to-soil map:", "")
        .replace("soil-to-fertilizer map:", "")
        .replace("fertilizer-to-water map:", "")
        .replace("water-to-light map:", "")
        .replace("light-to-temperature map:", "")
        .replace("temperature-to-humidity map:", "")
        .replace("humidity-to-location map:", "");
    
    // Seeds are separated by a newline, so splitting there will give the seed line.
    let (seeds, maps) = file.split_once("\n\n").expect("Couldn't split seeds!");
    
    // The seeds get parsed, after which all the map lines are collected.
    let seeds: Vec<i64> = seeds.split_whitespace().map(|n| n.parse().expect("Couldn't parse seed!")).collect();
    let maps: Vec<Vec<&str>> = maps.split("\n\n").map(|m| m.lines().filter(|l| l != &"").collect()).collect();
    
    let mut blocks: Vec<Vec<Map>> = Vec::new();

    // For every block, we parse every map into a Map struct and push this to the blocks vec.
    for block in maps {
        let mut map = Vec::new();
        
        for line in block {
            let categories: Vec<i64> = line.split_whitespace().map(|n| n.parse().expect("Couldn't parse maps!")).collect();
            
            map.push(Map {
                source: categories[1],
                destination: categories[0],
                range: categories[2],
            });
        }
        
        blocks.push(map);
    }

    (seeds, blocks)
}
