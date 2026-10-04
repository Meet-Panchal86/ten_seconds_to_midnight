use std::{fs, io::Write, path::PathBuf};

use bevy::{ecs::{entity::Entity, query::{With, Without}, system::{Commands, Query, Res, ResMut}}, input::{ButtonInput, keyboard::KeyCode}, math::Vec3, transform::components::Transform, window::{PrimaryWindow, Window}};
use directories::ProjectDirs;

use crate::{asset_resources::ZombieAnimationAssets, components::{Bullet, Dead, GameOverText, Health, Player, ShootCooldown, Zombie}, resources::{Kills, SaveData, Wave, WaveState}, zombie::spawn_wave};

pub fn start_selected_wave(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    player: Query<&Transform, With<Player>>,
    save_data: Res<SaveData>,
    input: Res<ButtonInput<KeyCode>>,
    mut wave: ResMut<Wave>,
    zombie_assets: Res<ZombieAnimationAssets>
) {
    let Ok(player_transform) = player.single() else {
        return;
    };

    if wave.state != WaveState::NotStarted {
        return;
    }

    if input.just_pressed(KeyCode::Digit1) {
        wave.number = 1;
        wave.timer.reset();
        wave.state = WaveState::Fighting;
        spawn_wave(&mut commands, wave.number, window, player_transform.translation, zombie_assets);
    } else if input.just_pressed(KeyCode::Digit2) {
        if save_data.highest_wave >= 1 {
            wave.timer.reset();    
            wave.number = save_data.highest_wave;
            wave.state = WaveState::Fighting;    
            spawn_wave(&mut commands, wave.number, window, player_transform.translation, zombie_assets);
        } 
    }
}

pub fn restart_game(
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(Entity, &mut Transform, &mut Health, &mut ShootCooldown), (With<Player>, With<Dead>)>,
    zombies: Query<Entity, (With<Zombie>, Without<Player>)>,
    bullets: Query<Entity, With<Bullet>>,
    mut kills: ResMut<Kills>,
    mut wave: ResMut<Wave>,
    gameover: Query<Entity, With<GameOverText>>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::KeyR) {
        
        let Ok((player_entity, mut transform, mut health, mut shoot_cooldown)) = player.single_mut() else {
            return;
        };

        for entity in bullets {
            commands.entity(entity).despawn();
        }

        for entity in zombies {
            commands.entity(entity).despawn();
        }
        
        if let Ok(entity) = gameover.single() {
            commands.entity(entity).despawn();
        }
        
        commands.entity(player_entity).remove::<Dead>();

        health.as_mut().current = 100.0;

        transform.translation = Vec3::new(0.0, 0.0, 0.0);

        shoot_cooldown.timer.reset();
        kills.kill = 0;
        wave.number = 1;
        wave.state = WaveState::NotStarted;
        wave.timer.reset();
    }
}

pub fn load_save_data(
    mut save_data: ResMut<SaveData>,
) {
    if let Some(highest_wave) = load_highest_wave_from_disk() {
        save_data.highest_wave = highest_wave;
    }
}

pub fn highest_wave_reached(
    mut save_data: ResMut<SaveData>,
    wave: Res<Wave>,
) {
    if wave.state == WaveState::NotStarted {
        return;
    }
    
    if wave.number > save_data.highest_wave {
        save_data.highest_wave = wave.number;
        save_highest_wave_to_disk(save_data.highest_wave);
    }
}


pub fn load_highest_wave_from_disk() -> Option<u32>{  
    if let Some(path) = save_path() {
        if let Ok(highest_wave) = fs::read_to_string(path) {
            if let Ok(highest_wave) = highest_wave.trim().parse::<u32>() {
                return Some(highest_wave)
            }    
        } 
    } 
    None
}


pub fn save_highest_wave_to_disk(highest_wave: u32) {
    if let Some(save_path) = save_path() {
        if let Some(parent_dir) = save_path.parent() {
            if let Ok(_) = std::fs::create_dir_all(parent_dir) {
                if let Ok(mut file) = std::fs::File::create(save_path) {
                    let data = format!("{}",highest_wave);
                    let data_as_bytes = data.as_bytes();     
                    
                    match file.write_all(data_as_bytes) {
                        Ok(_) => {},
                        Err(e) => eprintln!("{}",e)
                    }
                }
                else {
                    eprintln!("file creation failed")
                }
            } else {
                eprintln!("directory creation failed")
            };        
        } else {
            eprintln!("parent directory not found");
        };
    } else {
        eprintln!("save path unavailable");
    }
}


pub fn save_path() -> Option<PathBuf> {
    if let Some(dir) = ProjectDirs::from("com", "MeetPanchal", "ten_seconds_to_midnight") {
        let data_dir = dir.data_dir();
        let save_file = data_dir.join("save.dat");

        Some(save_file)
    } else {
        None
    }
}