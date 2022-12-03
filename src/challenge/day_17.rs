use std::iter::Iterator;

const SIZE: isize = 8;
const PADDING: isize = 6;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut pocket_dimension = PocketDimension::<3, { get_volume(3) }>::new(input);

    for _ in 0..6 {
        pocket_dimension.simulate();
    }

    Ok(pocket_dimension.count)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut pocket_dimension = PocketDimension::<4, { get_volume(4) }>::new(input);

    for _ in 0..6 {
        pocket_dimension.simulate();
    }

    Ok(pocket_dimension.count)
}

#[derive(Eq, PartialEq, Clone)]
struct Point<const D: usize>([isize; D]);

impl<const D: usize> Point<D> {
    fn index(&self) -> usize {
        self.0.iter()
            .enumerate()
            .rev()
            .fold(0, |sum, (index, value)| {
                sum * get_length(index) + value
            }) as usize
    }

    fn fill_neighbors(&self, points: &mut Vec<Self>) {
        points.clear();
        let this = self.clone();
        this.fill_neighbors_at_index(self, points, 0);
    }

    fn fill_neighbors_at_index(self, origin: &Self, points: &mut Vec<Self>, index: usize) {
        if index < D {
            let min = (self.0[index] - 1).max(0);
            let max = (self.0[index] + 1).min(get_length(index) - 1);

            for value in min..=max {
                self.update(index, value).fill_neighbors_at_index(origin, points, index + 1);
            }
        } else if origin != &self {
            points.push(self);
        }
    }

    fn update(&self, index: usize, value: isize) -> Self {
        let mut values = self.0.clone();
        values[index] = value;
        Point(values)
    }
}

struct PocketDimension<const D: usize, const V: usize> {
    count: usize,
    cubes: [bool; V],
    buffer: [bool; V],
}

impl<const D: usize, const V: usize> PocketDimension<D, V> {
    fn new(rows: &[&str]) -> Self {
        let mut count = 0;
        let mut cubes = [false; V];
        let buffer = [false; V];

        for (dy, row) in rows.iter().enumerate() {
            let y = dy as isize + PADDING;

            for (dx, state) in row.bytes().enumerate() {
                if state == b'#' {
                    let mut values = [PADDING; D];
                    values[0] = dx as isize + PADDING;
                    values[1] = y;
                    let point = Point(values);
                    count += 1;
                    cubes[point.index()] = true;
                }
            }
        }

        PocketDimension { count, cubes, buffer }
    }

    fn simulate(&mut self) {
        self.count = 0;
        self.buffer.fill(false);

        let mut neighbors = Vec::with_capacity(D.pow(D as u32) - 1);
        self.update_at_index(0, Point([0; D]), &mut neighbors);

        std::mem::swap(&mut self.cubes, &mut self.buffer)
    }

    fn update_at_index(&mut self, index: usize, point: Point<D>, neighbors: &mut Vec<Point<D>>) {
        if index == D {
            let index = point.index();
            point.fill_neighbors(neighbors);

            let count = neighbors
                .iter()
                .filter(|neighbor| self.cubes[neighbor.index()])
                .take(4)
                .count();

            if self.cubes[index] {
                if count == 2 || count == 3 {
                    self.count += 1;
                    self.buffer[index] = true;
                }
            } else {
                if count == 3 {
                    self.count += 1;
                    self.buffer[index] = true;
                }
            }
        } else {
            for value in 0..get_length(index) {
                self.update_at_index(index + 1, point.update(index, value), neighbors);
            }
        }
    }
}

const fn get_length(index: usize) -> isize {
    let base = if index < 2 { SIZE } else { 1 };
    base + 2 * PADDING
}

const fn get_volume(dimensions: usize) -> usize {
    let mut area = 1usize;
    let mut index = 0;

    while index < dimensions {
        area *= get_length(index) as usize;
        index += 1;
    }

    area
}
