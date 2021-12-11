use anyhow::Context;
use std::ops::RangeInclusive;
use std::str::FromStr;

const FIELD_COUNT: usize = 20;
const INVALID_RULE: &str = "Ticket rule is invalid";

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let rules = input[0..FIELD_COUNT]
        .iter()
        .map(|line| line.parse())
        .collect::<Result<Vec<Rule>, _>>()?;

    let mut error_rate = 0usize;

    for ticket in &input[FIELD_COUNT + 5..] {
        for number in ticket.split(',') {
            let number = number.parse()?;

            if !rules.iter().any(|rule| rule.matches(&number)) {
                error_rate += number;
                break;
            }
        }
    }

    Ok(error_rate)
}

struct Rule(RangeInclusive<usize>, RangeInclusive<usize>);

impl Rule {
    fn matches(&self, value: &usize) -> bool {
        self.0.contains(value) || self.1.contains(value)
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line = &line.split_once(':').context(INVALID_RULE)?.1;

        let (a, line) = line[1..].split_once('-').context(INVALID_RULE)?;
        let (b, line) = line.split_once(' ').context(INVALID_RULE)?;
        let (c, d) = line[3..].split_once('-').context(INVALID_RULE)?;

        Ok(Rule(a.parse()?..=b.parse()?, c.parse()?..=d.parse()?))
    }
}
