use std::collections::HashMap;
use utils::read_file;

fn main() {
    let lines = read_file("day-08/src/input.txt");

    let (instructions, nodes) = parse_file(lines);
    let steps = run_instructions(&instructions, nodes);
    println!("Steps needed: {}", steps);
}

fn parse_file<'a>(lines: Vec<String>) -> (Vec<Instruction>, HashMap<String, (String, String)>) {
    let instructions = lines[0].chars();
    let instructions: Vec<_> = instructions.into_iter().map(|c| Instruction::try_from(c).unwrap()).collect();

    let nodes = lines[2..lines.len()].iter().map(|line| (line[0..3].to_string(), (line[7..10].to_string(), line[12..15].to_string()))).collect::<HashMap<_, _>>();
    (instructions, nodes)
}

fn run_instructions(instructions: &Vec<Instruction>, nodes: HashMap<String, (String, String)>) -> i32 {
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
        let (instructions, nodes) = parse_file(lines);

        assert_eq!(instructions.len(), 2);
        assert_eq!(nodes.len(), 7);
    }

    #[test]
    fn test_run() {
        let lines:Vec<_> = INPUT.split('\n').map(|l| l.to_string()).collect();
        let (instructions, nodes) = parse_file(lines);

        let steps = run_instructions(&instructions, nodes);
        assert_eq!(steps, 2);
    }
}