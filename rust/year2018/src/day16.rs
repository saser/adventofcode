use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use base::{Part, Solver};

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day16)
}

struct Day16;

impl Solver for Day16 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let (samples, program) = parse_input(input);
        match part {
            Part::One => {
                let count = samples
                    .iter()
                    .filter(|sample| sample.candidates().len() >= 3)
                    .count();
                Ok(count.to_string())
            }
            Part::Two => {
                let opcode_map = determine_opcodes(&samples);
                let final_registers = program.iter().fold(vec![0; 4], |registers, &instruction| {
                    let opcode = opcode_map[&instruction.opcode];
                    opcode.apply(instruction, &registers)
                });
                Ok(final_registers[0].to_string())
            }
        }
    }
}

fn determine_opcodes(samples: &[Sample]) -> BTreeMap<usize, Opcode> {
    let mut samples_by_opcode = BTreeMap::new();
    for sample in samples {
        let opcode_samples = samples_by_opcode
            .entry(sample.instruction.opcode)
            .or_insert_with(Vec::new);
        opcode_samples.push(sample);
    }
    let mut candidates_by_opcode = samples_by_opcode
        .iter()
        .map(|(&opcode, samples)| {
            let candidates = samples
                .iter()
                .fold(Opcode::all(), |acc, sample| &acc & &sample.candidates());
            (opcode, candidates)
        })
        .collect::<BTreeMap<usize, BTreeSet<Opcode>>>();
    let mut determined = BTreeMap::new();
    while let Some((opcode, single_candidate_set)) = candidates_by_opcode
        .iter()
        .filter(|(_opcode, candidates)| candidates.len() == 1)
        .next()
    {
        let candidate = *single_candidate_set.iter().next().unwrap();
        determined.insert(*opcode, candidate);
        for candidate_set in candidates_by_opcode.values_mut() {
            candidate_set.remove(&candidate);
        }
    }
    determined
}

pub type Registers = Vec<usize>;
pub type Program = Vec<Instruction>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Instruction {
    pub opcode: usize,
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}", self.opcode, self.a, self.b, self.c)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Sample {
    before: Registers,
    instruction: Instruction,
    after: Registers,
}

impl Sample {
    fn candidates(&self) -> BTreeSet<Opcode> {
        Opcode::all()
            .into_iter()
            .filter(|opcode| opcode.apply(self.instruction, &self.before) == self.after)
            .collect()
    }
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
        let mut new_registers = registers.clone();
        new_registers[c] = output;
        new_registers
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Opcode {
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

impl Opcode {
    fn all() -> BTreeSet<Opcode> {
        let mut set = BTreeSet::new();
        for &opcode in &[
            Opcode::Addr,
            Opcode::Addi,
            Opcode::Mulr,
            Opcode::Muli,
            Opcode::Banr,
            Opcode::Bani,
            Opcode::Borr,
            Opcode::Bori,
            Opcode::Setr,
            Opcode::Seti,
            Opcode::Gtir,
            Opcode::Gtri,
            Opcode::Gtrr,
            Opcode::Eqir,
            Opcode::Eqri,
            Opcode::Eqrr,
        ] {
            set.insert(opcode);
        }
        set
    }
    fn op(&self) -> Op {
        let (op_type, a_mode, b_mode) = match *self {
            Opcode::Addr => (OpType::Addition, Mode::Register, Mode::Register),
            Opcode::Addi => (OpType::Addition, Mode::Register, Mode::Immediate),
            Opcode::Mulr => (OpType::Multiplication, Mode::Register, Mode::Register),
            Opcode::Muli => (OpType::Multiplication, Mode::Register, Mode::Immediate),
            Opcode::Banr => (OpType::BitwiseAnd, Mode::Register, Mode::Register),
            Opcode::Bani => (OpType::BitwiseAnd, Mode::Register, Mode::Immediate),
            Opcode::Borr => (OpType::BitwiseOr, Mode::Register, Mode::Register),
            Opcode::Bori => (OpType::BitwiseOr, Mode::Register, Mode::Immediate),
            Opcode::Setr => (OpType::Assignment, Mode::Register, Mode::Ignored),
            Opcode::Seti => (OpType::Assignment, Mode::Immediate, Mode::Ignored),
            Opcode::Gtir => (OpType::GreaterThanTesting, Mode::Immediate, Mode::Register),
            Opcode::Gtri => (OpType::GreaterThanTesting, Mode::Register, Mode::Immediate),
            Opcode::Gtrr => (OpType::GreaterThanTesting, Mode::Register, Mode::Register),
            Opcode::Eqir => (OpType::EqualityTesting, Mode::Immediate, Mode::Register),
            Opcode::Eqri => (OpType::EqualityTesting, Mode::Register, Mode::Immediate),
            Opcode::Eqrr => (OpType::EqualityTesting, Mode::Register, Mode::Register),
        };
        Op {
            op_type,
            a_mode,
            b_mode,
        }
    }

    pub fn apply(&self, instruction: Instruction, registers: &Registers) -> Registers {
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
    let mut registers = vec![0; 4];
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
            let input = include_str!("../../../inputs/2018/16").trim();
            let expected = "596";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../../inputs/2018/16").trim();
            let expected = "554";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
