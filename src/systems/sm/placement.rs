use crate::prelude::*;

#[system(for_each)]
#[read_component(Item)]
#[read_component(Carried)]
pub fn placement(
    entity: &Entity,
    want_place: &WantsToPlace,
    #[resource] map: &mut Map,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    let idx = map.point2d_to_index(want_place.destination);
    let mut item_query = <(Entity, &Item, &Carried)>::query();
    if map.tiles[idx] == TileType::Wall {
        if let Some(produce) = item_query.iter(ecs).find_map(|(entity, _, _)| Some(entity)) {
            commands.remove_component::<Carried>(*produce);
            commands.add_component(*produce, want_place.destination);
        }
    }
    commands.remove(*entity);
}
