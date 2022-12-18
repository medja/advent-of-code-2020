pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut cups = [0u8; 9];
    cups.copy_from_slice(input[0].as_bytes());

    let mut cups = cups.map(|char| char - b'1');
    let mut next_cups = [0u8; 9];

    for _ in 0..100 {
        let mut destination = (cups[0] + 8) % 9;

        while cups[1..4].contains(&destination) {
            destination = (destination + 8) % 9;
        }

        let index = cups.iter().position(|cup| *cup == destination).unwrap();

        // copy cups before removed cups
        next_cups[0..=index - 4].copy_from_slice(&cups[4..=index]);
        // copy removed cups
        next_cups[index - 3..index].copy_from_slice(&cups[1..4]);
        // copy cups after removed cups
        next_cups[index..cups.len() - 1].copy_from_slice(&cups[index + 1..]);
        // copy the first cup
        next_cups[cups.len() - 1] = cups[0];

        cups = next_cups;
    }

    let index = cups.iter().position(|cup| *cup == 0).unwrap();
    let mut result = 0usize;

    for cup in &cups[index + 1..] {
        result = 10 * result + *cup as usize + 1;
    }

    for cup in &cups[..index] {
        result = 10 * result + *cup as usize + 1;
    }

    Ok(result)
}
