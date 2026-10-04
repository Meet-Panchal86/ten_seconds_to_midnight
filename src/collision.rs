use bevy::{ecs::{entity::{Entity, EntityHashSet}, query::{With, Without}, system::{Commands, Query, Res, ResMut}}, math::{Vec2, Vec3}, time::Time, transform::components::Transform};

use crate::{components::{AttackCooldown, Bullet, Collider, Dead, Player, Zombie, ZombieAnimation, ZombieAnimationState}, resources::{Kills}};

const SEPARATION_FAR: f32 = 60.0;   // 45
pub const SEPARATION_NEAR: f32 = 5.0;   // 30
const NEAR_DISTANCE: f32 = 70.0;
const FAR_DISTANCE: f32 = 160.0;


pub fn zombie_player_collision(
    time: Res<Time>,
    player: Query<(&Transform, &Collider), (With<Player>, Without<Dead>)>,
    mut zombies: Query<(&mut Transform, &Collider, &mut AttackCooldown, &mut ZombieAnimation), (With<Zombie>, Without<Player>, Without<Dead>)>,
) {
    let Ok((player_transform, player_collider)) = player.single() else {
        return;
    };
    

    for (mut zombie_transform, zombie_collider, mut attack_cooldown, mut animation) in &mut zombies {
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
                animation.state = ZombieAnimationState::Attack;
                attack_cooldown.timer.reset();
            }
        }

    }
}

const SEPARATION_ITERATIONS: usize = 5;

pub fn zombie_zombie_collision(
    mut zombies: Query<(&mut Transform, &Collider), (With<Zombie>, Without<Player>, Without<Dead>)>,
    player: Query<&Transform, (With<Player>, Without<Zombie>)>,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();

    let mut bodies: Vec<(Vec2, f32)> = zombies
        .iter()
        .map(|(t, c)| (t.translation.truncate(), c.radius))
        .collect();

    for _ in 0..SEPARATION_ITERATIONS {
        for i in 0..bodies.len() {
            for j in (i + 1)..bodies.len() {
                let (pos_a, radius_a) = bodies[i];
                let (pos_b, radius_b) = bodies[j];

                let offset = pos_b - pos_a;
                let distance = offset.length();

                let player_distance = pos_a.distance(player_pos).min(pos_b.distance(player_pos));
                let t = ((player_distance - NEAR_DISTANCE) / (FAR_DISTANCE - NEAR_DISTANCE)).clamp(0.0, 1.0);
                let extra = SEPARATION_NEAR + (SEPARATION_FAR - SEPARATION_NEAR) * t;
                let min_allowed_distance = radius_a + radius_b + extra;

                if distance < min_allowed_distance {
                    let direction = if distance < 0.001 { Vec2::X } else { offset / distance };
                    let push = direction * (min_allowed_distance - distance) * 0.5;
                    bodies[i].0 -= push;
                    bodies[j].0 += push;
                }
            }
        }
    }

    for ((mut transform, _), (pos, _)) in zombies.iter_mut().zip(bodies) {
        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
    }
}

// pub fn zombie_zombie_collision(
//     mut zombies: Query<(Entity, &mut Transform, &Collider), (With<Zombie>, Without<Player>, Without<Dead>)>,
//     player: Query<&Transform, With<Player>>
// ) {
//     let Ok(player_transform) = player.single() else {
//         return;
//     };

//     let entities: Vec<Entity> = zombies
//         .iter()
//         .map(|(entity, _, _)| entity)
//         .collect();


//     for i in 0..entities.len() {
//         for j in (i + 1)..entities.len() {
//             let Ok([(_, mut transform_a, collider_a), (_, mut transform_b, collider_b)]) =
//                 zombies.get_many_mut([entities[i], entities[j]])
//             else {
//                 continue;
//             };

//             let offset = transform_b.translation - transform_a.translation;
//             let distance = offset.length();

//             let player_distance = transform_a.translation.distance(player_transform.translation).min(transform_b.translation.distance(player_transform.translation));

//             let t = ((player_distance - NEAR_DISTANCE) / (FAR_DISTANCE - NEAR_DISTANCE)).clamp(0.0, 1.0);

//             let extra = SEPARATION_NEAR + (SEPARATION_FAR - SEPARATION_NEAR) * t;

//             let min_allowed_distance = collider_a.radius + collider_b.radius + extra;

//             if distance < min_allowed_distance {
//                 let overlap = min_allowed_distance - distance;

//                 let direction = if distance == 0.0 {
//                     Vec3::X
//                 } else {
//                     offset.normalize()
//                 };

//                 transform_a.translation -= direction * overlap/ZOMBIE_SEPARATION_STRENGTH;
//                 transform_b.translation += direction * overlap/ZOMBIE_SEPARATION_STRENGTH;
//             }

//         }
//     }
// }

pub fn bullet_zombie_collision(
    zombies: Query<(Entity, &Transform, &Collider), (With<Zombie>, Without<Dead>)>,
    bullets: Query<(Entity, &Bullet, &Transform, &Collider), With<Bullet>>,
    mut kills: ResMut<Kills>,
    mut commands: Commands,
) {
    'bullet: for (bullet_entity, bullet, bullet_transform, bullet_collider) in &bullets {
        let start = bullet.previous_position;
        let end = bullet_transform.translation.truncate();

        let mut dead_zombies = EntityHashSet::new();

        for (zombie_entity, zombie_transform, zombie_collider) in  zombies {
            let zombie_position = zombie_transform.translation.truncate();
            let segment = end - start;

            let closest_point = if segment.length_squared() == 0.0 {
                start
            } else {
                let t = ((zombie_position - start).dot(segment)
                    / segment.length_squared())
                    .clamp(0.0, 1.0);

                start + segment * t
            };

            let distance = closest_point.distance(zombie_position);
            let collision_distance = zombie_collider.radius + bullet_collider.radius;

            if distance <= collision_distance {                
                if !dead_zombies.contains(&zombie_entity) {
                    commands.entity(bullet_entity).despawn();
                
                    commands.entity(zombie_entity).insert(Dead);
                    kills.kill += 1;

                    dead_zombies.entry(zombie_entity);
                }
                continue 'bullet;
            }
        }
    }
}