use std::fmt;

use base::{Part, Solver};

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day16)
}

struct Day16;

impl Solver for Day16 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let (samples, program) = parse_input(input);
        for sample in samples {
            println!("{}", sample);
            println!();
        }
        println!();
        for instruction in program {
            println!("{}", instruction);
        }
        println!();
        match part {
            Part::One => Err("day 16 part 1 not yet implemented".to_string()),
            Part::Two => Err("day 16 part 2 not yet implemented".to_string()),
        }
    }
}

type Registers = [usize; 4];
type Program = Vec<Instruction>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Instruction {
    opcode: usize,
    a: usize,
    b: usize,
    c: usize,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}", self.opcode, self.a, self.b, self.c)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Sample {
    before: Registers,
    instruction: Instruction,
    after: Registers,
}

impl fmt::Display for Sample {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Before: {:?}", self.before)?;
        writeln!(f, "{}", self.instruction)?;
        write!(f, "After: {:?}", self.after)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Mode {
    Ignored,
    Immediate,
    Register,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum OpType {
    Addition,
    Multiplication,
    BitwiseAnd,
    BitwiseOr,
    Assignment,
    GreaterThanTesting,
    EqualityTesting,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Op {
    op_type: OpType,
    a_mode: Mode,
    b_mode: Mode,
}

impl Op {
    fn apply(&self, instruction: Instruction, registers: &Registers) -> Registers {
        let a = match self.a_mode {
            Mode::Ignored => None,
            Mode::Immediate => Some(instruction.a),
            Mode::Register => Some(registers[instruction.a]),
        };

        let b = match self.b_mode {
            Mode::Ignored => None,
            Mode::Immediate => Some(instruction.b),
            Mode::Register => Some(registers[instruction.b]),
        };
        let c = instruction.c;
        let output = match self.op_type {
            OpType::Addition => a.unwrap() + b.unwrap(),
            OpType::Multiplication => a.unwrap() * b.unwrap(),
            OpType::BitwiseAnd => a.unwrap() & b.unwrap(),
            OpType::BitwiseOr => a.unwrap() | b.unwrap(),
            OpType::Assignment => a.unwrap(),
            OpType::GreaterThanTesting => usize::from(a.unwrap() > b.unwrap()),
            OpType::EqualityTesting => usize::from(a.unwrap() == b.unwrap()),
        };
        let mut new_registers = *registers;
        new_registers[c] = output;
        new_registers
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum OpCode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl OpCode {
    fn op(&self) -> Op {
        let (op_type, a_mode, b_mode) = match *self {
            OpCode::Addr => (OpType::Addition, Mode::Register, Mode::Register),
            OpCode::Addi => (OpType::Addition, Mode::Register, Mode::Immediate),
            OpCode::Mulr => (OpType::Multiplication, Mode::Register, Mode::Register),
            OpCode::Muli => (OpType::Multiplication, Mode::Register, Mode::Immediate),
            OpCode::Banr => (OpType::BitwiseAnd, Mode::Register, Mode::Register),
            OpCode::Bani => (OpType::BitwiseAnd, Mode::Register, Mode::Immediate),
            OpCode::Borr => (OpType::BitwiseOr, Mode::Register, Mode::Register),
            OpCode::Bori => (OpType::BitwiseOr, Mode::Register, Mode::Immediate),
            OpCode::Setr => (OpType::Assignment, Mode::Register, Mode::Ignored),
            OpCode::Seti => (OpType::Assignment, Mode::Immediate, Mode::Ignored),
            OpCode::Gtir => (OpType::GreaterThanTesting, Mode::Immediate, Mode::Register),
            OpCode::Gtri => (OpType::GreaterThanTesting, Mode::Register, Mode::Immediate),
            OpCode::Gtrr => (OpType::GreaterThanTesting, Mode::Register, Mode::Register),
            OpCode::Eqir => (OpType::EqualityTesting, Mode::Immediate, Mode::Register),
            OpCode::Eqri => (OpType::EqualityTesting, Mode::Register, Mode::Immediate),
            OpCode::Eqrr => (OpType::EqualityTesting, Mode::Register, Mode::Register),
        };
        Op {
            op_type,
            a_mode,
            b_mode,
        }
    }

    fn apply(&self, instruction: Instruction, registers: &Registers) -> Registers {
        self.op().apply(instruction, registers)
    }
}

fn parse_input(input: &str) -> (Vec<Sample>, Program) {
    let mut lines = input.lines().peekable();
    let mut samples = Vec::new();
    let mut next_line = lines.peek().unwrap();
    while *next_line != "" {
        let before_line = lines.next().unwrap();
        let before = parse_registers(before_line);

        let instruction_line = lines.next().unwrap();
        let instruction = parse_instruction(instruction_line);

        let after_line = lines.next().unwrap();
        let after = parse_registers(after_line);

        let sample = Sample {
            before,
            instruction,
            after,
        };
        samples.push(sample);

        // Advance past the expected empty line.
        lines.next();

        // Peek what the next line is. If it is an empty line, we have reached the end of all the
        // samples; if it is not empty, we assume that is the start of a new sample.
        next_line = lines.peek().unwrap();
    }
    let program = lines
        .skip_while(|&line| line == "")
        .map(parse_instruction)
        .collect();
    (samples, program)
}

fn parse_registers(line: &str) -> Registers {
    let is_bracket = |c| c == '[' || c == ']';
    let stripped = line
        .trim_matches(|c| !is_bracket(c))
        .trim_matches(is_bracket);
    let mut registers = [0; 4];
    for (i, part) in stripped.split(", ").enumerate() {
        let register = part.parse::<usize>().unwrap();
        registers[i] = register;
    }
    registers
}

fn parse_instruction(line: &str) -> Instruction {
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    let opcode = parts[0].parse::<usize>().unwrap();
    let a = parts[1].parse::<usize>().unwrap();
    let b = parts[2].parse::<usize>().unwrap();
    let c = parts[3].parse::<usize>().unwrap();
    Instruction { opcode, a, b, c }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/16").trim();
            let expected = "expected output";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/16").trim();
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
