use std::rc::Rc;

use super::{Graphics, Sprite, Texture};

/// A fixed-size sprite that displays a single texture.
#[derive(Default, Clone, Debug)]
pub struct ImageSprite {
    pub inner: Sprite,
    /// The texture being displayed, or None if none.
    pub texture: Option<Rc<dyn Texture>>,
}

impl ImageSprite {
    pub fn new(texture: Option<Rc<dyn Texture>>) -> Self {
        Self {
            inner: Sprite::new(),
            texture,
        }
    }

    // override
    pub fn draw(&self, gfx: &Box<dyn Graphics>) {
        if let Some(ref texture) = self.texture {
            gfx.draw_texture(texture, 0.0, 0.0);
        }
    }

    // override
    pub fn natural_width(&self) -> f32 {
        if let Some(ref texture) = self.texture {
            texture.width() as f32
        } else {
            0.0
        }
    }

    // override
    pub fn natural_height(&self) -> f32 {
        if let Some(ref texture) = self.texture {
            texture.height() as f32
        } else {
            0.0
        }
    }
}

impl AsRef<Sprite> for ImageSprite {
    fn as_ref(&self) -> &Sprite {
        &self.inner
    }
}
