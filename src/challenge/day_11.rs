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

#[derive(Copy, Clone, Eq, PartialEq)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

struct SeatLayout {
    width: usize,
    height: usize,
    seats: Vec<Seat>,
}

impl SeatLayout {
    fn new(input: &[&str]) -> Self {
        let width = input[0].len();
        let height = input.len();

        let values = input.iter().flat_map(|line| line.bytes()).map(|char| {
            if char == b'L' {
                Seat::Empty
            } else {
                Seat::Floor
            }
        });

        let mut seats = Vec::with_capacity(width * height);
        seats.extend(values);

        SeatLayout {
            width,
            height,
            seats,
        }
    }

    fn count_occupied(&self) -> usize {
        self.seats
            .iter()
            .filter(|seat| **seat == Seat::Occupied)
            .count()
    }

    fn simulate_v1(&mut self) -> bool {
        let mut changed = false;
        let mut seats = self.seats.clone();

        for y in 1..=self.height {
            for x in 1..=self.width {
                match self.get_seat(x, y, Seat::Floor) {
                    Seat::Empty if self.count_adjacent_occupied(x, y) == 0 => {
                        changed = true;
                        seats[(x - 1) + (y - 1) * self.width] = Seat::Occupied;
                    }
                    Seat::Occupied if self.count_adjacent_occupied(x, y) >= 4 => {
                        changed = true;
                        seats[(x - 1) + (y - 1) * self.width] = Seat::Empty;
                    }
                    _ => {}
                }
            }
        }

        self.seats = seats;
        changed
    }

    fn simulate_v2(&mut self) -> bool {
        let mut changed = false;
        let mut seats = self.seats.clone();

        for y in 1..=self.height {
            for x in 1..=self.width {
                match self.get_seat(x, y, Seat::Floor) {
                    Seat::Empty if self.count_visible_occupied(x, y) == 0 => {
                        changed = true;
                        seats[(x - 1) + (y - 1) * self.width] = Seat::Occupied;
                    }
                    Seat::Occupied if self.count_visible_occupied(x, y) >= 5 => {
                        changed = true;
                        seats[(x - 1) + (y - 1) * self.width] = Seat::Empty;
                    }
                    _ => {}
                }
            }
        }

        self.seats = seats;
        changed
    }

    fn get_seat(&self, x: usize, y: usize, default: Seat) -> Seat {
        if x > 0 && x <= self.width && y > 0 && y <= self.height {
            self.seats[(x - 1) + (y - 1) * self.width]
        } else {
            default
        }
    }

    fn count_adjacent_occupied(&self, x: usize, y: usize) -> usize {
        (self.get_seat(x - 1, y - 1, Seat::Floor) == Seat::Occupied) as usize
            + (self.get_seat(x, y - 1, Seat::Floor) == Seat::Occupied) as usize
            + (self.get_seat(x + 1, y - 1, Seat::Floor) == Seat::Occupied) as usize
            + (self.get_seat(x - 1, y, Seat::Floor) == Seat::Occupied) as usize
            + (self.get_seat(x + 1, y, Seat::Floor) == Seat::Occupied) as usize
            + (self.get_seat(x - 1, y + 1, Seat::Floor) == Seat::Occupied) as usize
            + (self.get_seat(x, y + 1, Seat::Floor) == Seat::Occupied) as usize
            + (self.get_seat(x + 1, y + 1, Seat::Floor) == Seat::Occupied) as usize
    }

    fn count_visible_occupied(&self, x: usize, y: usize) -> usize {
        (self.find_adjacent_seat(x, y, |x, y| (x - 1, y - 1)) == Seat::Occupied) as usize
            + (self.find_adjacent_seat(x, y, |x, y| (x, y - 1)) == Seat::Occupied) as usize
            + (self.find_adjacent_seat(x, y, |x, y| (x + 1, y - 1)) == Seat::Occupied) as usize
            + (self.find_adjacent_seat(x, y, |x, y| (x - 1, y)) == Seat::Occupied) as usize
            + (self.find_adjacent_seat(x, y, |x, y| (x + 1, y)) == Seat::Occupied) as usize
            + (self.find_adjacent_seat(x, y, |x, y| (x - 1, y + 1)) == Seat::Occupied) as usize
            + (self.find_adjacent_seat(x, y, |x, y| (x, y + 1)) == Seat::Occupied) as usize
            + (self.find_adjacent_seat(x, y, |x, y| (x + 1, y + 1)) == Seat::Occupied) as usize
    }

    fn find_adjacent_seat<F>(&self, mut x: usize, mut y: usize, f: F) -> Seat
    where
        F: Fn(usize, usize) -> (usize, usize),
    {
        loop {
            let next = f(x, y);
            x = next.0;
            y = next.1;

            let seat = self.get_seat(x, y, Seat::Empty);

            if seat != Seat::Floor {
                break seat;
            }
        }
    }
}
