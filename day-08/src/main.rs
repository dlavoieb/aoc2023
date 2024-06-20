use std::collections::HashMap;
use std::str::FromStr;
use utils::read_file;

fn main() {
    let lines = read_file("day-08/src/input.txt");

    let (instructions, nodes) = parse_file_p1(&lines);
    let steps = run_instructions_p1(&instructions, nodes);
    println!("Steps needed: {}", steps);

    let (instructions, nodes) = parse_file_p2(&lines);
    let steps = run_instructions_p2(&instructions, nodes);
    println!("Steps needed: {}", steps);
    assert_eq!(steps, 18215611419223);
}

fn parse_file_p1(lines: &Vec<String>) -> (Vec<Instruction>, HashMap<String, (String, String)>) {
    let instructions = lines[0].chars();
    let instructions: Vec<_> = instructions.into_iter().map(|c| Instruction::try_from(c).unwrap()).collect();

    let nodes = lines[2..lines.len()].iter().map(|line| (line[0..3].to_string(), (line[7..10].to_string(), line[12..15].to_string()))).collect::<HashMap<_, _>>();
    (instructions, nodes)
}

fn parse_file_p2(lines: &Vec<String>) -> (Vec<Instruction>, HashMap<Node, (Node, Node)>) {
    let instructions = lines[0].chars();
    let instructions: Vec<_> = instructions.into_iter().map(|c| Instruction::try_from(c).unwrap()).collect();

    let nodes = lines[2..lines.len()].iter().map(|line| (Node::from_str(&line[0..3]).unwrap(), (Node::from_str(&line[7..10]).unwrap(), Node::from_str(&line[12..15]).unwrap()))).collect::<HashMap<_, _>>();
    (instructions, nodes)
}

fn run_instructions_p1(instructions: &Vec<Instruction>, nodes: HashMap<String, (String, String)>) -> i32 {
    let mut steps = 0;
    let mut current_node = String::from("AAA");
    loop {
        for instruction in instructions {
            steps += 1;
            let (left, right) = &nodes.get(&current_node).unwrap();
            current_node =
                match instruction {
                    Instruction::Left => { left.to_string() }
                    Instruction::Right => { right.to_string() }
                };
            if current_node.starts_with("ZZZ") {
                return steps;
            }
        }
    }
}

fn run_instructions_p2(instructions: &Vec<Instruction>,  nodes: HashMap<Node, (Node, Node)>) -> i32{
    let mut steps = 0;

    let mut starting_nodes: Vec<_> = nodes.keys().filter(|c|c.is_starting_position()).collect();
    loop {
        for instruction in instructions {
            steps += 1;
            let mut new_nodes = Vec::new();
            for starting_node in starting_nodes {
                let (left, right) = &nodes.get(starting_node).unwrap();
                new_nodes.push(
                    match instruction {
                        Instruction::Left => { left }
                        Instruction::Right => { right }
                    });
            }
            starting_nodes = new_nodes;
            if starting_nodes.iter().filter(|node|!node.is_end_position()).count() == 0 {
                return steps;
            }
        }
    }
}
#[derive(Hash, Eq, PartialEq, Debug)]
struct Node {
    letters: [char;3]
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            return Err(())
        }
        Ok(Node{letters:s.chars().collect::<Vec<_>>().try_into().unwrap()})
    }
}

impl Node {
    fn is_starting_position(&self) -> bool {
        self.letters[2] == 'A'
    }
    fn is_end_position(&self) -> bool {
        self.letters[2] == 'Z'
    }
}



enum Instruction {
    Left,
    Right,
}

impl TryFrom<char> for Instruction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            _ => Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT:&str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_simple() {
        let lines:Vec<_> = INPUT.split('\n').map(|l| l.to_string()).collect();
        let (instructions, nodes) = parse_file_p1(&lines);

        assert_eq!(instructions.len(), 2);
        assert_eq!(nodes.len(), 7);
    }

    #[test]
    fn test_run() {
        let lines:Vec<_> = INPUT.split('\n').map(|l| l.to_string()).collect();
        let (instructions, nodes) = parse_file_p1(&lines);

        let steps = run_instructions_p1(&instructions, nodes);
        assert_eq!(steps, 2);
    }

    #[test]
    fn starting_nodes() {
        let lines:Vec<_> = INPUT.split('\n').map(|l| l.to_string()).collect();
        let (instructions, nodes) = parse_file_p2(&lines);
        let starting_nodes: Vec<_> = nodes.keys().filter(|c|c.is_starting_position()).collect();
        assert_eq!(starting_nodes.len(), 1);
    }
    #[test]
    fn p2() {
        let lines:Vec<_> = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)".split('\n').map(|l| l.to_string()).collect();
        let (instructions, nodes) = parse_file_p2(&lines);
        let steps = run_instructions_p2(&instructions, nodes);
        assert_eq!(steps, 6);

    }
}