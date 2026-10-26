use bevy::{asset::AssetServer, audio::{AudioPlayer, PlaybackMode, PlaybackSettings}, ecs::{entity::Entity, query::{With, Without}, system::{Commands, Query, Res}}, input::{ButtonInput, mouse::MouseButton}, math::{Quat, Vec2}, sprite::Sprite, time::{Time, Timer, TimerMode}, transform::components::Transform, utils::default};

use crate::{ audio::audio::AudioAssets, combat::components::Bullet, components::{Collider, Dead}, player::{components::{MuzzleFlash, Player, PlayerAnimation, PlayerAnimationState, ShootCooldown}, config::{MUZZLE_OFFSET, MUZZLE_SIDE}}, resources::AimOffset};

pub fn player_shooting(
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
           // volume: Volume::Linear(0.85),
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

pub fn player_aiming(
    aim: Res<AimOffset>,
    mut player: Query<&mut Transform, (With<Player>, Without<Dead>)>,
) {
    let Ok(mut player_transform) = player.single_mut() else { return };
    let angle = aim.0.y.atan2(aim.0.x);
    player_transform.rotation = Quat::from_rotation_z(angle);
}
