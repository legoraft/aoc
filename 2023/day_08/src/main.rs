use std::collections::HashMap;

fn main() {
    let input_file = include_str!("../../inputs/day_08.txt");

    let answer_one = part_one(input_file);
//    let answer_two = part_two(input_file);

    println!("Part one: {}\nPart two: ", answer_one, /* answer_two */);
}

#[derive(Debug)]
struct Node {
    id: String,
    map: [String; 2],
}

fn part_one(file: &str) -> i64 {
    let (instructions, nodes) = parse(file);
    let mut steps = 0;
    let mut node: &str = "AAA";
    
    for instruction in instructions {
        if node == "ZZZ" {
            break;
        }
        
        let elements = nodes[node];
    }
    
    0
}

fn parse(file: &str) -> (Vec<i64>, HashMap<&str, [String; 2]>) {
    let (instructions, elements) = file.split_once("\n\n").expect("Couldn't split off instructions!");
    
    let instructions: Vec<i64> = instructions.replace('L', "0").replace('R', "1").chars().map(|c| c.to_digit(10).expect("Couldn't parse character!") as i64).collect();
    
    let mut nodes: HashMap<&str, [String; 2]> = HashMap::new();
    
    for element in elements.lines() {
        let (id, map) = element.split_once(" = ").expect("Couldn't split id!");
        
        let map = map.replace(['(', ')'], "");
        let map: (&str, &str) = map.split_once(", ").expect("Couldn't split map!");
        let map: [String; 2] = [map.0.to_string(), map.1.to_string()];
        
        nodes.insert(id, map);
    }
    
    (instructions, nodes)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const INPUT: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    
    #[test]
    fn test_part_one() {
        let answer: i64 = 6;

        assert_eq!(answer, part_one(INPUT));
    }
}