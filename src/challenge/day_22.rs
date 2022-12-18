use std::cmp::Ordering;
use std::collections::VecDeque;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let (mut first, mut second) = parse_decks(input)?;

    while !first.is_empty() && !second.is_empty() {
        let first_card = first.draw_card();
        let second_card = second.draw_card();

        match first_card.cmp(&second_card) {
            Ordering::Less => second.add_cards([second_card, first_card]),
            Ordering::Greater => first.add_cards([first_card, second_card]),
            Ordering::Equal => unreachable!(),
        }
    }

    let mut winner = if first.is_empty() { second } else { first };

    Ok(winner.score())
}

fn parse_decks(input: &[&str]) -> anyhow::Result<(Deck, Deck)> {
    let index = input.iter().position(|line| line.is_empty()).unwrap();
    Ok((Deck::new(&input[..index])?, Deck::new(&input[index + 1..])?))
}

#[derive(Default)]
struct Deck(VecDeque<u8>);

impl Deck {
    fn new(cards: &[&str]) -> anyhow::Result<Self> {
        Ok(Deck(
            cards[1..]
                .iter()
                .map(|card| card.parse())
                .collect::<Result<_, _>>()?,
        ))
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn draw_card(&mut self) -> u8 {
        self.0.pop_front().unwrap()
    }

    fn add_cards(&mut self, cards: [u8; 2]) {
        self.0.push_back(cards[0]);
        self.0.push_back(cards[1]);
    }

    fn score(&mut self) -> usize {
        std::iter::once(0)
            .chain(self.0.iter().rev().map(|card| *card as usize))
            .enumerate()
            .map(|(i, card)| i * card)
            .sum::<usize>()
    }
}
