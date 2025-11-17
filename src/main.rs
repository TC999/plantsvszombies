use bevy::prelude::*;

mod systems;       // 包含所有游戏逻辑系统
mod components;    // 包含所有 ECS 组件

use systems::{
    game::game_logic, 
    input::{handle_input, SelectedPlant}, 
    movement::move_zombies,
    shooting::plant_shooting,
    projectile::move_projectiles,
    collision::projectile_collision,
    sun::{sunflower_produce_sun, sun_lifetime, collect_sun},
    spawning::{spawn_zombies, ZombieSpawnTimer},
};
use components::sun::SunResource;

const GRID_SIZE: f32 = 80.0; // 网格大小
const GRID_WIDTH: u32 = 9;   // 网格宽度
const GRID_HEIGHT: u32 = 5;  // 网格高度

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "植物大战僵尸 Rust Demo".to_string(),
                resolution: (GRID_WIDTH as f32 * GRID_SIZE, GRID_HEIGHT as f32 * GRID_SIZE).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<SunResource>()
        .init_resource::<SelectedPlant>()
        .init_resource::<ZombieSpawnTimer>()
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .add_systems(Update, game_logic)
        .add_systems(Update, move_zombies)
        .add_systems(Update, plant_shooting)
        .add_systems(Update, move_projectiles)
        .add_systems(Update, projectile_collision)
        .add_systems(Update, sunflower_produce_sun)
        .add_systems(Update, sun_lifetime)
        .add_systems(Update, collect_sun)
        .add_systems(Update, spawn_zombies)
        .add_systems(Update, ui_system)
        .run();
}

// 初始化系统
fn setup(mut commands: Commands) {
    // 添加 2D 摄像机
    commands.spawn(Camera2dBundle::default());

    // 添加草坪网格背景
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(
                            x as f32 * GRID_SIZE + GRID_SIZE / 2.0,
                            y as f32 * GRID_SIZE + GRID_SIZE / 2.0,
                            0.0,
                        ),
                        scale: Vec3::new(GRID_SIZE - 2.0, GRID_SIZE - 2.0, 1.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::rgb(0.3, 0.6, 0.3),
                        ..default()
                    },
                    ..default()
                },
                components::position::Position { x, y },
            ));
        }
    }

    // 添加UI文本
    commands.spawn((
        TextBundle::from_section(
            "Sun: 150",
            TextStyle {
                font_size: 30.0,
                color: Color::rgb(1.0, 1.0, 0.0),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        SunCounter,
    ));

    // 添加植物选择UI
    commands.spawn(
        TextBundle::from_section(
            "1: Peashooter (100)  2: Sunflower (50)  3: WallNut (50)  4: SnowPea (175)  5: Repeater (200)",
            TextStyle {
                font_size: 16.0,
                color: Color::rgb(1.0, 1.0, 1.0),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}

#[derive(Component)]
struct SunCounter;

// UI更新系统
fn ui_system(
    sun_resource: Res<SunResource>,
    mut query: Query<&mut Text, With<SunCounter>>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Sun: {}", sun_resource.amount);
    }
}
