use bevy::{ecs::{entity::Entity, query::{With, Without}, system::{Commands, Query, Res}}, input::{ButtonInput, keyboard::KeyCode}, math::Vec2, sprite::Sprite, time::{Time, Timer, TimerMode}, transform::components::Transform, utils::default, window::{PrimaryWindow, Window}};

use crate::{components::{Collider, Dead, Health}, player::{asset_resource::PlayerAnimationAssets, components::{Player, PlayerAnimation, PlayerAnimationState, ShootCooldown}, config::PLAYER_SPEED}};

pub fn spawn_player(mut commands: Commands, player_assets: Res<PlayerAnimationAssets>) {   
    commands.spawn((
        Player,
        Health { current: 100.0, max: 100.0},
        Collider { radius: 15f32 },
        ShootCooldown{ timer: Timer::from_seconds(0.15, bevy::time::TimerMode::Once)},
        Sprite {
            image: player_assets.idle.get(0).expect("Failed to load first idle frame for player sprite!").clone(),
            custom_size: Some(Vec2::new(65.0, 65.0)),
            ..default()
        },
        PlayerAnimation{
            idle_frames: player_assets.idle.clone(),
            move_frames: player_assets.movement.clone(),
            shoot_frames: player_assets.shoot.clone(),
            current_frame: 0,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            state: PlayerAnimationState::Idle,
            previous_state: PlayerAnimationState::Idle,
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

pub fn player_movement(window: Query<&Window, With<PrimaryWindow>>,input: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut query: Query<&mut Transform, (With<Player>, Without<Dead>)>) {
    let speed = PLAYER_SPEED;

    let Ok(mut player_transform) = query.single_mut() else {
        return;
    };

    let Ok(window) = window.single() else {
        return;
    };

    let max_x = window.width()/2.0 - 15f32;
    let max_y = window.height()/2.0 - 15f32;

    let mut direction = Vec2::ZERO;

    if input.pressed(KeyCode::KeyW) {      
       direction.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) {
       direction.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) {
       direction.x -= 1.0;
    }   
    if input.pressed(KeyCode::KeyD) {
       direction.x += 1.0;
    }
    
    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }
    
    let mut transform = player_transform.translation + direction.extend(0.0) * speed * time.delta_secs();

    let new_x = transform.x.clamp(-max_x, max_x);
    let new_y = transform.y.clamp(-max_y, max_y);

    transform.x = new_x;
    transform.y = new_y;

    player_transform.translation = transform;
}


pub fn player_death(
    mut player: Query<(Entity, &Health), With<Player>>, 
    mut commands: Commands,
) {
    let Ok((entity, health)) = player.single_mut() else {
        return;
    };

    if health.current <= 0.0 {
        commands.entity(entity).insert(Dead);
    }
}