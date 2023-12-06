use crate::output_part;

pub fn main() {
    let race = include_str!("../inputs/day_06.txt");

    println!("{}", output_part(|| part_one(race), || part_two(race), "06"))
}

pub fn part_one(race: &str) -> i64 {
    let (times, records) = parse(race);

    let times: Vec<i64> = times.iter().map(|num| num.parse().unwrap()).collect();
    let records: Vec<i64> = records.iter().map(|num| num.parse().unwrap()).collect();

    let mut game_multiplication: i64 = 1;

    for (index, record) in records.iter().enumerate() {
        for time in 0..=times[index] {
            let sum = time * (times[index] - time);
            if &sum > record {
                let win_ways = (time..=(times[index] - time)).count();
                game_multiplication = win_ways as i64 * game_multiplication;
                break;
            }
        }
    }
    game_multiplication
}

pub fn part_two(race: &str) -> i64 {
    let (times, records) = parse(race);

    let time: i64 = times.join("").parse().unwrap();
    let record: i64 = records.join("").parse().unwrap();

    let mut win_ways = 0;

    for t in 0..=time {
        let sum = t * (time - t);
        if sum > record {
            win_ways += (t..=(time - t)).count();
            break;
        }
    }
    win_ways as i64
}

fn parse(race: &str) -> (Vec<String>, Vec<String>) {
    let lines: Vec<&str> = race.lines().collect();
    let (_, times) = lines[0].split_once(": ").unwrap();
    let (_, records) = lines[1].split_once(": ").unwrap();

    let times: Vec<&str> = times.split_whitespace().collect();
    let records: Vec<&str> = records.split_whitespace().collect();

    let times: Vec<String> = times.iter().map(|s| s.to_string()).collect();
    let records: Vec<String> = records.iter().map(|s| s.to_string()).collect();

    (times, records)
}