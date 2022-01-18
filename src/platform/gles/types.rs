#![allow(non_camel_case_types)]

//! Contains all the GL types.

#[cfg(feature = "chlorine")]
use chlorine::*;
#[cfg(not(feature = "chlorine"))]
use std::os::raw::*;
// use super::*;
pub type GLenum = c_uint;
pub type GLboolean = c_uchar;
pub type GLbitfield = c_uint;
pub type GLvoid = c_void;
pub type GLbyte = i8;
pub type GLubyte = u8;
pub type GLshort = i16;
pub type GLushort = u16;
pub type GLint = c_int;
pub type GLuint = c_uint;
pub type GLclampx = i32;
pub type GLsizei = c_int;
pub type GLfloat = c_float;
pub type GLclampf = c_float;
pub type GLdouble = c_double;
pub type GLclampd = c_double;
pub type GLeglClientBufferEXT = *mut c_void;
pub type GLeglImageOES = *mut c_void;
pub type GLchar = c_char;
pub type GLcharARB = c_char;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub type GLhandleARB = *mut c_void;
#[cfg(not(any(target_os = "macos", target_os = "ios")))]
pub type GLhandleARB = c_uint;
pub type GLhalf = u16;
pub type GLhalfARB = u16;
pub type GLfixed = i32;
pub type GLintptr = isize;
pub type GLintptrARB = isize;
pub type GLsizeiptr = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64 = i64;
pub type GLint64EXT = i64;
pub type GLuint64 = u64;
pub type GLuint64EXT = u64;
#[doc(hidden)]
pub struct __GLsync {
    _priv: u8,
}
impl core::fmt::Debug for __GLsync {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "__GLsync")
    }
}
pub type GLsync = *mut __GLsync;
pub struct _cl_context {
    _priv: u8,
}
impl core::fmt::Debug for _cl_context {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "_cl_context")
    }
}
pub struct _cl_event {
    _priv: u8,
}
impl core::fmt::Debug for _cl_event {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "_cl_event")
    }
}
pub type GLDEBUGPROC = Option<
    unsafe extern "system" fn(
        source: GLenum,
        gltype: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut c_void,
    ),
>;
pub type GLDEBUGPROCARB = Option<
    extern "system" fn(
        source: GLenum,
        gltype: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut c_void,
    ),
>;
pub type GLDEBUGPROCKHR = Option<
    extern "system" fn(
        source: GLenum,
        gltype: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut c_void,
    ),
>;
pub type GLDEBUGPROCAMD = Option<
    extern "system" fn(
        id: GLuint,
        category: GLenum,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut c_void,
    ),
>;
pub type GLhalfNV = c_ushort;
pub type GLvdpauSurfaceNV = GLintptr;
pub type GLVULKANPROCNV = Option<extern "system" fn()>;
