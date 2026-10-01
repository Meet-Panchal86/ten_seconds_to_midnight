use bevy::{camera::Camera, color::Color, ecs::{entity::Entity, query::{With, Without}, system::{Commands, Query, Res}}, input::{ButtonInput, keyboard::KeyCode, mouse::MouseButton}, math::{Quat, Vec2}, sprite::Sprite, time::Time, transform::components::{GlobalTransform, Transform}, utils::default, window::{PrimaryWindow, Window}};
use crate::components::{Bullet, Collider, Dead, Health, Player};

const PLAYER_SPEED: f32 = 200.0;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Health { current: 100.0, max: 100.0},
        Collider { radius: 15f32 },
        Sprite {
            color: Color::srgb(0.2, 0.7, 0.9),
            custom_size: Some(Vec2::new(30.0, 30.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

pub fn player_death(
    mut player: Query<(Entity, &Health), With<Player>>, 
    mut commands: Commands
) {
    let Ok((entity, health)) = player.single_mut() else {
        return;
    };

    if health.current <= 0.0 {
        commands.entity(entity).insert(Dead);
    }
}

pub fn player_shooting(window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>, 
    player: Query<&Transform, (With<Player>, Without<Dead>)>,
    input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
) {
    if input.just_pressed(MouseButton::Left) {
        let Ok(player_transform) = player.single() else {
            return;
        };
        
        let Ok(window) = window.single() else {
            return;
        };
    
        let Some(cursor_position) = window.cursor_position() else {
            return;
        };

        let Ok((camera, camera_transform)) = camera.single() else {
            return;
        };

        let Ok(world_position) = camera.viewport_to_world_2d(camera_transform,  cursor_position) else {
            return;
        };

        let player_position = player_transform.translation;

        let direction = world_position - player_position.truncate();

        if direction == Vec2::ZERO {
            return;
        }

        let direction = direction.normalize();

        commands.spawn((
            Bullet{
                direction
            },
            Collider{ radius: 4f32 },
            Sprite {
                color: Color::srgb(1.0, 0.8, 0.1),
                custom_size: Some(Vec2::new(8f32, 8f32)),
                ..default()
            },
            Transform::from_translation(player_position + direction.extend(0.0) * 18.0)
        ));
    }
}

pub fn player_aiming(window: Query<&Window, With<PrimaryWindow>>, camera: Query<(&Camera, &GlobalTransform)>, mut player: Query<&mut Transform, (With<Player>, Without<Dead>)>) {

    let Ok(window) = window.single() else {
        return;
    };
    
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok((camera, camera_transform)) = camera.single() else {
        return;
    };

    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    let Ok(mut player_transform) = player
    .single_mut() else {
        return;
    };

    let player_position = player_transform.translation.truncate();

    let direction = world_position - player_position;

    if direction != Vec2::ZERO {
        let angle = direction.y.atan2(direction.x);

        player_transform.rotation = Quat::from_rotation_z(angle);
    }
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
