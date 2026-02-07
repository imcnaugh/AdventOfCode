#[derive(Debug)]
pub struct Computer {
    reg_a: usize,
    reg_b: usize,
    instructions: Vec<Instruction>,
    current_instruction_idx: usize,
}

impl Computer {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            reg_a: 1,
            reg_b: 0,
            instructions,
            current_instruction_idx: 0,
        }
    }

    pub fn is_done(&self) -> bool {
        self.current_instruction_idx >= self.instructions.len()
    }

    pub fn execute_next_instruction(&mut self) {
        match &self.instructions[self.current_instruction_idx] {
            Instruction::Hlf(reg) => {
                match reg.as_str() {
                    "a" => self.reg_a /= 2,
                    "b" => self.reg_b /= 2,
                    _ => {}
                }
                self.current_instruction_idx += 1;
            }
            Instruction::Tpl(reg) => {
                match reg.as_str() {
                    "a" => self.reg_a *= 3,
                    "b" => self.reg_b *= 3,
                    _ => {}
                }
                self.current_instruction_idx += 1;
            }
            Instruction::Inc(reg) => {
                match reg.as_str() {
                    "a" => self.reg_a += 1,
                    "b" => self.reg_b += 1,
                    _ => {}
                }
                self.current_instruction_idx += 1;
            }
            Instruction::Jmp(offset) => {
                let next = self.current_instruction_idx as isize + *offset;
                self.current_instruction_idx = next as usize;
            }
            Instruction::Jie(reg, offset) => {
                if self.is_reg_even(&reg) {
                    let next = self.current_instruction_idx as isize + *offset;
                    self.current_instruction_idx = next as usize;
                } else {
                    self.current_instruction_idx += 1;
                }
            }
            Instruction::Jio(reg, offset) => {
                let reg_val = match reg.as_str() {
                    "a" => self.reg_a,
                    "b" => self.reg_b,
                    _ => panic!("Unknown register: {}", reg),
                };

                if reg_val == 1 {
                    let next = self.current_instruction_idx as isize + *offset;
                    self.current_instruction_idx = next as usize;
                } else {
                    self.current_instruction_idx += 1;
                }
            }
        }
    }

    pub fn get_reg_a(&self) -> usize {
        self.reg_a
    }

    pub fn get_reg_b(&self) -> usize {
        self.reg_b
    }

    fn is_reg_even(&self, reg: &str) -> bool {
        match reg {
            "a" => self.reg_a % 2 == 0,
            "b" => self.reg_b % 2 == 0,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Hlf(String),
    Tpl(String),
    Inc(String),
    Jmp(isize),
    Jie(String, isize),
    Jio(String, isize),
}
