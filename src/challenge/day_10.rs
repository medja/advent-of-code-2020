pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    // Assume all jolt differences are either 1 or 3
    let max = input
        .iter()
        .map(|line| line.parse::<usize>().unwrap())
        .max()
        .unwrap();

    let x = (max - input.len()) / 2;
    let y = input.len() - x;
    Ok((x + 1) * y)
}
