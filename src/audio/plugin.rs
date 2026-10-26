use bevy::{app::{Plugin, Startup}, ecs::schedule::IntoScheduleConfigs};

use crate::audio::audio::{game_sound, load_audio_assets};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .add_systems(Startup, 
                (
                load_audio_assets,
                game_sound.after(load_audio_assets),
            ));
    }
}