use super::MapArchitect;
use crate::prelude::*;

const STORE: (&str, i32, i32) = (
    "
-------------------------------------
-------------------------------------
---▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓---
-------------------------------------
-------------------------------------
---▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓---
-------------------------------------
-------------------------------------
---▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓---
-------------------------------------
-------------------------------------
---▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓---
-------------------------------------
-------------------------------------
---▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓---
-------------------------------------
-------------------------------------
---▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓---
-------------------------------------
-------------------------------------
---▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓---
-------------------------------------
-------------------------------------
---▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓---
-------------------------------------
-------------------------------------
",
    37,
    26,
);

pub struct StoreArchitect {}

impl MapArchitect for StoreArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(SCREEN_WIDTH, SCREEN_HEIGHT),
            rooms: Vec::new(),
            spawns: Vec::new(),
            player_start: Point::zero(),
            exit_start: Point::zero(),
            theme: super::themes::RootedTheme::new(),
        };
        mb.fill(TileType::Floor);
        mb.player_start = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);

        let x = (SCREEN_WIDTH / 2) - (STORE.1 / 2);
        let y = (SCREEN_HEIGHT / 2) - (STORE.2 / 2);
        let placement = Point::new(x, y);
        let string_vec: Vec<char> = STORE
            .0
            .chars()
            .filter(|a| *a != '\r' && *a != '\n')
            .collect();
        let mut i = 0;
        for ty in placement.y..placement.y + STORE.2 {
            for tx in placement.x..placement.x + STORE.1 {
                let idx = mb.map.point2d_to_index(Point::new(tx, ty));
                let c = string_vec[i];
                match c {
                    '-' => mb.map.tiles[idx] = TileType::Floor,
                    '▓' => mb.map.tiles[idx] = TileType::Shelve,
                    '#' => mb.map.tiles[idx] = TileType::Wall,
                    'p' => mb.spawns.push(Point::new(tx, ty)),
                    _ => println!("No idea what to do with [{}]", c),
                }
                i += 1;
            }
        }

        const NUM_ENTITIES: usize = 200;
        let mut spawnable_tiles: Vec<Point> = mb
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, t)| **t == TileType::Shelve)
            .map(|(idx, _)| mb.map.index_to_point2d(idx))
            .collect();

        let mut spawns = Vec::new();
        for _ in 0..NUM_ENTITIES {
            let target_index = rng.random_slice_index(&spawnable_tiles).unwrap();
            spawns.push(spawnable_tiles[target_index]);
            spawnable_tiles.remove(target_index);
        }
        mb.spawns = spawns;

        mb
    }
}
