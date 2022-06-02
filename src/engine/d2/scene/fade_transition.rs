use crate::engine::d2::{animation::EaseFunction, display::Sprite, Entity, EntityManager};

use super::TweenTransition;

/// Fades the new scene in front of the old scene.

pub struct FadeTransition {
    pub inner: TweenTransition,
}

impl FadeTransition {
    pub fn new(duration: f32, ease: Option<EaseFunction>) -> Self {
        Self {
            inner: TweenTransition::new(duration, ease),
        }
    }

    // override
    pub fn init(&mut self, from: Entity, to: Entity) {
        self.inner.init(from, to);
        if let Some(ref mut sprite) = EntityManager::<Sprite>::get(&self.inner.inner.to) {
            sprite.alpha.set(0.0);
        } else {
            let sprite = Sprite::new();
            self.inner.inner.to.add(sprite.component);
            sprite.alpha.set(0.0);
        }
    }

    // override
    pub fn update(&mut self, dt: f32) -> bool {
        let done = self.inner.update(dt);
        if let Some(ref mut sprite) = EntityManager::<Sprite>::get(&self.inner.inner.to) {
            sprite.alpha.set(self.inner.interp(0.0, 1.0));
        }
        done
    }

    // override
    pub fn complete(&self) {
        if let Some(ref mut sprite) = EntityManager::<Sprite>::get(&self.inner.inner.to) {
            sprite.alpha.set(1.0);
        }
    }
}

impl AsRef<TweenTransition> for FadeTransition {
    fn as_ref(&self) -> &TweenTransition {
        &self.inner
    }
}
