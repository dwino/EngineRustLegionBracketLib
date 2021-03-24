#![warn(clippy::pedantic)]
use std::collections::HashSet;

mod camera;
mod components;
mod dkm;
mod eco_camera;
mod eco_state;
mod forage_map;
mod game_mode;
mod map;
mod map_builder;
mod rl_state;
mod spawner;
mod systems;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = 80;
    pub const DISPLAY_HEIGHT: i32 = 50;
    pub const TILE_DIMENSIONS_MAP: i32 = 13;
    pub const TILE_DIMENSIONS_TOOLTIP: i32 = TILE_DIMENSIONS_MAP / 2;
    pub const TOOLTIP_SCALE: i32 = TILE_DIMENSIONS_MAP / TILE_DIMENSIONS_TOOLTIP;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::dkm::*;
    pub use crate::eco_camera::*;
    pub use crate::eco_state::*;
    pub use crate::forage_map::*;
    pub use crate::game_mode::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::rl_state::*;
    pub use crate::spawner::FruitType;
    pub use crate::spawner::*;
    pub use crate::systems::*;
}

use prelude::*;

struct State {
    ecs: World,
    mode: GameMode,
    resources: Resources,
    rl_input_systems: Schedule,
    rl_player_systems: Schedule,
    rl_creature_and_plant_systems: Schedule,
    eco_input_system: Schedule,
    eco_logic_systems: Schedule,
    eco_render_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let ecs = World::default();
        let resources = Resources::default();
        Self {
            ecs,
            resources,
            mode: GameMode::Menu,
            rl_input_systems: build_rl_input_scheduler(),
            rl_player_systems: build_rl_player_scheduler(),
            rl_creature_and_plant_systems: build_rl_creature_and_plant_scheduler(),
            eco_input_system: build_input_scheduler(),
            eco_logic_systems: build_logic_scheduler(),
            eco_render_systems: build_render_scheduler(),
        }
    }

    //MODE:MENU
    ///////////
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.print_centered(5, "RooTed");
        ctx.print_centered(8, "(R) Roguelike Mode");
        ctx.print_centered(9, "(E) Ecosystem Mode");
        ctx.print_centered(10, "(Q) Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::R => self.roguelike_mode(),
                VirtualKeyCode::E => self.ecosystem_mode(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn roguelike_mode(&mut self) {
        self.init_rl_game_state();
        self.mode = GameMode::RogueLike;
    }
    fn ecosystem_mode(&mut self) {
        self.init_eco_game_state();
        self.mode = GameMode::Ecosystem;
    }

    //MODE:ROGUELIKE
    ////////////////
    fn init_rl_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut self.ecs, map_builder.player_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        spawn_level(&mut self.ecs, &mut rng, 0, &map_builder.monster_spawns);
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(RlState::AwaitingInput);
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
        let current_state = *self.resources.get::<RlState>().unwrap();
        match current_state {
            RlState::AwaitingInput => self
                .rl_input_systems
                .execute(&mut self.ecs, &mut self.resources),
            RlState::PlayerTurn => {
                self.rl_player_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            RlState::MonsterTurn => self
                .rl_creature_and_plant_systems
                .execute(&mut self.ecs, &mut self.resources),
            RlState::GameOver => {
                self.rl_game_over(ctx);
            }
            RlState::Victory => {
                self.rl_victory(ctx);
            }
            RlState::NextLevel => {
                self.rl_advance_level();
            }
        }
        render_draw_buffer(ctx).expect("Render error");
    }

    fn rl_advance_level(&mut self) {
        let player_entity = *<Entity>::query()
            .filter(component::<Player>())
            .iter(&self.ecs)
            .next()
            .unwrap();

        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);
        <(Entity, &Carried)>::query()
            .iter(&self.ecs)
            .filter(|(_e, carry)| carry.0 == player_entity)
            .map(|(e, _carry)| *e)
            .for_each(|e| {
                entities_to_keep.insert(e);
            });
        <(Entity, &Equiped)>::query()
            .iter(&self.ecs)
            .filter(|(_e, carry)| carry.0 == player_entity)
            .map(|(e, _carry)| *e)
            .for_each(|e| {
                entities_to_keep.insert(e);
            });

        let mut cb = CommandBuffer::new(&self.ecs);
        for e in Entity::query().iter(&self.ecs) {
            if !entities_to_keep.contains(e) {
                cb.remove(*e);
            }
        }
        cb.flush(&mut self.ecs);

        <&mut FieldOfView>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|fov| {
                fov.is_dirty = true;
                fov.sensing = false;
            });

        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                pos.x = map_builder.player_start.x;
                pos.y = map_builder.player_start.y;
            });
        if map_level == 2 {
            spawn_magic_droplet(&mut self.ecs, map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }
        spawn_level(
            &mut self.ecs,
            &mut rng,
            map_level as usize,
            &map_builder.monster_spawns,
        );
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(RlState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }

    fn rl_game_over(&mut self, ctx: &mut BTerm) {
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
            self.init_rl_game_state();
        }
    }

    fn rl_victory(&mut self, ctx: &mut BTerm) {
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
            self.init_rl_game_state();
        }
    }

    //MODE:ECOSYSTEM
    ////////////////
    fn init_eco_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_foraging_level(
            &mut self.ecs,
            &map_builder.map,
            &map_builder.map.forage_map.nest_positions,
            &map_builder.map.forage_map.forage_positions,
        );
        self.resources.insert(map_builder.map);
        self.resources.insert(EcoCamera::new(Point::new(
            SCREEN_WIDTH / 2,
            SCREEN_HEIGHT / 2,
        )));
        self.resources.insert(EcoState::Play);
        self.resources.insert(map_builder.theme);
    }

    fn execute_eco_game_state(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        let current_state = *self.resources.get::<EcoState>().unwrap();

        self.eco_input_system
            .execute(&mut self.ecs, &mut self.resources);

        match current_state {
            EcoState::Play => {
                self.eco_logic_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            EcoState::Pause => {}
        }
        self.eco_render_systems
            .execute(&mut self.ecs, &mut self.resources);

        render_draw_buffer(ctx).expect("Render error");
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::RogueLike => self.execute_rl_game_state(ctx),
            GameMode::Ecosystem => self.execute_eco_game_state(ctx),
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
