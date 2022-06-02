use std::rc::Rc;

use crate::engine::d2::animation::AnimatedFloat;

use super::{Graphics, Sprite, Texture};

/// A resizable sprite that tiles a texture over its area.
#[derive(Default, Clone, Debug)]
pub struct PatternSprite {
    pub inner: Sprite,
    /// The texture being displayed, or None if none.
    pub texture: Option<Rc<dyn Texture>>,

    pub width: AnimatedFloat,
    pub height: AnimatedFloat,
}

impl PatternSprite {
    // ?width: f32 = -1, ?height: f32 = -1
    pub fn new(texture: Option<Rc<dyn Texture>>, width: f32, height: f32) -> Self {
        let width = if width < 0.0 {
            if let Some(texture) = &texture {
                texture.width() as f32
            } else {
                0.0
            }
        } else {
            width
        };

        let height = if height < 0.0 {
            if let Some(texture) = &texture {
                texture.height() as f32
            } else {
                0.0
            }
        } else {
            height
        };

        Self {
            inner: Sprite::new(),
            texture: texture,
            width: AnimatedFloat::new(width, None),
            height: AnimatedFloat::new(height, None),
        }
    }

    // override
    pub fn draw(&self, gfx: &Box<dyn Graphics>) {
        if let Some(ref texture) = self.texture {
            gfx.draw_pattern(texture, 0.0, 0.0, self.width.get(), self.height.get());
        }
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

impl AsRef<Sprite> for PatternSprite {
    fn as_ref(&self) -> &Sprite {
        &self.inner
    }
}
