
use crate::output_part;

pub fn main() {
    let network = include_str!("../inputs/day_08.txt");

    println!("{}", output_part(|| part_one(network), || part_two(network), "08"))
}

#[derive(Debug, Clone)]
struct Node {
    id: String,
    elements: Vec<String>,
}

pub fn part_one(network: &str) -> i64 {
    let (instructions, nodes) = parse(network);
    let instructions = instructions.replace("L", "0").replace("R", "1");
    let mut index = 0;
    let mut steps = 0;

    while nodes[index].id != "ZZZ" {
        for instruction in instructions.chars().map(|c| c.to_digit(10).unwrap()) {
            index = nodes
                .iter()
                .enumerate()
                .find(|id| id.1.id == nodes[index].elements[instruction as usize])
                .unwrap().0;

            steps += 1;

            if nodes[index].id == "ZZZ" {
                break;
            }
        }
        dbg!(&nodes[index].id);
    }

    steps
}

pub fn part_two(network: &str) -> i64 {
    0
}

fn parse(network: &str) -> (&str, Vec<Node>) {
    let (instruction, network) = network.split_once("\n\n").unwrap();
    let network: Vec<&str> = network.lines().collect();

    let mut nodes: Vec<Node> = Vec::new();

    for node in network {
        let node = node.replace(")", "");
        let (id, elements) = node.split_once(" = (").unwrap();
        let elements: Vec<String> = elements.split(", ").map(|str| str.to_string()).collect();

        nodes.push(Node{
            id: id.to_string(),
            elements,
        })
    }

    (instruction, nodes)
}