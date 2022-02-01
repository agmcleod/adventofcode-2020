use std::collections::HashSet;
use std::io::Result;

use read_input::read_text;

type Range = (i32, i32);
type FlippedTiles = HashSet<(i32, i32, i32)>;

fn get_flipped_neighbours(tiles: &FlippedTiles, pos: &(i32, i32, i32)) -> usize {
    let mut count = 0;
    // ne
    if tiles.contains(&(pos.0 + 1, pos.1 - 1, pos.2)) {
        count += 1;
    }
    // nw
    if tiles.contains(&(pos.0, pos.1 - 1, pos.2 + 1)) {
        count += 1;
    }
    // se
    if tiles.contains(&(pos.0, pos.1 + 1, pos.2 - 1)) {
        count += 1;
    }
    // sw
    if tiles.contains(&(pos.0 - 1, pos.1 + 1, pos.2)) {
        count += 1;
    }
    // w
    if tiles.contains(&(pos.0 - 1, pos.1, pos.2 + 1)) {
        count += 1;
    }
    // e
    if tiles.contains(&(pos.0 + 1, pos.1, pos.2 - 1)) {
        count += 1;
    }

    count
}

fn take_turn(
    tiles: &FlippedTiles,
    next_state: &mut FlippedTiles,
    q_range: &mut Range,
    r_range: &mut Range,
    s_range: &mut Range,
) {
    let mut next_q_range = q_range.to_owned();
    let mut next_r_range = r_range.to_owned();
    let mut next_s_range = s_range.to_owned();

    for q in (q_range.0 - 1)..=(q_range.1 + 1) {
        for r in (r_range.0 - 1)..=(r_range.1 + 1) {
            for s in (s_range.0 - 1)..=(s_range.1 + 1) {
                let pos = (q, r, s);
                let count = get_flipped_neighbours(tiles, &pos);
                if tiles.contains(&pos) && (count == 0 || count > 2) {
                    next_state.remove(&pos);
                } else if !tiles.contains(&pos) && count == 2 {
                    next_q_range.0 = next_q_range.0.min(q);
                    next_q_range.1 = next_q_range.1.max(q);
                    next_r_range.0 = next_r_range.0.min(r);
                    next_r_range.1 = next_r_range.1.max(r);
                    next_s_range.0 = next_s_range.0.min(s);
                    next_s_range.1 = next_s_range.1.max(s);

                    next_state.insert(pos);
                }
            }
        }
    }

    *q_range = next_q_range;
    *r_range = next_r_range;
    *s_range = next_s_range;
}

fn get_initial_flipped_tiles(text: &String) -> (FlippedTiles, Range, Range, Range) {
    let mut flipped_tiles = HashSet::new();
    let mut q_range = (std::i32::MAX, 0);
    let mut r_range = (std::i32::MAX, 0);
    let mut s_range = (std::i32::MAX, 0);

    for line in text.lines() {
        let chars: Vec<char> = line.chars().collect();
        let mut scanned_indexes = HashSet::new();
        let mut coord = (0, 0, 0); // q, r, s
        for (i, pair) in chars.windows(2).enumerate() {
            // skip this one if we've used the index already
            if !scanned_indexes.contains(&i) {
                let mut used_pair = false;
                if pair == &['n', 'e'] {
                    coord.0 += 1;
                    coord.1 -= 1;
                    used_pair = true;
                } else if pair == &['n', 'w'] {
                    coord.1 -= 1;
                    coord.2 += 1;
                    used_pair = true;
                } else if pair == &['s', 'e'] {
                    coord.1 += 1;
                    coord.2 -= 1;
                    used_pair = true;
                } else if pair == &['s', 'w'] {
                    coord.0 -= 1;
                    coord.1 += 1;
                    used_pair = true;
                } else if pair[0] == 'w' {
                    coord.0 -= 1;
                    coord.2 += 1;
                } else if pair[0] == 'e' {
                    coord.0 += 1;
                    coord.2 -= 1;
                } else {
                    panic!("Dont know how to handle {:?}", pair);
                }

                q_range.0 = q_range.0.min(coord.0);
                q_range.1 = q_range.1.max(coord.0);
                r_range.0 = r_range.0.min(coord.1);
                r_range.1 = r_range.1.max(coord.1);
                s_range.0 = s_range.0.min(coord.2);
                s_range.1 = s_range.1.max(coord.2);

                scanned_indexes.insert(i);
                if used_pair {
                    scanned_indexes.insert(i + 1);
                }
            }

            // if on the last window, check the final digit
            if i == chars.len() - 2 && !scanned_indexes.contains(&(i + 1)) {
                if pair[1] == 'w' {
                    coord.0 -= 1;
                    coord.2 += 1;
                } else if pair[1] == 'e' {
                    coord.0 += 1;
                    coord.2 -= 1;
                } else {
                    panic!("unhandled trailing character {:?}", pair);
                }
            }
        }

        if flipped_tiles.contains(&coord) {
            flipped_tiles.remove(&coord);
        } else {
            flipped_tiles.insert(coord.clone());
        }
    }

    (flipped_tiles, q_range, r_range, s_range)
}

fn main() -> Result<()> {
    let text = read_text("24/input.txt")?;

    let (mut flipped_tiles, mut q_range, mut r_range, mut s_range) =
        get_initial_flipped_tiles(&text);
    println!("{}", flipped_tiles.len());

    // p2
    for _ in 0..100 {
        let mut next_state = flipped_tiles.clone();
        take_turn(
            &flipped_tiles,
            &mut next_state,
            &mut q_range,
            &mut r_range,
            &mut s_range,
        );
        flipped_tiles = next_state;
    }

    println!("{}", flipped_tiles.len());

    Ok(())
}
