use bevy::{app::{Plugin, Update}, camera::ClearColor, color::Color, ecs::schedule::IntoScheduleConfigs};

use crate::game::{game_lifecycle::{highest_wave_reached, restart_game, start_selected_wave}, resources::SaveData};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .insert_resource(SaveData::default())
            .insert_resource(ClearColor(Color::srgb(0.025, 0.025, 0.035)))
            .add_systems(
            Update, 
            (  
                restart_game,
                highest_wave_reached.after(start_selected_wave),
                start_selected_wave.after(restart_game),
        ));
    }
}