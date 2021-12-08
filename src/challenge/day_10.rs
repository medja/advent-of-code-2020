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

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut jolts = input
        .iter()
        .map(|line| line.parse())
        .collect::<Result<Vec<usize>, _>>()?;

    jolts.sort_unstable();

    let mut paths = vec![0; jolts.len()];

    paths[0] = 1;
    paths[1] = compute_paths_initial(jolts[1], &jolts[..1], &paths[..1]);
    paths[2] = compute_paths_initial(jolts[2], &jolts[..2], &paths[..2]);

    for (i, &x) in jolts[3..].iter().enumerate() {
        let j = i + 3;
        paths[j] = compute_paths(x, &jolts[i..j], &paths[i..j]);
    }

    Ok(paths[jolts.len() - 1])
}

fn compute_paths(current: usize, previous: &[usize], paths: &[usize]) -> usize {
    let mut result = 0usize;

    for (i, x) in previous.iter().enumerate() {
        if *x + 3 >= current {
            result += paths[i];
        }
    }

    result
}

fn compute_paths_initial(current: usize, previous: &[usize], paths: &[usize]) -> usize {
    let result = compute_paths(current, previous, paths);

    if current <= 3 {
        result + 1
    } else {
        result
    }
}

fn abs_diff(x: usize, y: usize) -> usize {
    if x > y {
        x - y
    } else {
        y - x
    }
}
