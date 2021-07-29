use cgmath::Point3;
use std::mem;
use wgpu_types::{BufferAddress, InputStepMode, VertexAttribute, VertexFormat};

use crate::Color;

use super::traits::{VertexAttributesLayout, VertexBufferLayout};

// Vertex3p4c:
// @pos: The actual position component of the position attribute
// @color: The actual color component of the color attribute
//
// A convenience vertex definition that can be used with
// primitive_new_p3c4().
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex3p4c {
    pub pos: Point3<f32>,
    pub color: Color,
}

impl Vertex3p4c {
    pub fn new(pos: Point3<f32>, color: Color) -> Self {
        Self { pos, color }
    }

    pub fn from_components(pos: [f32; 3], color: [f32; 4]) -> Self {
        Self {
            pos: Point3 {
                x: pos[0],
                y: pos[1],
                z: pos[2],
            },
            color: Color {
                red: color[0],
                green: color[1],
                blue: color[2],
                alpha: color[3],
            },
        }
    }
}

impl VertexAttributesLayout for Vertex3p4c {
    fn layout() -> &'static VertexBufferLayout<'static> {
        &VertexBufferLayout {
            array_stride: mem::size_of::<Vertex3p4c>() as BufferAddress,
            step_mode: InputStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as BufferAddress, // size of previous parts
                    shader_location: 1,
                    format: VertexFormat::Float32x4,
                },
            ],
        }
    }
}
