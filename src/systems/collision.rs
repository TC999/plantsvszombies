use bevy::prelude::*;
use crate::components::{projectile::Projectile, zombie::Zombie};

pub fn projectile_collision(
    mut commands: Commands,
    projectiles_query: Query<(Entity, &Transform, &Projectile)>,
    mut zombies_query: Query<(Entity, &Transform, &mut Zombie)>,
) {
    for (proj_entity, proj_transform, projectile) in projectiles_query.iter() {
        for (zombie_entity, zombie_transform, mut zombie) in zombies_query.iter_mut() {
            let distance = proj_transform.translation.distance(zombie_transform.translation);

            // 简单的碰撞检测
            if distance < 30.0 {
                // 造成伤害
                zombie.health = zombie.health.saturating_sub(projectile.damage);

                // 移除子弹
                commands.entity(proj_entity).despawn();

                // 如果僵尸死亡，移除僵尸
                if zombie.health == 0 {
                    commands.entity(zombie_entity).despawn();
                }

                break;
            }
        }
    }
}
