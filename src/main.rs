use bevy::prelude::*;

mod systems;       // 包含所有游戏逻辑系统
mod components;    // 包含所有 ECS 组件

use systems::{game::game_logic, input::handle_input};
use components::{plant::Plant, zombie::Zombie};

const GRID_SIZE: f32 = 50.0; // 网格大小
const GRID_WIDTH: u32 = 9;   // 网格宽度
const GRID_HEIGHT: u32 = 5;  // 网格高度

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "植物大战僵尸 Demo".to_string(),
                resolution: (GRID_WIDTH as f32 * GRID_SIZE, GRID_HEIGHT as f32 * GRID_SIZE).into(),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(handle_input)
        .add_system(game_logic)
        .run();
}

// 初始化系统
fn setup(mut commands: Commands) {
    // 添加 2D 摄像机
    commands.spawn(Camera2dBundle::default());

    // 添加默认网格背景
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(x as f32 * GRID_SIZE, y as f32 * GRID_SIZE, 0.0),
                        scale: Vec3::new(GRID_SIZE, GRID_SIZE, 1.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::rgb(0.5, 0.8, 0.5),
                        ..default()
                    },
                    ..default()
                },
                // 网格坐标作为组件
                components::position::Position { x, y },
            ));
        }
    }
}