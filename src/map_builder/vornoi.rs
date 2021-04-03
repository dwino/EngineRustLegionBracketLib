use super::MapArchitect;
use crate::prelude::*;

pub struct VornoiArchitect {}

impl MapArchitect for VornoiArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(SCREEN_WIDTH, SCREEN_HEIGHT),
            rooms: Vec::new(),
            player_start: Point::zero(),
            exit_start: Point::zero(),
            spawns: Vec::new(),
            theme: super::themes::RootedTheme::new(),
        };

        mb.fill(TileType::Floor);

        let mut seeds = Vec::new();
        for _ in 0..16 {
            seeds.push(Point::new(
                rng.range(1, SCREEN_WIDTH - 1),
                rng.range(1, SCREEN_HEIGHT - 1),
            ));
        }

        mb.player_start = seeds[0];
        mb.player_start = seeds[seeds.len() - 1];

        let mut membership = vec![0; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize];
        for (i, m) in membership.iter_mut().enumerate() {
            let my_pos = Point::new(i % SCREEN_WIDTH as usize, i / SCREEN_HEIGHT as usize);
            let closest = seeds
                .iter()
                .enumerate()
                .map(|(i, pos)| (i, DistanceAlg::Pythagoras.distance2d(my_pos, *pos)))
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap()
                .0;
            *m = closest;
        }
        for i in 0..SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize {
            let my_pos = mb.map.index_to_point2d(i);

            if my_pos.x == 0
                || my_pos.x == SCREEN_WIDTH as i32 - 1
                || my_pos.y == 0
                || my_pos.y == SCREEN_HEIGHT as i32 - 1
            {
                mb.map.tiles[i] = TileType::Wall;
            } else {
                if membership[i] != membership[i + 1]
                    || membership[i] != membership[i + SCREEN_WIDTH as usize]
                {
                    mb.map.tiles[i] = TileType::Wall;
                }
            }
        }

        let start = find_start();
        mb.spawns = mb.spawns(start, rng);
        mb
    }
}

fn find_start() -> Point {
    Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)
}
