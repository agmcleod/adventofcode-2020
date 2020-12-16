use std::collections::HashMap;

use read_input::read_text;

const OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn get_neighbour_counts(layout: &HashMap<(i32, i32), char>, x: i32, y: i32) -> (i32, i32) {
    let mut empty_count = 0;
    let mut occupied_count = 0;

    for offset in &OFFSETS {
        if let Some(ch) = layout.get(&(x + offset.0, y + offset.1)) {
            if *ch == 'L' {
                empty_count += 1;
            } else if *ch == '#' {
                occupied_count += 1;
            }
        }
    }

    (empty_count, occupied_count)
}

fn change_state(layout: &HashMap<(i32, i32), char>, max_x: i32, max_y: i32) -> HashMap<(i32, i32), char> {
    let mut new_state = layout.clone();

    for y in 0..max_y {
        for x in 0..max_x {
            let (_empty_count, occupied_count) = get_neighbour_counts(layout, x, y);
            let ch = layout.get(&(x, y)).unwrap();
            if *ch == 'L' && occupied_count == 0 {
                new_state.insert((x, y), '#');
            } else if *ch == '#' && occupied_count >= 4 {
                new_state.insert((x, y), 'L');
            }
        }
    }

    new_state
}

fn get_neighbour_counts_p2(layout: &HashMap<(i32, i32), char>, x: i32, y: i32) -> (i32, i32) {
    let mut empty_count = 0;
    let mut occupied_count = 0;

    for offset in &OFFSETS {
        if let Some(ch) = layout.get(&(x + offset.0, y + offset.1)) {
            if *ch == 'L' {
                empty_count += 1;
            } else if *ch == '#' {
                occupied_count += 1;
            }
        }
    }

    (empty_count, occupied_count)
}

fn change_state_p2(layout: &HashMap<(i32, i32), char>, max_x: i32, max_y: i32) -> HashMap<(i32, i32), char> {
    let mut new_state = layout.clone();

    for y in 0..max_y {
        for x in 0..max_x {
            let (_empty_count, occupied_count) = get_neighbour_counts+p2(layout, x, y);
            let ch = layout.get(&(x, y)).unwrap();
            if *ch == 'L' && occupied_count == 0 {
                new_state.insert((x, y), '#');
            } else if *ch == '#' && occupied_count >= 4 {
                new_state.insert((x, y), 'L');
            }
        }
    }

    new_state
}

fn get_initial_state(text: &String) -> HashMap<(i32, i32), char> {
    let mut layout = HashMap::new();

    let mut y = 0;
    for line in text.lines() {
        let mut x = 0;
        for ch in line.chars() {
            layout.insert((x, y), ch);
            x += 1;
        }
        y += 1;
    }

    layout
}

fn main() {
    let text = read_text("11/input.txt").unwrap();

    let mut layout = get_initial_state(&text);

    let max_x = text.lines().next().unwrap().chars().count() as i32;
    let max_y = text.lines().count() as i32;

    loop {
        let next_state = change_state(&layout, max_x, max_y);
        if next_state == layout {
            layout = next_state;
            break
        } else {
            layout = next_state;
        }
    }

    println!("{}", layout.iter().fold(0, |sum, (_, ch)| {
        if *ch == '#' {
            return sum + 1;
        }
        sum
    }));

    loop {
        let next_state = change_state_p2(&layout, max_x, max_y);
        if next_state == layout {
            layout = next_state;
            break
        } else {
            layout = next_state;
        }
    }
}
