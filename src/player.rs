use graphics::types::Color;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Copy)]
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

    pub fn text(&self) -> &str {
        match self {
            Player::Yellow => "Yellow",
            Player::Red => "Red",
        }
    }
}

