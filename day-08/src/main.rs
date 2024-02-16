use std::collections::HashMap;
use std::str::FromStr;
use utils::read_file;

fn main() {
    let lines = read_file("day-08/src/input.txt");

    let (instructions, nodes) = parse_file_p1(lines);
    let steps = run_instructions_p1(&instructions, nodes);
    println!("Steps needed: {}", steps);
}

fn parse_file_p1(lines: Vec<String>) -> (Vec<Instruction>, HashMap<String, (String, String)>) {
    let instructions = lines[0].chars();
    let instructions: Vec<_> = instructions.into_iter().map(|c| Instruction::try_from(c).unwrap()).collect();

    let nodes = lines[2..lines.len()].iter().map(|line| (line[0..3].to_string(), (line[7..10].to_string(), line[12..15].to_string()))).collect::<HashMap<_, _>>();
    (instructions, nodes)
}

fn parse_file_p2(lines: Vec<String>) -> (Vec<Instruction>, HashMap<Node, (Node, Node)>) {
    let instructions = lines[0].chars();
    let instructions: Vec<_> = instructions.into_iter().map(|c| Instruction::try_from(c).unwrap()).collect();

    let nodes = lines[2..lines.len()].iter().map(|line| (from_str(&line[0..3]).unwrap(), (from_str(&line[7..10]).unwrap(), from_str(&line[12..15]).unwrap()))).collect::<HashMap<_, _>>();
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

type Node = [char;3];

fn from_str(s: &str) -> Option<Node> {
    if s.len() != 3 {
        return None
    }
    Some(s.chars().collect::<Vec<_>>().try_into().unwrap() )
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
        let (instructions, nodes) = parse_file_p1(lines);

        assert_eq!(instructions.len(), 2);
        assert_eq!(nodes.len(), 7);
    }

    #[test]
    fn test_run() {
        let lines:Vec<_> = INPUT.split('\n').map(|l| l.to_string()).collect();
        let (instructions, nodes) = parse_file_p1(lines);

        let steps = run_instructions_p1(&instructions, nodes);
        assert_eq!(steps, 2);
    }
}