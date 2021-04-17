#![allow(unused_imports)]

use glib::translate::*;
use std::mem;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DebugObjectTypeInfo<'a> {
    pub name: &'a str, // TODO: may be Option<String>, see gdk::WindowAttr
    pub instance_count: u64,
}

// #[doc(hidden)]
// impl<'a> Uninitialized for DebugObjectTypeInfo<'a> {
//     #[inline]
//     unsafe fn uninitialized() -> Self {
//         mem::zeroed()
//     }
// }

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::CoglDebugObjectTypeInfo> for DebugObjectTypeInfo<'a> {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::CoglDebugObjectTypeInfo, Self> {
        let ptr: *const DebugObjectTypeInfo = &*self;
        Stash(ptr as *const ffi::CoglDebugObjectTypeInfo, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::CoglDebugObjectTypeInfo> for DebugObjectTypeInfo<'a> {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::CoglDebugObjectTypeInfo, Self> {
        let ptr: *mut DebugObjectTypeInfo = &mut *self;
        StashMut(ptr as *mut ffi::CoglDebugObjectTypeInfo, self)
    }
}

#[doc(hidden)]
impl<'a> FromGlibPtrNone<*const ffi::CoglDebugObjectTypeInfo> for DebugObjectTypeInfo<'a> {
    unsafe fn from_glib_none(ptr: *const ffi::CoglDebugObjectTypeInfo) -> Self {
        *(ptr as *const DebugObjectTypeInfo)
    }
}

#[doc(hidden)]
impl<'a> FromGlibPtrNone<*mut ffi::CoglDebugObjectTypeInfo> for DebugObjectTypeInfo<'a> {
    unsafe fn from_glib_none(ptr: *mut ffi::CoglDebugObjectTypeInfo) -> Self {
        *(ptr as *mut DebugObjectTypeInfo)
    }
}

#[doc(hidden)]
impl<'a> FromGlibPtrBorrow<*mut ffi::CoglDebugObjectTypeInfo> for DebugObjectTypeInfo<'a> {
    unsafe fn from_glib_borrow(
        ptr: *mut ffi::CoglDebugObjectTypeInfo,
    ) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *mut DebugObjectTypeInfo))
    }
}

#[doc(hidden)]
impl<'a> FromGlibPtrBorrow<*const ffi::CoglDebugObjectTypeInfo> for DebugObjectTypeInfo<'a> {
    unsafe fn from_glib_borrow(
        ptr: *const ffi::CoglDebugObjectTypeInfo,
    ) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *const DebugObjectTypeInfo))
    }
}
