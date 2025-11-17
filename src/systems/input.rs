use bevy::prelude::*;
use crate::components::{plant::{Plant, PlantType}, position::Position, sun::SunResource};
use crate::{GRID_SIZE, GRID_WIDTH, GRID_HEIGHT};

#[derive(Resource)]
pub struct SelectedPlant {
    pub plant_type: Option<PlantType>,
}

impl Default for SelectedPlant {
    fn default() -> Self {
        SelectedPlant {
            plant_type: Some(PlantType::Peashooter),
        }
    }
}

pub fn handle_input(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    mut sun_resource: ResMut<SunResource>,
    mut selected_plant: ResMut<SelectedPlant>,
    window_query: Query<&Window>,
    plants_query: Query<&Position, With<Plant>>,
) {
    // 键盘选择植物
    if keys.just_pressed(KeyCode::Key1) {
        selected_plant.plant_type = Some(PlantType::Peashooter);
    } else if keys.just_pressed(KeyCode::Key2) {
        selected_plant.plant_type = Some(PlantType::Sunflower);
    } else if keys.just_pressed(KeyCode::Key3) {
        selected_plant.plant_type = Some(PlantType::WallNut);
    } else if keys.just_pressed(KeyCode::Key4) {
        selected_plant.plant_type = Some(PlantType::SnowPea);
    } else if keys.just_pressed(KeyCode::Key5) {
        selected_plant.plant_type = Some(PlantType::Repeater);
    }

    // 鼠标放置植物
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(plant_type) = selected_plant.plant_type {
            if let Some(window) = window_query.get_single().ok() {
                if let Some(cursor_position) = window.cursor_position() {
                    let grid_x = (cursor_position.x / GRID_SIZE) as u32;
                    let grid_y = (cursor_position.y / GRID_SIZE) as u32;

                    // 检查是否在网格内
                    if grid_x >= GRID_WIDTH || grid_y >= GRID_HEIGHT {
                        return;
                    }

                    // 检查该位置是否已有植物
                    let has_plant = plants_query.iter().any(|pos| pos.x == grid_x && pos.y == grid_y);

                    if has_plant {
                        return;
                    }

                    let plant = Plant::new(plant_type);
                    let cost = plant.get_cost();

                    // 检查是否有足够的阳光
                    if sun_resource.amount >= cost {
                        sun_resource.amount -= cost;

                        let color = plant.get_color();

                        commands.spawn((
                            SpriteBundle {
                                transform: Transform {
                                    translation: Vec3::new(
                                        grid_x as f32 * GRID_SIZE,
                                        grid_y as f32 * GRID_SIZE,
                                        1.0,
                                    ),
                                    scale: Vec3::new(GRID_SIZE * 0.8, GRID_SIZE * 0.8, 1.0),
                                    ..default()
                                },
                                sprite: Sprite {
                                    color,
                                    ..default()
                                },
                                ..default()
                            },
                            plant,
                            Position { x: grid_x, y: grid_y },
                        ));
                    }
                }
            }
        }
    }
}
