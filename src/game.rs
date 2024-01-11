use crate::constants::{COLUMNS, ROWS};
use crate::game_state::GameState;
use crate::player::Player;

#[derive(Clone)]
pub struct Game {
    pub board: Vec<Vec<Option<Player>>>,
    pub state: GameState,
}

impl Game {
    pub fn initial() -> Game {
        Self {
            board: vec![vec![None; 6]; 7],
            state: GameState::initial(),
        }
    }

    pub fn update_state(&mut self) {
        for column in &self.board {
            let mut count = 0;
            let mut cell_owner = None;
            for cell in column {
                if *cell != None && *cell == cell_owner {
                    count += 1;
                    if count >= 4 {
                        self.state = GameState::Win(cell.clone().unwrap());
                        return;
                    }
                } else {
                    count = 1;
                    cell_owner = (*cell).clone();
                }
            }
        }

        for row in 0..ROWS {
            let mut count = 0;
            let mut cell_owner = None;
            for col in 0..COLUMNS {
                let cell = &self.board[col as usize][row as usize];

                if *cell != None && *cell == cell_owner {
                    count += 1;
                    if count >= 4 {
                        self.state = GameState::Win(cell.clone().unwrap());
                        return;
                    }
                } else {
                    count = 1;
                    cell_owner = (*cell).clone();
                }
            }
        }

        for start_col in 0..COLUMNS {
            let mut count = 0;
            let mut cell_owner = None;
            let mut col = start_col;
            let mut row = 0;

            while col < COLUMNS && row < ROWS {
                let cell = &self.board[col as usize][row as usize];

                if *cell != None && *cell == cell_owner {
                    count += 1;
                    if count >= 4 {
                        self.state = GameState::Win(cell.clone().unwrap());
                        return;
                    }
                } else {
                    count = 1;
                    cell_owner = (*cell).clone();
                }

                col += 1;
                row += 1;
            }
        }

        for start_col in (0..COLUMNS).rev() {
            let mut count = 0;
            let mut cell_owner = None;
            let mut col = start_col;
            let mut row = 0;

            while col < COLUMNS && row < ROWS {
                let cell = &self.board[col as usize][row as usize];
                if *cell != None && *cell == cell_owner {
                    count += 1;
                    if count >= 4 {
                        self.state = GameState::Win(cell.clone().unwrap());
                        return;
                    }
                } else {
                    count = 1;
                    cell_owner = (*cell).clone();
                }

                if col > 0 {
                    col -= 1;
                }
                row += 1;
            }
        }

        for col in &self.board {
            for cell in col {
                if let None = cell {
                    if let GameState::Running(cur_player) = &self.state {
                        self.state = GameState::Running(cur_player.op());
                        return;
                    }
                }
            }
        }

        self.state = GameState::Draw;
    }
}
