use std::{fs, io::Write, path::PathBuf};

use bevy::ecs::system::ResMut;
use directories::ProjectDirs;

use crate::game::resources::SaveData;

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

pub fn load_save_data(
    mut save_data: ResMut<SaveData>,
) {
    if let Some(highest_wave) = load_highest_wave_from_disk() {
        save_data.highest_wave = highest_wave;
    }
}