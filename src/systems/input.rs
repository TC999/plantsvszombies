use bevy::prelude::*;
use crate::components::{plant::Plant, position::Position};

pub fn handle_input(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(window) = windows.get_primary() {
            if let Some(cursor_position) = window.cursor_position() {
                // 将鼠标位置转换为网格坐标
                let grid_x = (cursor_position.x / 50.0).floor() as u32;
                let grid_y = (cursor_position.y / 50.0).floor() as u32;

                // 在指定位置生成植物
                commands.spawn()
                    .insert(Plant { attack_power: 10 })
                    .insert(Position { x: grid_x, y: grid_y });
            }
        }
    }
}