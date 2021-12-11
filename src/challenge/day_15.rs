use std::collections::HashMap;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut last = 0usize;
    let mut numbers = HashMap::<usize, usize>::new();

    for (i, number) in input[0].split(',').enumerate() {
        last = number.parse()?;
        numbers.insert(last, i);
    }

    numbers.remove(&last);

    for i in numbers.len()..(2020 - 1) {
        last = i - numbers.insert(last, i).unwrap_or(i);
    }

    Ok(last)
}
