use bevy::{ecs::{entity::Entity, query::With, system::{Commands, Query}}, text::TextFont, ui::{AlignItems, JustifyContent, Node, Val, widget::Text}, utils::default};

use crate::{components::Health, player::components::Player, ui::components::GameOverText};

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