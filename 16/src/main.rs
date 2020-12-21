use read_input::read_text;
use std::collections::{HashMap, HashSet};

#[derive(PartialEq)]
enum ScanMode {
    PropertyValidations,
    MyTicket,
    NearbyTickets,
}

struct Field {
    name: String,
    validations: Vec<(usize, usize)>,
}

impl Field {
    fn is_value_valid(&self, value: usize) -> bool {
        self.validations.iter().fold(false, |result, range| {
            result || (value >= range.0 && value <= range.1)
        })
    }
}

fn find_invalid_tickets(
    fields: &Vec<Field>,
    other_tickets: &Vec<Vec<usize>>,
) -> (HashSet<usize>, usize) {
    let mut invalid_count = 0;
    let mut invalid_tickets = HashSet::new();
    for (ticket_index, ticket_data) in other_tickets.iter().enumerate() {
        for field_value in ticket_data {
            let field_valid = fields.iter().fold(false, |result, field| {
                result || field.is_value_valid(*field_value)
            });

            if !field_valid {
                invalid_count += field_value;
                invalid_tickets.insert(ticket_index);
            }
        }
    }

    (invalid_tickets, invalid_count)
}

fn main() {
    let text = read_text("16/input.txt").unwrap();

    let mut scan_mode = ScanMode::PropertyValidations;
    let mut fields = Vec::new();
    let mut my_ticket: Vec<usize> = Vec::new();
    let mut other_tickets: Vec<Vec<usize>> = Vec::new();

    for line in text.lines() {
        if line == "" {
            if scan_mode == ScanMode::PropertyValidations {
                scan_mode = ScanMode::MyTicket;
            } else if scan_mode == ScanMode::MyTicket {
                scan_mode = ScanMode::NearbyTickets;
            }
        } else {
            match scan_mode {
                ScanMode::PropertyValidations => {
                    let mut iter = line.split(": ");
                    let name = iter.next().unwrap();
                    let ranges: Vec<(usize, usize)> = iter
                        .next()
                        .unwrap()
                        .split(" or ")
                        .map(|range| {
                            let mut r = range.split("-");
                            let first: usize = r.next().unwrap().parse().unwrap();
                            let second: usize = r.next().unwrap().parse().unwrap();

                            (first, second)
                        })
                        .collect();

                    fields.push(Field {
                        name: name.to_string(),
                        validations: ranges,
                    });
                }
                ScanMode::MyTicket => {
                    if line != "your ticket:" {
                        my_ticket
                            .append(&mut line.split(",").map(|v| v.parse().unwrap()).collect());
                    }
                }
                ScanMode::NearbyTickets => {
                    if line != "nearby tickets:" {
                        other_tickets.push(line.split(",").map(|v| v.parse().unwrap()).collect());
                    }
                }
            }
        }
    }

    let (invalid_tickets, invalid_field_count) = find_invalid_tickets(&fields, &other_tickets);

    println!("{}", invalid_field_count);

    let mut field_mappings: HashMap<String, usize> = HashMap::new();
    // go through each field on the ticket left to right
    for ticket_field_i in 0..other_tickets[0].len() {
        // go through each field definition, likely not in the same order as the ticket
        for field in &fields {
            // if we've grabbed this field already, skip it
            if field_mappings.contains_key(&field.name) {
                continue;
            }
            let mut is_field_valid = true;
            // go through each ticket
            for (ticket_i, ticket) in other_tickets.iter().enumerate() {
                // if its invalid, skip it
                if invalid_tickets.contains(&ticket_i) {
                    continue;
                }
                // if field is not valid for this index skip
                if !field.is_value_valid(ticket[ticket_field_i]) {
                    is_field_valid = false;
                    break;
                }
            }

            if is_field_valid {
                field_mappings.insert(field.name.clone(), ticket_field_i);
                break;
            }
        }
    }

    println!("{} {:?}", field_mappings.len(), field_mappings);

    let mut p2_value = 1;
    for (field_name, index) in &field_mappings {
        if field_name.contains("departure") {
            println!("{} {} {}", field_name, index, my_ticket[*index]);
            p2_value *= my_ticket[*index];
        }
    }
    println!("{}", p2_value);
}
