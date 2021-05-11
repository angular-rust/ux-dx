use glib::translate::*;
use std::mem;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OnscreenDirtyInfo {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[doc(hidden)]
impl Uninitialized for OnscreenDirtyInfo {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::CoglOnscreenDirtyInfo> for OnscreenDirtyInfo {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::CoglOnscreenDirtyInfo, Self> {
        let ptr: *const OnscreenDirtyInfo = &*self;
        Stash(ptr as *const ffi::CoglOnscreenDirtyInfo, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::CoglOnscreenDirtyInfo> for OnscreenDirtyInfo {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::CoglOnscreenDirtyInfo, Self> {
        let ptr: *mut OnscreenDirtyInfo = &mut *self;
        StashMut(ptr as *mut ffi::CoglOnscreenDirtyInfo, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::CoglOnscreenDirtyInfo> for OnscreenDirtyInfo {
    unsafe fn from_glib_none(ptr: *const ffi::CoglOnscreenDirtyInfo) -> Self {
        *(ptr as *const OnscreenDirtyInfo)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::CoglOnscreenDirtyInfo> for OnscreenDirtyInfo {
    unsafe fn from_glib_none(ptr: *mut ffi::CoglOnscreenDirtyInfo) -> Self {
        *(ptr as *mut OnscreenDirtyInfo)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::CoglOnscreenDirtyInfo> for OnscreenDirtyInfo {
    unsafe fn from_glib_borrow(
        ptr: *mut ffi::CoglOnscreenDirtyInfo,
    ) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *mut OnscreenDirtyInfo))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const ffi::CoglOnscreenDirtyInfo> for OnscreenDirtyInfo {
    unsafe fn from_glib_borrow(
        ptr: *const ffi::CoglOnscreenDirtyInfo,
    ) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *const OnscreenDirtyInfo))
    }
}
