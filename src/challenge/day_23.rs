pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(Cups::<9>::new(input[0]).simulate())
}

struct Cup {
    value: u8,
    next: u8,
}

struct Cups<const N: usize>([Cup; N]);

impl<const N: usize> Cups<N> {
    fn new(input: &str) -> Self {
        let mut cups = std::array::from_fn(|i| Cup {
            value: i as u8,
            next: i as u8 + 1,
        });

        for (i, value) in input.bytes().enumerate() {
            cups[i].value = value - b'1';
        }

        cups.last_mut().unwrap().next = 0;

        Cups(cups)
    }

    fn simulate(&mut self) -> usize {
        let length = self.0.len() as u8;
        let mut current = 0;

        for _ in 0..100 {
            let mut removed = [self.0[current].next as usize, 0, 0];
            removed[1] = self.0[removed[0]].next as usize;
            removed[2] = self.0[removed[1]].next as usize;

            self.0[current].next = self.0[removed[2]].next;

            let mut destination = (self.0[current].value + length - 1) % length;

            while removed
                .iter()
                .any(|index| self.0[*index].value == destination)
            {
                destination = (destination + length - 1) % length;
            }

            let destination = self
                .0
                .iter()
                .position(|cup| cup.value == destination)
                .unwrap();

            self.0[removed[2]].next = self.0[destination].next;
            self.0[destination].next = removed[0] as u8;

            current = self.0[current].next as usize;
        }

        let mut index = self
            .0
            .iter()
            .find(|cup| cup.value == 0)
            .map(|cup| cup.next)
            .unwrap() as usize;

        let mut result = 0;

        loop {
            let cup = &self.0[index];

            if cup.value == 0 {
                break result;
            }

            index = cup.next as usize;
            result = (result * 10) + cup.value as usize + 1;
        }
    }
}
