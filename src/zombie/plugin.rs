use bevy::prelude::*;

use crate::zombie::{animation::{animate_zombie, animate_zombie_death, zombie_animate_state}, asset_resource::load_zombie_animation, movement::zombie_movement, spawning::{spawn_zombie_death, wave_manager}};

pub struct ZombiePlugin;

impl Plugin for ZombiePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            load_zombie_animation,
        )
        .add_systems(
            Update,
            (
                zombie_movement,
                zombie_animate_state.before(animate_zombie),
                animate_zombie,
                spawn_zombie_death,
                animate_zombie_death.after(spawn_zombie_death),
                wave_manager
            ),
        );
    }
}