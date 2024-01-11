use graphics::math::Matrix2d;
use piston_window::G2d;

use crate::game::Game;
use crate::constants::GRAVITY;

pub trait Animatable {
    fn is_running(&self) -> bool;
    fn render(&mut self, ext_dt: f64, game: &mut Game, t_matrix: Matrix2d, gl: &mut G2d);
}

pub trait AnimationState {
    fn update(&mut self, ext_dt: f64) -> AnimationStatus;
}

#[derive(Debug)]
pub enum AnimationStatus {
    Running,
    Finished,
    Stopped,
}

#[derive(Debug)]
pub struct GravityFloorObject {
    pub position: (f64, f64),
    velocity: (f64, f64),
    floor: f64
}

impl GravityFloorObject {
    pub fn new(position: (f64, f64), velocity: (f64, f64), floor: f64) -> Self {
        Self { position, velocity, floor }
    }
}   

impl AnimationState for GravityFloorObject {
    fn update(&mut self, ext_dt: f64) -> AnimationStatus {
        self.velocity = (self.velocity.0, self.velocity.1 + GRAVITY * ext_dt);
        self.position = (self.position.0 + self.velocity.0, self.position.1 + self.velocity.1);
        
        if self.position.1 >= self.floor {
            self.position.1 = self.floor;
            return AnimationStatus::Finished;
        }

        AnimationStatus::Running
    }
}

pub struct Animation<T: AnimationState> {
    state: T,
    status: AnimationStatus,
    render_func: Box<dyn Fn(&T, Matrix2d, &mut G2d)>,
    finish_func: Box<dyn Fn(&mut Game)>
}

impl<T: AnimationState> Animation<T> {
    pub fn new<F, G>(state: T, render_func: F, finish_func: G) -> Self
        where
            F: 'static + Fn(&T, Matrix2d, &mut G2d),
            G: 'static + Fn(&mut Game)
    {
        Self {
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

    fn render(&mut self, ext_dt: f64, game: &mut Game, t_matrix: Matrix2d, gl: &mut G2d) {
        (self.render_func)(&self.state, t_matrix, gl);
        
        match self.status {
            AnimationStatus::Running => self.status = self.state.update(ext_dt),
            AnimationStatus::Finished => {
                (self.finish_func)(game);
                self.status = AnimationStatus::Stopped;
            }
            AnimationStatus::Stopped => ()
        }
    }

}
