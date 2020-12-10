use std::collections::{HashMap, HashSet};

use read_input::read_text;

fn main() {
    let text = read_text("7/input.txt").unwrap();

    let mut bags: HashMap<String, Vec<(usize, String)>> = HashMap::new();
    let mut bags_to_parent: HashMap<String, Vec<String>> = HashMap::new();

    for line in text.lines() {
        let mut iter = line.splitn(2, " bags contain ");
        let container_colour = iter.next().unwrap().replace(" bags", "").replace(" bag", "");
        let children = iter.next().unwrap();
        let mut child_bags = Vec::new();
        for child in children.replace(".", "").split(", ") {
            let mut iter = child.splitn(2, " ");
            if let Ok(n) = iter.next().unwrap().parse::<usize>() {
                let child_bag_name = iter.next().unwrap().replace(" bags", "").replace(" bag", "");

                child_bags.push((n, child_bag_name.to_string()));
                if bags_to_parent.contains_key(&child_bag_name) {
                    bags_to_parent.get_mut(&child_bag_name).unwrap().push(container_colour.clone());
                } else {
                    bags_to_parent.insert(child_bag_name, vec![container_colour.clone()]);
                }
            }

        }

        bags.insert(container_colour.to_string(), child_bags);
    }

    let mut bags_that_contain_shiny_gold = HashSet::new();
    let mut targets = vec!["shiny gold".to_string()];

    loop {
        let target = targets.pop();
        if target.is_none() {
            break
        }

        let target = target.unwrap();
        if let Some(parents) = bags_to_parent.get(&target) {
            for parent in parents {
                if !bags_that_contain_shiny_gold.contains(parent) {
                    targets.push(parent.clone());
                }
            }
        }

        if target != "shiny gold" && !bags_that_contain_shiny_gold.contains(&target) {
            bags_that_contain_shiny_gold.insert(target);
        }
    }

    println!("{}", bags_that_contain_shiny_gold.len());

    let mut targets = bags.get("shiny gold").unwrap().clone();
    let mut sum = 0;

    loop {
        let target = targets.pop();
        if target.is_none() {
            break
        }

    }
}
