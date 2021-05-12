use std::mem;

// * CoglVertexP2T2:
// * @x: The x component of a position attribute
// * @y: The y component of a position attribute
// * @s: The s component of a texture coordinate attribute
// * @t: The t component of a texture coordinate attribute
// *
// * A convenience vertex definition that can be used with
// * cogl_primitive_new_p2t2().
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VertexP2T2 {
    pub x: f32,
    pub y: f32,
    pub s: f32,
    pub t: f32,
}
