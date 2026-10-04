use bevy::{ecs::{entity::Entity, query::With, system::{Commands, Query, Res}}, text::TextFont, ui::{AlignItems, JustifyContent, Node, Val::{self, Px}, widget::Text}, utils::default};

use crate::{components::{GameOverText, Health, HealthText, KillText, Player, WaveText}, resources::{Kills, Wave}};

pub fn setup_kill_ui(mut commands: Commands) {
    commands.spawn((
        KillText,
        Text::new("KILLS: 0"),
        TextFont {
            font_size: bevy::text::FontSize::Px(20.0),
            ..default()
        },
        Node {
            position_type: bevy::ui::PositionType::Absolute,
            left: bevy::ui::Val::Px(15.0),
            top: Px(15.0),
            ..default()
        }
    ));
}

pub fn update_kill_text(
    kills: Res<Kills>, 
    mut kill_text: Query<&mut Text, With<KillText>>
) {
    let Ok(mut text) = kill_text.single_mut() else {
        return;
    };
   
    text.0 = format!("KILLS: {}", kills.kill);
}

pub fn setup_wave_ui(mut commands: Commands) {
    commands.spawn((
        WaveText,
        Text::new("WAVE 1"),
        TextFont{
            font_size: bevy::text::FontSize::Px(20.0),
            ..default()
        },
        Node {
            position_type: bevy::ui::PositionType::Absolute,
            top: Px(45.0), 
            left: Px(15.0),
            ..default()
        },
    ));
}

pub fn update_wave_text(
    wave: Res<Wave>,
    mut wave_text: Query<&mut Text, With<WaveText>>,
) {
    let Ok(mut text) = wave_text.single_mut() else {
        return;
    };

    text.0 = format!("WAVE: {}", wave.number); 
}

pub fn setup_health_ui(mut commands: Commands) {
    commands.spawn((
        HealthText,
        Text::new("HEALTH: 100 / 100"),
        TextFont{
            font_size: bevy::text::FontSize::Px(20.0),
            ..default()
        },
        Node {
            position_type: bevy::ui::PositionType::Absolute,
            top: Px(75.0), 
            left: Px(15.0),
            ..default()
        },
    ));
}

pub fn update_health_text(
    player_health: Query<&Health, With<Player>>,
    mut health_text: Query<&mut Text, With<HealthText>>
) {
    let Ok(player_health) = player_health.single() else {
        return;
    };

    let  Ok(mut text) = health_text.single_mut() else {
        return;
    }; 

    text.0 = format!("HEALTH: {} / {}", player_health.current, player_health.max);
}

pub fn gameover_text(
    mut commands: Commands,
    player: Query<&Health, With<Player>>,
    gameover: Query<Entity, With<GameOverText>>
) {
    let Ok(health) = player.single() else {
        return;
    };

    if health.current <= 0.0 {
        if gameover.is_empty() {
            commands
            .spawn((
                GameOverText,
                Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            }))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("GAME OVER"),
                    TextFont {
                        font_size: bevy::text::FontSize::Px(50.0),
                        ..default()
                    },
                ));
            });
        } 
    } else if !gameover.is_empty() {
        if let Ok(entity) = gameover.single() {
            commands.entity(entity).despawn();
        } 
    }
}