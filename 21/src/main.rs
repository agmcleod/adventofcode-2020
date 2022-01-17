use std::collections::{HashMap, HashSet};
use std::io::Result;

use read_input::read_text;

fn main() -> Result<()> {
    let text = read_text("21/input.txt")?;

    let mut singular_allergens = HashMap::new();

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

        if allergens.len() == 1 {
            singular_allergens.insert(allergens[0].clone(), ingredients.clone());
        }

        list.push((ingredients, allergens));
    }

    let mut no_allergen_ingredients = Vec::new();

    for (i, line) in list.iter().enumerate() {
        // might be able to do log(n) instead of o(n)^2 here
        for (j, line2) in list.iter().enumerate() {
            if i == j {
                continue;
            }
        }
    }

    // let mut scanned_ingredients = HashSet::new();
    // let mut no_allergen_ingredients = Vec::new();
    // for line in &list {
    //     if line.1.len() == 1 {
    //         continue;
    //     }
    //     // check each ingredient on this line
    //     for ing in &line.0 {
    //         // only check it if we have yet to scan it
    //         if !scanned_ingredients.contains(ing) {
    //             // check if the ingredient is part of other lines including just each allergen individually
    //             let count_in_singular = line.1.iter().fold(0, |sum, allergen| {
    //                 if let Some(other_ingr) = singular_allergens.get(allergen) {
    //                     sum + other_ingr.iter().filter(|v| *v == ing).count()
    //                 } else {
    //                     sum
    //                 }
    //             });

    //             if count_in_singular == 0 {
    //                 no_allergen_ingredients.push(ing.clone());
    //             }

    //             scanned_ingredients.insert(ing.clone());
    //         }
    //     }
    // }

    // println!("{:?}", no_allergen_ingredients);

    Ok(())
}
