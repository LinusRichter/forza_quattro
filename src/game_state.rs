use crate::player::Player;

#[derive(Debug)]
pub enum GameState {
    Starting,
    Running(Player),
    Win(Player),
    Draw,
}

impl GameState {
    pub fn initial() -> GameState {
        Self::Starting
    }
    pub fn next(&self) -> GameState {
        match self {
            GameState::Starting => GameState::Running(Player::Yellow),
            GameState::Running(p) => GameState::Running(p.op()),
            GameState::Win(_) | GameState::Draw => GameState::Starting
        }
    }
}
