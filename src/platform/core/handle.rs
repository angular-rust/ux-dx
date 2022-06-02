use std::{ffi::c_void, mem, ptr};

#[repr(C)]
#[derive(Debug)]
pub struct Handle(c_void);
