use cgmath::Point2;
use std::mem;

use crate::{
    foundation::colorspace::Color,
    platform::core::{
        traits::{VertexAttributesLayout, VertexBufferLayout},
        BufferAddress, VertexAttribute, VertexFormat, VertexStepMode,
    },
    prelude::color,
};

// Vertex2p4c:
// @pos: The actual position component of the position attribute
// @color: The actual color component of the color attribute
//
// A convenience vertex definition that can be used with
// primitive_new_p2c4().
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct P2C4 {
    pub pos: Point2<f32>,
    pub color: Color,
}

impl Default for P2C4 {
    fn default() -> Self {
        Self {
            pos: Point2 { x: 0.0, y: 0.0 },
            color: color::BLACK,
        }
    }
}

impl P2C4 {
    pub fn new(pos: Point2<f32>, color: Color) -> Self {
        Self { pos, color }
    }

    pub fn from_components(pos: [f32; 2], color: [f32; 4]) -> Self {
        Self {
            pos: Point2 {
                x: pos[0],
                y: pos[1],
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

impl VertexAttributesLayout for P2C4 {
    fn layout() -> &'static VertexBufferLayout<'static> {
        &VertexBufferLayout {
            array_stride: mem::size_of::<P2C4>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x2,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as BufferAddress, // size of previous parts
                    shader_location: 1,
                    format: VertexFormat::Float32x4,
                },
            ],
        }
    }
}
