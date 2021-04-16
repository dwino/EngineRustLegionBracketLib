use crate::prelude::*;

pub fn spawn_entities(
    ecs: &mut World,
    map: &Map,
    nest_positions: &Vec<usize>,
    foraging_positions: &Vec<usize>,
) {
    let mut commands = legion::systems::CommandBuffer::new(ecs);

    let nest_points: Vec<Point> = nest_positions
        .iter()
        .map(|idx| map.index_to_point2d(*idx))
        .collect();
    let foraging_points: Vec<Point> = foraging_positions
        .iter()
        .map(|idx| map.index_to_point2d(*idx))
        .collect();

    for pt in nest_points {
        commands.push((
            pt,
            Render {
                color: ColorPair::new(
                    RGB::from_hex("#D3494E").unwrap(),
                    RGB::from_hex("#D7E7D0").unwrap(),
                ),
                glyph: to_cp437('•'),
            },
            Name("Nest".to_string()),
            Energy {
                current: 115,
                max: 100,
            },
            SpawningForager {},
        ));
    }

    for pt in foraging_points {
        commands.push((
            pt,
            Render {
                color: ColorPair::new(
                    RGB::from_hex("#E3CF57").unwrap(),
                    RGB::from_hex("#D7E7D0").unwrap(),
                ),
                glyph: to_cp437('♠'),
            },
            Name("Sustenance".to_string()),
            ForageSource {},
        ));
    }

    commands.flush(ecs);
}
