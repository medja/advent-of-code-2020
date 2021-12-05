use anyhow::Context;
use std::collections::{HashMap, HashSet};

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut graph = HashMap::<&str, Vec<&str>>::new();

    for line in input {
        let (child, mut rest) = parse_color(line)?;

        while let Some(line) = seek_color(rest) {
            let (parent, buf) = parse_color(line)?;
            rest = buf;
            graph.entry(parent).or_default().push(child);
        }
    }

    let mut visited = HashSet::new();
    visit_children("shiny gold", &graph, &mut visited);

    Ok(visited.len())
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

fn seek_color(string: &str) -> Option<&str> {
    string
        .bytes()
        .enumerate()
        .skip_while(|(_, char)| !char.is_ascii_digit())
        .skip_while(|(_, char)| *char != b' ')
        .nth(1)
        .map(|(i, _)| &string[i..])
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
