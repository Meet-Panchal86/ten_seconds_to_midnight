use bevy::{color::Color, ecs::{entity::Entity, query::{With, Without}, system::{Commands, Query, Res, ResMut}}, math::{Vec2, Vec3}, sprite::Sprite, time::{Time, Timer}, transform::components::Transform, utils::default, window::{PrimaryWindow, Window}};
use crate::{components::{AttackCooldown, Collider, Dead, Player, Zombie}, resources::{Wave, WaveState::{self}}};

const MIN_SPAWN_DISTANCE: f32 = 300.0;
const SPAWN_PADDING: f32 = 30.0;
const ZOMBIE_SPEED: f32 = 140.0;
const MIN_ZOMBIE_SPAWN_DISTANCE: f32 = 50.0;
const MAX_SPAWN_ATTEMPTS: u32 = 100;

pub fn wave_manager(
    time: Res<Time>,
    mut wave: ResMut<Wave>,
    zombies: Query<Entity, With<Zombie>>,
    commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    player: Query<&Transform, (With<Player>, Without<Dead>)>,
) {
    if wave.state == WaveState::Fighting {
        if zombies.is_empty() {
            wave.state = WaveState::Waiting;
            wave.timer.reset();
        }    
    }else if wave.state == WaveState::Waiting {
            wave.timer.tick(time.delta());

            if wave.timer.is_finished() {
                let Ok(player_transform) = player.single() else {
                    return;
                };

                let player_position = player_transform.translation;
                
                wave.number += 1;
                spawn_wave(commands, wave.number, window, player_position);
                wave.state = WaveState::Fighting; 
            }
    }

}

pub fn spawn_wave(mut commands: Commands, wave_number: u32, window: Query<&Window, With<PrimaryWindow>>, player_position: Vec3) {
    let zombie_count = 3 * wave_number + 2;

    let Ok(window) = window.single() else {
        return;
    };

    let max_x = window.width()/2.0;
    let max_y = window.height()/2.0;    
    
    let mut local_zombie_positions = Vec::new();
    
    for _ in 0..zombie_count {
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
                if spawn_position.distance(*position) <= MIN_ZOMBIE_SPAWN_DISTANCE {
                    continue 'calculate_spawn_position;
                }
            }    
            if spawn_position.distance(player_position) >= MIN_SPAWN_DISTANCE {
                spawn_zombie(&mut commands, spawn_position);
                local_zombie_positions.push(spawn_position);
                break;
            }
        }
    } 
}

pub fn spawn_zombie(commands: &mut Commands, position: Vec3) {
    commands.spawn((
        Zombie,
        Collider{ radius: 10f32 },
        AttackCooldown{ timer: Timer::from_seconds(0.5, bevy::time::TimerMode::Once)},
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2::new(20.0,20.0)),
            ..default()
        },
        Transform::from_translation(position)
    ));
}

pub fn zombie_movement(
    time: Res<Time>, 
    player: Query<&Transform, (With<Player>, Without<Dead>)>,
    mut zombie: Query<&mut Transform, (With<Zombie>, Without<Player>)> ) {
    let speed = ZOMBIE_SPEED;

    let Ok(player_transform) = player.single() else {
        return;
    };

    for mut zombie_transform in &mut zombie {

        let direction = player_transform.translation - zombie_transform.translation;

        if direction == Vec3::ZERO {
            continue;
        }

        let direction = direction.normalize();

        zombie_transform.translation += direction * speed * time.delta_secs();
    }
}