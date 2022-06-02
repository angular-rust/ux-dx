//! OpenGL ES 3.2 Core support

// #![no_std]
#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    unused_imports,
    improper_ctypes
)]

use std::{
    ffi::{CStr, CString},
    mem::size_of,
    ptr,
    str::from_utf8,
};

use libc::c_char;

use super::ffi;
use crate::gles::{
    enums::{GL_RGBA, GL_TRUE, GL_UNSIGNED_BYTE},
    types::*,
};

pub mod gl {
    use super::*;

    // module-cascade infrastructure
    pub use crate::gles::core31::gl::*;
// glBlendBarrier
// glBlendEquationi
// glBlendEquationSeparatei
// glBlendFunci
// glBlendFuncSeparatei
// glColorMaski
// glCopyImageSubData
// glDebugMessageCallback
// glDebugMessageControl
// glDebugMessageInsert
// glDisablei
// glDrawElementsBaseVertex
// glDrawElementsInstancedBaseVertex
// glDrawRangeElementsBaseVertex
// glEnablei
// glFramebufferTexture
// glGetDebugMessageLog
// glGetGraphicsResetStatus
// glGetnUniformfv
// glGetnUniformiv
// glGetnUniformuiv
// glGetObjectLabel
// glGetObjectPtrLabel
// glGetPointerv
// glGetSamplerParameterIiv
// glGetSamplerParameterIuiv
// glGetTexParameterIiv
// glGetTexParameterIuiv
// glGetnUniformfv
// glGetnUniformiv
// glGetnUniformuiv
// glIsEnabledi
// glMinSampleShading
// glObjectLabel
// glObjectPtrLabel
// glPatchParameteri
// glPopDebugGroup
// glPrimitiveBoundingBox
// glPushDebugGroup
// glReadnPixels
// glSamplerParameterIiv
// glSamplerParameterIuiv
// glTexBuffer
// glTexBufferRange
// glTexParameterIiv
// glTexParameterIuiv
// glTexStorage3DMultisample
}