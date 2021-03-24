use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Equiped)]
#[read_component(Name)]
#[read_component(Entity)]
#[read_component(ProjectileStack)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query.iter(ecs).next().unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_centered(1, "Explore! vi-keys to move / melee");
    draw_batch.print_centered(2, "g gets an item, 1-9 uses / equips");

    draw_batch.print_centered(3, "hover over entities for info");
    draw_batch.print_centered(4, "tab cycles targets (Lshift stops targetting)");
    draw_batch.print_centered(5, "f fires first equiped(!) darts");

    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!(
            " Health: {} / {} ",
            player_health.current, player_health.max
        ),
        ColorPair::new(WHITE, RED),
    );

    let (player, map_level) = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, player)| Some((*entity, player.map_level)))
        .unwrap();

    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 1),
        format!("Dungeon Level: {}", map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );

    let mut item_query = <(&Item, &Name, &Carried)>::query();
    let mut y = 3;
    item_query
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player)
        .for_each(|(_, name, _)| {
            draw_batch.print(Point::new(3, y), format!("{} : {}", y - 2, &name.0));
            y += 1;
        });

    draw_batch.print_color(
        Point::new(3, 2),
        "Items carried",
        ColorPair::new(YELLOW, BLACK),
    );

    draw_batch.print_color(
        Point::new(3, y),
        "Items equiped",
        ColorPair::new(YELLOW, BLACK),
    );

    y += 1;

    let mut equipment_query = <(Entity, &Item, &Name, &Equiped)>::query();
    equipment_query
        .iter(ecs)
        .filter(|(_, _, _, equiped)| equiped.0 == player)
        .for_each(|(entity, _, name, _)| {
            if let Ok(e) = ecs.entry_ref(*entity) {
                if let Ok(proj) = e.get_component::<ProjectileStack>() {
                    draw_batch.print(
                        Point::new(3, y),
                        format!("{} : {} #{}", y - 3, &name.0, proj.0),
                    );
                }
            }
            draw_batch.print(Point::new(3, y), format!("{} : {}", y - 3, &name.0));
            y += 1;
        });

    draw_batch.submit(10000).expect("Batch error");
}
