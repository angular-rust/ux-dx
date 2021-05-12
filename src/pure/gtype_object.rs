use std::mem;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GtypeObject {
    // pub parent_instance: gobject::GTypeInstance, // TODO: deal with it
    pub dummy: u32,
}
