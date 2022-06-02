use std::{clone::Clone, fmt, sync::RwLock};

use super::{Context, Usage};

#[derive(Debug, Default)]
#[repr(C)]
pub struct IndexBuffer {
    // opengl handle to buffer, GLuint
    handle: Option<u32>,
    size: usize,
    indices: RwLock<Option<Vec<u16>>>,
    usage: Usage,
}

impl IndexBuffer {
    /// Declares a new `IndexBuffer` of `size` bytes to contain vertex
    /// indices. Once declared, data can be set using
    /// `buffer_set_data` or by mapping it into the application's
    /// address space using `buffer_map`.
    /// ## `context`
    /// A `Context`
    /// ## `bytes`
    /// The number of bytes to allocate for vertex attribute data.
    ///
    // ```no_run
    // let ib = IndexBuffer::new(3, Usage::Static);
    // ib.lock(|data| {
    //     data[0] = 0;
    //     data[1] = 1;
    //     data[2] = 2;
    // });
    // ```
    // # Returns
    //
    // A newly allocated `IndexBuffer`
    pub fn new(size: usize, usage: Usage) -> Self {
        let mut indices = Vec::with_capacity(size);
        indices.resize(size, 0);

        Self {
            handle: None,
            size,
            usage,
            indices: RwLock::new(Some(indices)),
        }
    }

    pub fn lock(&self, closure: impl Fn(&mut [u16])) {
        match self.indices.write() {
            Ok(ref mut indices) => match indices.as_mut() {
                Some(data) => {
                    closure(data.as_mut_slice());
                }
                None => {
                    log::warn!("IndexBuffer is empty");
                }
            },
            Err(e) => panic!("RwLock poisoned"),
        }
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        match self.handle {
            Some(handle) => {
                println!("Should free GPU buffer {}", handle);
            }
            None => {}
        }
    }
}

impl fmt::Display for IndexBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IndexBuffer")
    }
}
