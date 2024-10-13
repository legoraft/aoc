use std::collections::HashMap;

fn main() {
    let input_file = include_str!("../../inputs/day_08.txt");

    let answer_one = part_one(input_file);
    let answer_two = part_two(input_file);

    println!("Part one: {}\nPart two: {}", answer_one, answer_two);
}

fn part_one(file: &str) -> i64 {
    let (instructions, nodes) = parse(file);
    let mut steps = 0;
    let mut node: &str = "AAA";
    
    while node != "ZZZ" {
        for instruction in &instructions {
            let elements = &nodes[node];
            node = &elements[*instruction as usize];
            steps += 1;
        }
    }
    
    steps
}

fn part_two(file: &str) -> i64 {
    let (instructions, nodes) = parse(file);
    
    let start_nodes: Vec<&str> = nodes.keys().filter(|k| k.ends_with('A')).cloned().collect();
    let mut ghost_steps: Vec<i64> = Vec::new();
    
    for mut node in start_nodes {
        let mut steps = 0;
        
        while !node.ends_with('Z') {
            for instruction in &instructions {
                let elements = &nodes[node];
                node = &elements[*instruction as usize];
                steps += 1;
            }
        }
        
        ghost_steps.push(steps);
    }
    
    let result = lcm(&ghost_steps);
    
    result
}

fn lcm(nums: &[i64]) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
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
    
    #[test]
    fn test_part_one() {
        let input: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        
        let answer: i64 = 6;

        assert_eq!(answer, part_one(input));
    }
    
    #[test]
    fn test_part_two() {
        let input: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        
        let answer: i64 = 6;

        assert_eq!(answer, part_two(input));
    }
}