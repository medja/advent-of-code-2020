use anyhow::anyhow;
use std::ops::{AddAssign, SubAssign};

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut east: isize = 0;
    let mut north: isize = 0;
    let mut direction = Direction::East;

    for line in input {
        let (action, amount) = Action::parse(line)?;

        match action {
            Action::East => east += amount,
            Action::South => north -= amount,
            Action::West => east -= amount,
            Action::North => north += amount,
            Action::Left => direction -= amount,
            Action::Right => direction += amount,
            Action::Forward => match direction {
                Direction::East => east += amount,
                Direction::South => north -= amount,
                Direction::West => east -= amount,
                Direction::North => north += amount,
            },
        }
    }

    Ok(east.abs() + north.abs())
}

enum Action {
    East,
    South,
    West,
    North,
    Left,
    Right,
    Forward,
}

impl Action {
    fn parse(string: &str) -> anyhow::Result<(Self, isize)> {
        let action = match string.as_bytes()[0] {
            b'E' => Action::East,
            b'S' => Action::South,
            b'W' => Action::West,
            b'N' => Action::North,
            b'L' => Action::Left,
            b'R' => Action::Right,
            b'F' => Action::Forward,
            _ => return Err(anyhow!("Invalid action {}", string)),
        };

        let amount = string[1..].parse()?;
        Ok((action, amount))
    }
}

enum Direction {
    East,
    South,
    West,
    North,
}

impl AddAssign<isize> for Direction {
    fn add_assign(&mut self, rhs: isize) {
        let direction = match self {
            Direction::East => 0,
            Direction::South => 90,
            Direction::West => 180,
            Direction::North => 270,
        };

        *self = match (direction + rhs) % 360 {
            0 => Direction::East,
            90 => Direction::South,
            180 => Direction::West,
            270 => Direction::North,
            direction => panic!("Invalid direction {}", direction),
        }
    }
}

impl SubAssign<isize> for Direction {
    fn sub_assign(&mut self, rhs: isize) {
        self.add_assign(360 - rhs);
    }
}
