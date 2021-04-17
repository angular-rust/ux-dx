use glib::translate::*;
use std::mem;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VertexP3T2 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub s: f32,
    pub t: f32,
}

#[doc(hidden)]
impl Uninitialized for VertexP3T2 {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::CoglVertexP3T2> for VertexP3T2 {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::CoglVertexP3T2, Self> {
        let ptr: *const VertexP3T2 = &*self;
        Stash(ptr as *const ffi::CoglVertexP3T2, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::CoglVertexP3T2> for VertexP3T2 {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::CoglVertexP3T2, Self> {
        let ptr: *mut VertexP3T2 = &mut *self;
        StashMut(ptr as *mut ffi::CoglVertexP3T2, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::CoglVertexP3T2> for VertexP3T2 {
    unsafe fn from_glib_none(ptr: *const ffi::CoglVertexP3T2) -> Self {
        *(ptr as *const VertexP3T2)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::CoglVertexP3T2> for VertexP3T2 {
    unsafe fn from_glib_none(ptr: *mut ffi::CoglVertexP3T2) -> Self {
        *(ptr as *mut VertexP3T2)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::CoglVertexP3T2> for VertexP3T2 {
    unsafe fn from_glib_borrow(ptr: *mut ffi::CoglVertexP3T2) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *mut VertexP3T2))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const ffi::CoglVertexP3T2> for VertexP3T2 {
    unsafe fn from_glib_borrow(ptr: *const ffi::CoglVertexP3T2) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *const VertexP3T2))
    }
}
