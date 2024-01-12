use crate::animation::AnimationStatus;
use crate::animation::animation_state::AnimationState;
use crate::constants::GRAVITY;

#[derive(Debug)]
pub struct GravityFloorState {
    pub position: (f64, f64),
    velocity: (f64, f64),
    floor: f64,
}

impl GravityFloorState {
    pub fn new(position: (f64, f64), velocity: (f64, f64), floor: f64) -> Self {
        Self { position, velocity, floor }
    }
}     

impl AnimationState for GravityFloorState {
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


