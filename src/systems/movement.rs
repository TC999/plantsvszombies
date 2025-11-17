use bevy::prelude::*;
use crate::components::{zombie::Zombie, position::Position, plant::Plant};

pub fn move_zombies(
    mut query: Query<(&mut Transform, &Zombie), Without<Plant>>,
    plants_query: Query<(&Position, &Transform), With<Plant>>,
    time: Res<Time>,
) {
    for (mut transform, zombie) in query.iter_mut() {
        // 检查前方是否有植物
        let mut should_move = true;

        for (_plant_pos, plant_transform) in plants_query.iter() {
            let distance = transform.translation.distance(plant_transform.translation);

            // 如果前方有植物，停止移动
            if distance < 40.0 && transform.translation.x > plant_transform.translation.x {
                should_move = false;
                break;
            }
        }

        // 只有在前方没有植物时才移动
        if should_move {
            transform.translation.x -= zombie.speed * time.delta_seconds();
        }
    }
}
