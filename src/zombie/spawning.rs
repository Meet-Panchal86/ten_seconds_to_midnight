use bevy::{asset::{AssetServer, Handle}, audio::{AudioPlayer, PlaybackMode, PlaybackSettings, Volume}, ecs::{entity::Entity, query::{With, Without}, system::{Commands, Query, Res, ResMut}}, image::Image, math::{Vec2, Vec3}, sprite::Sprite, time::{Time, Timer, TimerMode}, transform::components::Transform, utils::default, window::{PrimaryWindow, Window}};

use crate::{audio::audio::AudioAssets, components::{Collider, Dead, Health}, player::components::Player, resources::{Wave, WaveState}, zombie::{asset_resource::ZombieAnimationAssets, components::{AttackCooldown, DeathEffect, DeathEffectSpawned, Zombie, ZombieAnimation, ZombieAnimationState}, config::{MAX_SPAWN_ATTEMPTS, MIN_PLAYER_ZOMBIE_SPAWN_DISTANCE, MIN_ZOMBIE_ZOMBIE_SPAWN_DISTANCE, SPAWN_PADDING}}};

pub fn spawn_zombie_death(
    mut commands: Commands,
    zombies: Query<(Entity, &Transform), (With<Zombie>, With<Dead>, Without<DeathEffectSpawned>)>,
    asset_server: Res<AssetServer>,
    audio_assets: Res<AudioAssets>,
) {
    if zombies.is_empty() {
        return;
    }

    let death_frames = (0..3)
    .map(|i| {
        asset_server.load(format!("zombie/death/B10{}.png", i))
    }).collect::<Vec<Handle<Image>>>();


    for (entity, transform) in zombies {
        commands.spawn((
            AudioPlayer::new(audio_assets.zombie_crunch.clone()),
            PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: Volume::Linear(0.85),
                //speed: 1.1,
                ..default()
            }
        ));
  
        commands.spawn((
            DeathEffect{
                frames: death_frames.clone(),
                current_frame: 0,
                timer: Timer::from_seconds(0.08, TimerMode::Once),
            },
            Sprite{
                image: asset_server.load("zombie/death/B100.png"),
                custom_size: Some(Vec2::new(55.0, 55.0)),
                ..default()
            },
            Transform::from_translation(transform.translation)
        ));
        
        commands.entity(entity).despawn();
    }
}


pub fn wave_manager(
    time: Res<Time>,
    mut wave: ResMut<Wave>,
    zombies: Query<Entity, (With<Zombie>, Without<Dead>)>,
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    mut player: Query<(&Transform, &mut Health), (With<Player>, Without<Dead>, Without<Zombie>)>,
    zombie_assets: Res<ZombieAnimationAssets>
) {
    if wave.state == WaveState::NotStarted {

    }
    else if wave.state == WaveState::Fighting {
        if zombies.is_empty() {
            wave.state = WaveState::Waiting;
            wave.timer.reset();
        }    
    }else if wave.state == WaveState::Waiting {
            wave.timer.tick(time.delta());

            if wave.timer.is_finished() {
                let Ok((player_transform,mut  health)) = player.single_mut() else {
                    return;
                };

                let player_position = player_transform.translation;

                if wave.number > 9 {
                    health.current = 100.0;
                }
                else if wave.number > 6 {
                    health.current = ( health.current + 50.0).min(100.0);
                }
                else {
                    health.current = (health.current + 20.0).min(100.0);
                }

                wave.number += 1;
                spawn_wave(&mut commands, wave.number, window, player_position, zombie_assets);

                wave.state = WaveState::Fighting; 
            }
    }

}

pub fn spawn_wave(
    mut commands: &mut Commands,
    wave_number: u32, 
    window: Query<&Window, With<PrimaryWindow>>, 
    player_position: Vec3,
    zombie_assets: Res<ZombieAnimationAssets>
) {
    let n0 = 8.0;
    let a = 5.0;
    let b = 1.5;
    let p = 1.2;

    let w = (wave_number - 1) as f32;
    let zombie_count = (n0 + a * w + b * w.powf(p)).floor();

    let Ok(window) = window.single() else {
        return;
    };

    let max_x = window.width()/2.0;
    let max_y = window.height()/2.0;    
    
    let mut local_zombie_positions = Vec::new();
    
    for _ in 0..zombie_count as u32{
        let mut attempt: u32  = 0;
        'calculate_spawn_position: loop { 
            if attempt >= MAX_SPAWN_ATTEMPTS {
                break;
            }

            attempt += 1;

            let random_x = rand::random_range((-max_x + SPAWN_PADDING)..(max_x - SPAWN_PADDING));
            let random_y = rand::random_range((-max_y + SPAWN_PADDING)..(max_y - SPAWN_PADDING));
            
            let spawn_position = Vec3::new(random_x, random_y, 0.0);

            for position in local_zombie_positions.iter() {
                if spawn_position.distance(*position) <= MIN_ZOMBIE_ZOMBIE_SPAWN_DISTANCE {
                    continue 'calculate_spawn_position;
                }
            }    
            if spawn_position.distance(player_position) >= MIN_PLAYER_ZOMBIE_SPAWN_DISTANCE {
                spawn_zombie(&mut commands, spawn_position, &zombie_assets);
                local_zombie_positions.push(spawn_position);
                break;
            }
        }
    } 
}

pub fn spawn_zombie(commands: &mut Commands, position: Vec3, animation: &Res<ZombieAnimationAssets>) {
    commands.spawn((
        Zombie,
        Collider{ radius: 14f32 },
        AttackCooldown{ timer: Timer::from_seconds(1.2, bevy::time::TimerMode::Once)},
        Sprite {
            image: animation.movement.get(0).expect("Failed to load first movement frame for zombie sprite").clone(),
            custom_size: Some(Vec2::new(55.0,55.0)),
            ..default()
        },
        ZombieAnimation {
            idle_frames: animation.idle.clone(),
            move_frames: animation.movement.clone(),
            attack_frames: animation.attack.clone(),
            current_frame: 0,
            timer: Timer::from_seconds(0.1, bevy::time::TimerMode::Repeating),
            state: ZombieAnimationState::Move,
            previous_state: ZombieAnimationState::Move
        },
        Transform::from_translation(position + Vec3::Z * 0.5)
    ));
}
