use std::fs;
use regex::Regex;

struct ThreeBitMachine {
    instruction_index: usize,
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    output: Vec<u64>,
    program: Vec<u8>,
}

impl ThreeBitMachine {
    pub fn new(a: u64, b: u64, c: u64, program: Vec<u8>) -> Self {
        Self {
            instruction_index: 0,
            reg_a: a,
            reg_b: b,
            reg_c: c,
            output: vec![],
            program,
        }
    }

    pub fn run_program(&mut self) {
        loop {
            if self.instruction_index >= self.program.len() {
                break;
            }

            let op_code = self.program[self.instruction_index];
            self.instruction_index += 1;
            self.run_instruction(op_code);
        }
    }

    pub fn get_output(&self) -> &Vec<u64> {
        &self.output
    }

    pub fn get_combo_operand_value(&self, operand: u8) -> u64 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Invalid operand value"),
        }
    }

    pub fn run_instruction(&mut self, op_code: u8) {
        let operand = self.program[self.instruction_index];
        self.instruction_index += 1;
        match op_code {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => panic!("Invalid instruction value"),
        }
    }

    fn adv(&mut self, operand: u8) {
        let operand = self.get_combo_operand_value(operand);
        let operand = 2_u64.pow(operand as u32);
        let result = self.reg_a / operand;
        self.reg_a = result;
    }

    fn bxl(&mut self, operand: u8) {
        let result = self.reg_b ^ operand as u64;
        self.reg_b = result;
    }

    fn bst(&mut self, operand: u8) {
        let operand = self.get_combo_operand_value(operand);
        let result = operand % 8;
        let result = 0b00000111 & result as u8;
        self.reg_b = result as u64
    }

    fn jnz(&mut self, operand: u8) {
        if self.reg_a != 0 {
            self.instruction_index = operand as usize;
        }
    }

    fn bxc(&mut self) {
        let result = self.reg_b ^ self.reg_c;
        self.reg_b = result;
    }

    fn out(&mut self, operand: u8) {
        let operand = self.get_combo_operand_value(operand);
        let result = operand % 8;
        self.output.push(result);
    }

    fn bdv(&mut self, operand: u8) {
        let operand = self.get_combo_operand_value(operand);
        let operand = 2_u64.pow(operand as u32);
        let result = self.reg_a / operand;
        self.reg_b = result;
    }

    fn cdv(&mut self, operand: u8) {
        let operand = self.get_combo_operand_value(operand);
        let operand = 2_u64.pow(operand as u32);
        let result = self.reg_a / operand;
        self.reg_c = result;
    }
}



fn main() {
    let input = read_file("resource/input.txt");
    let mac = parse_input(input);
    let result = part_1(mac);
    println!("{result}");
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn parse_input(input: String) -> ThreeBitMachine {
    let mut lines = input.lines();
    let a_line = lines.next().unwrap();
    let b_line = lines.next().unwrap();
    let c_line = lines.next().unwrap();
    lines.next();
    let program_line = lines.next().unwrap();

    let reg_rx = Regex::new(r"(?<reg_value>\d+)$").unwrap();
    let reg_a = reg_rx.captures(a_line).unwrap()["reg_value"].parse::<u64>().unwrap();
    let reg_b = reg_rx.captures(b_line).unwrap()["reg_value"].parse::<u64>().unwrap();
    let reg_c = reg_rx.captures(c_line).unwrap()["reg_value"].parse::<u64>().unwrap();

    let prog_rx = Regex::new(r"^.*: (?<prog>.*)").unwrap();
    let program_as_str = &prog_rx.captures(program_line).unwrap()["prog"];
    let program: Vec<u8> = program_as_str.split(",").map(|i| i.parse::<u8>().unwrap()).collect();

    ThreeBitMachine::new(reg_a, reg_b, reg_c, program)
}

fn part_1(mut input: ThreeBitMachine) -> String {
    input.run_program();

    let output = input.output.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",");
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_file("resource/test.txt");
        let mac = parse_input(input);
        let result = part_1(mac);
        assert_eq!("4,6,3,5,6,3,5,2,1,0", result);
    }

    #[test]
    fn u32_to_u8() {
        let test: u32 = 436;
        let idk = test as u8;
        let idk = 0b00000111 & idk;
        assert_eq!(0, idk);
    }
}
