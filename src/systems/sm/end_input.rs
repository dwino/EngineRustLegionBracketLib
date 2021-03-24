use crate::prelude::*;

#[system]
#[read_component(WantsEndSmInput)]
pub fn end_input(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    #[resource] turn_state: &mut SmState,
) {
    if let Some((message_entity, new_state)) = <(Entity, &WantsEndSmInput)>::query()
        .iter(ecs)
        .find_map(|(message_entity, message)| Some((message_entity, message.0)))
    {
        *turn_state = new_state;
        commands.remove(*message_entity);
    }
}
