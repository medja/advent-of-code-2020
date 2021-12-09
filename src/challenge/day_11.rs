use std::mem::size_of;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut layout = SeatLayout::new(input);
    while layout.simulate_v1() {}
    Ok(layout.count_occupied())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut layout = SeatLayout::new(input);
    while layout.simulate_v2() {}
    Ok(layout.count_occupied())
}

const EMPTY_BITS: u8 = 0b00;
const OCCUPIED_BITS: u8 = 0b01;
const FLOOR_BITS: u8 = 0b10;

const SEAT_MASK: usize = 0b11;
const BUCKET_SIZE: usize = size_of::<usize>() * 8;

struct SeatLayout {
    width: usize,
    height: usize,
    buckets: Vec<usize>,
}

impl SeatLayout {
    fn new(input: &[&str]) -> SeatLayout {
        let width = input[0].len();
        let height = input.len();

        let mut builder = SeatLayoutBuilder::new(width, height);

        input
            .iter()
            .flat_map(|line| line.bytes())
            .for_each(|byte| builder.add(byte == b'L'));

        SeatLayout {
            width,
            height,
            buckets: builder.build(),
        }
    }

    fn count_occupied(&self) -> usize {
        let mut count = 0;

        for y in 1..=self.height {
            for x in 1..=self.width {
                count += (self.get_seat(x, y, FLOOR_BITS) & OCCUPIED_BITS) as usize
            }
        }

        count
    }

    fn simulate_v1(&mut self) -> bool {
        let mut changed = false;
        let mut buckets = self.buckets.clone();

        for y in 1..=self.height {
            for x in 1..=self.width {
                match self.get_seat(x, y, FLOOR_BITS) {
                    EMPTY_BITS if self.count_adjacent_occupied_v1(x, y) == 0 => {
                        // set occupied
                        changed = true;
                        let index = self.get_index(x, y);
                        buckets[index / BUCKET_SIZE] |= 1 << (index % BUCKET_SIZE)
                    }
                    OCCUPIED_BITS if self.count_adjacent_occupied_v1(x, y) >= 4 => {
                        // set empty
                        changed = true;
                        let index = self.get_index(x, y);
                        buckets[index / BUCKET_SIZE] &= !(1 << (index % BUCKET_SIZE))
                    }
                    _ => {}
                }
            }
        }

        self.buckets = buckets;
        changed
    }

    fn simulate_v2(&mut self) -> bool {
        let mut changed = false;
        let mut buckets = self.buckets.clone();

        for y in 1..=self.height {
            for x in 1..=self.width {
                match self.get_seat(x, y, FLOOR_BITS) {
                    EMPTY_BITS if self.count_adjacent_occupied_v2(x, y) == 0 => {
                        // set occupied
                        changed = true;
                        let index = self.get_index(x, y);
                        buckets[index / BUCKET_SIZE] |= 1 << (index % BUCKET_SIZE)
                    }
                    OCCUPIED_BITS if self.count_adjacent_occupied_v2(x, y) >= 5 => {
                        // set empty
                        changed = true;
                        let index = self.get_index(x, y);
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
        if x > 0 && x <= self.width && y > 0 && y <= self.height {
            ((x - 1) + (y - 1) * self.width) * 2
        } else {
            usize::MAX
        }
    }

    fn get_seat(&self, x: usize, y: usize, default: u8) -> u8 {
        let index = self.get_index(x, y);

        if index == usize::MAX {
            default
        } else {
            (self.buckets[index / BUCKET_SIZE] >> (index % BUCKET_SIZE) & SEAT_MASK) as u8
        }
    }

    fn count_adjacent_occupied_v1(&self, x: usize, y: usize) -> usize {
        let count = (self.get_seat(x - 1, y - 1, FLOOR_BITS) & OCCUPIED_BITS)
            + (self.get_seat(x, y - 1, FLOOR_BITS) & OCCUPIED_BITS)
            + (self.get_seat(x + 1, y - 1, FLOOR_BITS) & OCCUPIED_BITS)
            + (self.get_seat(x - 1, y, FLOOR_BITS) & OCCUPIED_BITS)
            + (self.get_seat(x + 1, y, FLOOR_BITS) & OCCUPIED_BITS)
            + (self.get_seat(x - 1, y + 1, FLOOR_BITS) & OCCUPIED_BITS)
            + (self.get_seat(x, y + 1, FLOOR_BITS) & OCCUPIED_BITS)
            + (self.get_seat(x + 1, y + 1, FLOOR_BITS) & OCCUPIED_BITS);

        count as usize
    }

    fn count_adjacent_occupied_v2(&self, x: usize, y: usize) -> usize {
        let count = self.find_adjacent_seat(x, y, |x, y| (x - 1, y - 1))
            + self.find_adjacent_seat(x, y, |x, y| (x, y - 1))
            + self.find_adjacent_seat(x, y, |x, y| (x + 1, y - 1))
            + self.find_adjacent_seat(x, y, |x, y| (x - 1, y))
            + self.find_adjacent_seat(x, y, |x, y| (x + 1, y))
            + self.find_adjacent_seat(x, y, |x, y| (x - 1, y + 1))
            + self.find_adjacent_seat(x, y, |x, y| (x, y + 1))
            + self.find_adjacent_seat(x, y, |x, y| (x + 1, y + 1));

        count as usize
    }

    fn find_adjacent_seat<F>(&self, mut x: usize, mut y: usize, f: F) -> u8
    where
        F: Fn(usize, usize) -> (usize, usize),
    {
        loop {
            let next = f(x, y);
            x = next.0;
            y = next.1;

            match self.get_seat(x, y, EMPTY_BITS) {
                OCCUPIED_BITS => break OCCUPIED_BITS,
                EMPTY_BITS => break EMPTY_BITS,
                _ => {}
            }
        }
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
