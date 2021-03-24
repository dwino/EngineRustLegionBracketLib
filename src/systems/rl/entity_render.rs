use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
#[read_component(Plant)]
pub fn entity_render(#[resource] camera: &RlCamera, ecs: &SubWorld) {
    let mut renderables = <(&Point, &Render)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    let player_fov = fov.iter(ecs).next().unwrap();

    renderables
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    if player_fov.sensing {
        <(&Point, &Render, &Plant)>::query()
            .iter(ecs)
            .for_each(|(pos, render, _)| {
                draw_batch.set(*pos - offset, render.color, render.glyph);
            });
    }

    draw_batch.submit(5000).expect("Batch error");
}
