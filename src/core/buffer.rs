use super::BufferUpdateHint;
use bytes::Bytes;

// SECTION:buffer
// @short_description: Common buffer functions, including data upload APIs
// @stability: unstable
//
// The CoglBuffer API provides a common interface to manipulate
// buffers that have been allocated either via pixel_buffer_new()
// or attribute_buffer_new(). The API allows you to upload data
// to these buffers and define usage hints that help Cogl manage your
// buffer optimally.
//
// Data can either be uploaded by supplying a pointer and size so Cogl
// can copy your data, or you can mmap() a CoglBuffer and then you can
// copy data to the buffer directly.
//
// One of the most common uses for CoglBuffers is to upload texture
// data asynchronously since the ability to mmap the buffers into
// the CPU makes it possible for another thread to handle the IO
// of loading an image file and unpacking it into the mapped buffer
// without blocking other Cogl operations.

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
    // GLuint gl_handle;

    // size of the buffer, in bytes
    size: u32,
    usage_hint: BufferUsageHint,
    update_hint: BufferUpdateHint,

    // points to the mapped memory when the CoglBuffer is a VBO, PBO,
    // ... or points to allocated memory in the fallback paths
    data: Bytes, //&[u8],

    immutable_ref: i32,

    store_created: bool,
}
