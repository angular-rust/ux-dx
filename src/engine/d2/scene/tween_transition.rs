use std::rc::Rc;

use crate::engine::d2::{
    animation::{Ease, EaseFunction},
    Entity,
};

use super::Transition;

/// A helper extended by transitions that tween between two states.
pub struct TweenTransition {
    pub inner: Transition,
    pub ease: EaseFunction,
    pub elapsed: f32,
    pub duration: f32,
}

impl TweenTransition {
    pub fn new(duration: f32, ease: Option<EaseFunction>) -> Self {
        Self {
            inner: Transition::default(),
            elapsed: 0.0,
            duration,
            ease: if let Some(ease) = ease {
                ease
            } else {
                Rc::new(Ease::linear)
            },
        }
    }

    // override
    pub fn init(&mut self, from: Entity, to: Entity) {
        self.inner.init(from, to);
        self.elapsed = 0.0;
    }

    // override
    pub fn update(&mut self, dt: f32) -> bool {
        self.elapsed += dt;

        self.elapsed >= self.duration
    }

    pub fn interp(&self, from: f32, to: f32) -> f32 {
        from + (to - from) * (self.ease)(self.elapsed / self.duration)
    }
}

impl AsRef<Transition> for TweenTransition {
    fn as_ref(&self) -> &Transition {
        &self.inner
    }
}
