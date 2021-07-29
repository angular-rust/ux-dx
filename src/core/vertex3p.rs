use cgmath::Point3;
use std::mem;
use wgpu_types::{BufferAddress, InputStepMode, VertexAttribute, VertexFormat};

use super::traits::{VertexAttributesLayout, VertexBufferLayout};

// Vertex3p:
// @pos: The actual position component of the position attribute
//
// A convenience vertex definition that can be used with
// primitive_new_p3().
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex3p {
    pub pos: Point3<f32>,
}

impl Vertex3p {
    pub fn new(pos: Point3<f32>) -> Self {
        Self { pos }
    }

    pub fn from_components(pos: [f32; 3]) -> Self {
        Self {
            pos: Point3 {
                x: pos[0],
                y: pos[1],
                z: pos[2],
            },
        }
    }
}

impl VertexAttributesLayout for Vertex3p {
    fn layout() -> &'static VertexBufferLayout<'static> {
        &VertexBufferLayout {
            array_stride: mem::size_of::<Vertex3p>() as BufferAddress,
            step_mode: InputStepMode::Vertex,
            attributes: &[VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: VertexFormat::Float32x3,
            }],
        }
    }
}
