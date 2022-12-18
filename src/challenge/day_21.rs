use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(solve(input).unknown_ingredients.values().sum::<usize>())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut known_ingredients = solve(input)
        .known_ingredients
        .into_iter()
        .collect::<Vec<_>>();

    known_ingredients.sort_by_key(|(_, allergen)| *allergen);

    let result = known_ingredients
        .iter()
        .fold(String::new(), |mut string, (ingredient, _)| {
            if !string.is_empty() {
                string.push(',');
            }

            string.push_str(ingredient);
            string
        });

    Ok(result)
}

struct Solution<'a> {
    known_ingredients: HashMap<&'a str, &'a str>,
    unknown_ingredients: HashMap<&'a str, usize>,
}

fn solve<'a>(input: &[&'a str]) -> Solution<'a> {
    let mut known_ingredients = HashMap::new();
    let mut unknown_ingredients = HashMap::new();
    let mut possible_ingredients = HashMap::<&str, HashSet<&str>>::new();

    for line in input {
        let (ingredients, allergens) = line.split_once(" (contains ").unwrap();
        let ingredients = ingredients.split_ascii_whitespace().collect::<HashSet<_>>();

        for allergen in allergens[..allergens.len() - 1].split(", ") {
            match possible_ingredients.entry(allergen) {
                Entry::Occupied(mut entry) => {
                    entry
                        .get_mut()
                        .retain(|ingredient| ingredients.contains(ingredient));
                }
                Entry::Vacant(entry) => {
                    entry.insert(ingredients.clone());
                }
            };
        }

        for ingredient in ingredients {
            match unknown_ingredients.entry(ingredient) {
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() += 1;
                }
                Entry::Vacant(entry) => {
                    entry.insert(1);
                }
            }
        }
    }

    let mut solved = Vec::new();

    while !possible_ingredients.is_empty() {
        for (allergen, ingredients) in &mut possible_ingredients {
            ingredients.retain(|ingredient| !known_ingredients.contains_key(ingredient));

            if ingredients.len() == 1 {
                solved.push(*allergen);
            }
        }

        for allergen in solved.drain(..) {
            let ingredient = possible_ingredients
                .remove(allergen)
                .unwrap()
                .into_iter()
                .next()
                .unwrap();

            unknown_ingredients.remove(ingredient);
            known_ingredients.insert(ingredient, allergen);
        }
    }

    Solution {
        known_ingredients,
        unknown_ingredients,
    }
}
