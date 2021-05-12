use std::mem;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UserDataKey {
    pub unused: i32,
}
