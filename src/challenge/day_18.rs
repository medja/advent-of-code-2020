pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let result = input.iter()
        .map(|line| evaluate(line).map(|result| result.0).unwrap_or(0))
        .sum::<u64>();

    Ok(result)
}

fn evaluate(mut input: &str) -> anyhow::Result<(u64, &str)> {
    let mut lhs: u64;
    (lhs, input) = take_number(input)?;

    while !input.is_empty() {
        let rhs: u64;
        let operator = input.as_bytes()[0];

        if operator == b')' {
            return Ok((lhs, input[1..].trim_start_matches(' ')));
        }

        (rhs, input) = take_number(&input[2..])?;

        lhs = match operator {
            b'+' => lhs + rhs,
            b'*' => lhs * rhs,
            _ => unreachable!(),
        }
    }

    Ok((lhs, input))
}

fn take_number(input: &str) -> anyhow::Result<(u64, &str)> {
    if let Some(stripped) = input.strip_prefix('(') {
        evaluate(stripped)
    } else {
        match input.find(|char: char| !char.is_ascii_digit()) {
            None => Ok((input.parse()?, "")),
            Some(index) => Ok((input[..index].parse()?, input[index..].trim_start_matches(' ')))
        }
    }
}