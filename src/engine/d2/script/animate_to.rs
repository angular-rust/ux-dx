use std::rc::Rc;

use crate::engine::d2::{
    animation::{AnimatedFloat, Ease, EaseFunction, Tween},
    Entity,
};

use super::Action;

/// An action that tweens an AnimatedFloat to a certain value.
pub struct AnimateTo {
    tween: Rc<Tween>,

    value: AnimatedFloat,
    to: f32,
    seconds: f32,
}

impl AnimateTo {
    pub fn new(value: AnimatedFloat, to: f32, seconds: f32, easing: Option<EaseFunction>) -> Self {
        let easing = easing.unwrap_or(Rc::new(Ease::linear));
        let tween = Rc::new(Tween::new(value.get(), to, seconds, Some(easing)));
        value.set_behavior(Some(tween.clone()));

        Self {
            tween,
            value,
            to,
            seconds,
        }
    }
}

impl Action for AnimateTo {
    fn update(&self, dt: f32, actor: &mut Entity) -> f32 {
        self.value.update(dt); // Fake an update to account for this frame
        let overtime = self.tween.elapsed() - self.seconds;
        if overtime > 0.0 {
            0_f32.max(dt - overtime)
        } else {
            0.0
        }
    }
}
