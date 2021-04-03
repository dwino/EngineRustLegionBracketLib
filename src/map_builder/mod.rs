use crate::prelude::*;
mod empty;
mod rooms;
use rooms::RoomsArchitect;
mod automata;
use automata::CellularAutomataArchitect;
mod drunkard;
use drunkard::DrunkardsWalkArchitect;
mod prefab;
use prefab::apply;
mod themes;
pub use themes::*;
mod vornoi;
pub use vornoi::VornoiArchitect;
mod full;
pub use full::EmptyForagingArchitect;
mod store;
pub use store::StoreArchitect;

use self::empty::EmptyArchitect;

const UNREACHABLE: &f32 = &f32::MAX;

trait MapArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

pub trait MapTheme: Sync + Send {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType;
    fn in_fov_colorpair_to_render(&self, tile_type: TileType) -> ColorPair;
    fn out_fov_colorpair_to_render(&self, tile_type: TileType) -> ColorPair;
}

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub spawns: Vec<Point>,
    pub player_start: Point,
    pub exit_start: Point,
    pub theme: Box<dyn MapTheme>,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut architect: Box<dyn MapArchitect> = match rng.range(0, 4) {
            0 | 1 | 2 => Box::new(StoreArchitect {}),
            _ => Box::new(StoreArchitect {}),
        };
        let mut mb = architect.new(rng);
        // apply(&mut mb, rng);

        mb.theme = RootedTheme::new();

        // mb.tile_variety(rng);

        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn find_most_distant(&self) -> Point {
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &[self.map.point2d_to_index(self.player_start)],
            &self.map,
            1024.0,
        );

        self.map.index_to_point2d(
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist < UNREACHABLE)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0,
        )
    }

    // fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
    //     while self.rooms.len() < NUM_ROOMS {
    //         let room = Rect::with_size(
    //             rng.range(1, SCREEN_WIDTH - 10),
    //             rng.range(1, SCREEN_HEIGHT - 10),
    //             rng.range(2, 10),
    //             rng.range(2, 10),
    //         );
    //         let mut overlap = false;
    //         for r in &self.rooms {
    //             if r.intersect(&room) {
    //                 overlap = true;
    //             }
    //         }
    //         if !overlap {
    //             room.for_each(|p| {
    //                 if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
    //                     let idx = self.map.point2d_to_index(p);
    //                     self.map.tiles[idx] = TileType::Floor;
    //                 }
    //             });

    //             self.rooms.push(room)
    //         }
    //     }
    // }

    // fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
    //     use std::cmp::{max, min};
    //     for x in min(x1, x2)..=max(x1, x2) {
    //         if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
    //             self.map.tiles[idx as usize] = TileType::Floor;
    //         }
    //     }
    // }

    // fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
    //     use std::cmp::{max, min};
    //     for y in min(y1, y2)..=max(y1, y2) {
    //         if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
    //             self.map.tiles[idx as usize] = TileType::Floor;
    //         }
    //     }
    // }

    // fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
    //     let mut rooms = self.rooms.clone();
    //     rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

    //     for (i, room) in rooms.iter().enumerate().skip(1) {
    //         let prev = rooms[i - 1].center();
    //         let new = room.center();

    //         if rng.range(0, 2) == 1 {
    //             self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
    //             self.apply_vertical_tunnel(prev.y, new.y, new.x);
    //         } else {
    //             self.apply_vertical_tunnel(prev.y, new.y, prev.x);
    //             self.apply_horizontal_tunnel(prev.x, new.x, new.y);
    //         }
    //     }
    // }

    fn spawns(&self, start: Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        const NUM_ENTITIES: usize = 50;
        let mut spawnable_tiles: Vec<Point> = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, t)| {
                **t == TileType::Floor
                    && DistanceAlg::Pythagoras.distance2d(start, self.map.index_to_point2d(*idx))
                        > 10.0
            })
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();

        let mut spawns = Vec::new();
        for _ in 0..NUM_ENTITIES {
            let target_index = rng.random_slice_index(&spawnable_tiles).unwrap();
            spawns.push(spawnable_tiles[target_index]);
            spawnable_tiles.remove(target_index);
        }
        spawns
    }

    fn tile_variety(&mut self, rng: &mut RandomNumberGenerator) {
        let floor_tiletypes = [
            TileType::FloorVar1,
            TileType::FloorVar2,
            TileType::FloorVar3,
        ];
        let mut floor_tiles: Vec<Point> = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();

        let var_tile_amount = floor_tiles.len() / 10;

        for _ in 0..var_tile_amount {
            let idx = rng.random_slice_index(&floor_tiles).unwrap();
            let point = floor_tiles[idx];
            let point_idx = self.map.point2d_to_index(point);
            self.map.tiles[point_idx] = *rng.random_slice_entry(&floor_tiletypes).unwrap();
            floor_tiles.remove(idx);
        }

        let wall_tiletypes = [TileType::WallVar1, TileType::WallVar2, TileType::WallVar3];
        let mut wall_tiles: Vec<Point> = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Wall)
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();

        let var_wall_amount = wall_tiles.len() / 10;

        for _ in 0..var_wall_amount {
            let idx = rng.random_slice_index(&wall_tiles).unwrap();
            let point = wall_tiles[idx];
            let point_idx = self.map.point2d_to_index(point);
            self.map.tiles[point_idx] = *rng.random_slice_entry(&wall_tiletypes).unwrap();
            wall_tiles.remove(idx);
        }
    }
}
