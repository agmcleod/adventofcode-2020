use std::collections::{HashMap, HashSet};
use std::io::Result;

use read_input::read_text;

fn get_first_value_from_hashset(set: &HashSet<String>) -> String {
    set.iter().collect::<Vec<&String>>()[0].clone()
}

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

    // create a map of just the allergen ingredients
    let mut allergen_ingredients = HashMap::new();
    for (ing, allergens) in &ingredients_map {
        if allergens.len() > 0 {
            allergen_ingredients.insert(ing.to_owned(), allergens.clone());
        }
    }

    // keep track of what allergens we processed in p2 answer
    let mut processed_allergens = HashSet::new();
    loop {
        // get an allergen to remove from lists when it has a length of 1 and we havent processed it yet
        let allergen_to_process = allergen_ingredients.iter().find(|(_ing, allergens)| {
            allergens.len() == 1
                && !processed_allergens.contains(&get_first_value_from_hashset(allergens))
        });

        if allergen_to_process.is_none() {
            break;
        }

        let allergen = allergen_to_process
            .unwrap()
            .1
            .iter()
            .collect::<Vec<&String>>()[0]
            .clone();
        for (_ing, allergens) in &mut allergen_ingredients {
            if allergens.len() > 1 {
                allergens.remove(&allergen);
            }
        }

        processed_allergens.insert(allergen.to_owned());
    }

    let mut final_list = allergen_ingredients
        .iter()
        .map(|(ing, allergens)| (ing.to_owned(), get_first_value_from_hashset(allergens)))
        .collect::<Vec<(String, String)>>();

    final_list.sort_by(|a, b| a.1.cmp(&b.1));

    let final_list = final_list
        .iter()
        .map(|(ing, _)| ing.to_owned())
        .collect::<Vec<String>>();

    println!("{:?}", final_list.join(","));

    Ok(())
}
