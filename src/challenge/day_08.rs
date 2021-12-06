use anyhow::anyhow;
use std::mem::size_of;
use std::str::FromStr;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let instructions = input
        .iter()
        .map(|line| line.parse())
        .collect::<Result<Vec<Instruction>, _>>()?;

    let mut visited = BitSet::new(instructions.len());

    let mut acc = 0isize;
    let mut pos = 0usize;

    while visited.set(pos) {
        match instructions[pos] {
            Instruction::Acc(value) => {
                acc += value as isize;
                pos += 1;
            }
            Instruction::Jmp(value) => {
                if value.is_negative() {
                    pos -= -value as usize;
                } else {
                    pos += value as usize;
                }
            }
            Instruction::Nop => {
                pos += 1;
            }
        }
    }

    Ok(acc)
}

enum Instruction {
    Acc(i16),
    Jmp(i16),
    Nop,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match &string[..3] {
            "acc" => Ok(Instruction::Acc(string[4..].parse()?)),
            "jmp" => Ok(Instruction::Jmp(string[4..].parse()?)),
            "nop" => Ok(Instruction::Nop),
            string => Err(anyhow!("Unknown instruction {}", string)),
        }
    }
}

struct BitSet(Vec<usize>);

impl BitSet {
    fn new(capacity: usize) -> Self {
        BitSet(vec![0usize; (capacity - 1) / size_of::<usize>() + 1])
    }

    fn set(&mut self, bit: usize) -> bool {
        let bucket = &mut self.0[bit / size_of::<usize>()];
        let mask = 1 << (bit % size_of::<usize>());
        let is_unset = *bucket & mask == 0;
        *bucket |= mask;
        is_unset
    }
}
