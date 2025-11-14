use bevy::prelude::*;
use crate::GRID_SIZE;

pub fn handle_input(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    window_query: Query<&Window>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(window) = window_query.get_single().ok() {
            if let Some(cursor_position) = window.cursor_position() {
                let grid_x = (cursor_position.x / GRID_SIZE) as u32;
                let grid_y = (cursor_position.y / GRID_SIZE) as u32;

                commands.spawn(SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(grid_x as f32 * GRID_SIZE, grid_y as f32 * GRID_SIZE, 0.0),
                        scale: Vec3::new(GRID_SIZE, GRID_SIZE, 1.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::rgb(0.0, 1.0, 0.0),
                        ..default()
                    },
                    ..default()
                });
            }
        }
    }
}