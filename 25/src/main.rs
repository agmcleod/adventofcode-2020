fn single_step(value: usize, subject: usize) -> usize {
    (value * subject) % 20201227
}

fn transform_value(subject: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value = single_step(value, subject);
    }

    value
}

fn determine_loop_size(subject: usize, target: usize) -> usize {
    let mut value = 1;
    let mut loop_size = 0;

    while value != target {
        value = single_step(value, subject);

        loop_size += 1;
    }

    loop_size
}

fn main() {
    let card_public_key = 14788856;
    let door_public_key = 19316454;

    let card_loop_size = determine_loop_size(7, card_public_key);
    let door_loop_size = determine_loop_size(7, door_public_key);

    let key_a = transform_value(card_public_key, door_loop_size);
    let key_b = transform_value(door_public_key, card_loop_size);

    println!("{} = {} ", key_a, key_b);
}
