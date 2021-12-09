use std::mem::size_of;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut layout = SeatLayout::new(input);
    while layout.simulate() {}
    Ok(layout.count_occupied())
}

const EMPTY_BITS: u8 = 0b00;
const OCCUPIED_BITS: u8 = 0b01;

const SEAT_MASK: usize = 0b11;
const BUCKET_SIZE: usize = size_of::<usize>() * 8;

struct SeatLayout {
    width: usize,
    height: usize,
    buckets: Vec<usize>,
}

impl SeatLayout {
    fn new(input: &[&str]) -> SeatLayout {
        let width = input[0].len() + 2;
        let height = input.len() + 2;

        let mut builder = SeatLayoutBuilder::new(width, height);

        // Add empty seats at the top...
        for _ in 0..width {
            builder.add(false);
        }

        for line in input {
            // ...left...
            builder.add(false);

            for byte in line.bytes() {
                builder.add(byte == b'L')
            }

            // ...right...
            builder.add(false);
        }

        // ...and the bottom
        for _ in 0..width {
            builder.add(false);
        }

        SeatLayout {
            width,
            height,
            buckets: builder.build(),
        }
    }

    fn count_occupied(&self) -> usize {
        (1..(self.height - 1))
            .flat_map(|y| (1..(self.width - 1)).map(move |x| self.get_index(x, y)))
            .map(|index| (self.get_seat(index) & OCCUPIED_BITS) as usize)
            .sum::<usize>()
    }

    fn simulate(&mut self) -> bool {
        let mut changed = false;
        let mut buckets = self.buckets.clone();

        for y in 1..(self.height - 1) {
            for x in 1..(self.width - 1) {
                let index = self.get_index(x, y);

                match self.get_seat(index) {
                    EMPTY_BITS if self.count_adjacent_occupied(index) == 0 => {
                        // set occupied
                        changed = true;
                        buckets[index / BUCKET_SIZE] |= 1 << (index % BUCKET_SIZE)
                    }
                    OCCUPIED_BITS if self.count_adjacent_occupied(index) >= 4 => {
                        // set empty
                        changed = true;
                        buckets[index / BUCKET_SIZE] &= !(1 << (index % BUCKET_SIZE))
                    }
                    _ => {}
                }
            }
        }

        self.buckets = buckets;
        changed
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        (x + y * self.width) * 2
    }

    fn get_seat(&self, index: usize) -> u8 {
        (self.buckets[index / BUCKET_SIZE] >> (index % BUCKET_SIZE) & SEAT_MASK) as u8
    }

    fn count_adjacent_occupied(&self, index: usize) -> usize {
        let top = index - self.width * 2;
        let bottom = index + self.width * 2;

        let count = (self.get_seat(index - 2) & OCCUPIED_BITS)
            + (self.get_seat(index + 2) & OCCUPIED_BITS)
            + (self.get_seat(top) & OCCUPIED_BITS)
            + (self.get_seat(top - 2) & OCCUPIED_BITS)
            + (self.get_seat(top + 2) & OCCUPIED_BITS)
            + (self.get_seat(bottom) & OCCUPIED_BITS)
            + (self.get_seat(bottom - 2) & OCCUPIED_BITS)
            + (self.get_seat(bottom + 2) & OCCUPIED_BITS);

        count as usize
    }
}

struct SeatLayoutBuilder {
    position: usize,
    buffer: usize,
    buckets: Vec<usize>,
}

impl SeatLayoutBuilder {
    fn new(width: usize, height: usize) -> Self {
        SeatLayoutBuilder {
            position: 1,
            buffer: 0,
            buckets: Vec::with_capacity(((width * height * 2) - 1) / BUCKET_SIZE + 1),
        }
    }

    fn add(&mut self, exists: bool) {
        if !exists {
            self.buffer |= 1 << self.position
        }

        self.position = (self.position + 2) % BUCKET_SIZE;

        if self.position == 1 {
            self.buckets.push(self.buffer);
            self.buffer = 0;
        }
    }

    fn build(mut self) -> Vec<usize> {
        if self.position > 1 {
            self.buckets.push(self.buffer)
        }

        self.buckets
    }
}
