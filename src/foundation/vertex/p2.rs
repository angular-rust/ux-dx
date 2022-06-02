use cgmath::Point2;
use std::mem;

use crate::platform::core::{
    traits::{VertexAttributesLayout, VertexBufferLayout},
    BufferAddress, VertexAttribute, VertexFormat, VertexStepMode,
};

// Vertex2p:
// @pos: The actual position component of the position attribute
//
// A convenience vertex definition that can be used with
// primitive_new_p2().
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct P2 {
    pub pos: Point2<f32>,
}

impl Default for P2 {
    fn default() -> Self {
        Self {
            pos: Point2 { x: 0.0, y: 0.0 },
        }
    }
}

impl P2 {
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

impl VertexAttributesLayout for P2 {
    fn layout() -> &'static VertexBufferLayout<'static> {
        &VertexBufferLayout {
            array_stride: mem::size_of::<P2>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: VertexFormat::Float32x2,
            }],
        }
    }
}
