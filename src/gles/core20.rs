#![allow(dead_code)]

// -------------------------------------------------------------------------------------------------
// DEPENDENCIES
// -------------------------------------------------------------------------------------------------

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

    // // -------------------------------------------------------------------------------------------------
    // // TYPES
    // // -------------------------------------------------------------------------------------------------

    // pub type GLbitfield = c_uint;
    // pub type GLboolean = c_uchar;
    // pub type GLbyte = khronos_int8_t;
    // pub type GLchar = c_char;
    // pub type GLclampf = khronos_float_t;
    // pub type GLenum = c_uint;
    // pub type GLfixed = khronos_int32_t;
    // pub type GLfloat = khronos_float_t;
    // pub type GLint = c_int;
    // pub type GLshort = c_short;
    // pub type GLsizei = c_int;
    // pub type GLubyte = khronos_uint8_t;
    // pub type GLuint = c_uint;
    // pub type GLushort = c_ushort;
    // pub type GLvoid = c_void;
    // pub type GLintptr = khronos_intptr_t;
    // pub type GLsizeiptr = khronos_ssize_t;

    // -------------------------------------------------------------------------------------------------
    // STRUCTS
    // -------------------------------------------------------------------------------------------------

    pub struct Active {
        pub name: String,
        pub size: GLint,
        pub type_: GLenum,
    }

    pub struct ShaderPrecisionFormat {
        pub precision: GLint,
        pub range: [GLint; 2],
    }

    // // -------------------------------------------------------------------------------------------------
    // // CONSTANTS
    // // -------------------------------------------------------------------------------------------------

    // // OpenGL ES core versions
    // pub const ES_VERSION_2_0: GLint = 1;

    // // ClearBufferMask
    // pub const DEPTH_BUFFER_BIT: GLuint = 0x00000100;
    // pub const STENCIL_BUFFER_BIT: GLuint = 0x00000400;
    // pub const COLOR_BUFFER_BIT: GLuint = 0x00004000;

    // // boolean
    // pub const FALSE: GLboolean = 0;
    // pub const TRUE: GLboolean = 1;

    // // BeginMode
    // pub const POINTS: GLuint = 0x0000;
    // pub const LINES: GLuint = 0x0001;
    // pub const LINE_LOOP: GLuint = 0x0002;
    // pub const LINE_STRIP: GLuint = 0x0003;
    // pub const TRIANGLES: GLuint = 0x0004;
    // pub const TRIANGLE_STRIP: GLuint = 0x0005;
    // pub const TRIANGLE_FAN: GLuint = 0x0006;

    // // AlphaFunction(not supported in ES20)
    // //      GL_NEVER
    // //      GL_LESS
    // //      GL_EQUAL
    // //      GL_LEQUAL
    // //      GL_GREATER
    // //      GL_NOTEQUAL
    // //      GL_GEQUAL
    // //      GL_ALWAYS

    // // BlendingFactorDest
    // pub const ZERO: GLuint = 0;
    // pub const ONE: GLuint = 1;
    // pub const SRC_COLOR: GLuint = 0x0300;
    // pub const ONE_MINUS_SRC_COLOR: GLuint = 0x0301;
    // pub const SRC_ALPHA: GLuint = 0x0302;
    // pub const ONE_MINUS_SRC_ALPHA: GLuint = 0x0303;
    // pub const DST_ALPHA: GLuint = 0x0304;
    // pub const ONE_MINUS_DST_ALPHA: GLuint = 0x0305;

    // // BlendingFactorSrc
    // //      GL_ZERO
    // //      GL_ONE
    // pub const DST_COLOR: GLuint = 0x0306;
    // pub const ONE_MINUS_DST_COLOR: GLuint = 0x0307;
    // pub const SRC_ALPHA_SATURATE: GLuint = 0x0308;
    // //      GL_SRC_ALPHA
    // //      GL_ONE_MINUS_SRC_ALPHA
    // //      GL_DST_ALPHA
    // //      GL_ONE_MINUS_DST_ALPHA

    // // BlendEquationSeparate
    // pub const FUNC_ADD: GLuint = 0x8006;
    // pub const BLEND_EQUATION: GLuint = 0x8009;
    // pub const BLEND_EQUATION_RGB: GLuint = 0x8009; // same as BLEND_EQUATION
    // pub const BLEND_EQUATION_ALPHA: GLuint = 0x883D;

    // // BlendSubtract
    // pub const FUNC_SUBTRACT: GLuint = 0x800A;
    // pub const FUNC_REVERSE_SUBTRACT: GLuint = 0x800B;

    // // Separate Blend Functions
    // pub const BLEND_DST_RGB: GLuint = 0x80C8;
    // pub const BLEND_SRC_RGB: GLuint = 0x80C9;
    // pub const BLEND_DST_ALPHA: GLuint = 0x80CA;
    // pub const BLEND_SRC_ALPHA: GLuint = 0x80CB;
    // pub const CONSTANT_COLOR: GLuint = 0x8001;
    // pub const ONE_MINUS_CONSTANT_COLOR: GLuint = 0x8002;
    // pub const CONSTANT_ALPHA: GLuint = 0x8003;
    // pub const ONE_MINUS_CONSTANT_ALPHA: GLuint = 0x8004;
    // pub const BLEND_COLOR: GLuint = 0x8005;

    // // Buffer Objects
    // pub const ARRAY_BUFFER: GLuint = 0x8892;
    // pub const ELEMENT_ARRAY_BUFFER: GLuint = 0x8893;
    // pub const ARRAY_BUFFER_BINDING: GLuint = 0x8894;
    // pub const ELEMENT_ARRAY_BUFFER_BINDING: GLuint = 0x8895;

    // pub const STREAM_DRAW: GLuint = 0x88E0;
    // pub const STATIC_DRAW: GLuint = 0x88E4;
    // pub const DYNAMIC_DRAW: GLuint = 0x88E8;

    // pub const BUFFER_SIZE: GLuint = 0x8764;
    // pub const BUFFER_USAGE: GLuint = 0x8765;

    // pub const CURRENT_VERTEX_ATTRIB: GLuint = 0x8626;

    // // CullFaceMode
    // pub const FRONT: GLuint = 0x0404;
    // pub const BACK: GLuint = 0x0405;
    // pub const FRONT_AND_BACK: GLuint = 0x0408;

    // // DepthFunction
    // //      GL_NEVER
    // //      GL_LESS
    // //      GL_EQUAL
    // //      GL_LEQUAL
    // //      GL_GREATER
    // //      GL_NOTEQUAL
    // //      GL_GEQUAL
    // //      GL_ALWAYS

    // // EnableCap
    // pub const TEXTURE_2D: GLuint = 0x0DE1;
    // pub const CULL_FACE: GLuint = 0x0B44;
    // pub const BLEND: GLuint = 0x0BE2;
    // pub const DITHER: GLuint = 0x0BD0;
    // pub const STENCIL_TEST: GLuint = 0x0B90;
    // pub const DEPTH_TEST: GLuint = 0x0B71;
    // pub const SCISSOR_TEST: GLuint = 0x0C11;
    // pub const POLYGON_OFFSET_FILL: GLuint = 0x8037;
    // pub const SAMPLE_ALPHA_TO_COVERAGE: GLuint = 0x809E;
    // pub const SAMPLE_COVERAGE: GLuint = 0x80A0;

    // // ErrorCode
    // pub const NO_ERROR: GLuint = 0;
    // pub const INVALID_ENUM: GLuint = 0x0500;
    // pub const INVALID_VALUE: GLuint = 0x0501;
    // pub const INVALID_OPERATION: GLuint = 0x0502;
    // pub const OUT_OF_MEMORY: GLuint = 0x0505;

    // // FrontFaceDirection
    // pub const CW: GLint = 0x0900;
    // pub const CCW: GLint = 0x0901;

    // // GetPName
    // pub const LINE_WIDTH: GLuint = 0x0B21;
    // pub const ALIASED_POINT_SIZE_RANGE: GLuint = 0x846D;
    // pub const ALIASED_LINE_WIDTH_RANGE: GLuint = 0x846E;
    // pub const CULL_FACE_MODE: GLuint = 0x0B45;
    // pub const FRONT_FACE: GLuint = 0x0B46;
    // pub const DEPTH_RANGE: GLuint = 0x0B70;
    // pub const DEPTH_WRITEMASK: GLuint = 0x0B72;
    // pub const DEPTH_CLEAR_VALUE: GLuint = 0x0B73;
    // pub const DEPTH_FUNC: GLuint = 0x0B74;
    // pub const STENCIL_CLEAR_VALUE: GLuint = 0x0B91;
    // pub const STENCIL_FUNC: GLuint = 0x0B92;
    // pub const STENCIL_FAIL: GLuint = 0x0B94;
    // pub const STENCIL_PASS_DEPTH_FAIL: GLuint = 0x0B95;
    // pub const STENCIL_PASS_DEPTH_PASS: GLuint = 0x0B96;
    // pub const STENCIL_REF: GLuint = 0x0B97;
    // pub const STENCIL_VALUE_MASK: GLuint = 0x0B93;
    // pub const STENCIL_WRITEMASK: GLuint = 0x0B98;
    // pub const STENCIL_BACK_FUNC: GLuint = 0x8800;
    // pub const STENCIL_BACK_FAIL: GLuint = 0x8801;
    // pub const STENCIL_BACK_PASS_DEPTH_FAIL: GLuint = 0x8802;
    // pub const STENCIL_BACK_PASS_DEPTH_PASS: GLuint = 0x8803;
    // pub const STENCIL_BACK_REF: GLuint = 0x8CA3;
    // pub const STENCIL_BACK_VALUE_MASK: GLuint = 0x8CA4;
    // pub const STENCIL_BACK_WRITEMASK: GLuint = 0x8CA5;
    // pub const VIEWPORT: GLuint = 0x0BA2;
    // pub const SCISSOR_BOX: GLuint = 0x0C10;
    // //      GL_SCISSOR_TEST
    // pub const COLOR_CLEAR_VALUE: GLuint = 0x0C22;
    // pub const COLOR_WRITEMASK: GLuint = 0x0C23;
    // pub const UNPACK_ALIGNMENT: GLuint = 0x0CF5;
    // pub const PACK_ALIGNMENT: GLuint = 0x0D05;
    // pub const MAX_TEXTURE_SIZE: GLuint = 0x0D33;
    // pub const MAX_VIEWPORT_DIMS: GLuint = 0x0D3A;
    // pub const SUBPIXEL_BITS: GLuint = 0x0D50;
    // pub const RED_BITS: GLuint = 0x0D52;
    // pub const GREEN_BITS: GLuint = 0x0D53;
    // pub const BLUE_BITS: GLuint = 0x0D54;
    // pub const ALPHA_BITS: GLuint = 0x0D55;
    // pub const DEPTH_BITS: GLuint = 0x0D56;
    // pub const STENCIL_BITS: GLuint = 0x0D57;
    // pub const POLYGON_OFFSET_UNITS: GLuint = 0x2A00;
    // //      GL_POLYGON_OFFSET_FILL
    // pub const POLYGON_OFFSET_FACTOR: GLuint = 0x8038;
    // pub const TEXTURE_BINDING_2D: GLuint = 0x8069;
    // pub const SAMPLE_BUFFERS: GLuint = 0x80A8;
    // pub const SAMPLES: GLuint = 0x80A9;
    // pub const SAMPLE_COVERAGE_VALUE: GLuint = 0x80AA;
    // pub const SAMPLE_COVERAGE_INVERT: GLuint = 0x80AB;

    // // GetTextureParameter
    // //      GL_TEXTURE_MAG_FILTER
    // //      GL_TEXTURE_MIN_FILTER
    // //      GL_TEXTURE_WRAP_S
    // //      GL_TEXTURE_WRAP_T

    // pub const NUM_COMPRESSED_TEXTURE_FORMATS: GLuint = 0x86A2;
    // pub const COMPRESSED_TEXTURE_FORMATS: GLuint = 0x86A3;

    // // HintMode
    // pub const DONT_CARE: GLuint = 0x1100;
    // pub const FASTEST: GLuint = 0x1101;
    // pub const NICEST: GLuint = 0x1102;

    // // HintTarget
    // pub const GENERATE_MIPMAP_HINT: GLint = 0x8192;

    // // DataType
    // pub const BYTE: GLuint = 0x1400;
    // pub const UNSIGNED_BYTE: GLuint = 0x1401;
    // pub const SHORT: GLuint = 0x1402;
    // pub const UNSIGNED_SHORT: GLuint = 0x1403;
    // pub const INT: GLuint = 0x1404;
    // pub const UNSIGNED_INT: GLuint = 0x1405;
    // pub const FLOAT: GLuint = 0x1406;
    // pub const FIXED: GLuint = 0x140C;

    // // PixelFormat
    // pub const DEPTH_COMPONENT: GLuint = 0x1902;
    // pub const ALPHA: GLuint = 0x1906;
    // pub const RGB: GLuint = 0x1907;
    // pub const RGBA: GLuint = 0x1908;
    // pub const LUMINANCE: GLuint = 0x1909;
    // pub const LUMINANCE_ALPHA: GLuint = 0x190A;

    // // PixelType
    // //      GL_UNSIGNED_BYTE
    // pub const UNSIGNED_SHORT_4_4_4_4: GLuint = 0x8033;
    // pub const UNSIGNED_SHORT_5_5_5_1: GLuint = 0x8034;
    // pub const UNSIGNED_SHORT_5_6_5: GLuint = 0x8363;

    // // Shaders
    // pub const FRAGMENT_SHADER: GLuint = 0x8B30;
    // pub const VERTEX_SHADER: GLuint = 0x8B31;
    // pub const MAX_VERTEX_ATTRIBS: GLuint = 0x8869;
    // pub const MAX_VERTEX_UNIFORM_VECTORS: GLuint = 0x8DFB;
    // pub const MAX_VARYING_VECTORS: GLuint = 0x8DFC;
    // pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLuint = 0x8B4D;
    // pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLuint = 0x8B4C;
    // pub const MAX_TEXTURE_IMAGE_UNITS: GLuint = 0x8872;
    // pub const MAX_FRAGMENT_UNIFORM_VECTORS: GLuint = 0x8DFD;
    // pub const SHADER_TYPE: GLuint = 0x8B4F;
    // pub const DELETE_STATUS: GLuint = 0x8B80;
    // pub const LINK_STATUS: GLuint = 0x8B82;
    // pub const VALIDATE_STATUS: GLuint = 0x8B83;
    // pub const ATTACHED_SHADERS: GLuint = 0x8B85;
    // pub const ACTIVE_UNIFORMS: GLuint = 0x8B86;
    // pub const ACTIVE_UNIFORM_MAX_LENGTH: GLuint = 0x8B87;
    // pub const ACTIVE_ATTRIBUTES: GLuint = 0x8B89;
    // pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: GLuint = 0x8B8A;
    // pub const SHADING_LANGUAGE_VERSION: GLuint = 0x8B8C;
    // pub const CURRENT_PROGRAM: GLuint = 0x8B8D;

    // // StencilFunction
    // pub const NEVER: GLuint = 0x0200;
    // pub const LESS: GLuint = 0x0201;
    // pub const EQUAL: GLuint = 0x0202;
    // pub const LEQUAL: GLuint = 0x0203;
    // pub const GREATER: GLuint = 0x0204;
    // pub const NOTEQUAL: GLuint = 0x0205;
    // pub const GEQUAL: GLuint = 0x0206;
    // pub const ALWAYS: GLuint = 0x0207;

    // // StencilOp
    // //      GL_ZERO
    // pub const KEEP: GLuint = 0x1E00;
    // pub const REPLACE: GLuint = 0x1E01;
    // pub const INCR: GLuint = 0x1E02;
    // pub const DECR: GLuint = 0x1E03;
    // pub const INVERT: GLuint = 0x150A;
    // pub const INCR_WRAP: GLuint = 0x8507;
    // pub const DECR_WRAP: GLuint = 0x8508;

    // // StringName
    // pub const VENDOR: GLuint = 0x1F00;
    // pub const RENDERER: GLuint = 0x1F01;
    // pub const VERSION: GLuint = 0x1F02;
    // pub const EXTENSIONS: GLuint = 0x1F03;

    // // TextureMagFilter
    // pub const NEAREST: GLuint = 0x2600;
    // pub const LINEAR: GLuint = 0x2601;

    // // TextureMinFilter
    // //      GL_NEAREST
    // //      GL_LINEAR
    // pub const NEAREST_MIPMAP_NEAREST: GLuint = 0x2700;
    // pub const LINEAR_MIPMAP_NEAREST: GLuint = 0x2701;
    // pub const NEAREST_MIPMAP_LINEAR: GLuint = 0x2702;
    // pub const LINEAR_MIPMAP_LINEAR: GLuint = 0x2703;

    // // TextureParameterName
    // pub const TEXTURE_MAG_FILTER: GLuint = 0x2800;
    // pub const TEXTURE_MIN_FILTER: GLuint = 0x2801;
    // pub const TEXTURE_WRAP_S: GLuint = 0x2802;
    // pub const TEXTURE_WRAP_T: GLuint = 0x2803;

    // // TextureTarget
    // //      GL_TEXTURE_2D
    // pub const TEXTURE: GLuint = 0x1702;
    // pub const TEXTURE_CUBE_MAP: GLuint = 0x8513;
    // pub const TEXTURE_BINDING_CUBE_MAP: GLuint = 0x8514;
    // pub const TEXTURE_CUBE_MAP_POSITIVE_X: GLuint = 0x8515;
    // pub const TEXTURE_CUBE_MAP_NEGATIVE_X: GLuint = 0x8516;
    // pub const TEXTURE_CUBE_MAP_POSITIVE_Y: GLuint = 0x8517;
    // pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: GLuint = 0x8518;
    // pub const TEXTURE_CUBE_MAP_POSITIVE_Z: GLuint = 0x8519;
    // pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: GLuint = 0x851A;
    // pub const MAX_CUBE_MAP_TEXTURE_SIZE: GLuint = 0x851C;

    // // TextureUnit
    // pub const TEXTURE0: GLuint = 0x84C0;
    // pub const TEXTURE1: GLuint = 0x84C1;
    // pub const TEXTURE2: GLuint = 0x84C2;
    // pub const TEXTURE3: GLuint = 0x84C3;
    // pub const TEXTURE4: GLuint = 0x84C4;
    // pub const TEXTURE5: GLuint = 0x84C5;
    // pub const TEXTURE6: GLuint = 0x84C6;
    // pub const TEXTURE7: GLuint = 0x84C7;
    // pub const TEXTURE8: GLuint = 0x84C8;
    // pub const TEXTURE9: GLuint = 0x84C9;
    // pub const TEXTURE10: GLuint = 0x84CA;
    // pub const TEXTURE11: GLuint = 0x84CB;
    // pub const TEXTURE12: GLuint = 0x84CC;
    // pub const TEXTURE13: GLuint = 0x84CD;
    // pub const TEXTURE14: GLuint = 0x84CE;
    // pub const TEXTURE15: GLuint = 0x84CF;
    // pub const TEXTURE16: GLuint = 0x84D0;
    // pub const TEXTURE17: GLuint = 0x84D1;
    // pub const TEXTURE18: GLuint = 0x84D2;
    // pub const TEXTURE19: GLuint = 0x84D3;
    // pub const TEXTURE20: GLuint = 0x84D4;
    // pub const TEXTURE21: GLuint = 0x84D5;
    // pub const TEXTURE22: GLuint = 0x84D6;
    // pub const TEXTURE23: GLuint = 0x84D7;
    // pub const TEXTURE24: GLuint = 0x84D8;
    // pub const TEXTURE25: GLuint = 0x84D9;
    // pub const TEXTURE26: GLuint = 0x84DA;
    // pub const TEXTURE27: GLuint = 0x84DB;
    // pub const TEXTURE28: GLuint = 0x84DC;
    // pub const TEXTURE29: GLuint = 0x84DD;
    // pub const TEXTURE30: GLuint = 0x84DE;
    // pub const TEXTURE31: GLuint = 0x84DF;
    // pub const ACTIVE_TEXTURE: GLuint = 0x84E0;

    // // TextureWrapMode
    // pub const REPEAT: GLuint = 0x2901;
    // pub const CLAMP_TO_EDGE: GLuint = 0x812F;
    // pub const MIRRORED_REPEAT: GLuint = 0x8370;

    // // Uniform Types
    // pub const FLOAT_VEC2: GLuint = 0x8B50;
    // pub const FLOAT_VEC3: GLuint = 0x8B51;
    // pub const FLOAT_VEC4: GLuint = 0x8B52;
    // pub const INT_VEC2: GLuint = 0x8B53;
    // pub const INT_VEC3: GLuint = 0x8B54;
    // pub const INT_VEC4: GLuint = 0x8B55;
    // pub const BOOL: GLuint = 0x8B56;
    // pub const BOOL_VEC2: GLuint = 0x8B57;
    // pub const BOOL_VEC3: GLuint = 0x8B58;
    // pub const BOOL_VEC4: GLuint = 0x8B59;
    // pub const FLOAT_MAT2: GLuint = 0x8B5A;
    // pub const FLOAT_MAT3: GLuint = 0x8B5B;
    // pub const FLOAT_MAT4: GLuint = 0x8B5C;
    // pub const SAMPLER_2D: GLuint = 0x8B5E;
    // pub const SAMPLER_CUBE: GLuint = 0x8B60;

    // // Vertex Arrays
    // pub const VERTEX_ATTRIB_ARRAY_ENABLED: GLuint = 0x8622;
    // pub const VERTEX_ATTRIB_ARRAY_SIZE: GLuint = 0x8623;
    // pub const VERTEX_ATTRIB_ARRAY_STRIDE: GLuint = 0x8624;
    // pub const VERTEX_ATTRIB_ARRAY_TYPE: GLuint = 0x8625;
    // pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: GLuint = 0x886A;
    // pub const VERTEX_ATTRIB_ARRAY_POINTER: GLuint = 0x8645;
    // pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLuint = 0x889F;

    // // Read Format
    // pub const IMPLEMENTATION_COLOR_READ_TYPE: GLuint = 0x8B9A;
    // pub const IMPLEMENTATION_COLOR_READ_FORMAT: GLuint = 0x8B9B;

    // // Shader Source
    // pub const COMPILE_STATUS: GLuint = 0x8B81;
    // pub const INFO_LOG_LENGTH: GLuint = 0x8B84;
    // pub const SHADER_SOURCE_LENGTH: GLuint = 0x8B88;
    // pub const SHADER_COMPILER: GLuint = 0x8DFA;

    // // Shader Binary
    // pub const SHADER_BINARY_FORMATS: GLuint = 0x8DF8;
    // pub const NUM_SHADER_BINARY_FORMATS: GLuint = 0x8DF9;

    // // Shader Precision-Specified Types
    // pub const LOW_FLOAT: GLuint = 0x8DF0;
    // pub const MEDIUM_FLOAT: GLuint = 0x8DF1;
    // pub const HIGH_FLOAT: GLuint = 0x8DF2;
    // pub const LOW_INT: GLuint = 0x8DF3;
    // pub const MEDIUM_INT: GLuint = 0x8DF4;
    // pub const HIGH_INT: GLuint = 0x8DF5;

    // // Framebuffer Object.
    // pub const FRAMEBUFFER: GLuint = 0x8D40;
    // pub const RENDERBUFFER: GLuint = 0x8D41;

    // pub const RGBA4: GLuint = 0x8056;
    // pub const RGB5_A1: GLuint = 0x8057;
    // pub const RGB565: GLuint = 0x8D62;
    // pub const DEPTH_COMPONENT16: GLuint = 0x81A5;
    // pub const STENCIL_INDEX: GLuint = 0x1901;
    // pub const STENCIL_INDEX8: GLuint = 0x8D48;

    // pub const RENDERBUFFER_WIDTH: GLuint = 0x8D42;
    // pub const RENDERBUFFER_HEIGHT: GLuint = 0x8D43;
    // pub const RENDERBUFFER_INTERNAL_FORMAT: GLuint = 0x8D44;
    // pub const RENDERBUFFER_RED_SIZE: GLuint = 0x8D50;
    // pub const RENDERBUFFER_GREEN_SIZE: GLuint = 0x8D51;
    // pub const RENDERBUFFER_BLUE_SIZE: GLuint = 0x8D52;
    // pub const RENDERBUFFER_ALPHA_SIZE: GLuint = 0x8D53;
    // pub const RENDERBUFFER_DEPTH_SIZE: GLuint = 0x8D54;
    // pub const RENDERBUFFER_STENCIL_SIZE: GLuint = 0x8D55;

    // pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLuint = 0x8CD0;
    // pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLuint = 0x8CD1;
    // pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLuint = 0x8CD2;
    // pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLuint = 0x8CD3;

    // pub const COLOR_ATTACHMENT0: GLuint = 0x8CE0;
    // pub const DEPTH_ATTACHMENT: GLuint = 0x8D00;
    // pub const STENCIL_ATTACHMENT: GLuint = 0x8D20;

    // pub const NONE: GLuint = 0;

    // pub const FRAMEBUFFER_COMPLETE: GLuint = 0x8CD5;
    // pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLuint = 0x8CD6;
    // pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLuint = 0x8CD7;
    // pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: GLuint = 0x8CD9;
    // pub const FRAMEBUFFER_UNSUPPORTED: GLuint = 0x8CDD;

    // pub const FRAMEBUFFER_BINDING: GLuint = 0x8CA6;
    // pub const RENDERBUFFER_BINDING: GLuint = 0x8CA7;
    // pub const MAX_RENDERBUFFER_SIZE: GLuint = 0x84E8;

    // pub const INVALID_FRAMEBUFFER_OPERATION: GLuint = 0x0506;

    // -------------------------------------------------------------------------------------------------
    // FUNCTIONS
    // -------------------------------------------------------------------------------------------------

    pub fn active_texture(texture: GLenum) {
        unsafe { ffi::glActiveTexture(texture) }
    }

    pub fn attach_shader(program: GLuint, shader: GLuint) {
        unsafe { ffi::glAttachShader(program, shader) }
    }

    pub fn bind_attrib_location(program: GLuint, index: GLuint, name: &str) {
        unsafe {
            let c_str = CString::new(name).unwrap();

            ffi::glBindAttribLocation(program, index, c_str.as_ptr() as *const c_char)
        }
    }

    pub fn bind_buffer(target: GLenum, buffer: GLuint) {
        unsafe { ffi::glBindBuffer(target, buffer) }
    }

    pub fn bind_framebuffer(target: GLenum, framebuffer: GLuint) {
        unsafe { ffi::glBindFramebuffer(target, framebuffer) }
    }

    pub fn bind_renderbuffer(target: GLenum, renderbuffer: GLuint) {
        unsafe { ffi::glBindRenderbuffer(target, renderbuffer) }
    }

    pub fn bind_texture(target: GLenum, texture: GLuint) {
        unsafe { ffi::glBindTexture(target, texture) }
    }

    pub fn blend_color(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        unsafe { ffi::glBlendColor(red, green, blue, alpha) }
    }

    pub fn blend_equation(mode: GLenum) {
        unsafe { ffi::glBlendEquation(mode) }
    }

    pub fn blend_equation_separate(mode_rgb: GLenum, mode_alpha: GLenum) {
        unsafe { ffi::glBlendEquationSeparate(mode_rgb, mode_alpha) }
    }

    pub fn blend_func(src_factor: GLenum, dst_factor: GLenum) {
        unsafe { ffi::glBlendFunc(src_factor, dst_factor) }
    }

    pub fn blend_func_separate(
        src_rgb: GLenum,
        dst_rgb: GLenum,
        src_alpha: GLenum,
        dst_alpha: GLenum,
    ) {
        unsafe { ffi::glBlendFuncSeparate(src_rgb, dst_rgb, src_alpha, dst_alpha) }
    }

    pub fn buffer_data_size(target: GLenum, size: usize, usage: GLenum) {
        unsafe { ffi::glBufferData(target, size as GLsizeiptr, ptr::null(), usage) }
    }

    pub fn buffer_data<T>(target: GLenum, buffer: &[T], usage: GLenum) {
        unsafe {
            ffi::glBufferData(
                target,
                (buffer.len() * size_of::<T>()) as GLsizeiptr,
                buffer.as_ptr() as *const GLvoid,
                usage,
            )
        }
    }

    pub fn buffer_sub_data<T>(target: GLenum, offset: GLintptr, buffer: &[T]) {
        unsafe {
            let size = size_of::<T>();

            ffi::glBufferSubData(
                target,
                offset * size as GLintptr,
                (buffer.len() * size) as GLsizeiptr,
                buffer.as_ptr() as *const GLvoid,
            )
        }
    }

    pub fn check_framebuffer_status(target: GLenum) -> GLenum {
        unsafe { ffi::glCheckFramebufferStatus(target) }
    }

    pub fn clear(mask: GLbitfield) {
        unsafe { ffi::glClear(mask) }
    }

    pub fn clear_color(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        unsafe { ffi::glClearColor(red, green, blue, alpha) }
    }

    pub fn clear_depthf(depth: GLclampf) {
        unsafe { ffi::glClearDepthf(depth) }
    }

    pub fn clear_stencil(stencil: GLint) {
        unsafe { ffi::glClearStencil(stencil) }
    }

    pub fn color_mask(red: bool, green: bool, blue: bool, alpha: bool) {
        unsafe {
            ffi::glColorMask(
                red as GLboolean,
                green as GLboolean,
                blue as GLboolean,
                alpha as GLboolean,
            )
        }
    }

    pub fn compile_shader(shader: GLuint) {
        unsafe { ffi::glCompileShader(shader) }
    }

    pub fn compressed_tex_image_2d<T>(
        target: GLenum,
        level: GLint,
        internal_format: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        image_size: GLsizei,
        buffer: &[T],
    ) {
        unsafe {
            ffi::glCompressedTexImage2D(
                target,
                level,
                internal_format,
                width,
                height,
                border,
                image_size,
                buffer.as_ptr() as *const GLvoid,
            )
        }
    }

    pub fn compressed_tex_sub_image_2d<T>(
        target: GLenum,
        level: GLint,
        x_offset: GLint,
        y_offset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        image_size: GLsizei,
        buffer: &[T],
    ) {
        unsafe {
            ffi::glCompressedTexSubImage2D(
                target,
                level,
                x_offset,
                y_offset,
                width,
                height,
                format,
                image_size,
                buffer.as_ptr() as *const GLvoid,
            )
        }
    }

    pub fn copy_tex_image_2d(
        target: GLenum,
        level: GLint,
        internal_format: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
    ) {
        unsafe {
            ffi::glCopyTexImage2D(target, level, internal_format, x, y, width, height, border)
        }
    }

    pub fn copy_tex_sub_image_2d(
        target: GLenum,
        level: GLint,
        x_offset: GLint,
        y_offset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        unsafe { ffi::glCopyTexSubImage2D(target, level, x_offset, y_offset, x, y, width, height) }
    }

    pub fn create_program() -> GLuint {
        unsafe { ffi::glCreateProgram() }
    }

    pub fn create_shader(type_: GLenum) -> GLuint {
        unsafe { ffi::glCreateShader(type_) }
    }

    pub fn cull_face(mode: GLenum) {
        unsafe { ffi::glCullFace(mode) }
    }

    pub fn delete_buffers(buffers: &[GLuint]) {
        unsafe { ffi::glDeleteBuffers(buffers.len() as GLsizei, buffers.as_ptr()) }
    }

    pub fn delete_framebuffers(framebuffers: &[GLuint]) {
        unsafe { ffi::glDeleteFramebuffers(framebuffers.len() as GLsizei, framebuffers.as_ptr()) }
    }

    pub fn delete_program(program: GLuint) {
        unsafe { ffi::glDeleteProgram(program) }
    }

    pub fn delete_renderbuffers(renderbuffers: &[GLuint]) {
        unsafe {
            ffi::glDeleteRenderbuffers(renderbuffers.len() as GLsizei, renderbuffers.as_ptr())
        }
    }

    pub fn delete_shader(shader: GLuint) {
        unsafe { ffi::glDeleteShader(shader) }
    }

    pub fn delete_textures(textures: &[GLuint]) {
        unsafe { ffi::glDeleteTextures(textures.len() as GLsizei, textures.as_ptr()) }
    }

    pub fn depth_func(func: GLenum) {
        unsafe { ffi::glDepthFunc(func) }
    }

    pub fn depth_mask(flag: bool) {
        unsafe { ffi::glDepthMask(flag as GLboolean) }
    }

    pub fn depth_rangef(z_near: GLclampf, z_far: GLclampf) {
        unsafe { ffi::glDepthRangef(z_near, z_far) }
    }

    pub fn detach_shader(program: GLuint, shader: GLuint) {
        unsafe { ffi::glDetachShader(program, shader) }
    }

    pub fn disable(feature: GLenum) {
        unsafe { ffi::glDisable(feature) }
    }

    pub fn disable_vertex_attrib_array(index: GLuint) {
        unsafe { ffi::glDisableVertexAttribArray(index) }
    }

    pub fn draw_arrays(mode: GLenum, first: GLint, count: GLsizei) {
        unsafe { ffi::glDrawArrays(mode, first, count) }
    }

    pub fn draw_elements<T>(mode: GLenum, count: GLsizei, type_: GLenum, indices: &[T]) {
        unsafe { ffi::glDrawElements(mode, count, type_, indices.as_ptr() as *const GLvoid) }
    }

    pub fn draw_elements_offset(mode: GLenum, count: GLsizei, type_: GLenum, offset: GLuint) {
        unsafe { ffi::glDrawElements(mode, count, type_, offset as *const GLvoid) }
    }

    pub fn enable(feature: GLenum) {
        unsafe { ffi::glEnable(feature) }
    }

    pub fn enable_vertex_attrib_array(index: GLuint) {
        unsafe { ffi::glEnableVertexAttribArray(index) }
    }

    pub fn finish() {
        unsafe { ffi::glFinish() }
    }

    pub fn flush() {
        unsafe { ffi::glFlush() }
    }

    pub fn framebuffer_renderbuffer(
        target: GLenum,
        attachment: GLenum,
        renderbuffer_target: GLenum,
        renderbuffer: GLuint,
    ) {
        unsafe {
            ffi::glFramebufferRenderbuffer(target, attachment, renderbuffer_target, renderbuffer)
        }
    }

    pub fn framebuffer_texture_2d(
        target: GLenum,
        attachment: GLenum,
        texture_target: GLenum,
        texture: GLuint,
        level: GLint,
    ) {
        unsafe { ffi::glFramebufferTexture2D(target, attachment, texture_target, texture, level) }
    }

    pub fn front_face(mode: GLenum) {
        unsafe { ffi::glFrontFace(mode) }
    }

    #[inline]
    pub fn gen_buffer() -> GLuint {
        unsafe {
            let mut vec: Vec<GLuint> = Vec::with_capacity(1);

            ffi::glGenBuffers(1, vec.as_mut_ptr());

            vec.set_len(1);
            vec[0]
        }
    }

    pub fn gen_buffers(count: GLsizei) -> Vec<GLuint> {
        unsafe {
            let mut vec: Vec<GLuint> = Vec::with_capacity(count as usize);

            ffi::glGenBuffers(count, vec.as_mut_ptr());

            vec.set_len(count as usize);
            vec
        }
    }

    pub fn generate_mipmap(target: GLenum) {
        unsafe { ffi::glGenerateMipmap(target) }
    }

    #[inline]
    pub fn gen_framebuffer() -> GLuint {
        unsafe {
            let mut vec: Vec<GLuint> = Vec::with_capacity(1);

            ffi::glGenFramebuffers(1, vec.as_mut_ptr());

            vec.set_len(1);
            vec[0]
        }
    }

    pub fn gen_framebuffers(count: GLsizei) -> Vec<GLuint> {
        unsafe {
            let mut vec: Vec<GLuint> = Vec::with_capacity(count as usize);

            ffi::glGenFramebuffers(count, vec.as_mut_ptr());

            vec.set_len(count as usize);
            vec
        }
    }

    #[inline]
    pub fn gen_renderbuffer() -> GLuint {
        unsafe {
            let mut vec: Vec<GLuint> = Vec::with_capacity(1);

            ffi::glGenRenderbuffers(1, vec.as_mut_ptr());

            vec.set_len(1);
            vec[0]
        }
    }

    pub fn gen_renderbuffers(count: GLsizei) -> Vec<GLuint> {
        unsafe {
            let mut vec: Vec<GLuint> = Vec::with_capacity(count as usize);

            ffi::glGenRenderbuffers(count, vec.as_mut_ptr());

            vec.set_len(count as usize);
            vec
        }
    }

    #[inline]
    pub fn gen_texture() -> GLuint {
        unsafe {
            let mut vec: Vec<GLuint> = Vec::with_capacity(1);

            ffi::glGenTextures(1, vec.as_mut_ptr());

            vec.set_len(1);
            vec[0]
        }
    }

    pub fn gen_textures(count: GLsizei) -> Vec<GLuint> {
        unsafe {
            let mut vec: Vec<GLuint> = Vec::with_capacity(count as usize);

            ffi::glGenTextures(count, vec.as_mut_ptr());

            vec.set_len(count as usize);
            vec
        }
    }

    pub fn get_active_attrib(program: GLuint, index: GLuint) -> Option<Active> {
        unsafe {
            let mut length: GLsizei = 0;
            let mut size: GLint = 0;
            let mut type_: GLenum = 0;

            let mut name = String::with_capacity(256);

            ffi::glGetActiveAttrib(
                program,
                index,
                256,
                &mut length,
                &mut size,
                &mut type_,
                name.as_mut_vec().as_mut_ptr() as *mut GLchar,
            );

            if length > 0 {
                name.as_mut_vec().set_len(length as usize);
                name.truncate(length as usize);

                Some(Active {
                    name: name,
                    size: size,
                    type_: type_,
                })
            } else {
                None
            }
        }
    }

    pub fn get_active_uniform(program: GLuint, index: GLuint) -> Option<Active> {
        unsafe {
            let mut length: GLsizei = 0;
            let mut size: GLint = 0;
            let mut type_: GLenum = 0;

            let mut name = String::with_capacity(256);

            ffi::glGetActiveUniform(
                program,
                index,
                256,
                &mut length,
                &mut size,
                &mut type_,
                name.as_mut_vec().as_mut_ptr() as *mut GLchar,
            );

            if length > 0 {
                name.as_mut_vec().set_len(length as usize);
                name.truncate(length as usize);

                Some(Active {
                    name: name,
                    size: size,
                    type_: type_,
                })
            } else {
                None
            }
        }
    }

    pub fn get_attached_shaders(program: GLuint, max_count: GLsizei) -> Vec<GLuint> {
        unsafe {
            let mut count: GLsizei = 0;
            let mut vec: Vec<GLuint> = Vec::with_capacity(max_count as usize);

            ffi::glGetAttachedShaders(program, max_count, &mut count, vec.as_mut_ptr());

            vec.set_len(count as usize);
            vec.truncate(count as usize);
            vec
        }
    }

    pub fn get_attrib_location(program: GLuint, name: &str) -> GLint {
        unsafe {
            let c_str = CString::new(name).unwrap();

            ffi::glGetAttribLocation(program, c_str.as_ptr() as *const c_char)
        }
    }

    pub fn get_booleanv(name: GLenum) -> bool {
        unsafe {
            let mut value: GLboolean = 0;

            ffi::glGetBooleanv(name, &mut value);

            value == GL_TRUE
        }
    }

    pub fn get_buffer_parameteriv(target: GLenum, name: GLenum) -> GLint {
        unsafe {
            let mut value: GLint = 0;

            ffi::glGetBufferParameteriv(target, name, &mut value);

            value
        }
    }

    pub fn get_error() -> GLenum {
        unsafe { ffi::glGetError() }
    }

    pub fn get_floatv(name: GLenum) -> GLfloat {
        unsafe {
            let mut value: GLfloat = 0.0;

            ffi::glGetFloatv(name, &mut value);

            value
        }
    }

    pub fn get_framebuffer_attachment_parameteriv(
        target: GLenum,
        attachment: GLenum,
        name: GLenum,
    ) -> GLint {
        unsafe {
            let mut value: GLint = 0;

            ffi::glGetFramebufferAttachmentParameteriv(target, attachment, name, &mut value);

            value
        }
    }

    pub fn get_integerv(name: GLenum) -> GLint {
        unsafe {
            let mut value: GLint = 0;

            ffi::glGetIntegerv(name, &mut value);

            value
        }
    }

    pub fn get_programiv(program: GLuint, name: GLenum) -> GLint {
        unsafe {
            let mut value: GLint = 0;

            ffi::glGetProgramiv(program, name, &mut value);

            value
        }
    }

    pub fn get_program_info_log(program: GLuint, max_length: GLsizei) -> Option<String> {
        unsafe {
            let mut length: GLsizei = 0;
            let mut log = String::with_capacity(max_length as usize);

            ffi::glGetProgramInfoLog(
                program,
                max_length,
                &mut length,
                log.as_mut_vec().as_mut_ptr() as *mut i8,
            );

            if length > 0 {
                log.as_mut_vec().set_len(length as usize);
                log.truncate(length as usize);

                Some(log)
            } else {
                None
            }
        }
    }

    pub fn get_renderbuffer_parameteriv(target: GLenum, name: GLenum) -> GLint {
        unsafe {
            let mut value: GLint = 0;

            ffi::glGetRenderbufferParameteriv(target, name, &mut value);

            value
        }
    }

    pub fn get_shaderiv(shader: GLuint, name: GLenum) -> GLint {
        unsafe {
            let mut value: GLint = 0;

            ffi::glGetShaderiv(shader, name, &mut value);

            value
        }
    }

    pub fn get_shader_info_log(shader: GLuint, max_length: GLsizei) -> Option<String> {
        unsafe {
            let mut length: GLsizei = 0;
            let mut log = String::with_capacity(max_length as usize);

            ffi::glGetShaderInfoLog(
                shader,
                max_length,
                &mut length,
                log.as_mut_vec().as_mut_ptr() as *mut i8,
            );

            if length > 0 {
                log.as_mut_vec().set_len(length as usize);
                log.truncate(length as usize);

                Some(log)
            } else {
                None
            }
        }
    }

    pub fn get_shader_precision_format(
        shader_type: GLenum,
        precision_type: GLenum,
    ) -> ShaderPrecisionFormat {
        unsafe {
            let mut precision: GLint = 0;
            let mut range: [GLint; 2] = [0, 0];

            ffi::glGetShaderPrecisionFormat(
                shader_type,
                precision_type,
                range.as_mut_ptr(),
                &mut precision,
            );

            ShaderPrecisionFormat {
                precision: precision,
                range: range,
            }
        }
    }

    pub fn get_shader_source(shader: GLuint, max_length: GLsizei) -> Option<String> {
        unsafe {
            let mut length: GLsizei = 0;
            let mut source = String::with_capacity(max_length as usize);

            ffi::glGetShaderSource(
                shader,
                max_length,
                &mut length,
                source.as_mut_vec().as_mut_ptr() as *mut GLchar,
            );

            if length > 0 {
                source.as_mut_vec().set_len(length as usize);
                source.truncate(length as usize);

                Some(source)
            } else {
                None
            }
        }
    }

    pub fn get_string(name: GLenum) -> Option<String> {
        unsafe {
            let c_str = ffi::glGetString(name);

            if !c_str.is_null() {
                match from_utf8(CStr::from_ptr(c_str as *const i8).to_bytes()) {
                    Ok(s) => Some(s.to_string()),
                    Err(_) => None,
                }
            } else {
                None
            }
        }
    }

    pub fn get_tex_parameterfv(target: GLenum, name: GLenum) -> GLfloat {
        unsafe {
            let mut value: GLfloat = 0.0;

            ffi::glGetTexParameterfv(target, name, &mut value);

            value
        }
    }

    pub fn get_tex_parameteriv(target: GLenum, name: GLenum) -> GLint {
        unsafe {
            let mut value: GLint = 0;

            ffi::glGetTexParameteriv(target, name, &mut value);

            value
        }
    }

    pub fn get_uniformfv(program: GLuint, location: GLint) -> GLfloat {
        unsafe {
            let mut value: GLfloat = 0.0;

            ffi::glGetUniformfv(program, location, &mut value);

            value
        }
    }

    pub fn get_uniformiv(program: GLuint, location: GLint) -> GLint {
        unsafe {
            let mut value: GLint = 0;

            ffi::glGetUniformiv(program, location, &mut value);

            value
        }
    }

    pub fn get_uniform_location(program: GLuint, name: &str) -> GLint {
        unsafe {
            let c_str = CString::new(name).unwrap();

            ffi::glGetUniformLocation(program, c_str.as_ptr() as *const c_char)
        }
    }

    pub fn get_vertex_attribfv(index: GLuint, name: GLenum) -> GLfloat {
        unsafe {
            let mut value: GLfloat = 0.0;

            ffi::glGetVertexAttribfv(index, name, &mut value);

            value
        }
    }

    pub fn get_vertex_attribiv(index: GLuint, name: GLenum) -> GLint {
        unsafe {
            let mut value: GLint = 0;

            ffi::glGetVertexAttribiv(index, name, &mut value);

            value
        }
    }

    /*
    pub fn get_vertex_attrib_pointerv(index: GLuint, name: GLenum, pointer: &mut &mut GLvoid) {
        unsafe {
        }
    }
    */

    pub fn hint(target: GLenum, mode: GLenum) {
        unsafe { ffi::glHint(target, mode) }
    }

    pub fn is_buffer(buffer: GLuint) -> bool {
        unsafe { ffi::glIsBuffer(buffer) == GL_TRUE }
    }

    pub fn is_enabled(feature: GLenum) -> bool {
        unsafe { ffi::glIsEnabled(feature) == GL_TRUE }
    }

    pub fn is_framebuffer(framebuffer: GLuint) -> bool {
        unsafe { ffi::glIsFramebuffer(framebuffer) == GL_TRUE }
    }

    pub fn is_program(program: GLuint) -> bool {
        unsafe { ffi::glIsProgram(program) == GL_TRUE }
    }

    pub fn is_renderbuffer(renderbuffer: GLuint) -> bool {
        unsafe { ffi::glIsRenderbuffer(renderbuffer) == GL_TRUE }
    }

    pub fn is_shader(shader: GLuint) -> bool {
        unsafe { ffi::glIsShader(shader) == GL_TRUE }
    }

    pub fn is_texture(texture: GLuint) -> bool {
        unsafe { ffi::glIsTexture(texture) == GL_TRUE }
    }

    pub fn line_width(width: GLfloat) {
        unsafe { ffi::glLineWidth(width) }
    }

    pub fn link_program(program: GLuint) {
        unsafe { ffi::glLinkProgram(program) }
    }

    pub fn pixel_storei(name: GLenum, param: GLint) {
        unsafe { ffi::glPixelStorei(name, param) }
    }

    pub fn polygon_offset(factor: GLfloat, units: GLfloat) {
        unsafe { ffi::glPolygonOffset(factor, units) }
    }

    pub fn read_pixels<T>(
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        buffer: &mut [T],
    ) {
        unsafe {
            ffi::glReadPixels(
                x,
                y,
                width,
                height,
                format,
                type_,
                buffer.as_mut_ptr() as *mut GLvoid,
            )
        }
    }

    pub fn read_pixels_rgba(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> Vec<u8> {
        unsafe {
            let mut buffer: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);

            ffi::glReadPixels(
                x,
                y,
                width,
                height,
                GL_RGBA,
                GL_UNSIGNED_BYTE,
                buffer.as_mut_ptr() as *mut GLvoid,
            );

            buffer.set_len((width * height * 4) as usize);
            buffer
        }
    }

    pub fn release_shader_compiler() {
        unsafe { ffi::glReleaseShaderCompiler() }
    }

    pub fn renderbuffer_storage(
        target: GLenum,
        internal_format: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        unsafe { ffi::glRenderbufferStorage(target, internal_format, width, height) }
    }

    pub fn sample_coverage(value: GLclampf, invert: bool) {
        unsafe { ffi::glSampleCoverage(value, invert as GLboolean) }
    }

    pub fn scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        unsafe { ffi::glScissor(x, y, width, height) }
    }

    pub fn shader_binary<T>(shaders: &[GLuint], data_format: GLenum, data: &[T], length: GLsizei) {
        unsafe {
            ffi::glShaderBinary(
                shaders.len() as GLsizei,
                shaders.as_ptr(),
                data_format,
                data.as_ptr() as *const GLvoid,
                length,
            )
        }
    }

    pub fn shader_source(shader: GLuint, source: &[u8]) {
        unsafe {
            let length: GLsizei = source.len() as GLsizei;

            ffi::glShaderSource(shader, 1, &(source.as_ptr() as *const GLchar), &length)
        }
    }

    pub fn stencil_func(func: GLenum, ref_: GLint, mask: GLuint) {
        unsafe { ffi::glStencilFunc(func, ref_, mask) }
    }

    pub fn stencil_func_separate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) {
        unsafe { ffi::glStencilFuncSeparate(face, func, ref_, mask) }
    }

    pub fn stencil_mask(mask: GLuint) {
        unsafe { ffi::glStencilMask(mask) }
    }

    pub fn stencil_mask_separate(face: GLenum, mask: GLuint) {
        unsafe { ffi::glStencilMaskSeparate(face, mask) }
    }

    pub fn stencil_op(fail: GLenum, z_fail: GLenum, z_pass: GLenum) {
        unsafe { ffi::glStencilOp(fail, z_fail, z_pass) }
    }

    pub fn stencil_op_separate(face: GLenum, fail: GLenum, z_fail: GLenum, z_pass: GLenum) {
        unsafe { ffi::glStencilOpSeparate(face, fail, z_fail, z_pass) }
    }

    pub fn empty_tex_image_2d(
        target: GLenum,
        level: GLint,
        internal_format: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
    ) {
        unsafe {
            ffi::glTexImage2D(
                target,
                level,
                internal_format,
                width,
                height,
                border,
                format,
                type_,
                ptr::null(),
            )
        }
    }

    pub fn tex_image_2d<T>(
        target: GLenum,
        level: GLint,
        internal_format: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        buffer: &[T],
    ) {
        unsafe {
            ffi::glTexImage2D(
                target,
                level,
                internal_format,
                width,
                height,
                border,
                format,
                type_,
                buffer.as_ptr() as *const GLvoid,
            )
        }
    }

    pub fn tex_parameterf(target: GLenum, name: GLenum, value: GLfloat) {
        unsafe { ffi::glTexParameterf(target, name, value) }
    }

    pub fn tex_parameterfv(target: GLenum, name: GLenum, value: &GLfloat) {
        unsafe { ffi::glTexParameterfv(target, name, value) }
    }

    pub fn tex_parameteri(target: GLenum, name: GLenum, value: GLint) {
        unsafe { ffi::glTexParameteri(target, name, value) }
    }

    pub fn tex_parameteriv(target: GLenum, name: GLenum, value: &GLint) {
        unsafe { ffi::glTexParameteriv(target, name, value) }
    }

    pub fn empty_tex_sub_image_2d(
        target: GLenum,
        level: GLint,
        x_offset: GLint,
        y_offset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
    ) {
        unsafe {
            ffi::glTexSubImage2D(
                target,
                level,
                x_offset,
                y_offset,
                width,
                height,
                format,
                type_,
                ptr::null(),
            )
        }
    }

    pub fn tex_sub_image_2d<T>(
        target: GLenum,
        level: GLint,
        x_offset: GLint,
        y_offset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        buffer: &[T],
    ) {
        unsafe {
            ffi::glTexSubImage2D(
                target,
                level,
                x_offset,
                y_offset,
                width,
                height,
                format,
                type_,
                buffer.as_ptr() as *const GLvoid,
            )
        }
    }

    pub fn uniform1f(location: GLint, x: GLfloat) {
        unsafe { ffi::glUniform1f(location, x) }
    }

    pub fn uniform1fv(location: GLint, values: &[GLfloat]) {
        unsafe { ffi::glUniform1fv(location, values.len() as GLsizei, values.as_ptr()) }
    }

    pub fn uniform1i(location: GLint, x: GLint) {
        unsafe { ffi::glUniform1i(location, x) }
    }

    pub fn uniform1iv(location: GLint, values: &[GLint]) {
        unsafe { ffi::glUniform1iv(location, values.len() as GLsizei, values.as_ptr()) }
    }

    pub fn uniform2f(location: GLint, x: GLfloat, y: GLfloat) {
        unsafe { ffi::glUniform2f(location, x, y) }
    }

    pub fn uniform2fv(location: GLint, values: &[GLfloat]) {
        unsafe { ffi::glUniform2fv(location, (values.len() / 2) as GLsizei, values.as_ptr()) }
    }

    pub fn uniform2i(location: GLint, x: GLint, y: GLint) {
        unsafe { ffi::glUniform2i(location, x, y) }
    }

    pub fn uniform2iv(location: GLint, values: &[GLint]) {
        unsafe { ffi::glUniform2iv(location, (values.len() / 2) as GLsizei, values.as_ptr()) }
    }

    pub fn uniform3f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat) {
        unsafe { ffi::glUniform3f(location, x, y, z) }
    }

    pub fn uniform3fv(location: GLint, values: &[GLfloat]) {
        unsafe { ffi::glUniform3fv(location, (values.len() / 3) as GLsizei, values.as_ptr()) }
    }

    pub fn uniform3i(location: GLint, x: GLint, y: GLint, z: GLint) {
        unsafe { ffi::glUniform3i(location, x, y, z) }
    }

    pub fn uniform3iv(location: GLint, values: &[GLint]) {
        unsafe { ffi::glUniform3iv(location, (values.len() / 3) as GLsizei, values.as_ptr()) }
    }

    pub fn uniform4f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
        unsafe { ffi::glUniform4f(location, x, y, z, w) }
    }

    pub fn uniform4fv(location: GLint, values: &[GLfloat]) {
        unsafe { ffi::glUniform4fv(location, (values.len() / 4) as GLsizei, values.as_ptr()) }
    }

    pub fn uniform4i(location: GLint, x: GLint, y: GLint, z: GLint, w: GLint) {
        unsafe { ffi::glUniform4i(location, x, y, z, w) }
    }

    pub fn uniform4iv(location: GLint, values: &[GLint]) {
        unsafe { ffi::glUniform4iv(location, (values.len() / 4) as GLsizei, values.as_ptr()) }
    }

    pub fn uniform_matrix2fv(location: GLint, transpose: bool, values: &[GLfloat]) {
        unsafe {
            ffi::glUniformMatrix2fv(
                location,
                (values.len() / 4) as GLsizei,
                transpose as GLboolean,
                values.as_ptr() as *const GLfloat,
            )
        }
    }

    pub fn uniform_matrix3fv(location: GLint, transpose: bool, values: &[GLfloat]) {
        unsafe {
            ffi::glUniformMatrix3fv(
                location,
                (values.len() / 9) as GLsizei,
                transpose as GLboolean,
                values.as_ptr() as *const GLfloat,
            )
        }
    }

    pub fn uniform_matrix4fv(location: GLint, transpose: bool, values: &[GLfloat]) {
        unsafe {
            ffi::glUniformMatrix4fv(
                location,
                (values.len() / 16) as GLsizei,
                transpose as GLboolean,
                values.as_ptr() as *const GLfloat,
            )
        }
    }

    pub fn use_program(program: GLuint) {
        unsafe { ffi::glUseProgram(program) }
    }

    pub fn validate_program(program: GLuint) {
        unsafe { ffi::glValidateProgram(program) }
    }

    pub fn vertex_attrib1f(index: GLuint, x: GLfloat) {
        unsafe { ffi::glVertexAttrib1f(index, x) }
    }

    pub fn vertex_attrib1fv(index: GLuint, values: &[GLfloat]) {
        unsafe { ffi::glVertexAttrib1fv(index, values.as_ptr()) }
    }

    pub fn vertex_attrib2f(index: GLuint, x: GLfloat, y: GLfloat) {
        unsafe { ffi::glVertexAttrib2f(index, x, y) }
    }

    pub fn vertex_attrib2fv(index: GLuint, values: &[GLfloat]) {
        unsafe { ffi::glVertexAttrib2fv(index, values.as_ptr()) }
    }

    pub fn vertex_attrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
        unsafe { ffi::glVertexAttrib3f(index, x, y, z) }
    }

    pub fn vertex_attrib3fv(index: GLuint, values: &[GLfloat]) {
        unsafe { ffi::glVertexAttrib3fv(index, values.as_ptr()) }
    }

    pub fn vertex_attrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
        unsafe { ffi::glVertexAttrib4f(index, x, y, z, w) }
    }

    pub fn vertex_attrib4fv(index: GLuint, values: &[GLfloat]) {
        unsafe { ffi::glVertexAttrib4fv(index, values.as_ptr()) }
    }

    pub fn vertex_attrib_pointer<T>(
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: bool,
        stride: GLsizei,
        buffer: &[T],
    ) {
        unsafe {
            if buffer.len() == 0 {
                ffi::glVertexAttribPointer(
                    index,
                    size,
                    type_,
                    normalized as GLboolean,
                    stride,
                    &0 as *const i32 as *const GLvoid,
                )
            } else {
                ffi::glVertexAttribPointer(
                    index,
                    size,
                    type_,
                    normalized as GLboolean,
                    stride,
                    buffer.as_ptr() as *const GLvoid,
                )
            }
        }
    }

    pub fn vertex_attrib_pointer_offset(
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: bool,
        stride: GLsizei,
        offset: GLuint,
    ) {
        unsafe {
            ffi::glVertexAttribPointer(
                index,
                size,
                type_,
                normalized as GLboolean,
                stride,
                offset as *const GLvoid,
            )
        }
    }

    pub fn viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        unsafe { ffi::glViewport(x, y, width, height) }
    }
}
