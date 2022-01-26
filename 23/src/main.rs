use std::collections::LinkedList;

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

fn take_turn(input: &mut LinkedList<u32>, current_cup: &mut u32, highest_cup: u32) {
    let mut picked_up = LinkedList::new();
    // pick up the next 3 cups, remove them from the input
    let current_cup_index = input.iter().position(|v| *v == *current_cup).unwrap();
    let mut split = input.split_off(current_cup_index + 1);
    // let n = split.pop_front().unwrap();
    // input.push_back(n);
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
    let destination = get_destination(&input, *current_cup, highest_cup);
    let mut split = input.split_off(destination + 1);
    input.append(&mut picked_up);
    input.append(&mut split);

    // iterate until we find the current cup
    let mut iter = input.iter();
    while let Some(v) = iter.next() {
        if *v == *current_cup {
            break;
        }
    }
    // get the one after the curent cup
    if let Some(v) = iter.next() {
        *current_cup = *v;
    } else {
        // if it didn't exist, get the first one
        *current_cup = input.front().unwrap().to_owned();
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

    let mut current_cup = input.front().unwrap().to_owned();

    for _ in 0..100 {
        take_turn(&mut input, &mut current_cup, highest_cup);
    }

    // create p1 answer
    let start_index = input.iter().position(|v| *v == 1).unwrap();
    let mut split = input.split_off(start_index);
    split.pop_front();
    split.append(&mut input);
    let output: String = split.iter().map(|v| v.to_string()).collect();
    println!("{:?}", output);

    for n in highest_cup..=1_000_000 {
        p2_input.push_back(n);
    }

    let mut current_cup = p2_input.front().unwrap().to_owned();
    for _ in 0..10_000_000 {
        take_turn(&mut p2_input, &mut current_cup, 10_000_000);
    }

    let start_index = p2_input.iter().position(|v| *v == 1).unwrap();
    let split = p2_input.split_off(start_index);
    let mut iter = split.iter();
    iter.next();
    println!("{}", iter.next().unwrap() * iter.next().unwrap());
}
