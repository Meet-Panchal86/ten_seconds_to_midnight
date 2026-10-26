use bevy::{asset::Handle, ecs::component::Component, image::Image, time::Timer};

#[derive(Component)]
pub struct Zombie;

#[derive(Component)]
pub struct AttackCooldown {
    pub timer: Timer,
}

#[derive(Component)]
pub struct ZombieAnimation {
    pub idle_frames: Vec<Handle<Image>>,
    pub move_frames: Vec<Handle<Image>>,
    pub attack_frames: Vec<Handle<Image>>,
    pub current_frame: usize,
    pub timer: Timer,
    pub state: ZombieAnimationState,
    pub previous_state: ZombieAnimationState
}

#[derive(PartialEq, Clone, Copy)]
pub enum ZombieAnimationState {
    Idle,
    Move,
    Attack
}


#[derive(Component)]
pub struct DeathEffectSpawned;

#[derive(Component)]
pub struct DeathEffect{
    pub frames: Vec<Handle<Image>>,
    pub current_frame: usize,
    pub timer: Timer,
}
