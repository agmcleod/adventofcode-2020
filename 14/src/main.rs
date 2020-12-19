use std::char;
use std::collections::HashMap;

use read_input::read_text;

fn number_to_binary(num: usize, mask: &Vec<char>) -> Vec<char> {
    let mut binary_number: Vec<char> = format!("{:b}", num).chars().collect();
    let mut leading: Vec<char> = (0..mask.len() - binary_number.len()).map(|_| '0').collect();
    leading.append(&mut binary_number);

    leading
}

fn get_next_permutations(indexes: &Vec<usize>, current_index: usize) -> Vec<Vec<(char, usize)>> {
    if current_index + 1 == indexes.len() {
        (0..=1)
            .map(|i| vec![(char::from_digit(i, 10).unwrap(), indexes[current_index])])
            .collect()
    } else {
        let next = get_next_permutations(indexes, current_index + 1);
        let mut results = Vec::new();
        for i in 0..=1 {
            for opt in &next {
                let mut permutations =
                    vec![(char::from_digit(i, 10).unwrap(), indexes[current_index])];
                permutations.append(&mut opt.clone());
                results.push(permutations);
            }
        }

        results
    }
}

fn main() {
    let text = read_text("14/input.txt").unwrap();
    let mut addresses = HashMap::new();
    let mut addresses_p2 = HashMap::new();

    let mut mask = Vec::new();

    for line in text.lines() {
        if line.contains("mask = ") {
            mask = line.replace("mask = ", "").chars().collect();
        } else {
            let numbers: Vec<usize> = line
                .replace("mem[", "")
                .replace("]", "")
                .split(" = ")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();

            let mut binary_number = number_to_binary(numbers[1], &mask);
            for i in 0..binary_number.len() {
                let mask_value = mask[i];
                if mask_value == '1' {
                    binary_number[i] = '1';
                } else if mask_value == '0' {
                    binary_number[i] = '0';
                }
            }
            addresses.insert(numbers[0], binary_number);

            // p2 work
            let binary_number = number_to_binary(numbers[0], &mask);
            let mut one_indexes = Vec::new();
            let mut x_indexes = Vec::new();
            for (i, ch) in mask.iter().enumerate() {
                if *ch == '1' {
                    one_indexes.push(i);
                } else if *ch == 'X' {
                    x_indexes.push(i);
                }
            }

            let permutations = get_next_permutations(&x_indexes, 0);

            for permutation in &permutations {
                let mut bn = binary_number.clone();
                for oi in &one_indexes {
                    bn[*oi] = '1';
                }

                for (value, index) in permutation {
                    bn[*index] = *value;
                }

                let binary_string = bn.iter().collect::<String>();
                let addr = usize::from_str_radix(&binary_string, 2).unwrap();
                addresses_p2.insert(addr, numbers[1]);
            }
        }
    }

    let sum = addresses.iter().fold(0, |sum, (_addr, binary_chars)| {
        sum + usize::from_str_radix(&binary_chars.iter().collect::<String>(), 2).unwrap()
    });

    println!("{}", sum);

    println!(
        "{}",
        addresses_p2
            .iter()
            .fold(0, |sum, (_addr, value)| { sum + value })
    );
}
