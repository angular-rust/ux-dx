use crate::gles::{core30::gl, enums::*};
use std::marker::PhantomData;
// use std::rc::Rc;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum BufferTarget {
    Vertex,
    Element,
}

impl BufferTarget {
    pub(crate) fn to_flag(&self) -> u32 {
        match self {
            Self::Vertex => GL_ARRAY_BUFFER,
            Self::Element => GL_ELEMENT_ARRAY_BUFFER,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum BufferUsage {
    Static,
    Dynamic,
    Stream,
}

impl BufferUsage {
    pub(crate) fn to_flag(&self) -> u32 {
        match self {
            Self::Static => GL_STATIC_DRAW,
            Self::Dynamic => GL_DYNAMIC_DRAW,
            Self::Stream => GL_STREAM_DRAW,
        }
    }
}

// pub type BufferId = <Context as HasContext>::Buffer;

pub struct Buffer<T> {
    id: u32,
    target: BufferTarget,
    phantom: PhantomData<T>,
    unit_bytes_size: usize,
}

impl<T> Buffer<T> {
    pub fn new(target: BufferTarget) -> Result<Self, String> {
        let id = gl::gen_buffer();
        let unit_bytes_size = std::mem::size_of::<T>();
        Ok(Self {
            id,
            target,
            phantom: PhantomData,
            unit_bytes_size,
        })
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn target(&self) -> BufferTarget {
        self.target
    }

    pub fn unit_bytes_size(&self) -> usize {
        self.unit_bytes_size
    }

    pub fn bind(&self) {
        gl::bind_buffer(self.target.to_flag(), self.id);
    }

    pub fn unbind(&self) {
        gl::bind_buffer(self.target.to_flag(), 0);
    }

    pub fn init_size(&self, usage: BufferUsage, size: usize) {
        gl::buffer_data_size(
            self.target.to_flag(),
            self.unit_bytes_size * size,
            usage.to_flag(),
        );
    }

    pub fn init_with_data(&self, usage: BufferUsage, data: &[T]) {
        gl::buffer_data(self.target.to_flag(), data, usage.to_flag());
    }

    pub fn sub_data(&self, offset: usize, data: &[T]) {
        gl::buffer_sub_data(
            self.target.to_flag(),
            (self.unit_bytes_size * offset) as isize,
            data,
        );
    }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        gl::delete_buffers(&[self.id]);
    }
}

impl<T> PartialEq for Buffer<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub type VertexBuffer = Buffer<f32>;

impl VertexBuffer {
    pub fn new_vertex() -> Result<Self, String> {
        Self::new(BufferTarget::Vertex)
    }

    pub fn set_attrib_pointer_f32(&self, index: usize, size: usize, stride: usize, offset: usize) {
        gl::vertex_attrib_pointer_offset(
            index as u32,
            size as i32,
            GL_FLOAT,
            false,
            (self.unit_bytes_size * stride) as i32,
            (self.unit_bytes_size * offset) as u32,
        );
        gl::enable_vertex_attrib_array(index as u32);
    }
}

pub type ElementBuffer = Buffer<u16>;

impl ElementBuffer {
    pub fn new_element() -> Result<Self, String> {
        Self::new(BufferTarget::Element)
    }
}
