use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub enum ZombieType {
    Normal,        // 普通僵尸
    ConeHead,      // 路障僵尸
    BucketHead,    // 铁桶僵尸
    FlagZombie,    // 旗帜僵尸
    NewspaperZombie, // 读报僵尸
}

#[derive(Component)]
pub struct Zombie {
    pub zombie_type: ZombieType,
    pub health: u32,
    pub speed: f32,
    pub attack_power: u32,
    pub attack_cooldown: f32,
    pub attack_timer: f32,
}

impl Zombie {
    pub fn new(zombie_type: ZombieType) -> Self {
        match zombie_type {
            ZombieType::Normal => Zombie {
                zombie_type,
                health: 200,
                speed: 20.0,
                attack_power: 100,
                attack_cooldown: 1.0,
                attack_timer: 0.0,
            },
            ZombieType::ConeHead => Zombie {
                zombie_type,
                health: 640,
                speed: 20.0,
                attack_power: 100,
                attack_cooldown: 1.0,
                attack_timer: 0.0,
            },
            ZombieType::BucketHead => Zombie {
                zombie_type,
                health: 1370,
                speed: 20.0,
                attack_power: 100,
                attack_cooldown: 1.0,
                attack_timer: 0.0,
            },
            ZombieType::FlagZombie => Zombie {
                zombie_type,
                health: 200,
                speed: 40.0,
                attack_power: 100,
                attack_cooldown: 1.0,
                attack_timer: 0.0,
            },
            ZombieType::NewspaperZombie => Zombie {
                zombie_type,
                health: 650,
                speed: 20.0,
                attack_power: 100,
                attack_cooldown: 1.0,
                attack_timer: 0.0,
            },
        }
    }

    pub fn get_color(&self) -> Color {
        match self.zombie_type {
            ZombieType::Normal => Color::rgb(0.5, 0.5, 0.5),
            ZombieType::ConeHead => Color::rgb(0.8, 0.6, 0.3),
            ZombieType::BucketHead => Color::rgb(0.6, 0.6, 0.6),
            ZombieType::FlagZombie => Color::rgb(1.0, 0.0, 0.5),
            ZombieType::NewspaperZombie => Color::rgb(0.9, 0.9, 0.9),
        }
    }
}