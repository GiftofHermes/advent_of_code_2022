use crate::crane::{Crane, Instruction};
use regex::Regex;


fn parse_instructions(instructions: &str) -> Vec<Instruction>{ 
    let re = Regex::new(r"(\d*) from (\d) to (\d)").unwrap();

    let instructions: Vec<Instruction> = instructions.lines()
        .map(|line| re.captures(line).unwrap())
        //.map(|caps| caps.get().unwrap().as_str())
        .map(|caps| Instruction::new(caps.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                                                 caps.get(2).unwrap().as_str().parse::<u32>().unwrap(), 
                                                   caps.get(3).unwrap().as_str().parse::<u32>().unwrap())
            )
        .collect();
    
    instructions
}

fn parse_cargo(cargo: &str) -> Vec<Vec<&str>> { 

    vec![vec![]]
}

pub fn parse(data: &str) -> Crane { 
    let data: Vec<&str> = data.split("\n\n").collect();
    let cargo = data[0];
    let instructions = data[1];
    
    println!("{cargo}");
    println!("{instructions}");

    let instructions = parse_instructions(instructions);
    dbg![&instructions];

    Crane::new(instructions)
}