// Documents Flump's JSON format and adds some type-safety to parsing

pub struct Format {
    // A checksum of the original FLA library used to generate this file, used by the exporter tool
    // to detect modifications
    pub md5: String,

    // The frame rate
    pub frame_rate: f32,

    // All the movies and atlases in the library
    pub movies: Vec<MovieFormat>,
    pub texture_groups: Vec<TextureGroupFormat>,
}

pub struct TextureGroupFormat {
    // The additional scale factor (not supported, use different base scales instead)
    pub scale_factor: f32,

    // The atlases in this scale group
    pub atlases: Vec<AtlasFormat>,
}

#[derive(Default, Clone, Debug)]
pub struct MovieFormat {
    // The symbol name of this movie
    // TODO -> Why not call it symbol? Movies share the same namespace as textures
    pub id: Option<String>,

    pub layers: Vec<LayerFormat>,
}

#[derive(Default, Clone, Debug)]
pub struct LayerFormat {
    // The name of the layer
    pub name: String,

    // Optional: Whether this is a flipbook-style animation. Defaults to false
    pub flipbook: Option<bool>,

    pub keyframes: Vec<KeyframeFormat>,
}

#[derive(Default, Clone, Debug)]
pub struct KeyframeFormat {
    // The number of frames until the next keyframe
    pub duration: usize,

    // Optional: The name of the symbol that should be shown at this keyframe
    pub ref_: Option<String>,

    // Optional: Transform [x, y] properties. Defaults to [0, 0]
    pub loc: Option<Vec<f32>>,

    // Optional: Transform [scaleX, scaleY] properties. Defaults to [1, 1]
    pub scale: Option<Vec<f32>>,

    // Optional: Transform [skewX, skewY] in radians. Defaults to 0
    pub skew: Option<Vec<f32>>,

    // Optional: The anchor point [x, y]. Defaults to [0, 0]
    pub pivot: Option<Vec<f32>>,

    // Optional: Symbol alpha. Defaults to 1.0
    pub alpha: Option<f32>,

    // Optional: The frame label that was added to this keyframe
    pub label: Option<String>,

    // Optional: Whether this keyframe should be displayed. Defaults to true
    pub visible: Option<bool>,

    // Optional: Whether this keyframe is tweened into the next. Defaults to true
    pub tweened: Option<bool>,

    // Optional: Easing factor to tween this keyframe's properties, from -1.0 to 1.0. Defaults to 0
    pub ease: Option<f32>,
}

#[derive(Default, Clone, Debug)]
pub struct AtlasFormat {
    // The path to the atlas
    pub file: String,

    // The textures packed in this atlas
    pub textures: Vec<TextureFormat>,
}

#[derive(Default, Clone, Debug)]
pub struct TextureFormat {
    // The symbol name of this texture
    pub symbol: Option<String>,

    // The bitmap's anchor point, relative to the top left of its rect
    pub origin: Option<Vec<f32>>,

    // The rectangle bounding the texture in its atlas
    pub rect: Vec<i32>,
}
