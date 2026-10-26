use bevy::{ecs::{query::{With, Without}, system::{Query, Res}}, math::{Quat, Vec3}, time::Time, transform::components::Transform, window::{PrimaryWindow, Window}};

use crate::{components::Dead, player::{config::PLAYER_SPEED, components::Player}, resources::Wave, zombie::{config::BASE_SPEED, components::Zombie}};

pub fn zombie_movement(
    time: Res<Time>, 
    player: Query<&Transform, (With<Player>, Without<Dead>)>,
    mut zombie: Query<&mut Transform, (With<Zombie>, Without<Player>, Without<Dead>)>,
    window: Query<&Window, With<PrimaryWindow>>,
    wave: Res<Wave> 
) {
    let speed = (BASE_SPEED  * ( 1.0 + (0.02 * ( wave.number as f32 - 1.0 )))).min( PLAYER_SPEED * 0.85);

    let Ok(player_transform) = player.single() else {
        return;
    };

    let Ok(window) = window.single() else {
        return;
    };
    let max_x = window.width()/2.0 - 25.0;
    let max_y = window.height()/2.0 - 25.0;

    for mut zombie_transform in &mut zombie {

        let direction = player_transform.translation - zombie_transform.translation;

        if direction == Vec3::ZERO {
            continue;
        }

        let direction = direction.normalize();
        let mut transform = zombie_transform.translation + direction * speed as f32 * time.delta_secs();

        let new_x = transform.x.clamp(-max_x, max_x);
        let new_y = transform.y.clamp(-max_y, max_y); 

        transform.x = new_x;
        transform.y = new_y;

        zombie_transform.translation = transform;
        
        let angle = direction.y.atan2(direction.x);
        zombie_transform.rotation = Quat::from_rotation_z(angle);
    }
}