use bevy::{asset::{AssetServer, Handle}, audio::{AudioPlayer, PlaybackMode, PlaybackSettings, Volume}, ecs::{entity::Entity, query::{With, Without}, system::{Commands, Query, Res, ResMut}}, image::Image, math::{Quat, Vec2, Vec3}, sprite::Sprite, time::{Time, Timer, TimerMode}, transform::components::Transform, utils::default, window::{PrimaryWindow, Window}};
use crate::{asset_resources::{AudioAssets, ZombieAnimationAssets}, components::{AttackCooldown, Collider, Dead, DeathEffect, DeathEffectSpawned, Health, Player, Zombie, ZombieAnimation, ZombieAnimationState}, player::PLAYER_SPEED, resources::{Wave, WaveState::{self}}};

const MIN_PLAYER_ZOMBIE_SPAWN_DISTANCE: f32 = 240.0;
const SPAWN_PADDING: f32 = 30.0;
const BASE_SPEED: f32 = 140.0;
const MIN_ZOMBIE_ZOMBIE_SPAWN_DISTANCE: f32 = 60.0;
const MAX_SPAWN_ATTEMPTS: u32 = 200;

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


// use std::f32::consts::TAU;

// const SLOT_SPACING: f32 =  28.0  + SEPARATION_NEAR;     // matches SEPARATION_NEAR 30 + radii 28
// const RING_SPACING: f32 = 5.0;     // gap between rings
// const CONTACT_PADDING: f32 = 1.0;   // ring 0 sits this far past touching
// const ARRIVE_SLOWDOWN: f32 = 40.0;  // start slowing this far from the slot

// pub fn zombie_movement(
//     time: Res<Time>,
//     player: Query<(&Transform, &Collider), (With<Player>, Without<Dead>)>,
//     mut zombie: Query<&mut Transform, (With<Zombie>, Without<Player>, Without<Dead>)>,
//     window: Query<&Window, With<PrimaryWindow>>,
//     wave: Res<Wave>,
// ) {
//     let speed = (BASE_SPEED * (1.0 + 0.01 * (wave.number as f32 - 1.0))).min(PLAYER_SPEED * 0.85);

//     let Ok((player_transform, player_collider)) = player.single() else { return; };
//     let Ok(window) = window.single() else { return; };
//     let max_x = window.width() / 2.0 - 25.0;
//     let max_y = window.height() / 2.0 - 25.0;

//     let player_pos = player_transform.translation.truncate();
//     let contact = player_collider.radius + 14.0 + CONTACT_PADDING; // 14 = zombie radius

//     // (index, distance, angle) for every zombie
//     let mut zs: Vec<(usize, f32, f32)> = zombie
//         .iter()
//         .enumerate()
//         .map(|(i, t)| {
//             let d = t.translation.truncate() - player_pos;
//             (i, d.length(), d.y.atan2(d.x))
//         })
//         .collect();
//     zs.sort_by(|a, b| a.1.total_cmp(&b.1)); // closest zombies fill inner rings

//     let mut targets = vec![player_pos; zs.len()];
//     let (mut start, mut ring) = (0, 0);
//     while start < zs.len() {
//         let r = contact + ring as f32 * RING_SPACING;
//         let capacity = ((TAU * r / SLOT_SPACING).floor() as usize).max(1);
//         let end = (start + capacity).min(zs.len());
//         let group = &mut zs[start..end];
//         group.sort_by(|a, b| a.2.total_cmp(&b.2)); // keep cyclic order

//         let step = TAU / group.len() as f32;

//         // rotate the evenly spaced slots to best match current angles
//         let (mut sx, mut sy) = (0.0, 0.0);
//         for (k, z) in group.iter().enumerate() {
//             let residual = z.2 - k as f32 * step;
//             sx += residual.cos();
//             sy += residual.sin();
//         }
//         let offset = sy.atan2(sx);

//         for (k, z) in group.iter().enumerate() {
//             targets[z.0] = player_pos + Vec2::from_angle(offset + k as f32 * step) * r;
//         }
//         start = end;
//         ring += 1;
//     }

//     for (i, mut zombie_transform) in zombie.iter_mut().enumerate() {
//         let pos = zombie_transform.translation.truncate();
//         let to_target = targets[i] - pos;
//         let dist = to_target.length();

//         if dist > 0.5 {
//             let arrive = (dist / ARRIVE_SLOWDOWN).min(1.0); // ease into the slot
//             let step = to_target / dist * speed * arrive * time.delta_secs();
//             let new_pos = pos + step.clamp_length_max(dist);
//             zombie_transform.translation.x = new_pos.x.clamp(-max_x, max_x);
//             zombie_transform.translation.y = new_pos.y.clamp(-max_y, max_y);
//         }

//         // still face the player, not the slot
//         let to_player = player_pos - zombie_transform.translation.truncate();
//         zombie_transform.rotation = Quat::from_rotation_z(to_player.y.atan2(to_player.x));
//     }
// }

pub fn zombie_movement(
    time: Res<Time>, 
    player: Query<&Transform, (With<Player>, Without<Dead>)>,
    mut zombie: Query<&mut Transform, (With<Zombie>, Without<Player>, Without<Dead>)>,
    window: Query<&Window, With<PrimaryWindow>>,
    wave: Res<Wave> 
) {
    let speed = (BASE_SPEED  * ( 1.0 + (0.02 * ( wave.number as f32 - 1.0 )))).min( PLAYER_SPEED * 0.85);

    let Ok(player_transform) = player.single() else {
        return;
    };

    let Ok(window) = window.single() else {
        return;
    };
    let max_x = window.width()/2.0 - 25.0;
    let max_y = window.height()/2.0 - 25.0;

    for mut zombie_transform in &mut zombie {

        let direction = player_transform.translation - zombie_transform.translation;

        if direction == Vec3::ZERO {
            continue;
        }

        let direction = direction.normalize();
        let mut transform = zombie_transform.translation + direction * speed as f32 * time.delta_secs();

        let new_x = transform.x.clamp(-max_x, max_x);
        let new_y = transform.y.clamp(-max_y, max_y); 

        transform.x = new_x;
        transform.y = new_y;

        zombie_transform.translation = transform;
        
        let angle = direction.y.atan2(direction.x);
        zombie_transform.rotation = Quat::from_rotation_z(angle);
    }
}