use bevy::{color::Color, ecs::{query::With, system::{Commands, Query}}, math::Vec2, sprite::Sprite, transform::components::Transform, utils::default, window::{PrimaryWindow, Window}};

use crate::arena::{components::ArenaBorder, config::BORDER_THICKNESS};

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
