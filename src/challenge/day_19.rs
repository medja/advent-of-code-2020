use anyhow::Context;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let (rules, input) = parse_and_input(input, false)?;
    Ok(input.iter().filter(|line| rules.test(line)).count())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let (rules, input) = parse_and_input(input, true)?;
    Ok(input.iter().filter(|line| rules.test(line)).count())
}

fn parse_and_input<'a>(
    input: &'a [&'a str],
    with_loops: bool,
) -> anyhow::Result<(Rules, &'a [&'a str])> {
    let index = input
        .iter()
        .position(|line| line.is_empty())
        .context("Input does not contain an empty line")?;

    let rules = Rules::parse(&input[..index], with_loops)?;
    Ok((rules, &input[index + 1..]))
}

struct Rules(Vec<Pattern>);

impl Rules {
    fn parse(input: &[&str], with_loops: bool) -> anyhow::Result<Self> {
        let mut patterns = vec![Pattern::Literal('\0'); input.len()];

        for line in input {
            let (id, pattern) = Pattern::parse(line)?;
            patterns[id] = pattern;
        }

        if with_loops {
            patterns[8] = Pattern::Double(Combination::parse("42")?, Combination::parse("42 8")?);
            patterns[11] = Pattern::Double(
                Combination::parse("42 31")?,
                Combination::parse("42 11 31")?,
            );
        }

        Ok(Rules(patterns))
    }

    fn test(&self, input: &str) -> bool {
        self.0[0].test(input, 0, &self.0).contains(input.len())
    }
}

#[derive(Copy, Clone, Debug)]
enum Pattern {
    Literal(char),
    Single(Combination),
    Double(Combination, Combination),
}

impl Pattern {
    fn parse(input: &str) -> anyhow::Result<(usize, Self)> {
        let (id, input) = input
            .split_once(':')
            .with_context(|| format!("{} does not contain a :", input))?;

        let id = id.parse()?;

        let pattern = match input.split_once('|') {
            None if input.as_bytes()[1] == b'"' => {
                Pattern::Literal(input[2..].chars().next().unwrap())
            }
            None => Pattern::Single(Combination::parse(input)?),
            Some((first, second)) => {
                Pattern::Double(Combination::parse(first)?, Combination::parse(second)?)
            }
        };

        Ok((id, pattern))
    }

    fn test(&self, input: &str, start: usize, rules: &[Pattern]) -> BitSet {
        if start >= input.len() {
            return BitSet::empty();
        }

        match self {
            Pattern::Literal(literal) => {
                if input[start..].starts_with(*literal) {
                    BitSet::single(start + 1)
                } else {
                    BitSet::empty()
                }
            }
            Pattern::Single(combination) => combination.test(input, start, rules),
            Pattern::Double(first, second) => first
                .test(input, start, rules)
                .union(&second.test(input, start, rules)),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Combination {
    length: usize,
    references: [usize; 3],
}

impl Combination {
    fn parse(input: &str) -> anyhow::Result<Self> {
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

    fn test(&self, input: &str, start: usize, rules: &[Pattern]) -> BitSet {
        (0..self.length).fold(BitSet::single(start), |set, i| {
            set.iter().fold(BitSet::empty(), |set, start| {
                set.union(&rules[self.references[i]].test(input, start, rules))
            })
        })
    }
}

struct BitSet(u64, u64);

impl BitSet {
    fn empty() -> Self {
        BitSet(0, 0)
    }

    fn single(index: usize) -> Self {
        BitSet::empty().insert(index)
    }

    fn insert(mut self, index: usize) -> Self {
        if index > 64 {
            self.1 |= 1 << (index - 64);
        } else {
            self.0 |= 1 << index;
        }

        self
    }

    fn union(mut self, other: &Self) -> Self {
        self.0 |= other.0;
        self.1 |= other.1;
        self
    }

    fn contains(&self, index: usize) -> bool {
        if index > 64 {
            (self.1 & (1 << (index - 64))) != 0
        } else {
            (self.0 & (1 << index)) != 0
        }
    }

    fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        BitSetIterator {
            index: 0,
            set: self,
        }
    }
}

struct BitSetIterator<'a> {
    index: usize,
    set: &'a BitSet,
}

impl Iterator for BitSetIterator<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = self.index;

        while i < 128 {
            if self.set.contains(i) {
                self.index = i + 1;
                return Some(i);
            }

            i += 1;
        }

        self.index = i;
        None
    }
}
