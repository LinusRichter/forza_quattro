use opengl_graphics::OpenGL;
use piston::EventSettings;

pub const RENDERER: OpenGL = OpenGL::V4_5;
pub const COLUMNS: i32 = 7;
pub const ROWS: i32 = 6;
pub const GRAVITY: f64 = 981.0;

pub const EVENT_SETTINGS: EventSettings = piston_window::EventSettings {
        max_fps: 60,
        ups: 120,
        ups_reset: 2,
        swap_buffers: true,
        bench_mode: false,
        lazy: false,
    };

pub mod colors {
   pub const DARK_BLUE: [f32; 4] = [0.0, 0.1, 0.29, 1.0];
   pub const LIGHT_BLUE: [f32; 4] = [0.0, 0.17, 0.49, 1.0];
}
