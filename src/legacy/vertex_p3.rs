use glib::translate::*;
use std::mem;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VertexP3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[doc(hidden)]
impl Uninitialized for VertexP3 {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::CoglVertexP3> for VertexP3 {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::CoglVertexP3, Self> {
        let ptr: *const VertexP3 = &*self;
        Stash(ptr as *const ffi::CoglVertexP3, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::CoglVertexP3> for VertexP3 {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::CoglVertexP3, Self> {
        let ptr: *mut VertexP3 = &mut *self;
        StashMut(ptr as *mut ffi::CoglVertexP3, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::CoglVertexP3> for VertexP3 {
    unsafe fn from_glib_none(ptr: *const ffi::CoglVertexP3) -> Self {
        *(ptr as *const VertexP3)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::CoglVertexP3> for VertexP3 {
    unsafe fn from_glib_none(ptr: *mut ffi::CoglVertexP3) -> Self {
        *(ptr as *mut VertexP3)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::CoglVertexP3> for VertexP3 {
    unsafe fn from_glib_borrow(ptr: *mut ffi::CoglVertexP3) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *mut VertexP3))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const ffi::CoglVertexP3> for VertexP3 {
    unsafe fn from_glib_borrow(ptr: *const ffi::CoglVertexP3) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *const VertexP3))
    }
}
