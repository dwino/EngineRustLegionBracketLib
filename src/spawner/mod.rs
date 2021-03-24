use crate::prelude::*;
mod template;
pub use template::*;
mod sm_template;
pub use sm_template::*;
mod foraging_spawner;
pub use foraging_spawner::*;

pub fn spawn_employee(ecs: &mut World, pos: Point) {
    let mut commands = legion::systems::CommandBuffer::new(ecs);

    let player = ecs.push((
        Player { map_level: 0 },
        pos,
        Render {
            color: ColorPair::new(
                RGB::from_hex("#D7E7D0").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),
            glyph: to_cp437('@'),
        },
        Health {
            current: 100,
            max: 100,
        },
    ));

    commands.add_component(player, Defense(1));
}

pub fn spawn_store_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point],
) {
    let template = SmTemplates::load();
    template.spawn_entities(ecs, rng, level, spawn_points);
}

pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point],
) {
    let template = Templates::load();
    template.spawn_entities(ecs, rng, level, spawn_points);
}

pub fn spawn_player(ecs: &mut World, pos: Point) {
    let mut commands = legion::systems::CommandBuffer::new(ecs);

    let player = ecs.push((
        Player { map_level: 0 },
        pos,
        Render {
            color: ColorPair::new(
                RGB::from_hex("#D7E7D0").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),
            glyph: to_cp437('@'),
        },
        Health {
            current: 100,
            max: 100,
        },
        Targeting {
            targets: Vec::new(),
            current_target: None,
            index: usize::MAX,
        },
        FieldOfView::new(9),
        TargetRange::new(5),
        Damage(1),
    ));

    commands.add_component(player, Defense(1));
}

pub fn spawn_foraging_level(
    ecs: &mut World,
    map: &Map,
    nest_position: &Vec<usize>,
    foraging_positions: &Vec<usize>,
) {
    foraging_spawner::spawn_entities(ecs, map, nest_position, foraging_positions)
}

pub fn spawn_magic_droplet(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        MagicDroplet,
        pos,
        Render {
            color: ColorPair::new(
                RGB::from_hex("#5D76CB").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),
            glyph: to_cp437('♥'),
        },
        Name("Magic Droplet".to_string()),
    ));
}
