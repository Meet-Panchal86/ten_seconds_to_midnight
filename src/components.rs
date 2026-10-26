use bevy::{ecs::component::Component};

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
}


#[derive(Component)]
pub struct Dead;
