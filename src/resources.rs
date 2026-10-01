use bevy::{ecs::resource::Resource, time::Timer};

#[derive(Resource)]
pub struct Wave {
    pub number: u32,
    pub state: WaveState,
    pub timer: Timer,
}

#[derive(PartialEq)]
pub enum WaveState {
    Fighting,
    Waiting,
}