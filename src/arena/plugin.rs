use bevy::app::{Plugin, Startup, Update};

use crate::arena::border::{spawn_arena_border, update_arena_border};

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .add_systems(Startup, spawn_arena_border)
            .add_systems(Update, update_arena_border);
    }
}