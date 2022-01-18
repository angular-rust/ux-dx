use cgmath::Point3;
use std::mem;

use crate::platform::core::{
    traits::{VertexAttributesLayout, VertexBufferLayout},
    BufferAddress, VertexAttribute, VertexFormat, VertexStepMode,
};

// Vertex3p:
// @pos: The actual position component of the position attribute
//
// A convenience vertex definition that can be used with
// primitive_new_p3().
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct P3N3 {
    pub pos: Point3<f32>,
    pub nor: Point3<f32>,
}

impl Default for P3N3 {
    fn default() -> Self {
        Self {
            pos: Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            nor: Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}

impl P3N3 {
    pub fn new(pos: Point3<f32>, nor: Point3<f32>) -> Self {
        Self { pos, nor }
    }

    pub fn from_components(pos: [f32; 3], nor: [f32; 3]) -> Self {
        Self {
            pos: Point3 {
                x: pos[0],
                y: pos[1],
                z: pos[2],
            },
            nor: Point3 {
                x: nor[0],
                y: nor[1],
                z: nor[2],
            },
        }
    }
}

impl VertexAttributesLayout for P3N3 {
    fn layout() -> &'static VertexBufferLayout<'static> {
        &VertexBufferLayout {
            array_stride: mem::size_of::<P3N3>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as BufferAddress, // size of previous parts
                    shader_location: 1,
                    format: VertexFormat::Float32x3,
                },
            ],
        }
    }
}
