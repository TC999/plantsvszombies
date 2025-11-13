use bevy::prelude::*;

mod systems;
mod components;

use systems::{game::game_logic, input::handle_input};
use components::{plant::Plant, zombie::Zombie};

const GRID_SIZE: f32 = 50.0; // 网格大小
const GRID_WIDTH: u32 = 9;   // 网格宽度
const GRID_HEIGHT: u32 = 5;  // 网格高度

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "植物大战僵尸 Demo".into(),
            width: GRID_SIZE * GRID_WIDTH as f32,
            height: GRID_SIZE * GRID_HEIGHT as f32,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(handle_input.system())
        .add_system(game_logic.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    // 添加相机
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    
    // 添加网格背景（可选）
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            commands.spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x as f32 * GRID_SIZE, y as f32 * GRID_SIZE, 0.0),
                    scale: Vec3::new(GRID_SIZE, GRID_SIZE, 1.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.5, 0.8, 0.5),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}