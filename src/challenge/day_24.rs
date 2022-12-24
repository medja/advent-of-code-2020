use std::slice::Windows;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(BlackTiles::new(&parse_position(input)).count())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let positions = parse_position(input);
    let black_tiles = BlackTiles::new(&positions).map(normalize);
    let mut lobby = Lobby::new(150, 200, black_tiles);

    for _ in 0..99 {
        lobby.update();
    }

    Ok(lobby.update())
}

struct Lobby {
    width: usize,
    tiles: Vec<bool>,
    updated: Vec<usize>,
}

impl Lobby {
    fn new(width: usize, height: usize, black_tiles: impl Iterator<Item = (i8, i8)>) -> Self {
        let mut dx = (width / 2) as isize;
        let dy = (height / 2) as isize;

        if dx % 2 == 1 {
            dx += 1;
        }

        let mut tiles = vec![false; width * height];

        for (x, y) in black_tiles {
            tiles[(x as isize + dx + (y as isize + dy) * width as isize) as usize] = true;
        }

        Lobby {
            width,
            tiles,
            updated: Vec::new(),
        }
    }

    fn update(&mut self) -> usize {
        let mut count = 0;

        // skips the first and last row + 1 tile
        for index in self.width + 1..self.tiles.len() - self.width - 1 {
            let black_neighbors = self.count_black_neighbors(index);

            if self.tiles[index] {
                if black_neighbors == 0 || black_neighbors > 2 {
                    self.updated.push(index);
                } else {
                    count += 1;
                }
            } else if black_neighbors == 2 {
                self.updated.push(index);
                count += 1;
            }
        }

        for index in self.updated.drain(..) {
            self.tiles[index] = !self.tiles[index];
        }

        count
    }

    fn count_black_neighbors(&self, index: usize) -> usize {
        let neighbors = if (index / self.width) % 2 == 0 {
            [
                index + 1,
                index - 1,
                index + self.width,
                index + self.width - 1,
                index - self.width,
                index - self.width - 1,
            ]
        } else {
            [
                index + 1,
                index - 1,
                index + self.width,
                index + self.width + 1,
                index - self.width,
                index - self.width + 1,
            ]
        };

        neighbors.iter().filter(|index| self.tiles[**index]).count()
    }
}

struct BlackTiles<'a>(usize, Windows<'a, (i8, i8)>, (i8, i8));

impl<'a> BlackTiles<'a> {
    fn new(positions: &'a [(i8, i8)]) -> Self {
        BlackTiles(1, positions.windows(2), *positions.last().unwrap())
    }
}

impl Iterator for BlackTiles<'_> {
    type Item = (i8, i8);

    fn next(&mut self) -> Option<Self::Item> {
        for window in &mut self.1 {
            let previous = window[0];
            let current = window[1];

            if previous == current {
                self.0 += 1;
                continue;
            }

            let is_black = self.0 % 2 == 1;
            self.0 = 1;

            if is_black {
                return Some(previous);
            }
        }

        let is_black = self.0 % 2 == 1;
        self.0 = 0;

        if is_black {
            Some(self.2)
        } else {
            None
        }
    }
}

fn parse_position(input: &[&str]) -> Vec<(i8, i8)> {
    let mut positions = input.iter().map(|line| locate(line)).collect::<Vec<_>>();
    positions.sort();
    positions
}

fn normalize(position: (i8, i8)) -> (i8, i8) {
    let x = if position.0 % 2 != 0 {
        (position.0 - 1) / 2
    } else {
        position.0 / 2
    };

    (x, position.1)
}

fn locate(input: &str) -> (i8, i8) {
    let input = input.as_bytes();
    let (mut x, mut y) = (0, 0);
    let mut i = 0;

    while i < input.len() {
        let char = input[i];
        i += 1;

        match char {
            b'e' => {
                x -= 2;
                continue;
            }
            b'w' => {
                x += 2;
                continue;
            }
            b's' => y -= 1,
            b'n' => y += 1,
            _ => unreachable!(),
        }

        match input[i] {
            b'e' => x -= 1,
            b'w' => x += 1,
            _ => unreachable!(),
        }

        i += 1;
    }

    (x, y)
}
