use std::{fmt, rc::Rc};

use crate::engine::d2::display::{ImageSprite, Sprite, Texture};

use super::Symbol;
/// Defines a flipbook-style movie, typically created from a spritesheet. Use
/// `Library.fromFlipbooks()` to create a Library from a list of Flipbooks.
#[derive(Default, Clone, Debug)]
pub struct Flipbook {
    pub name: String,
    pub frames: Vec<FlipbookFrame>,
}

impl Flipbook {
    /// @param name The name of the symbol that will be placed in the library.
    /// @param textures The frames of the flipbook animation.
    pub fn new<A: Texture + 'static>(name: String, textures: Vec<Rc<A>>) -> Self {
        let mut instance = Self {
            name,
            frames: Vec::new(),
        };

        // By default, play the animation for one second
        let duration_per_frame = 1.0 / textures.len() as f32;
        for texture in textures {
            instance.frames.push(FlipbookFrame::new(
                (texture as Rc<dyn Texture>).clone(),
                duration_per_frame,
            ));
        }
        instance
    }

    /// Uniformly sets the duration for all frames in this flipbook, so that the entire movie takes
    /// the given duration.
    ///  *
    /// @param duration The movie duration, in seconds.
    /// @returns This instance, for chaining.
    pub fn set_duration(&mut self, duration: f32) -> &Self {
        let duration_per_frame = duration / self.frames.len() as f32;
        for frame in self.frames.iter_mut() {
            frame.duration = duration_per_frame;
        }

        self
    }

    /// Sets the anchor point for all frames in this flipbook.
    ///  *
    /// @returns This instance, for chaining.
    pub fn set_anchor(&mut self, x: f32, y: f32) -> &Self {
        for frame in self.frames.iter_mut() {
            frame.anchor_x = x;
            frame.anchor_y = y;
        }

        self
    }
}

#[derive(Clone, Debug)]
pub struct FlipbookFrame {
    /// The texture shown during this frame.
    pub texture: Rc<dyn Texture>,

    /// How long to show this frame, in seconds.
    pub duration: f32,

    /// The X position of this frame's anchor point.
    pub anchor_x: f32,

    /// The Y position of this frame's anchor point.
    pub anchor_y: f32,

    pub label: Option<String>,
}

impl FlipbookFrame {
    fn new(texture: Rc<dyn Texture>, duration: f32) -> Self {
        Self {
            texture,
            duration,
            anchor_x: 0.0,
            anchor_y: 0.0,
            label: None,
        }
    }

    fn to_symbol(&self) -> FrameSymbol {
        // impl Symbol<Sprite>
        FrameSymbol::new(self)
    }
}

pub struct FrameSymbol {
    pub name: String,
    texture: Rc<dyn Texture>,
    anchor_x: f32,
    anchor_y: f32,
}

impl FrameSymbol {
    pub fn new(frame: &FlipbookFrame) -> Self {
        Self {
            texture: frame.texture.clone(),
            anchor_x: frame.anchor_x,
            anchor_y: frame.anchor_y,
            name: String::new(),
        }
    }
}

impl Symbol<ImageSprite> for FrameSymbol {
    fn name(&self) -> Option<String> {
        None
    }

    fn create_sprite(&self) -> ImageSprite {
        let mut sprite = ImageSprite::new(Some(self.texture.clone()));
        sprite.inner.set_anchor(self.anchor_x, self.anchor_y);

        sprite
    }
}

impl fmt::Debug for FrameSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FrameSymbol")
            //  .field("x", &self.x)
            //  .field("y", &self.y)
            .finish()
    }
}
