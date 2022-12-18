use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let (mut santa, mut crab) = parse_decks(input)?;

    while !santa.is_empty() && !crab.is_empty() {
        let santa_card = santa.draw_card();
        let crab_card = crab.draw_card();

        match santa_card.cmp(&crab_card) {
            Ordering::Less => crab.add_cards([crab_card, santa_card]),
            Ordering::Greater => santa.add_cards([santa_card, crab_card]),
            Ordering::Equal => unreachable!(),
        }
    }

    let mut winner = if santa.is_empty() { crab } else { santa };

    Ok(winner.score())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let (santa, crab) = parse_decks(input)?;
    Ok(play(santa, crab).1.score())
}

fn play(mut santa: Deck, mut crab: Deck) -> (bool, Deck) {
    let mut previous_decks = HashSet::<(Deck, Deck)>::new();

    while !santa.is_empty() && !crab.is_empty() {
        let key = (santa, crab);

        if previous_decks.contains(&key) {
            return (true, key.0);
        }

        previous_decks.insert(key.clone());
        (santa, crab) = key; // move back the card decks

        let santa_card = santa.draw_card();
        let crab_card = crab.draw_card();

        let recurse =
            santa_card as usize <= santa.card_count() && crab_card as usize <= crab.card_count();

        let santa_wins = if recurse {
            play(santa.copy(santa_card), crab.copy(crab_card)).0
        } else {
            santa_card.cmp(&crab_card) == Ordering::Greater
        };

        if santa_wins {
            santa.add_cards([santa_card, crab_card]);
        } else {
            crab.add_cards([crab_card, santa_card]);
        }
    }

    if santa.is_empty() {
        (false, crab)
    } else {
        (true, santa)
    }
}

fn parse_decks(input: &[&str]) -> anyhow::Result<(Deck, Deck)> {
    let index = input.iter().position(|line| line.is_empty()).unwrap();
    Ok((Deck::new(&input[..index])?, Deck::new(&input[index + 1..])?))
}

#[derive(Default, Clone, Eq, PartialEq, Hash)]
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

    fn card_count(&self) -> usize {
        self.0.len()
    }

    fn draw_card(&mut self) -> u8 {
        self.0.pop_front().unwrap()
    }

    fn add_cards(&mut self, cards: [u8; 2]) {
        self.0.push_back(cards[0]);
        self.0.push_back(cards[1]);
    }

    fn copy(&self, count: u8) -> Self {
        Deck(self.0.iter().take(count as usize).copied().collect())
    }

    fn score(&mut self) -> usize {
        std::iter::once(0)
            .chain(self.0.iter().rev().map(|card| *card as usize))
            .enumerate()
            .map(|(i, card)| i * card)
            .sum::<usize>()
    }
}
