use std::thread::current;

fn get_destination(input: &Vec<u32>, current_cup: u32, highest_cup: u32) -> usize {
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

fn take_turn(input: &mut Vec<u32>, current_cup: &mut u32, highest_cup: u32) {
    let mut picked_up = Vec::with_capacity(3);
    // pick up the next 3 cups, remove them from the input
    let debug_input = input.clone();
    for _ in 0..3 {
        let current_cup_index = input.iter().position(|v| *v == *current_cup).unwrap();
        let next = current_cup_index + 1;
        let value = input.remove(next % input.len());
        picked_up.push(value);
    }

    // find the destination
    let destination = get_destination(&input, *current_cup, highest_cup);

    for (i, pickedup_cup) in picked_up.iter().enumerate() {
        input.insert(destination + 1 + i, pickedup_cup.to_owned());
    }

    let current_cup_index = input.iter().position(|v| *v == *current_cup).unwrap();
    *current_cup = input[(current_cup_index + 1) % input.len()];
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
        .collect::<Vec<u32>>();

    let mut p2_input = input.clone();

    let mut current_cup = input[0];

    for _ in 0..100 {
        take_turn(&mut input, &mut current_cup, highest_cup);
    }

    let start_index = input.iter().position(|v| *v == 1).unwrap();
    let mut output = Vec::with_capacity(input.len() - 1);
    for n in 1..9 {
        output.push(input[(start_index + n) % input.len()].to_string());
    }
    println!("{:?}", output.join(""));

    for n in highest_cup..=1_000_000 {
        p2_input.push(n);
    }

    let mut current_cup = p2_input[0];
    for _ in 0..10_000_000 {
        take_turn(&mut p2_input, &mut current_cup, 10_000_000);
    }

    let start_index = p2_input.iter().position(|v| *v == 1).unwrap();
    println!(
        "{}",
        p2_input[(start_index + 1) % p2_input.len()] * p2_input[(start_index + 2) % p2_input.len()]
    );
}
