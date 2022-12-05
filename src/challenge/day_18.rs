pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(solve(input, false))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(solve(input, true))
}

fn solve(input: &[&str], prioritize_sum: bool) -> u64 {
    input
        .iter()
        .map(|line| Calculator::evaluate(line, prioritize_sum).unwrap())
        .sum::<u64>()
}

struct Calculator<'a> {
    input: &'a str,
    prioritize_sum: bool,
}

impl Calculator<'_> {
    fn evaluate(input: &str, prioritize_sum: bool) -> anyhow::Result<u64> {
        let mut calculator = Calculator {
            input,
            prioritize_sum,
        };

        calculator.evaluate_expression()
    }

    fn evaluate_expression(&mut self) -> anyhow::Result<u64> {
        let mut lhs = self.evaluate_sum()?;

        while !self.input.is_empty() {
            let operator = self.input.as_bytes()[0];

            if operator == b')' {
                self.input = self.input[1..].trim_start_matches(' ');
                break;
            }

            self.input = &self.input[2..];
            let rhs = self.evaluate_sum()?;

            match operator {
                b'+' => lhs += rhs,
                b'*' => lhs *= rhs,
                _ => unreachable!(),
            }
        }

        Ok(lhs)
    }

    fn evaluate_sum(&mut self) -> anyhow::Result<u64> {
        let mut lhs = self.evaluate_number()?;

        if !self.prioritize_sum {
            return Ok(lhs);
        }

        while self.input.starts_with('+') {
            self.input = &self.input[2..];
            lhs += self.evaluate_number()?;
        }

        Ok(lhs)
    }

    fn evaluate_number(&mut self) -> anyhow::Result<u64> {
        if let Some(input) = self.input.strip_prefix('(') {
            self.input = input;
            return self.evaluate_expression();
        }

        let index = self
            .input
            .find(|char: char| !char.is_ascii_digit())
            .unwrap_or(self.input.len());

        let value = &self.input[..index];
        self.input = self.input[index..].trim_start_matches(' ');

        Ok(value.parse()?)
    }
}
