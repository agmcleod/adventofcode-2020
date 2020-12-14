use std::collections::HashMap;

use read_input::read_text;

#[derive(PartialEq)]
enum TileType {
    Open,
    Tree,
}

struct Slope {
    x_pos: usize,
    x_speed: usize,
    y_speed: usize,
    tree_count: usize,
}

impl Slope {
    fn new(x_speed: usize, y_speed: usize) -> Self {
        Slope {
            x_pos: 0,
            x_speed,
            y_speed,
            tree_count: 0,
        }
    }
}

fn main() {
    let text = read_text("3/input.txt").unwrap();

    let mut tiles = HashMap::new();

    let mut slopes = vec![
        Slope::new(3, 1),
        Slope::new(1, 1),
        Slope::new(5, 1),
        Slope::new(7, 1),
        Slope::new(1, 2),
    ];

    for (y, line) in text.lines().enumerate() {
        let width = line.chars().count();
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' => {
                    tiles.insert((x, y), TileType::Open);
                }
                '#' => {
                    tiles.insert((x, y), TileType::Tree);
                }
                _ => panic!("Invalid character {}", ch),
            }
        }

        // skip first line
        if y > 0 {
            for slope in &mut slopes {
                if y % slope.y_speed == 0 {
                    slope.x_pos += slope.x_speed;
                    if let Some(tile_type) = tiles.get(&(slope.x_pos % width, y)) {
                        if *tile_type == TileType::Tree {
                            slope.tree_count += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", slopes[0].tree_count);
    println!(
        "{}",
        slopes
            .iter()
            .fold(1, |acc, slope| { slope.tree_count * acc })
    );
}
