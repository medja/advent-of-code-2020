const BYR_OFFSET: usize = 0;
const IYR_OFFSET: usize = 1;
const EYR_OFFSET: usize = 2;
const HGT_OFFSET: usize = 3;
const HCL_OFFSET: usize = 4;
const ECL_OFFSET: usize = 5;
const PID_OFFSET: usize = 6;

const FIELD_NAMES: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const FIELD_MASK: u8 = 0b1111111;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(count_valid_passports(input, false))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(count_valid_passports(input, true))
}

fn count_valid_passports(input: &[&str], strict: bool) -> usize {
    input
        .split(|line| line.is_empty())
        .filter(|&chunk| validate(chunk, strict))
        .count()
}

fn validate(lines: &[&str], strict: bool) -> bool {
    let mut fields = 0u8;

    for field in lines.iter().flat_map(|line| line.split_whitespace()) {
        if let Some(offset) = FIELD_NAMES.iter().position(|&x| x == &field[0..3]) {
            fields |= 1 << offset;

            let valid = match offset {
                _ if !strict => true,
                BYR_OFFSET => validate_year(&field[4..], 1920, 2002),
                IYR_OFFSET => validate_year(&field[4..], 2010, 2020),
                EYR_OFFSET => validate_year(&field[4..], 2020, 2030),
                HGT_OFFSET => validate_height(&field[4..]),
                HCL_OFFSET => validate_color(&field[4..]),
                ECL_OFFSET => validate_eye_color(&field[4..]),
                PID_OFFSET => validate_passport_id(&field[4..]),
                _ => false,
            };

            if !valid {
                return false;
            }
        }
    }

    fields == FIELD_MASK
}

fn validate_year(value: &str, min: u32, max: u32) -> bool {
    matches!(value.parse::<u32>(), Ok(year) if year >= min && year <= max)
}

fn validate_height(value: &str) -> bool {
    let index = value.len() - 2;
    let height = value[..index].parse::<u8>().unwrap_or(0);

    match &value[index..] {
        "cm" => (150..=193).contains(&height),
        "in" => (59..=76).contains(&height),
        _ => false,
    }
}

fn validate_color(value: &str) -> bool {
    if value.len() != 7 || value.as_bytes()[0] != b'#' {
        return false;
    }

    value[1..].chars().all(|x| x.is_ascii_hexdigit())
}

fn validate_eye_color(value: &str) -> bool {
    matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}

fn validate_passport_id(value: &str) -> bool {
    value.len() == 9 && value.chars().all(|x| x.is_ascii_digit())
}
