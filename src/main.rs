use bevy::prelude::*;

mod components;
mod player;
mod zombie;
mod bullet;
mod collision;
mod resources;

use player::*;
use zombie::*;
use bullet::*;

use crate::{collision::{bullet_zombie_collision, zombie_player_collision, zombie_zombie_collision}, resources::{Wave, WaveState}};

const WAVE_INTERVAL: f32 = 4.0;


fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Wave{ number: 0, state: WaveState::Fighting, timer: Timer::from_seconds(WAVE_INTERVAL, TimerMode::Once), })
        .add_systems(Startup, (setup_camera, spawn_player))
        .add_systems(Update, (player_movement, player_aiming, player_shooting, player_death))
        .add_systems(Update, (bullet_movement, zombie_movement, zombie_player_collision, zombie_zombie_collision, bullet_zombie_collision, wave_manager))
        .run();
}


