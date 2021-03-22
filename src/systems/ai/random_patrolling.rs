use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(PatrollingRandomly)]
#[read_component(FieldOfView)]
#[read_component(WantsToPatrolRandomly)]
#[read_component(Health)]
pub fn random_patrolling(#[resource] map: &Map, ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut wants_to_patrol = <(Entity, &WantsToPatrolRandomly)>::query();
    let mut patrol_entities: Vec<Entity> = wants_to_patrol
        .iter(ecs)
        .map(|(message_entity, message_component)| *message_entity)
        .collect();
    let mut movers = <(Entity, &Point, &mut PatrollingRandomly, &FieldOfView)>::query();
    let mut positions = <&Point>::query();

    let mut search_targets = Vec::<usize>::new();
    positions
        .iter(ecs)
        .filter(|pt| map.can_enter_tile(**pt))
        .map(|pt| map.point2d_to_index(*pt))
        .for_each(|idx| search_targets.push(idx));

    movers
        .iter_mut(ecs)
        .filter(|(e, _, _, _)| patrol_entities.contains(e))
        .for_each(|(entity, pos, patrol, _fov)| {
            let mut next_step = *pos;

            if let Some(path) = &mut patrol.path {
                if !path.is_empty() {
                    let next = path[0];
                    path.remove(0);
                    next_step = map.index_to_point2d(next);
                } else if !search_targets.is_empty() {
                    let mut rng = RandomNumberGenerator::new();
                    let target_idx = rng.random_slice_index(&search_targets).unwrap();
                    let target = search_targets[target_idx];
                    search_targets.remove(target_idx);

                    let start = map.point2d_to_index(*pos);
                    let end = target;
                    let finder = a_star_search(start, end, map);
                    if finder.success {
                        commands.add_component(
                            *entity,
                            PatrollingRandomly {
                                path: Some(finder.steps),
                            },
                        );
                    } else {
                        println!("Failed to find the path");
                    }
                }
            }
            commands.push((
                (),
                WantsToMove {
                    entity: *entity,
                    destination: next_step,
                },
            ));
        });
}
