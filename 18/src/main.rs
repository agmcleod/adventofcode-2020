use std::str::Chars;

use regex;

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

fn run_addition(add_regex: &regex::Regex, mut value: String) -> String {
    loop {
        let add_capture = add_regex.captures(&value);
        if add_capture.is_none() {
            break;
        }

        let add_capture = &add_capture.unwrap()[0];
        let sum = add_capture
            .split("+")
            .fold(0, |sum, n| sum + n.parse::<usize>().unwrap());

        value = value.replacen(add_capture, &sum.to_string(), 1);
    }

    value
}

fn run_multiplication(mul_regex: &regex::Regex, mut value: String) -> String {
    loop {
        let mul_capture = mul_regex.captures(&value);
        if mul_capture.is_none() {
            break;
        }
        let mul_capture = &mul_capture.unwrap()[0];
        let product = mul_capture
            .split("*")
            .fold(1, |product, n| product * n.parse::<usize>().unwrap());

        value = value.replacen(mul_capture, &product.to_string(), 1);
    }
    value
}

fn get_group_regex() -> regex::Regex {
    regex::Regex::new(r"\([0-9*+]+\)").unwrap()
}

fn get_add_regex() -> regex::Regex {
    regex::Regex::new(r"(\d+\+\d+)").unwrap()
}

fn get_mult_regex() -> regex::Regex {
    regex::Regex::new(r"(\d+\*\d+)").unwrap()
}

fn calculate_expression_p2(mut line: String) -> usize {
    let group_regex = get_group_regex();
    let add_regex = get_add_regex();
    let mul_regex = get_mult_regex();

    loop {
        let group_capture = group_regex.captures(&line);
        if group_capture.is_none() {
            line = run_addition(&add_regex, line.to_string());
            line = run_multiplication(&mul_regex, line.to_string());
            break;
        }

        let group_capture = &group_capture.unwrap()[0];
        let mut result_str = run_addition(&add_regex, group_capture.to_string());
        result_str = run_multiplication(&mul_regex, result_str.clone());
        result_str = result_str.replace("(", "");
        result_str = result_str.replace(")", "");

        line = line.replacen(group_capture, &result_str, 1);
    }

    line.parse::<usize>().unwrap()
}

fn main() {
    let text = read_text("18/input.txt").unwrap();

    let mut sum = 0;
    let mut sum_p2 = 0;
    for line in text.lines() {
        let line = line.replace(" ", "");

        let mut chars = line.chars();
        let result = calculate_expression(&mut chars);
        sum += result;

        let result = calculate_expression_p2(line.clone());
        sum_p2 += result;
    }

    println!("p1 {}", sum);
    println!("p2 {}", sum_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_p2() {
        let line = "(5*4)".to_string();
        let result = run_multiplication(&get_mult_regex(), line);

        assert_eq!(result, "(20)");
    }
}
