use bevy::prelude::*;
use crate::components::{zombie::Zombie, position::Position};

pub fn move_zombies(
    mut query: Query<(&mut Position, &Zombie)>,
    time: Res<Time>,
) {
    for (mut pos, zombie) in query.iter_mut() {
        pos.x = (pos.x as f32 - zombie.speed * time.delta_seconds()) as u32;
    }
}