use glib::translate::*;
use std::mem;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VertexP2T2 {
    pub x: f32,
    pub y: f32,
    pub s: f32,
    pub t: f32,
}

#[doc(hidden)]
impl Uninitialized for VertexP2T2 {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::CoglVertexP2T2> for VertexP2T2 {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::CoglVertexP2T2, Self> {
        let ptr: *const VertexP2T2 = &*self;
        Stash(ptr as *const ffi::CoglVertexP2T2, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::CoglVertexP2T2> for VertexP2T2 {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::CoglVertexP2T2, Self> {
        let ptr: *mut VertexP2T2 = &mut *self;
        StashMut(ptr as *mut ffi::CoglVertexP2T2, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::CoglVertexP2T2> for VertexP2T2 {
    unsafe fn from_glib_none(ptr: *const ffi::CoglVertexP2T2) -> Self {
        *(ptr as *const VertexP2T2)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::CoglVertexP2T2> for VertexP2T2 {
    unsafe fn from_glib_none(ptr: *mut ffi::CoglVertexP2T2) -> Self {
        *(ptr as *mut VertexP2T2)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::CoglVertexP2T2> for VertexP2T2 {
    unsafe fn from_glib_borrow(ptr: *mut ffi::CoglVertexP2T2) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *mut VertexP2T2))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const ffi::CoglVertexP2T2> for VertexP2T2 {
    unsafe fn from_glib_borrow(ptr: *const ffi::CoglVertexP2T2) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *const VertexP2T2))
    }
}
