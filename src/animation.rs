use graphics::math::Matrix2d;
use piston_window::G2d;

use crate::game::Game;

pub type RenderFunc = Box<dyn Fn(f64, Matrix2d, &mut G2d) -> AnimationState>;
pub type FinishFunc = Box<dyn Fn(&mut Game)>;

pub enum AnimationState {
    Running,
    Stopped,
}

pub struct Animation {
    state: AnimationState,
    render_func: RenderFunc,
    finish_func: FinishFunc,
}

impl Animation {
    pub fn new<F, G>(render_func: F, finish_func: G) -> Self
        where
            F: 'static + Fn(u32, Matrix2d, &mut G2d) -> AnimationState,
            G: 'static + Fn(&mut Game),
    {
        Self {
            state: AnimationState::Running,
            frame: 0,
            render_func: Box::new(render_func),
            finish_func: Box::new(finish_func),
        }
    }

    pub fn is_running(&self) -> bool {
        match self.state {
            AnimationState::Running => true,
            AnimationState::Stopped => false
        }
    }

    pub fn render(&mut self, game: &mut Game, t_matrix: Matrix2d, gl: &mut G2d) {
        self.state = (self.render_func)(self.frame, t_matrix, gl);
        match self.state {
            AnimationState::Running => self.frame += 1,
            AnimationState::Stopped => (self.finish_func)(game)
        }
    }
}
