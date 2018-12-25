use base::{Part, Solver};

use crate::day16::{Instruction, Opcode, Registers};

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day19)
}

struct Day19;

type OpcodeProgram = Vec<OpcodeInstruction>;

impl Solver for Day19 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let (ip_register, program) = parse_input(input);
        match part {
            Part::One => Err("day 19 part 1 not yet implemented".to_string()),
            Part::Two => Err("day 19 part 2 not yet implemented".to_string()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct OpcodeInstruction {
    opcode: Opcode,
    instruction: Instruction,
}

impl OpcodeInstruction {
    fn apply(&self, registers: &Registers) -> Registers {
        self.opcode.apply(self.instruction, registers)
    }
}

fn parse_input(input: &str) -> (usize, OpcodeProgram) {
    let mut lines = input.lines();
    let ip_line = lines.next().unwrap();
    let ip_register = parse_ip_line(ip_line);
    let program = lines.map(parse_instruction_line).collect();
    (ip_register, program)
}

fn parse_ip_line(line: &str) -> usize {
    let parts = line.split(' ').collect::<Vec<&str>>();
    parts[1].parse().unwrap()
}

fn parse_instruction_line(line: &str) -> OpcodeInstruction {
    let parts = line.split(' ').collect::<Vec<&str>>();
    let opcode = match parts[0] {
        "addr" => Opcode::Addr,
        "addi" => Opcode::Addi,
        "mulr" => Opcode::Mulr,
        "muli" => Opcode::Muli,
        "banr" => Opcode::Banr,
        "bani" => Opcode::Bani,
        "borr" => Opcode::Borr,
        "bori" => Opcode::Bori,
        "setr" => Opcode::Setr,
        "seti" => Opcode::Seti,
        "gtir" => Opcode::Gtir,
        "gtri" => Opcode::Gtri,
        "gtrr" => Opcode::Gtrr,
        "eqir" => Opcode::Eqir,
        "eqri" => Opcode::Eqri,
        "eqrr" => Opcode::Eqrr,
        _ => unreachable!(),
    };
    let a = parts[1].parse::<usize>().unwrap();
    let b = parts[2].parse::<usize>().unwrap();
    let c = parts[3].parse::<usize>().unwrap();
    let instruction = Instruction { opcode: 0, a, b, c };
    OpcodeInstruction {
        opcode,
        instruction,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/19").trim();
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "put some input here";
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/19").trim();
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "put some input here";
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
