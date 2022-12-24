pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut cups = Cups::new(input[0], 9);
    cups.simulate(100);

    let mut index = cups.0[cups.find_index(0)].next;
    let mut result = 0;

    loop {
        let cup = &cups.0[index];

        if cup.value == 0 {
            break Ok(result);
        }

        index = cup.next;
        result = (result * 10) + cup.value + 1;
    }
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut cups = Cups::new(input[0], 1000000);
    cups.simulate(10000000);

    let index = cups.0[cups.find_index(0)].next;
    let cup = &cups.0[index];

    Ok((cup.value + 1) * (cups.0[cup.next].value + 1))
}

struct Cup {
    value: usize,
    next: usize,
}

struct Cups(Vec<Cup>);

impl Cups {
    fn new(input: &str, count: usize) -> Self {
        let mut cups = Vec::with_capacity(count);

        for (i, value) in input.bytes().enumerate() {
            cups.push(Cup {
                value: (value - b'1') as usize,
                next: i + 1,
            });
        }

        for i in 9..count {
            cups.push(Cup {
                value: i,
                next: i + 1,
            });
        }

        cups.last_mut().unwrap().next = 0;

        Cups(cups)
    }

    fn simulate(&mut self, moves: usize) {
        let mut current = 0;

        for _ in 0..moves {
            let mut removed = [self.0[current].next, 0, 0];
            removed[1] = self.0[removed[0]].next;
            removed[2] = self.0[removed[1]].next;

            self.0[current].next = self.0[removed[2]].next;

            let mut destination = (self.0[current].value + self.0.len() - 1) % self.0.len();

            while removed
                .iter()
                .any(|index| self.0[*index].value == destination)
            {
                destination = (destination + self.0.len() - 1) % self.0.len();
            }

            let destination = self.find_index(destination);
            self.0[removed[2]].next = self.0[destination].next;
            self.0[destination].next = removed[0];

            current = self.0[current].next;
        }
    }

    fn find_index(&self, value: usize) -> usize {
        if value < 9 {
            self.0.iter().position(|cup| cup.value == value).unwrap()
        } else {
            value
        }
    }
}
