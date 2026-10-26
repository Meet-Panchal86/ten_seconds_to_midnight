use bevy::{app::{Plugin, Startup, Update}, ecs::schedule::IntoScheduleConfigs};

use crate::{game::persistence::load_save_data, player::movement::spawn_player, ui::{crosshair::{hide_cursor, spawn_crosshair, toggle_cursor, update_aim_offset}, game_over::gameover_text, hud::{setup_health_ui, setup_kill_ui, setup_wave_ui, update_health_text, update_kill_text, update_wave_text}}, zombie::animation::animate_zombie};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app 
            .add_systems(Startup,
                (
                setup_kill_ui,
                setup_wave_ui,
                setup_health_ui,
                load_save_data,
                spawn_crosshair.after(spawn_player),
                hide_cursor,
            ))
            .add_systems(Update, 
                (
                update_kill_text,
                update_wave_text,
                update_health_text.after(animate_zombie),
                gameover_text,
                update_aim_offset,
                toggle_cursor,
            ));
    }
}