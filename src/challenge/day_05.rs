use anyhow::Context;
use std::ops::Add;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input.iter().map(|line| Seat::new(line)).max().unwrap().id())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut seats = input.iter().map(|line| Seat::new(line)).collect::<Vec<_>>();
    seats.sort();
    let missing_seat = find_missing_seat(seats).context("Cannot find missing seat")?;
    Ok(missing_seat.id())
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Seat(u16);

impl Seat {
    fn new(string: &str) -> Self {
        Seat(string.as_bytes().iter().fold(0u16, |acc, char| {
            acc << 1 | (matches!(char, b'B' | b'R') as u16)
        }))
    }

    fn id(&self) -> u16 {
        self.0
    }
}

impl Add<u16> for Seat {
    type Output = Seat;

    fn add(self, rhs: u16) -> Self::Output {
        Seat(self.0 + rhs)
    }
}

fn find_missing_seat<I>(seats: I) -> Option<Seat>
where
    I: IntoIterator<Item = Seat>,
{
    let mut iterator = seats.into_iter();
    let mut previous: Seat = iterator.next()?;

    for next in iterator {
        if previous + 2 == next {
            return Some(previous + 1);
        }

        previous = next;
    }

    None
}
