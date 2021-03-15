use crate::prelude::*;

mod ai;
mod combat;
mod digging;
mod eco;
mod fov;
mod rl;
mod targeting;
mod use_items;

// ROGUELIKE_MODE

pub fn build_rl_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(rl::input::input_system())
        .add_system(fov::fov_system())
        .flush()
        .add_system(targeting::targetting_system())
        .flush()
        .add_system(rl::map_render::map_render_system())
        .add_system(rl::entity_render::entity_render_system())
        .add_system(rl::hud::hud_system())
        .add_system(rl::tooltips::tooltips_system())
        .add_system(rl::end_input::end_input_system())
        .build()
}

pub fn build_rl_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(use_items::use_items_system())
        .add_system(combat::combat_system())
        .flush()
        .add_system(rl::movement::movement_system())
        .flush()
        .add_system(fov::fov_system())
        .flush()
        .add_system(rl::map_render::map_render_system())
        .add_system(rl::entity_render::entity_render_system())
        .add_system(rl::hud::hud_system())
        .add_system(rl::end_turn::end_turn_system())
        .build()
}

pub fn build_rl_creature_and_plant_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(ai::random_move::random_move_system())
        .add_system(ai::ranged::ranged_system())
        .add_system(ai::chasing::chasing_system())
        .add_system(ai::foraging::foraging_system())
        .flush()
        .add_system(ai::spawning_fruit::spawning_fruit_system())
        .add_system(ai::spawning_forager::spawning_forager_system())
        .flush()
        .add_system(use_items::use_items_system())
        .add_system(combat::combat_system())
        .flush()
        .add_system(rl::movement::movement_system())
        .flush()
        .add_system(fov::fov_system())
        .flush()
        .add_system(rl::map_render::map_render_system())
        .add_system(rl::entity_render::entity_render_system())
        .add_system(rl::hud::hud_system())
        .add_system(rl::end_turn::end_turn_system())
        .build()
}

// ECOSYSTEM_MODE

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(eco::input::input_system())
        .flush()
        .add_system(eco::state_switch::state_switch_system())
        .build()
}

pub fn build_logic_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(ai::random_move::random_move_system())
        .add_system(ai::random_patrolling::random_patrolling_system())
        .add_system(ai::foraging::foraging_system())
        .flush()
        .add_system(ai::spawning_fruit::spawning_fruit_system())
        .add_system(ai::spawning_forager::spawning_forager_system())
        .flush()
        .flush()
        .add_system(use_items::use_items_system())
        .add_system(combat::combat_system())
        .flush()
        .add_system(eco::movement::movement_system())
        .add_system(digging::digging_system())
        .flush()
        .add_system(fov::fov_system())
        .flush()
        .build()
}
pub fn build_render_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(eco::camera_movement::camera_movement_system())
        .flush()
        .add_system(eco::map_render::map_render_system())
        .add_system(eco::entity_render::entity_render_system())
        .add_system(eco::tooltips::tooltips_system())
        .build()
}
