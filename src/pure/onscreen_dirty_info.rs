use std::mem;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OnscreenDirtyInfo {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
