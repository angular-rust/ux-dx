use bytes::Bytes;
use std::{clone::Clone, fmt, sync::{Arc, RwLock}};

use super::{BufferUpdateHint, Usage, VertexAttribute, VertexFormat};

// @short_description: Common buffer functions, including data upload APIs
// @stability: unstable
//
// The Buffer API provides a common interface to manipulate
// buffers that have been allocated either via pixel_buffer_new()
// or attribute_buffer_new(). The API allows you to upload data
// to these buffers and define usage hints that help API manage your
// buffer optimally.
//
// Data can either be uploaded by supplying a pointer and size so API
// can copy your data, or you can mmap() a Buffer and then you can
// copy data to the buffer directly.
//
// One of the most common uses for Buffers is to upload texture
// data asynchronously since the ability to mmap the buffers into
// the CPU makes it possible for another thread to handle the IO
// of loading an image file and unpacking it into the mapped buffer
// without blocking other API operations.

#[derive(Debug, Clone)]
pub enum BufferFlags {
    None = 0,
    BufferObject = 1 << 0, // real openGL buffer object
    Mapped = 1 << 1,
    MappedFallback = 1 << 2,
}

impl Default for BufferFlags {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone)]
pub enum BufferUsageHint {
    Texture,
    AttributeBuffer,
    IndexBuffer,
}

impl Default for BufferUsageHint {
    fn default() -> Self {
        Self::Texture
    }
}

#[derive(Debug, Clone)]
pub enum BufferBindTarget {
    PixelPack,
    PixelUnpack,
    AttributeBuffer,
    IndexBuffer,
    Count,
}

impl Default for BufferBindTarget {
    fn default() -> Self {
        Self::PixelPack
    }
}

#[derive(Default, Debug, Clone)]
pub struct Buffer {
    // vtable: BufferVtable,
    last_target: BufferBindTarget,

    flags: BufferFlags,

    // OpenGL handle
    handle: u32,

    // size of the buffer, in bytes
    size: u32,
    usage_hint: BufferUsageHint,
    update_hint: BufferUpdateHint,

    // points to the mapped memory when the Buffer is a VBO, PBO,
    // ... or points to allocated memory in the fallback paths
    data: Bytes, //&[u8],

    immutable_ref: i32,

    store_created: bool,
}

#[derive(Default, Clone, Debug)]
pub struct VertexStructure {
    pub instanced: bool,
    pub elements: Vec<(String, VertexFormat)>,
}

impl VertexStructure {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn byte_size(&self) -> usize {
        unimplemented!()
    }

    pub fn add<T: Into<String>>(&mut self, name: T, format: VertexFormat) {
        self.elements.push((name.into(), format))
    }
}

// ```
// let mut structure = VertexStructure::new();
// structure.add("pos", VertexFormat::Float32x2);
//
// let vb = VertexBuffer::<P3>::new(3, structure, Usage::Static);
// println!("{:?}", vb);
// ```

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct VertexBuffer<T>
where
    T: Default + Clone + fmt::Debug,
{
    handle: Option<u32>,
    size: usize,
    vertices: Arc<RwLock<Option<Vec<T>>>>,
    atttribute_layout: VertexStructure,
    usage: Usage,
}

impl<T> VertexBuffer<T>
where
    T: Default + Clone + fmt::Debug,
{
    pub fn new(size: usize, atttribute_layout: VertexStructure, usage: Usage) -> Self {
        let mut vertices = Vec::with_capacity(size);
        vertices.resize(size, T::default());

        Self {
            handle: None,
            size,
            atttribute_layout,
            usage,
            vertices: Arc::new(RwLock::new(Some(vertices))),
        }
    }

    pub fn lock(&self, closure: impl Fn(&mut [T])) {
        match self.vertices.write() {
            Ok(ref mut vertices) => match vertices.as_mut() {
                Some(data) => {
                    closure(data.as_mut_slice());
                }
                None => {
                    log::warn!("VertexBuffer is empty");
                }
            },
            Err(e) => panic!("RwLock poisoned"),
        }
    }
}

impl<T> Drop for VertexBuffer<T>
where
    T: Default + Clone + fmt::Debug,
{
    fn drop(&mut self) {
        match self.handle {
            Some(handle) => {
                println!("Should free GPU buffer {}", handle);
            }
            None => {}
        }
    }
}
