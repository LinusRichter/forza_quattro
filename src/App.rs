use opengl_graphics::GlGraphics;

use crate::{Game, Size, Pos};

pub struct App {
    gl: GlGraphics,
    game: Game,
    window_size: Size,
    mouse_pos: Pos
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GRAY: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.17, 0.49, 1.0];
        const DARK_BLUE: [f32; 4] = [0.0, 0.0, 0.0, 0.2];
        const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        self.window_size = (args.window_size[0], args.window_size[1]);

        let ((offset_x, offset_y), (board_size, _)) = self.get_dimensions();

        let col_width = board_size / COLUMNS as f64;


        self.gl.draw(args.viewport(), |c, gl| {
            clear(GRAY, gl);

            let t_matrix = c.transform.trans(offset_x, offset_y);

            rectangle(BLUE, [0.0, 0.0, board_size, board_size], t_matrix, gl);

            for col in 0..COLUMNS {
                let x = col as f64 * col_width;

                if col % 2 == 0 {
                    rectangle(DARK_BLUE, [x, 0.0, col_width, board_size], t_matrix, gl);
                }

                for row in 0..ROWS {
                    let y = board_size - row as f64 * col_width - col_width;

                    if let Some(player) = &self.game.board[col as usize][row as usize] {
                        let player_color = if *player == Player::Yellow { YELLOW } else { RED };
                        ellipse(player_color, [x, y, col_width, col_width], t_matrix, gl);
                    } else {
                        ellipse(DARK_BLUE, [x, y, col_width, col_width], t_matrix, gl);
                    }
                }
            }
        });
    }

    //fn update(&mut self, args: &UpdateArgs) {}

    fn get_dimensions(&self) -> (Pos, Size) {
        let (w, h) = self.window_size;
        let board_size: f64 = w.min(h);
        let offset_x = (w - board_size) / 2.0;
        let offset_y = (h - board_size) / 2.0;

        ((offset_x, offset_y), (board_size, board_size))
    }

    fn handle_win(&mut self) {
        for column in &self.game.board {
            let mut count = 0;
            let mut cell_owner = None;
            for cell in column {
                if *cell != None && *cell == cell_owner {
                    count += 1;
                    if count >= 4 {
                        self.game.state = GameState::Win((*cell).clone().unwrap());
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
                let cell = &self.game.board[col as usize][row as usize];

                if *cell != None && *cell == cell_owner {
                    count += 1;
                    if count >= 4 {
                        self.game.state = GameState::Win((*cell).clone().unwrap());
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
                let cell = &self.game.board[col as usize][row as usize];
                if *cell != None && *cell == cell_owner {
                    count += 1;
                    if count >= 4 {
                        self.game.state = GameState::Win(cell.clone().unwrap());
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
                let cell = &self.game.board[col as usize][row as usize];
                if *cell != None && *cell == cell_owner {
                    count += 1;
                    if count >= 4 {
                        self.game.state = GameState::Win(cell.clone().unwrap());
                        return; // Exit early on win
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
    }

    fn handle_click(&mut self) {
        if let GameState::Running(player) = &self.game.state {
            let ((ox, oy), (w, _h)) = self.get_dimensions();
            let pos = (self.mouse_pos.0 - ox, self.mouse_pos.1 - oy);
            
            let column_width = w / COLUMNS as f64;
            let column_index = pos.0 / column_width;

            if column_index < 0.0 || column_index >= COLUMNS as f64 { return; }

            let column_index = column_index as usize;

            for cell in self.game.board[column_index].iter_mut() {
                match cell {
                    None => {
                        *cell = Some(player.clone());
                        self.game.state = GameState::Running(player.op());
                        self.handle_win();
                        return;
                    }
                    Some(_) => {}
                }
            }
        }
    }
}

