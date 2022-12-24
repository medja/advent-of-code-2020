pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut positions = input.iter().map(|line| locate(line)).collect::<Vec<_>>();
    positions.sort();

    let mut black = 0;
    let mut count = 1;

    for window in positions.windows(2) {
        let previous = window[0];
        let current = window[1];

        if current == previous {
            count += 1;
            continue;
        }

        if count % 2 == 1 {
            black += 1;
        }

        count = 1;
    }

    if count % 2 == 1 {
        black += 1;
    }

    Ok(black)
}

fn locate(input: &str) -> (i8, i8) {
    let input = input.as_bytes();
    let (mut x, mut y) = (0, 0);
    let mut i = 0;

    while i < input.len() {
        let char = input[i];
        i += 1;

        match char {
            b'e' => {
                x -= 2;
                continue;
            }
            b'w' => {
                x += 2;
                continue;
            }
            b's' => y -= 1,
            b'n' => y += 1,
            _ => unreachable!(),
        }

        match input[i] {
            b'e' => x -= 1,
            b'w' => x += 1,
            _ => unreachable!(),
        }

        i += 1;
    }

    (x, y)
}
