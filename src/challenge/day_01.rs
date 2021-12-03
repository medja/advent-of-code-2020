use anyhow::Context;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let values = input
        .iter()
        .map(|line| line.parse())
        .collect::<Result<Vec<u32>, _>>()?;

    let result = values
        .iter()
        .enumerate()
        .flat_map(|(i, &a)| values[(i + 1)..].iter().map(move |&b| (a, b)))
        .find(|(a, b)| a + b == 2020)
        .map(|(a, b)| a * b)
        .context("Could not find two values that sum up to 2020")?;

    Ok(result)
}
