use crate::engine::d2::{
    animation::EaseFunction, display::Sprite, Director, Entity, EntityManager,
};

use super::TweenTransition;

/// Slides the old scene off the stage, and the new scene into its place.
pub struct SlideTransition {
    pub inner: TweenTransition,
    direction: u32, // = LEFT;

    // Where the old scene should end up
    x: f32,
    y: f32,
}

impl SlideTransition {
    pub const UP: u32 = 0;
    pub const DOWN: u32 = 1;
    pub const LEFT: u32 = 2;
    pub const RIGHT: u32 = 3;

    pub fn new(duration: f32, ease: Option<EaseFunction>) -> Self {
        Self {
            inner: TweenTransition::new(duration, ease),
            direction: Self::LEFT,
            x: 0.0,
            y: 0.0,
        }
    }

    /// Slides the transition upwards.
    /// @returns This instance, for chaining.
    pub fn up_signal(&mut self) -> &Self {
        self.direction = Self::UP;

        self
    }

    /// Slides the transition downwards.
    /// @returns This instance, for chaining.
    pub fn down(&mut self) -> &Self {
        self.direction = Self::DOWN;

        self
    }

    /// Slides the transition to the left.
    /// @returns This instance, for chaining.
    pub fn left(&mut self) -> &Self {
        self.direction = Self::LEFT;

        self
    }

    /// Slides the transition to the right.
    /// @returns This instance, for chaining.
    pub fn right(&mut self) -> &Self {
        self.direction = Self::RIGHT;

        self
    }

    // override
    pub fn init(&mut self, from: Entity, to: Entity) {
        self.inner.init(from, to);

        match self.direction {
            Self::UP => {
                self.x = 0.0;
                self.y = -Director::height();
            }
            Self::DOWN => {
                self.x = 0.0;
                self.y = Director::height();
            }
            Self::LEFT => {
                self.x = -Director::width();
                self.y = 0.0;
            }
            Self::RIGHT => {
                self.x = Director::width();
                self.y = 0.0;
            }
            _ => {}
        }

        if let Some(ref mut sprite) = EntityManager::<Sprite>::get(&self.inner.inner.from) {
            sprite.set_xy(0.0, 0.0);
        } else {
            let mut sprite = Sprite::new();
            self.inner.inner.from.add(sprite.component.clone());
            sprite.set_xy(0.0, 0.0);
        }

        if let Some(ref mut sprite) = EntityManager::<Sprite>::get(&self.inner.inner.to) {
            sprite.set_xy(-self.x, -self.y);
        } else {
            let mut sprite = Sprite::new();
            self.inner.inner.to.add(sprite.component.clone());
            sprite.set_xy(-self.x, -self.y);
        }
    }

    // override
    pub fn update(&mut self, dt: f32) -> bool {
        let done = self.inner.update(dt);
        if let Some(ref mut sprite) = EntityManager::<Sprite>::get(&self.inner.inner.from) {
            sprite.set_xy(
                self.inner.interp(0.0, self.x),
                self.inner.interp(0.0, self.y),
            );
        }

        if let Some(ref mut sprite) = EntityManager::<Sprite>::get(&self.inner.inner.to) {
            sprite.set_xy(
                self.inner.interp(-self.x, 0.0),
                self.inner.interp(-self.y, 0.0),
            );
        }

        done
    }

    // override
    pub fn complete(&self) {
        if let Some(ref mut sprite) = EntityManager::<Sprite>::get(&self.inner.inner.from) {
            sprite.set_xy(0.0, 0.0);
        }
        if let Some(ref mut sprite) = EntityManager::<Sprite>::get(&self.inner.inner.to) {
            sprite.set_xy(0.0, 0.0);
        }
    }
}

impl AsRef<TweenTransition> for SlideTransition {
    fn as_ref(&self) -> &TweenTransition {
        &self.inner
    }
}
