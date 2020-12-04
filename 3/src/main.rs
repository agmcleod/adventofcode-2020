use std::collections::HashMap;

use read_input::read_text;

#[derive(PartialEq)]
enum TileType {
    Open,
    Tree,
}

fn main() {
    let text = read_text("3/input.txt").unwrap();

    let mut tiles = HashMap::new();

    let mut tobbogan_x = 0;
    let mut tree_count = 0;

    for (y, line) in text.lines().enumerate() {
        let width = line.chars().count();
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' => {
                    tiles.insert((x, y), TileType::Open);
                },
                '#' => {
                    tiles.insert((x, y), TileType::Tree);
                },
                _ => panic!("Invalid character {}", ch)
            }
        }

        // skip first line
        if y > 0 {
            tobbogan_x += 3;
            if let Some(tile_type) = tiles.get(&(tobbogan_x % width, y)) {
                if  *tile_type == TileType::Tree {
                    tree_count += 1;
                }
            } else {
                panic!("No coord found for: ({} % {} = {},{})", tobbogan_x, width, tobbogan_x % width, y);
            }
        }
    }

    println!("{}", tree_count);
}
