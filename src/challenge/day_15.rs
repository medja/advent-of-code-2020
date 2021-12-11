use std::collections::HashMap;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    play_until(input[0], 2020)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    play_until(input[0], 30000000)
}

fn play_until(starting_numbers: &str, last_step: usize) -> anyhow::Result<u64> {
    let mut last = 0u64;
    let mut numbers = HashMap::new();

    for (i, number) in starting_numbers.split(',').enumerate() {
        last = number.parse()?;
        numbers.insert(last, i as u64);
    }

    numbers.remove(&last);

    for i in (numbers.len() as u64)..(last_step as u64 - 1) {
        last = i - numbers.insert(last, i).unwrap_or(i);
    }

    Ok(last)
}
