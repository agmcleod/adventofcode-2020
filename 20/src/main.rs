use std::collections::HashMap;
use std::fmt::Debug;

use read_input::read_text;

struct Tile {
    id: String,
    edges: Vec<String>,
    data: HashMap<(i32, i32), char>,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tile: {}\n{}\nEdges:\n{}\n",
            self.id,
            (0..10)
                .map(|y| {
                    (0..10)
                        .map(|x| self.data.get(&(x, y)).unwrap())
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .join("\n"),
            self.edges.join("\n")
        )
    }
}

fn main() {
    let text = read_text("20/input.txt").unwrap();

    let mut tiles = Vec::new();
    let mut id = String::new();
    let mut tile_data: HashMap<(i32, i32), char> = HashMap::new();
    let mut y = 0;

    for line in text.lines() {
        if line.contains("Tile") {
            id = line.replace("Tile ", "").replace(":", "");
        } else if line == "" {
            let mut edges = Vec::new();
            edges.push(
                (0..10)
                    .map(|x| tile_data.get(&(x, 0)).unwrap().clone())
                    .collect::<String>(),
            );
            edges.push(
                (0..10)
                    .map(|y| tile_data.get(&(9, y)).unwrap().clone())
                    .collect::<String>(),
            );
            edges.push(
                (0..10)
                    .map(|x| tile_data.get(&(x, 9)).unwrap().clone())
                    .collect::<String>(),
            );
            edges.push(
                (0..10)
                    .map(|y| tile_data.get(&(0, y)).unwrap().clone())
                    .collect::<String>(),
            );
            tiles.push(Tile {
                id: id.clone(),
                edges,
                data: tile_data.clone(),
            });
            y = 0;
            tile_data.clear();
            continue;
        } else {
            for (x, ch) in line.chars().enumerate() {
                tile_data.insert((x as i32, y), ch);
            }
            y += 1;
        }
    }

    // tile id, Vec<(tile id, side)>
    let mut tile_connections: HashMap<String, Vec<(String, String)>> = HashMap::new();
    let mut work = vec![tiles[0].id.clone()];
    loop {}
}
