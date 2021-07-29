use crate::gles::enums::*;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum WrapMode {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
    // MirrorClampToEdge,
    ClampToBorder,
}

impl WrapMode {
    pub(crate) fn to_flag(&self) -> u32 {
        match self {
            Self::Repeat => GL_REPEAT,
            Self::MirroredRepeat => GL_MIRRORED_REPEAT,
            Self::ClampToEdge => GL_CLAMP_TO_EDGE,
            // Self::MirrorClampToEdge => GL_MIRROR_CLAMP_TO_EDGE,
            Self::ClampToBorder => GL_CLAMP_TO_BORDER,
        }
    }
}

impl Default for WrapMode {
    fn default() -> Self {
        WrapMode::Repeat
    }
}

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Wrap {
    pub horizontal: WrapMode,
    pub vertical: WrapMode,
    pub depth: WrapMode,
}

impl Wrap {
    pub fn new(horizontal: WrapMode, vertical: WrapMode, depth: WrapMode) -> Self {
        Self {
            horizontal,
            vertical,
            depth,
        }
    }

    pub fn uv(horizontal: WrapMode, vertical: WrapMode) -> Self {
        Self::new(horizontal, vertical, WrapMode::default())
    }
}
