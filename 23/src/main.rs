use std::collections::{HashMap, LinkedList};

struct Cups {
    data: HashMap<String, String>,
}

impl Cups {
    fn new() -> Self {
        Cups {
            data: HashMap::new(),
        }
    }
}

struct CupsIterator<'a> {
    cursor: &'a str,
    data: &'a HashMap<String, String>,
}

impl<'a> Iterator for CupsIterator<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        let result: Option<Self::Item> = self.data.get(self.cursor);
        if let Some(v) = result {
            self.cursor = v;
        }

        result
    }
}

fn get_destination(input: &LinkedList<u32>, current_cup: u32, highest_cup: u32) -> usize {
    let mut incr = 1;
    let mut destination = 0;
    let mut destination_cup = current_cup;
    loop {
        // try the index of current cup - 1, then -2, etc.
        let index = input.iter().position(|v| *v == destination_cup - incr);
        if index.is_some() {
            destination = index.unwrap().to_owned();
            break;
        }
        incr += 1;
        // if we go negative, go to the highest value
        if incr > destination_cup {
            incr = 0;
            destination_cup = highest_cup;
        }
    }
    destination
}

fn take_turn(input: &mut LinkedList<u32>, current_cup_index: &mut usize, highest_cup: u32) {
    let mut picked_up = LinkedList::new();
    // pick up the next 3 cups, remove them from the input
    // let debug_list = input.clone();
    let mut split = input.split_off(*current_cup_index + 1);
    let current_cup = input.back().unwrap().to_owned();
    for _ in 0..3 {
        let value = split.pop_front();
        if value.is_some() {
            picked_up.push_back(value.unwrap());
        } else {
            // ran out of the split, so start pulling off the front of the list
            picked_up.push_back(input.pop_front().unwrap());
        }
    }

    // add the split back
    input.append(&mut split);

    // find the destination
    let destination = get_destination(&input, current_cup, highest_cup);
    let mut split = input.split_off(destination + 1);
    // let debug_picked_up = picked_up.clone();
    input.append(&mut picked_up);
    input.append(&mut split);

    // println!(
    //     "current {} list {:?}\npicked up: {:?}\ndest idx: {}\n",
    //     current_cup, debug_list, debug_picked_up, destination
    // );

    if destination < *current_cup_index {
        *current_cup_index += 4;
    } else {
        *current_cup_index += 1;
    }
    if *current_cup_index >= input.len() {
        // *current_cup_index -= input.len();
        *current_cup_index = 0;
    }
}

fn main() {
    let mut highest_cup = 0;
    let mut input = "459672813"
        .chars()
        .map(|v| {
            let n = v.to_digit(10).unwrap();
            highest_cup = highest_cup.max(n);
            n
        })
        .collect::<LinkedList<u32>>();

    let mut p2_input = input.clone();

    let mut current_cup_index = 0;

    for _ in 0..100 {
        take_turn(&mut input, &mut current_cup_index, highest_cup);
    }

    // create p1 answer
    let start_index = input.iter().position(|v| *v == 1).unwrap();
    let mut split = input.split_off(start_index);
    split.pop_front();
    split.append(&mut input);
    let output: String = split.iter().map(|v| v.to_string()).collect();
    println!("{:?}", output);

    // p2
    for n in highest_cup..=1_000_000 {
        p2_input.push_back(n);
    }

    let mut current_cup_index = 0;
    for _ in 0..10_000_000 {
        take_turn(&mut p2_input, &mut current_cup_index, 10_000_000);
    }

    let start_index = p2_input.iter().position(|v| *v == 1).unwrap();
    let split = p2_input.split_off(start_index);
    let mut iter = split.iter();
    iter.next();
    println!("{}", iter.next().unwrap() * iter.next().unwrap());
}
