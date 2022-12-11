use anyhow::Context;
use regex::Regex;

const MAX_LOOPS: usize = 10;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let (rules, input) = parse_regex_and_input(input, false)?;
    Ok(input.iter().filter(|line| rules.is_match(line)).count())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let (rules, input) = parse_regex_and_input(input, true)?;
    Ok(input.iter().filter(|line| rules.is_match(line)).count())
}

fn parse_regex_and_input<'a>(
    input: &'a [&'a str],
    with_loops: bool,
) -> anyhow::Result<(Regex, &'a [&'a str])> {
    let index = input
        .iter()
        .position(|line| line.is_empty())
        .context("Input does not contain an empty line")?;

    Ok((
        parse_regex(&input[..index], with_loops)?,
        &input[index + 1..],
    ))
}

fn parse_regex(input: &[&str], with_loops: bool) -> anyhow::Result<Regex> {
    let mut rules = vec![""; input.len()];

    for line in input {
        let (id, line) = line
            .split_once(':')
            .with_context(|| format!("{} does not contain a :", line))?;

        rules[id.parse::<usize>()?] = &line[1..];
    }

    let mut builder = String::new();
    builder.push('^');
    build_rule(0, &rules, &mut builder, with_loops)?;
    builder.push('$');

    Ok(Regex::new(&builder)?)
}

fn build_rule(
    index: usize,
    rules: &[&str],
    builder: &mut String,
    with_loops: bool,
) -> anyhow::Result<()> {
    let rule = rules[index];

    if with_loops {
        if index == 8 {
            build_rule_8(rules, builder, with_loops)?;
            return Ok(());
        } else if index == 11 {
            build_rule_11(rules, builder, with_loops)?;
            return Ok(());
        }
    }

    if rule.starts_with('"') {
        builder.push(rule.chars().nth(1).context("Unexpected end of input")?);
        return Ok(());
    }

    match rule.split_once('|') {
        None => build_pattern(rule, rules, builder, with_loops)?,
        Some((first, second)) => {
            builder.push('(');
            build_pattern(first, rules, builder, with_loops)?;
            builder.push('|');
            build_pattern(second, rules, builder, with_loops)?;
            builder.push(')');
        }
    }

    Ok(())
}

fn build_pattern(
    pattern: &str,
    rules: &[&str],
    builder: &mut String,
    with_loops: bool,
) -> anyhow::Result<()> {
    for id in pattern.split_ascii_whitespace() {
        build_rule(id.parse()?, rules, builder, with_loops)?;
    }

    Ok(())
}

fn build_rule_8(rules: &[&str], builder: &mut String, with_loops: bool) -> anyhow::Result<()> {
    builder.push('(');
    build_rule(42, rules, builder, with_loops)?;
    builder.push(')');
    builder.push('+');
    Ok(())
}

fn build_rule_11(rules: &[&str], builder: &mut String, with_loops: bool) -> anyhow::Result<()> {
    let mut builder_42 = String::new();
    let mut builder_31 = String::new();
    build_rule(42, rules, &mut builder_42, with_loops)?;
    build_rule(31, rules, &mut builder_31, with_loops)?;

    builder.push('(');
    build_rule_11_combination(1, &builder_42, &builder_31, builder);

    for i in 2..MAX_LOOPS {
        builder.push('|');
        build_rule_11_combination(i, &builder_42, &builder_31, builder);
    }

    builder.push(')');

    Ok(())
}

fn build_rule_11_combination(count: usize, rule_42: &str, rule_31: &str, builder: &mut String) {
    builder.push('(');

    for _ in 0..count {
        builder.push_str(rule_42);
    }

    for _ in 0..count {
        builder.push_str(rule_31);
    }

    builder.push(')');
}
