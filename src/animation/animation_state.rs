use super::AnimationStatus;

pub trait AnimationState {
    fn update(&mut self, ext_dt: f64) -> AnimationStatus;
}

