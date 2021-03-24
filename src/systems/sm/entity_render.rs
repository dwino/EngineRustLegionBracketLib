use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(#[resource] camera: &RlCamera, ecs: &SubWorld) {
    let mut renderables = <(&Point, &Render)>::query();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    renderables.iter(ecs).for_each(|(pos, render)| {
        draw_batch.set(*pos - offset, render.color, render.glyph);
    });

    draw_batch.submit(5000).expect("Batch error");
}
