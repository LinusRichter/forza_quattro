use opengl_graphics::OpenGL;

pub const RENDERER: OpenGL = OpenGL::V4_5;
pub const COLUMNS: i32 = 7;
pub const ROWS: i32 = 6;
pub const GRAVITY: f64 = 981.0;

pub mod colors {
   pub const DARK_BLUE: [f32; 4] = [0.0, 0.1, 0.29, 1.0];
   pub const LIGHT_BLUE: [f32; 4] = [0.0, 0.17, 0.49, 1.0];
}
