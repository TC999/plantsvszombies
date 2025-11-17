use bevy::prelude::*;
use crate::components::{plant::Plant, zombie::Zombie, position::Position};

pub fn game_logic(
    mut commands: Commands,
    mut zombies_query: Query<(Entity, &mut Zombie, &Position, &Transform)>,
    mut plants_query: Query<(Entity, &mut Plant, &Position), Without<Zombie>>,
    time: Res<Time>,
) {
    // 僵尸攻击植物
    for (_zombie_entity, mut zombie, _zombie_pos, zombie_transform) in zombies_query.iter_mut() {
        for (plant_entity, mut plant, plant_pos) in plants_query.iter_mut() {
            // 检查是否在同一位置
            let distance = Vec2::new(zombie_transform.translation.x, zombie_transform.translation.y)
                .distance(Vec2::new(plant_pos.x as f32 * 50.0, plant_pos.y as f32 * 50.0));

            if distance < 40.0 {
                // 更新僵尸攻击计时器
                zombie.attack_timer += time.delta_seconds();

                if zombie.attack_timer >= zombie.attack_cooldown {
                    // 僵尸攻击植物
                    plant.health = plant.health.saturating_sub(zombie.attack_power);

                    zombie.attack_timer = 0.0;

                    // 植物死亡
                    if plant.health == 0 {
                        commands.entity(plant_entity).despawn();
                    }
                }
            }
        }
    }
}
