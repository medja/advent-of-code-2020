use anyhow::Context;
use std::collections::HashMap;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut memory = HashMap::new();

    let mut set_mask = u64::MIN;
    let mut unset_mask = u64::MAX;

    for line in input {
        let (left, right) = line
            .split_once('=')
            .context("Unexpected end of line, expecting `=`")?;

        if left.len() == 5 {
            set_mask = u64::MIN;
            unset_mask = u64::MIN;

            for (i, byte) in right.as_bytes()[1..].iter().enumerate() {
                match *byte {
                    b'0' => unset_mask |= 1 << (35 - i),
                    b'1' => set_mask |= 1 << (35 - i),
                    _ => {}
                }
            }

            unset_mask = !unset_mask;
        } else {
            let position = &left[4..left.len() - 2];
            let value = right[1..].parse::<u64>()?;
            memory.insert(position, (value | set_mask) & unset_mask);
        }
    }

    Ok(memory.values().sum::<u64>())
}
