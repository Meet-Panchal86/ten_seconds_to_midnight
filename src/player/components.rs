use bevy::{asset::Handle, ecs::component::Component, image::Image, time::Timer};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerAnimation {
    pub idle_frames: Vec<Handle<Image>>,
    pub move_frames: Vec<Handle<Image>>,
    pub shoot_frames: Vec<Handle<Image>>,
    pub current_frame: usize,
    pub timer: Timer,
    pub state: PlayerAnimationState,
    pub previous_state: PlayerAnimationState,
}

#[derive(PartialEq, Clone, Copy)]
pub enum PlayerAnimationState {
    Idle,
    Move,
    Shoot,
}

#[derive(Component)]
pub struct ShootCooldown {
    pub timer: Timer,
}

#[derive(Component)]
pub struct AimLine;

#[derive(Component)]
pub struct MuzzleFlash{
    pub timer: Timer
}