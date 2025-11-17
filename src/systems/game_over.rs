use bevy::prelude::*;
use crate::components::zombie::Zombie;
use crate::GRID_SIZE;
use crate::GRID_WIDTH;

#[derive(Resource)]
pub struct GameState {
    pub game_over: bool,
}

impl Default for GameState {
    fn default() -> Self {
        GameState { game_over: false }
    }
}

// 检测游戏是否结束（僵尸到达左边界）
pub fn check_game_over(
    mut game_state: ResMut<GameState>,
    zombies_query: Query<&Transform, With<Zombie>>,
) {
    // 在居中网格中，左边的边界应该是网格最左侧
    // 先将GRID_WIDTH转换为f32，再应用负号运算
    let left_boundary = -(GRID_WIDTH as f32) * GRID_SIZE / 2.0 - 50.0;
    
    for transform in zombies_query.iter() {
        if transform.translation.x < left_boundary {
            game_state.game_over = true;
            println!("游戏结束！僵尸突破了你的防线！");
            break;
        }
    }
}

// 显示游戏结束UI
pub fn display_game_over(
    game_state: Res<GameState>,
    mut query: Query<&mut Visibility, With<GameOverText>>,
) {
    if game_state.game_over {
        for mut visibility in query.iter_mut() {
            *visibility = Visibility::Visible;
        }
    }
}

#[derive(Component)]
pub struct GameOverText;
