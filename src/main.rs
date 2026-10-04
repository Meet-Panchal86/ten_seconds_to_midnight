#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::{prelude::*, window::PrimaryWindow};

mod components;
mod player;
mod zombie;
mod bullet;
mod collision;
mod resources;
mod scores;
mod game_lifecycle;
mod crosshair;
mod audio;
mod asset_resources;

use player::*;
use zombie::*;
use bullet::*;

use crate::{asset_resources::{load_player_animation, load_zombie_animation}, audio::{game_sound, load_audio_assets}, collision::{bullet_zombie_collision, zombie_player_collision, zombie_zombie_collision}, components::ArenaBorder, crosshair::{ hide_cursor, spawn_crosshair, toggle_cursor, update_aim_offset}, game_lifecycle::{highest_wave_reached, load_save_data, restart_game, start_selected_wave}, resources::{AimOffset, Kills, SaveData, Wave, WaveState}, scores::{gameover_text, setup_health_ui, setup_kill_ui, setup_wave_ui, update_health_text, update_kill_text, update_wave_text}};

const WAVE_INTERVAL: f32 = 4.0;
const BORDER_THICKNESS:f32 = 8.0;


fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}


pub fn spawn_arena_border(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>
) {
    let width;
    let  height;
    if let Ok(window) = window.single()  {
        width = window.width();
        height = window.height();
    } else {
        return;
    }
    
    // Top
    commands.spawn((
        ArenaBorder,
        Sprite {
            color: Color::srgb(0.12, 0.12, 0.15),
            custom_size: Some(Vec2::new(width, BORDER_THICKNESS)),
            ..default()
        },
        Transform::from_xyz(0.0, height / 2.0, 0.0),
    ));

    // Bottom
    commands.spawn((
        ArenaBorder,
        Sprite {
            color: Color::srgb(0.12, 0.12, 0.15),
            custom_size: Some(Vec2::new(width, BORDER_THICKNESS)),
            ..default()
        },
        Transform::from_xyz(0.0, -height / 2.0, 0.0),
    ));

    // Left
    commands.spawn((
        ArenaBorder,
        Sprite {
            color: Color::srgb(0.12, 0.12, 0.15),
            custom_size: Some(Vec2::new(BORDER_THICKNESS, height)),
            ..default()
        },
        Transform::from_xyz(-width / 2.0, 0.0, 0.0),
    ));

    // Right
    commands.spawn((
        ArenaBorder,
        Sprite {
            color: Color::srgb(0.12, 0.12, 0.15),
            custom_size: Some(Vec2::new(BORDER_THICKNESS, height)),
            ..default()
        },
        Transform::from_xyz(width / 2.0, 0.0, 0.0),
    ));
}

pub fn update_arena_border(
    window: Query<&Window, With<PrimaryWindow>>,
    mut borders: Query<(&mut Transform,&mut Sprite), With<ArenaBorder>>
) {
    let Ok(window) = window.single() else {
        return;
    };

    let height = window.height();
    let width = window.width();

    for (mut transform,mut sprite )in &mut borders {
        if let Some(vec) = sprite.custom_size {
            if vec.x == BORDER_THICKNESS {
                if transform.translation.x < 0.0 {
                    transform.translation.x = -width/2.0;
                }else {
                    transform.translation.x = width/2.0;
                }
                sprite.custom_size = Some(Vec2::new(BORDER_THICKNESS, height));
            } else {
                if transform.translation.y > 0.0 {
                    transform.translation.y = height/2.0;
                } else {
                    transform.translation.y = -height/2.0;
                }
                sprite.custom_size = Some(Vec2::new(width, BORDER_THICKNESS));
            }
        }
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.025, 0.025, 0.035)))
        .insert_resource(Wave{ number: 1, state: WaveState::NotStarted, timer: Timer::from_seconds(WAVE_INTERVAL, TimerMode::Once), })
        .insert_resource( Kills { kill: 0, })
        .insert_resource(SaveData::default())
        .insert_resource(AimOffset::default())
        .add_systems(
        Startup,(
                setup_camera,
                spawn_player.after(load_player_animation),
                load_audio_assets,
                setup_kill_ui,
                setup_wave_ui,
                setup_health_ui,
                load_save_data,
                spawn_aim_line.after(spawn_player),
                spawn_crosshair.after(spawn_player),
                hide_cursor,
                spawn_arena_border,
                game_sound.after(load_audio_assets),
                load_zombie_animation,
                load_player_animation
        ))
        .add_systems(
            Update, (
                restart_game,
                bevy::ecs::schedule::ApplyDeferred.after(restart_game),
                player_movement.after(ApplyDeferred), 
                player_aiming.after(ApplyDeferred), 
                player_shooting.after(ApplyDeferred), 
                player_death,
                animate_player, 
                player_animation_state
        ))
        .add_systems(Update, (
                bullet_movement,
                (
                    zombie_movement.after(player_movement),
                    zombie_player_collision,
                    zombie_zombie_collision,
                    zombie_animate_state,
                    animate_zombie,
                ).chain(),
                    bullet_zombie_collision
                    .after(bullet_movement)
                    .after(zombie_zombie_collision),
                spawn_zombie_death.after(bullet_zombie_collision),
                animate_zombie_death,
                wave_manager,
                update_kill_text,
                update_wave_text,
                update_health_text.after(animate_zombie),
                gameover_text,
                despawn_muzzle_flash,
        ))
        .add_systems(
            Update, 
            (  
                highest_wave_reached.after(start_selected_wave),
                start_selected_wave.after(restart_game),
                update_aim_offset,
                toggle_cursor,
                update_arena_border,
        ))
        .run();
}


