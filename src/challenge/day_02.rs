use anyhow::Context;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input.iter().filter(|&&line| validate(line)).count())
}

fn validate(line: &str) -> bool {
    match Entry::try_from(line) {
        Ok(entry) => entry.validate(),
        Err(_) => false,
    }
}

struct Entry<'a> {
    letter: u8,
    min: usize,
    max: usize,
    password: &'a [u8],
}

impl Entry<'_> {
    fn validate(&self) -> bool {
        let count = self
            .password
            .iter()
            .filter(|&&x| x == self.letter)
            .take(self.max + 1)
            .count();

        count >= self.min && count <= self.max
    }
}

impl<'a> TryFrom<&'a str> for Entry<'a> {
    type Error = anyhow::Error;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        let (min, line) = line
            .split_once('-')
            .context("Could not find range delimiter (`-`)")?;

        let (max, line) = line
            .split_once(' ')
            .context("Could not find end of range (` `)")?;

        let min = min.parse()?;
        let max = max.parse()?;

        let letter = line.as_bytes()[0];
        let password = &line[3..].as_bytes();

        Ok(Entry {
            letter,
            min,
            max,
            password,
        })
    }
}
