use bevy::prelude::*;
use crate::components::{plant::{Plant, PlantType}, position::Position, projectile::Projectile, zombie::Zombie};
use crate::GRID_SIZE;

pub fn plant_shooting(
    mut commands: Commands,
    mut plants_query: Query<(&mut Plant, &Position, &Transform)>,
    zombies_query: Query<(&Position, &Zombie)>,
    time: Res<Time>,
) {
    for (mut plant, plant_pos, transform) in plants_query.iter_mut() {
        // 更新攻击计时器
        plant.attack_timer += time.delta_seconds();

        // 只有豌豆射手类植物会射击
        if !matches!(plant.plant_type, PlantType::Peashooter | PlantType::SnowPea | PlantType::Repeater) {
            continue;
        }

        // 检查是否到了攻击时间
        if plant.attack_timer < plant.attack_cooldown {
            continue;
        }

        // 检查同一行是否有僵尸
        let has_zombie_in_row = zombies_query.iter().any(|(zombie_pos, _)| {
            zombie_pos.y == plant_pos.y && zombie_pos.x > plant_pos.x
        });

        if has_zombie_in_row {
            // 射击豌豆
            let projectile = if matches!(plant.plant_type, PlantType::SnowPea) {
                Projectile::new_frozen_pea()
            } else {
                Projectile::new_pea()
            };

            let color = projectile.get_color();

            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(
                            transform.translation.x + GRID_SIZE / 2.0,
                            transform.translation.y,
                            1.0,
                        ),
                        scale: Vec3::new(10.0, 10.0, 1.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color,
                        ..default()
                    },
                    ..default()
                },
                projectile,
                Position {
                    x: plant_pos.x,
                    y: plant_pos.y,
                },
            ));

            // 双发射手发射第二颗豌豆
            if matches!(plant.plant_type, PlantType::Repeater) {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(
                                transform.translation.x + GRID_SIZE / 2.0 + 15.0,
                                transform.translation.y,
                                1.0,
                            ),
                            scale: Vec3::new(10.0, 10.0, 1.0),
                            ..default()
                        },
                        sprite: Sprite {
                            color: Projectile::new_pea().get_color(),
                            ..default()
                        },
                        ..default()
                    },
                    Projectile::new_pea(),
                    Position {
                        x: plant_pos.x,
                        y: plant_pos.y,
                    },
                ));
            }

            plant.attack_timer = 0.0;
        }
    }
}
