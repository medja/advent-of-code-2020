pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let result = input
        .split(|line| line.is_empty())
        .map(|line| Answers::new(line).count())
        .sum::<usize>();

    Ok(result)
}

struct Answers(u32);

impl Answers {
    fn new(lines: &[&str]) -> Answers {
        let value = lines
            .iter()
            .flat_map(|line| line.bytes())
            .fold(0u32, |acc, byte| acc | 1 << (byte - b'a') as usize);

        Answers(value)
    }

    fn count(&self) -> usize {
        self.0.count_ones() as usize
    }
}
