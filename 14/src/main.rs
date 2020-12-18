use std::collections::HashMap;

use read_input::read_text;

fn main() {
    let text = read_text("14/input.txt").unwrap();
    let mut addresses = HashMap::new();

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

            let mut binary_number: Vec<char> = format!("{:b}", numbers[1]).chars().collect();
            let mut leading: Vec<char> =
                (0..mask.len() - binary_number.len()).map(|_| '0').collect();
            leading.append(&mut binary_number);
            let mut binary_number = leading;
            for i in 0..binary_number.len() {
                let mask_value = mask[i];
                if mask_value == '1' {
                    binary_number[i] = '1';
                } else if mask_value == '0' {
                    binary_number[i] = '0';
                }
            }
            addresses.insert(numbers[0], binary_number);
        }
    }

    let sum = addresses.iter().fold(0, |sum, (_addr, binary_chars)| {
        sum + usize::from_str_radix(&binary_chars.iter().collect::<String>(), 2).unwrap()
    });

    println!("{}", sum);
}
