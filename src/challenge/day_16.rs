use anyhow::Context;
use std::ops::{Index, RangeInclusive};
use std::str::FromStr;

const FIELD_COUNT: usize = 20;
const FIELDS_MASK: u32 = 0b11111111111111111111;
const DEPARTURE_FIELD_COUNT: usize = 6;
const MY_TICKET_OFFSET: usize = FIELD_COUNT + 2;
const OTHER_TICKETS_OFFSET: usize = MY_TICKET_OFFSET + 3;
const INVALID_RULE: &str = "Rule is invalid";

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let rules = parse_rules(&input[0..FIELD_COUNT])?;
    let mut error_rate = 0usize;

    for ticket in &input[OTHER_TICKETS_OFFSET..] {
        if let Some(error) = ticket.parse::<Ticket>()?.validate(&rules) {
            error_rate += error;
        }
    }

    Ok(error_rate)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let rules = parse_rules(&input[0..FIELD_COUNT])?;
    let mut possible_fields = [FIELDS_MASK; FIELD_COUNT];

    let my_ticket = input[MY_TICKET_OFFSET].parse::<Ticket>()?;
    check_ticket(&my_ticket, &rules, &mut possible_fields);

    for ticket in &input[OTHER_TICKETS_OFFSET..] {
        let ticket = ticket.parse::<Ticket>()?;

        if ticket.validate(&rules).is_none() {
            check_ticket(&ticket, &rules, &mut possible_fields);
        }
    }

    let departure_field_lookup = build_departure_field_lookup(&possible_fields);

    Ok(departure_field_lookup
        .iter()
        .map(|index| my_ticket[*index])
        .product::<usize>())
}

fn check_ticket(ticket: &Ticket, rules: &[Rule], possible_fields: &mut [u32; FIELD_COUNT]) {
    for (rule_index, rule) in rules.iter().enumerate() {
        for (field_index, value) in ticket.iter().enumerate() {
            if !rule.matches(value) {
                possible_fields[rule_index] &= !(1 << field_index)
            }
        }
    }
}

fn build_departure_field_lookup(
    possible_fields: &[u32; FIELD_COUNT],
) -> [usize; DEPARTURE_FIELD_COUNT] {
    let mut index = 0usize;

    let mut lookup = possible_fields.map(|mask| {
        let pair = (index, mask.count_ones());
        index += 1;
        pair
    });

    lookup.sort_unstable_by(|(_, x), (_, y)| x.cmp(y));

    let mut matched_fields_mask = 0u32;
    let mut departure_field_lookup = [0usize; DEPARTURE_FIELD_COUNT];

    for (rule_index, _) in lookup {
        // assumes we solved the problem and mark has one bit
        let mask = possible_fields[rule_index] & !matched_fields_mask;
        matched_fields_mask |= mask;

        if rule_index < DEPARTURE_FIELD_COUNT {
            departure_field_lookup[rule_index] = mask.trailing_zeros() as usize;
        }
    }

    departure_field_lookup
}

fn parse_rules(rules: &[&str]) -> anyhow::Result<Vec<Rule>> {
    let rules = rules
        .iter()
        .map(|line| line.parse())
        .collect::<Result<Vec<Rule>, _>>()?;

    Ok(rules)
}

struct Ticket([usize; FIELD_COUNT]);

impl Ticket {
    fn validate(&self, rules: &[Rule]) -> Option<usize> {
        self.0
            .iter()
            .find(|value| !rules.iter().any(|rule| rule.matches(value)))
            .cloned()
    }

    fn iter(&self) -> impl Iterator<Item = &usize> {
        self.0.iter()
    }
}

impl Index<usize> for Ticket {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl FromStr for Ticket {
    type Err = anyhow::Error;

    fn from_str(ticket: &str) -> Result<Self, Self::Err> {
        let mut values = [0usize; FIELD_COUNT];

        for (i, value) in ticket.split(',').enumerate() {
            values[i] = value.parse()?;
        }

        Ok(Ticket(values))
    }
}

struct Rule(RangeInclusive<usize>, RangeInclusive<usize>);

impl Rule {
    fn matches(&self, value: &usize) -> bool {
        self.0.contains(value) || self.1.contains(value)
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line = &line.split_once(':').context(INVALID_RULE)?.1;

        let (a, line) = line[1..].split_once('-').context(INVALID_RULE)?;
        let (b, line) = line.split_once(' ').context(INVALID_RULE)?;
        let (c, d) = line[3..].split_once('-').context(INVALID_RULE)?;

        Ok(Rule(a.parse()?..=b.parse()?, c.parse()?..=d.parse()?))
    }
}
