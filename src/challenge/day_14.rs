use anyhow::Context;
use std::collections::HashMap;

const BIT_COUNT: usize = 36;

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
                    b'0' => unset_mask |= 1 << (BIT_COUNT - 1 - i),
                    b'1' => set_mask |= 1 << (BIT_COUNT - 1 - i),
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

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut memory = HashMap::new();

    let mut bit_mask = u64::MIN;
    let mut floating_masks = Vec::<u64>::with_capacity(BIT_COUNT);

    for line in input {
        let (left, right) = line
            .split_once('=')
            .context("Unexpected end of line, expecting `=`")?;

        if left.len() == 5 {
            bit_mask = u64::MIN;
            floating_masks.clear();

            for (i, byte) in right.as_bytes()[1..].iter().enumerate() {
                match *byte {
                    b'1' => bit_mask |= 1 << (BIT_COUNT - 1 - i),
                    b'X' => floating_masks.push(1 << (BIT_COUNT - 1 - i)),
                    _ => {}
                }
            }
        } else {
            let position = left[4..left.len() - 2].parse::<u64>()? | bit_mask;
            let value = right[1..].parse::<u64>()?;
            update(position, &floating_masks, value, &mut memory);
        }
    }

    Ok(memory.values().sum::<u64>())
}

fn update(position: u64, masks: &[u64], value: u64, memory: &mut HashMap<u64, u64>) {
    if masks.is_empty() {
        memory.insert(position, value);
        return;
    }

    let mask = masks[0];
    let masks = &masks[1..];

    update(position | mask, masks, value, memory);
    update(position & !mask, masks, value, memory);
}
