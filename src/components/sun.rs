use bevy::prelude::*;

#[derive(Component)]
pub struct Sun {
    pub value: u32,
    pub lifetime: f32,
}

impl Sun {
    pub fn new() -> Self {
        Sun {
            value: 25,
            lifetime: 10.0,
        }
    }
}

// 阳光资源
#[derive(Resource)]
pub struct SunResource {
    pub amount: u32,
}

impl Default for SunResource {
    fn default() -> Self {
        SunResource { amount: 150 }
    }
}
