use std::rc::Rc;

use crate::engine::d2::{
    animation::{AnimatedFloat, Ease, EaseFunction, Tween},
    Entity,
};

use super::Action;

/// An action that tweens an AnimatedFloat by a certain delta.
pub struct AnimateBy {
    tween: Rc<Tween>,

    value: AnimatedFloat,
    by: f32,
    seconds: f32,
}

impl AnimateBy {
    pub fn new(value: AnimatedFloat, by: f32, seconds: f32, easing: Option<EaseFunction>) -> Self {
        let easing = easing.unwrap_or(Rc::new(Ease::linear));
        let tween = Rc::new(Tween::new(value.get(), value.get() + by, seconds, Some(easing)));
        value.set_behavior(Some(tween.clone()));

        Self {
            value,
            by,
            seconds,
            tween,
        }
    }
}

impl Action for AnimateBy {
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
