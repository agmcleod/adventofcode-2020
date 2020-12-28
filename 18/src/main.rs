use std::str::Chars;

use read_input::read_text;

enum Operator {
    Add,
    Multiply,
}

fn calculate_expression(chars: &mut Chars) -> usize {
    let mut result = 0;
    let mut operator = None;

    loop {
        let next = chars.next();
        if next.is_none() {
            break;
        }

        let next = next.unwrap();
        if next == ')' {
            break;
        } else if next == '(' {
            let sub_amount = calculate_expression(chars);
            if operator.is_some() {
                match operator.as_ref().unwrap() {
                    Operator::Add => {
                        result += sub_amount;
                    }
                    Operator::Multiply => {
                        result *= sub_amount;
                    }
                }
            } else {
                result = sub_amount;
            }
        } else if next == '+' {
            operator = Some(Operator::Add);
        } else if next == '*' {
            operator = Some(Operator::Multiply);
        } else {
            let num = next.to_digit(10).unwrap();
            if operator.is_some() {
                match operator.as_ref().unwrap() {
                    Operator::Add => {
                        result += num as usize;
                    }
                    Operator::Multiply => {
                        result *= num as usize;
                    }
                }
            } else {
                result = num as usize;
            }
        }
    }

    result
}

fn main() {
    let text = read_text("18/input.txt").unwrap();

    let mut sum = 0;
    for line in text.lines() {
        let line = line.replace(" ", "");

        let mut chars = line.chars();
        let result = calculate_expression(&mut chars);
        sum += result;
    }

    println!("{}", sum);
}
