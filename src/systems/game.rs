use bevy::prelude::*;
use crate::components::{plant::Plant, zombie::Zombie, position::Position};

pub fn game_logic(
    mut commands: Commands,
    mut zombies_query: Query<(Entity, &mut Zombie, &Position)>,
    plants_query: Query<(&Plant, &Position)>,
) {
    for (zombie_entity, mut zombie, zombie_pos) in zombies_query.iter_mut() {
        for (plant, plant_pos) in plants_query.iter() {
            if zombie_pos.x == plant_pos.x && zombie_pos.y == plant_pos.y {
                // 植物攻击僵尸
                zombie.health = zombie.health.saturating_sub(plant.attack_power);

                // 僵尸死亡
                if zombie.health == 0 {
                    commands.entity(zombie_entity).despawn();
                    break;
                }
            }
        }
    }
}