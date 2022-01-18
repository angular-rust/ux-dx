use cgmath::{Point2, Point3};
use std::mem;

use crate::{
    foundation::colorspace::Color,
    platform::core::{
        traits::{VertexAttributesLayout, VertexBufferLayout},
        BufferAddress, VertexAttribute, VertexFormat, VertexStepMode,
    },
    prelude::color,
};

// Vertex3p2t4c:
// @pos: The actual position component of the position attribute
// @uv: The actual uv component of a texture coordinate attribute
// @color: The actual color component of the color attribute
//
// A convenience vertex definition that can be used with
// primitive_new_p3t2c4().
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct P3T2C4 {
    pub pos: Point3<f32>,
    pub uv: Point2<f32>,
    pub color: Color,
}

impl Default for P3T2C4 {
    fn default() -> Self {
        Self {
            pos: Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            uv: Point2 { x: 0.0, y: 0.0 },
            color: color::BLACK,
        }
    }
}

impl P3T2C4 {
    pub fn new(pos: Point3<f32>, uv: Point2<f32>, color: Color) -> Self {
        Self { pos, uv, color }
    }

    pub fn from_components(pos: [f32; 3], uv: [f32; 2], color: [f32; 4]) -> Self {
        Self {
            pos: Point3 {
                x: pos[0],
                y: pos[1],
                z: pos[2],
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

impl VertexAttributesLayout for P3T2C4 {
    fn layout() -> &'static VertexBufferLayout<'static> {
        &VertexBufferLayout {
            array_stride: mem::size_of::<P3T2C4>() as BufferAddress,
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
                    format: VertexFormat::Float32x2,
                },
                VertexAttribute {
                    offset: mem::size_of::<[f32; 5]>() as BufferAddress, // size of previous parts
                    shader_location: 2,
                    format: VertexFormat::Float32x4,
                },
            ],
        }
    }
}
