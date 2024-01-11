use graphics::math::Matrix2d;
use piston_window::G2d;

use crate::game::Game;
use crate::constants::GRAVITY;

pub trait Animatable {
    fn is_running(&self) -> bool;
    fn update(&mut self, game: &mut Game, ext_dt: f64);
    fn render(&mut self, t_matrix: Matrix2d, gl: &mut G2d);
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
    fn update(&mut self, dt: f64) -> AnimationStatus {
        self.velocity = (self.velocity.0, self.velocity.1 + GRAVITY * dt);
        self.position = (self.position.0, self.position.1 + self.velocity.1 * dt);
        
        if self.position.1 >= self.floor {
            self.position.1 = self.floor;
            return AnimationStatus::Finished;
        }

        AnimationStatus::Running
    }
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
