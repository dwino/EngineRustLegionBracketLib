use super::MapArchitect;
use crate::prelude::*;
use std::cmp::{max, min};

const NUM_ROOMS: usize = 20;

pub struct RoomsArchitect {}

impl MapArchitect for RoomsArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(SCREEN_WIDTH, SCREEN_HEIGHT),
            rooms: Vec::new(),
            player_start: Point::zero(),
            exit_start: Point::zero(),
            spawns: Vec::new(),
            theme: super::themes::RootedTheme::new(),
        };

        mb.fill(TileType::Wall);
        build_random_rooms(&mut mb, rng);
        build_corridors(&mut mb, rng);
        mb.player_start = mb.rooms[0].center();
        mb.exit_start = mb.find_most_distant();
        for room in mb.rooms.iter().skip(1) {
            mb.spawns.push(room.center());
        }

        mb
    }
}

fn build_random_rooms(map_builder: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    while map_builder.rooms.len() < NUM_ROOMS {
        let room = Rect::with_size(
            rng.range(1, SCREEN_WIDTH - 10),
            rng.range(1, SCREEN_HEIGHT - 10),
            rng.range(2, 10),
            rng.range(2, 10),
        );
        let mut overlap = false;
        for r in &map_builder.rooms {
            if r.intersect(&room) {
                overlap = true;
            }
        }
        if !overlap {
            room.for_each(|p| {
                if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                    let idx = map_builder.map.point2d_to_index(p);
                    map_builder.map.tiles[idx] = TileType::Floor;
                }
            });

            map_builder.rooms.push(room)
        }
    }
}

fn apply_horizontal_tunnel(map_builder: &mut MapBuilder, x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        if let Some(idx) = map_builder.map.try_idx(Point::new(x, y)) {
            map_builder.map.tiles[idx as usize] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map_builder: &mut MapBuilder, y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        if let Some(idx) = map_builder.map.try_idx(Point::new(x, y)) {
            map_builder.map.tiles[idx as usize] = TileType::Floor;
        }
    }
}

fn build_corridors(map_builder: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    let mut rooms = map_builder.rooms.clone();
    rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

    for (i, room) in rooms.iter().enumerate().skip(1) {
        let prev = rooms[i - 1].center();
        let new = room.center();

        if rng.range(0, 2) == 1 {
            apply_horizontal_tunnel(map_builder, prev.x, new.x, prev.y);
            apply_vertical_tunnel(map_builder, prev.y, new.y, new.x);
        } else {
            apply_vertical_tunnel(map_builder, prev.y, new.y, prev.x);
            apply_horizontal_tunnel(map_builder, prev.x, new.x, new.y);
        }
    }
}
