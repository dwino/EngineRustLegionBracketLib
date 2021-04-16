use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(SpawningForager)]
#[write_component(Energy)]
#[read_component(Fruit)]
pub fn spawning_forager(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut forager_spawners = <(&Point, &Energy)>::query().filter(component::<SpawningForager>());
    let mut rng = RandomNumberGenerator::new();
    forager_spawners.iter_mut(ecs).for_each(|(pos, energy)| {
        if energy.current > energy.max {
            commands.push((
                *pos,
                Render {
                    color: ColorPair::new(
                        RGB::from_hex("#E3CF57").unwrap(),
                        RGB::from_hex("#D7E7D0").unwrap(),
                    ),
                    glyph: to_cp437('a'),
                },
                Name("Ant".to_string()),
                Creature {},
                FieldOfView::new(7),
                Health { current: 2, max: 2 },
                Energy {
                    current: 0,
                    max: 10,
                },
                // Targetable {},
                Foraging {},
            ));
            energy.current -= 1;
        }
    });
}
