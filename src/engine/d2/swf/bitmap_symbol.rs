use std::{fmt, rc::Rc};

use crate::engine::d2::{
    display::{ImageSprite, Sprite, SubTexture, Texture},
    swf::TextureFormat,
};

use super::Symbol;

/// Defines a Flump atlased texture.
pub struct BitmapSymbol {
    pub texture: Rc<dyn SubTexture>,
    pub anchor_x: f32,
    pub anchor_y: f32,

    name: Option<String>,
}

impl BitmapSymbol {
    pub fn new(json: TextureFormat, atlas: impl Texture) -> Self {
        let rect = json.rect;

        let mut instance = Self {
            name: json.symbol,
            texture: atlas.sub_texture(rect[0], rect[1], rect[2], rect[3]),
            anchor_x: 0.0,
            anchor_y: 0.0,
        };

        if let Some(origin) = json.origin {
            instance.anchor_x = origin[0];
            instance.anchor_y = origin[1];
        }

        instance
    }
}

impl Symbol<ImageSprite> for BitmapSymbol {
    #[inline]
    fn name(&self) -> Option<String> {
        self.name.clone()
    }

    fn create_sprite(&self) -> ImageSprite {
        // let sprite = ImageSprite::new(Some(self.texture));
        // sprite.inner.set_anchor(self.anchor_x, self.anchor_y);

        // sprite
        unimplemented!()
    }
}

impl fmt::Debug for BitmapSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BitmapSymbol")
            //  .field("x", &self.x)
            //  .field("y", &self.y)
            .finish()
    }
}
