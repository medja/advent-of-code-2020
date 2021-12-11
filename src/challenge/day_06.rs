use std::ops::{BitAnd, BitOr};

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(sum_answer_counts(input, Answers::bitor))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(sum_answer_counts(input, Answers::bitand))
}

fn sum_answer_counts(input: &[&str], f: impl Fn(Answers, Answers) -> Answers + Copy) -> usize {
    input
        .split(|line| line.is_empty())
        .map(|lines| {
            lines
                .iter()
                .map(|line| Answers::new(line))
                .reduce(f)
                .unwrap_or_default()
                .count()
        })
        .sum::<usize>()
}

#[derive(Default)]
struct Answers(u32);

impl Answers {
    fn new(line: &str) -> Answers {
        let value = line
            .bytes()
            .fold(0u32, |acc, byte| acc | 1 << (byte - b'a') as usize);

        Answers(value)
    }

    fn count(&self) -> usize {
        self.0.count_ones() as usize
    }
}

impl BitOr for Answers {
    type Output = Answers;

    fn bitor(self, rhs: Self) -> Self::Output {
        Answers(self.0 | rhs.0)
    }
}

impl BitAnd for Answers {
    type Output = Answers;

    fn bitand(self, rhs: Self) -> Self::Output {
        Answers(self.0 & rhs.0)
    }
}
