use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use read_input::read_text;

type TileData = Vec<Vec<String>>;

#[derive(Clone)]
struct Tile {
    id: i64,
    data: TileData,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tile: {}\n{}\n\n",
            self.id,
            self.data
                .iter()
                .map(|row| { row.join("") })
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

fn flip_tile_data<'a>(grid: &mut TileData) {
    for row in grid.iter_mut() {
        row.reverse();
    }
}

fn rotate_tile_data_by_90(grid: &mut TileData) {
    let mut rotated = Vec::new();
    let n = grid.len();

    for i in 0..n {
        rotated.push(Vec::new());
        for j in 0..n {
            rotated[i].push(grid[n - j - 1][i].clone());
        }
    }

    *grid = rotated;
}

fn tile_aligns_on_left(tile_one: &Tile, tile_two: &Tile) -> bool {
    tile_one.data[0][9] == tile_two.data[0][0]
        && tile_one.data[1][9] == tile_two.data[1][0]
        && tile_one.data[2][9] == tile_two.data[2][0]
        && tile_one.data[3][9] == tile_two.data[3][0]
        && tile_one.data[4][9] == tile_two.data[4][0]
        && tile_one.data[5][9] == tile_two.data[5][0]
        && tile_one.data[6][9] == tile_two.data[6][0]
        && tile_one.data[7][9] == tile_two.data[7][0]
        && tile_one.data[8][9] == tile_two.data[8][0]
        && tile_one.data[9][9] == tile_two.data[9][0]
}

fn tile_aligns_on_top(tile_one: &Tile, tile_two: &Tile) -> bool {
    tile_one.data[9] == tile_two.data[0]
}

fn try_next_tile(
    grid_size: i32,
    tiles: &Vec<Tile>,
    coord: (i32, i32),
    mut tile: Tile,
    tile_layout: HashMap<(i32, i32), Tile>,
    used_tiles: HashSet<i64>,
) -> Option<HashMap<(i32, i32), Tile>> {
    for n in 0..8 {
        // if tile above & to the left dont exist or they line up
        if (!tile_layout.contains_key(&(coord.0 - 1, coord.1))
            || tile_aligns_on_left(tile_layout.get(&(coord.0 - 1, coord.1)).unwrap(), &tile))
            && (!tile_layout.contains_key(&(coord.0, coord.1 - 1))
                || tile_aligns_on_top(tile_layout.get(&(coord.0, coord.1 - 1)).unwrap(), &tile))
        {
            let mut used_tiles = used_tiles.clone();
            used_tiles.insert(tile.id);
            let mut tile_layout = tile_layout.clone();
            tile_layout.insert(coord.clone(), tile.clone());

            if tile_layout.len() as i32 == grid_size * grid_size {
                return Some(tile_layout);
            }
            for other in tiles {
                if used_tiles.contains(&other.id) {
                    continue;
                }

                let mut coord = coord.clone();
                coord.0 += 1;
                if coord.0 >= grid_size {
                    coord.0 = 0;
                    coord.1 += 1;
                    if coord.1 >= grid_size {
                        panic!("Outside of target grid");
                    }
                }

                let res = try_next_tile(
                    grid_size,
                    tiles,
                    coord,
                    other.clone(),
                    tile_layout.clone(),
                    used_tiles.clone(),
                );

                if res.is_some() {
                    return res;
                }
            }
        }

        rotate_tile_data_by_90(&mut tile.data);
        // flip after rotating 4th time
        if n == 3 {
            flip_tile_data(&mut tile.data);
        }
    }

    None
}

fn main() {
    let text = read_text("20/input.txt").unwrap();

    let mut tiles = Vec::new();
    let mut id: i64 = 0;
    let mut tile_data = Vec::new();

    for line in text.lines() {
        if line.contains("Tile") {
            id = line.replace("Tile ", "").replace(":", "").parse().unwrap();
        } else if line == "" {
            tiles.push(Tile {
                id: id.clone(),
                data: tile_data.clone(),
            });
            tile_data.clear();
            id = 0;
        } else {
            let mut row = Vec::new();
            for ch in line.chars() {
                row.push(ch.to_string());
            }

            tile_data.push(row);
        }
    }

    if id != 0 {
        tiles.push(Tile {
            id: id.clone(),
            data: tile_data.clone(),
        });
    }

    let grid_size = (tiles.len() as f32).sqrt().round() as i32;
    for tile in &tiles {
        let res = try_next_tile(
            grid_size,
            &tiles,
            (0, 0),
            tile.clone(),
            HashMap::new(),
            HashSet::new(),
        );

        let grid_size_idx = grid_size - 1;
        if res.is_some() {
            let layout = res.unwrap();
            println!(
                "{}",
                layout.get(&(0, 0)).unwrap().id
                    * layout.get(&(grid_size_idx, 0)).unwrap().id
                    * layout.get(&(0, grid_size_idx)).unwrap().id
                    * layout.get(&(grid_size_idx, grid_size_idx)).unwrap().id
            );
            break;
        }
    }
}
