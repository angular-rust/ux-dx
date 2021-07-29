use cgmath::Point2;
use std::mem;
use wgpu_types::{BufferAddress, InputStepMode, VertexAttribute, VertexFormat};

use super::traits::{VertexAttributesLayout, VertexBufferLayout};

// Vertex2p:
// @pos: The actual position component of the position attribute
//
// A convenience vertex definition that can be used with
// primitive_new_p2().
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex2p {
    pub pos: Point2<f32>,
}

impl Vertex2p {
    pub fn new(pos: Point2<f32>) -> Self {
        Self { pos }
    }

    pub fn from_components(pos: [f32; 2]) -> Self {
        Self {
            pos: Point2 {
                x: pos[0],
                y: pos[1],
            },
        }
    }
}

impl VertexAttributesLayout for Vertex2p {
    fn layout() -> &'static VertexBufferLayout<'static> {
        &VertexBufferLayout {
            array_stride: mem::size_of::<Vertex2p>() as BufferAddress,
            step_mode: InputStepMode::Vertex,
            attributes: &[VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: VertexFormat::Float32x2,
            }],
        }
    }
}
