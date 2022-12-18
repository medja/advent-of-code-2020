use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut known_ingredients = HashSet::new();
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
            ingredients.retain(|ingredient| !known_ingredients.contains(ingredient));

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
            known_ingredients.insert(ingredient);
        }
    }

    Ok(unknown_ingredients.values().sum::<usize>())
}
