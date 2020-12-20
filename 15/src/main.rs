use std::collections::HashMap;

fn main() {
    let input = "0,14,1,3,7,9".to_string();
    let mut numbers: HashMap<usize, Vec<usize>> = HashMap::new();
    let start_numbers = input
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();
    let mut last_number = 0;

    for turn in 0..30000000 {
        if turn < start_numbers.len() {
            numbers.insert(start_numbers[turn], vec![turn + 1]);
            last_number = start_numbers[turn];
        } else {
            if numbers.contains_key(&last_number) {
                let last_turns = numbers.get_mut(&last_number).unwrap();
                if last_turns.len() == 1 {
                    last_number = 0;
                } else {
                    last_number = last_turns[1] - last_turns[0];
                }
            } else {
                numbers.insert(last_number, vec![turn + 1]);
                last_number = 0;
            }

            if numbers.contains_key(&last_number) {
                let turns = numbers.get_mut(&last_number).unwrap();
                turns.push(turn + 1);
                if turns.len() > 2 {
                    turns.remove(0);
                }
            } else {
                numbers.insert(last_number, vec![turn + 1]);
            }
        }
    }

    println!("{}", last_number);
}
