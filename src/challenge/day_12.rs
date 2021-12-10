use anyhow::anyhow;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut east = 0isize;
    let mut north = 0isize;
    let mut direction = Direction::East;

    for line in input {
        let (action, amount) = Action::parse(line)?;

        match action {
            Action::East => east += amount,
            Action::South => north -= amount,
            Action::West => east -= amount,
            Action::North => north += amount,
            Action::Left => direction = direction.rotate(360 - amount),
            Action::Right => direction = direction.rotate(amount),
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

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut waypoint_east = 10isize;
    let mut waypoint_north = 1isize;
    let mut ship_east = 0isize;
    let mut ship_north = 0isize;

    for line in input {
        let (action, amount) = Action::parse(line)?;

        match action {
            Action::East => waypoint_east += amount,
            Action::South => waypoint_north -= amount,
            Action::West => waypoint_east -= amount,
            Action::North => waypoint_north += amount,
            Action::Left => rotate_point(&mut waypoint_north, &mut waypoint_east, 360 - amount),
            Action::Right => rotate_point(&mut waypoint_north, &mut waypoint_east, amount),
            Action::Forward => {
                ship_east += waypoint_east * amount;
                ship_north += waypoint_north * amount;
            }
        }
    }

    Ok(ship_east.abs() + ship_north.abs())
}

fn rotate_point(north: &mut isize, east: &mut isize, amount: isize) {
    match amount {
        90 => {
            let tmp = *north;
            *north = -*east;
            *east = tmp;
        }
        180 => {
            *east = -*east;
            *north = -*north;
        }
        270 => {
            let tmp = *north;
            *north = *east;
            *east = -tmp;
        }
        _ => {}
    }
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

impl Direction {
    fn rotate(self, angle: isize) -> Direction {
        let direction = match self {
            Direction::East => 0,
            Direction::South => 1,
            Direction::West => 2,
            Direction::North => 3,
        };

        match (direction + (angle / 90)) % 4 {
            0 => Direction::East,
            1 => Direction::South,
            2 => Direction::West,
            3 => Direction::North,
            _ => unreachable!(),
        }
    }
}
