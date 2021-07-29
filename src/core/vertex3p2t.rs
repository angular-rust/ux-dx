use cgmath::{Point2, Point3};
use std::mem;
use wgpu_types::{BufferAddress, InputStepMode, VertexAttribute, VertexFormat};

use super::traits::{VertexAttributesLayout, VertexBufferLayout};

// Vertex3p2t:
// @pos: The actual position component of the position attribute
// @uv: The actual uv component of a texture coordinate attribute
//
// A convenience vertex definition that can be used with
// primitive_new_p3t2().
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex3p2t {
    pub pos: Point3<f32>,
    pub uv: Point2<f32>,
}

impl Vertex3p2t {
    pub fn new(pos: Point3<f32>, uv: Point2<f32>) -> Self {
        Self { pos, uv }
    }

    pub fn from_components(pos: [f32; 3], uv: [f32; 2]) -> Self {
        Self {
            pos: Point3 {
                x: pos[0],
                y: pos[1],
                z: pos[2],
            },
            uv: Point2 { x: uv[0], y: uv[1] },
        }
    }
}

impl VertexAttributesLayout for Vertex3p2t {
    fn layout() -> &'static VertexBufferLayout<'static> {
        &VertexBufferLayout {
            array_stride: mem::size_of::<Vertex3p2t>() as BufferAddress,
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
                    format: VertexFormat::Float32x2,
                },
            ],
        }
    }
}
