use crate::crane::{Crane, Instruction};
use regex::Regex;

fn parse_instructions(instructions: &str) -> Vec<Instruction> {
    let re = Regex::new(r"(\d*) from (\d) to (\d)").unwrap();

    let instructions: Vec<Instruction> = instructions
        .lines()
        .map(|line| re.captures(line).unwrap())
        //.map(|caps| caps.get().unwrap().as_str())
        .map(|caps| {
            Instruction::new(
                caps.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                caps.get(3).unwrap().as_str().parse::<u32>().unwrap(),
            )
        })
        .collect();

    instructions
}

fn parse_cargo_line(line: &str) -> Vec<Option<char>> {
    let mut cargo: Vec<Option<char>> = vec![];
    for i in (1..line.len()).step_by(4) {
        if line.chars().nth(i).unwrap() != ' ' {
            cargo.push(Some(line.chars().nth(i).unwrap()));
        } else {
            cargo.push(None);
        }
    }
    cargo
}

fn transpose<T>(v: Vec<Vec<Option<char>>>) -> Vec<Vec<char>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| {
            v.iter()
                .map(|inner| inner[i].clone())
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect::<Vec<char>>()
        })
        .collect()
}

fn parse_cargo(cargo: &str) -> Vec<Vec<char>> {
    let cargo: Vec<Vec<Option<char>>> = cargo
        .lines()
        .filter(|line| line.chars().nth(1).unwrap() != '1')
        .rev()
        .map(|line| parse_cargo_line(line))
        .collect();

    let cargo = transpose::<Option<char>>(cargo);

    cargo
}

pub fn parse(data: &str) -> Crane {
    let data: Vec<&str> = data.split("\n\n").collect();
    let cargo = data[0];
    let instructions = data[1];

    let cargo = parse_cargo(cargo);
    let instructions = parse_instructions(instructions);

    Crane::new(cargo, instructions)
}
