use anyhow::anyhow;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let numbers = parse_numbers(input)?;

    for (i, &x) in numbers[0..numbers.len() - 1].iter().enumerate() {
        let y = 2020 - x;

        if numbers[i + 1..].binary_search(&y).is_ok() {
            return Ok(x * y);
        }
    }

    Err(anyhow!("Could not find 2 values that sum up to 2020"))
}

// 61515678
pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let numbers = parse_numbers(input)?;

    for (i, &x) in numbers[0..numbers.len() - 2].iter().enumerate() {
        for (j, &y) in numbers[i + 1..numbers.len() - 1].iter().enumerate() {
            let z = 2020 - x - y;

            if numbers[i + j + 2..].binary_search(&z).is_ok() {
                return Ok(x * y * z);
            }
        }
    }

    Err(anyhow!("Could not find 3 values that sum up to 2020"))
}

fn parse_numbers(input: &[&str]) -> anyhow::Result<Vec<u32>> {
    let mut numbers = input
        .iter()
        .map(|line| line.parse())
        .collect::<Result<Vec<u32>, _>>()?;
    numbers.sort_unstable();
    Ok(numbers)
}
