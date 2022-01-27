use std::collections::HashSet;
use std::io::Result;

use read_input::read_text;

fn main() -> Result<()> {
    let text = read_text("24/input.txt")?;
    let mut flipped_tiles = HashSet::new();

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

        // println!("{:?}\n", steps);

        if flipped_tiles.contains(&coord) {
            flipped_tiles.remove(&coord);
        } else {
            flipped_tiles.insert(coord.clone());
        }
    }

    println!("{}", flipped_tiles.len());

    Ok(())
}
