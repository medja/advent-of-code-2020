use anyhow::Context;
use std::collections::{HashMap, HashSet};

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut graph = HashMap::<&str, Vec<&str>>::new();

    for line in input {
        let (child, mut rest) = parse_color(line)?;

        while let Some((_, line)) = parse_child(rest) {
            let (parent, buf) = parse_color(line)?;
            rest = buf;
            graph.entry(parent).or_default().push(child);
        }
    }

    let mut visited = HashSet::new();
    visit_children("shiny gold", &graph, &mut visited);

    Ok(visited.len())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut graph = HashMap::<&str, Entry>::new();

    for line in input {
        let (parent, mut rest) = parse_color(line)?;

        while let Some((count, line)) = parse_child(rest) {
            let count = count.parse()?;
            let (name, buf) = parse_color(line)?;
            rest = buf;

            graph
                .entry(parent)
                .or_default()
                .children
                .push(Child { name, count });
        }
    }

    Ok(count_children("shiny gold", &mut graph))
}

#[derive(Default)]
struct Entry<'a> {
    children: Vec<Child<'a>>,
    total_children: Option<usize>,
}

#[derive(Clone)]
struct Child<'a> {
    name: &'a str,
    count: usize,
}

fn parse_color(string: &str) -> anyhow::Result<(&str, &str)> {
    string
        .bytes()
        .enumerate()
        .filter(|(_, char)| *char == b' ')
        .nth(1)
        .map(|(i, _)| string.split_at(i))
        .context("Cannot parse color")
}

fn parse_child(string: &str) -> Option<(&str, &str)> {
    let (start, _) = string
        .bytes()
        .enumerate()
        .find(|(_, char)| char.is_ascii_digit())?;

    let (i, _) = string[start..]
        .bytes()
        .enumerate()
        .find(|(_, char)| *char == b' ')?;

    let end = start + i;

    Some((&string[start..end], &string[end + 1..]))
}

fn visit_children<'a>(
    parent: &str,
    graph: &HashMap<&str, Vec<&'a str>>,
    visited: &mut HashSet<&'a str>,
) {
    let children = match graph.get(parent) {
        Some(children) => children,
        None => return,
    };

    for child in children {
        if visited.insert(child) {
            visit_children(child, graph, visited);
        }
    }
}

fn count_children(parent: &str, graph: &mut HashMap<&str, Entry>) -> usize {
    let entry = match graph.get(parent) {
        Some(entry) => entry,
        None => return 0,
    };

    if let Some(total_children) = entry.total_children {
        return total_children;
    }

    let total_children = entry
        .children
        .clone()
        .iter()
        .map(|child| child.count * (1 + count_children(child.name, graph)))
        .sum();

    if let Some(entry) = graph.get_mut(parent) {
        entry.total_children = Some(total_children);
    }

    total_children
}
