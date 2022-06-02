use crate::engine::d2::animation::AnimatedFloat;

use super::{Graphics, Sprite};

/// A sprite that displays a rectangle filled with a given color.
#[derive(Default, Clone, Debug)]
pub struct FillSprite {
    pub inner: Sprite,
    pub color: i32,
    pub width: AnimatedFloat,
    pub height: AnimatedFloat,
}

impl FillSprite {
    pub fn new(color: i32, width: f32, height: f32) -> Self {
        Self {
            inner: Sprite::new(),
            color,
            width: AnimatedFloat::new(width, None),
            height: AnimatedFloat::new(height, None),
        }
    }

    // override
    pub fn draw(&self, gfx: &Box<dyn Graphics>) {
        gfx.fill_rect(self.color, 0.0, 0.0, self.width.get(), self.height.get());
    }

    // override
    pub fn natural_width(&self) -> f32 {
        self.width.get()
    }

    // override
    pub fn natural_height(&self) -> f32 {
        self.height.get()
    }

    /// Chainable convenience method to set the width and height.
    /// @returns This instance, for chaining.
    pub fn set_size(&mut self, width: f32, height: f32) -> &Self {
        self.width.set(width);
        self.height.set(height);

        self
    }

    // override
    pub fn on_update(&mut self, dt: f32) {
        self.inner.on_update(dt);
        self.width.update(dt);
        self.height.update(dt);
    }
}

impl AsRef<Sprite> for FillSprite {
    fn as_ref(&self) -> &Sprite {
        &self.inner
    }
}
