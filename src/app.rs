use gfx_device_gl::Device;
use graphics::Context;
use piston::{RenderArgs, UpdateArgs};
use piston_window::{G2d, Glyphs, G2dTexture};

use crate::animation::animatable::Animatable;
use crate::constants::colors::{DARK_BLUE, LIGHT_BLUE};
use crate::player::Player;
use crate::{Pos, Size};
use crate::animation::Animation;
use crate::constants::{COLUMNS, ROWS};
use crate::game::Game;
use crate::game_state::GameState;
use crate::gravity_floor_state::GravityFloorState;

pub struct App {
    game: Game,
    window_size: Size,
    font: Glyphs,
    tile: G2dTexture,
    tile_bg: G2dTexture,
    coin_red: G2dTexture,
    coin_yellow: G2dTexture,
    arrow_yellow: G2dTexture,
    arrow_red: G2dTexture,
    mouse_pos: Pos,
    animations: Vec<Box<dyn Animatable>>,
}

impl App {
    pub fn initial(font: Glyphs, tile: G2dTexture, tile_bg: G2dTexture, coin_yellow: G2dTexture, coin_red: G2dTexture, arrow_yellow: G2dTexture, arrow_red: G2dTexture) -> App {
        Self {
            game: Game::initial(),
            window_size: (0.0, 0.0),
            mouse_pos: (0.0, 0.0),
            font,
            tile,
            tile_bg,
            coin_yellow,
            coin_red,
            arrow_yellow,
            arrow_red,
            animations: vec![],
        }
    }

    pub fn render(&mut self,
                  args: &RenderArgs,
                  c: Context,
                  gl: &mut G2d,
                  d: &mut Device) {
        use graphics::*;
        
        clear(color::GRAY, gl);

        self.window_size = (args.window_size[0], args.window_size[1]);

        let ((offset_x, offset_y), (board_size, _)) = self.get_dimensions();

        let col_width = board_size / COLUMNS as f64;

        let hover_column = self.get_mouse_column();

        let t_matrix = c.transform.trans(offset_x, offset_y);
        
        for col in 0..COLUMNS {
            let x = col as f64 * col_width;
            for row in 0..ROWS {
                let y = board_size - row as f64 * col_width - col_width;
                
                image(&self.tile_bg, t_matrix.trans(x, y).scale(col_width / 400.0, col_width / 400.0), gl);
            }
        }

        // Animate'em
        self.animations.iter_mut().for_each(|animation| {
            animation.render(t_matrix, gl);
        }); 

        rectangle(LIGHT_BLUE, [0.0, 0.0, board_size, col_width], t_matrix, gl);
        
        for col in 0..COLUMNS {
            let x = col as f64 * col_width;
 
            for row in 0..ROWS {
                let y = board_size - row as f64 * col_width - col_width;
                
                if let Some(player) = &self.game.board[col as usize][row as usize] {
                    match player {
                        Player::Yellow => {
                            image(&self.coin_yellow, t_matrix.trans(x, y).scale(col_width / 400.0, col_width / 400.0), gl);
                        }

                        Player::Red => {
                            image(&self.coin_red, t_matrix.trans(x, y).scale(col_width / 400.0, col_width / 400.0), gl);
                        }
                    }
                } 
                

                image(&self.tile, t_matrix.trans(x, y).scale(col_width / 400.0, col_width / 400.0), gl);
            }
            
              

            if let GameState::Running(player) = &self.game.state {
                if let Some(col) = hover_column {
                    let s_matrix = t_matrix
                        .clone()
                        .trans(col as f64 * col_width, 0.0)
                        .scale(col_width / 400.0, col_width / 400.0);
                    
                    match player {
                        Player::Yellow => {
                            image(&self.arrow_yellow, s_matrix, gl);
                        }
                        Player::Red => {
                            image(&self.arrow_red, s_matrix, gl);
                        }
                    }

                }
            } 
            
            // Status bar
            let bar_height = col_width / 1.7;
            let bar_width = board_size;
            let font_size = bar_height / 2.0;

            rectangle(color::WHITE, [0.0, 0.0, board_size, bar_height], t_matrix, gl);
            line_from_to(graphics::color::BLACK, 2.0, [0.0, bar_height], [board_size, bar_height], t_matrix, gl);

            let text: String = match &self.game.state {
                GameState::Starting => {
                    String::from("Forza quattro! Click anywhere")
                }
                GameState::Running(player) => {
                    let p_text = player.text();
                    format!("{p_text}s turn! Click to place")
                }
                GameState::Win(player) => {
                    let p_text = player.text();
                    format!("{p_text} wins! Click to reset")
                }
                GameState::Draw => {
                    String::from("It's a draw! Click to reset")
                }
            };

            text::Text::new_color(color::BLACK, (bar_height * 0.5) as u32)
                .draw(&text,
                      &mut self.font,
                      &c.draw_state,
                      t_matrix.trans(bar_width * 0.02, bar_height * 0.5 + font_size / 3.0),
                      gl).unwrap();

            self.font.factory.encoder.flush(d);
        }
    }

    pub fn handle_click(&mut self) {
        let (_, (board_size, _)) = self.get_dimensions();
        let col_width = board_size / COLUMNS as f64;

        if !self.animations.is_empty() { return; };
        
        

        match self.game.state.clone() {
            GameState::Starting => {
                self.game.state = self.game.state.next();
            }

            GameState::Running(player) => {
                let coin_yellow = self.coin_yellow.clone();
                let coin_red = self.coin_red.clone();
                
                if let Some(col) = self.get_mouse_column() {
                    let x = col as f64 * col_width;
                    let player_clone = player.clone();

                    for (row, cell) in self.game.board[col].iter_mut().enumerate() {
                        let y = board_size - row as f64 * col_width - col_width;
                        
                        

                        if cell.is_none() {
                            self.animations.push(
                                Box::new(
                                    Animation::new(
                                        0.0,
                                        GravityFloorState::new((x, 0.0), (0.0, 3.0), y),
                                        move |state, t_matrix, gl| {
                                            use graphics::*;
                                            
                                            match player_clone {
                                                Player::Yellow => {
                                                    image(&coin_yellow,
                                                          t_matrix.trans(state.position.0, state.position.1).scale(col_width / 400.0, col_width / 400.0), gl);
                                                }

                                                Player::Red => {
                                                    image(&coin_red, t_matrix.trans(state.position.0, state.position.1).scale(col_width / 400.0, col_width / 400.0), gl);
                                                }
                                            }
                                        },
                                        move |game: &mut Game| {
                                            game.board[col][row] = Some(player_clone.clone());
                                            game.update_state();
                                        })));
                            return;
                        }
                    }
                } else {
                    return;
                }
            }

            GameState::Win(_) | GameState::Draw => {
                let cloned_board = self.game.board.clone();

                for col_i in 0..COLUMNS {
                    let x = col_i as f64 * col_width;

                    for (row_i, &cell) in cloned_board[col_i as usize].iter().enumerate() {
                        let y = board_size - row_i as f64 * col_width - col_width;
 
                        match cell {
                            Some(player) => {
                                let coin_yellow = self.coin_yellow.clone();
                                let coin_red = self.coin_red.clone();
                                self.animations.push(
                                    Box::new(
                                        Animation::new(
                                            row_i as f64 / (ROWS * 2) as f64,
                                            GravityFloorState::new((x, y), (0.0, 0.0), board_size + col_width),
                                            move |state, t_matrix, gl| {
                                                use graphics::*;

                                                match player {
                                                    Player::Yellow => {
                                                        image(&coin_yellow,
                                                              t_matrix.trans(state.position.0, state.position.1).scale(col_width / 400.0, col_width / 400.0), gl);
                                                    }

                                                    Player::Red => {
                                                        image(&coin_red, t_matrix.trans(state.position.0, state.position.1).scale(col_width / 400.0, col_width / 400.0), gl);
                                                    }
                                                }
                                            },
                                            move |_game: &mut Game| {
                                            })));

                            }
                            None => ()
                        }
                    }
                }

                self.reset();
            },
        }
    }
    
    pub fn update(&mut self, args: &UpdateArgs) {
        self.animations.iter_mut().for_each(|animation| {
            animation.update(&mut self.game, args.dt);
        });

        self.animations.retain(|animation| animation.is_running());    
    }

    pub fn reset(&mut self) {
        self.game = Game {
            board: vec![vec![None; 6]; 7],
            state: GameState::Starting,
        };
    }

    pub fn set_mouse_pos(&mut self, pos: Pos) {
        self.mouse_pos = pos;
    }

    fn get_dimensions(&self) -> (Pos, Size) {
        let (w, h) = self.window_size;
        let board_size: f64 = w.min(h);
        let offset_x = (w - board_size) / 2.0;
        let offset_y = (h - board_size) / 2.0;

        ((offset_x, offset_y), (board_size, board_size))
    }


    fn get_mouse_column(&self) -> Option<usize> {
        let ((ox, oy), (w, _h)) = self.get_dimensions();
        let pos = (self.mouse_pos.0 - ox, self.mouse_pos.1 - oy);

        let column_width = w / COLUMNS as f64;
        let column_index = pos.0 / column_width;

        if column_index < 0.0 || column_index >= COLUMNS as f64 {
            return None;
        }

        Some(column_index as usize)
    }
}

