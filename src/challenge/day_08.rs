use anyhow::{anyhow, Context};
use std::mem::{replace, size_of};
use std::str::FromStr;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(simulate(Instruction::parse(input)?))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    simulate_and_patch(Instruction::parse(input)?).context("Could not fix infinite loop")
}

fn simulate(instructions: Vec<Instruction>) -> isize {
    let mut program = Program::new(&instructions);
    program.simulate();
    program.acc()
}

fn simulate_and_patch(mut instructions: Vec<Instruction>) -> Option<isize> {
    let mut program = Program::new(&instructions);
    program.simulate();

    let visited = program.visited().clone();

    for pos in visited.iter() {
        let instruction = &mut instructions[pos];

        let alternative = match instruction.operation() {
            Operation::Jmp if !visited.is_set(pos + 1) => {
                Instruction::new(Operation::Nop, instruction.value())
            }
            Operation::Nop if !visited.is_set(advance_pos(pos, instruction.value())) => {
                Instruction::new(Operation::Jmp, instruction.value())
            }
            _ => continue,
        };

        let original = replace(instruction, alternative);
        let mut program = Program::new(&instructions);

        if program.simulate() {
            return Some(program.acc());
        }

        instructions[pos] = original;
    }

    None
}

struct Program<'a> {
    pos: usize,
    acc: isize,
    instructions: &'a [Instruction],
    visited: BitSet,
}

impl<'a> Program<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        Program {
            pos: 0,
            acc: 0,
            instructions,
            visited: BitSet::new(instructions.len()),
        }
    }

    fn acc(&self) -> isize {
        self.acc
    }

    fn visited(&self) -> &BitSet {
        &self.visited
    }

    fn simulate(&mut self) -> bool {
        while self.pos < self.instructions.len() && self.visited.set(self.pos) {
            self.execute(&self.instructions[self.pos]);
        }

        self.pos >= self.instructions.len()
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction.operation() {
            Operation::Acc => {
                self.acc += instruction.value() as isize;
                self.pos += 1;
            }
            Operation::Jmp => {
                self.pos = advance_pos(self.pos, instruction.value());
            }
            Operation::Nop => {
                self.pos += 1;
            }
        }
    }
}

fn advance_pos(position: usize, amount: i16) -> usize {
    if amount.is_negative() {
        position - (-amount) as usize
    } else {
        position + amount as usize
    }
}

#[derive(Copy, Clone)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

struct Instruction(Operation, i16);

impl Instruction {
    fn parse(lines: &[&str]) -> anyhow::Result<Vec<Instruction>> {
        lines.iter().map(|line| line.parse()).collect()
    }

    fn new(operation: Operation, value: i16) -> Self {
        Self(operation, value)
    }

    fn operation(&self) -> Operation {
        self.0
    }

    fn value(&self) -> i16 {
        self.1
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let operation = match &string[..3] {
            "acc" => Operation::Acc,
            "jmp" => Operation::Jmp,
            "nop" => Operation::Nop,
            string => return Err(anyhow!("Unknown instruction {}", string)),
        };

        Ok(Instruction(operation, string[4..].parse()?))
    }
}

const BUCKET_SIZE: usize = size_of::<usize>() * 8;

#[derive(Clone)]
struct BitSet(Vec<usize>);

impl BitSet {
    fn new(capacity: usize) -> Self {
        BitSet(vec![0usize; (capacity - 1) / BUCKET_SIZE + 1])
    }

    fn is_set(&self, bit: usize) -> bool {
        let bucket = bit / BUCKET_SIZE;
        bucket < self.0.len() && self.0[bucket] >> (bit % BUCKET_SIZE) & 1 == 1
    }

    fn set(&mut self, bit: usize) -> bool {
        let bucket = &mut self.0[bit / BUCKET_SIZE];
        let mask = 1 << (bit % BUCKET_SIZE);
        let is_unset = *bucket & mask == 0;
        *bucket |= mask;
        is_unset
    }

    fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.0.iter().enumerate().flat_map(|(bucket_idx, bucket)| {
            let bucket = *bucket;
            let offset = bucket_idx * BUCKET_SIZE;

            (0..BUCKET_SIZE)
                .filter(move |idx| bucket >> *idx & 1 == 1)
                .map(move |idx| idx + offset)
        })
    }
}
