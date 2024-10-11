fn main() {
    let input_file = include_str!("../../inputs/day_06.txt");

    let answer_one = part_one(input_file);
    let answer_two = part_two(input_file);

    println!("Part one: {}\nPart two: {}", answer_one, answer_two);
}

fn part_one(input: &str) -> i64 {
    let races = parse(input);
    let mut answer = 1;
    
    for (time, record) in races {
        let mut possibility = 0;
        
        for press in 0..time {
            let distance = (time - press) * press;
            
            if distance > record {
                possibility += 1;
            }
        }

        answer *= possibility;
    }
    
    answer
}

fn part_two(input: &str) -> i64 {
    let races = parse(input);
    
    let mut time: String = String::new();
    let mut record: String = String::new();
    
    for (times, records) in races {
        time.push_str(&times.to_string());
        record.push_str(&records.to_string());
    }
    
    dbg!(time);
    dbg!(record)
    
    0
}

fn parse(file: &str) -> Vec<(i64, i64)> {
    let lines: Vec<&str> = file.lines().collect();
    let (_, times) = lines[0].split_once(": ").unwrap();
    let (_, records) = lines[1].split_once(": ").unwrap();
    
    let times: Vec<i64> = times.split_whitespace().map(|num| num.parse().expect("Couldn't parse times!")).collect();
    let records: Vec<i64> = records.split_whitespace().map(|num| num.parse().expect("Couldn't parse records!")).collect();
    
    let mut races: Vec<(i64, i64)> = vec![];
    
    for (index, time) in times.iter().enumerate() {
        races.push((*time, records[index]));
    }
    
    races
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part_one() {
        let input_file: &str = "\
Time:      7  15   30
Distance:  9  40  200";
        
        let answer: i64 = 288;

        assert_eq!(answer, part_one(input_file));
    }
}