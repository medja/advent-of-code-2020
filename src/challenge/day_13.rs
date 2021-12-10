pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let arrived_at = input[0].parse::<usize>()?;

    let mut best_id = 0;
    let mut min_wait_time = usize::MAX;

    for id in input[1].split(',') {
        if id.starts_with('x') {
            continue;
        }

        let id = id.parse::<usize>()?;
        let wait_time = (id - (arrived_at % id)) % id;

        if wait_time < min_wait_time {
            best_id = id;
            min_wait_time = wait_time;
        }
    }

    Ok(best_id * min_wait_time)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut time = 0u64;
    let mut step = 1u64;

    for (i, id) in input[1].split(',').enumerate() {
        if id.starts_with('x') {
            continue;
        }

        let id = id.parse::<u64>()?;
        let offset = i as u64;

        while (time + offset) % id != 0 {
            time += step;
        }

        step *= id;
    }

    Ok(time)
}
