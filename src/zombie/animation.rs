use bevy::{audio::{AudioPlayer, PlaybackMode, PlaybackSettings, Volume}, ecs::{entity::Entity, query::{With, Without}, system::{Commands, Query, Res}}, sprite::Sprite, time::Time, utils::default};

use crate::{audio::audio::AudioAssets, components::{Dead, Health}, player::components::Player, zombie::components::{DeathEffect, Zombie, ZombieAnimation, ZombieAnimationState}};

pub fn animate_zombie_death(
    mut commands: Commands,
    mut death_effects: Query<(Entity, &mut DeathEffect, &mut Sprite), With<DeathEffect>>,
    time: Res<Time>
) {
    for (entity, mut effect, mut sprite) in &mut death_effects {
        effect.timer.tick(time.delta());
    
        
        if effect.timer.is_finished() {
            effect.current_frame += 1;

            if effect.current_frame == effect.frames.len() {
                commands.entity(entity).despawn();
                continue;
            }
            sprite.image = effect.frames[effect.current_frame].clone();
            effect.timer.reset(); 
        }
    }
}


pub fn zombie_animate_state(
    player: Query<Entity, (With<Player>, With<Dead>)>,
    mut zombie: Query<&mut ZombieAnimation, (With<Zombie>, Without<Player>)>
) {
    if let Ok(_) = player.single() {
        for mut animation in &mut zombie {
            animation.state = ZombieAnimationState::Idle;
        }
    }
}

pub fn animate_zombie(
    time: Res<Time>,
    mut zomies: Query<(&mut Sprite, &mut ZombieAnimation), (With<Zombie>, Without<Player>)>,
    audio_assets: Res<AudioAssets>,
    mut commands: Commands,
    mut player: Query<&mut Health, With<Player>>
) {
    let Ok(mut health) = player.single_mut() else {
        return;
    };

    for (mut sprite, mut animation) in &mut zomies {
        
        if animation.state != animation.previous_state {
            animation.current_frame = 0;
            animation.timer.reset();
            animation.previous_state = animation.state;
        }

        let frame_count = match animation.state {
            ZombieAnimationState::Move => animation.move_frames.len(),
            ZombieAnimationState::Attack => animation.attack_frames.len(),
            ZombieAnimationState::Idle => animation.idle_frames.len(),
        };

        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            if animation.state == ZombieAnimationState::Attack && animation.current_frame == frame_count - 1 {
                animation.state = ZombieAnimationState::Move;
                animation.current_frame = 0;
                animation.timer.reset();
                continue;
            }

            animation.current_frame =
                (animation.current_frame + 1) % frame_count;

            if animation.state == ZombieAnimationState::Attack && animation.current_frame == 5 {
                commands.spawn((
                    AudioPlayer::new(audio_assets.zombie_attack.clone()),
                    PlaybackSettings {
                        mode:  PlaybackMode::Despawn,
                        volume: Volume::Linear(0.85),
                        ..default()
                    }
                ));
                
                health.current = (health.current - 10.0).max(0.0);
            }    

            let image = match animation.state {
                ZombieAnimationState::Move => animation.move_frames[animation.current_frame].clone(),
                ZombieAnimationState::Attack => animation.attack_frames[animation.current_frame].clone(),
                ZombieAnimationState::Idle => animation.idle_frames[animation.current_frame].clone(),
            };

            sprite.image = image;
        }
    }
}
