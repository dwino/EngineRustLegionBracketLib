use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Creature)]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Equiped)]
#[read_component(ProjectileStack)]
#[read_component(Weapon)]
#[read_component(FieldOfView)]
#[read_component(Targeting)]
#[read_component(WantsCycleTarget)]
pub fn input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
) {
    let (player_entity, player_pos) = <(Entity, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .find_map(|(entity, pos)| Some((*entity, *pos)))
        .unwrap();

    if let Some(key) = *key {
        match key {
            VirtualKeyCode::H => {
                move_or_attack(ecs, commands, player_entity, player_pos, Point::new(-1, 0))
            }
            VirtualKeyCode::L => {
                move_or_attack(ecs, commands, player_entity, player_pos, Point::new(1, 0))
            }
            VirtualKeyCode::K => {
                move_or_attack(ecs, commands, player_entity, player_pos, Point::new(0, -1))
            }
            VirtualKeyCode::J => {
                move_or_attack(ecs, commands, player_entity, player_pos, Point::new(0, 1))
            }
            VirtualKeyCode::Y => {
                move_or_attack(ecs, commands, player_entity, player_pos, Point::new(-1, -1))
            }
            VirtualKeyCode::U => {
                move_or_attack(ecs, commands, player_entity, player_pos, Point::new(1, -1))
            }
            VirtualKeyCode::B => {
                move_or_attack(ecs, commands, player_entity, player_pos, Point::new(-1, 1))
            }
            VirtualKeyCode::N => {
                move_or_attack(ecs, commands, player_entity, player_pos, Point::new(1, 1))
            }
            VirtualKeyCode::Space => wait(commands),
            VirtualKeyCode::G => pick_up_item(ecs, commands, player_entity, player_pos),
            VirtualKeyCode::F => shoot_or_throw(ecs, commands, player_entity),
            VirtualKeyCode::Tab => target(ecs, commands, player_entity),
            VirtualKeyCode::LShift => stop_targeting(commands),
            VirtualKeyCode::Key1 => use_item(0, ecs, commands, player_entity),
            VirtualKeyCode::Key2 => use_item(1, ecs, commands, player_entity),
            VirtualKeyCode::Key3 => use_item(2, ecs, commands, player_entity),
            VirtualKeyCode::Key4 => use_item(3, ecs, commands, player_entity),
            VirtualKeyCode::Key5 => use_item(4, ecs, commands, player_entity),
            VirtualKeyCode::Key6 => use_item(5, ecs, commands, player_entity),
            VirtualKeyCode::Key7 => use_item(6, ecs, commands, player_entity),
            VirtualKeyCode::Key8 => use_item(7, ecs, commands, player_entity),
            VirtualKeyCode::Key9 => use_item(8, ecs, commands, player_entity),
            _ => send_end_input_message(commands, RlState::AwaitingInput),
        };
    }
}
fn move_or_attack(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    player_entity: Entity,
    player_pos: Point,
    delta: Point,
) {
    let destination = player_pos + delta;
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Creature>());
    let mut hit_something = false;
    enemies
        .iter(ecs)
        .filter(|(_, pos)| **pos == destination)
        .for_each(|(entity, _)| {
            hit_something = true;

            commands.push((
                (),
                WantsToAttack {
                    attacker: player_entity,
                    victim: *entity,
                },
            ));
        });

    if !hit_something {
        commands.push((
            (),
            WantsToMove {
                entity: player_entity,
                destination,
            },
        ));
    }
    send_end_input_message(commands, RlState::PlayerTurn);
}

fn wait(commands: &mut CommandBuffer) {
    commands.push(((), WantsEndInput(RlState::PlayerTurn)));
}

fn pick_up_item(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    player_entity: Entity,
    player_pos: Point,
) {
    if let Some(item_entity) = <(Entity, &Item, &Point)>::query()
        .iter(ecs)
        .filter(|(_entity, _item, &item_pos)| item_pos == player_pos)
        .find_map(|(item_entity, _item, _item_pos)| Some(item_entity))
    {
        commands.remove_component::<Point>(*item_entity);
        commands.add_component(*item_entity, Carried(player_entity));
        send_end_input_message(commands, RlState::PlayerTurn);
    }
}

fn shoot_or_throw(ecs: &mut SubWorld, commands: &mut CommandBuffer, player_entity: Entity) {
    if let Some(target) = <&Targeting>::query()
        .iter(ecs)
        .find_map(|targeting| targeting.current_target)
    {
        match ecs.entry_ref(target) {
            Ok(_) => {
                commands.push((
                    (),
                    WantsToRangedAttack {
                        attacker: player_entity,
                        victim: target,
                    },
                ));
                send_end_input_message(commands, RlState::PlayerTurn);
            }
            Err(_) => {
                send_end_input_message(commands, RlState::AwaitingInput);
            }
        }
    }
}

fn target(ecs: &mut SubWorld, commands: &mut CommandBuffer, player_entity: Entity) {
    if let Some((_, projectile)) = <(&Equiped, &ProjectileStack)>::query()
        .iter(ecs)
        .filter(|(equiped, _)| equiped.0 == player_entity)
        .next()
    {
        if projectile.0 >= 1 {
            commands.push(((), WantsCycleTarget {}));
            send_end_input_message(commands, RlState::AwaitingInput);
        }
    }
}

fn stop_targeting(commands: &mut CommandBuffer) {
    commands.push(((), WantsEndTargeting {}));
    send_end_input_message(commands, RlState::AwaitingInput);
}

fn use_item(n: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer, player_entity: Entity) {
    let item_entity = <(Entity, &Item, &Carried)>::query()
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player_entity)
        .enumerate()
        .filter(|(item_count, (_, _, _))| *item_count == n)
        .find_map(|(_, (item_entity, _, _))| Some(*item_entity));

    if let Some(item_entity) = item_entity {
        commands.push((
            (),
            ActivateItem {
                used_by: player_entity,
                item: item_entity,
            },
        ));
        send_end_input_message(commands, RlState::PlayerTurn);
    }
}

fn send_end_input_message(commands: &mut CommandBuffer, new_state: RlState) {
    commands.push(((), WantsEndInput(new_state)));
}
