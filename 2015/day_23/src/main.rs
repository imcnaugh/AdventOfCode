use crate::computer::{Computer, Instruction};

mod computer;

fn main() {
    let input = include_str!("../resources/input.txt");
    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    let instructions = parse_input(input);
    let mut computer = Computer::new(instructions);
    while !computer.is_done() {
        computer.execute_next_instruction();
    }
    computer.get_reg_b()
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let inst_as_str = &line[0..3];
            match inst_as_str {
                "hlf" => {
                    let reg = &line[4..];
                    Instruction::Hlf(reg.to_string())
                }
                "tpl" => {
                    let reg = &line[4..];
                    Instruction::Tpl(reg.to_string())
                }
                "inc" => {
                    let reg = &line[4..];
                    Instruction::Inc(reg.to_string())
                }
                "jmp" => {
                    let offset = &line[4..].parse::<isize>().unwrap();
                    Instruction::Jmp(*offset)
                }
                "jie" => {
                    let reg = &line[4..5];
                    let offset = &line[7..].parse().unwrap();
                    Instruction::Jie(reg.to_string(), *offset)
                }
                "jio" => {
                    let reg = &line[4..5];
                    let offset = &line[7..].parse().unwrap();
                    Instruction::Jio(reg.to_string(), *offset)
                }
                _ => panic!("Unknown instruction: {}", inst_as_str),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let line = r"inc b
tpl b
hlf b
inc a
jio a, +2
tpl a
inc a";
        let instructions = parse_input(line);
        let mut computer = Computer::new(instructions);
        while !computer.is_done() {
            computer.execute_next_instruction();
            println!("{:?}", computer);
        }
        assert_eq!(computer.get_reg_a(), 2);
        assert_eq!(computer.get_reg_b(), 1);
    }
}
