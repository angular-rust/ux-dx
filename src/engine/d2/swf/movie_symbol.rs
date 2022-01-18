use std::rc::Rc;

use crate::engine::d2::display::Sprite;

use super::{KeyframeFormat, LayerFormat, Library, MovieFormat, MovieSprite, Symbol};

/// Defines a Flump movie.
#[derive(Default, Clone, Debug)]
pub struct MovieSymbol {
    name: Option<String>,

    pub layers: Vec<MovieLayer>,

    /// The total number of frames in this movie.
    pub frames: usize,

    /// The rate that this movie is played, in frames per second.
    pub frame_rate: f32,

    /// The duration of this animation in seconds.
    pub duration: f32,
}

impl MovieSymbol {
    pub fn new(lib: Library, json: MovieFormat) -> Self {
        let mut frames = 0;
        let mut layers = Vec::with_capacity(json.layers.len());

        for item in json.layers.iter() {
            let layer = MovieLayer::new(item.clone());
            frames = layer.frames.max(frames);
            layers.push(layer);
        }

        let mut instance = Self {
            name: json.id,
            frame_rate: lib.frame_rate,
            frames,
            layers,
            ..Default::default()
        };

        instance.duration = instance.frames as f32 / instance.frame_rate as f32;

        instance
    }
}

impl Symbol<MovieSprite> for MovieSymbol {
    #[inline]
    fn name(&self) -> Option<String> {
        self.name.clone()
    }

    fn create_sprite(&self) -> MovieSprite {
        MovieSprite::new(self.clone())
    }
}

#[derive(Default, Clone, Debug)]
pub struct MovieLayer {
    pub name: String,
    pub keyframes: Vec<MovieKeyframe>,
    pub frames: usize,

    /// Whether this layer has no symbol instances.
    pub empty: bool,
}

impl MovieLayer {
    pub fn new(json: LayerFormat) -> Self {
        let mut keyframes = Vec::with_capacity(json.keyframes.len());

        let mut empty = false;
        let mut prev_kf = None;

        for item in json.keyframes.iter() {
            let key_frame = MovieKeyframe::new(item.clone(), prev_kf);
            keyframes.push(key_frame.clone());

            empty = empty && key_frame.symbol_name.is_empty();
            prev_kf = Some(key_frame);
        }

        Self {
            name: json.name,
            keyframes,
            empty,
            frames: if let Some(prev_kf) = prev_kf {
                prev_kf.index + prev_kf.duration
            } else {
                0
            },
            ..Default::default()
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct MovieKeyframe {
    pub index: usize,

    /// The length of this keyframe in frames.
    pub duration: usize,

    pub symbol_name: String,
    pub symbol: Option<Rc<dyn Symbol<MovieSprite>>>,

    pub label: String,

    pub x: f32,
    pub y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub skew_x: f32,
    pub skew_y: f32,

    pub pivot_x: f32,
    pub pivot_y: f32,

    pub alpha: f32,

    pub visible: bool,

    /// Whether this keyframe should be tweened to the next.
    pub tweened: bool,

    /// Easing amount, if tweened is true.
    pub ease: f32,
}

impl MovieKeyframe {
    pub fn new(json: KeyframeFormat, prev_kf: Option<MovieKeyframe>) -> Self {
        let mut instance = Self {
            index: if let Some(prev_kf) = prev_kf {
                prev_kf.index + prev_kf.duration
            } else {
                0
            },
            duration: json.duration,
            label: json.label.unwrap_or_default(),
            symbol_name: json.ref_.unwrap_or_default(),
            ..Default::default()
        };

        if let Some(loc) = json.loc {
            instance.x = loc[0];
            instance.y = loc[1];
        }

        if let Some(scale) = json.scale {
            instance.scale_x = scale[0];
            instance.scale_y = scale[1];
        }

        if let Some(skew) = json.skew {
            instance.skew_x = skew[0];
            instance.skew_y = skew[1];
        }

        if let Some(pivot) = json.pivot {
            instance.pivot_x = pivot[0];
            instance.pivot_y = pivot[1];
        }

        if let Some(alpha) = json.alpha {
            instance.alpha = alpha;
        }

        if let Some(visible) = json.visible {
            instance.visible = visible;
        }

        if let Some(tweened) = json.tweened {
            instance.tweened = tweened;
        }

        if let Some(ease) = json.ease {
            instance.ease = ease;
        }

        instance
    }

    #[inline]
    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    #[inline]
    fn set_symbol(&mut self, symbol: Option<Rc<dyn Symbol<MovieSprite>>>) {
        self.symbol = symbol;
    }
}
