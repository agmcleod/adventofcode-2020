use std::collections::HashMap;

use read_input::read_text;

fn main() {
    let text = read_text("6/input.txt").unwrap();

    let mut answered_questions = HashMap::new();

    let mut sum = 0;
    let mut sum_p2 = 0;
    let mut passenger_count = 0;
    for line in text.lines() {
        if line == "" {
            sum += answered_questions.len();
            for (_, count) in &answered_questions {
                if *count == passenger_count {
                    sum_p2 += 1;
                }
            }
            answered_questions.clear();
            passenger_count = 0;
        } else {
            for ch in line.chars() {
                if answered_questions.contains_key(&ch) {
                    *answered_questions.get_mut(&ch).unwrap() += 1;
                } else {
                    answered_questions.insert(ch, 1);
                }
            }

            passenger_count += 1;
        }
    }

    sum += answered_questions.len();

    for (_, count) in &answered_questions {
        if *count == passenger_count {
            sum_p2 += 1;
        }
    }

    println!("{} {}", sum, sum_p2);
}
