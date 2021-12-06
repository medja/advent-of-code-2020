use anyhow::{anyhow, Context};
use lazy_static::lazy_static;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Instant;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;

lazy_static! {
    static ref CHALLENGE_PATTERN: Regex =
        Regex::new("(?i)(?:Day\\W*)?(\\d\\d?)\\W*([AB])").unwrap();
    static ref SOLUTIONS: HashMap<Challenge, Box<dyn Solution + Sync + 'static>> = {
        use Day::*;
        use Part::*;

        let mut builder = HashMap::new();

        builder.insert(Challenge::new(Day01, PartA), box_solution(day_01::part_a));
        builder.insert(Challenge::new(Day01, PartB), box_solution(day_01::part_b));
        builder.insert(Challenge::new(Day02, PartA), box_solution(day_02::part_a));
        builder.insert(Challenge::new(Day02, PartB), box_solution(day_02::part_b));
        builder.insert(Challenge::new(Day03, PartA), box_solution(day_03::part_a));
        builder.insert(Challenge::new(Day03, PartB), box_solution(day_03::part_b));
        builder.insert(Challenge::new(Day04, PartA), box_solution(day_04::part_a));
        builder.insert(Challenge::new(Day04, PartB), box_solution(day_04::part_b));
        builder.insert(Challenge::new(Day05, PartA), box_solution(day_05::part_a));
        builder.insert(Challenge::new(Day05, PartB), box_solution(day_05::part_b));
        builder.insert(Challenge::new(Day06, PartA), box_solution(day_06::part_a));
        builder.insert(Challenge::new(Day06, PartB), box_solution(day_06::part_b));
        builder.insert(Challenge::new(Day07, PartA), box_solution(day_07::part_a));
        builder.insert(Challenge::new(Day07, PartB), box_solution(day_07::part_b));
        builder.insert(Challenge::new(Day08, PartA), box_solution(day_08::part_a));
        builder.insert(Challenge::new(Day08, PartB), box_solution(day_08::part_b));
        builder.insert(Challenge::new(Day09, PartA), box_solution(day_09::part_a));

        builder
    };
}

#[derive(
    IntoPrimitive, TryFromPrimitive, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,
)]
#[repr(u8)]
pub enum Day {
    Day01 = 1,
    Day02 = 2,
    Day03 = 3,
    Day04 = 4,
    Day05 = 5,
    Day06 = 6,
    Day07 = 7,
    Day08 = 8,
    Day09 = 9,
}

impl Day {
    fn name(&self) -> &'static str {
        use Day::*;

        match self {
            Day01 => "Report Repair",
            Day02 => "Password Philosophy",
            Day03 => "Toboggan Trajectory",
            Day04 => "Passport Processing",
            Day05 => "Binary Boarding",
            Day06 => "Custom Customs",
            Day07 => "Handy Haversacks",
            Day08 => "Handheld Halting",
            Day09 => "Encoding Error",
        }
    }

    async fn input(&self) -> anyhow::Result<String> {
        let index = u8::from(*self);
        crate::http::get(format!("https://adventofcode.com/2020/day/{}/input", index)).await
    }
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day {:02}: {}", u8::from(*self), self.name())
    }
}

impl FromStr for Day {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let index = string.parse::<u8>()?;

        index
            .try_into()
            .with_context(|| format!("Day {} is out of range", index))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Part {
    PartA,
    PartB,
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::PartA => write!(f, "Part A"),
            Part::PartB => write!(f, "Part B"),
        }
    }
}

impl FromStr for Part {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "a" | "A" => Ok(Part::PartA),
            "b" | "B" => Ok(Part::PartB),
            _ => Err(anyhow!("{} is not a valid part, expecting A or B", string)),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Challenge(Day, Part);

impl Challenge {
    pub fn new(day: Day, part: Part) -> Self {
        Challenge(day, part)
    }
}

impl FromStr for Challenge {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let captures = CHALLENGE_PATTERN
            .captures(string)
            .with_context(|| format!("{} is not a valid challenge, expecting \\d+[AB]", string))?;

        let day = captures
            .get(1)
            .context("Day capture group is missing")?
            .as_str()
            .parse()?;

        let part = captures
            .get(2)
            .context("Part capture group is missing")?
            .as_str()
            .parse()?;

        Ok(Challenge(day, part))
    }
}

impl std::fmt::Display for Challenge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.0, self.1)
    }
}

pub async fn solve(challenge: &Challenge) -> anyhow::Result<()> {
    let solution = SOLUTIONS
        .get(challenge)
        .with_context(|| format!("Cannot find solution for {}", challenge))?;

    let input = challenge.0.input().await?;
    solution.run(challenge, &input.lines().collect::<Vec<_>>())
}

fn box_solution<R: std::fmt::Display + 'static>(
    func: fn(&[&str]) -> anyhow::Result<R>,
) -> Box<dyn Solution + Sync + 'static> {
    Box::new(func)
}

trait Solution {
    fn run(&self, challenge: &Challenge, input: &[&str]) -> anyhow::Result<()>;
}

impl<R: std::fmt::Display> Solution for fn(&[&str]) -> anyhow::Result<R> {
    fn run(&self, challenge: &Challenge, input: &[&str]) -> anyhow::Result<()> {
        let start = Instant::now();
        let result = self(input)?;
        let duration = start.elapsed();
        println!("{}: {} (duration = {:?})", challenge, result, duration);
        Ok(())
    }
}
