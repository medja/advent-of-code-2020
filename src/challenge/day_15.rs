pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    play_until::<2020>(input[0])
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    play_until::<30000000>(input[0])
}

fn play_until<const N: usize>(starting_numbers: &str) -> anyhow::Result<usize> {
    let mut last = 0usize;
    let mut numbers = [usize::MAX; N];

    for (i, number) in starting_numbers.split(',').enumerate() {
        last = number.parse()?;
        numbers[last] = i;
    }

    numbers[last] = usize::MAX;

    for i in numbers.len()..(N - 1) {
        let time = numbers[last];

        if time == usize::MAX {
            last = 0;
        } else {
            last = i - time;
        }

        numbers[last] = i;
    }

    Ok(last)
}
