use bevy::{ecs::component::Component, math::Vec2, time::Timer};

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