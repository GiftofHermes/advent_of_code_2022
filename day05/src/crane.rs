use std::mem;

pub struct Crane {
    cargo: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

impl Crane {
    pub fn new(cargo: Vec<Vec<char>>, instructions: Vec<Instruction>) -> Crane {
        Crane {
            cargo,
            instructions,
        }
    }

    fn apply_move(&mut self, instruction: Instruction) {
        for _i in 0..instruction.amount {
            let crate_to_move = self.cargo[(instruction.from - 1) as usize].pop().unwrap();
            self.cargo[(instruction.to - 1) as usize].push(crate_to_move);
        }
    }

    pub fn apply_instructions(&mut self) {
        let instructions = mem::replace(&mut self.instructions, Vec::new());
        for instruction in instructions {
            self.apply_move(instruction);
        }
    }

    fn apply_move_upgraded(&mut self, instruction: Instruction) {
        let mut crates_to_move = vec![];
        for _i in 0..instruction.amount {
            let crate_to_move = self.cargo[(instruction.from - 1) as usize].pop().unwrap();
            crates_to_move.push(crate_to_move)
        }
        crates_to_move.reverse();
        self.cargo[(instruction.to - 1) as usize].append(&mut crates_to_move);
    }

    pub fn apply_instructions_upgraded(&mut self) {
        let instructions = mem::replace(&mut self.instructions, Vec::new());
        for instruction in instructions {
            self.apply_move_upgraded(instruction);
        }
    }

    pub fn highest_crates(&self) -> String {
        let top_letters: String = self.cargo.iter().map(|line| line.last().unwrap()).collect();

        top_letters
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub amount: u32,
    pub from: u32,
    pub to: u32,
}

impl Instruction {
    pub fn new(amount: u32, from: u32, to: u32) -> Instruction {
        Instruction { amount, from, to }
    }
}
