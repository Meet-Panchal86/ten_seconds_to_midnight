use bevy::{ecs::resource::Resource, math::Vec2, time::Timer};

use crate::player::config::MUZZLE_OFFSET;

#[derive(Resource)]
pub struct Wave {
    pub number: u32,
    pub state: WaveState,
    pub timer: Timer,
}

#[derive(PartialEq)]
pub enum WaveState {
    NotStarted,
    Fighting,
    Waiting,
}

#[derive(Resource)]
pub struct Kills {
    pub kill: u32, 
}

pub const AIM_RADIUS: f32 = MUZZLE_OFFSET + 20.0;       // fixed crosshair distance from player
pub const MOUSE_SENSITIVITY: f32 = 1.0;  // tweak to taste

#[derive(Resource)]
pub struct AimOffset(pub Vec2);

impl Default for AimOffset {
    fn default() -> Self {
        Self(Vec2::new(AIM_RADIUS, 0.0))
    }
}

