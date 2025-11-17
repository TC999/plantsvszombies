use bevy::prelude::*;
use crate::components::zombie::Zombie;

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
    for transform in zombies_query.iter() {
        if transform.translation.x < -50.0 {
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
