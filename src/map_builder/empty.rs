use super::MapArchitect;
use crate::prelude::*;

pub struct EmptyArchitect {}

impl MapArchitect for EmptyArchitect {
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
        mb.exit_start = mb.find_most_distant();
        for _ in 0..50 {
            mb.spawns.push(Point::new(
                rng.range(1, SCREEN_WIDTH),
                rng.range(1, SCREEN_HEIGHT),
            ))
        }
        mb
    }
}
