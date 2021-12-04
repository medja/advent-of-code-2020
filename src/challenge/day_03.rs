pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(Map::new(input).simulate(3, 1))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let map = Map::new(input);
    let mut product = 1usize;

    product *= map.simulate(1, 1);
    product *= map.simulate(3, 1);
    product *= map.simulate(5, 1);
    product *= map.simulate(7, 1);
    product *= map.simulate(1, 2);

    Ok(product)
}

struct Map(Vec<u32>, usize);

impl Map {
    fn new(lines: &[&str]) -> Map {
        let length = lines[0].len();
        let lines = lines.iter().map(|&line| Map::parse(line)).collect();

        Map(lines, length)
    }

    fn simulate(&self, dx: usize, dy: usize) -> usize {
        let (mut x, mut y) = (0, 0);
        let mut count = 0;

        loop {
            match self.is_tree(x, y) {
                Some(true) => count += 1,
                None => break count,
                _ => {}
            }

            x += dx;
            y += dy;
        }
    }

    fn is_tree(&self, x: usize, y: usize) -> Option<bool> {
        if y >= self.0.len() {
            None
        } else {
            Some((self.0[y] >> (x % self.1)) & 1 == 1)
        }
    }

    fn parse(line: &str) -> u32 {
        line.as_bytes().iter().rev().fold(0u32, Map::build_line)
    }

    fn build_line(acc: u32, char: &u8) -> u32 {
        if *char == b'#' {
            acc << 1 | 1
        } else {
            acc << 1
        }
    }
}
