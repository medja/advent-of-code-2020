pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input.iter().map(|line| Seat::new(line)).max().unwrap().id())
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Seat(u16);

impl Seat {
    fn new(string: &str) -> Self {
        Seat(string.as_bytes().iter().fold(0u16, |acc, char| {
            acc << 1 | (matches!(char, b'B' | b'R') as u16)
        }))
    }

    fn id(&self) -> u16 {
        self.0
    }
}
