use bevy::{app::{Plugin, Update}, ecs::schedule::IntoScheduleConfigs};

use crate::{combat::{bullet::bullet_movement, collision::{bullet_zombie_collision, zombie_player_collision, zombie_zombie_collision}}, zombie::spawning::spawn_zombie_death};

pub struct CombatPlugin;


impl Plugin for CombatPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app 
            .add_systems(
                Update
                , 
                (
                    bullet_movement.before(bullet_zombie_collision),
                    bullet_zombie_collision.before(spawn_zombie_death)
                                            .after(zombie_zombie_collision)
                                            .after(zombie_player_collision),
                    zombie_zombie_collision, 
                    zombie_player_collision,
                ));
    }
}