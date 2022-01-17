use std::collections::{HashMap, HashSet};
use std::io::Result;

use read_input::read_text;

fn main() -> Result<()> {
    let text = read_text("21/input.txt")?;

    let mut singular_allergens = HashMap::new();
    let mut ingredients_map = HashMap::new();
    let mut allergens_map = HashMap::new();

    let mut list = Vec::new();
    for line in text.lines() {
        let mut iter = line.split(" (");
        let ingredients = iter
            .next()
            .unwrap()
            .split(" ")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        let allergens = iter
            .next()
            .unwrap()
            .replace(")", "")
            .replace("contains ", "");
        let allergens = allergens
            .split(", ")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        for allergen in &allergens {
            let map = if allergens_map.contains_key(allergen) {
                allergens_map.get_mut(allergen).unwrap()
            } else {
                allergens_map.insert(allergen.to_owned(), HashSet::new());
                allergens_map.get_mut(allergen).unwrap()
            };
            for ing in &ingredients {
                map.insert(ing.to_owned());
            }
        }

        // TODO: need to update this to check for exclusions.
        // If we encounter a line again for the same single allergen, both lines must include the ingredient
        if allergens.len() == 1 {
            let allergen_key = allergens.get(0).unwrap();
            if singular_allergens.contains_key(allergen_key) {
                let set: &mut HashSet<String> = singular_allergens.get_mut(allergen_key).unwrap();
                for ing in &ingredients {
                    set.insert(ing.clone());
                }
            } else {
                let mut set = HashSet::new();
                for ing in &ingredients {
                    set.insert(ing.clone());
                }
                singular_allergens.insert(allergen_key.to_owned(), set);
            }
        }

        for ing in &ingredients {
            for allergen in &allergens {
                let map = if ingredients_map.contains_key(ing) {
                    ingredients_map.get_mut(ing).unwrap()
                } else {
                    ingredients_map.insert(ing.to_owned(), HashSet::new());
                    ingredients_map.get_mut(ing).unwrap()
                };

                map.insert(allergen.to_owned());
            }
        }

        list.push((ingredients, allergens));
    }

    // go through each allergen to figure out its potential owners
    for (allergen, allergen_ingredients) in &allergens_map {
        // get the list of ingredients where one of these MUST include the allergen
        let required_ingredients = singular_allergens.get(allergen).unwrap();
        let mut allergen_count_in_lines = 0;
        for line in &list {
            // if the line includes the current allergen
            if line.1.contains(allergen) {
                allergen_count_in_lines += 1;

                // check if the ingredients align with the required ones
                for ingredient in &line.0 {
                    if !required_ingredients.contains(ingredient) {
                        // cannot contain the current allergen
                        ingredients_map
                            .get_mut(ingredient)
                            .unwrap()
                            .remove(allergen);
                    }
                }
            }
        }

        // see how many ingredients are using this allergen now
        let mut allergen_usage_count = 0;
        let mut ingredient_holding_allergen = String::new();
        for ing in allergen_ingredients {
            if ingredients_map.get(ing).unwrap().contains(allergen) {
                allergen_usage_count += 1;
                // store the ingredient key so we can clear out other allergens after
                // will ovewrite in loop, but if count is > 1, doesn't matter anyways
                ingredient_holding_allergen = ing.clone();
            }
        }

        // if count is 1, remove any other allergens from that ingredient
        if allergen_usage_count == 1 {
            let mut new_set = HashSet::new();
            new_set.insert(allergen.to_owned());
            // replace the set with one just holding the single allergen
            ingredients_map.insert(ingredient_holding_allergen, new_set);
        }
    }

    println!("{:?}", ingredients_map);

    Ok(())
}
