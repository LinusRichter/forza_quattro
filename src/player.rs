use graphics::types::Color;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Player {
    Yellow,
    Red,
}

impl Player {
    pub fn op(&self) -> Player {
        match self {
            Player::Yellow => Player::Red,
            Player::Red => Player::Yellow
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Player::Yellow => graphics::color::YELLOW,
            Player::Red => graphics::color::RED
        }
    }

    pub fn shade_color(&self) -> Color {
        match self {
            Player::Yellow => [0.7, 0.7, 0.0, 1.0],
            Player::Red => [0.7, 0.0, 0.0, 1.0]
        }
    }

    pub fn text(&self) -> &str {
        match self {
            Player::Yellow => "Yellow",
            Player::Red => "Red",
        }
    }
}

