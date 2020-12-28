use std::cmp;
use std::collections::HashMap;

use read_input::read_text;

type Coord = (i32, i32, i32, i32);

fn find_active_count(
    coords: &HashMap<Coord, char>,
    coord: &Coord,
    axis_count: i32,
    current_axis: i32,
    inputs: Vec<i32>,
) -> i32 {
    let mut active_count = 0;
    if current_axis >= axis_count {
        let mut input_iter = inputs.iter();
        let x = input_iter.next().unwrap();
        let y = input_iter.next().unwrap();
        let z = input_iter.next().unwrap();
        let w = input_iter.next().unwrap_or(&0);
        if *x == 0 && *y == 0 && *z == 0 && *w == 0 {
            return active_count;
        }
        let neighbour = (coord.0 + *x, coord.1 + *y, coord.2 + *z, coord.3 + *w);
        if let Some(state) = coords.get(&neighbour) {
            if *state == '#' {
                active_count += 1;
            }
        }

        return active_count;
    }
    for axis_value in -1..=1 {
        let mut inputs = inputs.clone();
        inputs.push(axis_value);
        active_count += find_active_count(coords, coord, axis_count, current_axis + 1, inputs);
    }

    active_count
}

fn update_state_for_coord(
    coords: &mut HashMap<Coord, char>,
    next_coords: &mut HashMap<Coord, char>,
    coord: &Coord,
    next_ranges: &mut Vec<(i32, i32)>,
) {
    if !coords.contains_key(coord) {
        coords.insert(coord.clone(), '.');
    }
    let current_state = coords.get(coord).unwrap();
    let active_count = find_active_count(coords, coord, next_ranges.len() as i32, 0, Vec::new());

    if (*current_state == '#' && (active_count == 2 || active_count == 3))
        || (*current_state == '.' && active_count == 3)
    {
        let axis_values = vec![coord.0, coord.1, coord.2, coord.3];
        for (i, range) in next_ranges.iter_mut().enumerate() {
            range.0 = cmp::min(range.0, axis_values[i] - 1);
            range.1 = cmp::max(range.1, axis_values[i] + 1);
        }
        next_coords.insert(coord.clone(), '#');
    } else {
        next_coords.insert(coord.clone(), '.');
    }
}

fn get_coords_from_input(input: &String) -> (HashMap<Coord, char>, (i32, i32), (i32, i32)) {
    let mut coords: HashMap<Coord, char> = HashMap::new();
    let mut x_range = (-1, 0);
    let mut y_range = (-1, 0);
    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for ch in line.chars() {
            coords.insert((x, y, 0, 0), ch);
            x += 1;
        }
        y += 1;
        x_range.1 = cmp::max(x_range.1, x);
    }

    y_range.1 = y + 1;
    x_range.1 += 1;

    (coords, x_range, y_range)
}

fn for_next_range(
    coords: &mut HashMap<Coord, char>,
    next_state: &mut HashMap<Coord, char>,
    ranges: &Vec<(i32, i32)>,
    range_idx: usize,
    next_ranges: &mut Vec<(i32, i32)>,
    inputs: Vec<i32>,
) {
    if range_idx >= ranges.len() {
        let mut input_iter = inputs.iter();
        let x = input_iter.next().unwrap();
        let y = input_iter.next().unwrap();
        let z = input_iter.next().unwrap();
        let w = input_iter.next().unwrap_or(&0);
        update_state_for_coord(coords, next_state, &(*x, *y, *z, *w), next_ranges);
    } else {
        for axis_value in ranges[range_idx].0..=ranges[range_idx].1 {
            // a lot of allocation this way
            let mut inputs = inputs.clone();
            inputs.push(axis_value);
            for_next_range(
                coords,
                next_state,
                ranges,
                range_idx + 1,
                next_ranges,
                inputs,
            );
        }
    }
}

fn main() {
    let input = read_text("17/input.txt").unwrap();

    let (mut coords, x_range, y_range) = get_coords_from_input(&input);
    let mut ranges = vec![x_range, y_range, (-1, 1)];

    for _ in 0..6 {
        let mut next_state = HashMap::new();
        let mut next_ranges = ranges.clone();

        for_next_range(
            &mut coords,
            &mut next_state,
            &ranges,
            0,
            &mut next_ranges,
            Vec::new(),
        );

        ranges = next_ranges;
        coords = next_state;
    }

    println!(
        "{}",
        coords.iter().fold(0, |sum, (_, state)| {
            if *state == '#' {
                return sum + 1;
            }

            sum
        })
    );

    let (mut coords, x_range, y_range) = get_coords_from_input(&input);
    let mut ranges = vec![x_range, y_range, (-1, 1), (-1, 1)];

    for _ in 0..6 {
        let mut next_state = HashMap::new();
        let mut next_ranges = ranges.clone();

        for_next_range(
            &mut coords,
            &mut next_state,
            &ranges,
            0,
            &mut next_ranges,
            Vec::new(),
        );

        ranges = next_ranges;
        coords = next_state;
    }

    println!(
        "{}",
        coords.iter().fold(0, |sum, (_, state)| {
            if *state == '#' {
                return sum + 1;
            }

            sum
        })
    );
}
