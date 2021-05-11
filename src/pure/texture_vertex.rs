#![allow(unused_imports)]
use crate::Color;
use glib::translate::*;
use std::mem;

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct TextureVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub tx: f32,
    pub ty: f32,
    pub color: Color, //TODO: fixme Copy
}

#[doc(hidden)]
impl Uninitialized for TextureVertex {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::CoglTextureVertex> for TextureVertex {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::CoglTextureVertex, Self> {
        let ptr: *const TextureVertex = &*self;
        Stash(ptr as *const ffi::CoglTextureVertex, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::CoglTextureVertex> for TextureVertex {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::CoglTextureVertex, Self> {
        let ptr: *mut TextureVertex = &mut *self;
        StashMut(ptr as *mut ffi::CoglTextureVertex, self)
    }
}

// #[doc(hidden)]
// impl FromGlibPtrNone<*const ffi::CoglTextureVertex> for TextureVertex {
//     unsafe fn from_glib_none(ptr: *const ffi::CoglTextureVertex) -> Self {
//         *(ptr as *const TextureVertex)
//     }
// }

// #[doc(hidden)]
// impl FromGlibPtrNone<*mut ffi::CoglTextureVertex> for TextureVertex {
//     unsafe fn from_glib_none(ptr: *mut ffi::CoglTextureVertex) -> Self {
//         *(ptr as *mut TextureVertex)
//     }
// }

// #[doc(hidden)]
// impl FromGlibPtrBorrow<*mut ffi::CoglTextureVertex> for TextureVertex {
//     unsafe fn from_glib_borrow(
//         ptr: *mut ffi::CoglTextureVertex,
//     ) -> glib::translate::Borrowed<Self> {
//         glib::translate::Borrowed::new(*(ptr as *mut TextureVertex))
//     }
// }

// #[doc(hidden)]
// impl FromGlibPtrBorrow<*const ffi::CoglTextureVertex> for TextureVertex {
//     unsafe fn from_glib_borrow(
//         ptr: *const ffi::CoglTextureVertex,
//     ) -> glib::translate::Borrowed<Self> {
//         glib::translate::Borrowed::new(*(ptr as *const TextureVertex))
//     }
// }
