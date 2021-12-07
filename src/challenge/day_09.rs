use anyhow::anyhow;
use std::cmp::Ordering;

const PREAMBLE_SIZE: usize = 25;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut preamble = Preamble::new();

    for line in &input[..PREAMBLE_SIZE] {
        preamble.add(line.parse()?);
    }

    for line in &input[PREAMBLE_SIZE..] {
        let value = line.parse()?;

        if !preamble.add_checked(value) {
            return Ok(value);
        }
    }

    Err(anyhow!("Could not find invalid number"))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut preamble = Preamble::new();
    let mut numbers = Vec::with_capacity(input.len());

    for line in &input[..PREAMBLE_SIZE] {
        let value = line.parse()?;
        numbers.push(value);
        preamble.add(value);
    }

    let mut invalid = 0u64;

    for line in &input[PREAMBLE_SIZE..] {
        let value = line.parse()?;
        numbers.push(value);

        if !preamble.add_checked(value) {
            invalid = value;
            break;
        }
    }

    let range = find_range(&numbers, invalid);
    let min = range.iter().min().unwrap();
    let max = range.iter().max().unwrap();

    Ok(min + max)
}

fn find_range(numbers: &[u64], sum: u64) -> &[u64] {
    let mut start = 0usize;
    let mut end = 0usize;
    let mut acc = 0u64;

    loop {
        match acc.cmp(&sum) {
            Ordering::Less => {
                acc += numbers[end];
                end += 1;
            }
            Ordering::Greater => {
                acc -= numbers[start];
                start += 1;
            }
            Ordering::Equal => break &numbers[start..end],
        }
    }
}

struct Preamble(usize, [u64; PREAMBLE_SIZE]);

impl Preamble {
    fn new() -> Self {
        Preamble(0, [0; PREAMBLE_SIZE])
    }

    fn add(&mut self, value: u64) {
        self.1[self.0] = value;
        self.0 = (self.0 + 1) % PREAMBLE_SIZE;
    }

    fn add_checked(&mut self, value: u64) -> bool {
        if self.check(value) {
            self.add(value);
            true
        } else {
            false
        }
    }

    fn check(&self, value: u64) -> bool {
        for (i, &x) in self.1[..PREAMBLE_SIZE - 1].iter().enumerate() {
            if value > x && self.1[i + 1..].contains(&(value - x)) {
                return true;
            }
        }

        false
    }
}
