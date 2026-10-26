use bevy::{app::{Plugin, Startup, Update}, ecs::schedule::{ApplyDeferred, IntoScheduleConfigs}};

use crate::player::{animation::{animate_player, despawn_muzzle_flash, player_animation_state, spawn_aim_line}, asset_resource::load_player_animation, combat::{player_aiming, player_shooting}, movement::{player_death, player_movement, spawn_player}};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app 
            .add_systems(
                Startup, 
                (
                load_player_animation,
                spawn_player.after(load_player_animation)
            ))
            .add_systems(Update,
                 (
                spawn_aim_line,
                player_movement.after(ApplyDeferred), 
                player_aiming.after(ApplyDeferred), 
                player_shooting.after(ApplyDeferred), 
                player_death,
                animate_player, 
                player_animation_state.before(animate_player),
                despawn_muzzle_flash,
            ));
    }
}