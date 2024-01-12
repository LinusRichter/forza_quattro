use graphics::types::Matrix2d;
use piston_window::G2d;

use crate::game::Game;

pub trait Animatable {
    fn is_running(&self) -> bool;
    fn update(&mut self, game: &mut Game, ext_dt: f64);
    fn render(&mut self, t_matrix: Matrix2d, gl: &mut G2d);
}

