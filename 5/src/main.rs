use std::cmp;
use std::collections::HashMap;

use read_input::read_text;

fn main() {
    let text = read_text("5/input.txt").unwrap();

    let mut seats = HashMap::new();

    let mut highest_seat_id = 0;
    for line in text.lines() {
        let (first, last) = line.split_at(7);
        let mut min = 0;
        let mut max = 127;

        let mut row = 0;
        for instruction in first.chars() {
            if instruction == 'F' {
                max = (max - min) / 2 + min;
                row = max;
            } else if instruction == 'B' {
                min = (max - min) / 2 + min;
                row = max;
            } else {
                panic!("Unrecognized instruction {}", instruction);
            }
        }

        let mut min = 0;
        let mut max = 7;

        let mut col = 0;
        for instruction in last.chars() {
            if instruction == 'L' {
                max = (max - min) / 2 + min;
                col = max;
            } else if instruction == 'R' {
                min = (max - min) / 2 + min;
                col = max;
            } else {
                panic!("Unrecognized instruction {}", instruction);
            }
        }

        let seat_id = row * 8 + col;
        seats.insert((col, row), seat_id);

        highest_seat_id = cmp::max(highest_seat_id, seat_id);
    }

    println!("{}", highest_seat_id);

    for row in 1..127 {
        for col in 0..8 {
            if !seats.contains_key(&(col, row)) {
                println!("{},{}", col, row);
            }
        }
    }
}
