use bevy::prelude::*;
use crate::components::{zombie::{Zombie, ZombieType}, position::Position};
use crate::{GRID_SIZE, GRID_WIDTH, GRID_HEIGHT};
use rand::Rng;

#[derive(Resource)]
pub struct ZombieSpawnTimer {
    pub timer: f32,
    pub interval: f32,
}

impl Default for ZombieSpawnTimer {
    fn default() -> Self {
        ZombieSpawnTimer {
            timer: 0.0,
            interval: 10.0, // 每10秒生成一个僵尸
        }
    }
}

pub fn spawn_zombies(
    mut commands: Commands,
    mut spawn_timer: ResMut<ZombieSpawnTimer>,
    time: Res<Time>,
) {
    spawn_timer.timer += time.delta_seconds();

    if spawn_timer.timer >= spawn_timer.interval {
        spawn_timer.timer = 0.0;

        // 随机选择一行
        let mut rng = rand::thread_rng();
        let row = rng.gen_range(0..GRID_HEIGHT);

        // 随机选择僵尸类型
        let zombie_type = match rng.gen_range(0..100) {
            0..=60 => ZombieType::Normal,
            61..=80 => ZombieType::ConeHead,
            81..=90 => ZombieType::BucketHead,
            91..=95 => ZombieType::FlagZombie,
            _ => ZombieType::NewspaperZombie,
        };

        let zombie = Zombie::new(zombie_type);
        let color = zombie.get_color();

        // 计算居中网格中的僵尸生成位置
        // 标记为未使用的变量，以避免编译器警告
        let _screen_width = GRID_WIDTH as f32 * GRID_SIZE;
        let zombie_x = (GRID_WIDTH as f32 + 1.0) * GRID_SIZE / 2.0 + 100.0; // 屏幕右侧
        let zombie_y = (row as f32 - (GRID_HEIGHT as f32 - 1.0) / 2.0) * GRID_SIZE; // 居中行位置
        
        // 在屏幕右侧生成僵尸
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        zombie_x,
                        zombie_y,
                        1.0,
                    ),
                    scale: Vec3::new(40.0, 40.0, 1.0),
                    ..default()
                },
                sprite: Sprite {
                    color,
                    ..default()
                },
                ..default()
            },
            zombie,
            Position {
                x: GRID_WIDTH + 2,
                y: row,
            },
        ));
    }
}
