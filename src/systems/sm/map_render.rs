use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
pub fn map_render(
    #[resource] map: &Map,
    #[resource] camera: &RlCamera,
    #[resource] theme: &Box<dyn MapTheme>,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if map.in_bounds(pt) {
                let idx = map.point2d_to_index(pt);
                let colorpair = theme.in_fov_colorpair_to_render(map.tiles[idx]);

                let glyph = theme.tile_to_render(map.tiles[idx]);
                draw_batch.set(
                    pt - offset,
                    ColorPair::new(colorpair.fg, colorpair.bg),
                    glyph,
                );
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}
