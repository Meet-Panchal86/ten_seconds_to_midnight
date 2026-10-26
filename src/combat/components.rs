use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component)]
pub struct Bullet {
    pub direction: Vec2,
    pub previous_position: Vec2,
}