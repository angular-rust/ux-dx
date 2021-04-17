use glib::translate::*;
use std::mem;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct KmsCrtc {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    // _truncated_record_marker: libc::c_void, // TODO: deal with it
}

#[doc(hidden)]
impl Uninitialized for KmsCrtc {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::CoglKmsCrtc> for KmsCrtc {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::CoglKmsCrtc, Self> {
        let ptr: *const KmsCrtc = &*self;
        Stash(ptr as *const ffi::CoglKmsCrtc, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::CoglKmsCrtc> for KmsCrtc {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::CoglKmsCrtc, Self> {
        let ptr: *mut KmsCrtc = &mut *self;
        StashMut(ptr as *mut ffi::CoglKmsCrtc, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::CoglKmsCrtc> for KmsCrtc {
    unsafe fn from_glib_none(ptr: *const ffi::CoglKmsCrtc) -> Self {
        *(ptr as *const KmsCrtc)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::CoglKmsCrtc> for KmsCrtc {
    unsafe fn from_glib_none(ptr: *mut ffi::CoglKmsCrtc) -> Self {
        *(ptr as *mut KmsCrtc)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::CoglKmsCrtc> for KmsCrtc {
    unsafe fn from_glib_borrow(ptr: *mut ffi::CoglKmsCrtc) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *mut KmsCrtc))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const ffi::CoglKmsCrtc> for KmsCrtc {
    unsafe fn from_glib_borrow(ptr: *const ffi::CoglKmsCrtc) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *const KmsCrtc))
    }
}
