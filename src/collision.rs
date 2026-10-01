use bevy::{ecs::{entity::Entity, query::{With, Without}, system::{Commands, Query, Res}}, math::Vec3, time::Time, transform::components::Transform};

use crate::components::{AttackCooldown, Bullet, Collider, Dead, Health, Player, Zombie};

pub fn zombie_player_collision(
    time: Res<Time>,
    mut player: Query<(&Transform, &Collider, &mut Health), (With<Player>, Without<Dead>)>,
    mut zombies: Query<(&mut Transform, &Collider, &mut AttackCooldown), (With<Zombie>, Without<Player>)>
) {
    let Ok((player_transform, player_collider, mut player_health)) = player.single_mut() else {
        return;
    };
    

    for (mut zombie_transform, zombie_collider, mut attack_cooldown) in &mut zombies {
        let distance = player_transform.translation.distance(zombie_transform.translation);
        let min_allowed_distance =  player_collider.radius + zombie_collider.radius;

        attack_cooldown.timer.tick(time.delta());
        if distance <= min_allowed_distance {
            let direction = if distance == 0.0 {
                Vec3::X 
            }
            else {
                (zombie_transform.translation - player_transform.translation).normalize()
            };
 
            zombie_transform.translation = player_transform.translation + direction * min_allowed_distance;

            if attack_cooldown.timer.is_finished() {
                player_health.current = (player_health.current - 10.0).max(0.0);
                attack_cooldown.timer.reset();
                println!("{}", player_health.current);
            }
        }

    }
}

pub fn zombie_zombie_collision(
    mut zombies: Query<(Entity, &mut Transform, &Collider), With<Zombie>>,
) {
    let entities: Vec<Entity> = zombies
        .iter()
        .map(|(entity, _, _)| entity)
        .collect();

    for i in 0..entities.len() {
        for j in (i + 1)..entities.len() {
            let Ok([(_, mut transform_a, collider_a), (_, mut transform_b, collider_b)]) =
                zombies.get_many_mut([entities[i], entities[j]])
            else {
                continue;
            };

            let offset = transform_b.translation - transform_a.translation;
            let distance = offset.length();
            let min_allowed_distance = collider_a.radius + collider_b.radius + ZOMBIE_SEPARATION ;
            const ZOMBIE_SEPARATION: f32 = 10.0;

            if distance < min_allowed_distance {
                let overlap = min_allowed_distance - distance;
                const ZOMBIE_SEPARATION_STRENGTH: f32 = 2.0;

                let direction = if distance == 0.0 {
                    Vec3::X
                } else {
                    offset.normalize()
                };

                transform_a.translation -= direction * overlap/ZOMBIE_SEPARATION_STRENGTH;
                transform_b.translation += direction * overlap/ZOMBIE_SEPARATION_STRENGTH;
            }

        }
    }
}

pub fn bullet_zombie_collision(
    zombies: Query<(Entity, &Transform, &Collider), With<Zombie>>,
    bullets: Query<(Entity, &Transform, &Collider), With<Bullet>>,
    mut commands: Commands
) {
    'bullet: for (bullet_entity, bullet_transform, bullet_collider) in &bullets {
        for (zombie_entity, zombie_transform, zombie_collider) in &zombies {
            let distance = bullet_transform.translation.distance(zombie_transform.translation);
            let collision_distance = zombie_collider.radius + bullet_collider.radius;

            if distance <= collision_distance {
                commands.entity(bullet_entity).despawn();
                commands.entity(zombie_entity).despawn();

                continue 'bullet;
            }
        }
    }
}