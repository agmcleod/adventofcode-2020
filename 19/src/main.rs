use std::collections::HashMap;

use regex::Regex;

use read_input::read_text;

fn main() {
    let text = read_text("19/input.txt").unwrap();

    let mut rules = HashMap::new();
    let mut rules_to_process = HashMap::new();
    let mut reading_messages = false;

    let mut messages = Vec::new();

    for line in text.lines() {
        if line == "" {
            reading_messages = true;
            continue;
        }

        if reading_messages {
            messages.push(line.to_string());
        } else {
            let mut split = line.split(": ");
            let num = split.next().unwrap();
            let result = split.next().unwrap();
            if result.contains("\"") {
                rules.insert(num, result.replace("\"", ""));
            } else {
                rules_to_process.insert(num, result.to_string());
            }
        }
    }

    let mut keys = rules_to_process.keys().map(|k| *k).collect::<Vec<&str>>();

    let numbers_re = Regex::new(r"\d+").unwrap();

    loop {
        let mut index = 0;
        while index < keys.len() {
            let references = rules_to_process.get(&keys[index]).unwrap();

            let mut regex_pattern = references.clone();
            let mut captures_to_replace = Vec::new();

            let mut matched_all = true;

            for cap in numbers_re.captures_iter(references) {
                let value = cap.get(0).unwrap().as_str();
                if rules.contains_key(value) {
                    captures_to_replace.push(value.clone());
                } else {
                    matched_all = false;
                }
            }

            if matched_all {
                for cap in &captures_to_replace {
                    regex_pattern = regex_pattern.replacen(*cap, rules.get(*cap).unwrap(), 1);
                }

                regex_pattern = regex_pattern.replace(" ", "");
                // if this reference level contained a pipe, use braces
                if references.contains("|") {
                    regex_pattern = format!("({})", regex_pattern);
                }
                rules.insert(keys[index].clone(), regex_pattern);

                keys.remove(index);
            } else {
                index += 1;
            }
        }

        if keys.len() == 0 {
            break;
        }
    }

    let zero_re = Regex::new(&format!("^{}$", rules.get("0").unwrap())).unwrap();

    let matched = messages.iter().fold(0, |sum, message| {
        if zero_re.is_match(message) {
            return sum + 1;
        }
        sum
    });

    println!("{}", matched);
}
