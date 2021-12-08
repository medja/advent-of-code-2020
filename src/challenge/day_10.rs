pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut odds = 0usize;
    let mut max = 0usize;

    for line in input {
        let value = line.parse::<usize>()?;

        if value & 1 == 1 {
            odds += 1;
        }

        if value > max {
            max = value;
        }
    }

    let evens = input.len() - odds + 1;

    let x = (max - input.len()) / 2;
    // & !1 accounts for an additional difference of 1 caused by an odd number of inputs
    let y = abs_diff(odds, evens) & !1;
    let z = input.len() - x - y;

    Ok((x + 1) * z)
}

fn abs_diff(x: usize, y: usize) -> usize {
    if x > y {
        x - y
    } else {
        y - x
    }
}
