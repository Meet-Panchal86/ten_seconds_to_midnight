use bevy::{asset::AssetServer, ecs::{entity::Entity, query::With, system::{Commands, Query, Res, ResMut, Single}}, input::{ButtonInput, keyboard::KeyCode, mouse::AccumulatedMouseMotion}, math::Vec2, sprite::Sprite, transform::components::Transform, utils::default, window::{CursorGrabMode, CursorOptions, PrimaryWindow, Window, WindowMode}};

use crate::{components::{Crosshair, Player}, player::MUZZLE_SIDE, resources::{AIM_RADIUS, AimOffset, MOUSE_SENSITIVITY}};

pub fn hide_cursor(mut cursor_options: Single<&mut CursorOptions>, mut window: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = window.single_mut() {
        window.mode = WindowMode::BorderlessFullscreen(bevy::window::MonitorSelection::Primary);
    }
    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Locked;
}

pub fn toggle_cursor(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut cursor_options: Single<&mut CursorOptions>,
    mut window: Query<&mut Window, With<PrimaryWindow>>
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if let Ok(mut window) = window.single_mut() {
            if window.mode == WindowMode::BorderlessFullscreen(bevy::window::MonitorSelection::Primary) {
                window.mode = WindowMode::Windowed;
            } else {
                window.mode = WindowMode::BorderlessFullscreen(bevy::window::MonitorSelection::Primary);
            }
        }

        cursor_options.visible = !cursor_options.visible;
        if cursor_options.grab_mode == CursorGrabMode::Locked {
            cursor_options.grab_mode = CursorGrabMode::None;
        } else {
            cursor_options.grab_mode = CursorGrabMode::Locked;
        }
    }
}

pub fn spawn_crosshair(mut commands: Commands, player: Query<Entity, With<Player>>, asset_server: Res<AssetServer>) {

    let Ok(entity) = player.single() else {
        return;
    };

    let crosshair = commands.spawn((
        Crosshair,
        Sprite {
            image: asset_server.load("crosshair/crosshair.png"),
            custom_size: Some(Vec2::new(12.0, 12.0)),
            ..default()
        },
        Transform::from_xyz(AIM_RADIUS, MUZZLE_SIDE, 1.0),
    )).id();

    commands.entity(entity).add_child(crosshair);
}

// pub fn despawn_crosshair(
//     mut commands: Commands,
//     player: Query<Entity, (With<Player>, With<Dead>)>,
//     crosshair: Query<Entity, With<Crosshair>>
// ) {
//     if let Ok(_) = player.single() {
//         let Ok(entity) = crosshair.single() else {
//             return;
//         } ;
//         commands.entity(entity).despawn();
//     }
// }

pub fn update_aim_offset(
    motion: Res<AccumulatedMouseMotion>,
    mut aim: ResMut<AimOffset>,
) {
    let delta = motion.delta;
    if delta == Vec2::ZERO {
        return;
    }

    // Screen Y is down, world Y is up, so flip it
    aim.0 += Vec2::new(delta.x, -delta.y) * MOUSE_SENSITIVITY;

    // Keep the crosshair on a fixed-radius ring around the player
    aim.0 = aim.0.normalize_or_zero() * AIM_RADIUS;
    if aim.0 == Vec2::ZERO {
        aim.0 = Vec2::new(AIM_RADIUS, 0.0);
    }
}

// pub fn update_crosshair(
//     aim: Res<AimOffset>,
//     player: Query<&Transform, (With<Player>, Without<Crosshair>)>,
//     mut crosshair: Query<&mut Transform, With<Crosshair>>,
// ) {
//     let Ok(player_tf) = player.single() else { return };
//     let Ok(mut tf) = crosshair.single_mut() else { return };
//     tf.translation = (player_tf.translation.truncate() + aim.0).extend(10.0);
// }