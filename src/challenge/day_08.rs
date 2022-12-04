use anyhow::anyhow;
use std::str::FromStr;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let instructions = Instruction::parse(input)?;

    let mut program = Program::new(&instructions);
    let mut visited = BitSet::new(instructions.len());

    while visited.set(program.pos()) {
        program.execute();
    }

    Ok(program.acc())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let instructions = Instruction::parse(input)?;
    let mut destinations = find_destinations(&instructions);

    // Resolve twice to handle both backwards and forward jumps
    resolve_destinations(&mut destinations);
    resolve_destinations(&mut destinations);

    let mut program = Program::new(&instructions);

    loop {
        let instruction = &instructions[program.pos()];

        if let Some(pos) = find_loop_exit_pos(program.pos(), instruction, &destinations) {
            program.set_pos(pos);
            break;
        }

        program.execute();
    }

    while program.pos() < instructions.len() {
        program.execute()
    }

    Ok(program.acc())
}

fn find_loop_exit_pos(
    pos: usize,
    instruction: &Instruction,
    destinations: &[usize],
) -> Option<usize> {
    let pos = match instruction.operation() {
        Operation::Jmp => pos + 1,
        Operation::Nop => advance_pos(pos, instruction.value()),
        _ => return None,
    };

    if destinations[pos] == usize::MAX {
        Some(pos)
    } else {
        None
    }
}

fn find_destinations(instructions: &[Instruction]) -> Vec<usize> {
    let mut destinations = vec![0usize; instructions.len()];
    let mut last_destination = usize::MAX;

    for (pos, instruction) in instructions.iter().enumerate().rev() {
        if matches!(instruction.operation(), Operation::Jmp) {
            let next_pos = advance_pos(pos, instruction.value());

            last_destination = if next_pos >= destinations.len() {
                usize::MAX
            } else if next_pos > pos {
                destinations[next_pos]
            } else {
                next_pos
            }
        }

        if last_destination == pos {
            last_destination = usize::MIN;
        }

        destinations[pos] = last_destination;
    }

    destinations
}

fn resolve_destinations(destinations: &mut [usize]) {
    for pos in 0..destinations.len() {
        let dest = destinations[pos];

        if dest == usize::MIN || dest == usize::MAX {
            continue;
        }

        destinations[pos] = destinations[dest];
    }
}

struct Program<'a> {
    pos: usize,
    acc: isize,
    instructions: &'a [Instruction],
}

impl<'a> Program<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        Program {
            pos: 0usize,
            acc: 0isize,
            instructions,
        }
    }

    fn acc(&self) -> isize {
        self.acc
    }

    fn pos(&self) -> usize {
        self.pos
    }

    fn set_pos(&mut self, value: usize) {
        self.pos = value;
    }

    fn execute(&mut self) {
        let instruction = &self.instructions[self.pos];

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

const BUCKET_SIZE: usize = usize::BITS as usize;

#[derive(Clone)]
struct BitSet(Vec<usize>);

impl BitSet {
    fn new(capacity: usize) -> Self {
        BitSet(vec![0usize; (capacity - 1) / BUCKET_SIZE + 1])
    }

    fn set(&mut self, bit: usize) -> bool {
        let bucket = &mut self.0[bit / BUCKET_SIZE];
        let mask = 1 << (bit % BUCKET_SIZE);
        let is_unset = *bucket & mask == 0;
        *bucket |= mask;
        is_unset
    }
}
