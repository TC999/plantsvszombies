use bevy::prelude::*;

// 网格位置标记组件
#[derive(Component)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}