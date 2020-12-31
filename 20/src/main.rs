use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Debug;

use read_input::read_text;

#[derive(Clone)]
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

struct Connection {
    tile_id: String,
    side: String,
}

struct Work {
    level: usize,
    tile_id: String,
    tiles: Vec<Tile>,
    scanned_tiles: HashSet<String>,
    tile_connections: HashMap<String, Vec<Connection>>,
}

impl Work {
    fn new(
        level: usize,
        tile_id: String,
        tiles: Vec<Tile>,
        scanned_tiles: HashSet<String>,
        tile_connections: HashMap<String, Vec<Connection>>,
    ) -> Self {
        Work {
            level,
            tile_id,
            tiles,
            scanned_tiles,
            tile_connections,
        }
    }
}

impl Ord for Work {
    fn cmp(&self, other: &Self) -> Ordering {
        self.level.cmp(&other.level)
    }
}

impl PartialOrd for Work {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Work {}

impl PartialEq for Work {
    fn eq(&self, other: &Self) -> bool {
        self.level == other.level
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

    let mut work_heap = BinaryHeap::new();
    work_heap.push(Work::new(
        0,
        tiles[0].id.clone(),
        tiles.clone(),
        HashSet::new(),
        HashMap::new(),
    ));
    loop {
        let next_work = work_heap.pop();

        if next_work.is_none() {
            break;
        }

        let work = next_work.unwrap();

        for tile in &tiles {
            if work.scanned_tiles.contains(&tile.id) {
                continue;
            }
        }
    }
}
