use crate::prelude::*;

#[system(for_each)]
#[read_component(FieldOfView)]
pub fn digging(
    entity: &Entity,
    want_dig: &WantsToDig,
    #[resource] map: &mut Map,
    commands: &mut CommandBuffer,
) {
    let idx = map.point2d_to_index(want_dig.destination);
    map.tiles[idx] = TileType::Floor;
    commands.remove(*entity);
}
