use std::cmp;

use read_input::read_text;

fn main() {
    let text = read_text("9/input.txt").unwrap();

    let mut number_set = Vec::new();
    let mut invalid_number = 0;
    let mut all_numbers = Vec::new();
    for line in text.lines() {
        let n: usize = line.parse().unwrap();

        if number_set.len() == 25 {
            let mut is_valid = false;
            'i_loop: for i in 0..25 {
                for j in i..25 {
                    if number_set[i] + number_set[j] == n {
                        is_valid = true;
                        break 'i_loop;
                    }
                }
            }
            if !is_valid {
                invalid_number = n;
            }
            number_set.remove(0);
        }

        number_set.push(n);
        all_numbers.push(n);
    }

    println!("{}", invalid_number);

    for i in 0..all_numbers.len() {
        let mut next_index = i + 1;
        let mut end_index = None;
        let mut value = all_numbers[i];
        loop {
            value += all_numbers[next_index];
            if value == invalid_number {
                end_index = Some(next_index);
                break;
            }

            next_index += 1;
            if next_index == all_numbers.len() {
                break;
            }
        }
        if end_index.is_some() {
            let mut min = usize::MAX;
            let mut max = 0;
            for index in i..end_index.unwrap() {
                min = cmp::min(min, all_numbers[index]);
                max = cmp::max(max, all_numbers[index]);
            }
            println!("{}", min + max);
            break;
        }
    }
}
