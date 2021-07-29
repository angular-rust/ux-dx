use wgpu_types::{BufferAddress, InputStepMode, VertexAttribute};

pub use super::FramebufferExt;
pub use super::TextureExt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VertexBufferLayout<'a> {
    pub array_stride: BufferAddress,
    pub step_mode: InputStepMode,
    pub attributes: &'a [VertexAttribute],
}

pub trait VertexAttributesLayout {
    /// `shader_location` in VertexAttribute should be adjusted inside application in glsl100 mode
    fn layout() -> &'static VertexBufferLayout<'static>;
}
