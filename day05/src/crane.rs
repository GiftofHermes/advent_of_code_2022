use regex::internal::Inst;

pub struct Crane { 
    instructions: Vec<Instruction>,
}

impl Crane { 
    pub fn new(instructions: Vec<Instruction>) -> Crane { 
        Crane { 
            instructions
        }
    }
}

#[derive(Debug)]
pub struct Instruction { 
    pub amount: u32,
    pub from: u32,
    pub to: u32
}

impl Instruction { 
    pub fn new(amount:u32, from:u32, to:u32) -> Instruction { 
        Instruction { 
            amount,
            from,
            to
        }
    }
}