use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(MagicDroplet)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut RlState, #[resource] map: &Map) {
    let mut player_hp = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut amulet = <&Point>::query().filter(component::<MagicDroplet>());
    let current_state = *turn_state;

    let mut new_state = match current_state {
        RlState::AwaitingInput => return,
        RlState::PlayerTurn => RlState::MonsterTurn,
        RlState::MonsterTurn => RlState::AwaitingInput,
        _ => current_state,
    };

    let amulet_default = Point::new(-1, -1);
    let amulet_pos = amulet.iter(ecs).next().unwrap_or(&amulet_default);

    player_hp.iter(ecs).for_each(|(hp, pos)| {
        if hp.current < 1 {
            new_state = RlState::GameOver;
        }
        if pos == amulet_pos {
            new_state = RlState::Victory;
        }
        let idx = map.point2d_to_index(*pos);
        if map.tiles[idx] == TileType::Exit {
            new_state = RlState::NextLevel;
        }
    });

    *turn_state = new_state;
}
