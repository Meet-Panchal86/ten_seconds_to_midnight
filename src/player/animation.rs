use bevy::{color::Color, ecs::{entity::Entity, query::{With, Without}, system::{Commands, Query, Res}}, input::{ButtonInput, keyboard::KeyCode}, math::Vec2, sprite::Sprite, time::Time, transform::components::Transform, utils::default};

use crate::{components::Dead, player::{components::{AimLine, MuzzleFlash, Player, PlayerAnimation, PlayerAnimationState}, config::{AIMLINE_LENGTH, AIMLINE_WIDTH, MUZZLE_OFFSET, MUZZLE_SIDE}}};

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

pub fn despawn_muzzle_flash(mut muzzle_flashes: Query<(Entity, &mut MuzzleFlash), With<MuzzleFlash>>,mut commands: Commands, time: Res<Time>) {
    for (entity,mut flash) in &mut muzzle_flashes {
        flash.timer.tick(time.delta());
        if flash.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
