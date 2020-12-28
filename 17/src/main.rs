use std::cmp;
use std::collections::HashMap;

use read_input::read_text;

type Coord = (i32, i32, i32);

fn update_state_for_coord(
    coords: &mut HashMap<Coord, char>,
    next_coords: &mut HashMap<Coord, char>,
    coord: &Coord,
    next_x_range: &mut (i32, i32),
    next_y_range: &mut (i32, i32),
    next_z_range: &mut (i32, i32),
) {
    if !coords.contains_key(coord) {
        coords.insert(coord.clone(), '.');
    }
    let current_state = coords.get(coord).unwrap();
    let mut active_count = 0;
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }

                let neighbour = (coord.0 + x, coord.1 + y, coord.2 + z);
                if let Some(state) = coords.get(&neighbour) {
                    if *state == '#' {
                        active_count += 1;
                    }
                } else {
                    next_coords.insert(neighbour, '.');
                }
            }
        }
    }

    if (*current_state == '#' && (active_count == 2 || active_count == 3))
        || (*current_state == '.' && active_count == 3)
    {
        next_x_range.0 = cmp::min(next_x_range.0, coord.0 - 1);
        next_x_range.1 = cmp::max(next_x_range.1, coord.0 + 1);
        next_y_range.0 = cmp::min(next_y_range.0, coord.1 - 1);
        next_y_range.1 = cmp::max(next_y_range.1, coord.1 + 1);
        next_z_range.0 = cmp::min(next_z_range.0, coord.2 - 1);
        next_z_range.1 = cmp::max(next_z_range.1, coord.2 + 1);
        next_coords.insert(coord.clone(), '#');
    } else {
        next_coords.insert(coord.clone(), '.');
    }
}

fn get_coords_from_input(
    input: &String,
) -> (HashMap<Coord, char>, (i32, i32), (i32, i32), (i32, i32)) {
    let mut coords: HashMap<Coord, char> = HashMap::new();
    let mut x_range = (-1, 0);
    let mut y_range = (-1, 0);
    let z_range = (-1, 1);
    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for ch in line.chars() {
            coords.insert((x, y, 0), ch);
            x += 1;
        }
        y += 1;
        x_range.1 = cmp::max(x_range.1, x);
    }

    y_range.1 = y + 1;
    x_range.1 += 1;

    (coords, x_range, y_range, z_range)
}

fn main() {
    let input = read_text("17/input.txt").unwrap();

    let (mut coords, mut x_range, mut y_range, mut z_range) = get_coords_from_input(&input);

    for step in 0..6 {
        let mut next_state = HashMap::new();
        let mut next_x_range = x_range.clone();
        let mut next_y_range = y_range.clone();
        let mut next_z_range = z_range.clone();
        for x in x_range.0..=x_range.1 {
            for y in y_range.0..=y_range.1 {
                for z in z_range.0..=z_range.1 {
                    update_state_for_coord(
                        &mut coords,
                        &mut next_state,
                        &(x, y, z),
                        &mut next_x_range,
                        &mut next_y_range,
                        &mut next_z_range,
                    );
                }
            }
        }

        x_range = next_x_range;
        y_range = next_y_range;
        z_range = next_z_range;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_state_for_coord() {
        let input = ".#.\n..#\n###".to_string();
        let (coords, _x_range, _y_range, _z_range) = get_coords_from_input(&input);
        let mut next_state = HashMap::new();
        for (coord, _current_state) in &coords {
            update_state_for_coord(&coords, &mut next_state, coord);
        }

        assert_eq!(*next_state.get(&(0, 1, -1)).unwrap(), '#');
        assert_eq!(*next_state.get(&(1, 1, -1)).unwrap(), '.');
        assert_eq!(*next_state.get(&(2, 1, -1)).unwrap(), '.');
        assert_eq!(*next_state.get(&(0, 2, -1)).unwrap(), '.');
        assert_eq!(*next_state.get(&(1, 2, -1)).unwrap(), '.');
        assert_eq!(*next_state.get(&(2, 2, -1)).unwrap(), '#');
    }
}
