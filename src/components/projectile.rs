use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub damage: u32,
    pub speed: f32,
    pub is_frozen: bool, // 是否是冰冻豌豆
}

impl Projectile {
    pub fn new_pea() -> Self {
        Projectile {
            damage: 20,
            speed: 200.0,
            is_frozen: false,
        }
    }

    pub fn new_frozen_pea() -> Self {
        Projectile {
            damage: 20,
            speed: 200.0,
            is_frozen: true,
        }
    }

    pub fn get_color(&self) -> Color {
        if self.is_frozen {
            Color::rgb(0.5, 0.8, 1.0)
        } else {
            Color::rgb(0.0, 0.8, 0.0)
        }
    }
}
