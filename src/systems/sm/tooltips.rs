use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(FieldOfView)]
#[read_component(Player)]
#[read_component(Targeting)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &RlCamera) {
    let mut positions = <(Entity, &Point, &Name)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    positions
        .iter(ecs)
        .filter(|(_, pos, _)| **pos == map_pos)
        .for_each(|(entity, _, name)| {
            let screen_pos = *mouse_pos * TOOLTIP_SCALE;
            let display =
                if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                    format!("{} : {} hp", &name.0, health.current)
                } else {
                    name.0.clone()
                };
            draw_batch.print(screen_pos, &display);
        });

    if let Some(target) = <&Targeting>::query()
        .iter(ecs)
        .find_map(|targeting| targeting.current_target)
    {
        if let Ok(target_ref) = ecs.entry_ref(target) {
            let target_pos = target_ref.get_component::<Point>().unwrap();
            let screen_pos = (*target_pos - offset) * TOOLTIP_SCALE;
            draw_batch.print(screen_pos, "target".to_string());
        }
    }
    draw_batch.submit(10100).expect("Batch error");
}
