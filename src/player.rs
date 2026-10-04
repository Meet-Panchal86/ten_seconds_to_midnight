use bevy::{audio::{PlaybackMode, Volume}, ecs::{entity::Entity, query::{With, Without}, system::{Commands, Query, Res}}, input::{ButtonInput, keyboard::KeyCode, mouse::MouseButton}, math::{Quat, Vec2}, prelude::*, sprite::Sprite, time::{Time, Timer}, transform::components::Transform, utils::default, window::{PrimaryWindow, Window}};
use crate::{asset_resources::{AudioAssets, PlayerAnimationAssets}, components::{AimLine, Bullet, Collider, Dead, Health, MuzzleFlash, Player, PlayerAnimation, PlayerAnimationState, ShootCooldown}, resources::AimOffset};

pub const PLAYER_SPEED: f32 = 200.0;
pub const MUZZLE_OFFSET: f32 = 38f32;
pub const MUZZLE_SIDE: f32 = -15.6f32;
const AIMLINE_LENGTH: f32 = 5f32;
const AIMLINE_WIDTH:f32 = 1.1f32;

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

pub fn spawn_aim_line(player: Query<Entity, With<Player>>, mut commands: Commands) {

    let Ok(entity) = player.single() else {
        return;
    };

    let aim_line1 = commands.spawn((
        AimLine,
        Sprite {
            color: Color::linear_rgb(1.0, 0.0, 0.2),
            custom_size: Some(Vec2::new(AIMLINE_LENGTH, AIMLINE_WIDTH)),
            ..default()
        },
        Transform::from_xyz( MUZZLE_OFFSET + 5.0, MUZZLE_SIDE, 0.0)
    )).id();

    let aim_line2 = commands.spawn((
        AimLine,
        Sprite {
            color: Color::linear_rgb(1.0, 0.0, 0.2),
            custom_size: Some(Vec2::new(AIMLINE_LENGTH, AIMLINE_WIDTH)),
            ..default()
        },
        Transform::from_xyz( MUZZLE_OFFSET + 15.0, MUZZLE_SIDE, 0.0)
    )).id();

    commands.entity(entity).add_child(aim_line1);
    commands.entity(entity).add_child(aim_line2);
}

pub fn player_animation_state(
    key_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut PlayerAnimation, With<Player>>,
) {
    let movement_keys = [
        KeyCode::KeyW,
        KeyCode::KeyA,
        KeyCode::KeyS,
        KeyCode::KeyD,
    ];

    let is_moving = key_input.any_pressed(movement_keys);

    for mut animation in &mut player {
        if animation.state == PlayerAnimationState::Shoot {
            continue;
        }

        if is_moving {
            animation.state = PlayerAnimationState::Move;
        } else {
            animation.state = PlayerAnimationState::Idle;
        }
    }
}

pub fn animate_player(
    time: Res<Time>,
    mut player: Query<
        (&mut Sprite, &mut PlayerAnimation),
        (With<Player>, Without<Dead>),
    >,
) {
    for (mut sprite, mut animation) in &mut player {

        if animation.state != animation.previous_state {
            animation.current_frame = 0;
            animation.timer.reset();
            animation.previous_state = animation.state;
        }

        let frame_count = match animation.state {
            PlayerAnimationState::Idle => animation.idle_frames.len(),
            PlayerAnimationState::Move => animation.move_frames.len(),
            PlayerAnimationState::Shoot => animation.shoot_frames.len(),
        };

        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {

            if animation.state == PlayerAnimationState::Shoot
                && animation.current_frame == frame_count - 1 {
                animation.state = PlayerAnimationState::Idle;
                animation.current_frame = 0;
                animation.timer.reset();
                continue;
            }

            animation.current_frame =
                (animation.current_frame + 1) % frame_count;

            let image = match animation.state {
                PlayerAnimationState::Idle => {
                    animation.idle_frames[animation.current_frame].clone()
                },
                PlayerAnimationState::Move => {
                    animation.move_frames[animation.current_frame].clone()
                },
                PlayerAnimationState::Shoot => {
                    animation.shoot_frames[animation.current_frame].clone()
                }
            };
            
            sprite.image = image;
        }
    }
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

pub fn player_shooting(
//    window: Query<&Window, With<PrimaryWindow>>,
//    camera: Query<(&Camera, &GlobalTransform)>, 
    mut player: Query<(Entity, &Transform, &mut ShootCooldown, &mut PlayerAnimation), (With<Player>, Without<Dead>)>,
    input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    aim: Res<AimOffset>,
    audio_assets: Res<AudioAssets>,
) {
    let Ok((player_entity, player_transform, mut shoot_cooldown, mut animation)) = player.single_mut() else {
        return;
    };
    shoot_cooldown.timer.tick(time.delta());

    if input.pressed(MouseButton::Left) && shoot_cooldown.timer.is_finished() {
        let player_position = player_transform.translation;

        let direction = aim.0.normalize();

        if direction == Vec2::ZERO {
            return;
        }

        let angle = direction.y.atan2(direction.x);
        let perpendicular = Vec2::new(-direction.y, direction.x);

        let final_translation = player_position + direction.extend(0.0) * MUZZLE_OFFSET + perpendicular.extend(0.0) * MUZZLE_SIDE;
  
        commands.spawn((
            Bullet{
                direction,
                previous_position: final_translation.truncate()
            },
            Collider{ radius: 6f32 },
            Sprite {
                image: asset_server.load("bullet/bullet.png"),
                custom_size: Some(Vec2::new(7f32, 7f32)),
                ..default()
            },
            Transform {
                translation: final_translation,
                rotation: Quat::from_rotation_z(angle),
                ..default()
            }
        ));
        commands.spawn((
            AudioPlayer::new(audio_assets.gunshot.clone()),
            PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: Volume::Linear(0.85),
            //  speed: 2.4,
            ..default()
            }
        ));
        let flash = commands.spawn((
            MuzzleFlash{ timer: Timer::from_seconds(0.13, TimerMode::Once) },
            Sprite {
                image: asset_server.load("bullet/muzzle_flash_3.png"),
                custom_size: Some(Vec2::new(34.0, 34.0)),
                ..default()
            },
            Transform::from_xyz(MUZZLE_OFFSET + 1.0, MUZZLE_SIDE, 0.1),
        )).id();
        commands.entity(player_entity).add_child(flash);
        animation.state = PlayerAnimationState::Shoot;
        
        shoot_cooldown.timer.reset();
    }
}

// pub fn despawn_aim_lines(
//     mut commands: Commands,
//     player: Query<Entity,(With<Player>, With<Dead>)> ,
//     aim_lines: Query<Entity, With<AimLine>>
// ){
//     if let Ok(_) = player.single() {
//         for entity in aim_lines {
//             commands.entity(entity).despawn();
//         }
//     }
// }

// pub fn player_shooting(
//     aim: Res<AimOffset>,
//     mut player: Query<(Entity, &Transform, &mut ShootCooldown, &mut PlayerAnimation), (With<Player>, Without<Dead>)>,
//     input: Res<ButtonInput<MouseButton>>,
//     mut commands: Commands,
//     time: Res<Time>,
//     asset_server: Res<AssetServer>,
// ) {
//     let Ok((player_entity, player_transform, mut shoot_cooldown, mut animation)) = player.single_mut() else {
//         return;
//     };
//     shoot_cooldown.timer.tick(time.delta());

//     if input.pressed(MouseButton::Left) && shoot_cooldown.timer.is_finished() {
//         let direction = aim.0.normalize_or_zero();
//         if direction == Vec2::ZERO {
//             return;
//         }

//         let player_position = player_transform.translation;
//         let angle = direction.y.atan2(direction.x);
//         let perpendicular = Vec2::new(-direction.y, direction.x);
//         let final_translation = player_position
//             + direction.extend(0.0) * MUZZLE_OFFSET
//             + perpendicular.extend(0.0) * MUZZLE_SIDE;
            
//         commands.spawn((
//             Bullet{
//                 direction
//             },
//             Collider{ radius: 4f32 },
//             Sprite {
//                 image: asset_server.load("bullet/bullet.png"),
//                 custom_size: Some(Vec2::new(7f32, 7f32)),
//                 ..default()
//             },
//             Transform {
//                 translation: final_translation,
//                 rotation: Quat::from_rotation_z(angle),
//                 ..default()
//             }
//         ));
//         let flash = commands.spawn((
//             MuzzleFlash{ timer: Timer::from_seconds(0.13, TimerMode::Once) },
//             Sprite {
//                 image: asset_server.load("bullet/muzzle_flash_3.png"),
//                 custom_size: Some(Vec2::new(34.0, 34.0)),
//                 ..default()
//             },
//             Transform::from_xyz(MUZZLE_OFFSET + 1.0, MUZZLE_SIDE, 0.1),
//         )).id();
//         commands.entity(player_entity).add_child(flash);
//         animation.state = PlayerAnimationState::Shoot;
        
//         shoot_cooldown.timer.reset();
//     }
// }

pub fn despawn_muzzle_flash(mut muzzle_flashes: Query<(Entity, &mut MuzzleFlash), With<MuzzleFlash>>,mut commands: Commands, time: Res<Time>) {
    for (entity,mut flash) in &mut muzzle_flashes {
        flash.timer.tick(time.delta());
        if flash.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

// pub fn player_aiming(window: Query<&Window, With<PrimaryWindow>>, camera: Query<(&Camera, &GlobalTransform)>, mut player: Query<&mut Transform, (With<Player>, Without<Dead>)>) {

//     let Ok(window) = window.single() else {
//         return;
//     };
    
//     let Some(cursor_position) = window.cursor_position() else {
//         return;
//     };

//     let Ok((camera, camera_transform)) = camera.single() else {
//         return;
//     };

//     let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
//         return;
//     };

//     let Ok(mut player_transform) = player
//     .single_mut() else {
//         return;
//     };

//     let player_position = player_transform.translation.truncate();

//     let direction = world_position - player_position;

//     if direction != Vec2::ZERO {
//         let direction = direction.normalize();
//         let angle = direction.y.atan2(direction.x);

//         player_transform.rotation = Quat::from_rotation_z(angle);
//     }
// }

pub fn player_aiming(
    aim: Res<AimOffset>,
    mut player: Query<&mut Transform, (With<Player>, Without<Dead>)>,
) {
    let Ok(mut player_transform) = player.single_mut() else { return };
    let angle = aim.0.y.atan2(aim.0.x);
    player_transform.rotation = Quat::from_rotation_z(angle);
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
