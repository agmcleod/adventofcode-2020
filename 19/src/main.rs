use std::collections::HashMap;

use regex::Regex;

use read_input::read_text;

fn create_regex_rules<'a>(
    rules_to_process: &'a HashMap<&'a str, String>,
    starting_values: &'a HashMap<&'a str, String>,
    use_p2_rules: bool,
) -> HashMap<&'a str, String> {
    let mut rules = HashMap::new();
    let mut keys = rules_to_process.keys().map(|k| *k).collect::<Vec<&str>>();

    for (num, starting_value) in starting_values {
        rules.insert(*num, starting_value.clone());
    }

    let numbers_re = Regex::new(r"\d+").unwrap();

    loop {
        let mut index = 0;
        while index < keys.len() {
            let num = keys[index];
            let references = rules_to_process.get(&num).unwrap();

            let mut regex_pattern = references.clone();
            let mut captures_to_replace = Vec::new();

            let mut matched_all = true;

            // check if we have these rules for this line built
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

                if use_p2_rules && (num == "8" || num == "11") {
                    if num == "8" {
                        rules.insert(num.clone(), format!("{}+", regex_pattern));
                    } else if num == "11" {
                        let rule_42 = rules.get("42").unwrap();
                        let rule_31 = rules.get("31").unwrap();
                        let pattern = (0..10)
                            .map(|i| format!("{}{{{}}}{}{{{}}}", rule_42, i + 1, rule_31, i + 1))
                            .collect::<Vec<String>>()
                            .join("|");
                        // println!("{}", pattern);
                        rules.insert(num.clone(), format!("({})", pattern));
                    }
                } else {
                    // if this reference level contained a pipe, use braces
                    if references.contains("|") {
                        regex_pattern = format!("({})", regex_pattern);
                    }
                    rules.insert(num.clone(), regex_pattern);
                }

                keys.remove(index);
            } else {
                index += 1;
            }
        }

        if keys.len() == 0 {
            break;
        }
    }

    rules
}

fn main() {
    let text = read_text("19/input.txt").unwrap();

    let mut starting_values = HashMap::new();
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
                starting_values.insert(num, result.replace("\"", ""));
            } else {
                rules_to_process.insert(num, result.to_string());
            }
        }
    }

    let rules = create_regex_rules(&rules_to_process, &starting_values, false);
    let zero_re = Regex::new(&format!("^{}$", rules.get("0").unwrap())).unwrap();
    let matched = messages.iter().fold(0, |sum, message| {
        if zero_re.is_match(message) {
            return sum + 1;
        }
        sum
    });
    println!("{}", matched);

    let rules = create_regex_rules(&rules_to_process, &starting_values, true);
    let zero_re = Regex::new(&format!("^{}$", rules.get("0").unwrap())).unwrap();

    let matched = messages.iter().fold(0, |sum, message| {
        if zero_re.is_match(message) {
            return sum + 1;
        }
        sum
    });
    println!("{}", matched);
}
