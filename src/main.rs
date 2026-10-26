#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::prelude::*;

mod components;
mod player;
mod combat;
mod resources;
mod ui;
mod game;
mod audio;
mod zombie;
mod arena;

use crate::{arena::plugin::ArenaPlugin, audio::plugin::AudioPlugin, combat::plugin::CombatPlugin, game::{config::WAVE_INTERVAL, plugin::GamePlugin}, player::plugin::PlayerPlugin, resources::{AimOffset, Kills, Wave, WaveState}, ui::plugin::UiPlugin, zombie::plugin::ZombiePlugin};

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ArenaPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ZombiePlugin)
        .add_plugins(CombatPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(AudioPlugin)
        .insert_resource(
            Wave{   
                number: 1, 
                state: WaveState::NotStarted, 
                timer: Timer::from_seconds(WAVE_INTERVAL, TimerMode::Once), 
            }
        )
        .insert_resource( Kills { kill: 0, })
        .insert_resource(AimOffset::default())
        .add_systems(
        Startup,
        (
                setup_camera,
        ))
        .run();
}
