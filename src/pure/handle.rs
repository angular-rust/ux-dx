use std::{mem, ptr, ffi::c_void};

#[repr(C)]
#[derive(Debug)]
pub struct Handle(c_void);
