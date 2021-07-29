use cgmath::Point2;
use std::mem;
use wgpu_types::{BufferAddress, InputStepMode, VertexAttribute, VertexFormat};

use crate::Color;

use super::traits::{VertexAttributesLayout, VertexBufferLayout};

// Vertex2p2t4c:
// @pos: The actual position component of the position attribute
// @uv: The actual uv component of a texture coordinate attribute
// @color: The actual color component of the color attribute
//
// A convenience vertex definition that can be used with
// primitive_new_p3t2c4().
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex2p2t4c {
    pub pos: Point2<f32>,
    pub uv: Point2<f32>,
    pub color: Color,
}

impl Vertex2p2t4c {
    pub fn new(pos: Point2<f32>, uv: Point2<f32>, color: Color) -> Self {
        Self { pos, uv, color }
    }

    pub fn from_components(pos: [f32; 2], uv: [f32; 2], color: [f32; 4]) -> Self {
        Self {
            pos: Point2 {
                x: pos[0],
                y: pos[1],
            },
            uv: Point2 { x: uv[0], y: uv[1] },
            color: Color {
                red: color[0],
                green: color[1],
                blue: color[2],
                alpha: color[3],
            },
        }
    }
}

impl VertexAttributesLayout for Vertex2p2t4c {
    fn layout() -> &'static VertexBufferLayout<'static> {
        &VertexBufferLayout {
            array_stride: mem::size_of::<Vertex2p2t4c>() as BufferAddress,
            step_mode: InputStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x2,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as BufferAddress, // size of previous parts
                    shader_location: 1,
                    format: VertexFormat::Float32x2,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as BufferAddress, // size of previous parts
                    shader_location: 2,
                    format: VertexFormat::Float32x4,
                },
            ],
        }
    }
}
