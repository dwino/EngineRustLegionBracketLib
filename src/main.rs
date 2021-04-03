#![warn(clippy::pedantic)]
use std::collections::HashSet;

mod components;
mod dkm;
mod eco_camera;
mod eco_state;
mod forage_map;
mod game_mode;
mod map;
mod map_builder;
mod rl_camera;
mod rl_state;
mod sm_state;
mod spawner;
mod systems;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT;
    pub const TILE_DIMENSIONS_MAP: i32 = 13;
    pub const TILE_DIMENSIONS_TOOLTIP: i32 = TILE_DIMENSIONS_MAP / 2;
    pub const TOOLTIP_SCALE: i32 = TILE_DIMENSIONS_MAP / TILE_DIMENSIONS_TOOLTIP;
    pub use crate::components::*;
    pub use crate::dkm::*;
    pub use crate::eco_camera::*;
    pub use crate::eco_state::*;
    pub use crate::forage_map::*;
    pub use crate::game_mode::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::rl_camera::*;
    pub use crate::rl_state::*;
    pub use crate::sm_state::*;
    pub use crate::spawner::FruitType;
    pub use crate::spawner::*;
    pub use crate::systems::*;
}

use prelude::*;

struct State {
    ecs: World,
    mode: GameMode,
    resources: Resources,
    sm_input_systems: Schedule,
    sm_employee_systems: Schedule,
    sm_customers_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let ecs = World::default();
        let resources = Resources::default();
        Self {
            ecs,
            resources,
            mode: GameMode::Menu,
            sm_input_systems: build_sm_input_scheduler(),
            sm_employee_systems: build_sm_employee_scheduler(),
            sm_customers_systems: build_sm_customers_scheduler(),
        }
    }

    //MODE:MENU
    ///////////
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.print_centered(5, "Superm@rket");
        ctx.print_centered(8, "(R) Roguelike Mode");
        ctx.print_centered(10, "(Q) Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::R => self.supermarket_mode(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn supermarket_mode(&mut self) {
        self.init_sm_game_state();
        self.mode = GameMode::RogueLike;
    }

    //MODE:ROGUELIKE
    ////////////////
    fn init_sm_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        spawn_employee(&mut self.ecs, map_builder.player_start);
        spawn_store_level(&mut self.ecs, &mut rng, 0, &map_builder.spawns);
        self.resources.insert(map_builder.map);
        self.resources
            .insert(RlCamera::new(map_builder.player_start));
        self.resources.insert(SmState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }

    fn execute_rl_game_state(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        let current_state = *self.resources.get::<SmState>().unwrap();
        match current_state {
            SmState::AwaitingInput => self
                .sm_input_systems
                .execute(&mut self.ecs, &mut self.resources),
            SmState::EmployeeTurn => {
                self.sm_employee_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            SmState::CustomerTurn => self
                .sm_customers_systems
                .execute(&mut self.ecs, &mut self.resources),
            SmState::GameOver => {
                self.sm_game_over(ctx);
            }
            SmState::Victory => {
                self.sm_victory(ctx);
            }
            SmState::NextLevel => {
                // self.sm_advance_level();
            }
        }
        render_draw_buffer(ctx).expect("Render error");
    }

    // fn sm_advance_level(&mut self) {
    //     let player_entity = *<Entity>::query()
    //         .filter(component::<Player>())
    //         .iter(&self.ecs)
    //         .next()
    //         .unwrap();

    //     let mut entities_to_keep = HashSet::new();
    //     entities_to_keep.insert(player_entity);
    //     <(Entity, &Carried)>::query()
    //         .iter(&self.ecs)
    //         .filter(|(_e, carry)| carry.0 == player_entity)
    //         .map(|(e, _carry)| *e)
    //         .for_each(|e| {
    //             entities_to_keep.insert(e);
    //         });
    //     <(Entity, &Equiped)>::query()
    //         .iter(&self.ecs)
    //         .filter(|(_e, carry)| carry.0 == player_entity)
    //         .map(|(e, _carry)| *e)
    //         .for_each(|e| {
    //             entities_to_keep.insert(e);
    //         });

    //     let mut cb = CommandBuffer::new(&self.ecs);
    //     for e in Entity::query().iter(&self.ecs) {
    //         if !entities_to_keep.contains(e) {
    //             cb.remove(*e);
    //         }
    //     }
    //     cb.flush(&mut self.ecs);

    //     <&mut FieldOfView>::query()
    //         .iter_mut(&mut self.ecs)
    //         .for_each(|fov| {
    //             fov.is_dirty = true;
    //             fov.sensing = false;
    //         });

    //     let mut rng = RandomNumberGenerator::new();
    //     let mut map_builder = MapBuilder::new(&mut rng);
    //     let mut map_level = 0;
    //     <(&mut Player, &mut Point)>::query()
    //         .iter_mut(&mut self.ecs)
    //         .for_each(|(player, pos)| {
    //             player.map_level += 1;
    //             map_level = player.map_level;
    //             pos.x = map_builder.player_start.x;
    //             pos.y = map_builder.player_start.y;
    //         });
    //     spawn_level(
    //         &mut self.ecs,
    //         &mut rng,
    //         map_level as usize,
    //         &map_builder.spawns,
    //     );
    //     self.resources.insert(map_builder.map);
    //     self.resources
    //         .insert(RlCamera::new(map_builder.player_start));
    //     self.resources.insert(SmState::AwaitingInput);
    //     self.resources.insert(map_builder.theme);
    // }

    fn sm_game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "Slain by a monster, root's journey has come to a premature end.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "The Magic Droplet remains unclaimed, and Home Tree is still dying.",
        );
        ctx.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "Don't worry, you can always try again with a new root.",
        );
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.init_sm_game_state();
        }
    }

    fn sm_victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "You absord the Magic Droplet and feel its power course through your veins.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "You return to Home Tree and rejoin it's rootsystem. It immediatly starts growing strong again!",
        );
        ctx.print_color_centered(7, GREEN, BLACK, "Press 1 to play again.");
        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.init_sm_game_state();
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::RogueLike => self.execute_rl_game_state(ctx),
            _ => {}
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(TILE_DIMENSIONS_MAP, TILE_DIMENSIONS_MAP)
        .with_resource_path("resources/")
        .with_font("Kren_13x13.png", 13, 13)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "Kren_13x13.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "Kren_13x13.png")
        .with_simple_console_no_bg(
            DISPLAY_WIDTH * TOOLTIP_SCALE as i32,
            DISPLAY_HEIGHT * TOOLTIP_SCALE as i32,
            "terminal8x8.png",
        )
        .build()?;

    main_loop(context, State::new())
}
