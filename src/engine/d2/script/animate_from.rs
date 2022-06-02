use std::rc::Rc;

use crate::engine::d2::{
    animation::{AnimatedFloat, Ease, EaseFunction, Tween},
    Entity,
};

use super::Action;

/// An action that tweens an AnimatedFloat from a certain value to its current value.
pub struct AnimateFrom {
    tween: Rc<Tween>,

    value: AnimatedFloat,
    from: f32,
    to: f32,
    seconds: f32,
}

impl AnimateFrom {
    pub fn new(
        value: AnimatedFloat,
        from: f32,
        seconds: f32,
        easing: Option<EaseFunction>,
    ) -> Self {
        let easing = easing.unwrap_or(Rc::new(Ease::linear));
        let to = value.get();
        let tween = Rc::new(Tween::new(from, to, seconds, Some(easing)));
        // Move to initial value
        value.set(from);
        value.set_behavior(Some(tween.clone()));

        Self {
            tween,
            value,
            from,
            to,
            seconds,
        }
    }
}

impl Action for AnimateFrom {
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
