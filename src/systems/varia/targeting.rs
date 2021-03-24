use crate::prelude::*;

#[system]
#[read_component(WantsCycleTarget)]
#[read_component(WantsEndTargeting)]
#[read_component(Point)]
#[read_component(TargetRange)]
#[read_component(Targeting)]
#[read_component(Targetable)]
#[read_component(Player)]
#[read_component(Entity)]

pub fn targetting(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    if let Some(message) = <(Entity, &WantsCycleTarget)>::query()
        .iter(ecs)
        .map(|(message_entity, _)| message_entity)
        .next()
    {
        let (player_entity, player_pos, player_target_range, player_targetting) =
            <(Entity, &Point, &TargetRange, &Targeting)>::query()
                .filter(component::<Player>())
                .iter(ecs)
                .find_map(|(e, p, tr, t)| Some((e, p, tr, t)))
                .unwrap();

        let mut possible_targets = <(Entity, &Targetable, &Point)>::query();
        let mut targets = possible_targets
            .iter(ecs)
            .filter(|(_, _, pt)| player_target_range.reachable_tiles.contains(&pt))
            .map(|(e, _, pt)| (*e, DistanceAlg::Pythagoras.distance2d(*player_pos, *pt)))
            .collect::<Vec<(Entity, f32)>>();

        targets.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let mut new_index = 0;
        let current_target = if targets.is_empty() {
            None
        } else {
            let target;
            if player_targetting.index > targets.len() - 1 && player_targetting.index < usize::MAX {
                new_index = usize::MAX;
                target = None
            } else {
                if player_targetting.index == usize::MAX
                    || player_targetting.index == targets.len() - 1
                {
                    new_index = 0;
                } else {
                    new_index = player_targetting.index + 1;
                }
                target = Some(targets[new_index].0);
            }
            target
        };
        commands.add_component(
            *player_entity,
            Targeting {
                targets,
                current_target,
                index: new_index,
            },
        );
        commands.remove(*message);
    }

    if let Some(message) = <(Entity, &WantsEndTargeting)>::query()
        .iter(ecs)
        .map(|(message_entity, _)| message_entity)
        .next()
    {
        if let Some(targeting_entity) = <(Entity, &Targeting)>::query()
            .iter(ecs)
            .find_map(|(targeting_entity, targeting_component)| Some(targeting_entity))
        {
            commands.add_component(
                *targeting_entity,
                Targeting {
                    targets: Vec::new(),
                    current_target: None,
                    index: usize::MAX,
                },
            )
        }
    }
}
