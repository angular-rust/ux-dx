use std::mem;

// * VertexP3T2:
// * @x: The x component of a position attribute
// * @y: The y component of a position attribute
// * @z: The z component of a position attribute
// * @s: The s component of a texture coordinate attribute
// * @t: The t component of a texture coordinate attribute
// *
// * A convenience vertex definition that can be used with
// * primitive_new_p3t2().
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VertexP3T2 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub s: f32,
    pub t: f32,
}