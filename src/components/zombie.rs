use bevy::prelude::*;

#[derive(Component)]
pub struct Zombie {
    pub health: u32,
    pub speed: f32,
}