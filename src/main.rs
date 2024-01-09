extern crate opengl_graphics;
extern crate piston;

use gfx_device_gl::Device;
use graphics::Context;
use graphics::types::Color;
use opengl_graphics::OpenGL;
use piston::{Button, ButtonArgs, Input, Loop, ButtonState, Motion, MouseButton};
use piston::input::RenderArgs;
use piston::window::WindowSettings;
use piston_window::{PistonWindow, Glyphs, G2d, Event};

const RENDERER: OpenGL = OpenGL::V3_2;
const COLUMNS: i32 = 7;
const ROWS: i32 = 6;

type Pos = (f64, f64);
type Size = (f64, f64);

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
enum Player {
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

#[derive(Debug)]
enum GameState {
    Starting,
    Running(Player),
    Win(Player),
    Draw,
}

pub struct Game {
    board: Vec<Vec<Option<Player>>>,
    state: GameState,
}

pub struct App {
    game: Game,
    window_size: Size,
    mouse_pos: Pos,
    font: Glyphs
}

impl App {
    fn render(&mut self,
              args: &RenderArgs,
              c: Context, 
              gl: &mut G2d,
              d: &mut Device) {
        use graphics::*;

        const SHADE: [f32; 4] = [0.0, 0.0, 0.0, 0.2];

        self.window_size = (args.window_size[0], args.window_size[1]);

        let ((offset_x, offset_y), (board_size, _)) = self.get_dimensions();

        let col_width = board_size / COLUMNS as f64;
        
        let hover_column = self.get_mouse_column();
        
        clear(color::GRAY, gl);

        let t_matrix = c.transform.trans(offset_x, offset_y);

        rectangle(color::BLUE, [0.0, 0.0, board_size, board_size], t_matrix, gl);

        for col in 0..COLUMNS {
            let x = col as f64 * col_width;

            if col % 2 == 0 {
                rectangle(SHADE, [x, 0.0, col_width, board_size], t_matrix, gl);
            }
            
            if let GameState::Running(player) = &self.game.state {
                for row in 0..ROWS {
                    let y = board_size - row as f64 * col_width - col_width;

                    if let Some(player) = &self.game.board[col as usize][row as usize] {
                        ellipse(player.color(), [x, y, col_width, col_width], t_matrix, gl);
                    } else {
                        ellipse(SHADE, [x, y, col_width, col_width], t_matrix, gl);
                    }
                }
                
                if let Some(col) = hover_column {
                        let rotated_matrix = t_matrix
                                            .clone()
                                            .trans(col as f64 * col_width + col_width / 2.0, col_width / 3.0)
                                            .rot_deg(45.0);
                        
                        rectangle(player.color(), [0.0, 0.0, col_width / 2.0, col_width / 2.0], rotated_matrix, gl);
                }
            }
            
            let bar_height = col_width / 1.5;
            let bar_width  = board_size;
            let font_size  = bar_height / 2.0;

            rectangle(color::WHITE, [0.0, 0.0, board_size, bar_height], t_matrix, gl);
            line_from_to(graphics::color::BLACK, 2.0, [0.0, bar_height], [board_size, bar_height], t_matrix, gl);
            
            let text: String = match &self.game.state {
                GameState::Starting => {
                    String::from("Four wins! Click anywhere")  
                },
                GameState::Running(player) => {
                    let p_text = player.text();
                    format!("{p_text}s turn! Click to place")
                },
                GameState::Win(player) => {
                    let p_text = player.text();
                    format!("{p_text} wins! Click to reset")
                },
                GameState::Draw => {
                    String::from("It's a draw! Click to reset")
                }
            };
            
            text::Text::new_color(color::BLACK, (bar_height * 0.5) as u32)
                .draw(&text,
                      &mut self.font,
                      &c.draw_state,
                      t_matrix.trans(bar_width*0.02, bar_height*0.5 + font_size / 3.0),
                      gl).unwrap();

            self.font.factory.encoder.flush(d);
        }
    }

    fn get_dimensions(&self) -> (Pos, Size) {
        let (w, h) = self.window_size;
        let board_size: f64 = w.min(h);
        let offset_x = (w - board_size) / 2.0;
        let offset_y = (h - board_size) / 2.0;

        ((offset_x, offset_y), (board_size, board_size))
    }

    fn handle_win(&mut self) -> GameState {
        for column in &self.game.board {
            let mut count = 0;
            let mut cell_owner = None;
            for cell in column {
                if *cell != None && *cell == cell_owner {
                    count += 1;
                    if count >= 4 {
                        return GameState::Win(cell.clone().unwrap());
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
                        return GameState::Win(cell.clone().unwrap());
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
                        return GameState::Win(cell.clone().unwrap());
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
                        return GameState::Win(cell.clone().unwrap());
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
        
        for col in &self.game.board {
            for cell in col {
                if let None = cell {
                    if let GameState::Running(cur_player) = &self.game.state {
                        return GameState::Running(cur_player.op());
                    }
                }
            }
        }

        GameState::Draw
    }

    fn handle_click(&mut self) {
        match &self.game.state {
            GameState::Starting => {
                self.game.state = GameState::Running(Player::Yellow);
            },

            GameState::Running(player) => {
                match self.get_mouse_column() {
                    Some(column_index) => {
                        for cell in self.game.board[column_index].iter_mut() {
                            if let None = cell {
                                *cell = Some(player.clone());
                                self.game.state = self.handle_win();
                                return;
                            }
                        }
                    },
                    None => return,
                }

            },

            GameState::Win(_) | GameState::Draw => self.reset(),
        }
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
    
    fn reset(&mut self) {
        self.game = Game {
            board: vec![vec![None; 6]; 7],
            state: GameState::Starting,
        };
    }

    fn initial(font: Glyphs) -> App {
        App {
            game: Game {
                board: vec![vec![None; 6]; 7],
                state: GameState::Starting,
            },
            window_size: (0.0, 0.0),
            mouse_pos: (0.0, 0.0),
            font 
        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Four Wins", [800, 800])
        .graphics_api(RENDERER)
        .samples(4)
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let assets = find_folder::Search::ParentsThenKids(3, 3)
                .for_folder("assets")
                .unwrap();
    
    let glyphs = window.load_font(assets.join("RobotoMono-Regular.ttf")).unwrap();
    
    let mut app = App::initial(glyphs);

    while let Some(e) = window.next() {
        match e {
            Event::Loop(Loop::Render(args)) => {
                window.draw_2d(&e, |c, g, d| {
                    app.render(&args, c, g, d);
                });
            },

            Event::Input(
                Input::Button(
                    ButtonArgs {
                        state: ButtonState::Press,
                        button: Button::Mouse(MouseButton::Left),
                        ..
                    }), _) => {
                app.handle_click();
            },

            Event::Input(Input::Move(Motion::MouseCursor([x, y])), _) => {
                app.mouse_pos = (x, y);
            },

            _ => ()
        };
    }
}
