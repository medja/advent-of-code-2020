use anyhow::anyhow;

const PREAMBLE_SIZE: usize = 25;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut preamble = Preamble::new();

    for line in &input[..PREAMBLE_SIZE] {
        preamble.add(line.parse()?);
    }

    for line in &input[PREAMBLE_SIZE..] {
        let value = line.parse()?;

        if preamble.check(value) {
            preamble.add(value);
        } else {
            return Ok(value);
        }
    }

    Err(anyhow!("Could not find invalid number"))
}

struct Preamble(usize, [usize; PREAMBLE_SIZE]);

impl Preamble {
    fn new() -> Self {
        Preamble(0, [0; PREAMBLE_SIZE])
    }

    fn add(&mut self, value: usize) {
        self.1[self.0] = value;
        self.0 = (self.0 + 1) % PREAMBLE_SIZE;
    }

    fn check(&self, value: usize) -> bool {
        for (i, &x) in self.1[..PREAMBLE_SIZE - 1].iter().enumerate() {
            if value > x && self.1[i + 1..].contains(&(value - x)) {
                return true;
            }
        }

        false
    }
}
