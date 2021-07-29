use crate::gles::enums::*;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Attachment {
    Color(u32),
    Depth,
    Stencil,
    DepthStencil,
}

impl Attachment {
    pub(crate) fn to_flag(&self) -> u32 {
        match self {
            Self::Color(i) => GL_COLOR_ATTACHMENT0 + *i,
            Self::Depth => GL_DEPTH_ATTACHMENT,
            Self::Stencil => GL_STENCIL_ATTACHMENT,
            Self::DepthStencil => GL_DEPTH_STENCIL_ATTACHMENT,
        }
    }
}
