use std::{cmp, fmt};

use crate::engine::d2::{
    animation::AnimatedFloat,
    display::Sprite,
    util::{BitSets, Signal0},
    Entity, EntityManager,
};

use super::{MovieLayer, MovieSymbol, Symbol};

/// An instanced Flump animation.
#[derive(Default, Clone, Debug)]
pub struct MovieSprite {
    pub inner: Sprite,
    /// The symbol this sprite displays.
    pub symbol: MovieSymbol,

    /// The playback speed multiplier of this movie, defaults to 1.0. Higher values will play faster.
    /// This does not affect the speed of nested child movies, use `SpeedAdjuster` if you need
    /// that.
    pub speed: AnimatedFloat,

    /// Whether this movie is currently paused.
    pub paused: bool,

    animators: Vec<LayerAnimator>,

    /// The current playback position in seconds.
    position: f32,
    frame: usize,

    /// Emitted when this movie loops back to the beginning.
    looped: Option<Signal0>,
}

impl MovieSprite {
    // Component flags
    pub const PAUSED: u32 = Sprite::NEXT_FLAG << 0;
    pub const SKIP_NEXT: u32 = Sprite::NEXT_FLAG << 1;
    pub const NEXT_FLAG: u32 = Sprite::NEXT_FLAG << 2; // Must be last!

    pub fn new(symbol: MovieSymbol) -> Self {
        let mut animators = Vec::with_capacity(symbol.layers.len());
        for layer in symbol.layers.iter() {
            animators.push(LayerAnimator::new(layer.clone()));
        }

        let mut instance = Self {
            inner: Sprite::new(),
            symbol,
            speed: AnimatedFloat::new(1.0, None),
            animators,
            frame: 0,
            position: 0.0,
            ..Default::default()
        };

        instance.goto(1);

        instance
    }

    /// Retrieves a named layer from this movie. Children can be added to the returned entity to add
    /// sprites that move with the layer, which for example, can be used to add equipment sprites to
    /// an avatar.
    /// @param required If true and the layer is not found, an error is thrown.
    //  required :bool = true
    pub fn layer(&self, name: String, required: bool) -> Option<Entity> {
        for animator in self.animators.iter() {
            if animator.layer.name == name {
                return Some(animator.content.clone());
            }
        }

        if required {
            panic!("Missing layer {}", name);
        }

        None
    }

    // override
    pub fn on_added(&self) {
        self.inner.on_added();

        for animator in self.animators.iter() {
            if let Some(ref owner) = self.inner.component.owner {
                owner
                    .entity()
                    .add_child(animator.content.clone(), true, None);
            }
        }
    }

    // override
    pub fn on_removed(&mut self) {
        self.inner.on_removed();

        // Detach the animator content layers so they don't get disconnected during a disposal. This
        // may be a little hacky as it prevents child components from ever being formally removed.
        for animator in self.animators.iter() {
            if let Some(ref owner) = self.inner.component.owner {
                owner.entity().remove_child(&animator.content);
            }
        }
    }

    // override
    pub fn on_update(&mut self, dt: f32) {
        self.inner.on_update(dt);

        self.speed.update(dt);

        match self.inner.component.flags & (Self::PAUSED | Self::SKIP_NEXT) {
            0 => {
                // Neither paused nor skipping set, advance time
                self.position += self.speed.get() * dt;
                if self.position > self.symbol.duration {
                    self.position = self.position % self.symbol.duration;

                    if let Some(ref looped) = self.looped {
                        looped.emit();
                    }
                }
            }
            Self::SKIP_NEXT => {
                // Not paused, but skip this time step
                self.inner.component.flags = self.inner.component.flags.remove(Self::SKIP_NEXT);
            }
            _ => {}
        }

        let new_frame = (self.position * self.symbol.frame_rate).round() as usize;
        self.goto(new_frame);
    }

    pub fn goto(&mut self, frame: usize) {
        if self.frame == frame {
            return; // No change
        }

        let wrapped = frame < self.frame;
        if wrapped {
            for animator in self.animators.iter_mut() {
                animator.needs_keyframe_update = true;
                animator.keyframe_idx = 0;
            }
        }

        for animator in self.animators.iter_mut() {
            animator.compose_frame(frame);
        }

        self.frame = frame;
    }

    #[inline]
    pub fn position(&self) -> f32 {
        self.position
    }

    pub fn set_position(&mut self, position: f32) {
        self.position = position.clamp(0.0, self.symbol.duration);
    }

    #[inline]
    pub fn paused(&self) -> bool {
        self.inner.component.flags.contains(Self::PAUSED)
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.inner.component.flags = self.inner.component.flags.set(Self::PAUSED, paused);
    }

    pub fn looped(&mut self) -> Option<Signal0> {
        if self.looped.is_none() {
            self.looped = Some(Signal0::new(None));
        }

        self.looped.clone()
    }

    // override
    pub fn set_pixel_snapping(&mut self, pixel_snapping: bool) {
        for layer in self.animators.iter_mut() {
            layer.set_pixel_snapping(pixel_snapping);
        }

        self.inner.set_pixel_snapping(pixel_snapping);
    }

    /// Internal method to set the position to 0 and skip the next update. This is required to modify
    /// the playback position of child movies during an update step, so that after the update
    /// trickles through the children, they end up at position=0 instead of position=dt.
    pub fn rewind(&mut self) {
        self.position = 0.0;
        self.inner.component.flags = self.inner.component.flags.add(Self::SKIP_NEXT);
    }
}

impl AsRef<Sprite> for MovieSprite {
    fn as_ref(&self) -> &Sprite {
        &self.inner
    }
}

impl fmt::Display for MovieSprite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MovieSprite")
    }
}

impl PartialEq for MovieSprite {
    fn eq(&self, other: &Self) -> bool {
        // self.path == other.path
        unimplemented!()
    }
}

impl PartialOrd for MovieSprite {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        // self.path.partial_cmp(&other.path)
        unimplemented!()
    }
}

// impl fmt::Debug for MovieSprite {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("MovieSprite")
//         //  .field("x", &self.x)
//         //  .field("y", &self.y)
//          .finish()
//     }
// }

#[derive(Default, Clone, Debug)]
pub struct LayerAnimator {
    pub content: Entity,

    pub needs_keyframe_update: bool,
    pub keyframe_idx: usize,

    pub layer: MovieLayer,

    // The sprite to show at each keyframe index, or None if this layer has no symbol instances
    sprites: Option<Vec<Sprite>>,
}

impl LayerAnimator {
    pub fn new(layer: MovieLayer) -> Self {
        let keyframes_len = layer.keyframes.len();

        let instance = Self {
            layer: layer.clone(),
            content: Entity::new(),
            sprites: None,
            needs_keyframe_update: false,
            keyframe_idx: 0,
        };

        if !layer.empty {
            // Populate _sprites with the Sprite at each keyframe, reusing consecutive symbols
            let sprites = Vec::<Sprite>::with_capacity(keyframes_len);

            // for ii in 0..instance.sprites.len() {
            //     let kf = layer.keyframes[ii];
            //     if ii > 0 && layer.keyframes[ii - 1].symbol == kf.symbol {
            //         // sprites[ii] = instance.sprites[ii - 1];
            //         todo!("should deal with it");
            //     } else {
            //         if let Some(ref symbol) = kf.symbol {
            //             // sprites[ii] = symbol.createSprite();
            //             todo!("should deal with it");
            //         } else {
            //             sprites[ii] = Sprite::new();
            //         }
            //     }
            // }
            todo!("should deal with it");
            // instance.content.add(sprites[0].as_ref());
            // instance.sprites = Some(sprites);
        }

        instance
    }

    pub fn compose_frame(&mut self, frame: usize) {
        if self.sprites.is_none() {
            // TODO -> Test this code path
            // Don't animate empty layers
            return;
        }

        // let keyframes = self.layer.keyframes;
        let final_frame = self.layer.keyframes.len() - 1;

        if frame > self.layer.frames {
            // TODO -> Test this code path
            // Not enough frames on this layer, hide it
            if let Some(mut sprite) = EntityManager::<Sprite>::get(&self.content) {
                sprite.visible = false;
            }

            self.keyframe_idx = final_frame;
            self.needs_keyframe_update = true;
            return;
        }

        while self.keyframe_idx < final_frame
            && self.layer.keyframes[self.keyframe_idx + 1].index <= frame
        {
            self.keyframe_idx += 1;
            self.needs_keyframe_update = true;
        }

        let mut sprite: Sprite = if self.needs_keyframe_update {
            self.needs_keyframe_update = false;
            if let Some(ref sprites) = self.sprites {
                // Switch to the next instance if this is a multi-layer symbol
                let sprite = sprites[self.keyframe_idx].clone();
                todo!("should deal with it");
                // if sprite != *AsRef::<Sprite>::as_ref(&self.content) {
                //     // if Type::getClass(sprite) == MovieSprite {
                //     //     let movie: MovieSprite = sprite;
                //     //     movie.rewind();
                //     // }
                //     // self.content.add(sprite);
                //     unimplemented!()
                // }
            } else {
                Sprite::default()
            }
        } else {
            AsRef::<Sprite>::as_ref(&self.content).clone()
        };

        let kf = self.layer.keyframes[self.keyframe_idx].clone();
        let visible = kf.visible && kf.symbol.is_some();
        sprite.visible = visible;

        if !visible {
            return; // Don't bother animating invisible layers
        }

        let mut x = kf.x;
        let mut y = kf.y;
        let mut scale_x = kf.scale_x;
        let mut scale_y = kf.scale_y;
        let mut skew_x = kf.skew_x;
        let mut skew_y = kf.skew_y;
        let mut alpha = kf.alpha;

        if kf.tweened && self.keyframe_idx < final_frame {
            let mut interp = (frame as f32 - kf.index as f32) / kf.duration as f32;
            let mut ease = kf.ease;
            if ease != 0.0 {
                let t;
                if ease < 0.0 {
                    // Ease in
                    let inv = 1.0 - interp;
                    t = 1.0 - inv * inv;
                    ease = -ease;
                } else {
                    // Ease out
                    t = interp * interp;
                }
                interp = ease * t + (1.0 - ease) * interp;
            }

            if let Some(next_kf) = self.layer.keyframes.get(self.keyframe_idx + 1) {
                x += (next_kf.x - x) * interp;
                y += (next_kf.y - y) * interp;
                scale_x += (next_kf.scale_x - scale_x) * interp;
                scale_y += (next_kf.scale_y - scale_y) * interp;
                skew_x += (next_kf.skew_x - skew_x) * interp;
                skew_y += (next_kf.skew_y - skew_y) * interp;
                alpha += (next_kf.alpha - alpha) * interp;
            }
        }

        // From an identity matrix, append the translation, skew, and scale
        let mut matrix = sprite.local_matrix();
        let mut sin_x = 0.0;
        let mut cos_x = 1.0;
        let mut sin_y = 0.0;
        let mut cos_y = 1.0;

        if skew_x != 0.0 {
            sin_x = skew_x.sin();
            cos_x = skew_x.cos();
        }

        if skew_y != 0.0 {
            sin_y = skew_y.sin();
            cos_y = skew_y.cos();
        }

        matrix.set(
            cos_y * scale_x,
            sin_y * scale_x,
            -sin_x * scale_y,
            cos_x * scale_y,
            x,
            y,
        );

        // Append the pivot
        matrix.translate(-kf.pivot_x, -kf.pivot_y);

        sprite.alpha.set(alpha);
    }

    pub fn set_pixel_snapping(&mut self, pixel_snapping: bool) {
        if let Some(ref mut sprites) = self.sprites {
            for sprite in sprites {
                sprite.pixel_snapping = pixel_snapping;
            }
        }
    }
}
