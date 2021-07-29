use super::PrimitiveType;
use crate::gles::{core30::gl, enums::*};
// use std::rc::Rc;

// pub type VertexArrayId = <Context as HasContext>::VertexArray;

pub struct VertexArray {
    id: u32,
}

impl VertexArray {
    pub fn new() -> Result<Self, String> {
        let id = gl::gen_vertex_array();
        Ok(Self { id })
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn bind(&self) {
        gl::bind_vertex_array(self.id);
    }

    pub fn unbind(&self) {
        gl::bind_vertex_array(0);
    }

    pub fn draw_arrays(&self, primitive: PrimitiveType, first: usize, count: usize) {
        gl::draw_arrays(primitive.to_flag(), first as i32, count as i32);
    }

    pub fn draw_elements(&self, primitive: PrimitiveType, count: usize, offset: usize) {
        gl::draw_elements_offset(
            primitive.to_flag(),
            count as i32,
            GL_UNSIGNED_SHORT,
            offset as u32,
        );
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        gl::delete_vertex_arrays(&[self.id]);
    }
}

impl PartialEq for VertexArray {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
