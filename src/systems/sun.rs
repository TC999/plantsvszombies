use bevy::prelude::*;
use crate::components::{sun::{Sun, SunResource}, plant::{Plant, PlantType}};
use crate::GRID_SIZE;

// 向日葵生成阳光系统
pub fn sunflower_produce_sun(
    mut commands: Commands,
    mut sunflowers_query: Query<(&mut Plant, &Transform), With<Plant>>,
    time: Res<Time>,
) {
    for (mut plant, transform) in sunflowers_query.iter_mut() {
        if !matches!(plant.plant_type, PlantType::Sunflower) {
            continue;
        }

        plant.attack_timer += time.delta_seconds();

        if plant.attack_timer >= plant.attack_cooldown {
            // 生成阳光
            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(
                            transform.translation.x,
                            transform.translation.y + GRID_SIZE,
                            2.0,
                        ),
                        scale: Vec3::new(20.0, 20.0, 1.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::rgb(1.0, 1.0, 0.0),
                        ..default()
                    },
                    ..default()
                },
                Sun::new(),
            ));

            plant.attack_timer = 0.0;
        }
    }
}

// 阳光消失系统
pub fn sun_lifetime(
    mut commands: Commands,
    mut suns_query: Query<(Entity, &mut Sun)>,
    time: Res<Time>,
) {
    for (entity, mut sun) in suns_query.iter_mut() {
        sun.lifetime -= time.delta_seconds();

        if sun.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

// 收集阳光系统
pub fn collect_sun(
    mut commands: Commands,
    mut sun_resource: ResMut<SunResource>,
    suns_query: Query<(Entity, &Transform, &Sun)>,
    buttons: Res<Input<MouseButton>>,
    window_query: Query<&Window>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(window) = window_query.get_single().ok() {
            if let Some(cursor_position) = window.cursor_position() {
                for (entity, transform, sun) in suns_query.iter() {
                    let sun_pos = Vec2::new(transform.translation.x, transform.translation.y);
                    let distance = cursor_position.distance(sun_pos);

                    if distance < 30.0 {
                        sun_resource.amount += sun.value;
                        commands.entity(entity).despawn();
                        break;
                    }
                }
            }
        }
    }
}
