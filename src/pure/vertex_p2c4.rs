use std::mem;

// * VertexP2C4:
// * @x: The x component of a position attribute
// * @y: The y component of a position attribute
// * @r: The red component of a color attribute
// * @b: The green component of a color attribute
// * @g: The blue component of a color attribute
// * @a: The alpha component of a color attribute
// *
// * A convenience vertex definition that can be used with
// * primitive_new_p2c4().
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VertexP2C4 {
    pub x: f32,
    pub y: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}