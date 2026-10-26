use bevy::{ecs::{entity::Entity, system::{Commands, Query, Res}}, time::Time, transform::components::Transform};

use crate::combat::{components::Bullet, config::{BULLET_DESPAWN_BOUNDRY, BULLET_SPEED}};


pub fn bullet_movement(time: Res<Time>, mut bullet: Query<(Entity, &mut Bullet, &mut Transform)>, mut commands: Commands) {
    let speed = BULLET_SPEED;

    for (entity, mut bullet, mut transform) in &mut bullet {
        bullet.previous_position = transform.translation.truncate();

        transform.translation += bullet.direction.extend(0.0) * speed * time.delta_secs();
        
        let position = transform.translation.truncate();
    
        if  position.abs().x > BULLET_DESPAWN_BOUNDRY || position.abs().y > BULLET_DESPAWN_BOUNDRY {
            commands.entity(entity).despawn();
        }
    } 
}

