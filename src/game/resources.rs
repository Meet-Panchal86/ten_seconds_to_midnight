use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct SaveData {
    pub highest_wave: u32,
}

impl Default for SaveData {
    fn default() -> Self {
        Self {
            highest_wave: 0,
        }
    }
}