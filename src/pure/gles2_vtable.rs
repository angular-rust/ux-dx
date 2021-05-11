#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into,
    clippy::upper_case_acronyms
)]

use glib::translate::*;
use std::mem;

#[repr(C)]
pub struct GLES2Vtable {
    // TODO:
}

#[doc(hidden)]
impl Uninitialized for GLES2Vtable {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

// #[doc(hidden)]
// impl<'a> ToGlibPtr<'a, *const ffi::CoglGLES2Vtable> for GLES2Vtable {
//     type Storage = &'a Self;

//     #[inline]
//     fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GtkRequisition, Self> {
//         let ptr: *const GLES2Vtable = &*self;
//         Stash(ptr as *const ffi::CoglGLES2Vtable, self)
//     }
// }

// #[doc(hidden)]
// impl<'a> ToGlibPtrMut<'a, *mut ffi::CoglGLES2Vtable> for GLES2Vtable {
//     type Storage = &'a mut Self;

//     #[inline]
//     fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::CoglGLES2Vtable, Self> {
//         let ptr: *mut GLES2Vtable = &mut *self;
//         StashMut(ptr as *mut ffi::CoglGLES2Vtable, self)
//     }
// }

// #[doc(hidden)]
// impl FromGlibPtrNone<*const ffi::CoglGLES2Vtable> for GLES2Vtable {
//     unsafe fn from_glib_none(ptr: *const ffi::CoglGLES2Vtable) -> Self {
//         *(ptr as *const GLES2Vtable)
//     }
// }

// #[doc(hidden)]
// impl FromGlibPtrNone<*mut ffi::CoglGLES2Vtable> for GLES2Vtable {
//     unsafe fn from_glib_none(ptr: *mut ffi::CoglGLES2Vtable) -> Self {
//         *(ptr as *mut GLES2Vtable)
//     }
// }

// #[doc(hidden)]
// impl FromGlibPtrBorrow<*mut ffi::CoglGLES2Vtable> for GLES2Vtable {
//     unsafe fn from_glib_borrow(ptr: *mut ffi::CoglGLES2Vtable) -> glib::translate::Borrowed<Self> {
//         glib::translate::Borrowed::new(*(ptr as *mut GLES2Vtable))
//     }
// }

// #[doc(hidden)]
// impl FromGlibPtrBorrow<*const ffi::CoglGLES2Vtable> for GLES2Vtable {
//     unsafe fn from_glib_borrow(ptr: *const ffi::CoglGLES2Vtable) -> glib::translate::Borrowed<Self> {
//         glib::translate::Borrowed::new(*(ptr as *const GLES2Vtable))
//     }
// }
