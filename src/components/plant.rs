use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum PlantType {
    Peashooter,   // 豌豆射手
    Sunflower,    // 向日葵
    CherryBomb,   // 樱桃炸弹
    WallNut,      // 坚果墙
    PotatoMine,   // 土豆雷
    SnowPea,      // 寒冰射手
    Chomper,      // 大嘴花
    Repeater,     // 双发射手
}

#[derive(Component)]
pub struct Plant {
    pub plant_type: PlantType,
    pub health: u32,
    #[allow(dead_code)]
    pub attack_power: u32,
    pub attack_cooldown: f32,
    pub attack_timer: f32,
}

impl Plant {
    pub fn new(plant_type: PlantType) -> Self {
        match plant_type {
            PlantType::Peashooter => Plant {
                plant_type,
                health: 300,
                attack_power: 20,
                attack_cooldown: 1.4,
                attack_timer: 0.0,
            },
            PlantType::Sunflower => Plant {
                plant_type,
                health: 300,
                attack_power: 0,
                attack_cooldown: 24.0,
                attack_timer: 0.0,
            },
            PlantType::CherryBomb => Plant {
                plant_type,
                health: 300,
                attack_power: 1800,
                attack_cooldown: 0.0,
                attack_timer: 0.0,
            },
            PlantType::WallNut => Plant {
                plant_type,
                health: 4000,
                attack_power: 0,
                attack_cooldown: 0.0,
                attack_timer: 0.0,
            },
            PlantType::PotatoMine => Plant {
                plant_type,
                health: 300,
                attack_power: 1800,
                attack_cooldown: 14.0,
                attack_timer: 0.0,
            },
            PlantType::SnowPea => Plant {
                plant_type,
                health: 300,
                attack_power: 20,
                attack_cooldown: 1.4,
                attack_timer: 0.0,
            },
            PlantType::Chomper => Plant {
                plant_type,
                health: 300,
                attack_power: 9999,
                attack_cooldown: 42.0,
                attack_timer: 0.0,
            },
            PlantType::Repeater => Plant {
                plant_type,
                health: 300,
                attack_power: 20,
                attack_cooldown: 1.4,
                attack_timer: 0.0,
            },
        }
    }

    pub fn get_color(&self) -> Color {
        match self.plant_type {
            PlantType::Peashooter => Color::rgb(0.0, 1.0, 0.0),
            PlantType::Sunflower => Color::rgb(1.0, 1.0, 0.0),
            PlantType::CherryBomb => Color::rgb(1.0, 0.0, 0.0),
            PlantType::WallNut => Color::rgb(0.6, 0.4, 0.2),
            PlantType::PotatoMine => Color::rgb(0.5, 0.3, 0.1),
            PlantType::SnowPea => Color::rgb(0.0, 0.8, 1.0),
            PlantType::Chomper => Color::rgb(0.5, 0.0, 1.0),
            PlantType::Repeater => Color::rgb(0.0, 0.7, 0.0),
        }
    }

    pub fn get_cost(&self) -> u32 {
        match self.plant_type {
            PlantType::Peashooter => 100,
            PlantType::Sunflower => 50,
            PlantType::CherryBomb => 150,
            PlantType::WallNut => 50,
            PlantType::PotatoMine => 25,
            PlantType::SnowPea => 175,
            PlantType::Chomper => 150,
            PlantType::Repeater => 200,
        }
    }
}