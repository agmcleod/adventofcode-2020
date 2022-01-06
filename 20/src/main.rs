use std::collections::HashSet;
use std::fmt::Debug;

use read_input::read_text;

type TileData = Vec<Vec<String>>;

#[derive(Clone)]
struct Tile {
    id: String,
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

fn do_tiles_align(tile_one: &Tile, tile_two: &Tile) -> bool {
    if tile_one.data[0] == tile_two.data[9] {
        true
    } else if tile_one.data[9] == tile_two.data[0] {
        true
    } else if tile_one.data[0][0] == tile_two.data[0][9]
        && tile_one.data[1][0] == tile_two.data[1][9]
        && tile_one.data[2][0] == tile_two.data[2][9]
        && tile_one.data[3][0] == tile_two.data[3][9]
        && tile_one.data[4][0] == tile_two.data[4][9]
        && tile_one.data[5][0] == tile_two.data[5][9]
        && tile_one.data[6][0] == tile_two.data[6][9]
        && tile_one.data[7][0] == tile_two.data[7][9]
        && tile_one.data[8][0] == tile_two.data[8][9]
        && tile_one.data[9][0] == tile_two.data[9][9]
    {
        true
    } else if tile_one.data[0][9] == tile_two.data[0][0]
        && tile_one.data[1][9] == tile_two.data[1][0]
        && tile_one.data[2][9] == tile_two.data[2][0]
        && tile_one.data[3][9] == tile_two.data[3][0]
        && tile_one.data[4][9] == tile_two.data[4][0]
        && tile_one.data[5][9] == tile_two.data[5][0]
        && tile_one.data[6][9] == tile_two.data[6][0]
        && tile_one.data[7][9] == tile_two.data[7][0]
        && tile_one.data[8][9] == tile_two.data[8][0]
        && tile_one.data[9][9] == tile_two.data[9][0]
    {
        true
    } else {
        false
    }
}

fn main() {
    let text = read_text("20/input.txt").unwrap();

    let mut tiles = Vec::new();
    let mut id = String::new();
    let mut tile_data = Vec::new();

    for line in text.lines() {
        if line.contains("Tile") {
            id = line.replace("Tile ", "").replace(":", "");
        } else if line == "" {
            tiles.push(Tile {
                id: id.clone(),
                data: tile_data.clone(),
            });
            tile_data.clear();
        } else {
            let mut row = Vec::new();
            for ch in line.chars() {
                row.push(ch.to_string());
            }

            tile_data.push(row);
        }
    }

    for tile in &tiles {
        let mut used_tiles = HashSet::new();
        for other_tile in &tiles {
            if other_tile.id == tile.id {
                continue;
            }

            let mut other_tile = other_tile.clone();

            let mut count = 0;
            let mut found = true;
            while !do_tiles_align(tile, &other_tile) {
                rotate_tile_data_by_90(&mut other_tile.data);
                if count == 4 {
                    flip_tile_data(&mut other_tile.data);
                }

                if count >= 7 {
                    found = false;
                    break;
                }
                count += 1;
            }

            if found {
                used_tiles.insert(other_tile.id);
            }
        }
    }
}
