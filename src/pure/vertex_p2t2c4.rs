use std::mem;

// * CoglVertexP2T2C4:
// * @x: The x component of a position attribute
// * @y: The y component of a position attribute
// * @s: The s component of a texture coordinate attribute
// * @t: The t component of a texture coordinate attribute
// * @r: The red component of a color attribute
// * @b: The green component of a color attribute
// * @g: The blue component of a color attribute
// * @a: The alpha component of a color attribute
// *
// * A convenience vertex definition that can be used with
// * cogl_primitive_new_p3t2c4().
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VertexP2T2C4 {
    pub x: f32,
    pub y: f32,
    pub s: f32,
    pub t: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
