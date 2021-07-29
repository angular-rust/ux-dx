use crate::gles::enums::*;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum FilterMode {
    Nearest,
    Linear,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Filter {
    pub min: FilterMode,
    pub mag: FilterMode,
    pub mipmap: Option<FilterMode>,
}

impl Filter {
    pub fn new(min: FilterMode, mag: FilterMode, mipmap: Option<FilterMode>) -> Self {
        Self { min, mag, mipmap }
    }

    pub(crate) fn to_min_flag(&self) -> u32 {
        match (self.min, self.mipmap) {
            (FilterMode::Nearest, None) => GL_NEAREST,
            (FilterMode::Linear, None) => GL_LINEAR,
            (FilterMode::Nearest, Some(FilterMode::Nearest)) => GL_NEAREST_MIPMAP_NEAREST,
            (FilterMode::Linear, Some(FilterMode::Nearest)) => GL_LINEAR_MIPMAP_NEAREST,
            (FilterMode::Nearest, Some(FilterMode::Linear)) => GL_NEAREST_MIPMAP_LINEAR,
            (FilterMode::Linear, Some(FilterMode::Linear)) => GL_LINEAR_MIPMAP_LINEAR,
        }
    }

    pub(crate) fn to_mag_flag(&self) -> u32 {
        match self.mag {
            FilterMode::Nearest => GL_NEAREST,
            FilterMode::Linear => GL_LINEAR,
        }
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::new(FilterMode::Nearest, FilterMode::Nearest, None)
    }
}
