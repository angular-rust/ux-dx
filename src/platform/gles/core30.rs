//! OpenGL ES 3.0 Core support

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
use super::{
    enums::{GL_RGBA, GL_TRUE, GL_UNSIGNED_BYTE},
    types::*,
};

pub mod gl {
    use super::*;

    // module-cascade infrastructure
    pub use crate::platform::gles::core20::gl::*;

    // -------------------------------------------------------------------------------------------------
    // TYPES
    // -------------------------------------------------------------------------------------------------

    // #[repr(C)]
    // #[derive(Debug, Copy, Clone)]
    // pub struct __GLsync {
    //     _unused: [u8; 0],
    // }
    // pub type GLsync = *mut __GLsync;

    // pub type GLint64 = khronos_int64_t;
    // pub type GLuint64 = khronos_uint64_t;

    // -------------------------------------------------------------------------------------------------
    // STRUCTS
    // -------------------------------------------------------------------------------------------------

    // -------------------------------------------------------------------------------------------------
    // CONSTANTS
    // -------------------------------------------------------------------------------------------------

    // pub const __gles2_gl3_h_: GLuint = 1;
    // pub const _STDINT_H: GLuint = 1;
    // pub const _FEATURES_H: GLuint = 1;
    // pub const _DEFAULT_SOURCE: GLuint = 1;
    // pub const __GLIBC_USE_ISOC2X: GLuint = 0;
    // pub const __USE_ISOC11: GLuint = 1;
    // pub const __USE_ISOC99: GLuint = 1;
    // pub const __USE_ISOC95: GLuint = 1;
    // pub const __USE_POSIX_IMPLICITLY: GLuint = 1;
    // pub const _POSIX_SOURCE: GLuint = 1;
    // pub const _POSIX_C_SOURCE: GLuint = 200809;
    // pub const __USE_POSIX: GLuint = 1;
    // pub const __USE_POSIX2: GLuint = 1;
    // pub const __USE_POSIX199309: GLuint = 1;
    // pub const __USE_POSIX199506: GLuint = 1;
    // pub const __USE_XOPEN2K: GLuint = 1;
    // pub const __USE_XOPEN2K8: GLuint = 1;
    // pub const _ATFILE_SOURCE: GLuint = 1;
    // pub const __USE_MISC: GLuint = 1;
    // pub const __USE_ATFILE: GLuint = 1;
    // pub const __USE_FORTIFY_LEVEL: GLuint = 0;
    // pub const __GLIBC_USE_DEPRECATED_GETS: GLuint = 0;
    // pub const __GLIBC_USE_DEPRECATED_SCANF: GLuint = 0;
    // pub const _STDC_PREDEF_H: GLuint = 1;
    // pub const __STDC_IEC_559__: GLuint = 1;
    // pub const __STDC_IEC_559_COMPLEX__: GLuint = 1;
    // pub const __STDC_ISO_10646__: GLuint = 201706;
    // pub const __GNU_LIBRARY__: GLuint = 6;
    // pub const __GLIBC__: GLuint = 2;
    // pub const __GLIBC_MINOR__: GLuint = 31;
    // pub const _SYS_CDEFS_H: GLuint = 1;
    // pub const __glibc_c99_flexarr_available: GLuint = 1;
    // pub const __WORDSIZE: GLuint = 64;
    // pub const __WORDSIZE_TIME64_COMPAT32: GLuint = 1;
    // pub const __SYSCALL_WORDSIZE: GLuint = 64;
    // pub const __LONG_DOUBLE_USES_FLOAT128: GLuint = 0;
    // pub const __HAVE_GENERIC_SELECTION: GLuint = 1;
    // pub const __GLIBC_USE_LIB_EXT2: GLuint = 0;
    // pub const __GLIBC_USE_IEC_60559_BFP_EXT: GLuint = 0;
    // pub const __GLIBC_USE_IEC_60559_BFP_EXT_C2X: GLuint = 0;
    // pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: GLuint = 0;
    // pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C2X: GLuint = 0;
    // pub const __GLIBC_USE_IEC_60559_TYPES_EXT: GLuint = 0;
    // pub const _BITS_TYPES_H: GLuint = 1;
    // pub const __TIMESIZE: GLuint = 64;
    // pub const _BITS_TYPESIZES_H: GLuint = 1;
    // pub const __OFF_T_MATCHES_OFF64_T: GLuint = 1;
    // pub const __INO_T_MATCHES_INO64_T: GLuint = 1;
    // pub const __RLIM_T_MATCHES_RLIM64_T: GLuint = 1;
    // pub const __STATFS_MATCHES_STATFS64: GLuint = 1;
    // pub const __FD_SETSIZE: GLuint = 1024;
    // pub const _BITS_TIME64_H: GLuint = 1;
    // pub const _BITS_WCHAR_H: GLuint = 1;
    // pub const _BITS_STDINT_INTN_H: GLuint = 1;
    // pub const _BITS_STDINT_UINTN_H: GLuint = 1;
    // pub const INT8_MIN: i32 = -128;
    // pub const INT16_MIN: i32 = -32768;
    // pub const INT32_MIN: i32 = -2147483648;
    // pub const INT8_MAX: GLuint = 127;
    // pub const INT16_MAX: GLuint = 32767;
    // pub const INT32_MAX: GLuint = 2147483647;
    // pub const UINT8_MAX: GLuint = 255;
    // pub const UINT16_MAX: GLuint = 65535;
    // pub const UINT32_MAX: GLuint = 4294967295;
    // pub const INT_LEAST8_MIN: i32 = -128;
    // pub const INT_LEAST16_MIN: i32 = -32768;
    // pub const INT_LEAST32_MIN: i32 = -2147483648;
    // pub const INT_LEAST8_MAX: GLuint = 127;
    // pub const INT_LEAST16_MAX: GLuint = 32767;
    // pub const INT_LEAST32_MAX: GLuint = 2147483647;
    // pub const UINT_LEAST8_MAX: GLuint = 255;
    // pub const UINT_LEAST16_MAX: GLuint = 65535;
    // pub const UINT_LEAST32_MAX: GLuint = 4294967295;
    // pub const INT_FAST8_MIN: i32 = -128;
    // pub const INT_FAST16_MIN: i64 = -9223372036854775808;
    // pub const INT_FAST32_MIN: i64 = -9223372036854775808;
    // pub const INT_FAST8_MAX: GLuint = 127;
    // pub const INT_FAST16_MAX: u64 = 9223372036854775807;
    // pub const INT_FAST32_MAX: u64 = 9223372036854775807;
    // pub const UINT_FAST8_MAX: GLuint = 255;
    // pub const UINT_FAST16_MAX: i32 = -1;
    // pub const UINT_FAST32_MAX: i32 = -1;
    // pub const INTPTR_MIN: i64 = -9223372036854775808;
    // pub const INTPTR_MAX: u64 = 9223372036854775807;
    // pub const UINTPTR_MAX: i32 = -1;
    // pub const PTRDIFF_MIN: i64 = -9223372036854775808;
    // pub const PTRDIFF_MAX: u64 = 9223372036854775807;
    // pub const SIG_ATOMIC_MIN: i32 = -2147483648;
    // pub const SIG_ATOMIC_MAX: GLuint = 2147483647;
    // pub const SIZE_MAX: i32 = -1;
    // pub const WINT_MIN: GLuint = 0;
    // pub const WINT_MAX: GLuint = 4294967295;

    // pub const KHRONOS_SUPPORT_INT64: GLuint = 1;
    // pub const KHRONOS_SUPPORT_FLOAT: GLuint = 1;
    // pub const KHRONOS_MAX_ENUM: GLuint = 2147483647;
    // pub const GLES_PROTOTYPES: GLuint = 1;
    // pub const ES_VERSION_2_0: GLuint = 1;
    // pub const DEPTH_BUFFER_BIT: GLuint = 256;
    // pub const STENCIL_BUFFER_BIT: GLuint = 1024;
    // pub const COLOR_BUFFER_BIT: GLuint = 16384;
    // pub const FALSE: GLboolean = 0;
    // pub const TRUE: GLboolean = 1;
    // pub const POINTS: GLuint = 0;
    // pub const LINES: GLuint = 1;
    // pub const LINE_LOOP: GLuint = 2;
    // pub const LINE_STRIP: GLuint = 3;
    // pub const TRIANGLES: GLuint = 4;
    // pub const TRIANGLE_STRIP: GLuint = 5;
    // pub const TRIANGLE_FAN: GLuint = 6;
    // pub const ZERO: GLuint = 0;
    // pub const ONE: GLuint = 1;
    // pub const SRC_COLOR: GLuint = 768;
    // pub const ONE_MINUS_SRC_COLOR: GLuint = 769;
    // pub const SRC_ALPHA: GLuint = 770;
    // pub const ONE_MINUS_SRC_ALPHA: GLuint = 771;
    // pub const DST_ALPHA: GLuint = 772;
    // pub const ONE_MINUS_DST_ALPHA: GLuint = 773;
    // pub const DST_COLOR: GLuint = 774;
    // pub const ONE_MINUS_DST_COLOR: GLuint = 775;
    // pub const SRC_ALPHA_SATURATE: GLuint = 776;
    // pub const FUNC_ADD: GLuint = 32774;
    // pub const BLEND_EQUATION: GLuint = 32777;
    // pub const BLEND_EQUATION_RGB: GLuint = 32777;
    // pub const BLEND_EQUATION_ALPHA: GLuint = 34877;
    // pub const FUNC_SUBTRACT: GLuint = 32778;
    // pub const FUNC_REVERSE_SUBTRACT: GLuint = 32779;
    // pub const BLEND_DST_RGB: GLuint = 32968;
    // pub const BLEND_SRC_RGB: GLuint = 32969;
    // pub const BLEND_DST_ALPHA: GLuint = 32970;
    // pub const BLEND_SRC_ALPHA: GLuint = 32971;
    // pub const CONSTANT_COLOR: GLuint = 32769;
    // pub const ONE_MINUS_CONSTANT_COLOR: GLuint = 32770;
    // pub const CONSTANT_ALPHA: GLuint = 32771;
    // pub const ONE_MINUS_CONSTANT_ALPHA: GLuint = 32772;
    // pub const BLEND_COLOR: GLuint = 32773;
    // pub const ARRAY_BUFFER: GLuint = 34962;
    // pub const ELEMENT_ARRAY_BUFFER: GLuint = 34963;
    // pub const ARRAY_BUFFER_BINDING: GLuint = 34964;
    // pub const ELEMENT_ARRAY_BUFFER_BINDING: GLuint = 34965;
    // pub const STREAM_DRAW: GLuint = 35040;
    // pub const STATIC_DRAW: GLuint = 35044;
    // pub const DYNAMIC_DRAW: GLuint = 35048;
    // pub const BUFFER_SIZE: GLuint = 34660;
    // pub const BUFFER_USAGE: GLuint = 34661;
    // pub const CURRENT_VERTEX_ATTRIB: GLuint = 34342;
    // pub const FRONT: GLuint = 1028;
    // pub const BACK: GLuint = 1029;
    // pub const FRONT_AND_BACK: GLuint = 1032;
    // pub const TEXTURE_2D: GLuint = 3553;
    // pub const CULL_FACE: GLuint = 2884;
    // pub const BLEND: GLuint = 3042;
    // pub const DITHER: GLuint = 3024;
    // pub const STENCIL_TEST: GLuint = 2960;
    // pub const DEPTH_TEST: GLuint = 2929;
    // pub const SCISSOR_TEST: GLuint = 3089;
    // pub const POLYGON_OFFSET_FILL: GLuint = 32823;
    // pub const SAMPLE_ALPHA_TO_COVERAGE: GLuint = 32926;
    // pub const SAMPLE_COVERAGE: GLuint = 32928;
    // pub const NO_ERROR: GLuint = 0;
    // pub const INVALID_ENUM: GLuint = 1280;
    // pub const INVALID_VALUE: GLuint = 1281;
    // pub const INVALID_OPERATION: GLuint = 1282;
    // pub const OUT_OF_MEMORY: GLuint = 1285;
    // pub const CW: GLuint = 2304;
    // pub const CCW: GLuint = 2305;
    // pub const LINE_WIDTH: GLuint = 2849;
    // pub const ALIASED_POINT_SIZE_RANGE: GLuint = 33901;
    // pub const ALIASED_LINE_WIDTH_RANGE: GLuint = 33902;
    // pub const CULL_FACE_MODE: GLuint = 2885;
    // pub const FRONT_FACE: GLuint = 2886;
    // pub const DEPTH_RANGE: GLuint = 2928;
    // pub const DEPTH_WRITEMASK: GLuint = 2930;
    // pub const DEPTH_CLEAR_VALUE: GLuint = 2931;
    // pub const DEPTH_FUNC: GLuint = 2932;
    // pub const STENCIL_CLEAR_VALUE: GLuint = 2961;
    // pub const STENCIL_FUNC: GLuint = 2962;
    // pub const STENCIL_FAIL: GLuint = 2964;
    // pub const STENCIL_PASS_DEPTH_FAIL: GLuint = 2965;
    // pub const STENCIL_PASS_DEPTH_PASS: GLuint = 2966;
    // pub const STENCIL_REF: GLuint = 2967;
    // pub const STENCIL_VALUE_MASK: GLuint = 2963;
    // pub const STENCIL_WRITEMASK: GLuint = 2968;
    // pub const STENCIL_BACK_FUNC: GLuint = 34816;
    // pub const STENCIL_BACK_FAIL: GLuint = 34817;
    // pub const STENCIL_BACK_PASS_DEPTH_FAIL: GLuint = 34818;
    // pub const STENCIL_BACK_PASS_DEPTH_PASS: GLuint = 34819;
    // pub const STENCIL_BACK_REF: GLuint = 36003;
    // pub const STENCIL_BACK_VALUE_MASK: GLuint = 36004;
    // pub const STENCIL_BACK_WRITEMASK: GLuint = 36005;
    // pub const VIEWPORT: GLuint = 2978;
    // pub const SCISSOR_BOX: GLuint = 3088;
    // pub const COLOR_CLEAR_VALUE: GLuint = 3106;
    // pub const COLOR_WRITEMASK: GLuint = 3107;
    // pub const UNPACK_ALIGNMENT: GLuint = 3317;
    // pub const PACK_ALIGNMENT: GLuint = 3333;
    // pub const MAX_TEXTURE_SIZE: GLuint = 3379;
    // pub const MAX_VIEWPORT_DIMS: GLuint = 3386;
    // pub const SUBPIXEL_BITS: GLuint = 3408;
    // pub const RED_BITS: GLuint = 3410;
    // pub const GREEN_BITS: GLuint = 3411;
    // pub const BLUE_BITS: GLuint = 3412;
    // pub const ALPHA_BITS: GLuint = 3413;
    // pub const DEPTH_BITS: GLuint = 3414;
    // pub const STENCIL_BITS: GLuint = 3415;
    // pub const POLYGON_OFFSET_UNITS: GLuint = 10752;
    // pub const POLYGON_OFFSET_FACTOR: GLuint = 32824;
    // pub const TEXTURE_BINDING_2D: GLuint = 32873;
    // pub const SAMPLE_BUFFERS: GLuint = 32936;
    // pub const SAMPLES: GLuint = 32937;
    // pub const SAMPLE_COVERAGE_VALUE: GLuint = 32938;
    // pub const SAMPLE_COVERAGE_INVERT: GLuint = 32939;
    // pub const NUM_COMPRESSED_TEXTURE_FORMATS: GLuint = 34466;
    // pub const COMPRESSED_TEXTURE_FORMATS: GLuint = 34467;
    // pub const DONT_CARE: GLuint = 4352;
    // pub const FASTEST: GLuint = 4353;
    // pub const NICEST: GLuint = 4354;
    // pub const GENERATE_MIPMAP_HINT: GLuint = 33170;
    // pub const BYTE: GLuint = 5120;
    // pub const UNSIGNED_BYTE: GLuint = 5121;
    // pub const SHORT: GLuint = 5122;
    // pub const UNSIGNED_SHORT: GLuint = 5123;
    // pub const INT: GLuint = 5124;
    // pub const UNSIGNED_INT: GLuint = 5125;
    // pub const FLOAT: GLuint = 5126;
    // pub const FIXED: GLuint = 5132;
    // pub const DEPTH_COMPONENT: GLuint = 6402;
    // pub const ALPHA: GLuint = 6406;
    // pub const RGB: GLuint = 6407;
    // pub const RGBA: GLuint = 6408;
    // pub const LUMINANCE: GLuint = 6409;
    // pub const LUMINANCE_ALPHA: GLuint = 6410;
    // pub const UNSIGNED_SHORT_4_4_4_4: GLuint = 32819;
    // pub const UNSIGNED_SHORT_5_5_5_1: GLuint = 32820;
    // pub const UNSIGNED_SHORT_5_6_5: GLuint = 33635;
    // pub const FRAGMENT_SHADER: GLuint = 35632;
    // pub const VERTEX_SHADER: GLuint = 35633;
    // pub const MAX_VERTEX_ATTRIBS: GLuint = 34921;
    // pub const MAX_VERTEX_UNIFORM_VECTORS: GLuint = 36347;
    // pub const MAX_VARYING_VECTORS: GLuint = 36348;
    // pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLuint = 35661;
    // pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLuint = 35660;
    // pub const MAX_TEXTURE_IMAGE_UNITS: GLuint = 34930;
    // pub const MAX_FRAGMENT_UNIFORM_VECTORS: GLuint = 36349;
    // pub const SHADER_TYPE: GLuint = 35663;
    // pub const DELETE_STATUS: GLuint = 35712;
    // pub const LINK_STATUS: GLuint = 35714;
    // pub const VALIDATE_STATUS: GLuint = 35715;
    // pub const ATTACHED_SHADERS: GLuint = 35717;
    // pub const ACTIVE_UNIFORMS: GLuint = 35718;
    // pub const ACTIVE_UNIFORM_MAX_LENGTH: GLuint = 35719;
    // pub const ACTIVE_ATTRIBUTES: GLuint = 35721;
    // pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: GLuint = 35722;
    // pub const SHADING_LANGUAGE_VERSION: GLuint = 35724;
    // pub const CURRENT_PROGRAM: GLuint = 35725;
    // pub const NEVER: GLuint = 512;
    // pub const LESS: GLuint = 513;
    // pub const EQUAL: GLuint = 514;
    // pub const LEQUAL: GLuint = 515;
    // pub const GREATER: GLuint = 516;
    // pub const NOTEQUAL: GLuint = 517;
    // pub const GEQUAL: GLuint = 518;
    // pub const ALWAYS: GLuint = 519;
    // pub const KEEP: GLuint = 7680;
    // pub const REPLACE: GLuint = 7681;
    // pub const INCR: GLuint = 7682;
    // pub const DECR: GLuint = 7683;
    // pub const INVERT: GLuint = 5386;
    // pub const INCR_WRAP: GLuint = 34055;
    // pub const DECR_WRAP: GLuint = 34056;
    // pub const VENDOR: GLuint = 7936;
    // pub const RENDERER: GLuint = 7937;
    // pub const VERSION: GLuint = 7938;
    // pub const EXTENSIONS: GLuint = 7939;
    // pub const NEAREST: GLuint = 9728;
    // pub const LINEAR: GLuint = 9729;
    // pub const NEAREST_MIPMAP_NEAREST: GLuint = 9984;
    // pub const LINEAR_MIPMAP_NEAREST: GLuint = 9985;
    // pub const NEAREST_MIPMAP_LINEAR: GLuint = 9986;
    // pub const LINEAR_MIPMAP_LINEAR: GLuint = 9987;
    // pub const TEXTURE_MAG_FILTER: GLuint = 10240;
    // pub const TEXTURE_MIN_FILTER: GLuint = 10241;
    // pub const TEXTURE_WRAP_S: GLuint = 10242;
    // pub const TEXTURE_WRAP_T: GLuint = 10243;
    // pub const TEXTURE: GLuint = 5890;
    // pub const TEXTURE_CUBE_MAP: GLuint = 34067;
    // pub const TEXTURE_BINDING_CUBE_MAP: GLuint = 34068;
    // pub const TEXTURE_CUBE_MAP_POSITIVE_X: GLuint = 34069;
    // pub const TEXTURE_CUBE_MAP_NEGATIVE_X: GLuint = 34070;
    // pub const TEXTURE_CUBE_MAP_POSITIVE_Y: GLuint = 34071;
    // pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: GLuint = 34072;
    // pub const TEXTURE_CUBE_MAP_POSITIVE_Z: GLuint = 34073;
    // pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: GLuint = 34074;
    // pub const MAX_CUBE_MAP_TEXTURE_SIZE: GLuint = 34076;
    // pub const TEXTURE0: GLuint = 33984;
    // pub const TEXTURE1: GLuint = 33985;
    // pub const TEXTURE2: GLuint = 33986;
    // pub const TEXTURE3: GLuint = 33987;
    // pub const TEXTURE4: GLuint = 33988;
    // pub const TEXTURE5: GLuint = 33989;
    // pub const TEXTURE6: GLuint = 33990;
    // pub const TEXTURE7: GLuint = 33991;
    // pub const TEXTURE8: GLuint = 33992;
    // pub const TEXTURE9: GLuint = 33993;
    // pub const TEXTURE10: GLuint = 33994;
    // pub const TEXTURE11: GLuint = 33995;
    // pub const TEXTURE12: GLuint = 33996;
    // pub const TEXTURE13: GLuint = 33997;
    // pub const TEXTURE14: GLuint = 33998;
    // pub const TEXTURE15: GLuint = 33999;
    // pub const TEXTURE16: GLuint = 34000;
    // pub const TEXTURE17: GLuint = 34001;
    // pub const TEXTURE18: GLuint = 34002;
    // pub const TEXTURE19: GLuint = 34003;
    // pub const TEXTURE20: GLuint = 34004;
    // pub const TEXTURE21: GLuint = 34005;
    // pub const TEXTURE22: GLuint = 34006;
    // pub const TEXTURE23: GLuint = 34007;
    // pub const TEXTURE24: GLuint = 34008;
    // pub const TEXTURE25: GLuint = 34009;
    // pub const TEXTURE26: GLuint = 34010;
    // pub const TEXTURE27: GLuint = 34011;
    // pub const TEXTURE28: GLuint = 34012;
    // pub const TEXTURE29: GLuint = 34013;
    // pub const TEXTURE30: GLuint = 34014;
    // pub const TEXTURE31: GLuint = 34015;
    // pub const ACTIVE_TEXTURE: GLuint = 34016;
    // pub const REPEAT: GLuint = 10497;
    // pub const CLAMP_TO_EDGE: GLuint = 33071;
    // pub const MIRRORED_REPEAT: GLuint = 33648;
    // pub const FLOAT_VEC2: GLuint = 35664;
    // pub const FLOAT_VEC3: GLuint = 35665;
    // pub const FLOAT_VEC4: GLuint = 35666;
    // pub const INT_VEC2: GLuint = 35667;
    // pub const INT_VEC3: GLuint = 35668;
    // pub const INT_VEC4: GLuint = 35669;
    // pub const BOOL: GLuint = 35670;
    // pub const BOOL_VEC2: GLuint = 35671;
    // pub const BOOL_VEC3: GLuint = 35672;
    // pub const BOOL_VEC4: GLuint = 35673;
    // pub const FLOAT_MAT2: GLuint = 35674;
    // pub const FLOAT_MAT3: GLuint = 35675;
    // pub const FLOAT_MAT4: GLuint = 35676;
    // pub const SAMPLER_2D: GLuint = 35678;
    // pub const SAMPLER_CUBE: GLuint = 35680;
    // pub const VERTEX_ATTRIB_ARRAY_ENABLED: GLuint = 34338;
    // pub const VERTEX_ATTRIB_ARRAY_SIZE: GLuint = 34339;
    // pub const VERTEX_ATTRIB_ARRAY_STRIDE: GLuint = 34340;
    // pub const VERTEX_ATTRIB_ARRAY_TYPE: GLuint = 34341;
    // pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: GLuint = 34922;
    // pub const VERTEX_ATTRIB_ARRAY_POINTER: GLuint = 34373;
    // pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLuint = 34975;
    // pub const IMPLEMENTATION_COLOR_READ_TYPE: GLuint = 35738;
    // pub const IMPLEMENTATION_COLOR_READ_FORMAT: GLuint = 35739;
    // pub const COMPILE_STATUS: GLuint = 35713;
    // pub const INFO_LOG_LENGTH: GLuint = 35716;
    // pub const SHADER_SOURCE_LENGTH: GLuint = 35720;
    // pub const SHADER_COMPILER: GLuint = 36346;
    // pub const SHADER_BINARY_FORMATS: GLuint = 36344;
    // pub const NUM_SHADER_BINARY_FORMATS: GLuint = 36345;
    // pub const LOW_FLOAT: GLuint = 36336;
    // pub const MEDIUM_FLOAT: GLuint = 36337;
    // pub const HIGH_FLOAT: GLuint = 36338;
    // pub const LOW_INT: GLuint = 36339;
    // pub const MEDIUM_INT: GLuint = 36340;
    // pub const HIGH_INT: GLuint = 36341;
    // pub const FRAMEBUFFER: GLuint = 36160;
    // pub const RENDERBUFFER: GLuint = 36161;
    // pub const RGBA4: GLuint = 32854;
    // pub const RGB5_A1: GLuint = 32855;
    // pub const RGB565: GLuint = 36194;
    // pub const DEPTH_COMPONENT16: GLuint = 33189;
    // pub const STENCIL_INDEX8: GLuint = 36168;
    // pub const RENDERBUFFER_WIDTH: GLuint = 36162;
    // pub const RENDERBUFFER_HEIGHT: GLuint = 36163;
    // pub const RENDERBUFFER_INTERNAL_FORMAT: GLuint = 36164;
    // pub const RENDERBUFFER_RED_SIZE: GLuint = 36176;
    // pub const RENDERBUFFER_GREEN_SIZE: GLuint = 36177;
    // pub const RENDERBUFFER_BLUE_SIZE: GLuint = 36178;
    // pub const RENDERBUFFER_ALPHA_SIZE: GLuint = 36179;
    // pub const RENDERBUFFER_DEPTH_SIZE: GLuint = 36180;
    // pub const RENDERBUFFER_STENCIL_SIZE: GLuint = 36181;
    // pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLuint = 36048;
    // pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLuint = 36049;
    // pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLuint = 36050;
    // pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLuint = 36051;
    // pub const COLOR_ATTACHMENT0: GLuint = 36064;
    // pub const DEPTH_ATTACHMENT: GLuint = 36096;
    // pub const STENCIL_ATTACHMENT: GLuint = 36128;
    // pub const NONE: GLuint = 0;
    // pub const FRAMEBUFFER_COMPLETE: GLuint = 36053;
    // pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLuint = 36054;
    // pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLuint = 36055;
    // pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: GLuint = 36057;
    // pub const FRAMEBUFFER_UNSUPPORTED: GLuint = 36061;
    // pub const FRAMEBUFFER_BINDING: GLuint = 36006;
    // pub const RENDERBUFFER_BINDING: GLuint = 36007;
    // pub const MAX_RENDERBUFFER_SIZE: GLuint = 34024;
    // pub const INVALID_FRAMEBUFFER_OPERATION: GLuint = 1286;
    // pub const ES_VERSION_3_0: GLuint = 1;
    // pub const READ_BUFFER: GLuint = 3074;
    // pub const UNPACK_ROW_LENGTH: GLuint = 3314;
    // pub const UNPACK_SKIP_ROWS: GLuint = 3315;
    // pub const UNPACK_SKIP_PIXELS: GLuint = 3316;
    // pub const PACK_ROW_LENGTH: GLuint = 3330;
    // pub const PACK_SKIP_ROWS: GLuint = 3331;
    // pub const PACK_SKIP_PIXELS: GLuint = 3332;
    // pub const COLOR: GLuint = 6144;
    // pub const DEPTH: GLuint = 6145;
    // pub const STENCIL: GLuint = 6146;
    // pub const RED: GLuint = 6403;
    // pub const RGB8: GLuint = 32849;
    // pub const RGBA8: GLuint = 32856;
    // pub const RGB10_A2: GLuint = 32857;
    // pub const TEXTURE_BINDING_3D: GLuint = 32874;
    // pub const UNPACK_SKIP_IMAGES: GLuint = 32877;
    // pub const UNPACK_IMAGE_HEIGHT: GLuint = 32878;
    // pub const TEXTURE_3D: GLuint = 32879;
    // pub const TEXTURE_WRAP_R: GLuint = 32882;
    // pub const MAX_3D_TEXTURE_SIZE: GLuint = 32883;
    // pub const UNSIGNED_INT_2_10_10_10_REV: GLuint = 33640;
    // pub const MAX_ELEMENTS_VERTICES: GLuint = 33000;
    // pub const MAX_ELEMENTS_INDICES: GLuint = 33001;
    // pub const TEXTURE_MIN_LOD: GLuint = 33082;
    // pub const TEXTURE_MAX_LOD: GLuint = 33083;
    // pub const TEXTURE_BASE_LEVEL: GLuint = 33084;
    // pub const TEXTURE_MAX_LEVEL: GLuint = 33085;
    // pub const MIN: GLuint = 32775;
    // pub const MAX: GLuint = 32776;
    // pub const DEPTH_COMPONENT24: GLuint = 33190;
    // pub const MAX_TEXTURE_LOD_BIAS: GLuint = 34045;
    // pub const TEXTURE_COMPARE_MODE: GLuint = 34892;
    // pub const TEXTURE_COMPARE_FUNC: GLuint = 34893;
    // pub const CURRENT_QUERY: GLuint = 34917;
    // pub const QUERY_RESULT: GLuint = 34918;
    // pub const QUERY_RESULT_AVAILABLE: GLuint = 34919;
    // pub const BUFFER_MAPPED: GLuint = 35004;
    // pub const BUFFER_MAP_POINTER: GLuint = 35005;
    // pub const STREAM_READ: GLuint = 35041;
    // pub const STREAM_COPY: GLuint = 35042;
    // pub const STATIC_READ: GLuint = 35045;
    // pub const STATIC_COPY: GLuint = 35046;
    // pub const DYNAMIC_READ: GLuint = 35049;
    // pub const DYNAMIC_COPY: GLuint = 35050;
    // pub const MAX_DRAW_BUFFERS: GLuint = 34852;
    // pub const DRAW_BUFFER0: GLuint = 34853;
    // pub const DRAW_BUFFER1: GLuint = 34854;
    // pub const DRAW_BUFFER2: GLuint = 34855;
    // pub const DRAW_BUFFER3: GLuint = 34856;
    // pub const DRAW_BUFFER4: GLuint = 34857;
    // pub const DRAW_BUFFER5: GLuint = 34858;
    // pub const DRAW_BUFFER6: GLuint = 34859;
    // pub const DRAW_BUFFER7: GLuint = 34860;
    // pub const DRAW_BUFFER8: GLuint = 34861;
    // pub const DRAW_BUFFER9: GLuint = 34862;
    // pub const DRAW_BUFFER10: GLuint = 34863;
    // pub const DRAW_BUFFER11: GLuint = 34864;
    // pub const DRAW_BUFFER12: GLuint = 34865;
    // pub const DRAW_BUFFER13: GLuint = 34866;
    // pub const DRAW_BUFFER14: GLuint = 34867;
    // pub const DRAW_BUFFER15: GLuint = 34868;
    // pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: GLuint = 35657;
    // pub const MAX_VERTEX_UNIFORM_COMPONENTS: GLuint = 35658;
    // pub const SAMPLER_3D: GLuint = 35679;
    // pub const SAMPLER_2D_SHADOW: GLuint = 35682;
    // pub const FRAGMENT_SHADER_DERIVATIVE_HINT: GLuint = 35723;
    // pub const PIXEL_PACK_BUFFER: GLuint = 35051;
    // pub const PIXEL_UNPACK_BUFFER: GLuint = 35052;
    // pub const PIXEL_PACK_BUFFER_BINDING: GLuint = 35053;
    // pub const PIXEL_UNPACK_BUFFER_BINDING: GLuint = 35055;
    // pub const FLOAT_MAT2x3: GLuint = 35685;
    // pub const FLOAT_MAT2x4: GLuint = 35686;
    // pub const FLOAT_MAT3x2: GLuint = 35687;
    // pub const FLOAT_MAT3x4: GLuint = 35688;
    // pub const FLOAT_MAT4x2: GLuint = 35689;
    // pub const FLOAT_MAT4x3: GLuint = 35690;
    // pub const SRGB: GLuint = 35904;
    // pub const SRGB8: GLuint = 35905;
    // pub const SRGB8_ALPHA8: GLuint = 35907;
    // pub const COMPARE_REF_TO_TEXTURE: GLuint = 34894;
    // pub const MAJOR_VERSION: GLuint = 33307;
    // pub const MINOR_VERSION: GLuint = 33308;
    // pub const NUM_EXTENSIONS: GLuint = 33309;
    // pub const RGBA32F: GLuint = 34836;
    // pub const RGB32F: GLuint = 34837;
    // pub const RGBA16F: GLuint = 34842;
    // pub const RGB16F: GLuint = 34843;
    // pub const VERTEX_ATTRIB_ARRAY_INTEGER: GLuint = 35069;
    // pub const MAX_ARRAY_TEXTURE_LAYERS: GLuint = 35071;
    // pub const MIN_PROGRAM_TEXEL_OFFSET: GLuint = 35076;
    // pub const MAX_PROGRAM_TEXEL_OFFSET: GLuint = 35077;
    // pub const MAX_VARYING_COMPONENTS: GLuint = 35659;
    // pub const TEXTURE_2D_ARRAY: GLuint = 35866;
    // pub const TEXTURE_BINDING_2D_ARRAY: GLuint = 35869;
    // pub const R11F_G11F_B10F: GLuint = 35898;
    // pub const UNSIGNED_INT_10F_11F_11F_REV: GLuint = 35899;
    // pub const RGB9_E5: GLuint = 35901;
    // pub const UNSIGNED_INT_5_9_9_9_REV: GLuint = 35902;
    // pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GLuint = 35958;
    // pub const TRANSFORM_FEEDBACK_BUFFER_MODE: GLuint = 35967;
    // pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLuint = 35968;
    // pub const TRANSFORM_FEEDBACK_VARYINGS: GLuint = 35971;
    // pub const TRANSFORM_FEEDBACK_BUFFER_START: GLuint = 35972;
    // pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: GLuint = 35973;
    // pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLuint = 35976;
    // pub const RASTERIZER_DISCARD: GLuint = 35977;
    // pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLuint = 35978;
    // pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLuint = 35979;
    // pub const INTERLEAVED_ATTRIBS: GLuint = 35980;
    // pub const SEPARATE_ATTRIBS: GLuint = 35981;
    // pub const TRANSFORM_FEEDBACK_BUFFER: GLuint = 35982;
    // pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: GLuint = 35983;
    // pub const RGBA32UI: GLuint = 36208;
    // pub const RGB32UI: GLuint = 36209;
    // pub const RGBA16UI: GLuint = 36214;
    // pub const RGB16UI: GLuint = 36215;
    // pub const RGBA8UI: GLuint = 36220;
    // pub const RGB8UI: GLuint = 36221;
    // pub const RGBA32I: GLuint = 36226;
    // pub const RGB32I: GLuint = 36227;
    // pub const RGBA16I: GLuint = 36232;
    // pub const RGB16I: GLuint = 36233;
    // pub const RGBA8I: GLuint = 36238;
    // pub const RGB8I: GLuint = 36239;
    // pub const RED_INTEGER: GLuint = 36244;
    // pub const RGB_INTEGER: GLuint = 36248;
    // pub const RGBA_INTEGER: GLuint = 36249;
    // pub const SAMPLER_2D_ARRAY: GLuint = 36289;
    // pub const SAMPLER_2D_ARRAY_SHADOW: GLuint = 36292;
    // pub const SAMPLER_CUBE_SHADOW: GLuint = 36293;
    // pub const UNSIGNED_INT_VEC2: GLuint = 36294;
    // pub const UNSIGNED_INT_VEC3: GLuint = 36295;
    // pub const UNSIGNED_INT_VEC4: GLuint = 36296;
    // pub const INT_SAMPLER_2D: GLuint = 36298;
    // pub const INT_SAMPLER_3D: GLuint = 36299;
    // pub const INT_SAMPLER_CUBE: GLuint = 36300;
    // pub const INT_SAMPLER_2D_ARRAY: GLuint = 36303;
    // pub const UNSIGNED_INT_SAMPLER_2D: GLuint = 36306;
    // pub const UNSIGNED_INT_SAMPLER_3D: GLuint = 36307;
    // pub const UNSIGNED_INT_SAMPLER_CUBE: GLuint = 36308;
    // pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: GLuint = 36311;
    // pub const BUFFER_ACCESS_FLAGS: GLuint = 37151;
    // pub const BUFFER_MAP_LENGTH: GLuint = 37152;
    // pub const BUFFER_MAP_OFFSET: GLuint = 37153;
    // pub const DEPTH_COMPONENT32F: GLuint = 36012;
    // pub const DEPTH32F_STENCIL8: GLuint = 36013;
    // pub const FLOAT_32_UNSIGNED_INT_24_8_REV: GLuint = 36269;
    // pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLuint = 33296;
    // pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLuint = 33297;
    // pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLuint = 33298;
    // pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLuint = 33299;
    // pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLuint = 33300;
    // pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLuint = 33301;
    // pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLuint = 33302;
    // pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLuint = 33303;
    // pub const FRAMEBUFFER_DEFAULT: GLuint = 33304;
    // pub const FRAMEBUFFER_UNDEFINED: GLuint = 33305;
    // pub const DEPTH_STENCIL_ATTACHMENT: GLuint = 33306;
    // pub const DEPTH_STENCIL: GLuint = 34041;
    // pub const UNSIGNED_INT_24_8: GLuint = 34042;
    // pub const DEPTH24_STENCIL8: GLuint = 35056;
    // pub const UNSIGNED_NORMALIZED: GLuint = 35863;
    // pub const DRAW_FRAMEBUFFER_BINDING: GLuint = 36006;
    // pub const READ_FRAMEBUFFER: GLuint = 36008;
    // pub const DRAW_FRAMEBUFFER: GLuint = 36009;
    // pub const READ_FRAMEBUFFER_BINDING: GLuint = 36010;
    // pub const RENDERBUFFER_SAMPLES: GLuint = 36011;
    // pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLuint = 36052;
    // pub const MAX_COLOR_ATTACHMENTS: GLuint = 36063;
    // pub const COLOR_ATTACHMENT1: GLuint = 36065;
    // pub const COLOR_ATTACHMENT2: GLuint = 36066;
    // pub const COLOR_ATTACHMENT3: GLuint = 36067;
    // pub const COLOR_ATTACHMENT4: GLuint = 36068;
    // pub const COLOR_ATTACHMENT5: GLuint = 36069;
    // pub const COLOR_ATTACHMENT6: GLuint = 36070;
    // pub const COLOR_ATTACHMENT7: GLuint = 36071;
    // pub const COLOR_ATTACHMENT8: GLuint = 36072;
    // pub const COLOR_ATTACHMENT9: GLuint = 36073;
    // pub const COLOR_ATTACHMENT10: GLuint = 36074;
    // pub const COLOR_ATTACHMENT11: GLuint = 36075;
    // pub const COLOR_ATTACHMENT12: GLuint = 36076;
    // pub const COLOR_ATTACHMENT13: GLuint = 36077;
    // pub const COLOR_ATTACHMENT14: GLuint = 36078;
    // pub const COLOR_ATTACHMENT15: GLuint = 36079;
    // pub const COLOR_ATTACHMENT16: GLuint = 36080;
    // pub const COLOR_ATTACHMENT17: GLuint = 36081;
    // pub const COLOR_ATTACHMENT18: GLuint = 36082;
    // pub const COLOR_ATTACHMENT19: GLuint = 36083;
    // pub const COLOR_ATTACHMENT20: GLuint = 36084;
    // pub const COLOR_ATTACHMENT21: GLuint = 36085;
    // pub const COLOR_ATTACHMENT22: GLuint = 36086;
    // pub const COLOR_ATTACHMENT23: GLuint = 36087;
    // pub const COLOR_ATTACHMENT24: GLuint = 36088;
    // pub const COLOR_ATTACHMENT25: GLuint = 36089;
    // pub const COLOR_ATTACHMENT26: GLuint = 36090;
    // pub const COLOR_ATTACHMENT27: GLuint = 36091;
    // pub const COLOR_ATTACHMENT28: GLuint = 36092;
    // pub const COLOR_ATTACHMENT29: GLuint = 36093;
    // pub const COLOR_ATTACHMENT30: GLuint = 36094;
    // pub const COLOR_ATTACHMENT31: GLuint = 36095;
    // pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLuint = 36182;
    // pub const MAX_SAMPLES: GLuint = 36183;
    // pub const HALF_FLOAT: GLuint = 5131;
    // pub const MAP_READ_BIT: GLuint = 1;
    // pub const MAP_WRITE_BIT: GLuint = 2;
    // pub const MAP_INVALIDATE_RANGE_BIT: GLuint = 4;
    // pub const MAP_INVALIDATE_BUFFER_BIT: GLuint = 8;
    // pub const MAP_FLUSH_EXPLICIT_BIT: GLuint = 16;
    // pub const MAP_UNSYNCHRONIZED_BIT: GLuint = 32;
    // pub const RG: GLuint = 33319;
    // pub const RG_INTEGER: GLuint = 33320;
    // pub const R8: GLuint = 33321;
    // pub const RG8: GLuint = 33323;
    // pub const R16F: GLuint = 33325;
    // pub const R32F: GLuint = 33326;
    // pub const RG16F: GLuint = 33327;
    // pub const RG32F: GLuint = 33328;
    // pub const R8I: GLuint = 33329;
    // pub const R8UI: GLuint = 33330;
    // pub const R16I: GLuint = 33331;
    // pub const R16UI: GLuint = 33332;
    // pub const R32I: GLuint = 33333;
    // pub const R32UI: GLuint = 33334;
    // pub const RG8I: GLuint = 33335;
    // pub const RG8UI: GLuint = 33336;
    // pub const RG16I: GLuint = 33337;
    // pub const RG16UI: GLuint = 33338;
    // pub const RG32I: GLuint = 33339;
    // pub const RG32UI: GLuint = 33340;
    // pub const VERTEX_ARRAY_BINDING: GLuint = 34229;
    // pub const R8_SNORM: GLuint = 36756;
    // pub const RG8_SNORM: GLuint = 36757;
    // pub const RGB8_SNORM: GLuint = 36758;
    // pub const RGBA8_SNORM: GLuint = 36759;
    // pub const SIGNED_NORMALIZED: GLuint = 36764;
    // pub const PRIMITIVE_RESTART_FIXED_INDEX: GLuint = 36201;
    // pub const COPY_READ_BUFFER: GLuint = 36662;
    // pub const COPY_WRITE_BUFFER: GLuint = 36663;
    // pub const COPY_READ_BUFFER_BINDING: GLuint = 36662;
    // pub const COPY_WRITE_BUFFER_BINDING: GLuint = 36663;
    // pub const UNIFORM_BUFFER: GLuint = 35345;
    // pub const UNIFORM_BUFFER_BINDING: GLuint = 35368;
    // pub const UNIFORM_BUFFER_START: GLuint = 35369;
    // pub const UNIFORM_BUFFER_SIZE: GLuint = 35370;
    // pub const MAX_VERTEX_UNIFORM_BLOCKS: GLuint = 35371;
    // pub const MAX_FRAGMENT_UNIFORM_BLOCKS: GLuint = 35373;
    // pub const MAX_COMBINED_UNIFORM_BLOCKS: GLuint = 35374;
    // pub const MAX_UNIFORM_BUFFER_BINDINGS: GLuint = 35375;
    // pub const MAX_UNIFORM_BLOCK_SIZE: GLuint = 35376;
    // pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLuint = 35377;
    // pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLuint = 35379;
    // pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLuint = 35380;
    // pub const ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GLuint = 35381;
    // pub const ACTIVE_UNIFORM_BLOCKS: GLuint = 35382;
    // pub const UNIFORM_TYPE: GLuint = 35383;
    // pub const UNIFORM_SIZE: GLuint = 35384;
    // pub const UNIFORM_NAME_LENGTH: GLuint = 35385;
    // pub const UNIFORM_BLOCK_INDEX: GLuint = 35386;
    // pub const UNIFORM_OFFSET: GLuint = 35387;
    // pub const UNIFORM_ARRAY_STRIDE: GLuint = 35388;
    // pub const UNIFORM_MATRIX_STRIDE: GLuint = 35389;
    // pub const UNIFORM_IS_ROW_MAJOR: GLuint = 35390;
    // pub const UNIFORM_BLOCK_BINDING: GLuint = 35391;
    // pub const UNIFORM_BLOCK_DATA_SIZE: GLuint = 35392;
    // pub const UNIFORM_BLOCK_NAME_LENGTH: GLuint = 35393;
    // pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLuint = 35394;
    // pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLuint = 35395;
    // pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLuint = 35396;
    // pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLuint = 35398;
    // pub const INVALID_INDEX: GLuint = 4294967295;
    // pub const MAX_VERTEX_OUTPUT_COMPONENTS: GLuint = 37154;
    // pub const MAX_FRAGMENT_INPUT_COMPONENTS: GLuint = 37157;
    // pub const MAX_SERVER_WAIT_TIMEOUT: GLuint = 37137;
    // pub const OBJECT_TYPE: GLuint = 37138;
    // pub const SYNC_CONDITION: GLuint = 37139;
    // pub const SYNC_STATUS: GLuint = 37140;
    // pub const SYNC_FLAGS: GLuint = 37141;
    // pub const SYNC_FENCE: GLuint = 37142;
    // pub const SYNC_GPU_COMMANDS_COMPLETE: GLuint = 37143;
    // pub const UNSIGNALED: GLuint = 37144;
    // pub const SIGNALED: GLuint = 37145;
    // pub const ALREADY_SIGNALED: GLuint = 37146;
    // pub const TIMEOUT_EXPIRED: GLuint = 37147;
    // pub const CONDITION_SATISFIED: GLuint = 37148;
    // pub const WAIT_FAILED: GLuint = 37149;
    // pub const SYNC_FLUSH_COMMANDS_BIT: GLuint = 1;
    // pub const TIMEOUT_IGNORED: i32 = -1;
    // pub const VERTEX_ATTRIB_ARRAY_DIVISOR: GLuint = 35070;
    // pub const ANY_SAMPLES_PASSED: GLuint = 35887;
    // pub const ANY_SAMPLES_PASSED_CONSERVATIVE: GLuint = 36202;
    // pub const SAMPLER_BINDING: GLuint = 35097;
    // pub const RGB10_A2UI: GLuint = 36975;
    // pub const TEXTURE_SWIZZLE_R: GLuint = 36418;
    // pub const TEXTURE_SWIZZLE_G: GLuint = 36419;
    // pub const TEXTURE_SWIZZLE_B: GLuint = 36420;
    // pub const TEXTURE_SWIZZLE_A: GLuint = 36421;
    // pub const GREEN: GLuint = 6404;
    // pub const BLUE: GLuint = 6405;
    // pub const INT_2_10_10_10_REV: GLuint = 36255;
    // pub const TRANSFORM_FEEDBACK: GLuint = 36386;
    // pub const TRANSFORM_FEEDBACK_PAUSED: GLuint = 36387;
    // pub const TRANSFORM_FEEDBACK_ACTIVE: GLuint = 36388;
    // pub const TRANSFORM_FEEDBACK_BINDING: GLuint = 36389;
    // pub const PROGRAM_BINARY_RETRIEVABLE_HINT: GLuint = 33367;
    // pub const PROGRAM_BINARY_LENGTH: GLuint = 34625;
    // pub const NUM_PROGRAM_BINARY_FORMATS: GLuint = 34814;
    // pub const PROGRAM_BINARY_FORMATS: GLuint = 34815;
    // pub const COMPRESSED_R11_EAC: GLuint = 37488;
    // pub const COMPRESSED_SIGNED_R11_EAC: GLuint = 37489;
    // pub const COMPRESSED_RG11_EAC: GLuint = 37490;
    // pub const COMPRESSED_SIGNED_RG11_EAC: GLuint = 37491;
    // pub const COMPRESSED_RGB8_ETC2: GLuint = 37492;
    // pub const COMPRESSED_SRGB8_ETC2: GLuint = 37493;
    // pub const COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLuint = 37494;
    // pub const COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLuint = 37495;
    // pub const COMPRESSED_RGBA8_ETC2_EAC: GLuint = 37496;
    // pub const COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: GLuint = 37497;
    // pub const TEXTURE_IMMUTABLE_FORMAT: GLuint = 37167;
    // pub const MAX_ELEMENT_INDEX: GLuint = 36203;
    // pub const NUM_SAMPLE_COUNTS: GLuint = 37760;
    // pub const TEXTURE_IMMUTABLE_LEVELS: GLuint = 33503;

    // -------------------------------------------------------------------------------------------------
    // FUNCTIONS v3.0
    // -------------------------------------------------------------------------------------------------

    pub fn begin_query(target: GLenum, id: GLuint) {
        unsafe { ffi::glBeginQuery(target, id) }
    }

    pub fn begin_transform_feedback(primitive_mode: GLenum) {
        unsafe { ffi::glBeginTransformFeedback(primitive_mode) }
    }

    pub fn bind_buffer_base(target: GLenum, index: GLuint, buffer: GLuint) {
        unsafe { ffi::glBindBufferBase(target, index, buffer) }
    }

    pub fn bind_buffer_range(
        target: GLenum,
        index: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
    ) {
        unsafe { ffi::glBindBufferRange(target, index, buffer, offset, size) }
    }

    pub fn bind_sampler(unit: GLuint, sampler: GLuint) {
        unsafe { ffi::glBindSampler(unit, sampler) }
    }

    pub fn bind_transform_feedback(target: GLenum, id: GLuint) {
        unsafe { ffi::glBindTransformFeedback(target, id) }
    }

    pub fn bind_vertex_array(array: GLuint) {
        unsafe { ffi::glBindVertexArray(array) }
    }

    pub fn blit_framebuffer(
        src_x0: GLint,
        src_y0: GLint,
        src_x1: GLint,
        src_y1: GLint,
        dst_x0: GLint,
        dst_y0: GLint,
        dst_x1: GLint,
        dst_y1: GLint,
        mask: GLbitfield,
        filter: GLenum,
    ) {
        unsafe {
            ffi::glBlitFramebuffer(
                src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter,
            )
        }
    }

    pub fn clear_buffer_iv(buffer: GLenum, draw_buffer: GLint, value: GLint) {
        unsafe { ffi::glClearBufferiv(buffer, draw_buffer, &value) }
    }

    pub fn clear_buffer_uiv(buffer: GLenum, draw_buffer: GLint, value: GLuint) {
        unsafe { ffi::glClearBufferuiv(buffer, draw_buffer, &value) }
    }

    pub fn clear_buffer_fv(buffer: GLenum, draw_buffer: GLint, value: &[GLfloat; 4]) {
        // TODO: check it and complete other methods
        unsafe { ffi::glClearBufferfv(buffer, draw_buffer, &value[0]) }
    }

    pub fn clear_buffer_fi(buffer: GLenum, draw_buffer: GLint, depth: GLfloat, stencil: GLint) {
        unsafe { ffi::glClearBufferfi(buffer, draw_buffer, depth, stencil) }
    }

    pub fn client_wait_sync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum {
        unsafe { ffi::glClientWaitSync(sync, flags, timeout) }
    }

    pub fn compressed_tex_image_3d<T>(
        target: GLenum,
        level: GLint,
        internal_format: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        image_size: GLsizei,
        buffer: &[T],
    ) {
        unsafe {
            ffi::glCompressedTexImage3D(
                target,
                level,
                internal_format,
                width,
                height,
                depth,
                border,
                image_size,
                buffer.as_ptr() as *const GLvoid,
            )
        }
    }

    pub fn compressed_tex_sub_image_3d<T>(
        target: GLenum,
        level: GLint,
        x_offset: GLint,
        y_offset: GLint,
        z_offset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        image_size: GLsizei,
        buffer: &[T],
    ) {
        unsafe {
            ffi::glCompressedTexSubImage3D(
                target,
                level,
                x_offset,
                y_offset,
                z_offset,
                width,
                height,
                depth,
                format,
                image_size,
                buffer.as_ptr() as *const GLvoid,
            )
        }
    }

    pub fn copy_buffer_sub_data(
        read_target: GLenum,
        write_target: GLenum,
        read_offset: GLintptr,
        write_offset: GLintptr,
        size: GLsizeiptr,
    ) {
        unsafe {
            ffi::glCopyBufferSubData(read_target, write_target, read_offset, write_offset, size)
        }
    }

    pub fn copy_tex_sub_image_3d(
        target: GLenum,
        level: GLint,
        x_offset: GLint,
        y_offset: GLint,
        z_offset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        unsafe {
            ffi::glCopyTexSubImage3D(
                target, level, x_offset, y_offset, z_offset, x, y, width, height,
            )
        }
    }

    pub fn delete_queries(ids: &[GLuint]) {
        unsafe { ffi::glDeleteQueries(ids.len() as GLsizei, ids.as_ptr()) }
    }

    pub fn delete_samplers(samplers: &[GLuint]) {
        unsafe { ffi::glDeleteSamplers(samplers.len() as GLsizei, samplers.as_ptr()) }
    }

    pub fn delete_sync(sync: GLsync) {
        unsafe { ffi::glDeleteSync(sync) }
    }

    pub fn delete_transform_feedbacks(ids: &[GLuint]) {
        unsafe { ffi::glDeleteTransformFeedbacks(ids.len() as GLsizei, ids.as_ptr()) }
    }

    pub fn delete_vertex_arrays(arrays: &[GLuint]) {
        unsafe { ffi::glDeleteVertexArrays(arrays.len() as GLsizei, arrays.as_ptr()) }
    }

    pub fn draw_arrays_instanced(mode: GLenum, first: GLint, count: GLsizei, primcount: GLsizei) {
        unsafe { ffi::glDrawArraysInstanced(mode, first, count, primcount) }
    }

    pub fn draw_buffers(bufs: &[GLenum]) {
        unsafe { ffi::glDrawBuffers(bufs.len() as GLsizei, bufs.as_ptr()) }
    }

    pub fn draw_elements_instanced<T>(
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: &[T],
        primcount: GLsizei,
    ) {
        unsafe {
            ffi::glDrawElementsInstanced(
                mode,
                count,
                type_,
                indices.as_ptr() as *const GLvoid,
                primcount,
            )
        }
    }

    pub fn draw_elements_instanced_offset(
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        offset: GLuint,
        primcount: GLsizei,
    ) {
        unsafe {
            ffi::glDrawElementsInstanced(mode, count, type_, offset as *const GLvoid, primcount)
        }
    }

    pub fn draw_range_elements<T>(
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        type_: GLenum,
        indices: &[T],
    ) {
        unsafe {
            ffi::glDrawRangeElements(
                mode,
                start,
                end,
                count,
                type_,
                indices.as_ptr() as *const GLvoid,
            )
        }
    }

    pub fn end_query(target: GLenum) {
        unsafe { ffi::glEndQuery(target) }
    }

    pub fn end_transform_feedback() {
        unsafe { ffi::glEndTransformFeedback() }
    }

    pub fn fence_sync(condition: GLenum, flags: GLbitfield) -> GLsync {
        unsafe { ffi::glFenceSync(condition, flags) }
    }

    pub fn flush_mapped_buffer_range(target: GLenum, offset: GLintptr, length: GLsizeiptr) {
        unsafe { ffi::glFlushMappedBufferRange(target, offset, length) }
    }

    pub fn framebuffer_texture_layer(
        target: GLenum,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
        layer: GLint,
    ) {
        unsafe { ffi::glFramebufferTextureLayer(target, attachment, texture, level, layer) }
    }

    pub fn gen_queries(count: GLsizei) -> Vec<GLuint> {
        unsafe {
            let mut vec: Vec<GLuint> = Vec::with_capacity(count as usize);

            ffi::glGenQueries(count, vec.as_mut_ptr());

            vec.set_len(count as usize);
            vec
        }
    }

    pub fn gen_samplers(count: GLsizei) -> Vec<GLuint> {
        unsafe {
            let mut vec: Vec<GLuint> = Vec::with_capacity(count as usize);

            ffi::glGenSamplers(count, vec.as_mut_ptr());

            vec.set_len(count as usize);
            vec
        }
    }

    pub fn gen_transform_feedbacks(count: GLsizei) -> Vec<GLuint> {
        unsafe {
            let mut vec: Vec<GLuint> = Vec::with_capacity(count as usize);

            ffi::glGenTransformFeedbacks(count, vec.as_mut_ptr());

            vec.set_len(count as usize);
            vec
        }
    }

    #[inline]
    pub fn gen_vertex_array() -> GLuint {
        unsafe {
            let mut vec: Vec<GLuint> = Vec::with_capacity(1);

            ffi::glGenVertexArrays(1, vec.as_mut_ptr());

            vec.set_len(1);
            vec[0]
        }
    }

    pub fn gen_vertex_arrays(count: GLsizei) -> Vec<GLuint> {
        unsafe {
            let mut vec: Vec<GLuint> = Vec::with_capacity(count as usize);

            ffi::glGenVertexArrays(count, vec.as_mut_ptr());

            vec.set_len(count as usize);
            vec
        }
    }

    pub fn get_active_uniform_blockiv(program: GLuint, index: GLuint, name: GLenum) -> GLint {
        unsafe {
            let mut value: GLint = 0;

            ffi::glGetActiveUniformBlockiv(program, index, name, &mut value);

            value
        }
    }

    pub fn get_active_uniform_block_name(
        program: GLuint,
        index: GLuint,
        max_length: GLsizei,
    ) -> Option<String> {
        unsafe {
            let mut length: GLsizei = 0;
            let mut source = String::with_capacity(max_length as usize);

            ffi::glGetActiveUniformBlockName(
                program,
                index,
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

    // TODO: glGetActiveUniformsiv
    pub fn get_buffer_parameteri64v(target: GLenum, name: GLenum) -> GLint64 {
        unsafe {
            let mut value: GLint64 = 0;

            ffi::glGetBufferParameteri64v(target, name, &mut value);

            value
        }
    }

    // TODO: glGetBufferPointerv
    pub fn get_frag_data_location(program: GLuint, name: &GLchar) -> GLint {
        unsafe { ffi::glGetFragDataLocation(program, name) }
    }

    pub fn get_integer64v(name: GLenum) -> GLint64 {
        unsafe {
            let mut value: GLint64 = 0;

            ffi::glGetInteger64v(name, &mut value);

            value
        }
    }

    pub fn get_integeriv(name: GLenum, index: GLuint) -> GLint {
        unsafe {
            let mut value: GLint = 0;

            ffi::glGetIntegeri_v(name, index, &mut value);

            value
        }
    }

    pub fn get_integer64iv(name: GLenum, index: GLuint) -> GLint64 {
        unsafe {
            let mut value: GLint64 = 0;

            ffi::glGetInteger64i_v(name, index, &mut value);

            value
        }
    }

    pub fn get_internal_formativ(target: GLenum, internalformat: GLenum, name: GLenum) -> GLint {
        unsafe {
            let mut value: GLint = 0;

            ffi::glGetInternalformativ(target, internalformat, name, 1, &mut value);

            value
        }
    }

    pub fn get_program_binary(
        program: GLuint,
        binaryFormat: GLenum,
        max_length: GLsizei,
    ) -> Option<Vec<u8>> {
        unsafe {
            let mut length: GLsizei = 0;
            let mut binary = Vec::with_capacity(max_length as usize);
            let mut binaryFormat = binaryFormat;

            ffi::glGetProgramBinary(
                program,
                max_length,
                &mut length,
                &mut binaryFormat,
                binary.as_mut_ptr() as *mut GLvoid,
            );

            if length > 0 {
                binary.set_len(length as usize);
                binary.truncate(length as usize);

                Some(binary)
            } else {
                None
            }
        }
    }

    pub fn get_queryiv(target: GLenum, name: GLenum) -> GLint {
        unsafe {
            let mut value: GLint = 0;

            ffi::glGetProgramiv(target, name, &mut value);

            value
        }
    }

    pub fn get_query_objectiv(id: GLuint, name: GLenum) -> GLuint {
        unsafe {
            let mut value: GLuint = 0;

            ffi::glGetQueryObjectuiv(id, name, &mut value);

            value
        }
    }

    pub fn get_sampler_parameterfv(sampler: GLuint, name: GLenum) -> GLfloat {
        unsafe {
            let mut value: GLfloat = 0.0;

            ffi::glGetSamplerParameterfv(sampler, name, &mut value);

            value
        }
    }

    pub fn get_sampler_parameteriv(sampler: GLuint, name: GLenum) -> GLint {
        unsafe {
            let mut value: GLint = 0;

            ffi::glGetSamplerParameteriv(sampler, name, &mut value);

            value
        }
    }

    pub fn get_stringi(name: GLenum, index: GLuint) -> Option<String> {
        unsafe {
            let c_str = ffi::glGetStringi(name, index);

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

    pub fn get_synciv(sync: GLsync, name: GLenum) -> GLint {
        unsafe {
            let mut value: GLint = 0;
            let mut length: GLsizei = 0;

            ffi::glGetSynciv(sync, name, 1, &mut length, &mut value);

            value
        }
    }

    // TODO: glGetTransformFeedbackVarying
    pub fn get_uniformuiv(program: GLuint, location: GLint) -> GLuint {
        unsafe {
            let mut value: GLuint = 0;

            ffi::glGetUniformuiv(program, location, &mut value);

            value
        }
    }

    pub fn get_uniform_block_index(program: GLuint, name: &str) -> GLuint {
        unsafe {
            let c_str = CString::new(name).unwrap();

            ffi::glGetUniformBlockIndex(program, c_str.as_ptr() as *const c_char)
        }
    }

    // TODO: glGetUniformIndices
    pub fn get_vertex_attribiiv(index: GLuint, name: GLenum) -> GLint {
        unsafe {
            let mut value: GLint = 0;

            ffi::glGetVertexAttribIiv(index, name, &mut value);

            value
        }
    }

    pub fn get_vertex_attribiuiv(index: GLuint, name: GLenum) -> GLuint {
        unsafe {
            let mut value: GLuint = 0;

            ffi::glGetVertexAttribIuiv(index, name, &mut value);

            value
        }
    }

    pub fn invalidate_framebuffer(target: GLenum, attachments: &[GLenum]) {
        unsafe {
            ffi::glInvalidateFramebuffer(target, attachments.len() as GLsizei, attachments.as_ptr())
        }
    }

    pub fn invalidate_sub_framebuffer(
        target: GLenum,
        attachments: &[GLenum],
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        unsafe {
            ffi::glInvalidateSubFramebuffer(
                target,
                attachments.len() as GLsizei,
                attachments.as_ptr(),
                x,
                y,
                width,
                height,
            )
        }
    }

    pub fn is_query(id: GLuint) -> bool {
        unsafe { ffi::glIsQuery(id) == GL_TRUE }
    }

    pub fn is_sampler(id: GLuint) -> bool {
        unsafe { ffi::glIsSampler(id) == GL_TRUE }
    }

    pub fn is_sync(sync: GLsync) -> bool {
        unsafe { ffi::glIsSync(sync) == GL_TRUE }
    }

    pub fn is_transform_feedback(id: GLuint) -> bool {
        unsafe { ffi::glIsTransformFeedback(id) == GL_TRUE }
    }

    pub fn is_vertex_array(array: GLuint) -> bool {
        unsafe { ffi::glIsVertexArray(array) == GL_TRUE }
    }

    // TODO: glMapBufferRange
    // pub fn map_buffer_range(target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> GLvoid {
    //     unsafe { ffi::glMapBufferRange(target, offset, length, access) }
    // }

    pub fn pause_transform_feedback() {
        unsafe { ffi::glPauseTransformFeedback() }
    }

    pub fn unmap_buffer(target: GLenum) -> bool {
        unsafe { ffi::glUnmapBuffer(target) == GL_TRUE }
    }

    pub fn program_binary<T>(program: GLuint, binary_format: GLenum, data: &[T]) {
        unsafe {
            let length: GLsizei = data.len() as GLsizei;

            ffi::glProgramBinary(
                // program.len() as GLsizei,
                program,
                binary_format,
                data.as_ptr() as *const GLvoid,
                length,
            )
        }
    }

    pub fn program_parameteri(program: GLuint, name: GLenum, value: GLint) {
        unsafe { ffi::glProgramParameteri(program, name, value) }
    }

    pub fn read_buffer(src: GLenum) {
        unsafe { ffi::glReadBuffer(src) }
    }

    pub fn renderbuffer_storage_multisample(
        target: GLenum,
        samples: GLsizei,
        internal_format: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        unsafe {
            ffi::glRenderbufferStorageMultisample(target, samples, internal_format, width, height)
        }
    }

    pub fn resume_transform_feedback() {
        unsafe { ffi::glResumeTransformFeedback() }
    }

    pub fn sampler_parameterf(sampler: GLuint, name: GLenum, param: GLfloat) {
        unsafe { ffi::glSamplerParameterf(sampler, name, param) }
    }

    pub fn sampler_parameteri(sampler: GLuint, name: GLenum, param: GLint) {
        unsafe { ffi::glSamplerParameteri(sampler, name, param) }
    }

    pub fn sampler_parameterfv(sampler: GLuint, name: GLenum, params: &[GLfloat]) {
        unsafe { ffi::glSamplerParameterfv(sampler, name, params.as_ptr() as *const GLfloat) }
    }

    pub fn sampler_parameteriv(sampler: GLuint, name: GLenum, params: &[GLint]) {
        unsafe { ffi::glSamplerParameteriv(sampler, name, params.as_ptr() as *const GLint) }
    }

    pub fn tex_image_3d<T>(
        target: GLenum,
        level: GLint,
        internal_format: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        buffer: &[T],
    ) {
        unsafe {
            ffi::glTexImage3D(
                target,
                level,
                internal_format,
                width,
                height,
                depth,
                border,
                format,
                type_,
                buffer.as_ptr() as *const GLvoid,
            )
        }
    }

    pub fn tex_storage_3d(
        target: GLenum,
        level: GLint,
        internal_format: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
    ) {
        unsafe { ffi::glTexStorage3D(target, level, internal_format, width, height, depth) }
    }

    pub fn tex_sub_image_3d<T>(
        target: GLenum,
        level: GLint,
        x_offset: GLint,
        y_offset: GLint,
        z_offset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        buffer: &[T],
    ) {
        unsafe {
            ffi::glTexSubImage3D(
                target,
                level,
                x_offset,
                y_offset,
                z_offset,
                width,
                height,
                depth,
                format,
                type_,
                buffer.as_ptr() as *const GLvoid,
            )
        }
    }

    // TODO: glTransformFeedbackVaryings
    pub fn uniform1ui(location: GLint, x: GLuint) {
        unsafe { ffi::glUniform1ui(location, x) }
    }

    pub fn uniform2ui(location: GLint, x: GLuint, y: GLuint) {
        unsafe { ffi::glUniform2ui(location, x, y) }
    }

    pub fn uniform3ui(location: GLint, x: GLuint, y: GLuint, z: GLuint) {
        unsafe { ffi::glUniform3ui(location, x, y, z) }
    }

    pub fn uniform4ui(location: GLint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) {
        unsafe { ffi::glUniform4ui(location, x, y, z, w) }
    }

    pub fn uniform1uiv(location: GLint, values: &[GLuint]) {
        unsafe { ffi::glUniform1uiv(location, values.len() as GLsizei, values.as_ptr()) }
    }

    pub fn uniform2uiv(location: GLint, values: &[GLuint]) {
        unsafe { ffi::glUniform2uiv(location, (values.len() / 2) as GLsizei, values.as_ptr()) }
    }

    pub fn uniform3uiv(location: GLint, values: &[GLuint]) {
        unsafe { ffi::glUniform3uiv(location, (values.len() / 3) as GLsizei, values.as_ptr()) }
    }

    pub fn uniform4uiv(location: GLint, values: &[GLuint]) {
        unsafe { ffi::glUniform4uiv(location, (values.len() / 4) as GLsizei, values.as_ptr()) }
    }

    pub fn uniform_matrix2x3fv(location: GLint, transpose: bool, values: &[GLfloat]) {
        unsafe {
            ffi::glUniformMatrix2x3fv(
                location,
                (values.len() / 6) as GLsizei,
                transpose as GLboolean,
                values.as_ptr() as *const GLfloat,
            )
        }
    }

    pub fn uniform_matrix3x2fv(location: GLint, transpose: bool, values: &[GLfloat]) {
        unsafe {
            ffi::glUniformMatrix3x2fv(
                location,
                (values.len() / 6) as GLsizei,
                transpose as GLboolean,
                values.as_ptr() as *const GLfloat,
            )
        }
    }

    pub fn uniform_matrix2x4fv(location: GLint, transpose: bool, values: &[GLfloat]) {
        unsafe {
            ffi::glUniformMatrix2x4fv(
                location,
                (values.len() / 8) as GLsizei,
                transpose as GLboolean,
                values.as_ptr() as *const GLfloat,
            )
        }
    }

    pub fn uniform_matrix4x2fv(location: GLint, transpose: bool, values: &[GLfloat]) {
        unsafe {
            ffi::glUniformMatrix2fv(
                location,
                (values.len() / 8) as GLsizei,
                transpose as GLboolean,
                values.as_ptr() as *const GLfloat,
            )
        }
    }

    pub fn uniform_matrix3x4fv(location: GLint, transpose: bool, values: &[GLfloat]) {
        unsafe {
            ffi::glUniformMatrix3x4fv(
                location,
                (values.len() / 12) as GLsizei,
                transpose as GLboolean,
                values.as_ptr() as *const GLfloat,
            )
        }
    }

    pub fn uniform_matrix4x3fv(location: GLint, transpose: bool, values: &[GLfloat]) {
        unsafe {
            ffi::glUniformMatrix4x3fv(
                location,
                (values.len() / 12) as GLsizei,
                transpose as GLboolean,
                values.as_ptr() as *const GLfloat,
            )
        }
    }

    pub fn uniform_block_binding(program: GLuint, index: GLuint, binding: GLuint) {
        unsafe { ffi::glUniformBlockBinding(program, index, binding) }
    }

    pub fn vertex_attribi4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) {
        unsafe { ffi::glVertexAttribI4i(index, x, y, z, w) }
    }
    pub fn vertex_attribi4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) {
        unsafe { ffi::glVertexAttribI4ui(index, x, y, z, w) }
    }
    pub fn vertex_attribi4iv(index: GLuint, values: &[GLint]) {
        unsafe { ffi::glVertexAttribI4iv(index, values.as_ptr()) }
    }
    pub fn vertex_attribi4uiv(index: GLuint, values: &[GLuint]) {
        unsafe { ffi::glVertexAttribI4uiv(index, values.as_ptr()) }
    }

    pub fn vertex_attrib_divisor(index: GLuint, divisor: GLuint) {
        unsafe { ffi::glVertexAttribDivisor(index, divisor) }
    }

    pub fn vertex_attrib_ipointer<T>(
        index: GLuint,
        size: GLint,
        type_: GLenum,
        stride: GLsizei,
        buffer: &[T],
    ) {
        unsafe {
            if buffer.len() == 0 {
                ffi::glVertexAttribIPointer(
                    index,
                    size,
                    type_,
                    stride,
                    &0 as *const i32 as *const GLvoid,
                )
            } else {
                ffi::glVertexAttribIPointer(
                    index,
                    size,
                    type_,
                    stride,
                    buffer.as_ptr() as *const GLvoid,
                )
            }
        }
    }

    pub fn wait_sync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) {
        unsafe { ffi::glWaitSync(sync, flags, timeout) }
    }
}
