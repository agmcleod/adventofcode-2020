use std::collections::{HashMap, HashSet};
use std::io::Result;

use read_input::read_text;

fn main() -> Result<()> {
    let text = read_text("21/input.txt")?;

    // let mut singular_allergens = HashMap::new();
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

        // map ingredients to each allergen, so we can grab all ingredients by allergen.
        for allergen in &allergens {
            let map = if allergens_map.contains_key(allergen) {
                allergens_map.get_mut(allergen).unwrap()
            } else {
                allergens_map.insert(allergen.to_owned(), HashSet::new());
                allergens_map.get_mut(allergen).unwrap()
            };
            for ing in &ingredients {
                map.insert(ing.to_owned());

                // also map the opposite direction of an ingredient to possible allergens
                let map = if ingredients_map.contains_key(ing) {
                    ingredients_map.get_mut(ing).unwrap()
                } else {
                    ingredients_map.insert(ing.to_owned(), HashSet::new());
                    ingredients_map.get_mut(ing).unwrap()
                };

                map.insert(allergen.to_owned());
            }
        }

        // if it's a singular allergen for the line of ingredients, we want to add it to a set
        // this is a source of truth for what ingredients must be allocated to a specific allergen
        // if allergens.len() == 1 {
        //     let allergen_key = allergens.get(0).unwrap();
        //     if singular_allergens.contains_key(allergen_key) {
        //         let set = singular_allergens.get(allergen_key).unwrap();
        //         let mut new_set = HashSet::new();
        //         // go through existing set, keep if both lines have it
        //         for ing in set {
        //             if ingredients.contains(ing) {
        //                 new_set.insert(ing.to_owned());
        //             }
        //         }
        //         singular_allergens.insert(allergen_key.to_owned(), new_set);
        //     } else {
        //         let mut set = HashSet::new();
        //         for ing in &ingredients {
        //             set.insert(ing.clone());
        //         }
        //         singular_allergens.insert(allergen_key.to_owned(), set);
        //     }
        // }

        list.push((ingredients, allergens));
    }

    // go through each allergen to figure out its potential owners
    for (allergen, allergen_ingredients) in &allergens_map {
        // TODO: need to rework this to check if an ingredient is in all recipes for a given allergen.
        let mut possible_ingredients = HashSet::new();
        for line in &list {
            // if the line includes the current allergen
            if line.1.contains(allergen) {
                let mut next_set_possible_ingredients = HashSet::new();
                for ingredient in &line.0 {
                    // have yet to populate the hash set, so just do inserts
                    if possible_ingredients.len() == 0 {
                        next_set_possible_ingredients.insert(ingredient.to_owned());
                    } else {
                        // update the hashset to only have ones that are consistent.
                        if possible_ingredients.contains(ingredient) {
                            next_set_possible_ingredients.insert(ingredient.to_owned());
                        }
                    }
                }

                possible_ingredients = next_set_possible_ingredients;
            }
        }

        // go through each ingredient of the allergen, and remove it from the map if its not in our possible hash set
        for ingredient in allergen_ingredients {
            if !possible_ingredients.contains(ingredient) {
                ingredients_map
                    .get_mut(ingredient)
                    .unwrap()
                    .remove(allergen);
            }
        }
    }

    let allergen_free_ingredients = ingredients_map
        .iter()
        .filter(|(_ing, allergen)| allergen.len() == 0)
        .map(|(ing, _allergen)| ing)
        .collect::<Vec<&String>>();

    let mut counter = 0;
    for line in &list {
        for ing in &allergen_free_ingredients {
            if line.0.contains(ing) {
                counter += 1;
            }
        }
    }

    println!("{}", counter);

    Ok(())
}
