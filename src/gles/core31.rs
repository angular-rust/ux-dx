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
    pub use crate::gles::core30::gl::*;

// glActiveShaderProgram
// glBindImageTexture
// glBindProgramPipeline
// glBindVertexBuffer
// glCreateShaderProgramv
// glDeleteProgramPipelines
// glDispatchCompute
// glDispatchComputeIndirect
// glDrawArraysIndirect
// glDrawElementsIndirect
// glFramebufferParameteri
// glGenProgramPipelines
// glGetBooleani_v
// glGetFramebufferParameteriv
// glGetMultisamplefv
// glGetProgramInterfaceiv
// glGetProgramPipelineiv
// glGetProgramResourceiv
// glGetProgramResourceIndex
// glGetProgramResourceLocation
// glGetProgramResourceName
// glGetTexLevelParameterfv
// glGetTexLevelParameteriv
// glIsProgramPipeline
// glMemoryBarrier
// glMemoryBarrierByRegion
// glProgramUniform1f
// glProgramUniform2f
// glProgramUniform3f
// glProgramUniform4f
// glProgramUniform1i
// glProgramUniform2i
// glProgramUniform3i
// glProgramUniform4i
// glProgramUniform1ui
// glProgramUniform2ui
// glProgramUniform3ui
// glProgramUniform4ui
// glProgramUniform1fv
// glProgramUniform2fv
// glProgramUniform3fv
// glProgramUniform4fv
// glProgramUniform1iv
// glProgramUniform2iv
// glProgramUniform3iv
// glProgramUniform4iv
// glProgramUniform1uiv
// glProgramUniform2uiv
// glProgramUniform3uiv
// glProgramUniform4uiv
// glProgramUniformMatrix2fv
// glProgramUniformMatrix3fv
// glProgramUniformMatrix4fv
// glProgramUniformMatrix2x3fv
// glProgramUniformMatrix3x2fv
// glProgramUniformMatrix2x4fv
// glProgramUniformMatrix4x2fv
// glProgramUniformMatrix3x4fv
// glProgramUniformMatrix4x3fv
// glSampleMaski
// glTexStorage2DMultisample
// glUseProgramStages
// glValidateProgramPipeline
// glVertexAttribBinding
// glVertexAttribFormat
// glVertexAttribIFormat
// glVertexBindingDivisor
}