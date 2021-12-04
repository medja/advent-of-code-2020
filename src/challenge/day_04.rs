const FIELD_NAMES: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const FIELD_MASK: u8 = 0b1111111;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let count = input
        .split(|line| line.is_empty())
        .filter(|&chunk| validate(chunk))
        .count();

    Ok(count)
}

fn validate(lines: &[&str]) -> bool {
    let mut fields = 0u8;

    for field in lines.iter().flat_map(|line| line.split_whitespace()) {
        if let Some(offset) = FIELD_NAMES.iter().position(|&x| x == &field[0..3]) {
            fields |= 1 << offset;
        }
    }

    fields == FIELD_MASK
}
