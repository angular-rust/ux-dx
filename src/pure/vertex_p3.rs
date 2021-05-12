use std::mem;

// * VertexP3:
// * @x: The x component of a position attribute
// * @y: The y component of a position attribute
// * @z: The z component of a position attribute
// *
// * A convenience vertex definition that can be used with
// * primitive_new_p3().
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VertexP3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}