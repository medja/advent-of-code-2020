use anyhow::Context;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(count_valid(input, validate_a))
}

fn validate_a(entry: &Entry) -> bool {
    let count = entry
        .password
        .iter()
        .filter(|&&x| x == entry.letter)
        .take(entry.second + 1)
        .count();

    count >= entry.first && count <= entry.second
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(count_valid(input, validate_b))
}

fn validate_b(entry: &Entry) -> bool {
    let first = entry.password[entry.first - 1] == entry.letter;
    let second = entry.password[entry.second - 1] == entry.letter;

    first ^ second
}

fn count_valid(input: &[&str], predicate: impl Fn(&Entry) -> bool) -> usize {
    input
        .iter()
        .filter_map(|&line| Entry::try_from(line).ok())
        .filter(predicate)
        .count()
}

struct Entry<'a> {
    letter: u8,
    first: usize,
    second: usize,
    password: &'a [u8],
}

impl<'a> TryFrom<&'a str> for Entry<'a> {
    type Error = anyhow::Error;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        let (first, line) = line
            .split_once('-')
            .context("Could not find range delimiter (`-`)")?;

        let (second, line) = line
            .split_once(' ')
            .context("Could not find end of range (` `)")?;

        let first = first.parse()?;
        let second = second.parse()?;

        let letter = line.as_bytes()[0];
        let password = &line[3..].as_bytes();

        Ok(Entry {
            letter,
            first,
            second,
            password,
        })
    }
}
