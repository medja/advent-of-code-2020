use anyhow::Context;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let (rules, input) = parse_and_input(input)?;
    Ok(input.iter().filter(|line| rules.is_match(line)).count())
}

struct Rules(Vec<Pattern>);

impl Rules {
    fn is_match(&self, input: &str) -> bool {
        matches!(self.0[0].strip_pattern(input, &self.0), Some(remaining) if remaining.is_empty())
    }
}

#[derive(Copy, Clone)]
enum Pattern {
    Literal(char),
    Single(Combination),
    Double(Combination, Combination),
}

impl Pattern {
    fn strip_pattern<'a>(&self, input: &'a str, rules: &[Pattern]) -> Option<&'a str> {
        match self {
            Pattern::Literal(literal) => input.strip_prefix(*literal),
            Pattern::Single(combination) => combination.strip_pattern(input, rules),
            Pattern::Double(first, second) => first
                .strip_pattern(input, rules)
                .or_else(|| second.strip_pattern(input, rules)),
        }
    }
}

#[derive(Copy, Clone)]
struct Combination {
    length: usize,
    references: [usize; 3],
}

impl Combination {
    fn strip_pattern<'a>(&self, input: &'a str, rules: &[Pattern]) -> Option<&'a str> {
        self.references[..self.length]
            .iter()
            .try_fold(input, |input, reference| {
                rules[*reference].strip_pattern(input, rules)
            })
    }
}

fn parse_and_input<'a>(input: &'a [&'a str]) -> anyhow::Result<(Rules, &'a [&'a str])> {
    let index = input
        .iter()
        .position(|line| line.is_empty())
        .context("Input does not contain an empty line")?;

    Ok((parse_rules(&input[..index])?, &input[index + 1..]))
}

fn parse_rules(input: &[&str]) -> anyhow::Result<Rules> {
    let mut patterns = vec![Pattern::Literal('\0'); input.len()];

    for line in input {
        let (id, pattern) = parse_pattern(line)?;
        patterns[id] = pattern;
    }

    Ok(Rules(patterns))
}

fn parse_pattern(input: &str) -> anyhow::Result<(usize, Pattern)> {
    let (id, input) = input
        .split_once(':')
        .with_context(|| format!("{} does not contain a :", input))?;

    let id = id.parse()?;

    let pattern = match input.split_once('|') {
        None if input.as_bytes()[1] == b'"' => Pattern::Literal(input[2..].chars().next().unwrap()),
        None => Pattern::Single(parse_combination(input)?),
        Some((first, second)) => {
            Pattern::Double(parse_combination(first)?, parse_combination(second)?)
        }
    };

    Ok((id, pattern))
}

fn parse_combination(input: &str) -> anyhow::Result<Combination> {
    let mut i = 0;
    let mut references = [0usize; 3];

    for value in input.split_ascii_whitespace() {
        references[i] = value.parse()?;
        i += 1;
    }

    let combination = Combination {
        length: i,
        references,
    };

    Ok(combination)
}
