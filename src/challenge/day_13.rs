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
