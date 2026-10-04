use bevy::{asset::Handle, ecs::component::Component, image::Image, math::Vec2, time::Timer};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Bullet {
    pub direction: Vec2,
    pub previous_position: Vec2,
}

#[derive(Component)]
pub struct Zombie;

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
}

#[derive(Component)]
pub struct AttackCooldown {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Dead;

#[derive(Component)]
pub struct KillText;

#[derive(Component)]
pub struct ShootCooldown {
    pub timer: Timer,
}

#[derive(Component)]
pub struct WaveText;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct GameOverText;

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
pub struct MuzzleFlash{
    pub timer: Timer
}

#[derive(Component)]
pub struct AimLine;

#[derive(Component)]
pub struct Crosshair;

#[derive(Component)]
pub struct ArenaBorder;

#[derive(Component)]
pub struct DeathEffectSpawned;

#[derive(Component)]
pub struct DeathEffect{
    pub frames: Vec<Handle<Image>>,
    pub current_frame: usize,
    pub timer: Timer,
}
