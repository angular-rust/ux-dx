use std::mem;

// * VertexP2:
// * @x: The x component of a position attribute
// * @y: The y component of a position attribute
// *
// * A convenience vertex definition that can be used with
// * primitive_new_p2().
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VertexP2 {
    pub x: f32,
    pub y: f32,
}
