pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut layout = SeatLayoutBuilder::new(input).build(false);
    while layout.simulate(false) {}
    Ok(layout.count_occupied())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut layout = SeatLayoutBuilder::new(input).build(true);
    while layout.simulate(true) {}
    Ok(layout.count_occupied())
}

struct Seat {
    index: usize,
    neighbors: [usize; 8],
}

impl Seat {
    fn new(index: usize, neighbors: [usize; 8]) -> Self {
        Seat { index, neighbors }
    }

    fn index(&self) -> usize {
        self.index
    }

    fn occupied_neighbors(&self, occupied: &[bool]) -> usize {
        self.neighbors
            .iter()
            .filter(|index| occupied[**index])
            .count()
    }
}

struct SeatLayout {
    seats: Vec<Seat>,
    occupied: Vec<bool>,
}

impl SeatLayout {
    fn new(width: usize, height: usize, seats: Vec<Seat>) -> Self {
        let occupied = vec![false; width * height + 1];
        SeatLayout { seats, occupied }
    }

    fn simulate(&mut self, extended: bool) -> bool {
        let limit = if extended { 5 } else { 4 };

        let mut changed = false;
        let mut occupied = self.occupied.clone();

        for seat in &self.seats {
            let count = seat.occupied_neighbors(&self.occupied);

            if self.occupied[seat.index()] {
                if count >= limit {
                    changed = true;
                    occupied[seat.index()] = false;
                }
            } else if count == 0 {
                changed = true;
                occupied[seat.index()] = true;
            }
        }

        self.occupied = occupied;
        changed
    }

    fn count_occupied(&self) -> usize {
        self.occupied.iter().filter(|occupied| **occupied).count()
    }
}

struct SeatLayoutBuilder {
    width: usize,
    height: usize,
    seats: Vec<bool>,
}

impl SeatLayoutBuilder {
    fn new(input: &[&str]) -> Self {
        let width = input[0].len();
        let height = input.len();

        let seats = input
            .iter()
            .flat_map(|line| line.bytes())
            .map(|byte| byte == b'L')
            .collect::<Vec<_>>();

        SeatLayoutBuilder {
            width,
            height,
            seats,
        }
    }

    fn build(&self, extended: bool) -> SeatLayout {
        let mut seats = Vec::with_capacity(self.seats.len());

        for y in 0..self.height {
            for x in 0..self.width {
                let index = x + y * self.width;

                if self.seats[index] {
                    let neighbors = if extended {
                        self.find_visible_neighbors(x, y, index)
                    } else {
                        self.find_direct_neighbors(x, y, index)
                    };

                    seats.push(Seat::new(index, neighbors));
                }
            }
        }

        SeatLayout::new(self.width, self.height, seats)
    }

    fn find_direct_neighbors(&self, x: usize, y: usize, index: usize) -> [usize; 8] {
        let mut count = 0;
        let mut neighbors = [self.width * self.height; 8];

        if y > 0 {
            let index = index - self.width;

            if x > 0 {
                neighbors[count] = index - 1;
                count += 1;
            }

            neighbors[count] = index;
            count += 1;

            if x + 1 < self.width {
                neighbors[count] = index + 1;
                count += 1;
            }
        }

        if x > 0 {
            neighbors[count] = index - 1;
            count += 1;
        }

        if x + 1 < self.width {
            neighbors[count] = index + 1;
            count += 1;
        }

        if y + 1 < self.height {
            let index = index + self.width;

            if x > 0 {
                neighbors[count] = index - 1;
                count += 1;
            }

            neighbors[count] = index;
            count += 1;

            if x + 1 < self.width {
                neighbors[count] = index + 1;
            }
        }

        neighbors
    }

    fn find_visible_neighbors(&self, x: usize, y: usize, index: usize) -> [usize; 8] {
        let mut count = 0;
        let mut neighbors = [self.width * self.height; 8];

        {
            let mut x = x;
            let mut y = y;
            let mut index = index;

            while x > 0 && y > 0 {
                x -= 1;
                y -= 1;
                index -= self.width + 1;

                if self.seats[index] {
                    neighbors[count] = index;
                    count += 1;
                    break;
                }
            }
        }

        {
            let mut y = y;
            let mut index = index;

            while y > 0 {
                y -= 1;
                index -= self.width;

                if self.seats[index] {
                    neighbors[count] = index;
                    count += 1;
                    break;
                }
            }
        }

        {
            let mut x = x;
            let mut y = y;
            let mut index = index;

            while x + 1 < self.width && y > 0 {
                x += 1;
                y -= 1;
                index -= self.width - 1;

                if self.seats[index] {
                    neighbors[count] = index;
                    count += 1;
                    break;
                }
            }
        }

        {
            let mut x = x;
            let mut index = index;

            while x > 0 {
                x -= 1;
                index -= 1;

                if self.seats[index] {
                    neighbors[count] = index;
                    count += 1;
                    break;
                }
            }
        }

        {
            let mut x = x;
            let mut index = index;

            while x + 1 < self.width {
                x += 1;
                index += 1;

                if self.seats[index] {
                    neighbors[count] = index;
                    count += 1;
                    break;
                }
            }
        }

        {
            let mut x = x;
            let mut y = y;
            let mut index = index;

            while x > 0 && y + 1 < self.height {
                x -= 1;
                y += 1;
                index += self.width - 1;

                if self.seats[index] {
                    neighbors[count] = index;
                    count += 1;
                    break;
                }
            }
        }

        {
            let mut y = y;
            let mut index = index;

            while y + 1 < self.height {
                y += 1;
                index += self.width;

                if self.seats[index] {
                    neighbors[count] = index;
                    count += 1;
                    break;
                }
            }
        }

        {
            let mut x = x;
            let mut y = y;
            let mut index = index;

            while x + 1 < self.width && y + 1 < self.height {
                x += 1;
                y += 1;
                index += self.width + 1;

                if self.seats[index] {
                    neighbors[count] = index;
                    break;
                }
            }
        }

        neighbors
    }
}

/*#[derive(Copy, Clone, Eq, PartialEq)]
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

    fn find_adjacent_seat(
        &self,
        mut x: usize,
        mut y: usize,
        f: impl Fn(usize, usize) -> (usize, usize),
    ) -> Seat {
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
}*/
