use graphics::types::Matrix2d;
use piston_window::G2d;

use crate::game::Game;

use self::{animatable::Animatable, animation_state::AnimationState};

pub mod animatable;
pub mod animation_state;

#[derive(Debug)]
pub enum AnimationStatus {
    Running,
    Finished,
    Stopped,
}

pub struct Animation<T: AnimationState> {
    delay: f64,
    state: T,
    status: AnimationStatus,
    render_func: Box<dyn Fn(&T, Matrix2d, &mut G2d)>,
    finish_func: Box<dyn Fn(&mut Game)>
}

impl<T: AnimationState> Animation<T> {
    pub fn new<F, G>(delay: f64, state: T, render_func: F, finish_func: G) -> Self
        where
            F: 'static + Fn(&T, Matrix2d, &mut G2d),
            G: 'static + Fn(&mut Game)
    {
        Self {
            delay,
            state, 
            status: AnimationStatus::Running,
            render_func: Box::new(render_func),
            finish_func: Box::new(finish_func),
        }
    }
}

impl<T: AnimationState + 'static> Animatable for Animation<T> {
    fn is_running(&self) -> bool {
        match self.status {
            AnimationStatus::Running | AnimationStatus::Finished => true,
            AnimationStatus::Stopped => false
        }
    }
    
    fn update(&mut self, game: &mut Game, ext_dt: f64) {
        match self.status {
            AnimationStatus::Running => {
                if self.delay > 0.0 {
                    self.delay -= ext_dt;
                }else{
                    self.status = self.state.update(ext_dt)
                }
            },
            AnimationStatus::Finished => {
                (self.finish_func)(game);
                self.status = AnimationStatus::Stopped;
            }
            AnimationStatus::Stopped => ()
        }
    }

    fn render(&mut self, t_matrix: Matrix2d, gl: &mut G2d) {
        (self.render_func)(&self.state, t_matrix, gl); 
    }

}
