#![allow(non_upper_case_globals)]
//! Contains all the GL enumerated values.
//!
//! In C these are called 'enums', but in Rust we call them a 'const'. Whatever.
use super::types::*;

#[doc = "`GL_3DC_XY_AMD: GLenum = 0x87FA`"]
#[cfg(any(feature = "GL_AMD_compressed_3DC_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_AMD_compressed_3DC_texture"))))]
pub const GL_3DC_XY_AMD: GLenum = 0x87FA;
#[doc = "`GL_3DC_X_AMD: GLenum = 0x87F9`"]
#[cfg(any(feature = "GL_AMD_compressed_3DC_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_AMD_compressed_3DC_texture"))))]
pub const GL_3DC_X_AMD: GLenum = 0x87F9;
#[doc = "`GL_ACCUM_ADJACENT_PAIRS_NV: GLenum = 0x90AD`"]
#[doc = "* **Group:** PathListMode"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_ACCUM_ADJACENT_PAIRS_NV: GLenum = 0x90AD;
#[doc = "`GL_ACTIVE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D9`"]
#[doc = "* **Group:** ProgramPropertyARB"]
pub const GL_ACTIVE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D9;
#[doc = "`GL_ACTIVE_ATTRIBUTES: GLenum = 0x8B89`"]
#[doc = "* **Group:** ProgramPropertyARB"]
pub const GL_ACTIVE_ATTRIBUTES: GLenum = 0x8B89;
#[doc = "`GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = 0x8B8A`"]
#[doc = "* **Group:** ProgramPropertyARB"]
pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = 0x8B8A;
#[doc = "`GL_ACTIVE_PROGRAM: GLenum = 0x8259`"]
#[doc = "* **Group:** PipelineParameterName"]
pub const GL_ACTIVE_PROGRAM: GLenum = 0x8259;
#[doc = "`GL_ACTIVE_PROGRAM_EXT: GLenum = 0x8259`"]
#[cfg(any(feature = "GL_EXT_separate_shader_objects"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_separate_shader_objects"))))]
pub const GL_ACTIVE_PROGRAM_EXT: GLenum = 0x8259;
#[doc = "`GL_ACTIVE_RESOURCES: GLenum = 0x92F5`"]
#[doc = "* **Group:** ProgramInterfacePName"]
pub const GL_ACTIVE_RESOURCES: GLenum = 0x92F5;
#[doc = "`GL_ACTIVE_TEXTURE: GLenum = 0x84E0`"]
#[doc = "* **Group:** GetPName"]
pub const GL_ACTIVE_TEXTURE: GLenum = 0x84E0;
#[doc = "`GL_ACTIVE_UNIFORMS: GLenum = 0x8B86`"]
#[doc = "* **Group:** ProgramPropertyARB"]
pub const GL_ACTIVE_UNIFORMS: GLenum = 0x8B86;
#[doc = "`GL_ACTIVE_UNIFORM_BLOCKS: GLenum = 0x8A36`"]
#[doc = "* **Group:** ProgramPropertyARB"]
pub const GL_ACTIVE_UNIFORM_BLOCKS: GLenum = 0x8A36;
#[doc = "`GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GLenum = 0x8A35`"]
#[doc = "* **Group:** ProgramPropertyARB"]
pub const GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GLenum = 0x8A35;
#[doc = "`GL_ACTIVE_UNIFORM_MAX_LENGTH: GLenum = 0x8B87`"]
#[doc = "* **Group:** ProgramPropertyARB"]
pub const GL_ACTIVE_UNIFORM_MAX_LENGTH: GLenum = 0x8B87;
#[doc = "`GL_ACTIVE_VARIABLES: GLenum = 0x9305`"]
#[doc = "* **Group:** ProgramResourceProperty"]
pub const GL_ACTIVE_VARIABLES: GLenum = 0x9305;
#[doc = "`GL_ADJACENT_PAIRS_NV: GLenum = 0x90AE`"]
#[doc = "* **Group:** PathListMode"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_ADJACENT_PAIRS_NV: GLenum = 0x90AE;
#[doc = "`GL_AFFINE_2D_NV: GLenum = 0x9092`"]
#[doc = "* **Group:** PathTransformType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_AFFINE_2D_NV: GLenum = 0x9092;
#[doc = "`GL_AFFINE_3D_NV: GLenum = 0x9094`"]
#[doc = "* **Group:** PathTransformType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_AFFINE_3D_NV: GLenum = 0x9094;
#[doc = "`GL_ALIASED_LINE_WIDTH_RANGE: GLenum = 0x846E`"]
#[doc = "* **Group:** GetPName"]
pub const GL_ALIASED_LINE_WIDTH_RANGE: GLenum = 0x846E;
#[doc = "`GL_ALIASED_POINT_SIZE_RANGE: GLenum = 0x846D`"]
#[doc = "* **Group:** GetPName"]
pub const GL_ALIASED_POINT_SIZE_RANGE: GLenum = 0x846D;
#[doc = "`GL_ALL_BARRIER_BITS: GLbitfield = 0xFFFFFFFF`"]
#[doc = "* **Group:** MemoryBarrierMask"]
pub const GL_ALL_BARRIER_BITS: GLbitfield = 0xFFFFFFFF;
#[doc = "`GL_ALL_COMPLETED_NV: GLenum = 0x84F2`"]
#[doc = "* **Group:** FenceConditionNV"]
#[cfg(any(feature = "GL_NV_fence"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_fence"))))]
pub const GL_ALL_COMPLETED_NV: GLenum = 0x84F2;
#[doc = "`GL_ALL_SHADER_BITS: GLbitfield = 0xFFFFFFFF`"]
#[doc = "* **Group:** UseProgramStageMask"]
pub const GL_ALL_SHADER_BITS: GLbitfield = 0xFFFFFFFF;
#[doc = "`GL_ALL_SHADER_BITS_EXT: GLbitfield = 0xFFFFFFFF`"]
#[doc = "* **Group:** UseProgramStageMask"]
#[cfg(any(feature = "GL_EXT_separate_shader_objects"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_separate_shader_objects"))))]
pub const GL_ALL_SHADER_BITS_EXT: GLbitfield = 0xFFFFFFFF;
#[doc = "`GL_ALPHA: GLenum = 0x1906`"]
#[doc = "* **Groups:** TextureSwizzle, CombinerPortionNV, PathColorFormat, CombinerComponentUsageNV, PixelFormat"]
pub const GL_ALPHA: GLenum = 0x1906;
#[doc = "`GL_ALPHA16F_EXT: GLenum = 0x881C`"]
#[cfg(any(feature = "GL_EXT_texture_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_storage"))))]
pub const GL_ALPHA16F_EXT: GLenum = 0x881C;
#[doc = "`GL_ALPHA32F_EXT: GLenum = 0x8816`"]
#[cfg(any(feature = "GL_EXT_texture_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_storage"))))]
pub const GL_ALPHA32F_EXT: GLenum = 0x8816;
#[doc = "`GL_ALPHA8_EXT: GLenum = 0x803C`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_storage"))))]
pub const GL_ALPHA8_EXT: GLenum = 0x803C;
#[doc = "`GL_ALPHA8_OES: GLenum = 0x803C`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_required_internalformat"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_required_internalformat"))))]
pub const GL_ALPHA8_OES: GLenum = 0x803C;
#[doc = "`GL_ALPHA_BITS: GLenum = 0x0D55`"]
#[doc = "* **Group:** GetPName"]
pub const GL_ALPHA_BITS: GLenum = 0x0D55;
#[doc = "`GL_ALPHA_TEST_FUNC_QCOM: GLenum = 0x0BC1`"]
#[doc = "* **Group:** GetPName"]
#[cfg(any(feature = "GL_QCOM_alpha_test"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_alpha_test"))))]
pub const GL_ALPHA_TEST_FUNC_QCOM: GLenum = 0x0BC1;
#[doc = "`GL_ALPHA_TEST_QCOM: GLenum = 0x0BC0`"]
#[doc = "* **Group:** GetPName"]
#[cfg(any(feature = "GL_QCOM_alpha_test"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_alpha_test"))))]
pub const GL_ALPHA_TEST_QCOM: GLenum = 0x0BC0;
#[doc = "`GL_ALPHA_TEST_REF_QCOM: GLenum = 0x0BC2`"]
#[doc = "* **Group:** GetPName"]
#[cfg(any(feature = "GL_QCOM_alpha_test"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_alpha_test"))))]
pub const GL_ALPHA_TEST_REF_QCOM: GLenum = 0x0BC2;
#[doc = "`GL_ALREADY_SIGNALED: GLenum = 0x911A`"]
#[doc = "* **Group:** SyncStatus"]
pub const GL_ALREADY_SIGNALED: GLenum = 0x911A;
#[doc = "`GL_ALREADY_SIGNALED_APPLE: GLenum = 0x911A`"]
#[cfg(any(feature = "GL_APPLE_sync"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_sync"))))]
pub const GL_ALREADY_SIGNALED_APPLE: GLenum = 0x911A;
#[doc = "`GL_ALWAYS: GLenum = 0x0207`"]
#[doc = "* **Groups:** StencilFunction, IndexFunctionEXT, AlphaFunction, DepthFunction"]
pub const GL_ALWAYS: GLenum = 0x0207;
#[doc = "`GL_ANY_SAMPLES_PASSED: GLenum = 0x8C2F`"]
#[doc = "* **Group:** QueryTarget"]
pub const GL_ANY_SAMPLES_PASSED: GLenum = 0x8C2F;
#[doc = "`GL_ANY_SAMPLES_PASSED_CONSERVATIVE: GLenum = 0x8D6A`"]
#[doc = "* **Group:** QueryTarget"]
pub const GL_ANY_SAMPLES_PASSED_CONSERVATIVE: GLenum = 0x8D6A;
#[doc = "`GL_ANY_SAMPLES_PASSED_CONSERVATIVE_EXT: GLenum = 0x8D6A`"]
#[cfg(any(feature = "GL_EXT_occlusion_query_boolean"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_occlusion_query_boolean"))))]
pub const GL_ANY_SAMPLES_PASSED_CONSERVATIVE_EXT: GLenum = 0x8D6A;
#[doc = "`GL_ANY_SAMPLES_PASSED_EXT: GLenum = 0x8C2F`"]
#[cfg(any(feature = "GL_EXT_occlusion_query_boolean"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_occlusion_query_boolean"))))]
pub const GL_ANY_SAMPLES_PASSED_EXT: GLenum = 0x8C2F;
#[doc = "`GL_ARC_TO_NV: GLenum = 0xFE`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_ARC_TO_NV: GLenum = 0xFE;
#[doc = "`GL_ARRAY_BUFFER: GLenum = 0x8892`"]
#[doc = "* **Groups:** CopyBufferSubDataTarget, BufferTargetARB, BufferStorageTarget"]
pub const GL_ARRAY_BUFFER: GLenum = 0x8892;
#[doc = "`GL_ARRAY_BUFFER_BINDING: GLenum = 0x8894`"]
#[doc = "* **Group:** GetPName"]
pub const GL_ARRAY_BUFFER_BINDING: GLenum = 0x8894;
#[doc = "`GL_ARRAY_SIZE: GLenum = 0x92FB`"]
#[doc = "* **Group:** ProgramResourceProperty"]
pub const GL_ARRAY_SIZE: GLenum = 0x92FB;
#[doc = "`GL_ARRAY_STRIDE: GLenum = 0x92FE`"]
#[doc = "* **Group:** ProgramResourceProperty"]
pub const GL_ARRAY_STRIDE: GLenum = 0x92FE;
#[doc = "`GL_ATC_RGBA_EXPLICIT_ALPHA_AMD: GLenum = 0x8C93`"]
#[cfg(any(feature = "GL_AMD_compressed_ATC_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_AMD_compressed_ATC_texture"))))]
pub const GL_ATC_RGBA_EXPLICIT_ALPHA_AMD: GLenum = 0x8C93;
#[doc = "`GL_ATC_RGBA_INTERPOLATED_ALPHA_AMD: GLenum = 0x87EE`"]
#[cfg(any(feature = "GL_AMD_compressed_ATC_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_AMD_compressed_ATC_texture"))))]
pub const GL_ATC_RGBA_INTERPOLATED_ALPHA_AMD: GLenum = 0x87EE;
#[doc = "`GL_ATC_RGB_AMD: GLenum = 0x8C92`"]
#[cfg(any(feature = "GL_AMD_compressed_ATC_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_AMD_compressed_ATC_texture"))))]
pub const GL_ATC_RGB_AMD: GLenum = 0x8C92;
#[doc = "`GL_ATOMIC_COUNTER_BARRIER_BIT: GLbitfield = 0x00001000`"]
#[doc = "* **Group:** MemoryBarrierMask"]
pub const GL_ATOMIC_COUNTER_BARRIER_BIT: GLbitfield = 0x00001000;
#[doc = "`GL_ATOMIC_COUNTER_BUFFER: GLenum = 0x92C0`"]
#[doc = "* **Groups:** CopyBufferSubDataTarget, BufferTargetARB, BufferStorageTarget"]
pub const GL_ATOMIC_COUNTER_BUFFER: GLenum = 0x92C0;
#[doc = "`GL_ATOMIC_COUNTER_BUFFER_BINDING: GLenum = 0x92C1`"]
#[doc = "* **Group:** AtomicCounterBufferPName"]
pub const GL_ATOMIC_COUNTER_BUFFER_BINDING: GLenum = 0x92C1;
#[doc = "`GL_ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 0x9301`"]
#[doc = "* **Group:** ProgramResourceProperty"]
pub const GL_ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 0x9301;
#[doc = "`GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_MESH_SHADER_NV: GLenum = 0x959E`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_MESH_SHADER_NV: GLenum = 0x959E;
#[doc = "`GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TASK_SHADER_NV: GLenum = 0x959F`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TASK_SHADER_NV: GLenum = 0x959F;
#[doc = "`GL_ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92C3`"]
pub const GL_ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92C3;
#[doc = "`GL_ATOMIC_COUNTER_BUFFER_START: GLenum = 0x92C2`"]
pub const GL_ATOMIC_COUNTER_BUFFER_START: GLenum = 0x92C2;
#[doc = "`GL_ATTACHED_MEMORY_OBJECT_NV: GLenum = 0x95A4`"]
#[cfg(any(feature = "GL_NV_memory_attachment"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_memory_attachment"))))]
pub const GL_ATTACHED_MEMORY_OBJECT_NV: GLenum = 0x95A4;
#[doc = "`GL_ATTACHED_MEMORY_OFFSET_NV: GLenum = 0x95A5`"]
#[cfg(any(feature = "GL_NV_memory_attachment"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_memory_attachment"))))]
pub const GL_ATTACHED_MEMORY_OFFSET_NV: GLenum = 0x95A5;
#[doc = "`GL_ATTACHED_SHADERS: GLenum = 0x8B85`"]
#[doc = "* **Group:** ProgramPropertyARB"]
pub const GL_ATTACHED_SHADERS: GLenum = 0x8B85;
#[doc = "`GL_BACK: GLenum = 0x0405`"]
#[doc = "* **Groups:** ColorBuffer, ColorMaterialFace, CullFaceMode, DrawBufferMode, ReadBufferMode, StencilFaceDirection, MaterialFace"]
pub const GL_BACK: GLenum = 0x0405;
#[doc = "`GL_BEVEL_NV: GLenum = 0x90A6`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_BEVEL_NV: GLenum = 0x90A6;
#[doc = "`GL_BGRA8_EXT: GLenum = 0x93A1`"]
#[cfg(any(
    feature = "GL_APPLE_texture_format_BGRA8888",
    feature = "GL_EXT_texture_storage"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_APPLE_texture_format_BGRA8888",
        feature = "GL_EXT_texture_storage"
    )))
)]
pub const GL_BGRA8_EXT: GLenum = 0x93A1;
#[doc = "`GL_BGRA_EXT: GLenum = 0x80E1`"]
#[cfg(any(
    feature = "GL_APPLE_texture_format_BGRA8888",
    feature = "GL_EXT_read_format_bgra",
    feature = "GL_EXT_texture_format_BGRA8888",
    feature = "GL_MESA_bgra"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_APPLE_texture_format_BGRA8888",
        feature = "GL_EXT_read_format_bgra",
        feature = "GL_EXT_texture_format_BGRA8888",
        feature = "GL_MESA_bgra"
    )))
)]
pub const GL_BGRA_EXT: GLenum = 0x80E1;
#[doc = "`GL_BGRA_IMG: GLenum = 0x80E1`"]
#[cfg(any(feature = "GL_IMG_read_format"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_IMG_read_format"))))]
pub const GL_BGRA_IMG: GLenum = 0x80E1;
#[doc = "`GL_BGR_EXT: GLenum = 0x80E0`"]
#[cfg(any(feature = "GL_MESA_bgra"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_MESA_bgra"))))]
pub const GL_BGR_EXT: GLenum = 0x80E0;
#[doc = "`GL_BINNING_CONTROL_HINT_QCOM: GLenum = 0x8FB0`"]
#[doc = "* **Group:** HintTarget"]
#[cfg(any(feature = "GL_QCOM_binning_control"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_binning_control"))))]
pub const GL_BINNING_CONTROL_HINT_QCOM: GLenum = 0x8FB0;
#[doc = "`GL_BLACKHOLE_RENDER_INTEL: GLenum = 0x83FC`"]
#[cfg(any(feature = "GL_INTEL_blackhole_render"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_INTEL_blackhole_render"))))]
pub const GL_BLACKHOLE_RENDER_INTEL: GLenum = 0x83FC;
#[doc = "`GL_BLEND: GLenum = 0x0BE2`"]
#[doc = "* **Groups:** TextureEnvMode, EnableCap, GetPName"]
pub const GL_BLEND: GLenum = 0x0BE2;
#[doc = "`GL_BLEND_ADVANCED_COHERENT_KHR: GLenum = 0x9285`"]
#[cfg(any(feature = "GL_KHR_blend_equation_advanced_coherent"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_KHR_blend_equation_advanced_coherent")))
)]
pub const GL_BLEND_ADVANCED_COHERENT_KHR: GLenum = 0x9285;
#[doc = "`GL_BLEND_ADVANCED_COHERENT_NV: GLenum = 0x9285`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced_coherent"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_NV_blend_equation_advanced_coherent")))
)]
pub const GL_BLEND_ADVANCED_COHERENT_NV: GLenum = 0x9285;
#[doc = "`GL_BLEND_COLOR: GLenum = 0x8005`"]
#[doc = "* **Group:** GetPName"]
pub const GL_BLEND_COLOR: GLenum = 0x8005;
#[doc = "`GL_BLEND_DST_ALPHA: GLenum = 0x80CA`"]
#[doc = "* **Group:** GetPName"]
pub const GL_BLEND_DST_ALPHA: GLenum = 0x80CA;
#[doc = "`GL_BLEND_DST_RGB: GLenum = 0x80C8`"]
#[doc = "* **Group:** GetPName"]
pub const GL_BLEND_DST_RGB: GLenum = 0x80C8;
#[doc = "`GL_BLEND_EQUATION: GLenum = 0x8009`"]
pub const GL_BLEND_EQUATION: GLenum = 0x8009;
#[doc = "`GL_BLEND_EQUATION_ALPHA: GLenum = 0x883D`"]
#[doc = "* **Group:** GetPName"]
pub const GL_BLEND_EQUATION_ALPHA: GLenum = 0x883D;
#[doc = "`GL_BLEND_EQUATION_RGB: GLenum = 0x8009`"]
#[doc = "* **Group:** GetPName"]
pub const GL_BLEND_EQUATION_RGB: GLenum = 0x8009;
#[doc = "`GL_BLEND_OVERLAP_NV: GLenum = 0x9281`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_BLEND_OVERLAP_NV: GLenum = 0x9281;
#[doc = "`GL_BLEND_PREMULTIPLIED_SRC_NV: GLenum = 0x9280`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_BLEND_PREMULTIPLIED_SRC_NV: GLenum = 0x9280;
#[doc = "`GL_BLEND_SRC_ALPHA: GLenum = 0x80CB`"]
#[doc = "* **Group:** GetPName"]
pub const GL_BLEND_SRC_ALPHA: GLenum = 0x80CB;
#[doc = "`GL_BLEND_SRC_RGB: GLenum = 0x80C9`"]
#[doc = "* **Group:** GetPName"]
pub const GL_BLEND_SRC_RGB: GLenum = 0x80C9;
#[doc = "`GL_BLOCK_INDEX: GLenum = 0x92FD`"]
#[doc = "* **Group:** ProgramResourceProperty"]
pub const GL_BLOCK_INDEX: GLenum = 0x92FD;
#[doc = "`GL_BLUE: GLenum = 0x1905`"]
#[doc = "* **Groups:** TextureSwizzle, CombinerComponentUsageNV, PixelFormat"]
pub const GL_BLUE: GLenum = 0x1905;
#[doc = "`GL_BLUE_BITS: GLenum = 0x0D54`"]
#[doc = "* **Group:** GetPName"]
pub const GL_BLUE_BITS: GLenum = 0x0D54;
#[doc = "`GL_BLUE_NV: GLenum = 0x1905`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_BLUE_NV: GLenum = 0x1905;
#[doc = "`GL_BOLD_BIT_NV: GLbitfield = 0x01`"]
#[doc = "* **Group:** PathFontStyle"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_BOLD_BIT_NV: GLbitfield = 0x01;
#[doc = "`GL_BOOL: GLenum = 0x8B56`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_BOOL: GLenum = 0x8B56;
#[doc = "`GL_BOOL_VEC2: GLenum = 0x8B57`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_BOOL_VEC2: GLenum = 0x8B57;
#[doc = "`GL_BOOL_VEC3: GLenum = 0x8B58`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_BOOL_VEC3: GLenum = 0x8B58;
#[doc = "`GL_BOOL_VEC4: GLenum = 0x8B59`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_BOOL_VEC4: GLenum = 0x8B59;
#[doc = "`GL_BOUNDING_BOX_NV: GLenum = 0x908D`"]
#[doc = "* **Group:** PathCoverMode"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_BOUNDING_BOX_NV: GLenum = 0x908D;
#[doc = "`GL_BOUNDING_BOX_OF_BOUNDING_BOXES_NV: GLenum = 0x909C`"]
#[doc = "* **Group:** PathCoverMode"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_BOUNDING_BOX_OF_BOUNDING_BOXES_NV: GLenum = 0x909C;
#[doc = "`GL_BUFFER: GLenum = 0x82E0`"]
#[doc = "* **Group:** ObjectIdentifier"]
pub const GL_BUFFER: GLenum = 0x82E0;
#[doc = "`GL_BUFFER_ACCESS_FLAGS: GLenum = 0x911F`"]
#[doc = "* **Groups:** VertexBufferObjectParameter, BufferPNameARB"]
pub const GL_BUFFER_ACCESS_FLAGS: GLenum = 0x911F;
#[doc = "`GL_BUFFER_ACCESS_OES: GLenum = 0x88BB`"]
#[cfg(any(feature = "GL_OES_mapbuffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_mapbuffer"))))]
pub const GL_BUFFER_ACCESS_OES: GLenum = 0x88BB;
#[doc = "`GL_BUFFER_BINDING: GLenum = 0x9302`"]
#[doc = "* **Group:** ProgramResourceProperty"]
pub const GL_BUFFER_BINDING: GLenum = 0x9302;
#[doc = "`GL_BUFFER_DATA_SIZE: GLenum = 0x9303`"]
#[doc = "* **Group:** ProgramResourceProperty"]
pub const GL_BUFFER_DATA_SIZE: GLenum = 0x9303;
#[doc = "`GL_BUFFER_IMMUTABLE_STORAGE_EXT: GLenum = 0x821F`"]
#[cfg(any(feature = "GL_EXT_buffer_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_buffer_storage"))))]
pub const GL_BUFFER_IMMUTABLE_STORAGE_EXT: GLenum = 0x821F;
#[doc = "`GL_BUFFER_KHR: GLenum = 0x82E0`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_BUFFER_KHR: GLenum = 0x82E0;
#[doc = "`GL_BUFFER_MAPPED: GLenum = 0x88BC`"]
#[doc = "* **Groups:** VertexBufferObjectParameter, BufferPNameARB"]
pub const GL_BUFFER_MAPPED: GLenum = 0x88BC;
#[doc = "`GL_BUFFER_MAPPED_OES: GLenum = 0x88BC`"]
#[cfg(any(feature = "GL_OES_mapbuffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_mapbuffer"))))]
pub const GL_BUFFER_MAPPED_OES: GLenum = 0x88BC;
#[doc = "`GL_BUFFER_MAP_LENGTH: GLenum = 0x9120`"]
#[doc = "* **Groups:** VertexBufferObjectParameter, BufferPNameARB"]
pub const GL_BUFFER_MAP_LENGTH: GLenum = 0x9120;
#[doc = "`GL_BUFFER_MAP_OFFSET: GLenum = 0x9121`"]
#[doc = "* **Groups:** VertexBufferObjectParameter, BufferPNameARB"]
pub const GL_BUFFER_MAP_OFFSET: GLenum = 0x9121;
#[doc = "`GL_BUFFER_MAP_POINTER: GLenum = 0x88BD`"]
#[doc = "* **Group:** BufferPointerNameARB"]
pub const GL_BUFFER_MAP_POINTER: GLenum = 0x88BD;
#[doc = "`GL_BUFFER_MAP_POINTER_OES: GLenum = 0x88BD`"]
#[cfg(any(feature = "GL_OES_mapbuffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_mapbuffer"))))]
pub const GL_BUFFER_MAP_POINTER_OES: GLenum = 0x88BD;
#[doc = "`GL_BUFFER_OBJECT_EXT: GLenum = 0x9151`"]
#[cfg(any(feature = "GL_EXT_debug_label"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_debug_label"))))]
pub const GL_BUFFER_OBJECT_EXT: GLenum = 0x9151;
#[doc = "`GL_BUFFER_SIZE: GLenum = 0x8764`"]
#[doc = "* **Groups:** VertexBufferObjectParameter, BufferPNameARB"]
pub const GL_BUFFER_SIZE: GLenum = 0x8764;
#[doc = "`GL_BUFFER_STORAGE_FLAGS_EXT: GLenum = 0x8220`"]
#[cfg(any(feature = "GL_EXT_buffer_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_buffer_storage"))))]
pub const GL_BUFFER_STORAGE_FLAGS_EXT: GLenum = 0x8220;
#[doc = "`GL_BUFFER_UPDATE_BARRIER_BIT: GLbitfield = 0x00000200`"]
#[doc = "* **Group:** MemoryBarrierMask"]
pub const GL_BUFFER_UPDATE_BARRIER_BIT: GLbitfield = 0x00000200;
#[doc = "`GL_BUFFER_USAGE: GLenum = 0x8765`"]
#[doc = "* **Groups:** VertexBufferObjectParameter, BufferPNameARB"]
pub const GL_BUFFER_USAGE: GLenum = 0x8765;
#[doc = "`GL_BUFFER_VARIABLE: GLenum = 0x92E5`"]
#[doc = "* **Group:** ProgramInterface"]
pub const GL_BUFFER_VARIABLE: GLenum = 0x92E5;
#[doc = "`GL_BYTE: GLenum = 0x1400`"]
#[doc = "* **Groups:** VertexAttribIType, WeightPointerTypeARB, TangentPointerTypeEXT, BinormalPointerTypeEXT, ColorPointerType, ListNameType, NormalPointerType, PixelType, VertexAttribType, VertexAttribPointerType"]
pub const GL_BYTE: GLenum = 0x1400;
#[doc = "`GL_CCW: GLenum = 0x0901`"]
#[doc = "* **Group:** FrontFaceDirection"]
pub const GL_CCW: GLenum = 0x0901;
#[doc = "`GL_CIRCULAR_CCW_ARC_TO_NV: GLenum = 0xF8`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_CIRCULAR_CCW_ARC_TO_NV: GLenum = 0xF8;
#[doc = "`GL_CIRCULAR_CW_ARC_TO_NV: GLenum = 0xFA`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_CIRCULAR_CW_ARC_TO_NV: GLenum = 0xFA;
#[doc = "`GL_CIRCULAR_TANGENT_ARC_TO_NV: GLenum = 0xFC`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_CIRCULAR_TANGENT_ARC_TO_NV: GLenum = 0xFC;
#[doc = "`GL_CLAMP_TO_BORDER: GLenum = 0x812D`"]
#[doc = "* **Group:** TextureWrapMode"]
pub const GL_CLAMP_TO_BORDER: GLenum = 0x812D;
#[doc = "`GL_CLAMP_TO_BORDER_EXT: GLenum = 0x812D`"]
#[cfg(any(feature = "GL_EXT_texture_border_clamp"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_border_clamp"))))]
pub const GL_CLAMP_TO_BORDER_EXT: GLenum = 0x812D;
#[doc = "`GL_CLAMP_TO_BORDER_NV: GLenum = 0x812D`"]
#[doc = "* **Group:** TextureWrapMode"]
#[cfg(any(feature = "GL_NV_texture_border_clamp"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_texture_border_clamp"))))]
pub const GL_CLAMP_TO_BORDER_NV: GLenum = 0x812D;
#[doc = "`GL_CLAMP_TO_BORDER_OES: GLenum = 0x812D`"]
#[cfg(any(feature = "GL_OES_texture_border_clamp"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_border_clamp"))))]
pub const GL_CLAMP_TO_BORDER_OES: GLenum = 0x812D;
#[doc = "`GL_CLAMP_TO_EDGE: GLenum = 0x812F`"]
#[doc = "* **Group:** TextureWrapMode"]
pub const GL_CLAMP_TO_EDGE: GLenum = 0x812F;
#[doc = "`GL_CLIENT_MAPPED_BUFFER_BARRIER_BIT_EXT: GLbitfield = 0x00004000`"]
#[doc = "* **Group:** MemoryBarrierMask"]
#[cfg(any(feature = "GL_EXT_buffer_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_buffer_storage"))))]
pub const GL_CLIENT_MAPPED_BUFFER_BARRIER_BIT_EXT: GLbitfield = 0x00004000;
#[doc = "`GL_CLIENT_STORAGE_BIT_EXT: GLbitfield = 0x0200`"]
#[doc = "* **Group:** BufferStorageMask"]
#[cfg(any(feature = "GL_EXT_buffer_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_buffer_storage"))))]
pub const GL_CLIENT_STORAGE_BIT_EXT: GLbitfield = 0x0200;
#[doc = "`GL_CLIP_DEPTH_MODE_EXT: GLenum = 0x935D`"]
#[doc = "* **Alias Of:** `GL_CLIP_DEPTH_MODE`"]
#[cfg(any(feature = "GL_EXT_clip_control"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_clip_control"))))]
pub const GL_CLIP_DEPTH_MODE_EXT: GLenum = 0x935D;
#[doc = "`GL_CLIP_DISTANCE0_APPLE: GLenum = 0x3000`"]
#[cfg(any(feature = "GL_APPLE_clip_distance"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_clip_distance"))))]
pub const GL_CLIP_DISTANCE0_APPLE: GLenum = 0x3000;
#[doc = "`GL_CLIP_DISTANCE0_EXT: GLenum = 0x3000`"]
#[doc = "* **Alias Of:** `GL_CLIP_PLANE0`"]
#[cfg(any(feature = "GL_EXT_clip_cull_distance"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_clip_cull_distance"))))]
pub const GL_CLIP_DISTANCE0_EXT: GLenum = 0x3000;
#[doc = "`GL_CLIP_DISTANCE1_APPLE: GLenum = 0x3001`"]
#[cfg(any(feature = "GL_APPLE_clip_distance"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_clip_distance"))))]
pub const GL_CLIP_DISTANCE1_APPLE: GLenum = 0x3001;
#[doc = "`GL_CLIP_DISTANCE1_EXT: GLenum = 0x3001`"]
#[doc = "* **Alias Of:** `GL_CLIP_PLANE1`"]
#[cfg(any(feature = "GL_EXT_clip_cull_distance"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_clip_cull_distance"))))]
pub const GL_CLIP_DISTANCE1_EXT: GLenum = 0x3001;
#[doc = "`GL_CLIP_DISTANCE2_APPLE: GLenum = 0x3002`"]
#[cfg(any(feature = "GL_APPLE_clip_distance"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_clip_distance"))))]
pub const GL_CLIP_DISTANCE2_APPLE: GLenum = 0x3002;
#[doc = "`GL_CLIP_DISTANCE2_EXT: GLenum = 0x3002`"]
#[doc = "* **Alias Of:** `GL_CLIP_PLANE2`"]
#[cfg(any(feature = "GL_EXT_clip_cull_distance"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_clip_cull_distance"))))]
pub const GL_CLIP_DISTANCE2_EXT: GLenum = 0x3002;
#[doc = "`GL_CLIP_DISTANCE3_APPLE: GLenum = 0x3003`"]
#[cfg(any(feature = "GL_APPLE_clip_distance"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_clip_distance"))))]
pub const GL_CLIP_DISTANCE3_APPLE: GLenum = 0x3003;
#[doc = "`GL_CLIP_DISTANCE3_EXT: GLenum = 0x3003`"]
#[doc = "* **Alias Of:** `GL_CLIP_PLANE3`"]
#[cfg(any(feature = "GL_EXT_clip_cull_distance"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_clip_cull_distance"))))]
pub const GL_CLIP_DISTANCE3_EXT: GLenum = 0x3003;
#[doc = "`GL_CLIP_DISTANCE4_APPLE: GLenum = 0x3004`"]
#[cfg(any(feature = "GL_APPLE_clip_distance"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_clip_distance"))))]
pub const GL_CLIP_DISTANCE4_APPLE: GLenum = 0x3004;
#[doc = "`GL_CLIP_DISTANCE4_EXT: GLenum = 0x3004`"]
#[doc = "* **Alias Of:** `GL_CLIP_PLANE4`"]
#[cfg(any(feature = "GL_EXT_clip_cull_distance"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_clip_cull_distance"))))]
pub const GL_CLIP_DISTANCE4_EXT: GLenum = 0x3004;
#[doc = "`GL_CLIP_DISTANCE5_APPLE: GLenum = 0x3005`"]
#[cfg(any(feature = "GL_APPLE_clip_distance"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_clip_distance"))))]
pub const GL_CLIP_DISTANCE5_APPLE: GLenum = 0x3005;
#[doc = "`GL_CLIP_DISTANCE5_EXT: GLenum = 0x3005`"]
#[doc = "* **Alias Of:** `GL_CLIP_PLANE5`"]
#[cfg(any(feature = "GL_EXT_clip_cull_distance"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_clip_cull_distance"))))]
pub const GL_CLIP_DISTANCE5_EXT: GLenum = 0x3005;
#[doc = "`GL_CLIP_DISTANCE6_APPLE: GLenum = 0x3006`"]
#[cfg(any(feature = "GL_APPLE_clip_distance"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_clip_distance"))))]
pub const GL_CLIP_DISTANCE6_APPLE: GLenum = 0x3006;
#[doc = "`GL_CLIP_DISTANCE6_EXT: GLenum = 0x3006`"]
#[doc = "* **Alias Of:** `GL_CLIP_DISTANCE6`"]
#[cfg(any(feature = "GL_EXT_clip_cull_distance"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_clip_cull_distance"))))]
pub const GL_CLIP_DISTANCE6_EXT: GLenum = 0x3006;
#[doc = "`GL_CLIP_DISTANCE7_APPLE: GLenum = 0x3007`"]
#[cfg(any(feature = "GL_APPLE_clip_distance"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_clip_distance"))))]
pub const GL_CLIP_DISTANCE7_APPLE: GLenum = 0x3007;
#[doc = "`GL_CLIP_DISTANCE7_EXT: GLenum = 0x3007`"]
#[doc = "* **Alias Of:** `GL_CLIP_DISTANCE7`"]
#[cfg(any(feature = "GL_EXT_clip_cull_distance"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_clip_cull_distance"))))]
pub const GL_CLIP_DISTANCE7_EXT: GLenum = 0x3007;
#[doc = "`GL_CLIP_ORIGIN_EXT: GLenum = 0x935C`"]
#[doc = "* **Alias Of:** `GL_CLIP_ORIGIN`"]
#[cfg(any(feature = "GL_EXT_clip_control"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_clip_control"))))]
pub const GL_CLIP_ORIGIN_EXT: GLenum = 0x935C;
#[doc = "`GL_CLOSE_PATH_NV: GLenum = 0x00`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_CLOSE_PATH_NV: GLenum = 0x00;
#[doc = "`GL_COLOR: GLenum = 0x1800`"]
#[doc = "* **Groups:** Buffer, PixelCopyType, InvalidateFramebufferAttachment"]
pub const GL_COLOR: GLenum = 0x1800;
#[doc = "`GL_COLORBURN: GLenum = 0x929A`"]
pub const GL_COLORBURN: GLenum = 0x929A;
#[doc = "`GL_COLORBURN_KHR: GLenum = 0x929A`"]
#[cfg(any(feature = "GL_KHR_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_blend_equation_advanced"))))]
pub const GL_COLORBURN_KHR: GLenum = 0x929A;
#[doc = "`GL_COLORBURN_NV: GLenum = 0x929A`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_COLORBURN_NV: GLenum = 0x929A;
#[doc = "`GL_COLORDODGE: GLenum = 0x9299`"]
pub const GL_COLORDODGE: GLenum = 0x9299;
#[doc = "`GL_COLORDODGE_KHR: GLenum = 0x9299`"]
#[cfg(any(feature = "GL_KHR_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_blend_equation_advanced"))))]
pub const GL_COLORDODGE_KHR: GLenum = 0x9299;
#[doc = "`GL_COLORDODGE_NV: GLenum = 0x9299`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_COLORDODGE_NV: GLenum = 0x9299;
#[doc = "`GL_COLOR_ATTACHMENT0: GLenum = 0x8CE0`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, ReadBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT0: GLenum = 0x8CE0;
#[doc = "`GL_COLOR_ATTACHMENT0_EXT: GLenum = 0x8CE0`"]
#[doc = "* **Group:** InvalidateFramebufferAttachment"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_COLOR_ATTACHMENT0_EXT: GLenum = 0x8CE0;
#[doc = "`GL_COLOR_ATTACHMENT0_NV: GLenum = 0x8CE0`"]
#[doc = "* **Groups:** InvalidateFramebufferAttachment, DrawBufferModeATI"]
#[cfg(any(
    feature = "GL_NV_draw_buffers",
    feature = "GL_NV_fbo_color_attachments"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_NV_draw_buffers",
        feature = "GL_NV_fbo_color_attachments"
    )))
)]
pub const GL_COLOR_ATTACHMENT0_NV: GLenum = 0x8CE0;
#[doc = "`GL_COLOR_ATTACHMENT1: GLenum = 0x8CE1`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, ReadBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT1: GLenum = 0x8CE1;
#[doc = "`GL_COLOR_ATTACHMENT10: GLenum = 0x8CEA`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, ReadBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT10: GLenum = 0x8CEA;
#[doc = "`GL_COLOR_ATTACHMENT10_EXT: GLenum = 0x8CEA`"]
#[doc = "* **Group:** InvalidateFramebufferAttachment"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_COLOR_ATTACHMENT10_EXT: GLenum = 0x8CEA;
#[doc = "`GL_COLOR_ATTACHMENT10_NV: GLenum = 0x8CEA`"]
#[doc = "* **Groups:** InvalidateFramebufferAttachment, DrawBufferModeATI"]
#[cfg(any(
    feature = "GL_NV_draw_buffers",
    feature = "GL_NV_fbo_color_attachments"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_NV_draw_buffers",
        feature = "GL_NV_fbo_color_attachments"
    )))
)]
pub const GL_COLOR_ATTACHMENT10_NV: GLenum = 0x8CEA;
#[doc = "`GL_COLOR_ATTACHMENT11: GLenum = 0x8CEB`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, ReadBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT11: GLenum = 0x8CEB;
#[doc = "`GL_COLOR_ATTACHMENT11_EXT: GLenum = 0x8CEB`"]
#[doc = "* **Group:** InvalidateFramebufferAttachment"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_COLOR_ATTACHMENT11_EXT: GLenum = 0x8CEB;
#[doc = "`GL_COLOR_ATTACHMENT11_NV: GLenum = 0x8CEB`"]
#[doc = "* **Groups:** InvalidateFramebufferAttachment, DrawBufferModeATI"]
#[cfg(any(
    feature = "GL_NV_draw_buffers",
    feature = "GL_NV_fbo_color_attachments"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_NV_draw_buffers",
        feature = "GL_NV_fbo_color_attachments"
    )))
)]
pub const GL_COLOR_ATTACHMENT11_NV: GLenum = 0x8CEB;
#[doc = "`GL_COLOR_ATTACHMENT12: GLenum = 0x8CEC`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, ReadBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT12: GLenum = 0x8CEC;
#[doc = "`GL_COLOR_ATTACHMENT12_EXT: GLenum = 0x8CEC`"]
#[doc = "* **Group:** InvalidateFramebufferAttachment"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_COLOR_ATTACHMENT12_EXT: GLenum = 0x8CEC;
#[doc = "`GL_COLOR_ATTACHMENT12_NV: GLenum = 0x8CEC`"]
#[doc = "* **Groups:** InvalidateFramebufferAttachment, DrawBufferModeATI"]
#[cfg(any(
    feature = "GL_NV_draw_buffers",
    feature = "GL_NV_fbo_color_attachments"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_NV_draw_buffers",
        feature = "GL_NV_fbo_color_attachments"
    )))
)]
pub const GL_COLOR_ATTACHMENT12_NV: GLenum = 0x8CEC;
#[doc = "`GL_COLOR_ATTACHMENT13: GLenum = 0x8CED`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, ReadBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT13: GLenum = 0x8CED;
#[doc = "`GL_COLOR_ATTACHMENT13_EXT: GLenum = 0x8CED`"]
#[doc = "* **Group:** InvalidateFramebufferAttachment"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_COLOR_ATTACHMENT13_EXT: GLenum = 0x8CED;
#[doc = "`GL_COLOR_ATTACHMENT13_NV: GLenum = 0x8CED`"]
#[doc = "* **Groups:** InvalidateFramebufferAttachment, DrawBufferModeATI"]
#[cfg(any(
    feature = "GL_NV_draw_buffers",
    feature = "GL_NV_fbo_color_attachments"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_NV_draw_buffers",
        feature = "GL_NV_fbo_color_attachments"
    )))
)]
pub const GL_COLOR_ATTACHMENT13_NV: GLenum = 0x8CED;
#[doc = "`GL_COLOR_ATTACHMENT14: GLenum = 0x8CEE`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, ReadBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT14: GLenum = 0x8CEE;
#[doc = "`GL_COLOR_ATTACHMENT14_EXT: GLenum = 0x8CEE`"]
#[doc = "* **Group:** InvalidateFramebufferAttachment"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_COLOR_ATTACHMENT14_EXT: GLenum = 0x8CEE;
#[doc = "`GL_COLOR_ATTACHMENT14_NV: GLenum = 0x8CEE`"]
#[doc = "* **Groups:** InvalidateFramebufferAttachment, DrawBufferModeATI"]
#[cfg(any(
    feature = "GL_NV_draw_buffers",
    feature = "GL_NV_fbo_color_attachments"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_NV_draw_buffers",
        feature = "GL_NV_fbo_color_attachments"
    )))
)]
pub const GL_COLOR_ATTACHMENT14_NV: GLenum = 0x8CEE;
#[doc = "`GL_COLOR_ATTACHMENT15: GLenum = 0x8CEF`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, ReadBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT15: GLenum = 0x8CEF;
#[doc = "`GL_COLOR_ATTACHMENT15_EXT: GLenum = 0x8CEF`"]
#[doc = "* **Group:** InvalidateFramebufferAttachment"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_COLOR_ATTACHMENT15_EXT: GLenum = 0x8CEF;
#[doc = "`GL_COLOR_ATTACHMENT15_NV: GLenum = 0x8CEF`"]
#[doc = "* **Groups:** InvalidateFramebufferAttachment, DrawBufferModeATI"]
#[cfg(any(
    feature = "GL_NV_draw_buffers",
    feature = "GL_NV_fbo_color_attachments"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_NV_draw_buffers",
        feature = "GL_NV_fbo_color_attachments"
    )))
)]
pub const GL_COLOR_ATTACHMENT15_NV: GLenum = 0x8CEF;
#[doc = "`GL_COLOR_ATTACHMENT16: GLenum = 0x8CF0`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT16: GLenum = 0x8CF0;
#[doc = "`GL_COLOR_ATTACHMENT17: GLenum = 0x8CF1`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT17: GLenum = 0x8CF1;
#[doc = "`GL_COLOR_ATTACHMENT18: GLenum = 0x8CF2`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT18: GLenum = 0x8CF2;
#[doc = "`GL_COLOR_ATTACHMENT19: GLenum = 0x8CF3`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT19: GLenum = 0x8CF3;
#[doc = "`GL_COLOR_ATTACHMENT1_EXT: GLenum = 0x8CE1`"]
#[doc = "* **Group:** InvalidateFramebufferAttachment"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_COLOR_ATTACHMENT1_EXT: GLenum = 0x8CE1;
#[doc = "`GL_COLOR_ATTACHMENT1_NV: GLenum = 0x8CE1`"]
#[doc = "* **Groups:** InvalidateFramebufferAttachment, DrawBufferModeATI"]
#[cfg(any(
    feature = "GL_NV_draw_buffers",
    feature = "GL_NV_fbo_color_attachments"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_NV_draw_buffers",
        feature = "GL_NV_fbo_color_attachments"
    )))
)]
pub const GL_COLOR_ATTACHMENT1_NV: GLenum = 0x8CE1;
#[doc = "`GL_COLOR_ATTACHMENT2: GLenum = 0x8CE2`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, ReadBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT2: GLenum = 0x8CE2;
#[doc = "`GL_COLOR_ATTACHMENT20: GLenum = 0x8CF4`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT20: GLenum = 0x8CF4;
#[doc = "`GL_COLOR_ATTACHMENT21: GLenum = 0x8CF5`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT21: GLenum = 0x8CF5;
#[doc = "`GL_COLOR_ATTACHMENT22: GLenum = 0x8CF6`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT22: GLenum = 0x8CF6;
#[doc = "`GL_COLOR_ATTACHMENT23: GLenum = 0x8CF7`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT23: GLenum = 0x8CF7;
#[doc = "`GL_COLOR_ATTACHMENT24: GLenum = 0x8CF8`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT24: GLenum = 0x8CF8;
#[doc = "`GL_COLOR_ATTACHMENT25: GLenum = 0x8CF9`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT25: GLenum = 0x8CF9;
#[doc = "`GL_COLOR_ATTACHMENT26: GLenum = 0x8CFA`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT26: GLenum = 0x8CFA;
#[doc = "`GL_COLOR_ATTACHMENT27: GLenum = 0x8CFB`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT27: GLenum = 0x8CFB;
#[doc = "`GL_COLOR_ATTACHMENT28: GLenum = 0x8CFC`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT28: GLenum = 0x8CFC;
#[doc = "`GL_COLOR_ATTACHMENT29: GLenum = 0x8CFD`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT29: GLenum = 0x8CFD;
#[doc = "`GL_COLOR_ATTACHMENT2_EXT: GLenum = 0x8CE2`"]
#[doc = "* **Group:** InvalidateFramebufferAttachment"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_COLOR_ATTACHMENT2_EXT: GLenum = 0x8CE2;
#[doc = "`GL_COLOR_ATTACHMENT2_NV: GLenum = 0x8CE2`"]
#[doc = "* **Groups:** InvalidateFramebufferAttachment, DrawBufferModeATI"]
#[cfg(any(
    feature = "GL_NV_draw_buffers",
    feature = "GL_NV_fbo_color_attachments"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_NV_draw_buffers",
        feature = "GL_NV_fbo_color_attachments"
    )))
)]
pub const GL_COLOR_ATTACHMENT2_NV: GLenum = 0x8CE2;
#[doc = "`GL_COLOR_ATTACHMENT3: GLenum = 0x8CE3`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, ReadBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT3: GLenum = 0x8CE3;
#[doc = "`GL_COLOR_ATTACHMENT30: GLenum = 0x8CFE`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT30: GLenum = 0x8CFE;
#[doc = "`GL_COLOR_ATTACHMENT31: GLenum = 0x8CFF`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT31: GLenum = 0x8CFF;
#[doc = "`GL_COLOR_ATTACHMENT3_EXT: GLenum = 0x8CE3`"]
#[doc = "* **Group:** InvalidateFramebufferAttachment"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_COLOR_ATTACHMENT3_EXT: GLenum = 0x8CE3;
#[doc = "`GL_COLOR_ATTACHMENT3_NV: GLenum = 0x8CE3`"]
#[doc = "* **Groups:** InvalidateFramebufferAttachment, DrawBufferModeATI"]
#[cfg(any(
    feature = "GL_NV_draw_buffers",
    feature = "GL_NV_fbo_color_attachments"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_NV_draw_buffers",
        feature = "GL_NV_fbo_color_attachments"
    )))
)]
pub const GL_COLOR_ATTACHMENT3_NV: GLenum = 0x8CE3;
#[doc = "`GL_COLOR_ATTACHMENT4: GLenum = 0x8CE4`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, ReadBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT4: GLenum = 0x8CE4;
#[doc = "`GL_COLOR_ATTACHMENT4_EXT: GLenum = 0x8CE4`"]
#[doc = "* **Group:** InvalidateFramebufferAttachment"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_COLOR_ATTACHMENT4_EXT: GLenum = 0x8CE4;
#[doc = "`GL_COLOR_ATTACHMENT4_NV: GLenum = 0x8CE4`"]
#[doc = "* **Groups:** InvalidateFramebufferAttachment, DrawBufferModeATI"]
#[cfg(any(
    feature = "GL_NV_draw_buffers",
    feature = "GL_NV_fbo_color_attachments"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_NV_draw_buffers",
        feature = "GL_NV_fbo_color_attachments"
    )))
)]
pub const GL_COLOR_ATTACHMENT4_NV: GLenum = 0x8CE4;
#[doc = "`GL_COLOR_ATTACHMENT5: GLenum = 0x8CE5`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, ReadBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT5: GLenum = 0x8CE5;
#[doc = "`GL_COLOR_ATTACHMENT5_EXT: GLenum = 0x8CE5`"]
#[doc = "* **Group:** InvalidateFramebufferAttachment"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_COLOR_ATTACHMENT5_EXT: GLenum = 0x8CE5;
#[doc = "`GL_COLOR_ATTACHMENT5_NV: GLenum = 0x8CE5`"]
#[doc = "* **Groups:** InvalidateFramebufferAttachment, DrawBufferModeATI"]
#[cfg(any(
    feature = "GL_NV_draw_buffers",
    feature = "GL_NV_fbo_color_attachments"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_NV_draw_buffers",
        feature = "GL_NV_fbo_color_attachments"
    )))
)]
pub const GL_COLOR_ATTACHMENT5_NV: GLenum = 0x8CE5;
#[doc = "`GL_COLOR_ATTACHMENT6: GLenum = 0x8CE6`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, ReadBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT6: GLenum = 0x8CE6;
#[doc = "`GL_COLOR_ATTACHMENT6_EXT: GLenum = 0x8CE6`"]
#[doc = "* **Group:** InvalidateFramebufferAttachment"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_COLOR_ATTACHMENT6_EXT: GLenum = 0x8CE6;
#[doc = "`GL_COLOR_ATTACHMENT6_NV: GLenum = 0x8CE6`"]
#[doc = "* **Groups:** InvalidateFramebufferAttachment, DrawBufferModeATI"]
#[cfg(any(
    feature = "GL_NV_draw_buffers",
    feature = "GL_NV_fbo_color_attachments"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_NV_draw_buffers",
        feature = "GL_NV_fbo_color_attachments"
    )))
)]
pub const GL_COLOR_ATTACHMENT6_NV: GLenum = 0x8CE6;
#[doc = "`GL_COLOR_ATTACHMENT7: GLenum = 0x8CE7`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, ReadBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT7: GLenum = 0x8CE7;
#[doc = "`GL_COLOR_ATTACHMENT7_EXT: GLenum = 0x8CE7`"]
#[doc = "* **Group:** InvalidateFramebufferAttachment"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_COLOR_ATTACHMENT7_EXT: GLenum = 0x8CE7;
#[doc = "`GL_COLOR_ATTACHMENT7_NV: GLenum = 0x8CE7`"]
#[doc = "* **Groups:** InvalidateFramebufferAttachment, DrawBufferModeATI"]
#[cfg(any(
    feature = "GL_NV_draw_buffers",
    feature = "GL_NV_fbo_color_attachments"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_NV_draw_buffers",
        feature = "GL_NV_fbo_color_attachments"
    )))
)]
pub const GL_COLOR_ATTACHMENT7_NV: GLenum = 0x8CE7;
#[doc = "`GL_COLOR_ATTACHMENT8: GLenum = 0x8CE8`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, ReadBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT8: GLenum = 0x8CE8;
#[doc = "`GL_COLOR_ATTACHMENT8_EXT: GLenum = 0x8CE8`"]
#[doc = "* **Group:** InvalidateFramebufferAttachment"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_COLOR_ATTACHMENT8_EXT: GLenum = 0x8CE8;
#[doc = "`GL_COLOR_ATTACHMENT8_NV: GLenum = 0x8CE8`"]
#[doc = "* **Groups:** InvalidateFramebufferAttachment, DrawBufferModeATI"]
#[cfg(any(
    feature = "GL_NV_draw_buffers",
    feature = "GL_NV_fbo_color_attachments"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_NV_draw_buffers",
        feature = "GL_NV_fbo_color_attachments"
    )))
)]
pub const GL_COLOR_ATTACHMENT8_NV: GLenum = 0x8CE8;
#[doc = "`GL_COLOR_ATTACHMENT9: GLenum = 0x8CE9`"]
#[doc = "* **Groups:** ColorBuffer, DrawBufferMode, ReadBufferMode, FramebufferAttachment, InvalidateFramebufferAttachment"]
pub const GL_COLOR_ATTACHMENT9: GLenum = 0x8CE9;
#[doc = "`GL_COLOR_ATTACHMENT9_EXT: GLenum = 0x8CE9`"]
#[doc = "* **Group:** InvalidateFramebufferAttachment"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_COLOR_ATTACHMENT9_EXT: GLenum = 0x8CE9;
#[doc = "`GL_COLOR_ATTACHMENT9_NV: GLenum = 0x8CE9`"]
#[doc = "* **Groups:** InvalidateFramebufferAttachment, DrawBufferModeATI"]
#[cfg(any(
    feature = "GL_NV_draw_buffers",
    feature = "GL_NV_fbo_color_attachments"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_NV_draw_buffers",
        feature = "GL_NV_fbo_color_attachments"
    )))
)]
pub const GL_COLOR_ATTACHMENT9_NV: GLenum = 0x8CE9;
#[doc = "`GL_COLOR_ATTACHMENT_EXT: GLenum = 0x90F0`"]
#[cfg(any(feature = "GL_EXT_multiview_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_multiview_draw_buffers"))))]
pub const GL_COLOR_ATTACHMENT_EXT: GLenum = 0x90F0;
#[doc = "`GL_COLOR_BUFFER_BIT: GLbitfield = 0x00004000`"]
#[doc = "* **Groups:** ClearBufferMask, AttribMask"]
pub const GL_COLOR_BUFFER_BIT: GLbitfield = 0x00004000;
#[doc = "`GL_COLOR_BUFFER_BIT0_QCOM: GLbitfield = 0x00000001`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_COLOR_BUFFER_BIT0_QCOM: GLbitfield = 0x00000001;
#[doc = "`GL_COLOR_BUFFER_BIT1_QCOM: GLbitfield = 0x00000002`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_COLOR_BUFFER_BIT1_QCOM: GLbitfield = 0x00000002;
#[doc = "`GL_COLOR_BUFFER_BIT2_QCOM: GLbitfield = 0x00000004`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_COLOR_BUFFER_BIT2_QCOM: GLbitfield = 0x00000004;
#[doc = "`GL_COLOR_BUFFER_BIT3_QCOM: GLbitfield = 0x00000008`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_COLOR_BUFFER_BIT3_QCOM: GLbitfield = 0x00000008;
#[doc = "`GL_COLOR_BUFFER_BIT4_QCOM: GLbitfield = 0x00000010`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_COLOR_BUFFER_BIT4_QCOM: GLbitfield = 0x00000010;
#[doc = "`GL_COLOR_BUFFER_BIT5_QCOM: GLbitfield = 0x00000020`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_COLOR_BUFFER_BIT5_QCOM: GLbitfield = 0x00000020;
#[doc = "`GL_COLOR_BUFFER_BIT6_QCOM: GLbitfield = 0x00000040`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_COLOR_BUFFER_BIT6_QCOM: GLbitfield = 0x00000040;
#[doc = "`GL_COLOR_BUFFER_BIT7_QCOM: GLbitfield = 0x00000080`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_COLOR_BUFFER_BIT7_QCOM: GLbitfield = 0x00000080;
#[doc = "`GL_COLOR_CLEAR_VALUE: GLenum = 0x0C22`"]
#[doc = "* **Group:** GetPName"]
pub const GL_COLOR_CLEAR_VALUE: GLenum = 0x0C22;
#[doc = "`GL_COLOR_EXT: GLenum = 0x1800`"]
#[doc = "* **Group:** PixelCopyType"]
#[cfg(any(feature = "GL_EXT_discard_framebuffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_discard_framebuffer"))))]
pub const GL_COLOR_EXT: GLenum = 0x1800;
#[doc = "`GL_COLOR_SAMPLES_NV: GLenum = 0x8E20`"]
#[cfg(any(feature = "GL_NV_framebuffer_mixed_samples"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_framebuffer_mixed_samples"))))]
pub const GL_COLOR_SAMPLES_NV: GLenum = 0x8E20;
#[doc = "`GL_COLOR_WRITEMASK: GLenum = 0x0C23`"]
#[doc = "* **Group:** GetPName"]
pub const GL_COLOR_WRITEMASK: GLenum = 0x0C23;
#[doc = "`GL_COMMAND_BARRIER_BIT: GLbitfield = 0x00000040`"]
#[doc = "* **Group:** MemoryBarrierMask"]
pub const GL_COMMAND_BARRIER_BIT: GLbitfield = 0x00000040;
#[doc = "`GL_COMPARE_REF_TO_TEXTURE: GLenum = 0x884E`"]
#[doc = "* **Group:** TextureCompareMode"]
#[doc = "* **Alias Of:** `GL_COMPARE_R_TO_TEXTURE`"]
pub const GL_COMPARE_REF_TO_TEXTURE: GLenum = 0x884E;
#[doc = "`GL_COMPARE_REF_TO_TEXTURE_EXT: GLenum = 0x884E`"]
#[cfg(any(feature = "GL_EXT_shadow_samplers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_shadow_samplers"))))]
pub const GL_COMPARE_REF_TO_TEXTURE_EXT: GLenum = 0x884E;
#[doc = "`GL_COMPILE_STATUS: GLenum = 0x8B81`"]
#[doc = "* **Group:** ShaderParameterName"]
pub const GL_COMPILE_STATUS: GLenum = 0x8B81;
#[doc = "`GL_COMPLETION_STATUS_KHR: GLenum = 0x91B1`"]
#[cfg(any(feature = "GL_KHR_parallel_shader_compile"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_parallel_shader_compile"))))]
pub const GL_COMPLETION_STATUS_KHR: GLenum = 0x91B1;
#[doc = "`GL_COMPRESSED_R11_EAC: GLenum = 0x9270`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_R11_EAC: GLenum = 0x9270;
#[doc = "`GL_COMPRESSED_RED_GREEN_RGTC2_EXT: GLenum = 0x8DBD`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_compression_rgtc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_compression_rgtc"))))]
pub const GL_COMPRESSED_RED_GREEN_RGTC2_EXT: GLenum = 0x8DBD;
#[doc = "`GL_COMPRESSED_RED_RGTC1_EXT: GLenum = 0x8DBB`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_compression_rgtc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_compression_rgtc"))))]
pub const GL_COMPRESSED_RED_RGTC1_EXT: GLenum = 0x8DBB;
#[doc = "`GL_COMPRESSED_RG11_EAC: GLenum = 0x9272`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_RG11_EAC: GLenum = 0x9272;
#[doc = "`GL_COMPRESSED_RGB8_ETC2: GLenum = 0x9274`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_RGB8_ETC2: GLenum = 0x9274;
#[doc = "`GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9276`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9276;
#[doc = "`GL_COMPRESSED_RGBA8_ETC2_EAC: GLenum = 0x9278`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_RGBA8_ETC2_EAC: GLenum = 0x9278;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_10x10: GLenum = 0x93BB`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_RGBA_ASTC_10x10: GLenum = 0x93BB;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_10x10_KHR: GLenum = 0x93BB`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_RGBA_ASTC_10x10_KHR: GLenum = 0x93BB;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_10x5: GLenum = 0x93B8`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_RGBA_ASTC_10x5: GLenum = 0x93B8;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_10x5_KHR: GLenum = 0x93B8`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_RGBA_ASTC_10x5_KHR: GLenum = 0x93B8;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_10x6: GLenum = 0x93B9`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_RGBA_ASTC_10x6: GLenum = 0x93B9;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_10x6_KHR: GLenum = 0x93B9`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_RGBA_ASTC_10x6_KHR: GLenum = 0x93B9;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_10x8: GLenum = 0x93BA`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_RGBA_ASTC_10x8: GLenum = 0x93BA;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_10x8_KHR: GLenum = 0x93BA`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_RGBA_ASTC_10x8_KHR: GLenum = 0x93BA;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_12x10: GLenum = 0x93BC`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_RGBA_ASTC_12x10: GLenum = 0x93BC;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_12x10_KHR: GLenum = 0x93BC`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_RGBA_ASTC_12x10_KHR: GLenum = 0x93BC;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_12x12: GLenum = 0x93BD`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_RGBA_ASTC_12x12: GLenum = 0x93BD;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_12x12_KHR: GLenum = 0x93BD`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_RGBA_ASTC_12x12_KHR: GLenum = 0x93BD;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_3x3x3_OES: GLenum = 0x93C0`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_texture_compression_astc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_compression_astc"))))]
pub const GL_COMPRESSED_RGBA_ASTC_3x3x3_OES: GLenum = 0x93C0;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_4x3x3_OES: GLenum = 0x93C1`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_texture_compression_astc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_compression_astc"))))]
pub const GL_COMPRESSED_RGBA_ASTC_4x3x3_OES: GLenum = 0x93C1;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_4x4: GLenum = 0x93B0`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_RGBA_ASTC_4x4: GLenum = 0x93B0;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_4x4_KHR: GLenum = 0x93B0`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_RGBA_ASTC_4x4_KHR: GLenum = 0x93B0;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_4x4x3_OES: GLenum = 0x93C2`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_texture_compression_astc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_compression_astc"))))]
pub const GL_COMPRESSED_RGBA_ASTC_4x4x3_OES: GLenum = 0x93C2;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_4x4x4_OES: GLenum = 0x93C3`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_texture_compression_astc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_compression_astc"))))]
pub const GL_COMPRESSED_RGBA_ASTC_4x4x4_OES: GLenum = 0x93C3;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_5x4: GLenum = 0x93B1`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_RGBA_ASTC_5x4: GLenum = 0x93B1;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_5x4_KHR: GLenum = 0x93B1`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_RGBA_ASTC_5x4_KHR: GLenum = 0x93B1;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_5x4x4_OES: GLenum = 0x93C4`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_texture_compression_astc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_compression_astc"))))]
pub const GL_COMPRESSED_RGBA_ASTC_5x4x4_OES: GLenum = 0x93C4;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_5x5: GLenum = 0x93B2`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_RGBA_ASTC_5x5: GLenum = 0x93B2;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_5x5_KHR: GLenum = 0x93B2`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_RGBA_ASTC_5x5_KHR: GLenum = 0x93B2;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_5x5x4_OES: GLenum = 0x93C5`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_texture_compression_astc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_compression_astc"))))]
pub const GL_COMPRESSED_RGBA_ASTC_5x5x4_OES: GLenum = 0x93C5;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_5x5x5_OES: GLenum = 0x93C6`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_texture_compression_astc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_compression_astc"))))]
pub const GL_COMPRESSED_RGBA_ASTC_5x5x5_OES: GLenum = 0x93C6;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_6x5: GLenum = 0x93B3`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_RGBA_ASTC_6x5: GLenum = 0x93B3;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_6x5_KHR: GLenum = 0x93B3`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_RGBA_ASTC_6x5_KHR: GLenum = 0x93B3;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_6x5x5_OES: GLenum = 0x93C7`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_texture_compression_astc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_compression_astc"))))]
pub const GL_COMPRESSED_RGBA_ASTC_6x5x5_OES: GLenum = 0x93C7;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_6x6: GLenum = 0x93B4`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_RGBA_ASTC_6x6: GLenum = 0x93B4;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_6x6_KHR: GLenum = 0x93B4`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_RGBA_ASTC_6x6_KHR: GLenum = 0x93B4;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_6x6x5_OES: GLenum = 0x93C8`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_texture_compression_astc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_compression_astc"))))]
pub const GL_COMPRESSED_RGBA_ASTC_6x6x5_OES: GLenum = 0x93C8;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_6x6x6_OES: GLenum = 0x93C9`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_texture_compression_astc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_compression_astc"))))]
pub const GL_COMPRESSED_RGBA_ASTC_6x6x6_OES: GLenum = 0x93C9;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_8x5: GLenum = 0x93B5`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_RGBA_ASTC_8x5: GLenum = 0x93B5;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_8x5_KHR: GLenum = 0x93B5`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_RGBA_ASTC_8x5_KHR: GLenum = 0x93B5;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_8x6: GLenum = 0x93B6`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_RGBA_ASTC_8x6: GLenum = 0x93B6;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_8x6_KHR: GLenum = 0x93B6`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_RGBA_ASTC_8x6_KHR: GLenum = 0x93B6;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_8x8: GLenum = 0x93B7`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_RGBA_ASTC_8x8: GLenum = 0x93B7;
#[doc = "`GL_COMPRESSED_RGBA_ASTC_8x8_KHR: GLenum = 0x93B7`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_RGBA_ASTC_8x8_KHR: GLenum = 0x93B7;
#[doc = "`GL_COMPRESSED_RGBA_BPTC_UNORM_EXT: GLenum = 0x8E8C`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_compression_bptc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_compression_bptc"))))]
pub const GL_COMPRESSED_RGBA_BPTC_UNORM_EXT: GLenum = 0x8E8C;
#[doc = "`GL_COMPRESSED_RGBA_PVRTC_2BPPV1_IMG: GLenum = 0x8C03`"]
#[cfg(any(feature = "GL_IMG_texture_compression_pvrtc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_IMG_texture_compression_pvrtc"))))]
pub const GL_COMPRESSED_RGBA_PVRTC_2BPPV1_IMG: GLenum = 0x8C03;
#[doc = "`GL_COMPRESSED_RGBA_PVRTC_2BPPV2_IMG: GLenum = 0x9137`"]
#[cfg(any(feature = "GL_IMG_texture_compression_pvrtc2"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_IMG_texture_compression_pvrtc2"))))]
pub const GL_COMPRESSED_RGBA_PVRTC_2BPPV2_IMG: GLenum = 0x9137;
#[doc = "`GL_COMPRESSED_RGBA_PVRTC_4BPPV1_IMG: GLenum = 0x8C02`"]
#[cfg(any(feature = "GL_IMG_texture_compression_pvrtc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_IMG_texture_compression_pvrtc"))))]
pub const GL_COMPRESSED_RGBA_PVRTC_4BPPV1_IMG: GLenum = 0x8C02;
#[doc = "`GL_COMPRESSED_RGBA_PVRTC_4BPPV2_IMG: GLenum = 0x9138`"]
#[cfg(any(feature = "GL_IMG_texture_compression_pvrtc2"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_IMG_texture_compression_pvrtc2"))))]
pub const GL_COMPRESSED_RGBA_PVRTC_4BPPV2_IMG: GLenum = 0x9138;
#[doc = "`GL_COMPRESSED_RGBA_S3TC_DXT1_EXT: GLenum = 0x83F1`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_EXT_texture_compression_dxt1",
    feature = "GL_EXT_texture_compression_s3tc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_texture_compression_dxt1",
        feature = "GL_EXT_texture_compression_s3tc"
    )))
)]
pub const GL_COMPRESSED_RGBA_S3TC_DXT1_EXT: GLenum = 0x83F1;
#[doc = "`GL_COMPRESSED_RGBA_S3TC_DXT3_ANGLE: GLenum = 0x83F2`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_ANGLE_texture_compression_dxt3"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_ANGLE_texture_compression_dxt3"))))]
pub const GL_COMPRESSED_RGBA_S3TC_DXT3_ANGLE: GLenum = 0x83F2;
#[doc = "`GL_COMPRESSED_RGBA_S3TC_DXT3_EXT: GLenum = 0x83F2`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_compression_s3tc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_compression_s3tc"))))]
pub const GL_COMPRESSED_RGBA_S3TC_DXT3_EXT: GLenum = 0x83F2;
#[doc = "`GL_COMPRESSED_RGBA_S3TC_DXT5_ANGLE: GLenum = 0x83F3`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_ANGLE_texture_compression_dxt5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_ANGLE_texture_compression_dxt5"))))]
pub const GL_COMPRESSED_RGBA_S3TC_DXT5_ANGLE: GLenum = 0x83F3;
#[doc = "`GL_COMPRESSED_RGBA_S3TC_DXT5_EXT: GLenum = 0x83F3`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_compression_s3tc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_compression_s3tc"))))]
pub const GL_COMPRESSED_RGBA_S3TC_DXT5_EXT: GLenum = 0x83F3;
#[doc = "`GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT_EXT: GLenum = 0x8E8E`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_compression_bptc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_compression_bptc"))))]
pub const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT_EXT: GLenum = 0x8E8E;
#[doc = "`GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_EXT: GLenum = 0x8E8F`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_compression_bptc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_compression_bptc"))))]
pub const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_EXT: GLenum = 0x8E8F;
#[doc = "`GL_COMPRESSED_RGB_PVRTC_2BPPV1_IMG: GLenum = 0x8C01`"]
#[cfg(any(feature = "GL_IMG_texture_compression_pvrtc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_IMG_texture_compression_pvrtc"))))]
pub const GL_COMPRESSED_RGB_PVRTC_2BPPV1_IMG: GLenum = 0x8C01;
#[doc = "`GL_COMPRESSED_RGB_PVRTC_4BPPV1_IMG: GLenum = 0x8C00`"]
#[cfg(any(feature = "GL_IMG_texture_compression_pvrtc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_IMG_texture_compression_pvrtc"))))]
pub const GL_COMPRESSED_RGB_PVRTC_4BPPV1_IMG: GLenum = 0x8C00;
#[doc = "`GL_COMPRESSED_RGB_S3TC_DXT1_EXT: GLenum = 0x83F0`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_EXT_texture_compression_dxt1",
    feature = "GL_EXT_texture_compression_s3tc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_texture_compression_dxt1",
        feature = "GL_EXT_texture_compression_s3tc"
    )))
)]
pub const GL_COMPRESSED_RGB_S3TC_DXT1_EXT: GLenum = 0x83F0;
#[doc = "`GL_COMPRESSED_SIGNED_R11_EAC: GLenum = 0x9271`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_SIGNED_R11_EAC: GLenum = 0x9271;
#[doc = "`GL_COMPRESSED_SIGNED_RED_GREEN_RGTC2_EXT: GLenum = 0x8DBE`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_compression_rgtc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_compression_rgtc"))))]
pub const GL_COMPRESSED_SIGNED_RED_GREEN_RGTC2_EXT: GLenum = 0x8DBE;
#[doc = "`GL_COMPRESSED_SIGNED_RED_RGTC1_EXT: GLenum = 0x8DBC`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_compression_rgtc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_compression_rgtc"))))]
pub const GL_COMPRESSED_SIGNED_RED_RGTC1_EXT: GLenum = 0x8DBC;
#[doc = "`GL_COMPRESSED_SIGNED_RG11_EAC: GLenum = 0x9273`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_SIGNED_RG11_EAC: GLenum = 0x9273;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x10: GLenum = 0x93DB`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x10: GLenum = 0x93DB;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x10_KHR: GLenum = 0x93DB`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x10_KHR: GLenum = 0x93DB;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x5: GLenum = 0x93D8`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x5: GLenum = 0x93D8;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR: GLenum = 0x93D8`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR: GLenum = 0x93D8;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x6: GLenum = 0x93D9`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x6: GLenum = 0x93D9;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x6_KHR: GLenum = 0x93D9`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x6_KHR: GLenum = 0x93D9;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x8: GLenum = 0x93DA`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x8: GLenum = 0x93DA;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x8_KHR: GLenum = 0x93DA`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x8_KHR: GLenum = 0x93DA;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x10: GLenum = 0x93DC`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x10: GLenum = 0x93DC;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x10_KHR: GLenum = 0x93DC`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x10_KHR: GLenum = 0x93DC;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x12: GLenum = 0x93DD`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x12: GLenum = 0x93DD;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x12_KHR: GLenum = 0x93DD`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x12_KHR: GLenum = 0x93DD;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_3x3x3_OES: GLenum = 0x93E0`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_texture_compression_astc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_compression_astc"))))]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_3x3x3_OES: GLenum = 0x93E0;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x3x3_OES: GLenum = 0x93E1`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_texture_compression_astc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_compression_astc"))))]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x3x3_OES: GLenum = 0x93E1;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4: GLenum = 0x93D0`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4: GLenum = 0x93D0;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR: GLenum = 0x93D0`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR: GLenum = 0x93D0;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4x3_OES: GLenum = 0x93E2`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_texture_compression_astc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_compression_astc"))))]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4x3_OES: GLenum = 0x93E2;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4x4_OES: GLenum = 0x93E3`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_texture_compression_astc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_compression_astc"))))]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4x4_OES: GLenum = 0x93E3;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4: GLenum = 0x93D1`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4: GLenum = 0x93D1;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4_KHR: GLenum = 0x93D1`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4_KHR: GLenum = 0x93D1;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4x4_OES: GLenum = 0x93E4`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_texture_compression_astc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_compression_astc"))))]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4x4_OES: GLenum = 0x93E4;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5: GLenum = 0x93D2`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5: GLenum = 0x93D2;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR: GLenum = 0x93D2`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR: GLenum = 0x93D2;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5x4_OES: GLenum = 0x93E5`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_texture_compression_astc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_compression_astc"))))]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5x4_OES: GLenum = 0x93E5;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5x5_OES: GLenum = 0x93E6`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_texture_compression_astc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_compression_astc"))))]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5x5_OES: GLenum = 0x93E6;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5: GLenum = 0x93D3`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5: GLenum = 0x93D3;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5_KHR: GLenum = 0x93D3`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5_KHR: GLenum = 0x93D3;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5x5_OES: GLenum = 0x93E7`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_texture_compression_astc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_compression_astc"))))]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5x5_OES: GLenum = 0x93E7;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6: GLenum = 0x93D4`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6: GLenum = 0x93D4;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR: GLenum = 0x93D4`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR: GLenum = 0x93D4;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6x5_OES: GLenum = 0x93E8`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_texture_compression_astc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_compression_astc"))))]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6x5_OES: GLenum = 0x93E8;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6x6_OES: GLenum = 0x93E9`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_texture_compression_astc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_compression_astc"))))]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6x6_OES: GLenum = 0x93E9;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x5: GLenum = 0x93D5`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x5: GLenum = 0x93D5;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR: GLenum = 0x93D5`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR: GLenum = 0x93D5;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x6: GLenum = 0x93D6`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x6: GLenum = 0x93D6;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR: GLenum = 0x93D6`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR: GLenum = 0x93D6;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x8: GLenum = 0x93D7`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x8: GLenum = 0x93D7;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x8_KHR: GLenum = 0x93D7`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_KHR_texture_compression_astc_hdr",
    feature = "GL_KHR_texture_compression_astc_ldr",
    feature = "GL_OES_texture_compression_astc"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_KHR_texture_compression_astc_hdr",
        feature = "GL_KHR_texture_compression_astc_ldr",
        feature = "GL_OES_texture_compression_astc"
    )))
)]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x8_KHR: GLenum = 0x93D7;
#[doc = "`GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: GLenum = 0x9279`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: GLenum = 0x9279;
#[doc = "`GL_COMPRESSED_SRGB8_ETC2: GLenum = 0x9275`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_SRGB8_ETC2: GLenum = 0x9275;
#[doc = "`GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9277`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9277;
#[doc = "`GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM_EXT: GLenum = 0x8E8D`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_compression_bptc"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_compression_bptc"))))]
pub const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM_EXT: GLenum = 0x8E8D;
#[doc = "`GL_COMPRESSED_SRGB_ALPHA_PVRTC_2BPPV1_EXT: GLenum = 0x8A56`"]
#[cfg(any(feature = "GL_EXT_pvrtc_sRGB"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_pvrtc_sRGB"))))]
pub const GL_COMPRESSED_SRGB_ALPHA_PVRTC_2BPPV1_EXT: GLenum = 0x8A56;
#[doc = "`GL_COMPRESSED_SRGB_ALPHA_PVRTC_2BPPV2_IMG: GLenum = 0x93F0`"]
#[cfg(any(feature = "GL_EXT_pvrtc_sRGB"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_pvrtc_sRGB"))))]
pub const GL_COMPRESSED_SRGB_ALPHA_PVRTC_2BPPV2_IMG: GLenum = 0x93F0;
#[doc = "`GL_COMPRESSED_SRGB_ALPHA_PVRTC_4BPPV1_EXT: GLenum = 0x8A57`"]
#[cfg(any(feature = "GL_EXT_pvrtc_sRGB"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_pvrtc_sRGB"))))]
pub const GL_COMPRESSED_SRGB_ALPHA_PVRTC_4BPPV1_EXT: GLenum = 0x8A57;
#[doc = "`GL_COMPRESSED_SRGB_ALPHA_PVRTC_4BPPV2_IMG: GLenum = 0x93F1`"]
#[cfg(any(feature = "GL_EXT_pvrtc_sRGB"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_pvrtc_sRGB"))))]
pub const GL_COMPRESSED_SRGB_ALPHA_PVRTC_4BPPV2_IMG: GLenum = 0x93F1;
#[doc = "`GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT: GLenum = 0x8C4D`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_compression_s3tc_srgb"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_texture_compression_s3tc_srgb")))
)]
pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT: GLenum = 0x8C4D;
#[doc = "`GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT1_NV: GLenum = 0x8C4D`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_NV_sRGB_formats"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_sRGB_formats"))))]
pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT1_NV: GLenum = 0x8C4D;
#[doc = "`GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT: GLenum = 0x8C4E`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_compression_s3tc_srgb"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_texture_compression_s3tc_srgb")))
)]
pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT: GLenum = 0x8C4E;
#[doc = "`GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT3_NV: GLenum = 0x8C4E`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_NV_sRGB_formats"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_sRGB_formats"))))]
pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT3_NV: GLenum = 0x8C4E;
#[doc = "`GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT: GLenum = 0x8C4F`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_compression_s3tc_srgb"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_texture_compression_s3tc_srgb")))
)]
pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT: GLenum = 0x8C4F;
#[doc = "`GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT5_NV: GLenum = 0x8C4F`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_NV_sRGB_formats"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_sRGB_formats"))))]
pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT5_NV: GLenum = 0x8C4F;
#[doc = "`GL_COMPRESSED_SRGB_PVRTC_2BPPV1_EXT: GLenum = 0x8A54`"]
#[cfg(any(feature = "GL_EXT_pvrtc_sRGB"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_pvrtc_sRGB"))))]
pub const GL_COMPRESSED_SRGB_PVRTC_2BPPV1_EXT: GLenum = 0x8A54;
#[doc = "`GL_COMPRESSED_SRGB_PVRTC_4BPPV1_EXT: GLenum = 0x8A55`"]
#[cfg(any(feature = "GL_EXT_pvrtc_sRGB"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_pvrtc_sRGB"))))]
pub const GL_COMPRESSED_SRGB_PVRTC_4BPPV1_EXT: GLenum = 0x8A55;
#[doc = "`GL_COMPRESSED_SRGB_S3TC_DXT1_EXT: GLenum = 0x8C4C`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_compression_s3tc_srgb"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_texture_compression_s3tc_srgb")))
)]
pub const GL_COMPRESSED_SRGB_S3TC_DXT1_EXT: GLenum = 0x8C4C;
#[doc = "`GL_COMPRESSED_SRGB_S3TC_DXT1_NV: GLenum = 0x8C4C`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_NV_sRGB_formats"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_sRGB_formats"))))]
pub const GL_COMPRESSED_SRGB_S3TC_DXT1_NV: GLenum = 0x8C4C;
#[doc = "`GL_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A3`"]
#[doc = "* **Group:** GetPName"]
pub const GL_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A3;
#[doc = "`GL_COMPUTE_SHADER: GLenum = 0x91B9`"]
#[doc = "* **Group:** ShaderType"]
pub const GL_COMPUTE_SHADER: GLenum = 0x91B9;
#[doc = "`GL_COMPUTE_SHADER_BIT: GLbitfield = 0x00000020`"]
#[doc = "* **Group:** UseProgramStageMask"]
pub const GL_COMPUTE_SHADER_BIT: GLbitfield = 0x00000020;
#[doc = "`GL_COMPUTE_WORK_GROUP_SIZE: GLenum = 0x8267`"]
#[doc = "* **Group:** ProgramPropertyARB"]
pub const GL_COMPUTE_WORK_GROUP_SIZE: GLenum = 0x8267;
#[doc = "`GL_CONDITION_SATISFIED: GLenum = 0x911C`"]
#[doc = "* **Group:** SyncStatus"]
pub const GL_CONDITION_SATISFIED: GLenum = 0x911C;
#[doc = "`GL_CONDITION_SATISFIED_APPLE: GLenum = 0x911C`"]
#[cfg(any(feature = "GL_APPLE_sync"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_sync"))))]
pub const GL_CONDITION_SATISFIED_APPLE: GLenum = 0x911C;
#[doc = "`GL_CONFORMANT_NV: GLenum = 0x9374`"]
#[cfg(any(feature = "GL_NV_internalformat_sample_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_internalformat_sample_query"))))]
pub const GL_CONFORMANT_NV: GLenum = 0x9374;
#[doc = "`GL_CONIC_CURVE_TO_NV: GLenum = 0x1A`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_CONIC_CURVE_TO_NV: GLenum = 0x1A;
#[doc = "`GL_CONJOINT_NV: GLenum = 0x9284`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_CONJOINT_NV: GLenum = 0x9284;
#[doc = "`GL_CONSERVATIVE_RASTERIZATION_INTEL: GLenum = 0x83FE`"]
#[cfg(any(feature = "GL_INTEL_conservative_rasterization"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_INTEL_conservative_rasterization")))
)]
pub const GL_CONSERVATIVE_RASTERIZATION_INTEL: GLenum = 0x83FE;
#[doc = "`GL_CONSERVATIVE_RASTERIZATION_NV: GLenum = 0x9346`"]
#[cfg(any(feature = "GL_NV_conservative_raster"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_conservative_raster"))))]
pub const GL_CONSERVATIVE_RASTERIZATION_NV: GLenum = 0x9346;
#[doc = "`GL_CONSERVATIVE_RASTER_MODE_NV: GLenum = 0x954D`"]
#[cfg(any(feature = "GL_NV_conservative_raster_pre_snap_triangles"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_NV_conservative_raster_pre_snap_triangles")))
)]
pub const GL_CONSERVATIVE_RASTER_MODE_NV: GLenum = 0x954D;
#[doc = "`GL_CONSERVATIVE_RASTER_MODE_POST_SNAP_NV: GLenum = 0x954E`"]
#[cfg(any(feature = "GL_NV_conservative_raster_pre_snap_triangles"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_NV_conservative_raster_pre_snap_triangles")))
)]
pub const GL_CONSERVATIVE_RASTER_MODE_POST_SNAP_NV: GLenum = 0x954E;
#[doc = "`GL_CONSERVATIVE_RASTER_MODE_PRE_SNAP_NV: GLenum = 0x9550`"]
#[cfg(any(feature = "GL_NV_conservative_raster_pre_snap"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_conservative_raster_pre_snap"))))]
pub const GL_CONSERVATIVE_RASTER_MODE_PRE_SNAP_NV: GLenum = 0x9550;
#[doc = "`GL_CONSERVATIVE_RASTER_MODE_PRE_SNAP_TRIANGLES_NV: GLenum = 0x954F`"]
#[cfg(any(feature = "GL_NV_conservative_raster_pre_snap_triangles"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_NV_conservative_raster_pre_snap_triangles")))
)]
pub const GL_CONSERVATIVE_RASTER_MODE_PRE_SNAP_TRIANGLES_NV: GLenum = 0x954F;
#[doc = "`GL_CONSTANT_ALPHA: GLenum = 0x8003`"]
#[doc = "* **Group:** BlendingFactor"]
pub const GL_CONSTANT_ALPHA: GLenum = 0x8003;
#[doc = "`GL_CONSTANT_COLOR: GLenum = 0x8001`"]
#[doc = "* **Group:** BlendingFactor"]
pub const GL_CONSTANT_COLOR: GLenum = 0x8001;
#[doc = "`GL_CONTEXT_FLAGS: GLenum = 0x821E`"]
#[doc = "* **Group:** GetPName"]
pub const GL_CONTEXT_FLAGS: GLenum = 0x821E;
#[doc = "`GL_CONTEXT_FLAG_DEBUG_BIT: GLbitfield = 0x00000002`"]
#[doc = "* **Group:** ContextFlagMask"]
pub const GL_CONTEXT_FLAG_DEBUG_BIT: GLbitfield = 0x00000002;
#[doc = "`GL_CONTEXT_FLAG_DEBUG_BIT_KHR: GLbitfield = 0x00000002`"]
#[doc = "* **Group:** ContextFlagMask"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_CONTEXT_FLAG_DEBUG_BIT_KHR: GLbitfield = 0x00000002;
#[doc = "`GL_CONTEXT_FLAG_NO_ERROR_BIT_KHR: GLbitfield = 0x00000008`"]
#[doc = "* **Group:** ContextFlagMask"]
#[doc = "* **Alias Of:** `GL_CONTEXT_FLAG_NO_ERROR_BIT`"]
#[cfg(any(feature = "GL_KHR_no_error"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_no_error"))))]
pub const GL_CONTEXT_FLAG_NO_ERROR_BIT_KHR: GLbitfield = 0x00000008;
#[doc = "`GL_CONTEXT_FLAG_PROTECTED_CONTENT_BIT_EXT: GLbitfield = 0x00000010`"]
#[doc = "* **Group:** ContextFlagMask"]
#[cfg(any(feature = "GL_EXT_protected_textures"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_protected_textures"))))]
pub const GL_CONTEXT_FLAG_PROTECTED_CONTENT_BIT_EXT: GLbitfield = 0x00000010;
#[doc = "`GL_CONTEXT_FLAG_ROBUST_ACCESS_BIT: GLbitfield = 0x00000004`"]
#[doc = "* **Group:** ContextFlagMask"]
pub const GL_CONTEXT_FLAG_ROBUST_ACCESS_BIT: GLbitfield = 0x00000004;
#[doc = "`GL_CONTEXT_LOST: GLenum = 0x0507`"]
pub const GL_CONTEXT_LOST: GLenum = 0x0507;
#[doc = "`GL_CONTEXT_LOST_KHR: GLenum = 0x0507`"]
#[cfg(any(feature = "GL_KHR_robustness"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_robustness"))))]
pub const GL_CONTEXT_LOST_KHR: GLenum = 0x0507;
#[doc = "`GL_CONTEXT_RELEASE_BEHAVIOR_FLUSH_KHR: GLenum = 0x82FC`"]
#[cfg(any(feature = "GL_KHR_context_flush_control"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_context_flush_control"))))]
pub const GL_CONTEXT_RELEASE_BEHAVIOR_FLUSH_KHR: GLenum = 0x82FC;
#[doc = "`GL_CONTEXT_RELEASE_BEHAVIOR_KHR: GLenum = 0x82FB`"]
#[cfg(any(feature = "GL_KHR_context_flush_control"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_context_flush_control"))))]
pub const GL_CONTEXT_RELEASE_BEHAVIOR_KHR: GLenum = 0x82FB;
#[doc = "`GL_CONTEXT_ROBUST_ACCESS_EXT: GLenum = 0x90F3`"]
#[cfg(any(feature = "GL_EXT_robustness"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_robustness"))))]
pub const GL_CONTEXT_ROBUST_ACCESS_EXT: GLenum = 0x90F3;
#[doc = "`GL_CONTEXT_ROBUST_ACCESS_KHR: GLenum = 0x90F3`"]
#[cfg(any(feature = "GL_KHR_robustness"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_robustness"))))]
pub const GL_CONTEXT_ROBUST_ACCESS_KHR: GLenum = 0x90F3;
#[doc = "`GL_CONTRAST_NV: GLenum = 0x92A1`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_CONTRAST_NV: GLenum = 0x92A1;
#[doc = "`GL_CONVEX_HULL_NV: GLenum = 0x908B`"]
#[doc = "* **Group:** PathCoverMode"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_CONVEX_HULL_NV: GLenum = 0x908B;
#[doc = "`GL_COPY_READ_BUFFER: GLenum = 0x8F36`"]
#[doc = "* **Groups:** CopyBufferSubDataTarget, BufferTargetARB, BufferStorageTarget"]
pub const GL_COPY_READ_BUFFER: GLenum = 0x8F36;
#[doc = "`GL_COPY_READ_BUFFER_BINDING: GLenum = 0x8F36`"]
#[doc = "* **Alias Of:** `GL_COPY_READ_BUFFER`"]
pub const GL_COPY_READ_BUFFER_BINDING: GLenum = 0x8F36;
#[doc = "`GL_COPY_READ_BUFFER_NV: GLenum = 0x8F36`"]
#[cfg(any(feature = "GL_NV_copy_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_copy_buffer"))))]
pub const GL_COPY_READ_BUFFER_NV: GLenum = 0x8F36;
#[doc = "`GL_COPY_WRITE_BUFFER: GLenum = 0x8F37`"]
#[doc = "* **Groups:** CopyBufferSubDataTarget, BufferTargetARB, BufferStorageTarget"]
pub const GL_COPY_WRITE_BUFFER: GLenum = 0x8F37;
#[doc = "`GL_COPY_WRITE_BUFFER_BINDING: GLenum = 0x8F37`"]
#[doc = "* **Alias Of:** `GL_COPY_WRITE_BUFFER`"]
pub const GL_COPY_WRITE_BUFFER_BINDING: GLenum = 0x8F37;
#[doc = "`GL_COPY_WRITE_BUFFER_NV: GLenum = 0x8F37`"]
#[cfg(any(feature = "GL_NV_copy_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_copy_buffer"))))]
pub const GL_COPY_WRITE_BUFFER_NV: GLenum = 0x8F37;
#[doc = "`GL_COUNTER_RANGE_AMD: GLenum = 0x8BC1`"]
#[cfg(any(feature = "GL_AMD_performance_monitor"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_AMD_performance_monitor"))))]
pub const GL_COUNTER_RANGE_AMD: GLenum = 0x8BC1;
#[doc = "`GL_COUNTER_TYPE_AMD: GLenum = 0x8BC0`"]
#[cfg(any(feature = "GL_AMD_performance_monitor"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_AMD_performance_monitor"))))]
pub const GL_COUNTER_TYPE_AMD: GLenum = 0x8BC0;
#[doc = "`GL_COUNT_DOWN_NV: GLenum = 0x9089`"]
#[doc = "* **Group:** PathFillMode"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_COUNT_DOWN_NV: GLenum = 0x9089;
#[doc = "`GL_COUNT_UP_NV: GLenum = 0x9088`"]
#[doc = "* **Group:** PathFillMode"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_COUNT_UP_NV: GLenum = 0x9088;
#[doc = "`GL_COVERAGE_ALL_FRAGMENTS_NV: GLenum = 0x8ED5`"]
#[cfg(any(feature = "GL_NV_coverage_sample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_coverage_sample"))))]
pub const GL_COVERAGE_ALL_FRAGMENTS_NV: GLenum = 0x8ED5;
#[doc = "`GL_COVERAGE_ATTACHMENT_NV: GLenum = 0x8ED2`"]
#[cfg(any(feature = "GL_NV_coverage_sample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_coverage_sample"))))]
pub const GL_COVERAGE_ATTACHMENT_NV: GLenum = 0x8ED2;
#[doc = "`GL_COVERAGE_AUTOMATIC_NV: GLenum = 0x8ED7`"]
#[cfg(any(feature = "GL_NV_coverage_sample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_coverage_sample"))))]
pub const GL_COVERAGE_AUTOMATIC_NV: GLenum = 0x8ED7;
#[doc = "`GL_COVERAGE_BUFFERS_NV: GLenum = 0x8ED3`"]
#[cfg(any(feature = "GL_NV_coverage_sample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_coverage_sample"))))]
pub const GL_COVERAGE_BUFFERS_NV: GLenum = 0x8ED3;
#[doc = "`GL_COVERAGE_BUFFER_BIT_NV: GLbitfield = 0x00008000`"]
#[doc = "* **Group:** ClearBufferMask"]
#[cfg(any(feature = "GL_NV_coverage_sample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_coverage_sample"))))]
pub const GL_COVERAGE_BUFFER_BIT_NV: GLbitfield = 0x00008000;
#[doc = "`GL_COVERAGE_COMPONENT4_NV: GLenum = 0x8ED1`"]
#[cfg(any(feature = "GL_NV_coverage_sample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_coverage_sample"))))]
pub const GL_COVERAGE_COMPONENT4_NV: GLenum = 0x8ED1;
#[doc = "`GL_COVERAGE_COMPONENT_NV: GLenum = 0x8ED0`"]
#[cfg(any(feature = "GL_NV_coverage_sample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_coverage_sample"))))]
pub const GL_COVERAGE_COMPONENT_NV: GLenum = 0x8ED0;
#[doc = "`GL_COVERAGE_EDGE_FRAGMENTS_NV: GLenum = 0x8ED6`"]
#[cfg(any(feature = "GL_NV_coverage_sample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_coverage_sample"))))]
pub const GL_COVERAGE_EDGE_FRAGMENTS_NV: GLenum = 0x8ED6;
#[doc = "`GL_COVERAGE_MODULATION_NV: GLenum = 0x9332`"]
#[cfg(any(feature = "GL_NV_framebuffer_mixed_samples"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_framebuffer_mixed_samples"))))]
pub const GL_COVERAGE_MODULATION_NV: GLenum = 0x9332;
#[doc = "`GL_COVERAGE_MODULATION_TABLE_NV: GLenum = 0x9331`"]
#[cfg(any(feature = "GL_NV_framebuffer_mixed_samples"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_framebuffer_mixed_samples"))))]
pub const GL_COVERAGE_MODULATION_TABLE_NV: GLenum = 0x9331;
#[doc = "`GL_COVERAGE_MODULATION_TABLE_SIZE_NV: GLenum = 0x9333`"]
#[cfg(any(feature = "GL_NV_framebuffer_mixed_samples"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_framebuffer_mixed_samples"))))]
pub const GL_COVERAGE_MODULATION_TABLE_SIZE_NV: GLenum = 0x9333;
#[doc = "`GL_COVERAGE_SAMPLES_NV: GLenum = 0x8ED4`"]
#[cfg(any(feature = "GL_NV_coverage_sample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_coverage_sample"))))]
pub const GL_COVERAGE_SAMPLES_NV: GLenum = 0x8ED4;
#[doc = "`GL_CPU_OPTIMIZED_QCOM: GLenum = 0x8FB1`"]
#[cfg(any(feature = "GL_QCOM_binning_control"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_binning_control"))))]
pub const GL_CPU_OPTIMIZED_QCOM: GLenum = 0x8FB1;
#[doc = "`GL_CUBIC_CURVE_TO_NV: GLenum = 0x0C`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_CUBIC_CURVE_TO_NV: GLenum = 0x0C;
#[doc = "`GL_CUBIC_IMG: GLenum = 0x9139`"]
#[cfg(any(feature = "GL_IMG_texture_filter_cubic"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_IMG_texture_filter_cubic"))))]
pub const GL_CUBIC_IMG: GLenum = 0x9139;
#[doc = "`GL_CUBIC_MIPMAP_LINEAR_IMG: GLenum = 0x913B`"]
#[cfg(any(feature = "GL_IMG_texture_filter_cubic"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_IMG_texture_filter_cubic"))))]
pub const GL_CUBIC_MIPMAP_LINEAR_IMG: GLenum = 0x913B;
#[doc = "`GL_CUBIC_MIPMAP_NEAREST_IMG: GLenum = 0x913A`"]
#[cfg(any(feature = "GL_IMG_texture_filter_cubic"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_IMG_texture_filter_cubic"))))]
pub const GL_CUBIC_MIPMAP_NEAREST_IMG: GLenum = 0x913A;
#[doc = "`GL_CULL_FACE: GLenum = 0x0B44`"]
#[doc = "* **Groups:** GetPName, EnableCap"]
pub const GL_CULL_FACE: GLenum = 0x0B44;
#[doc = "`GL_CULL_FACE_MODE: GLenum = 0x0B45`"]
#[doc = "* **Group:** GetPName"]
pub const GL_CULL_FACE_MODE: GLenum = 0x0B45;
#[doc = "`GL_CURRENT_PROGRAM: GLenum = 0x8B8D`"]
#[doc = "* **Group:** GetPName"]
pub const GL_CURRENT_PROGRAM: GLenum = 0x8B8D;
#[doc = "`GL_CURRENT_QUERY: GLenum = 0x8865`"]
#[doc = "* **Group:** QueryParameterName"]
pub const GL_CURRENT_QUERY: GLenum = 0x8865;
#[doc = "`GL_CURRENT_QUERY_EXT: GLenum = 0x8865`"]
#[cfg(any(
    feature = "GL_EXT_disjoint_timer_query",
    feature = "GL_EXT_occlusion_query_boolean"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_disjoint_timer_query",
        feature = "GL_EXT_occlusion_query_boolean"
    )))
)]
pub const GL_CURRENT_QUERY_EXT: GLenum = 0x8865;
#[doc = "`GL_CURRENT_VERTEX_ATTRIB: GLenum = 0x8626`"]
#[doc = "* **Groups:** VertexAttribEnum, VertexAttribPropertyARB"]
pub const GL_CURRENT_VERTEX_ATTRIB: GLenum = 0x8626;
#[doc = "`GL_CW: GLenum = 0x0900`"]
#[doc = "* **Group:** FrontFaceDirection"]
pub const GL_CW: GLenum = 0x0900;
#[doc = "`GL_D3D12_FENCE_VALUE_EXT: GLenum = 0x9595`"]
#[doc = "* **Group:** SemaphoreParameterName"]
#[cfg(any(feature = "GL_EXT_semaphore_win32"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_semaphore_win32"))))]
pub const GL_D3D12_FENCE_VALUE_EXT: GLenum = 0x9595;
#[doc = "`GL_DARKEN: GLenum = 0x9297`"]
pub const GL_DARKEN: GLenum = 0x9297;
#[doc = "`GL_DARKEN_KHR: GLenum = 0x9297`"]
#[cfg(any(feature = "GL_KHR_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_blend_equation_advanced"))))]
pub const GL_DARKEN_KHR: GLenum = 0x9297;
#[doc = "`GL_DARKEN_NV: GLenum = 0x9297`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_DARKEN_NV: GLenum = 0x9297;
#[doc = "`GL_DEBUG_CALLBACK_FUNCTION: GLenum = 0x8244`"]
#[doc = "* **Group:** GetPointervPName"]
pub const GL_DEBUG_CALLBACK_FUNCTION: GLenum = 0x8244;
#[doc = "`GL_DEBUG_CALLBACK_FUNCTION_KHR: GLenum = 0x8244`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_CALLBACK_FUNCTION_KHR: GLenum = 0x8244;
#[doc = "`GL_DEBUG_CALLBACK_USER_PARAM: GLenum = 0x8245`"]
#[doc = "* **Group:** GetPointervPName"]
pub const GL_DEBUG_CALLBACK_USER_PARAM: GLenum = 0x8245;
#[doc = "`GL_DEBUG_CALLBACK_USER_PARAM_KHR: GLenum = 0x8245`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_CALLBACK_USER_PARAM_KHR: GLenum = 0x8245;
#[doc = "`GL_DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826D`"]
#[doc = "* **Group:** GetPName"]
pub const GL_DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826D;
#[doc = "`GL_DEBUG_GROUP_STACK_DEPTH_KHR: GLenum = 0x826D`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_GROUP_STACK_DEPTH_KHR: GLenum = 0x826D;
#[doc = "`GL_DEBUG_LOGGED_MESSAGES: GLenum = 0x9145`"]
pub const GL_DEBUG_LOGGED_MESSAGES: GLenum = 0x9145;
#[doc = "`GL_DEBUG_LOGGED_MESSAGES_KHR: GLenum = 0x9145`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_LOGGED_MESSAGES_KHR: GLenum = 0x9145;
#[doc = "`GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: GLenum = 0x8243`"]
pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: GLenum = 0x8243;
#[doc = "`GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH_KHR: GLenum = 0x8243`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH_KHR: GLenum = 0x8243;
#[doc = "`GL_DEBUG_OUTPUT: GLenum = 0x92E0`"]
#[doc = "* **Group:** EnableCap"]
pub const GL_DEBUG_OUTPUT: GLenum = 0x92E0;
#[doc = "`GL_DEBUG_OUTPUT_KHR: GLenum = 0x92E0`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_OUTPUT_KHR: GLenum = 0x92E0;
#[doc = "`GL_DEBUG_OUTPUT_SYNCHRONOUS: GLenum = 0x8242`"]
#[doc = "* **Group:** EnableCap"]
pub const GL_DEBUG_OUTPUT_SYNCHRONOUS: GLenum = 0x8242;
#[doc = "`GL_DEBUG_OUTPUT_SYNCHRONOUS_KHR: GLenum = 0x8242`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_OUTPUT_SYNCHRONOUS_KHR: GLenum = 0x8242;
#[doc = "`GL_DEBUG_SEVERITY_HIGH: GLenum = 0x9146`"]
#[doc = "* **Group:** DebugSeverity"]
pub const GL_DEBUG_SEVERITY_HIGH: GLenum = 0x9146;
#[doc = "`GL_DEBUG_SEVERITY_HIGH_KHR: GLenum = 0x9146`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_SEVERITY_HIGH_KHR: GLenum = 0x9146;
#[doc = "`GL_DEBUG_SEVERITY_LOW: GLenum = 0x9148`"]
#[doc = "* **Group:** DebugSeverity"]
pub const GL_DEBUG_SEVERITY_LOW: GLenum = 0x9148;
#[doc = "`GL_DEBUG_SEVERITY_LOW_KHR: GLenum = 0x9148`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_SEVERITY_LOW_KHR: GLenum = 0x9148;
#[doc = "`GL_DEBUG_SEVERITY_MEDIUM: GLenum = 0x9147`"]
#[doc = "* **Group:** DebugSeverity"]
pub const GL_DEBUG_SEVERITY_MEDIUM: GLenum = 0x9147;
#[doc = "`GL_DEBUG_SEVERITY_MEDIUM_KHR: GLenum = 0x9147`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_SEVERITY_MEDIUM_KHR: GLenum = 0x9147;
#[doc = "`GL_DEBUG_SEVERITY_NOTIFICATION: GLenum = 0x826B`"]
#[doc = "* **Group:** DebugSeverity"]
pub const GL_DEBUG_SEVERITY_NOTIFICATION: GLenum = 0x826B;
#[doc = "`GL_DEBUG_SEVERITY_NOTIFICATION_KHR: GLenum = 0x826B`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_SEVERITY_NOTIFICATION_KHR: GLenum = 0x826B;
#[doc = "`GL_DEBUG_SOURCE_API: GLenum = 0x8246`"]
#[doc = "* **Group:** DebugSource"]
pub const GL_DEBUG_SOURCE_API: GLenum = 0x8246;
#[doc = "`GL_DEBUG_SOURCE_API_KHR: GLenum = 0x8246`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_SOURCE_API_KHR: GLenum = 0x8246;
#[doc = "`GL_DEBUG_SOURCE_APPLICATION: GLenum = 0x824A`"]
#[doc = "* **Group:** DebugSource"]
pub const GL_DEBUG_SOURCE_APPLICATION: GLenum = 0x824A;
#[doc = "`GL_DEBUG_SOURCE_APPLICATION_KHR: GLenum = 0x824A`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_SOURCE_APPLICATION_KHR: GLenum = 0x824A;
#[doc = "`GL_DEBUG_SOURCE_OTHER: GLenum = 0x824B`"]
#[doc = "* **Group:** DebugSource"]
pub const GL_DEBUG_SOURCE_OTHER: GLenum = 0x824B;
#[doc = "`GL_DEBUG_SOURCE_OTHER_KHR: GLenum = 0x824B`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_SOURCE_OTHER_KHR: GLenum = 0x824B;
#[doc = "`GL_DEBUG_SOURCE_SHADER_COMPILER: GLenum = 0x8248`"]
#[doc = "* **Group:** DebugSource"]
pub const GL_DEBUG_SOURCE_SHADER_COMPILER: GLenum = 0x8248;
#[doc = "`GL_DEBUG_SOURCE_SHADER_COMPILER_KHR: GLenum = 0x8248`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_SOURCE_SHADER_COMPILER_KHR: GLenum = 0x8248;
#[doc = "`GL_DEBUG_SOURCE_THIRD_PARTY: GLenum = 0x8249`"]
#[doc = "* **Group:** DebugSource"]
pub const GL_DEBUG_SOURCE_THIRD_PARTY: GLenum = 0x8249;
#[doc = "`GL_DEBUG_SOURCE_THIRD_PARTY_KHR: GLenum = 0x8249`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_SOURCE_THIRD_PARTY_KHR: GLenum = 0x8249;
#[doc = "`GL_DEBUG_SOURCE_WINDOW_SYSTEM: GLenum = 0x8247`"]
#[doc = "* **Group:** DebugSource"]
pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM: GLenum = 0x8247;
#[doc = "`GL_DEBUG_SOURCE_WINDOW_SYSTEM_KHR: GLenum = 0x8247`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM_KHR: GLenum = 0x8247;
#[doc = "`GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR: GLenum = 0x824D`"]
#[doc = "* **Group:** DebugType"]
pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR: GLenum = 0x824D;
#[doc = "`GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR_KHR: GLenum = 0x824D`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR_KHR: GLenum = 0x824D;
#[doc = "`GL_DEBUG_TYPE_ERROR: GLenum = 0x824C`"]
#[doc = "* **Group:** DebugType"]
pub const GL_DEBUG_TYPE_ERROR: GLenum = 0x824C;
#[doc = "`GL_DEBUG_TYPE_ERROR_KHR: GLenum = 0x824C`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_TYPE_ERROR_KHR: GLenum = 0x824C;
#[doc = "`GL_DEBUG_TYPE_MARKER: GLenum = 0x8268`"]
#[doc = "* **Group:** DebugType"]
pub const GL_DEBUG_TYPE_MARKER: GLenum = 0x8268;
#[doc = "`GL_DEBUG_TYPE_MARKER_KHR: GLenum = 0x8268`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_TYPE_MARKER_KHR: GLenum = 0x8268;
#[doc = "`GL_DEBUG_TYPE_OTHER: GLenum = 0x8251`"]
#[doc = "* **Group:** DebugType"]
pub const GL_DEBUG_TYPE_OTHER: GLenum = 0x8251;
#[doc = "`GL_DEBUG_TYPE_OTHER_KHR: GLenum = 0x8251`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_TYPE_OTHER_KHR: GLenum = 0x8251;
#[doc = "`GL_DEBUG_TYPE_PERFORMANCE: GLenum = 0x8250`"]
#[doc = "* **Group:** DebugType"]
pub const GL_DEBUG_TYPE_PERFORMANCE: GLenum = 0x8250;
#[doc = "`GL_DEBUG_TYPE_PERFORMANCE_KHR: GLenum = 0x8250`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_TYPE_PERFORMANCE_KHR: GLenum = 0x8250;
#[doc = "`GL_DEBUG_TYPE_POP_GROUP: GLenum = 0x826A`"]
#[doc = "* **Group:** DebugType"]
pub const GL_DEBUG_TYPE_POP_GROUP: GLenum = 0x826A;
#[doc = "`GL_DEBUG_TYPE_POP_GROUP_KHR: GLenum = 0x826A`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_TYPE_POP_GROUP_KHR: GLenum = 0x826A;
#[doc = "`GL_DEBUG_TYPE_PORTABILITY: GLenum = 0x824F`"]
#[doc = "* **Group:** DebugType"]
pub const GL_DEBUG_TYPE_PORTABILITY: GLenum = 0x824F;
#[doc = "`GL_DEBUG_TYPE_PORTABILITY_KHR: GLenum = 0x824F`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_TYPE_PORTABILITY_KHR: GLenum = 0x824F;
#[doc = "`GL_DEBUG_TYPE_PUSH_GROUP: GLenum = 0x8269`"]
#[doc = "* **Group:** DebugType"]
pub const GL_DEBUG_TYPE_PUSH_GROUP: GLenum = 0x8269;
#[doc = "`GL_DEBUG_TYPE_PUSH_GROUP_KHR: GLenum = 0x8269`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_TYPE_PUSH_GROUP_KHR: GLenum = 0x8269;
#[doc = "`GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR: GLenum = 0x824E`"]
#[doc = "* **Group:** DebugType"]
pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR: GLenum = 0x824E;
#[doc = "`GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR_KHR: GLenum = 0x824E`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR_KHR: GLenum = 0x824E;
#[doc = "`GL_DECODE_EXT: GLenum = 0x8A49`"]
#[cfg(any(feature = "GL_EXT_texture_sRGB_decode"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_sRGB_decode"))))]
pub const GL_DECODE_EXT: GLenum = 0x8A49;
#[doc = "`GL_DECR: GLenum = 0x1E03`"]
#[doc = "* **Group:** StencilOp"]
pub const GL_DECR: GLenum = 0x1E03;
#[doc = "`GL_DECR_WRAP: GLenum = 0x8508`"]
#[doc = "* **Group:** StencilOp"]
pub const GL_DECR_WRAP: GLenum = 0x8508;
#[doc = "`GL_DEDICATED_MEMORY_OBJECT_EXT: GLenum = 0x9581`"]
#[doc = "* **Group:** MemoryObjectParameterName"]
#[cfg(any(feature = "GL_EXT_memory_object"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_memory_object"))))]
pub const GL_DEDICATED_MEMORY_OBJECT_EXT: GLenum = 0x9581;
#[doc = "`GL_DELETE_STATUS: GLenum = 0x8B80`"]
#[doc = "* **Groups:** ProgramPropertyARB, ShaderParameterName"]
pub const GL_DELETE_STATUS: GLenum = 0x8B80;
#[doc = "`GL_DEPTH: GLenum = 0x1801`"]
#[doc = "* **Groups:** Buffer, PixelCopyType, InvalidateFramebufferAttachment"]
pub const GL_DEPTH: GLenum = 0x1801;
#[doc = "`GL_DEPTH24_STENCIL8: GLenum = 0x88F0`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_DEPTH24_STENCIL8: GLenum = 0x88F0;
#[doc = "`GL_DEPTH24_STENCIL8_OES: GLenum = 0x88F0`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_ANGLE_depth_texture",
    feature = "GL_OES_packed_depth_stencil",
    feature = "GL_OES_required_internalformat"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_ANGLE_depth_texture",
        feature = "GL_OES_packed_depth_stencil",
        feature = "GL_OES_required_internalformat"
    )))
)]
pub const GL_DEPTH24_STENCIL8_OES: GLenum = 0x88F0;
#[doc = "`GL_DEPTH32F_STENCIL8: GLenum = 0x8CAD`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_DEPTH32F_STENCIL8: GLenum = 0x8CAD;
#[doc = "`GL_DEPTH_ATTACHMENT: GLenum = 0x8D00`"]
#[doc = "* **Groups:** InvalidateFramebufferAttachment, FramebufferAttachment"]
pub const GL_DEPTH_ATTACHMENT: GLenum = 0x8D00;
#[doc = "`GL_DEPTH_BITS: GLenum = 0x0D56`"]
#[doc = "* **Group:** GetPName"]
pub const GL_DEPTH_BITS: GLenum = 0x0D56;
#[doc = "`GL_DEPTH_BUFFER_BIT: GLbitfield = 0x00000100`"]
#[doc = "* **Groups:** ClearBufferMask, AttribMask"]
pub const GL_DEPTH_BUFFER_BIT: GLbitfield = 0x00000100;
#[doc = "`GL_DEPTH_BUFFER_BIT0_QCOM: GLbitfield = 0x00000100`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_DEPTH_BUFFER_BIT0_QCOM: GLbitfield = 0x00000100;
#[doc = "`GL_DEPTH_BUFFER_BIT1_QCOM: GLbitfield = 0x00000200`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_DEPTH_BUFFER_BIT1_QCOM: GLbitfield = 0x00000200;
#[doc = "`GL_DEPTH_BUFFER_BIT2_QCOM: GLbitfield = 0x00000400`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_DEPTH_BUFFER_BIT2_QCOM: GLbitfield = 0x00000400;
#[doc = "`GL_DEPTH_BUFFER_BIT3_QCOM: GLbitfield = 0x00000800`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_DEPTH_BUFFER_BIT3_QCOM: GLbitfield = 0x00000800;
#[doc = "`GL_DEPTH_BUFFER_BIT4_QCOM: GLbitfield = 0x00001000`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_DEPTH_BUFFER_BIT4_QCOM: GLbitfield = 0x00001000;
#[doc = "`GL_DEPTH_BUFFER_BIT5_QCOM: GLbitfield = 0x00002000`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_DEPTH_BUFFER_BIT5_QCOM: GLbitfield = 0x00002000;
#[doc = "`GL_DEPTH_BUFFER_BIT6_QCOM: GLbitfield = 0x00004000`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_DEPTH_BUFFER_BIT6_QCOM: GLbitfield = 0x00004000;
#[doc = "`GL_DEPTH_BUFFER_BIT7_QCOM: GLbitfield = 0x00008000`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_DEPTH_BUFFER_BIT7_QCOM: GLbitfield = 0x00008000;
#[doc = "`GL_DEPTH_CLAMP_EXT: GLenum = 0x864F`"]
#[cfg(any(feature = "GL_EXT_depth_clamp"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_depth_clamp"))))]
pub const GL_DEPTH_CLAMP_EXT: GLenum = 0x864F;
#[doc = "`GL_DEPTH_CLEAR_VALUE: GLenum = 0x0B73`"]
#[doc = "* **Group:** GetPName"]
pub const GL_DEPTH_CLEAR_VALUE: GLenum = 0x0B73;
#[doc = "`GL_DEPTH_COMPONENT: GLenum = 0x1902`"]
#[doc = "* **Groups:** InternalFormat, PixelFormat"]
pub const GL_DEPTH_COMPONENT: GLenum = 0x1902;
#[doc = "`GL_DEPTH_COMPONENT16: GLenum = 0x81A5`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_DEPTH_COMPONENT16: GLenum = 0x81A5;
#[doc = "`GL_DEPTH_COMPONENT16_NONLINEAR_NV: GLenum = 0x8E2C`"]
#[cfg(any(feature = "GL_NV_depth_nonlinear"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_depth_nonlinear"))))]
pub const GL_DEPTH_COMPONENT16_NONLINEAR_NV: GLenum = 0x8E2C;
#[doc = "`GL_DEPTH_COMPONENT16_OES: GLenum = 0x81A5`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_required_internalformat"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_required_internalformat"))))]
pub const GL_DEPTH_COMPONENT16_OES: GLenum = 0x81A5;
#[doc = "`GL_DEPTH_COMPONENT24: GLenum = 0x81A6`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_DEPTH_COMPONENT24: GLenum = 0x81A6;
#[doc = "`GL_DEPTH_COMPONENT24_OES: GLenum = 0x81A6`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_depth24", feature = "GL_OES_required_internalformat"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_OES_depth24", feature = "GL_OES_required_internalformat")))
)]
pub const GL_DEPTH_COMPONENT24_OES: GLenum = 0x81A6;
#[doc = "`GL_DEPTH_COMPONENT32F: GLenum = 0x8CAC`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_DEPTH_COMPONENT32F: GLenum = 0x8CAC;
#[doc = "`GL_DEPTH_COMPONENT32_OES: GLenum = 0x81A7`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_ANGLE_depth_texture",
    feature = "GL_OES_depth32",
    feature = "GL_OES_required_internalformat"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_ANGLE_depth_texture",
        feature = "GL_OES_depth32",
        feature = "GL_OES_required_internalformat"
    )))
)]
pub const GL_DEPTH_COMPONENT32_OES: GLenum = 0x81A7;
#[doc = "`GL_DEPTH_EXT: GLenum = 0x1801`"]
#[doc = "* **Group:** PixelCopyType"]
#[cfg(any(feature = "GL_EXT_discard_framebuffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_discard_framebuffer"))))]
pub const GL_DEPTH_EXT: GLenum = 0x1801;
#[doc = "`GL_DEPTH_FUNC: GLenum = 0x0B74`"]
#[doc = "* **Group:** GetPName"]
pub const GL_DEPTH_FUNC: GLenum = 0x0B74;
#[doc = "`GL_DEPTH_RANGE: GLenum = 0x0B70`"]
#[doc = "* **Group:** GetPName"]
pub const GL_DEPTH_RANGE: GLenum = 0x0B70;
#[doc = "`GL_DEPTH_SAMPLES_NV: GLenum = 0x932D`"]
#[cfg(any(feature = "GL_NV_framebuffer_mixed_samples"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_framebuffer_mixed_samples"))))]
pub const GL_DEPTH_SAMPLES_NV: GLenum = 0x932D;
#[doc = "`GL_DEPTH_STENCIL: GLenum = 0x84F9`"]
#[doc = "* **Groups:** InternalFormat, PixelFormat"]
pub const GL_DEPTH_STENCIL: GLenum = 0x84F9;
#[doc = "`GL_DEPTH_STENCIL_ATTACHMENT: GLenum = 0x821A`"]
#[doc = "* **Group:** InvalidateFramebufferAttachment"]
pub const GL_DEPTH_STENCIL_ATTACHMENT: GLenum = 0x821A;
#[doc = "`GL_DEPTH_STENCIL_OES: GLenum = 0x84F9`"]
#[doc = "* **Group:** InternalFormat"]
#[cfg(any(
    feature = "GL_ANGLE_depth_texture",
    feature = "GL_OES_packed_depth_stencil"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_ANGLE_depth_texture",
        feature = "GL_OES_packed_depth_stencil"
    )))
)]
pub const GL_DEPTH_STENCIL_OES: GLenum = 0x84F9;
#[doc = "`GL_DEPTH_STENCIL_TEXTURE_MODE: GLenum = 0x90EA`"]
#[doc = "* **Group:** TextureParameterName"]
pub const GL_DEPTH_STENCIL_TEXTURE_MODE: GLenum = 0x90EA;
#[doc = "`GL_DEPTH_TEST: GLenum = 0x0B71`"]
#[doc = "* **Groups:** GetPName, EnableCap"]
pub const GL_DEPTH_TEST: GLenum = 0x0B71;
#[doc = "`GL_DEPTH_WRITEMASK: GLenum = 0x0B72`"]
#[doc = "* **Group:** GetPName"]
pub const GL_DEPTH_WRITEMASK: GLenum = 0x0B72;
#[doc = "`GL_DETACHED_BUFFERS_NV: GLenum = 0x95AB`"]
#[cfg(any(feature = "GL_NV_memory_attachment"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_memory_attachment"))))]
pub const GL_DETACHED_BUFFERS_NV: GLenum = 0x95AB;
#[doc = "`GL_DETACHED_MEMORY_INCARNATION_NV: GLenum = 0x95A9`"]
#[cfg(any(feature = "GL_NV_memory_attachment"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_memory_attachment"))))]
pub const GL_DETACHED_MEMORY_INCARNATION_NV: GLenum = 0x95A9;
#[doc = "`GL_DETACHED_TEXTURES_NV: GLenum = 0x95AA`"]
#[cfg(any(feature = "GL_NV_memory_attachment"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_memory_attachment"))))]
pub const GL_DETACHED_TEXTURES_NV: GLenum = 0x95AA;
#[doc = "`GL_DEVICE_LUID_EXT: GLenum = 0x9599`"]
#[doc = "* **Group:** GetPName"]
#[cfg(any(
    feature = "GL_EXT_memory_object_win32",
    feature = "GL_EXT_semaphore_win32"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_memory_object_win32",
        feature = "GL_EXT_semaphore_win32"
    )))
)]
pub const GL_DEVICE_LUID_EXT: GLenum = 0x9599;
#[doc = "`GL_DEVICE_NODE_MASK_EXT: GLenum = 0x959A`"]
#[doc = "* **Group:** GetPName"]
#[cfg(any(
    feature = "GL_EXT_memory_object_win32",
    feature = "GL_EXT_semaphore_win32"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_memory_object_win32",
        feature = "GL_EXT_semaphore_win32"
    )))
)]
pub const GL_DEVICE_NODE_MASK_EXT: GLenum = 0x959A;
#[doc = "`GL_DEVICE_UUID_EXT: GLenum = 0x9597`"]
#[doc = "* **Group:** GetPName"]
#[cfg(any(feature = "GL_EXT_memory_object", feature = "GL_EXT_semaphore"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_memory_object", feature = "GL_EXT_semaphore")))
)]
pub const GL_DEVICE_UUID_EXT: GLenum = 0x9597;
#[doc = "`GL_DIFFERENCE: GLenum = 0x929E`"]
pub const GL_DIFFERENCE: GLenum = 0x929E;
#[doc = "`GL_DIFFERENCE_KHR: GLenum = 0x929E`"]
#[cfg(any(feature = "GL_KHR_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_blend_equation_advanced"))))]
pub const GL_DIFFERENCE_KHR: GLenum = 0x929E;
#[doc = "`GL_DIFFERENCE_NV: GLenum = 0x929E`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_DIFFERENCE_NV: GLenum = 0x929E;
#[doc = "`GL_DISJOINT_NV: GLenum = 0x9283`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_DISJOINT_NV: GLenum = 0x9283;
#[doc = "`GL_DISPATCH_INDIRECT_BUFFER: GLenum = 0x90EE`"]
#[doc = "* **Groups:** CopyBufferSubDataTarget, BufferTargetARB, BufferStorageTarget"]
pub const GL_DISPATCH_INDIRECT_BUFFER: GLenum = 0x90EE;
#[doc = "`GL_DISPATCH_INDIRECT_BUFFER_BINDING: GLenum = 0x90EF`"]
#[doc = "* **Group:** GetPName"]
pub const GL_DISPATCH_INDIRECT_BUFFER_BINDING: GLenum = 0x90EF;
#[doc = "`GL_DITHER: GLenum = 0x0BD0`"]
#[doc = "* **Groups:** GetPName, EnableCap"]
pub const GL_DITHER: GLenum = 0x0BD0;
#[doc = "`GL_DMP_PROGRAM_BINARY_DMP: GLenum = 0x9253`"]
#[cfg(any(feature = "GL_DMP_program_binary"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_DMP_program_binary"))))]
pub const GL_DMP_PROGRAM_BINARY_DMP: GLenum = 0x9253;
#[doc = "`GL_DONT_CARE: GLenum = 0x1100`"]
#[doc = "* **Groups:** DebugSeverity, HintMode, DebugSource, DebugType"]
pub const GL_DONT_CARE: GLenum = 0x1100;
#[doc = "`GL_DOWNSAMPLE_SCALES_IMG: GLenum = 0x913E`"]
#[cfg(any(feature = "GL_IMG_framebuffer_downsample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_IMG_framebuffer_downsample"))))]
pub const GL_DOWNSAMPLE_SCALES_IMG: GLenum = 0x913E;
#[doc = "`GL_DRAW_BUFFER0: GLenum = 0x8825`"]
pub const GL_DRAW_BUFFER0: GLenum = 0x8825;
#[doc = "`GL_DRAW_BUFFER0_EXT: GLenum = 0x8825`"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_DRAW_BUFFER0_EXT: GLenum = 0x8825;
#[doc = "`GL_DRAW_BUFFER0_NV: GLenum = 0x8825`"]
#[cfg(any(feature = "GL_NV_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_draw_buffers"))))]
pub const GL_DRAW_BUFFER0_NV: GLenum = 0x8825;
#[doc = "`GL_DRAW_BUFFER1: GLenum = 0x8826`"]
pub const GL_DRAW_BUFFER1: GLenum = 0x8826;
#[doc = "`GL_DRAW_BUFFER10: GLenum = 0x882F`"]
pub const GL_DRAW_BUFFER10: GLenum = 0x882F;
#[doc = "`GL_DRAW_BUFFER10_EXT: GLenum = 0x882F`"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_DRAW_BUFFER10_EXT: GLenum = 0x882F;
#[doc = "`GL_DRAW_BUFFER10_NV: GLenum = 0x882F`"]
#[cfg(any(feature = "GL_NV_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_draw_buffers"))))]
pub const GL_DRAW_BUFFER10_NV: GLenum = 0x882F;
#[doc = "`GL_DRAW_BUFFER11: GLenum = 0x8830`"]
pub const GL_DRAW_BUFFER11: GLenum = 0x8830;
#[doc = "`GL_DRAW_BUFFER11_EXT: GLenum = 0x8830`"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_DRAW_BUFFER11_EXT: GLenum = 0x8830;
#[doc = "`GL_DRAW_BUFFER11_NV: GLenum = 0x8830`"]
#[cfg(any(feature = "GL_NV_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_draw_buffers"))))]
pub const GL_DRAW_BUFFER11_NV: GLenum = 0x8830;
#[doc = "`GL_DRAW_BUFFER12: GLenum = 0x8831`"]
pub const GL_DRAW_BUFFER12: GLenum = 0x8831;
#[doc = "`GL_DRAW_BUFFER12_EXT: GLenum = 0x8831`"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_DRAW_BUFFER12_EXT: GLenum = 0x8831;
#[doc = "`GL_DRAW_BUFFER12_NV: GLenum = 0x8831`"]
#[cfg(any(feature = "GL_NV_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_draw_buffers"))))]
pub const GL_DRAW_BUFFER12_NV: GLenum = 0x8831;
#[doc = "`GL_DRAW_BUFFER13: GLenum = 0x8832`"]
pub const GL_DRAW_BUFFER13: GLenum = 0x8832;
#[doc = "`GL_DRAW_BUFFER13_EXT: GLenum = 0x8832`"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_DRAW_BUFFER13_EXT: GLenum = 0x8832;
#[doc = "`GL_DRAW_BUFFER13_NV: GLenum = 0x8832`"]
#[cfg(any(feature = "GL_NV_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_draw_buffers"))))]
pub const GL_DRAW_BUFFER13_NV: GLenum = 0x8832;
#[doc = "`GL_DRAW_BUFFER14: GLenum = 0x8833`"]
pub const GL_DRAW_BUFFER14: GLenum = 0x8833;
#[doc = "`GL_DRAW_BUFFER14_EXT: GLenum = 0x8833`"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_DRAW_BUFFER14_EXT: GLenum = 0x8833;
#[doc = "`GL_DRAW_BUFFER14_NV: GLenum = 0x8833`"]
#[cfg(any(feature = "GL_NV_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_draw_buffers"))))]
pub const GL_DRAW_BUFFER14_NV: GLenum = 0x8833;
#[doc = "`GL_DRAW_BUFFER15: GLenum = 0x8834`"]
pub const GL_DRAW_BUFFER15: GLenum = 0x8834;
#[doc = "`GL_DRAW_BUFFER15_EXT: GLenum = 0x8834`"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_DRAW_BUFFER15_EXT: GLenum = 0x8834;
#[doc = "`GL_DRAW_BUFFER15_NV: GLenum = 0x8834`"]
#[cfg(any(feature = "GL_NV_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_draw_buffers"))))]
pub const GL_DRAW_BUFFER15_NV: GLenum = 0x8834;
#[doc = "`GL_DRAW_BUFFER1_EXT: GLenum = 0x8826`"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_DRAW_BUFFER1_EXT: GLenum = 0x8826;
#[doc = "`GL_DRAW_BUFFER1_NV: GLenum = 0x8826`"]
#[cfg(any(feature = "GL_NV_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_draw_buffers"))))]
pub const GL_DRAW_BUFFER1_NV: GLenum = 0x8826;
#[doc = "`GL_DRAW_BUFFER2: GLenum = 0x8827`"]
pub const GL_DRAW_BUFFER2: GLenum = 0x8827;
#[doc = "`GL_DRAW_BUFFER2_EXT: GLenum = 0x8827`"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_DRAW_BUFFER2_EXT: GLenum = 0x8827;
#[doc = "`GL_DRAW_BUFFER2_NV: GLenum = 0x8827`"]
#[cfg(any(feature = "GL_NV_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_draw_buffers"))))]
pub const GL_DRAW_BUFFER2_NV: GLenum = 0x8827;
#[doc = "`GL_DRAW_BUFFER3: GLenum = 0x8828`"]
pub const GL_DRAW_BUFFER3: GLenum = 0x8828;
#[doc = "`GL_DRAW_BUFFER3_EXT: GLenum = 0x8828`"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_DRAW_BUFFER3_EXT: GLenum = 0x8828;
#[doc = "`GL_DRAW_BUFFER3_NV: GLenum = 0x8828`"]
#[cfg(any(feature = "GL_NV_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_draw_buffers"))))]
pub const GL_DRAW_BUFFER3_NV: GLenum = 0x8828;
#[doc = "`GL_DRAW_BUFFER4: GLenum = 0x8829`"]
pub const GL_DRAW_BUFFER4: GLenum = 0x8829;
#[doc = "`GL_DRAW_BUFFER4_EXT: GLenum = 0x8829`"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_DRAW_BUFFER4_EXT: GLenum = 0x8829;
#[doc = "`GL_DRAW_BUFFER4_NV: GLenum = 0x8829`"]
#[cfg(any(feature = "GL_NV_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_draw_buffers"))))]
pub const GL_DRAW_BUFFER4_NV: GLenum = 0x8829;
#[doc = "`GL_DRAW_BUFFER5: GLenum = 0x882A`"]
pub const GL_DRAW_BUFFER5: GLenum = 0x882A;
#[doc = "`GL_DRAW_BUFFER5_EXT: GLenum = 0x882A`"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_DRAW_BUFFER5_EXT: GLenum = 0x882A;
#[doc = "`GL_DRAW_BUFFER5_NV: GLenum = 0x882A`"]
#[cfg(any(feature = "GL_NV_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_draw_buffers"))))]
pub const GL_DRAW_BUFFER5_NV: GLenum = 0x882A;
#[doc = "`GL_DRAW_BUFFER6: GLenum = 0x882B`"]
pub const GL_DRAW_BUFFER6: GLenum = 0x882B;
#[doc = "`GL_DRAW_BUFFER6_EXT: GLenum = 0x882B`"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_DRAW_BUFFER6_EXT: GLenum = 0x882B;
#[doc = "`GL_DRAW_BUFFER6_NV: GLenum = 0x882B`"]
#[cfg(any(feature = "GL_NV_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_draw_buffers"))))]
pub const GL_DRAW_BUFFER6_NV: GLenum = 0x882B;
#[doc = "`GL_DRAW_BUFFER7: GLenum = 0x882C`"]
pub const GL_DRAW_BUFFER7: GLenum = 0x882C;
#[doc = "`GL_DRAW_BUFFER7_EXT: GLenum = 0x882C`"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_DRAW_BUFFER7_EXT: GLenum = 0x882C;
#[doc = "`GL_DRAW_BUFFER7_NV: GLenum = 0x882C`"]
#[cfg(any(feature = "GL_NV_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_draw_buffers"))))]
pub const GL_DRAW_BUFFER7_NV: GLenum = 0x882C;
#[doc = "`GL_DRAW_BUFFER8: GLenum = 0x882D`"]
pub const GL_DRAW_BUFFER8: GLenum = 0x882D;
#[doc = "`GL_DRAW_BUFFER8_EXT: GLenum = 0x882D`"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_DRAW_BUFFER8_EXT: GLenum = 0x882D;
#[doc = "`GL_DRAW_BUFFER8_NV: GLenum = 0x882D`"]
#[cfg(any(feature = "GL_NV_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_draw_buffers"))))]
pub const GL_DRAW_BUFFER8_NV: GLenum = 0x882D;
#[doc = "`GL_DRAW_BUFFER9: GLenum = 0x882E`"]
pub const GL_DRAW_BUFFER9: GLenum = 0x882E;
#[doc = "`GL_DRAW_BUFFER9_EXT: GLenum = 0x882E`"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_DRAW_BUFFER9_EXT: GLenum = 0x882E;
#[doc = "`GL_DRAW_BUFFER9_NV: GLenum = 0x882E`"]
#[cfg(any(feature = "GL_NV_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_draw_buffers"))))]
pub const GL_DRAW_BUFFER9_NV: GLenum = 0x882E;
#[doc = "`GL_DRAW_BUFFER_EXT: GLenum = 0x0C01`"]
#[doc = "* **Group:** GetPName"]
#[cfg(any(feature = "GL_EXT_multiview_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_multiview_draw_buffers"))))]
pub const GL_DRAW_BUFFER_EXT: GLenum = 0x0C01;
#[doc = "`GL_DRAW_FRAMEBUFFER: GLenum = 0x8CA9`"]
#[doc = "* **Groups:** CheckFramebufferStatusTarget, FramebufferTarget"]
pub const GL_DRAW_FRAMEBUFFER: GLenum = 0x8CA9;
#[doc = "`GL_DRAW_FRAMEBUFFER_ANGLE: GLenum = 0x8CA9`"]
#[cfg(any(feature = "GL_ANGLE_framebuffer_blit"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_ANGLE_framebuffer_blit"))))]
pub const GL_DRAW_FRAMEBUFFER_ANGLE: GLenum = 0x8CA9;
#[doc = "`GL_DRAW_FRAMEBUFFER_APPLE: GLenum = 0x8CA9`"]
#[cfg(any(feature = "GL_APPLE_framebuffer_multisample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_framebuffer_multisample"))))]
pub const GL_DRAW_FRAMEBUFFER_APPLE: GLenum = 0x8CA9;
#[doc = "`GL_DRAW_FRAMEBUFFER_BINDING: GLenum = 0x8CA6`"]
#[doc = "* **Group:** GetPName"]
pub const GL_DRAW_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
#[doc = "`GL_DRAW_FRAMEBUFFER_BINDING_ANGLE: GLenum = 0x8CA6`"]
#[cfg(any(feature = "GL_ANGLE_framebuffer_blit"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_ANGLE_framebuffer_blit"))))]
pub const GL_DRAW_FRAMEBUFFER_BINDING_ANGLE: GLenum = 0x8CA6;
#[doc = "`GL_DRAW_FRAMEBUFFER_BINDING_APPLE: GLenum = 0x8CA6`"]
#[cfg(any(feature = "GL_APPLE_framebuffer_multisample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_framebuffer_multisample"))))]
pub const GL_DRAW_FRAMEBUFFER_BINDING_APPLE: GLenum = 0x8CA6;
#[doc = "`GL_DRAW_FRAMEBUFFER_BINDING_NV: GLenum = 0x8CA6`"]
#[cfg(any(feature = "GL_NV_framebuffer_blit"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_framebuffer_blit"))))]
pub const GL_DRAW_FRAMEBUFFER_BINDING_NV: GLenum = 0x8CA6;
#[doc = "`GL_DRAW_FRAMEBUFFER_NV: GLenum = 0x8CA9`"]
#[cfg(any(feature = "GL_NV_framebuffer_blit"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_framebuffer_blit"))))]
pub const GL_DRAW_FRAMEBUFFER_NV: GLenum = 0x8CA9;
#[doc = "`GL_DRAW_INDIRECT_BUFFER: GLenum = 0x8F3F`"]
#[doc = "* **Groups:** CopyBufferSubDataTarget, BufferTargetARB, BufferStorageTarget"]
pub const GL_DRAW_INDIRECT_BUFFER: GLenum = 0x8F3F;
#[doc = "`GL_DRAW_INDIRECT_BUFFER_BINDING: GLenum = 0x8F43`"]
pub const GL_DRAW_INDIRECT_BUFFER_BINDING: GLenum = 0x8F43;
#[doc = "`GL_DRIVER_UUID_EXT: GLenum = 0x9598`"]
#[doc = "* **Group:** GetPName"]
#[cfg(any(feature = "GL_EXT_memory_object", feature = "GL_EXT_semaphore"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_memory_object", feature = "GL_EXT_semaphore")))
)]
pub const GL_DRIVER_UUID_EXT: GLenum = 0x9598;
#[doc = "`GL_DST_ALPHA: GLenum = 0x0304`"]
#[doc = "* **Group:** BlendingFactor"]
pub const GL_DST_ALPHA: GLenum = 0x0304;
#[doc = "`GL_DST_ATOP_NV: GLenum = 0x928F`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_DST_ATOP_NV: GLenum = 0x928F;
#[doc = "`GL_DST_COLOR: GLenum = 0x0306`"]
#[doc = "* **Group:** BlendingFactor"]
pub const GL_DST_COLOR: GLenum = 0x0306;
#[doc = "`GL_DST_IN_NV: GLenum = 0x928B`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_DST_IN_NV: GLenum = 0x928B;
#[doc = "`GL_DST_NV: GLenum = 0x9287`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_DST_NV: GLenum = 0x9287;
#[doc = "`GL_DST_OUT_NV: GLenum = 0x928D`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_DST_OUT_NV: GLenum = 0x928D;
#[doc = "`GL_DST_OVER_NV: GLenum = 0x9289`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_DST_OVER_NV: GLenum = 0x9289;
#[doc = "`GL_DUP_FIRST_CUBIC_CURVE_TO_NV: GLenum = 0xF2`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_DUP_FIRST_CUBIC_CURVE_TO_NV: GLenum = 0xF2;
#[doc = "`GL_DUP_LAST_CUBIC_CURVE_TO_NV: GLenum = 0xF4`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_DUP_LAST_CUBIC_CURVE_TO_NV: GLenum = 0xF4;
#[doc = "`GL_DYNAMIC_COPY: GLenum = 0x88EA`"]
#[doc = "* **Groups:** VertexBufferObjectUsage, BufferUsageARB"]
pub const GL_DYNAMIC_COPY: GLenum = 0x88EA;
#[doc = "`GL_DYNAMIC_DRAW: GLenum = 0x88E8`"]
#[doc = "* **Groups:** VertexBufferObjectUsage, BufferUsageARB"]
pub const GL_DYNAMIC_DRAW: GLenum = 0x88E8;
#[doc = "`GL_DYNAMIC_READ: GLenum = 0x88E9`"]
#[doc = "* **Groups:** VertexBufferObjectUsage, BufferUsageARB"]
pub const GL_DYNAMIC_READ: GLenum = 0x88E9;
#[doc = "`GL_DYNAMIC_STORAGE_BIT_EXT: GLbitfield = 0x0100`"]
#[doc = "* **Group:** BufferStorageMask"]
#[cfg(any(feature = "GL_EXT_buffer_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_buffer_storage"))))]
pub const GL_DYNAMIC_STORAGE_BIT_EXT: GLbitfield = 0x0100;
#[doc = "`GL_EFFECTIVE_RASTER_SAMPLES_EXT: GLenum = 0x932C`"]
#[cfg(any(
    feature = "GL_EXT_raster_multisample",
    feature = "GL_NV_framebuffer_mixed_samples"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_raster_multisample",
        feature = "GL_NV_framebuffer_mixed_samples"
    )))
)]
pub const GL_EFFECTIVE_RASTER_SAMPLES_EXT: GLenum = 0x932C;
#[doc = "`GL_ELEMENT_ARRAY_BARRIER_BIT: GLbitfield = 0x00000002`"]
#[doc = "* **Group:** MemoryBarrierMask"]
pub const GL_ELEMENT_ARRAY_BARRIER_BIT: GLbitfield = 0x00000002;
#[doc = "`GL_ELEMENT_ARRAY_BUFFER: GLenum = 0x8893`"]
#[doc = "* **Groups:** CopyBufferSubDataTarget, BufferTargetARB, BufferStorageTarget"]
pub const GL_ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
#[doc = "`GL_ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 0x8895`"]
#[doc = "* **Group:** GetPName"]
pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 0x8895;
#[doc = "`GL_EQUAL: GLenum = 0x0202`"]
#[doc = "* **Groups:** StencilFunction, IndexFunctionEXT, AlphaFunction, DepthFunction"]
pub const GL_EQUAL: GLenum = 0x0202;
#[doc = "`GL_ETC1_RGB8_OES: GLenum = 0x8D64`"]
#[cfg(any(feature = "GL_OES_compressed_ETC1_RGB8_texture"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_OES_compressed_ETC1_RGB8_texture")))
)]
pub const GL_ETC1_RGB8_OES: GLenum = 0x8D64;
#[doc = "`GL_ETC1_SRGB8_NV: GLenum = 0x88EE`"]
#[cfg(any(feature = "GL_NV_sRGB_formats"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_sRGB_formats"))))]
pub const GL_ETC1_SRGB8_NV: GLenum = 0x88EE;
#[doc = "`GL_EXCLUSION: GLenum = 0x92A0`"]
pub const GL_EXCLUSION: GLenum = 0x92A0;
#[doc = "`GL_EXCLUSION_KHR: GLenum = 0x92A0`"]
#[cfg(any(feature = "GL_KHR_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_blend_equation_advanced"))))]
pub const GL_EXCLUSION_KHR: GLenum = 0x92A0;
#[doc = "`GL_EXCLUSION_NV: GLenum = 0x92A0`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_EXCLUSION_NV: GLenum = 0x92A0;
#[doc = "`GL_EXCLUSIVE_EXT: GLenum = 0x8F11`"]
#[cfg(any(feature = "GL_EXT_window_rectangles"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_window_rectangles"))))]
pub const GL_EXCLUSIVE_EXT: GLenum = 0x8F11;
#[doc = "`GL_EXTENSIONS: GLenum = 0x1F03`"]
#[doc = "* **Group:** StringName"]
pub const GL_EXTENSIONS: GLenum = 0x1F03;
#[doc = "`GL_FACTOR_MAX_AMD: GLenum = 0x901D`"]
#[cfg(any(feature = "GL_NV_blend_minmax_factor"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_minmax_factor"))))]
pub const GL_FACTOR_MAX_AMD: GLenum = 0x901D;
#[doc = "`GL_FACTOR_MIN_AMD: GLenum = 0x901C`"]
#[cfg(any(feature = "GL_NV_blend_minmax_factor"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_minmax_factor"))))]
pub const GL_FACTOR_MIN_AMD: GLenum = 0x901C;
#[doc = "`GL_FALSE: GLenum = 0`"]
#[doc = "* **Groups:** Boolean, VertexShaderWriteMaskEXT, ClampColorModeARB"]
pub const GL_FALSE: GLboolean = 0;
#[doc = "`GL_FASTEST: GLenum = 0x1101`"]
#[doc = "* **Group:** HintMode"]
pub const GL_FASTEST: GLenum = 0x1101;
#[doc = "`GL_FENCE_CONDITION_NV: GLenum = 0x84F4`"]
#[doc = "* **Group:** FenceParameterNameNV"]
#[cfg(any(feature = "GL_NV_fence"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_fence"))))]
pub const GL_FENCE_CONDITION_NV: GLenum = 0x84F4;
#[doc = "`GL_FENCE_STATUS_NV: GLenum = 0x84F3`"]
#[doc = "* **Group:** FenceParameterNameNV"]
#[cfg(any(feature = "GL_NV_fence"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_fence"))))]
pub const GL_FENCE_STATUS_NV: GLenum = 0x84F3;
#[doc = "`GL_FETCH_PER_SAMPLE_ARM: GLenum = 0x8F65`"]
#[cfg(any(feature = "GL_ARM_shader_framebuffer_fetch"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_ARM_shader_framebuffer_fetch"))))]
pub const GL_FETCH_PER_SAMPLE_ARM: GLenum = 0x8F65;
#[doc = "`GL_FILE_NAME_NV: GLenum = 0x9074`"]
#[doc = "* **Group:** PathFontTarget"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_FILE_NAME_NV: GLenum = 0x9074;
#[doc = "`GL_FILL_NV: GLenum = 0x1B02`"]
#[doc = "* **Group:** EvalMapsModeNV"]
#[cfg(any(feature = "GL_NV_polygon_mode"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_polygon_mode"))))]
pub const GL_FILL_NV: GLenum = 0x1B02;
#[doc = "`GL_FILL_RECTANGLE_NV: GLenum = 0x933C`"]
#[cfg(any(feature = "GL_NV_fill_rectangle"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_fill_rectangle"))))]
pub const GL_FILL_RECTANGLE_NV: GLenum = 0x933C;
#[doc = "`GL_FIRST_TO_REST_NV: GLenum = 0x90AF`"]
#[doc = "* **Group:** PathListMode"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_FIRST_TO_REST_NV: GLenum = 0x90AF;
#[doc = "`GL_FIRST_VERTEX_CONVENTION: GLenum = 0x8E4D`"]
#[doc = "* **Group:** VertexProvokingMode"]
pub const GL_FIRST_VERTEX_CONVENTION: GLenum = 0x8E4D;
#[doc = "`GL_FIRST_VERTEX_CONVENTION_EXT: GLenum = 0x8E4D`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_FIRST_VERTEX_CONVENTION_EXT: GLenum = 0x8E4D;
#[doc = "`GL_FIRST_VERTEX_CONVENTION_OES: GLenum = 0x8E4D`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_FIRST_VERTEX_CONVENTION_OES: GLenum = 0x8E4D;
#[doc = "`GL_FIXED: GLenum = 0x140C`"]
#[doc = "* **Groups:** VertexAttribPointerType, VertexAttribType"]
pub const GL_FIXED: GLenum = 0x140C;
#[doc = "`GL_FLOAT: GLenum = 0x1406`"]
#[doc = "* **Groups:** GlslTypeToken, MapTypeNV, SecondaryColorPointerTypeIBM, WeightPointerTypeARB, VertexWeightPointerTypeEXT, TangentPointerTypeEXT, BinormalPointerTypeEXT, FogCoordinatePointerType, FogPointerTypeEXT, FogPointerTypeIBM, IndexPointerType, ListNameType, NormalPointerType, PixelType, TexCoordPointerType, VertexPointerType, VertexAttribType, AttributeType, UniformType, VertexAttribPointerType"]
pub const GL_FLOAT: GLenum = 0x1406;
#[doc = "`GL_FLOAT16_NV: GLenum = 0x8FF8`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_FLOAT16_NV: GLenum = 0x8FF8;
#[doc = "`GL_FLOAT16_VEC2_NV: GLenum = 0x8FF9`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_FLOAT16_VEC2_NV: GLenum = 0x8FF9;
#[doc = "`GL_FLOAT16_VEC3_NV: GLenum = 0x8FFA`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_FLOAT16_VEC3_NV: GLenum = 0x8FFA;
#[doc = "`GL_FLOAT16_VEC4_NV: GLenum = 0x8FFB`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_FLOAT16_VEC4_NV: GLenum = 0x8FFB;
#[doc = "`GL_FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = 0x8DAD`"]
pub const GL_FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = 0x8DAD;
#[doc = "`GL_FLOAT_MAT2: GLenum = 0x8B5A`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_FLOAT_MAT2: GLenum = 0x8B5A;
#[doc = "`GL_FLOAT_MAT2x3: GLenum = 0x8B65`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_FLOAT_MAT2x3: GLenum = 0x8B65;
#[doc = "`GL_FLOAT_MAT2x3_NV: GLenum = 0x8B65`"]
#[doc = "* **Group:** AttributeType"]
#[cfg(any(feature = "GL_NV_non_square_matrices"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_non_square_matrices"))))]
pub const GL_FLOAT_MAT2x3_NV: GLenum = 0x8B65;
#[doc = "`GL_FLOAT_MAT2x4: GLenum = 0x8B66`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_FLOAT_MAT2x4: GLenum = 0x8B66;
#[doc = "`GL_FLOAT_MAT2x4_NV: GLenum = 0x8B66`"]
#[doc = "* **Group:** AttributeType"]
#[cfg(any(feature = "GL_NV_non_square_matrices"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_non_square_matrices"))))]
pub const GL_FLOAT_MAT2x4_NV: GLenum = 0x8B66;
#[doc = "`GL_FLOAT_MAT3: GLenum = 0x8B5B`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_FLOAT_MAT3: GLenum = 0x8B5B;
#[doc = "`GL_FLOAT_MAT3x2: GLenum = 0x8B67`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_FLOAT_MAT3x2: GLenum = 0x8B67;
#[doc = "`GL_FLOAT_MAT3x2_NV: GLenum = 0x8B67`"]
#[doc = "* **Group:** AttributeType"]
#[cfg(any(feature = "GL_NV_non_square_matrices"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_non_square_matrices"))))]
pub const GL_FLOAT_MAT3x2_NV: GLenum = 0x8B67;
#[doc = "`GL_FLOAT_MAT3x4: GLenum = 0x8B68`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_FLOAT_MAT3x4: GLenum = 0x8B68;
#[doc = "`GL_FLOAT_MAT3x4_NV: GLenum = 0x8B68`"]
#[doc = "* **Group:** AttributeType"]
#[cfg(any(feature = "GL_NV_non_square_matrices"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_non_square_matrices"))))]
pub const GL_FLOAT_MAT3x4_NV: GLenum = 0x8B68;
#[doc = "`GL_FLOAT_MAT4: GLenum = 0x8B5C`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_FLOAT_MAT4: GLenum = 0x8B5C;
#[doc = "`GL_FLOAT_MAT4x2: GLenum = 0x8B69`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_FLOAT_MAT4x2: GLenum = 0x8B69;
#[doc = "`GL_FLOAT_MAT4x2_NV: GLenum = 0x8B69`"]
#[doc = "* **Group:** AttributeType"]
#[cfg(any(feature = "GL_NV_non_square_matrices"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_non_square_matrices"))))]
pub const GL_FLOAT_MAT4x2_NV: GLenum = 0x8B69;
#[doc = "`GL_FLOAT_MAT4x3: GLenum = 0x8B6A`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_FLOAT_MAT4x3: GLenum = 0x8B6A;
#[doc = "`GL_FLOAT_MAT4x3_NV: GLenum = 0x8B6A`"]
#[doc = "* **Group:** AttributeType"]
#[cfg(any(feature = "GL_NV_non_square_matrices"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_non_square_matrices"))))]
pub const GL_FLOAT_MAT4x3_NV: GLenum = 0x8B6A;
#[doc = "`GL_FLOAT_VEC2: GLenum = 0x8B50`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_FLOAT_VEC2: GLenum = 0x8B50;
#[doc = "`GL_FLOAT_VEC3: GLenum = 0x8B51`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_FLOAT_VEC3: GLenum = 0x8B51;
#[doc = "`GL_FLOAT_VEC4: GLenum = 0x8B52`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_FLOAT_VEC4: GLenum = 0x8B52;
#[doc = "`GL_FONT_ASCENDER_BIT_NV: GLbitfield = 0x00200000`"]
#[doc = "* **Group:** PathMetricMask"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_FONT_ASCENDER_BIT_NV: GLbitfield = 0x00200000;
#[doc = "`GL_FONT_DESCENDER_BIT_NV: GLbitfield = 0x00400000`"]
#[doc = "* **Group:** PathMetricMask"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_FONT_DESCENDER_BIT_NV: GLbitfield = 0x00400000;
#[doc = "`GL_FONT_GLYPHS_AVAILABLE_NV: GLenum = 0x9368`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_FONT_GLYPHS_AVAILABLE_NV: GLenum = 0x9368;
#[doc = "`GL_FONT_HAS_KERNING_BIT_NV: GLbitfield = 0x10000000`"]
#[doc = "* **Group:** PathMetricMask"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_FONT_HAS_KERNING_BIT_NV: GLbitfield = 0x10000000;
#[doc = "`GL_FONT_HEIGHT_BIT_NV: GLbitfield = 0x00800000`"]
#[doc = "* **Group:** PathMetricMask"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_FONT_HEIGHT_BIT_NV: GLbitfield = 0x00800000;
#[doc = "`GL_FONT_MAX_ADVANCE_HEIGHT_BIT_NV: GLbitfield = 0x02000000`"]
#[doc = "* **Group:** PathMetricMask"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_FONT_MAX_ADVANCE_HEIGHT_BIT_NV: GLbitfield = 0x02000000;
#[doc = "`GL_FONT_MAX_ADVANCE_WIDTH_BIT_NV: GLbitfield = 0x01000000`"]
#[doc = "* **Group:** PathMetricMask"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_FONT_MAX_ADVANCE_WIDTH_BIT_NV: GLbitfield = 0x01000000;
#[doc = "`GL_FONT_NUM_GLYPH_INDICES_BIT_NV: GLbitfield = 0x20000000`"]
#[doc = "* **Group:** PathMetricMask"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_FONT_NUM_GLYPH_INDICES_BIT_NV: GLbitfield = 0x20000000;
#[doc = "`GL_FONT_TARGET_UNAVAILABLE_NV: GLenum = 0x9369`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_FONT_TARGET_UNAVAILABLE_NV: GLenum = 0x9369;
#[doc = "`GL_FONT_UNAVAILABLE_NV: GLenum = 0x936A`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_FONT_UNAVAILABLE_NV: GLenum = 0x936A;
#[doc = "`GL_FONT_UNDERLINE_POSITION_BIT_NV: GLbitfield = 0x04000000`"]
#[doc = "* **Group:** PathMetricMask"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_FONT_UNDERLINE_POSITION_BIT_NV: GLbitfield = 0x04000000;
#[doc = "`GL_FONT_UNDERLINE_THICKNESS_BIT_NV: GLbitfield = 0x08000000`"]
#[doc = "* **Group:** PathMetricMask"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_FONT_UNDERLINE_THICKNESS_BIT_NV: GLbitfield = 0x08000000;
#[doc = "`GL_FONT_UNINTELLIGIBLE_NV: GLenum = 0x936B`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_FONT_UNINTELLIGIBLE_NV: GLenum = 0x936B;
#[doc = "`GL_FONT_UNITS_PER_EM_BIT_NV: GLbitfield = 0x00100000`"]
#[doc = "* **Group:** PathMetricMask"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_FONT_UNITS_PER_EM_BIT_NV: GLbitfield = 0x00100000;
#[doc = "`GL_FONT_X_MAX_BOUNDS_BIT_NV: GLbitfield = 0x00040000`"]
#[doc = "* **Group:** PathMetricMask"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_FONT_X_MAX_BOUNDS_BIT_NV: GLbitfield = 0x00040000;
#[doc = "`GL_FONT_X_MIN_BOUNDS_BIT_NV: GLbitfield = 0x00010000`"]
#[doc = "* **Group:** PathMetricMask"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_FONT_X_MIN_BOUNDS_BIT_NV: GLbitfield = 0x00010000;
#[doc = "`GL_FONT_Y_MAX_BOUNDS_BIT_NV: GLbitfield = 0x00080000`"]
#[doc = "* **Group:** PathMetricMask"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_FONT_Y_MAX_BOUNDS_BIT_NV: GLbitfield = 0x00080000;
#[doc = "`GL_FONT_Y_MIN_BOUNDS_BIT_NV: GLbitfield = 0x00020000`"]
#[doc = "* **Group:** PathMetricMask"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_FONT_Y_MIN_BOUNDS_BIT_NV: GLbitfield = 0x00020000;
#[doc = "`GL_FOVEATION_ENABLE_BIT_QCOM: GLbitfield = 0x00000001`"]
#[doc = "* **Group:** FoveationConfigBitQCOM"]
#[cfg(any(
    feature = "GL_QCOM_framebuffer_foveated",
    feature = "GL_QCOM_texture_foveated"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_QCOM_framebuffer_foveated",
        feature = "GL_QCOM_texture_foveated"
    )))
)]
pub const GL_FOVEATION_ENABLE_BIT_QCOM: GLbitfield = 0x00000001;
#[doc = "`GL_FOVEATION_SCALED_BIN_METHOD_BIT_QCOM: GLbitfield = 0x00000002`"]
#[doc = "* **Group:** FoveationConfigBitQCOM"]
#[cfg(any(
    feature = "GL_QCOM_framebuffer_foveated",
    feature = "GL_QCOM_motion_estimation",
    feature = "GL_QCOM_texture_foveated"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_QCOM_framebuffer_foveated",
        feature = "GL_QCOM_motion_estimation",
        feature = "GL_QCOM_texture_foveated"
    )))
)]
pub const GL_FOVEATION_SCALED_BIN_METHOD_BIT_QCOM: GLbitfield = 0x00000002;
#[doc = "`GL_FOVEATION_SUBSAMPLED_LAYOUT_METHOD_BIT_QCOM: GLbitfield = 0x00000004`"]
#[doc = "* **Group:** FoveationConfigBitQCOM"]
#[cfg(any(feature = "GL_QCOM_texture_foveated_subsampled_layout"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_QCOM_texture_foveated_subsampled_layout")))
)]
pub const GL_FOVEATION_SUBSAMPLED_LAYOUT_METHOD_BIT_QCOM: GLbitfield = 0x00000004;
#[doc = "`GL_FRACTIONAL_EVEN: GLenum = 0x8E7C`"]
pub const GL_FRACTIONAL_EVEN: GLenum = 0x8E7C;
#[doc = "`GL_FRACTIONAL_EVEN_EXT: GLenum = 0x8E7C`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_FRACTIONAL_EVEN_EXT: GLenum = 0x8E7C;
#[doc = "`GL_FRACTIONAL_EVEN_OES: GLenum = 0x8E7C`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_FRACTIONAL_EVEN_OES: GLenum = 0x8E7C;
#[doc = "`GL_FRACTIONAL_ODD: GLenum = 0x8E7B`"]
pub const GL_FRACTIONAL_ODD: GLenum = 0x8E7B;
#[doc = "`GL_FRACTIONAL_ODD_EXT: GLenum = 0x8E7B`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_FRACTIONAL_ODD_EXT: GLenum = 0x8E7B;
#[doc = "`GL_FRACTIONAL_ODD_OES: GLenum = 0x8E7B`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_FRACTIONAL_ODD_OES: GLenum = 0x8E7B;
#[doc = "`GL_FRAGMENT_COVERAGE_COLOR_NV: GLenum = 0x92DE`"]
#[cfg(any(feature = "GL_NV_fragment_coverage_to_color"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_fragment_coverage_to_color"))))]
pub const GL_FRAGMENT_COVERAGE_COLOR_NV: GLenum = 0x92DE;
#[doc = "`GL_FRAGMENT_COVERAGE_TO_COLOR_NV: GLenum = 0x92DD`"]
#[cfg(any(feature = "GL_NV_fragment_coverage_to_color"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_fragment_coverage_to_color"))))]
pub const GL_FRAGMENT_COVERAGE_TO_COLOR_NV: GLenum = 0x92DD;
#[doc = "`GL_FRAGMENT_INPUT_NV: GLenum = 0x936D`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_FRAGMENT_INPUT_NV: GLenum = 0x936D;
#[doc = "`GL_FRAGMENT_INTERPOLATION_OFFSET_BITS: GLenum = 0x8E5D`"]
pub const GL_FRAGMENT_INTERPOLATION_OFFSET_BITS: GLenum = 0x8E5D;
#[doc = "`GL_FRAGMENT_INTERPOLATION_OFFSET_BITS_OES: GLenum = 0x8E5D`"]
#[cfg(any(feature = "GL_OES_shader_multisample_interpolation"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_OES_shader_multisample_interpolation")))
)]
pub const GL_FRAGMENT_INTERPOLATION_OFFSET_BITS_OES: GLenum = 0x8E5D;
#[doc = "`GL_FRAGMENT_SHADER: GLenum = 0x8B30`"]
#[doc = "* **Groups:** PipelineParameterName, ShaderType"]
pub const GL_FRAGMENT_SHADER: GLenum = 0x8B30;
#[doc = "`GL_FRAGMENT_SHADER_BIT: GLbitfield = 0x00000002`"]
#[doc = "* **Group:** UseProgramStageMask"]
pub const GL_FRAGMENT_SHADER_BIT: GLbitfield = 0x00000002;
#[doc = "`GL_FRAGMENT_SHADER_BIT_EXT: GLbitfield = 0x00000002`"]
#[doc = "* **Group:** UseProgramStageMask"]
#[cfg(any(feature = "GL_EXT_separate_shader_objects"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_separate_shader_objects"))))]
pub const GL_FRAGMENT_SHADER_BIT_EXT: GLbitfield = 0x00000002;
#[doc = "`GL_FRAGMENT_SHADER_DERIVATIVE_HINT: GLenum = 0x8B8B`"]
#[doc = "* **Groups:** HintTarget, GetPName"]
pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT: GLenum = 0x8B8B;
#[doc = "`GL_FRAGMENT_SHADER_DERIVATIVE_HINT_OES: GLenum = 0x8B8B`"]
#[doc = "* **Group:** HintTarget"]
#[cfg(any(feature = "GL_OES_standard_derivatives"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_standard_derivatives"))))]
pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT_OES: GLenum = 0x8B8B;
#[doc = "`GL_FRAGMENT_SHADER_DISCARDS_SAMPLES_EXT: GLenum = 0x8A52`"]
#[cfg(any(
    feature = "GL_EXT_shader_framebuffer_fetch",
    feature = "GL_EXT_shader_framebuffer_fetch_non_coherent"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_shader_framebuffer_fetch",
        feature = "GL_EXT_shader_framebuffer_fetch_non_coherent"
    )))
)]
pub const GL_FRAGMENT_SHADER_DISCARDS_SAMPLES_EXT: GLenum = 0x8A52;
#[doc = "`GL_FRAGMENT_SHADER_FRAMEBUFFER_FETCH_MRT_ARM: GLenum = 0x8F66`"]
#[cfg(any(feature = "GL_ARM_shader_framebuffer_fetch"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_ARM_shader_framebuffer_fetch"))))]
pub const GL_FRAGMENT_SHADER_FRAMEBUFFER_FETCH_MRT_ARM: GLenum = 0x8F66;
#[doc = "`GL_FRAMEBUFFER: GLenum = 0x8D40`"]
#[doc = "* **Groups:** ObjectIdentifier, FramebufferTarget, CheckFramebufferStatusTarget"]
pub const GL_FRAMEBUFFER: GLenum = 0x8D40;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = 0x8215`"]
#[doc = "* **Group:** FramebufferAttachmentParameterName"]
pub const GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = 0x8215;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_ANGLE: GLenum = 0x93A3`"]
#[cfg(any(feature = "GL_ANGLE_texture_usage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_ANGLE_texture_usage"))))]
pub const GL_FRAMEBUFFER_ATTACHMENT_ANGLE: GLenum = 0x93A3;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = 0x8214`"]
#[doc = "* **Group:** FramebufferAttachmentParameterName"]
pub const GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = 0x8214;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = 0x8210`"]
#[doc = "* **Group:** FramebufferAttachmentParameterName"]
pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = 0x8210;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT: GLenum = 0x8210`"]
#[doc = "* **Group:** FramebufferAttachmentParameterName"]
#[cfg(any(feature = "GL_EXT_sRGB"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_sRGB"))))]
pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT: GLenum = 0x8210;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = 0x8211`"]
#[doc = "* **Group:** FramebufferAttachmentParameterName"]
pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = 0x8211;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT: GLenum = 0x8211`"]
#[doc = "* **Group:** FramebufferAttachmentParameterName"]
#[cfg(any(feature = "GL_EXT_color_buffer_half_float"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_color_buffer_half_float"))))]
pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT: GLenum = 0x8211;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = 0x8216`"]
#[doc = "* **Group:** FramebufferAttachmentParameterName"]
pub const GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = 0x8216;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = 0x8213`"]
#[doc = "* **Group:** FramebufferAttachmentParameterName"]
pub const GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = 0x8213;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_LAYERED: GLenum = 0x8DA7`"]
#[doc = "* **Group:** FramebufferAttachmentParameterName"]
pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED: GLenum = 0x8DA7;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_LAYERED_EXT: GLenum = 0x8DA7`"]
#[doc = "* **Group:** FramebufferAttachmentParameterName"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED_EXT: GLenum = 0x8DA7;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_LAYERED_OES: GLenum = 0x8DA7`"]
#[doc = "* **Group:** FramebufferAttachmentParameterName"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED_OES: GLenum = 0x8DA7;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 0x8CD1`"]
#[doc = "* **Group:** FramebufferAttachmentParameterName"]
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 0x8CD1;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 0x8CD0`"]
#[doc = "* **Group:** FramebufferAttachmentParameterName"]
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 0x8CD0;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = 0x8212`"]
#[doc = "* **Group:** FramebufferAttachmentParameterName"]
pub const GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = 0x8212;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = 0x8217`"]
#[doc = "* **Group:** FramebufferAttachmentParameterName"]
pub const GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = 0x8217;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_3D_ZOFFSET_OES: GLenum = 0x8CD4`"]
#[doc = "* **Group:** FramebufferAttachmentParameterName"]
#[cfg(any(feature = "GL_OES_texture_3D"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_3D"))))]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_3D_ZOFFSET_OES: GLenum = 0x8CD4;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_BASE_VIEW_INDEX_OVR: GLenum = 0x9632`"]
#[doc = "* **Group:** FramebufferAttachmentParameterName"]
#[cfg(any(feature = "GL_OVR_multiview"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OVR_multiview"))))]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_BASE_VIEW_INDEX_OVR: GLenum = 0x9632;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 0x8CD3`"]
#[doc = "* **Group:** FramebufferAttachmentParameterName"]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 0x8CD3;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = 0x8CD4`"]
#[doc = "* **Group:** FramebufferAttachmentParameterName"]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = 0x8CD4;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 0x8CD2`"]
#[doc = "* **Group:** FramebufferAttachmentParameterName"]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 0x8CD2;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_NUM_VIEWS_OVR: GLenum = 0x9630`"]
#[doc = "* **Group:** FramebufferAttachmentParameterName"]
#[cfg(any(feature = "GL_OVR_multiview"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OVR_multiview"))))]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_NUM_VIEWS_OVR: GLenum = 0x9630;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_SAMPLES_EXT: GLenum = 0x8D6C`"]
#[doc = "* **Group:** FramebufferAttachmentParameterName"]
#[cfg(any(feature = "GL_EXT_multisampled_render_to_texture"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_multisampled_render_to_texture")))
)]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_SAMPLES_EXT: GLenum = 0x8D6C;
#[doc = "`GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_SCALE_IMG: GLenum = 0x913F`"]
#[doc = "* **Group:** FramebufferAttachmentParameterName"]
#[cfg(any(feature = "GL_IMG_framebuffer_downsample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_IMG_framebuffer_downsample"))))]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_SCALE_IMG: GLenum = 0x913F;
#[doc = "`GL_FRAMEBUFFER_BARRIER_BIT: GLbitfield = 0x00000400`"]
#[doc = "* **Group:** MemoryBarrierMask"]
pub const GL_FRAMEBUFFER_BARRIER_BIT: GLbitfield = 0x00000400;
#[doc = "`GL_FRAMEBUFFER_BINDING: GLenum = 0x8CA6`"]
pub const GL_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
#[doc = "`GL_FRAMEBUFFER_COMPLETE: GLenum = 0x8CD5`"]
#[doc = "* **Group:** FramebufferStatus"]
pub const GL_FRAMEBUFFER_COMPLETE: GLenum = 0x8CD5;
#[doc = "`GL_FRAMEBUFFER_DEFAULT: GLenum = 0x8218`"]
pub const GL_FRAMEBUFFER_DEFAULT: GLenum = 0x8218;
#[doc = "`GL_FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9314`"]
#[doc = "* **Groups:** GetFramebufferParameter, FramebufferParameterName"]
pub const GL_FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9314;
#[doc = "`GL_FRAMEBUFFER_DEFAULT_HEIGHT: GLenum = 0x9311`"]
#[doc = "* **Groups:** GetFramebufferParameter, FramebufferParameterName"]
pub const GL_FRAMEBUFFER_DEFAULT_HEIGHT: GLenum = 0x9311;
#[doc = "`GL_FRAMEBUFFER_DEFAULT_LAYERS: GLenum = 0x9312`"]
#[doc = "* **Groups:** GetFramebufferParameter, FramebufferParameterName"]
pub const GL_FRAMEBUFFER_DEFAULT_LAYERS: GLenum = 0x9312;
#[doc = "`GL_FRAMEBUFFER_DEFAULT_LAYERS_EXT: GLenum = 0x9312`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_FRAMEBUFFER_DEFAULT_LAYERS_EXT: GLenum = 0x9312;
#[doc = "`GL_FRAMEBUFFER_DEFAULT_LAYERS_OES: GLenum = 0x9312`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_FRAMEBUFFER_DEFAULT_LAYERS_OES: GLenum = 0x9312;
#[doc = "`GL_FRAMEBUFFER_DEFAULT_SAMPLES: GLenum = 0x9313`"]
#[doc = "* **Groups:** GetFramebufferParameter, FramebufferParameterName"]
pub const GL_FRAMEBUFFER_DEFAULT_SAMPLES: GLenum = 0x9313;
#[doc = "`GL_FRAMEBUFFER_DEFAULT_WIDTH: GLenum = 0x9310`"]
#[doc = "* **Groups:** GetFramebufferParameter, FramebufferParameterName"]
pub const GL_FRAMEBUFFER_DEFAULT_WIDTH: GLenum = 0x9310;
#[doc = "`GL_FRAMEBUFFER_FETCH_NONCOHERENT_QCOM: GLenum = 0x96A2`"]
#[doc = "* **Group:** FramebufferFetchNoncoherent"]
#[cfg(any(feature = "GL_QCOM_shader_framebuffer_fetch_noncoherent"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_QCOM_shader_framebuffer_fetch_noncoherent")))
)]
pub const GL_FRAMEBUFFER_FETCH_NONCOHERENT_QCOM: GLenum = 0x96A2;
#[doc = "`GL_FRAMEBUFFER_FLIP_X_MESA: GLenum = 0x8BBC`"]
#[cfg(any(feature = "GL_MESA_framebuffer_flip_x"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_MESA_framebuffer_flip_x"))))]
pub const GL_FRAMEBUFFER_FLIP_X_MESA: GLenum = 0x8BBC;
#[doc = "`GL_FRAMEBUFFER_FLIP_Y_MESA: GLenum = 0x8BBB`"]
#[cfg(any(feature = "GL_MESA_framebuffer_flip_y"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_MESA_framebuffer_flip_y"))))]
pub const GL_FRAMEBUFFER_FLIP_Y_MESA: GLenum = 0x8BBB;
#[doc = "`GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 0x8CD6`"]
#[doc = "* **Group:** FramebufferStatus"]
pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 0x8CD6;
#[doc = "`GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS: GLenum = 0x8CD9`"]
pub const GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS: GLenum = 0x8CD9;
#[doc = "`GL_FRAMEBUFFER_INCOMPLETE_FOVEATION_QCOM: GLenum = 0x8BFF`"]
#[cfg(any(feature = "GL_QCOM_texture_foveated"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_texture_foveated"))))]
pub const GL_FRAMEBUFFER_INCOMPLETE_FOVEATION_QCOM: GLenum = 0x8BFF;
#[doc = "`GL_FRAMEBUFFER_INCOMPLETE_INSUFFICIENT_SHADER_COMBINED_LOCAL_STORAGE_EXT: GLenum = 0x9652`"]
#[cfg(any(feature = "GL_EXT_shader_pixel_local_storage2"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_shader_pixel_local_storage2"))))]
pub const GL_FRAMEBUFFER_INCOMPLETE_INSUFFICIENT_SHADER_COMBINED_LOCAL_STORAGE_EXT: GLenum = 0x9652;
#[doc = "`GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: GLenum = 0x8DA8`"]
#[doc = "* **Group:** FramebufferStatus"]
pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: GLenum = 0x8DA8;
#[doc = "`GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS_EXT: GLenum = 0x8DA8`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS_EXT: GLenum = 0x8DA8;
#[doc = "`GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS_OES: GLenum = 0x8DA8`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS_OES: GLenum = 0x8DA8;
#[doc = "`GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 0x8CD7`"]
#[doc = "* **Group:** FramebufferStatus"]
pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 0x8CD7;
#[doc = "`GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = 0x8D56`"]
#[doc = "* **Group:** FramebufferStatus"]
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = 0x8D56;
#[doc = "`GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_AND_DOWNSAMPLE_IMG: GLenum = 0x913C`"]
#[cfg(any(feature = "GL_IMG_framebuffer_downsample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_IMG_framebuffer_downsample"))))]
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_AND_DOWNSAMPLE_IMG: GLenum = 0x913C;
#[doc = "`GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_ANGLE: GLenum = 0x8D56`"]
#[cfg(any(feature = "GL_ANGLE_framebuffer_multisample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_ANGLE_framebuffer_multisample"))))]
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_ANGLE: GLenum = 0x8D56;
#[doc = "`GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_APPLE: GLenum = 0x8D56`"]
#[cfg(any(feature = "GL_APPLE_framebuffer_multisample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_framebuffer_multisample"))))]
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_APPLE: GLenum = 0x8D56;
#[doc = "`GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_EXT: GLenum = 0x8D56`"]
#[cfg(any(feature = "GL_EXT_multisampled_render_to_texture"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_multisampled_render_to_texture")))
)]
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_EXT: GLenum = 0x8D56;
#[doc = "`GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_IMG: GLenum = 0x9134`"]
#[cfg(any(feature = "GL_IMG_multisampled_render_to_texture"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_IMG_multisampled_render_to_texture")))
)]
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_IMG: GLenum = 0x9134;
#[doc = "`GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_NV: GLenum = 0x8D56`"]
#[cfg(any(feature = "GL_NV_framebuffer_multisample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_framebuffer_multisample"))))]
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_NV: GLenum = 0x8D56;
#[doc = "`GL_FRAMEBUFFER_INCOMPLETE_VIEW_TARGETS_OVR: GLenum = 0x9633`"]
#[cfg(any(feature = "GL_OVR_multiview"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OVR_multiview"))))]
pub const GL_FRAMEBUFFER_INCOMPLETE_VIEW_TARGETS_OVR: GLenum = 0x9633;
#[doc = "`GL_FRAMEBUFFER_PROGRAMMABLE_SAMPLE_LOCATIONS_NV: GLenum = 0x9342`"]
#[cfg(any(feature = "GL_NV_sample_locations"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_sample_locations"))))]
pub const GL_FRAMEBUFFER_PROGRAMMABLE_SAMPLE_LOCATIONS_NV: GLenum = 0x9342;
#[doc = "`GL_FRAMEBUFFER_SAMPLE_LOCATION_PIXEL_GRID_NV: GLenum = 0x9343`"]
#[cfg(any(feature = "GL_NV_sample_locations"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_sample_locations"))))]
pub const GL_FRAMEBUFFER_SAMPLE_LOCATION_PIXEL_GRID_NV: GLenum = 0x9343;
#[doc = "`GL_FRAMEBUFFER_SRGB_EXT: GLenum = 0x8DB9`"]
#[cfg(any(feature = "GL_EXT_sRGB_write_control"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_sRGB_write_control"))))]
pub const GL_FRAMEBUFFER_SRGB_EXT: GLenum = 0x8DB9;
#[doc = "`GL_FRAMEBUFFER_SWAP_XY_MESA: GLenum = 0x8BBD`"]
#[cfg(any(feature = "GL_MESA_framebuffer_swap_xy"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_MESA_framebuffer_swap_xy"))))]
pub const GL_FRAMEBUFFER_SWAP_XY_MESA: GLenum = 0x8BBD;
#[doc = "`GL_FRAMEBUFFER_UNDEFINED: GLenum = 0x8219`"]
#[doc = "* **Group:** FramebufferStatus"]
pub const GL_FRAMEBUFFER_UNDEFINED: GLenum = 0x8219;
#[doc = "`GL_FRAMEBUFFER_UNDEFINED_OES: GLenum = 0x8219`"]
#[cfg(any(feature = "GL_OES_surfaceless_context"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_surfaceless_context"))))]
pub const GL_FRAMEBUFFER_UNDEFINED_OES: GLenum = 0x8219;
#[doc = "`GL_FRAMEBUFFER_UNSUPPORTED: GLenum = 0x8CDD`"]
#[doc = "* **Group:** FramebufferStatus"]
pub const GL_FRAMEBUFFER_UNSUPPORTED: GLenum = 0x8CDD;
#[doc = "`GL_FRONT: GLenum = 0x0404`"]
#[doc = "* **Groups:** ColorBuffer, ColorMaterialFace, CullFaceMode, DrawBufferMode, ReadBufferMode, StencilFaceDirection, MaterialFace"]
pub const GL_FRONT: GLenum = 0x0404;
#[doc = "`GL_FRONT_AND_BACK: GLenum = 0x0408`"]
#[doc = "* **Groups:** ColorBuffer, ColorMaterialFace, CullFaceMode, DrawBufferMode, StencilFaceDirection, MaterialFace"]
pub const GL_FRONT_AND_BACK: GLenum = 0x0408;
#[doc = "`GL_FRONT_FACE: GLenum = 0x0B46`"]
#[doc = "* **Group:** GetPName"]
pub const GL_FRONT_FACE: GLenum = 0x0B46;
#[doc = "`GL_FUNC_ADD: GLenum = 0x8006`"]
#[doc = "* **Group:** BlendEquationModeEXT"]
pub const GL_FUNC_ADD: GLenum = 0x8006;
#[doc = "`GL_FUNC_REVERSE_SUBTRACT: GLenum = 0x800B`"]
#[doc = "* **Group:** BlendEquationModeEXT"]
pub const GL_FUNC_REVERSE_SUBTRACT: GLenum = 0x800B;
#[doc = "`GL_FUNC_SUBTRACT: GLenum = 0x800A`"]
#[doc = "* **Group:** BlendEquationModeEXT"]
pub const GL_FUNC_SUBTRACT: GLenum = 0x800A;
#[doc = "`GL_GCCSO_SHADER_BINARY_FJ: GLenum = 0x9260`"]
#[doc = "* **Group:** ShaderBinaryFormat"]
#[cfg(any(feature = "GL_FJ_shader_binary_GCCSO"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_FJ_shader_binary_GCCSO"))))]
pub const GL_GCCSO_SHADER_BINARY_FJ: GLenum = 0x9260;
#[doc = "`GL_GENERATE_MIPMAP_HINT: GLenum = 0x8192`"]
#[doc = "* **Group:** HintTarget"]
pub const GL_GENERATE_MIPMAP_HINT: GLenum = 0x8192;
#[doc = "`GL_GEOMETRY_INPUT_TYPE: GLenum = 0x8917`"]
#[doc = "* **Group:** ProgramPropertyARB"]
pub const GL_GEOMETRY_INPUT_TYPE: GLenum = 0x8917;
#[doc = "`GL_GEOMETRY_LINKED_INPUT_TYPE_EXT: GLenum = 0x8917`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_GEOMETRY_LINKED_INPUT_TYPE_EXT: GLenum = 0x8917;
#[doc = "`GL_GEOMETRY_LINKED_INPUT_TYPE_OES: GLenum = 0x8917`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_GEOMETRY_LINKED_INPUT_TYPE_OES: GLenum = 0x8917;
#[doc = "`GL_GEOMETRY_LINKED_OUTPUT_TYPE_EXT: GLenum = 0x8918`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_GEOMETRY_LINKED_OUTPUT_TYPE_EXT: GLenum = 0x8918;
#[doc = "`GL_GEOMETRY_LINKED_OUTPUT_TYPE_OES: GLenum = 0x8918`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_GEOMETRY_LINKED_OUTPUT_TYPE_OES: GLenum = 0x8918;
#[doc = "`GL_GEOMETRY_LINKED_VERTICES_OUT_EXT: GLenum = 0x8916`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_GEOMETRY_LINKED_VERTICES_OUT_EXT: GLenum = 0x8916;
#[doc = "`GL_GEOMETRY_LINKED_VERTICES_OUT_OES: GLenum = 0x8916`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_GEOMETRY_LINKED_VERTICES_OUT_OES: GLenum = 0x8916;
#[doc = "`GL_GEOMETRY_OUTPUT_TYPE: GLenum = 0x8918`"]
#[doc = "* **Group:** ProgramPropertyARB"]
pub const GL_GEOMETRY_OUTPUT_TYPE: GLenum = 0x8918;
#[doc = "`GL_GEOMETRY_SHADER: GLenum = 0x8DD9`"]
#[doc = "* **Groups:** PipelineParameterName, ShaderType"]
pub const GL_GEOMETRY_SHADER: GLenum = 0x8DD9;
#[doc = "`GL_GEOMETRY_SHADER_BIT: GLbitfield = 0x00000004`"]
#[doc = "* **Group:** UseProgramStageMask"]
pub const GL_GEOMETRY_SHADER_BIT: GLbitfield = 0x00000004;
#[doc = "`GL_GEOMETRY_SHADER_BIT_EXT: GLbitfield = 0x00000004`"]
#[doc = "* **Group:** UseProgramStageMask"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_GEOMETRY_SHADER_BIT_EXT: GLbitfield = 0x00000004;
#[doc = "`GL_GEOMETRY_SHADER_BIT_OES: GLbitfield = 0x00000004`"]
#[doc = "* **Group:** UseProgramStageMask"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_GEOMETRY_SHADER_BIT_OES: GLbitfield = 0x00000004;
#[doc = "`GL_GEOMETRY_SHADER_EXT: GLenum = 0x8DD9`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_GEOMETRY_SHADER_EXT: GLenum = 0x8DD9;
#[doc = "`GL_GEOMETRY_SHADER_INVOCATIONS: GLenum = 0x887F`"]
pub const GL_GEOMETRY_SHADER_INVOCATIONS: GLenum = 0x887F;
#[doc = "`GL_GEOMETRY_SHADER_INVOCATIONS_EXT: GLenum = 0x887F`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_GEOMETRY_SHADER_INVOCATIONS_EXT: GLenum = 0x887F;
#[doc = "`GL_GEOMETRY_SHADER_INVOCATIONS_OES: GLenum = 0x887F`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_GEOMETRY_SHADER_INVOCATIONS_OES: GLenum = 0x887F;
#[doc = "`GL_GEOMETRY_SHADER_OES: GLenum = 0x8DD9`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_GEOMETRY_SHADER_OES: GLenum = 0x8DD9;
#[doc = "`GL_GEOMETRY_VERTICES_OUT: GLenum = 0x8916`"]
#[doc = "* **Group:** ProgramPropertyARB"]
pub const GL_GEOMETRY_VERTICES_OUT: GLenum = 0x8916;
#[doc = "`GL_GEQUAL: GLenum = 0x0206`"]
#[doc = "* **Groups:** StencilFunction, IndexFunctionEXT, AlphaFunction, DepthFunction"]
pub const GL_GEQUAL: GLenum = 0x0206;
#[doc = "`GL_GLYPH_HAS_KERNING_BIT_NV: GLbitfield = 0x100`"]
#[doc = "* **Group:** PathMetricMask"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_GLYPH_HAS_KERNING_BIT_NV: GLbitfield = 0x100;
#[doc = "`GL_GLYPH_HEIGHT_BIT_NV: GLbitfield = 0x02`"]
#[doc = "* **Group:** PathMetricMask"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_GLYPH_HEIGHT_BIT_NV: GLbitfield = 0x02;
#[doc = "`GL_GLYPH_HORIZONTAL_BEARING_ADVANCE_BIT_NV: GLbitfield = 0x10`"]
#[doc = "* **Group:** PathMetricMask"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_GLYPH_HORIZONTAL_BEARING_ADVANCE_BIT_NV: GLbitfield = 0x10;
#[doc = "`GL_GLYPH_HORIZONTAL_BEARING_X_BIT_NV: GLbitfield = 0x04`"]
#[doc = "* **Group:** PathMetricMask"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_GLYPH_HORIZONTAL_BEARING_X_BIT_NV: GLbitfield = 0x04;
#[doc = "`GL_GLYPH_HORIZONTAL_BEARING_Y_BIT_NV: GLbitfield = 0x08`"]
#[doc = "* **Group:** PathMetricMask"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_GLYPH_HORIZONTAL_BEARING_Y_BIT_NV: GLbitfield = 0x08;
#[doc = "`GL_GLYPH_VERTICAL_BEARING_ADVANCE_BIT_NV: GLbitfield = 0x80`"]
#[doc = "* **Group:** PathMetricMask"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_GLYPH_VERTICAL_BEARING_ADVANCE_BIT_NV: GLbitfield = 0x80;
#[doc = "`GL_GLYPH_VERTICAL_BEARING_X_BIT_NV: GLbitfield = 0x20`"]
#[doc = "* **Group:** PathMetricMask"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_GLYPH_VERTICAL_BEARING_X_BIT_NV: GLbitfield = 0x20;
#[doc = "`GL_GLYPH_VERTICAL_BEARING_Y_BIT_NV: GLbitfield = 0x40`"]
#[doc = "* **Group:** PathMetricMask"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_GLYPH_VERTICAL_BEARING_Y_BIT_NV: GLbitfield = 0x40;
#[doc = "`GL_GLYPH_WIDTH_BIT_NV: GLbitfield = 0x01`"]
#[doc = "* **Group:** PathMetricMask"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_GLYPH_WIDTH_BIT_NV: GLbitfield = 0x01;
#[doc = "`GL_GPU_DISJOINT_EXT: GLenum = 0x8FBB`"]
#[cfg(any(feature = "GL_EXT_disjoint_timer_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_disjoint_timer_query"))))]
pub const GL_GPU_DISJOINT_EXT: GLenum = 0x8FBB;
#[doc = "`GL_GPU_OPTIMIZED_QCOM: GLenum = 0x8FB2`"]
#[cfg(any(feature = "GL_QCOM_binning_control"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_binning_control"))))]
pub const GL_GPU_OPTIMIZED_QCOM: GLenum = 0x8FB2;
#[doc = "`GL_GREATER: GLenum = 0x0204`"]
#[doc = "* **Groups:** StencilFunction, IndexFunctionEXT, AlphaFunction, DepthFunction"]
pub const GL_GREATER: GLenum = 0x0204;
#[doc = "`GL_GREEN: GLenum = 0x1904`"]
#[doc = "* **Groups:** TextureSwizzle, PixelFormat"]
pub const GL_GREEN: GLenum = 0x1904;
#[doc = "`GL_GREEN_BITS: GLenum = 0x0D53`"]
#[doc = "* **Group:** GetPName"]
pub const GL_GREEN_BITS: GLenum = 0x0D53;
#[doc = "`GL_GREEN_NV: GLenum = 0x1904`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_GREEN_NV: GLenum = 0x1904;
#[doc = "`GL_GUILTY_CONTEXT_RESET: GLenum = 0x8253`"]
#[doc = "* **Group:** GraphicsResetStatus"]
pub const GL_GUILTY_CONTEXT_RESET: GLenum = 0x8253;
#[doc = "`GL_GUILTY_CONTEXT_RESET_EXT: GLenum = 0x8253`"]
#[cfg(any(feature = "GL_EXT_robustness"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_robustness"))))]
pub const GL_GUILTY_CONTEXT_RESET_EXT: GLenum = 0x8253;
#[doc = "`GL_GUILTY_CONTEXT_RESET_KHR: GLenum = 0x8253`"]
#[cfg(any(feature = "GL_KHR_robustness"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_robustness"))))]
pub const GL_GUILTY_CONTEXT_RESET_KHR: GLenum = 0x8253;
#[doc = "`GL_HALF_FLOAT: GLenum = 0x140B`"]
#[doc = "* **Groups:** VertexAttribPointerType, VertexAttribType"]
pub const GL_HALF_FLOAT: GLenum = 0x140B;
#[doc = "`GL_HALF_FLOAT_OES: GLenum = 0x8D61`"]
#[cfg(any(
    feature = "GL_OES_texture_half_float",
    feature = "GL_OES_vertex_half_float"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_OES_texture_half_float",
        feature = "GL_OES_vertex_half_float"
    )))
)]
pub const GL_HALF_FLOAT_OES: GLenum = 0x8D61;
#[doc = "`GL_HANDLE_TYPE_D3D11_IMAGE_EXT: GLenum = 0x958B`"]
#[doc = "* **Group:** ExternalHandleType"]
#[cfg(any(feature = "GL_EXT_memory_object_win32"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_memory_object_win32"))))]
pub const GL_HANDLE_TYPE_D3D11_IMAGE_EXT: GLenum = 0x958B;
#[doc = "`GL_HANDLE_TYPE_D3D11_IMAGE_KMT_EXT: GLenum = 0x958C`"]
#[doc = "* **Group:** ExternalHandleType"]
#[cfg(any(feature = "GL_EXT_memory_object_win32"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_memory_object_win32"))))]
pub const GL_HANDLE_TYPE_D3D11_IMAGE_KMT_EXT: GLenum = 0x958C;
#[doc = "`GL_HANDLE_TYPE_D3D12_FENCE_EXT: GLenum = 0x9594`"]
#[doc = "* **Group:** ExternalHandleType"]
#[cfg(any(feature = "GL_EXT_semaphore_win32"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_semaphore_win32"))))]
pub const GL_HANDLE_TYPE_D3D12_FENCE_EXT: GLenum = 0x9594;
#[doc = "`GL_HANDLE_TYPE_D3D12_RESOURCE_EXT: GLenum = 0x958A`"]
#[doc = "* **Group:** ExternalHandleType"]
#[cfg(any(feature = "GL_EXT_memory_object_win32"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_memory_object_win32"))))]
pub const GL_HANDLE_TYPE_D3D12_RESOURCE_EXT: GLenum = 0x958A;
#[doc = "`GL_HANDLE_TYPE_D3D12_TILEPOOL_EXT: GLenum = 0x9589`"]
#[doc = "* **Group:** ExternalHandleType"]
#[cfg(any(feature = "GL_EXT_memory_object_win32"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_memory_object_win32"))))]
pub const GL_HANDLE_TYPE_D3D12_TILEPOOL_EXT: GLenum = 0x9589;
#[doc = "`GL_HANDLE_TYPE_OPAQUE_FD_EXT: GLenum = 0x9586`"]
#[doc = "* **Group:** ExternalHandleType"]
#[cfg(any(feature = "GL_EXT_memory_object_fd", feature = "GL_EXT_semaphore_fd"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_memory_object_fd", feature = "GL_EXT_semaphore_fd")))
)]
pub const GL_HANDLE_TYPE_OPAQUE_FD_EXT: GLenum = 0x9586;
#[doc = "`GL_HANDLE_TYPE_OPAQUE_WIN32_EXT: GLenum = 0x9587`"]
#[doc = "* **Group:** ExternalHandleType"]
#[cfg(any(
    feature = "GL_EXT_memory_object_win32",
    feature = "GL_EXT_semaphore_win32"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_memory_object_win32",
        feature = "GL_EXT_semaphore_win32"
    )))
)]
pub const GL_HANDLE_TYPE_OPAQUE_WIN32_EXT: GLenum = 0x9587;
#[doc = "`GL_HANDLE_TYPE_OPAQUE_WIN32_KMT_EXT: GLenum = 0x9588`"]
#[doc = "* **Group:** ExternalHandleType"]
#[cfg(any(
    feature = "GL_EXT_memory_object_win32",
    feature = "GL_EXT_semaphore_win32"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_memory_object_win32",
        feature = "GL_EXT_semaphore_win32"
    )))
)]
pub const GL_HANDLE_TYPE_OPAQUE_WIN32_KMT_EXT: GLenum = 0x9588;
#[doc = "`GL_HARDLIGHT: GLenum = 0x929B`"]
pub const GL_HARDLIGHT: GLenum = 0x929B;
#[doc = "`GL_HARDLIGHT_KHR: GLenum = 0x929B`"]
#[cfg(any(feature = "GL_KHR_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_blend_equation_advanced"))))]
pub const GL_HARDLIGHT_KHR: GLenum = 0x929B;
#[doc = "`GL_HARDLIGHT_NV: GLenum = 0x929B`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_HARDLIGHT_NV: GLenum = 0x929B;
#[doc = "`GL_HARDMIX_NV: GLenum = 0x92A9`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_HARDMIX_NV: GLenum = 0x92A9;
#[doc = "`GL_HIGH_FLOAT: GLenum = 0x8DF2`"]
#[doc = "* **Group:** PrecisionType"]
pub const GL_HIGH_FLOAT: GLenum = 0x8DF2;
#[doc = "`GL_HIGH_INT: GLenum = 0x8DF5`"]
#[doc = "* **Group:** PrecisionType"]
pub const GL_HIGH_INT: GLenum = 0x8DF5;
#[doc = "`GL_HORIZONTAL_LINE_TO_NV: GLenum = 0x06`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_HORIZONTAL_LINE_TO_NV: GLenum = 0x06;
#[doc = "`GL_HSL_COLOR: GLenum = 0x92AF`"]
pub const GL_HSL_COLOR: GLenum = 0x92AF;
#[doc = "`GL_HSL_COLOR_KHR: GLenum = 0x92AF`"]
#[cfg(any(feature = "GL_KHR_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_blend_equation_advanced"))))]
pub const GL_HSL_COLOR_KHR: GLenum = 0x92AF;
#[doc = "`GL_HSL_COLOR_NV: GLenum = 0x92AF`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_HSL_COLOR_NV: GLenum = 0x92AF;
#[doc = "`GL_HSL_HUE: GLenum = 0x92AD`"]
pub const GL_HSL_HUE: GLenum = 0x92AD;
#[doc = "`GL_HSL_HUE_KHR: GLenum = 0x92AD`"]
#[cfg(any(feature = "GL_KHR_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_blend_equation_advanced"))))]
pub const GL_HSL_HUE_KHR: GLenum = 0x92AD;
#[doc = "`GL_HSL_HUE_NV: GLenum = 0x92AD`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_HSL_HUE_NV: GLenum = 0x92AD;
#[doc = "`GL_HSL_LUMINOSITY: GLenum = 0x92B0`"]
pub const GL_HSL_LUMINOSITY: GLenum = 0x92B0;
#[doc = "`GL_HSL_LUMINOSITY_KHR: GLenum = 0x92B0`"]
#[cfg(any(feature = "GL_KHR_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_blend_equation_advanced"))))]
pub const GL_HSL_LUMINOSITY_KHR: GLenum = 0x92B0;
#[doc = "`GL_HSL_LUMINOSITY_NV: GLenum = 0x92B0`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_HSL_LUMINOSITY_NV: GLenum = 0x92B0;
#[doc = "`GL_HSL_SATURATION: GLenum = 0x92AE`"]
pub const GL_HSL_SATURATION: GLenum = 0x92AE;
#[doc = "`GL_HSL_SATURATION_KHR: GLenum = 0x92AE`"]
#[cfg(any(feature = "GL_KHR_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_blend_equation_advanced"))))]
pub const GL_HSL_SATURATION_KHR: GLenum = 0x92AE;
#[doc = "`GL_HSL_SATURATION_NV: GLenum = 0x92AE`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_HSL_SATURATION_NV: GLenum = 0x92AE;
#[doc = "`GL_IMAGE_2D: GLenum = 0x904D`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType"]
pub const GL_IMAGE_2D: GLenum = 0x904D;
#[doc = "`GL_IMAGE_2D_ARRAY: GLenum = 0x9053`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType"]
pub const GL_IMAGE_2D_ARRAY: GLenum = 0x9053;
#[doc = "`GL_IMAGE_3D: GLenum = 0x904E`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType"]
pub const GL_IMAGE_3D: GLenum = 0x904E;
#[doc = "`GL_IMAGE_BINDING_ACCESS: GLenum = 0x8F3E`"]
pub const GL_IMAGE_BINDING_ACCESS: GLenum = 0x8F3E;
#[doc = "`GL_IMAGE_BINDING_FORMAT: GLenum = 0x906E`"]
pub const GL_IMAGE_BINDING_FORMAT: GLenum = 0x906E;
#[doc = "`GL_IMAGE_BINDING_LAYER: GLenum = 0x8F3D`"]
pub const GL_IMAGE_BINDING_LAYER: GLenum = 0x8F3D;
#[doc = "`GL_IMAGE_BINDING_LAYERED: GLenum = 0x8F3C`"]
pub const GL_IMAGE_BINDING_LAYERED: GLenum = 0x8F3C;
#[doc = "`GL_IMAGE_BINDING_LEVEL: GLenum = 0x8F3B`"]
pub const GL_IMAGE_BINDING_LEVEL: GLenum = 0x8F3B;
#[doc = "`GL_IMAGE_BINDING_NAME: GLenum = 0x8F3A`"]
pub const GL_IMAGE_BINDING_NAME: GLenum = 0x8F3A;
#[doc = "`GL_IMAGE_BUFFER: GLenum = 0x9051`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType"]
pub const GL_IMAGE_BUFFER: GLenum = 0x9051;
#[doc = "`GL_IMAGE_BUFFER_EXT: GLenum = 0x9051`"]
#[cfg(any(feature = "GL_EXT_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_buffer"))))]
pub const GL_IMAGE_BUFFER_EXT: GLenum = 0x9051;
#[doc = "`GL_IMAGE_BUFFER_OES: GLenum = 0x9051`"]
#[cfg(any(feature = "GL_OES_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_buffer"))))]
pub const GL_IMAGE_BUFFER_OES: GLenum = 0x9051;
#[doc = "`GL_IMAGE_CUBE: GLenum = 0x9050`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType"]
pub const GL_IMAGE_CUBE: GLenum = 0x9050;
#[doc = "`GL_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x9054`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType"]
pub const GL_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x9054;
#[doc = "`GL_IMAGE_CUBE_MAP_ARRAY_EXT: GLenum = 0x9054`"]
#[cfg(any(feature = "GL_EXT_texture_cube_map_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_cube_map_array"))))]
pub const GL_IMAGE_CUBE_MAP_ARRAY_EXT: GLenum = 0x9054;
#[doc = "`GL_IMAGE_CUBE_MAP_ARRAY_OES: GLenum = 0x9054`"]
#[cfg(any(feature = "GL_OES_texture_cube_map_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_cube_map_array"))))]
pub const GL_IMAGE_CUBE_MAP_ARRAY_OES: GLenum = 0x9054;
#[doc = "`GL_IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: GLenum = 0x90C9`"]
pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: GLenum = 0x90C9;
#[doc = "`GL_IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: GLenum = 0x90C8`"]
pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: GLenum = 0x90C8;
#[doc = "`GL_IMAGE_FORMAT_COMPATIBILITY_TYPE: GLenum = 0x90C7`"]
#[doc = "* **Group:** InternalFormatPName"]
pub const GL_IMAGE_FORMAT_COMPATIBILITY_TYPE: GLenum = 0x90C7;
#[doc = "`GL_IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = 0x8B9B`"]
#[doc = "* **Groups:** GetFramebufferParameter, GetPName"]
pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = 0x8B9B;
#[doc = "`GL_IMPLEMENTATION_COLOR_READ_TYPE: GLenum = 0x8B9A`"]
#[doc = "* **Groups:** GetFramebufferParameter, GetPName"]
pub const GL_IMPLEMENTATION_COLOR_READ_TYPE: GLenum = 0x8B9A;
#[doc = "`GL_INCLUSIVE_EXT: GLenum = 0x8F10`"]
#[cfg(any(feature = "GL_EXT_window_rectangles"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_window_rectangles"))))]
pub const GL_INCLUSIVE_EXT: GLenum = 0x8F10;
#[doc = "`GL_INCR: GLenum = 0x1E02`"]
#[doc = "* **Group:** StencilOp"]
pub const GL_INCR: GLenum = 0x1E02;
#[doc = "`GL_INCR_WRAP: GLenum = 0x8507`"]
#[doc = "* **Group:** StencilOp"]
pub const GL_INCR_WRAP: GLenum = 0x8507;
#[doc = "`GL_INFO_LOG_LENGTH: GLenum = 0x8B84`"]
#[doc = "* **Groups:** ProgramPropertyARB, ShaderParameterName, PipelineParameterName"]
pub const GL_INFO_LOG_LENGTH: GLenum = 0x8B84;
#[doc = "`GL_INNOCENT_CONTEXT_RESET: GLenum = 0x8254`"]
#[doc = "* **Group:** GraphicsResetStatus"]
pub const GL_INNOCENT_CONTEXT_RESET: GLenum = 0x8254;
#[doc = "`GL_INNOCENT_CONTEXT_RESET_EXT: GLenum = 0x8254`"]
#[cfg(any(feature = "GL_EXT_robustness"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_robustness"))))]
pub const GL_INNOCENT_CONTEXT_RESET_EXT: GLenum = 0x8254;
#[doc = "`GL_INNOCENT_CONTEXT_RESET_KHR: GLenum = 0x8254`"]
#[cfg(any(feature = "GL_KHR_robustness"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_robustness"))))]
pub const GL_INNOCENT_CONTEXT_RESET_KHR: GLenum = 0x8254;
#[doc = "`GL_INT: GLenum = 0x1404`"]
#[doc = "* **Groups:** VertexAttribIType, SecondaryColorPointerTypeIBM, WeightPointerTypeARB, TangentPointerTypeEXT, BinormalPointerTypeEXT, IndexPointerType, ListNameType, NormalPointerType, PixelType, TexCoordPointerType, VertexPointerType, VertexAttribType, AttributeType, UniformType, VertexAttribPointerType, GlslTypeToken"]
pub const GL_INT: GLenum = 0x1404;
#[doc = "`GL_INT16_NV: GLenum = 0x8FE4`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_INT16_NV: GLenum = 0x8FE4;
#[doc = "`GL_INT16_VEC2_NV: GLenum = 0x8FE5`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_INT16_VEC2_NV: GLenum = 0x8FE5;
#[doc = "`GL_INT16_VEC3_NV: GLenum = 0x8FE6`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_INT16_VEC3_NV: GLenum = 0x8FE6;
#[doc = "`GL_INT16_VEC4_NV: GLenum = 0x8FE7`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_INT16_VEC4_NV: GLenum = 0x8FE7;
#[doc = "`GL_INT64_NV: GLenum = 0x140E`"]
#[doc = "* **Groups:** VertexAttribPointerType, AttributeType"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_INT64_NV: GLenum = 0x140E;
#[doc = "`GL_INT64_VEC2_NV: GLenum = 0x8FE9`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_INT64_VEC2_NV: GLenum = 0x8FE9;
#[doc = "`GL_INT64_VEC3_NV: GLenum = 0x8FEA`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_INT64_VEC3_NV: GLenum = 0x8FEA;
#[doc = "`GL_INT64_VEC4_NV: GLenum = 0x8FEB`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_INT64_VEC4_NV: GLenum = 0x8FEB;
#[doc = "`GL_INT8_NV: GLenum = 0x8FE0`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_INT8_NV: GLenum = 0x8FE0;
#[doc = "`GL_INT8_VEC2_NV: GLenum = 0x8FE1`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_INT8_VEC2_NV: GLenum = 0x8FE1;
#[doc = "`GL_INT8_VEC3_NV: GLenum = 0x8FE2`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_INT8_VEC3_NV: GLenum = 0x8FE2;
#[doc = "`GL_INT8_VEC4_NV: GLenum = 0x8FE3`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_INT8_VEC4_NV: GLenum = 0x8FE3;
#[doc = "`GL_INTERLEAVED_ATTRIBS: GLenum = 0x8C8C`"]
#[doc = "* **Group:** TransformFeedbackBufferMode"]
pub const GL_INTERLEAVED_ATTRIBS: GLenum = 0x8C8C;
#[doc = "`GL_INT_10_10_10_2_OES: GLenum = 0x8DF7`"]
#[cfg(any(feature = "GL_OES_vertex_type_10_10_10_2"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_vertex_type_10_10_10_2"))))]
pub const GL_INT_10_10_10_2_OES: GLenum = 0x8DF7;
#[doc = "`GL_INT_2_10_10_10_REV: GLenum = 0x8D9F`"]
#[doc = "* **Groups:** VertexAttribPointerType, VertexAttribType"]
pub const GL_INT_2_10_10_10_REV: GLenum = 0x8D9F;
#[doc = "`GL_INT_IMAGE_2D: GLenum = 0x9058`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType"]
pub const GL_INT_IMAGE_2D: GLenum = 0x9058;
#[doc = "`GL_INT_IMAGE_2D_ARRAY: GLenum = 0x905E`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType"]
pub const GL_INT_IMAGE_2D_ARRAY: GLenum = 0x905E;
#[doc = "`GL_INT_IMAGE_3D: GLenum = 0x9059`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType"]
pub const GL_INT_IMAGE_3D: GLenum = 0x9059;
#[doc = "`GL_INT_IMAGE_BUFFER: GLenum = 0x905C`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType"]
pub const GL_INT_IMAGE_BUFFER: GLenum = 0x905C;
#[doc = "`GL_INT_IMAGE_BUFFER_EXT: GLenum = 0x905C`"]
#[cfg(any(feature = "GL_EXT_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_buffer"))))]
pub const GL_INT_IMAGE_BUFFER_EXT: GLenum = 0x905C;
#[doc = "`GL_INT_IMAGE_BUFFER_OES: GLenum = 0x905C`"]
#[cfg(any(feature = "GL_OES_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_buffer"))))]
pub const GL_INT_IMAGE_BUFFER_OES: GLenum = 0x905C;
#[doc = "`GL_INT_IMAGE_CUBE: GLenum = 0x905B`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType"]
pub const GL_INT_IMAGE_CUBE: GLenum = 0x905B;
#[doc = "`GL_INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x905F`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType"]
pub const GL_INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x905F;
#[doc = "`GL_INT_IMAGE_CUBE_MAP_ARRAY_EXT: GLenum = 0x905F`"]
#[cfg(any(feature = "GL_EXT_texture_cube_map_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_cube_map_array"))))]
pub const GL_INT_IMAGE_CUBE_MAP_ARRAY_EXT: GLenum = 0x905F;
#[doc = "`GL_INT_IMAGE_CUBE_MAP_ARRAY_OES: GLenum = 0x905F`"]
#[cfg(any(feature = "GL_OES_texture_cube_map_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_cube_map_array"))))]
pub const GL_INT_IMAGE_CUBE_MAP_ARRAY_OES: GLenum = 0x905F;
#[doc = "`GL_INT_SAMPLER_2D: GLenum = 0x8DCA`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_INT_SAMPLER_2D: GLenum = 0x8DCA;
#[doc = "`GL_INT_SAMPLER_2D_ARRAY: GLenum = 0x8DCF`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_INT_SAMPLER_2D_ARRAY: GLenum = 0x8DCF;
#[doc = "`GL_INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x9109`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x9109;
#[doc = "`GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910C`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910C;
#[doc = "`GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY_OES: GLenum = 0x910C`"]
#[cfg(any(feature = "GL_OES_texture_storage_multisample_2d_array"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_OES_texture_storage_multisample_2d_array")))
)]
pub const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY_OES: GLenum = 0x910C;
#[doc = "`GL_INT_SAMPLER_3D: GLenum = 0x8DCB`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_INT_SAMPLER_3D: GLenum = 0x8DCB;
#[doc = "`GL_INT_SAMPLER_BUFFER: GLenum = 0x8DD0`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_INT_SAMPLER_BUFFER: GLenum = 0x8DD0;
#[doc = "`GL_INT_SAMPLER_BUFFER_EXT: GLenum = 0x8DD0`"]
#[cfg(any(feature = "GL_EXT_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_buffer"))))]
pub const GL_INT_SAMPLER_BUFFER_EXT: GLenum = 0x8DD0;
#[doc = "`GL_INT_SAMPLER_BUFFER_OES: GLenum = 0x8DD0`"]
#[cfg(any(feature = "GL_OES_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_buffer"))))]
pub const GL_INT_SAMPLER_BUFFER_OES: GLenum = 0x8DD0;
#[doc = "`GL_INT_SAMPLER_CUBE: GLenum = 0x8DCC`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_INT_SAMPLER_CUBE: GLenum = 0x8DCC;
#[doc = "`GL_INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900E`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900E;
#[doc = "`GL_INT_SAMPLER_CUBE_MAP_ARRAY_EXT: GLenum = 0x900E`"]
#[cfg(any(feature = "GL_EXT_texture_cube_map_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_cube_map_array"))))]
pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY_EXT: GLenum = 0x900E;
#[doc = "`GL_INT_SAMPLER_CUBE_MAP_ARRAY_OES: GLenum = 0x900E`"]
#[cfg(any(feature = "GL_OES_texture_cube_map_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_cube_map_array"))))]
pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY_OES: GLenum = 0x900E;
#[doc = "`GL_INT_VEC2: GLenum = 0x8B53`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_INT_VEC2: GLenum = 0x8B53;
#[doc = "`GL_INT_VEC3: GLenum = 0x8B54`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_INT_VEC3: GLenum = 0x8B54;
#[doc = "`GL_INT_VEC4: GLenum = 0x8B55`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_INT_VEC4: GLenum = 0x8B55;
#[doc = "`GL_INVALID_ENUM: GLenum = 0x0500`"]
#[doc = "* **Group:** ErrorCode"]
pub const GL_INVALID_ENUM: GLenum = 0x0500;
#[doc = "`GL_INVALID_FRAMEBUFFER_OPERATION: GLenum = 0x0506`"]
#[doc = "* **Group:** ErrorCode"]
pub const GL_INVALID_FRAMEBUFFER_OPERATION: GLenum = 0x0506;
#[doc = "`GL_INVALID_INDEX: GLenum = 0xFFFFFFFF`"]
pub const GL_INVALID_INDEX: GLenum = 0xFFFFFFFF;
#[doc = "`GL_INVALID_OPERATION: GLenum = 0x0502`"]
#[doc = "* **Group:** ErrorCode"]
pub const GL_INVALID_OPERATION: GLenum = 0x0502;
#[doc = "`GL_INVALID_VALUE: GLenum = 0x0501`"]
#[doc = "* **Group:** ErrorCode"]
pub const GL_INVALID_VALUE: GLenum = 0x0501;
#[doc = "`GL_INVERT: GLenum = 0x150A`"]
#[doc = "* **Groups:** PathFillMode, LogicOp, StencilOp"]
pub const GL_INVERT: GLenum = 0x150A;
#[doc = "`GL_INVERT_OVG_NV: GLenum = 0x92B4`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_INVERT_OVG_NV: GLenum = 0x92B4;
#[doc = "`GL_INVERT_RGB_NV: GLenum = 0x92A3`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_INVERT_RGB_NV: GLenum = 0x92A3;
#[doc = "`GL_ISOLINES: GLenum = 0x8E7A`"]
pub const GL_ISOLINES: GLenum = 0x8E7A;
#[doc = "`GL_ISOLINES_EXT: GLenum = 0x8E7A`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_ISOLINES_EXT: GLenum = 0x8E7A;
#[doc = "`GL_ISOLINES_OES: GLenum = 0x8E7A`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_ISOLINES_OES: GLenum = 0x8E7A;
#[doc = "`GL_IS_PER_PATCH: GLenum = 0x92E7`"]
#[doc = "* **Group:** ProgramResourceProperty"]
pub const GL_IS_PER_PATCH: GLenum = 0x92E7;
#[doc = "`GL_IS_PER_PATCH_EXT: GLenum = 0x92E7`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_IS_PER_PATCH_EXT: GLenum = 0x92E7;
#[doc = "`GL_IS_PER_PATCH_OES: GLenum = 0x92E7`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_IS_PER_PATCH_OES: GLenum = 0x92E7;
#[doc = "`GL_IS_ROW_MAJOR: GLenum = 0x9300`"]
#[doc = "* **Group:** ProgramResourceProperty"]
pub const GL_IS_ROW_MAJOR: GLenum = 0x9300;
#[doc = "`GL_ITALIC_BIT_NV: GLbitfield = 0x02`"]
#[doc = "* **Group:** PathFontStyle"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_ITALIC_BIT_NV: GLbitfield = 0x02;
#[doc = "`GL_KEEP: GLenum = 0x1E00`"]
#[doc = "* **Group:** StencilOp"]
pub const GL_KEEP: GLenum = 0x1E00;
#[doc = "`GL_LARGE_CCW_ARC_TO_NV: GLenum = 0x16`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_LARGE_CCW_ARC_TO_NV: GLenum = 0x16;
#[doc = "`GL_LARGE_CW_ARC_TO_NV: GLenum = 0x18`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_LARGE_CW_ARC_TO_NV: GLenum = 0x18;
#[doc = "`GL_LAST_VERTEX_CONVENTION: GLenum = 0x8E4E`"]
#[doc = "* **Group:** VertexProvokingMode"]
pub const GL_LAST_VERTEX_CONVENTION: GLenum = 0x8E4E;
#[doc = "`GL_LAST_VERTEX_CONVENTION_EXT: GLenum = 0x8E4E`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_LAST_VERTEX_CONVENTION_EXT: GLenum = 0x8E4E;
#[doc = "`GL_LAST_VERTEX_CONVENTION_OES: GLenum = 0x8E4E`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_LAST_VERTEX_CONVENTION_OES: GLenum = 0x8E4E;
#[doc = "`GL_LAYER_PROVOKING_VERTEX: GLenum = 0x825E`"]
#[doc = "* **Group:** GetPName"]
pub const GL_LAYER_PROVOKING_VERTEX: GLenum = 0x825E;
#[doc = "`GL_LAYER_PROVOKING_VERTEX_EXT: GLenum = 0x825E`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_LAYER_PROVOKING_VERTEX_EXT: GLenum = 0x825E;
#[doc = "`GL_LAYER_PROVOKING_VERTEX_OES: GLenum = 0x825E`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_LAYER_PROVOKING_VERTEX_OES: GLenum = 0x825E;
#[doc = "`GL_LAYOUT_COLOR_ATTACHMENT_EXT: GLenum = 0x958E`"]
#[doc = "* **Group:** TextureLayout"]
#[cfg(any(feature = "GL_EXT_semaphore"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_semaphore"))))]
pub const GL_LAYOUT_COLOR_ATTACHMENT_EXT: GLenum = 0x958E;
#[doc = "`GL_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_EXT: GLenum = 0x9531`"]
#[doc = "* **Group:** TextureLayout"]
#[cfg(any(feature = "GL_EXT_semaphore"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_semaphore"))))]
pub const GL_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_EXT: GLenum = 0x9531;
#[doc = "`GL_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_EXT: GLenum = 0x9530`"]
#[doc = "* **Group:** TextureLayout"]
#[cfg(any(feature = "GL_EXT_semaphore"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_semaphore"))))]
pub const GL_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_EXT: GLenum = 0x9530;
#[doc = "`GL_LAYOUT_DEPTH_STENCIL_ATTACHMENT_EXT: GLenum = 0x958F`"]
#[doc = "* **Group:** TextureLayout"]
#[cfg(any(feature = "GL_EXT_semaphore"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_semaphore"))))]
pub const GL_LAYOUT_DEPTH_STENCIL_ATTACHMENT_EXT: GLenum = 0x958F;
#[doc = "`GL_LAYOUT_DEPTH_STENCIL_READ_ONLY_EXT: GLenum = 0x9590`"]
#[doc = "* **Group:** TextureLayout"]
#[cfg(any(feature = "GL_EXT_semaphore"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_semaphore"))))]
pub const GL_LAYOUT_DEPTH_STENCIL_READ_ONLY_EXT: GLenum = 0x9590;
#[doc = "`GL_LAYOUT_GENERAL_EXT: GLenum = 0x958D`"]
#[doc = "* **Group:** TextureLayout"]
#[cfg(any(feature = "GL_EXT_semaphore"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_semaphore"))))]
pub const GL_LAYOUT_GENERAL_EXT: GLenum = 0x958D;
#[doc = "`GL_LAYOUT_SHADER_READ_ONLY_EXT: GLenum = 0x9591`"]
#[doc = "* **Group:** TextureLayout"]
#[cfg(any(feature = "GL_EXT_semaphore"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_semaphore"))))]
pub const GL_LAYOUT_SHADER_READ_ONLY_EXT: GLenum = 0x9591;
#[doc = "`GL_LAYOUT_TRANSFER_DST_EXT: GLenum = 0x9593`"]
#[doc = "* **Group:** TextureLayout"]
#[cfg(any(feature = "GL_EXT_semaphore"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_semaphore"))))]
pub const GL_LAYOUT_TRANSFER_DST_EXT: GLenum = 0x9593;
#[doc = "`GL_LAYOUT_TRANSFER_SRC_EXT: GLenum = 0x9592`"]
#[doc = "* **Group:** TextureLayout"]
#[cfg(any(feature = "GL_EXT_semaphore"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_semaphore"))))]
pub const GL_LAYOUT_TRANSFER_SRC_EXT: GLenum = 0x9592;
#[doc = "`GL_LEQUAL: GLenum = 0x0203`"]
#[doc = "* **Groups:** StencilFunction, IndexFunctionEXT, AlphaFunction, DepthFunction"]
pub const GL_LEQUAL: GLenum = 0x0203;
#[doc = "`GL_LESS: GLenum = 0x0201`"]
#[doc = "* **Groups:** StencilFunction, IndexFunctionEXT, AlphaFunction, DepthFunction"]
pub const GL_LESS: GLenum = 0x0201;
#[doc = "`GL_LIGHTEN: GLenum = 0x9298`"]
pub const GL_LIGHTEN: GLenum = 0x9298;
#[doc = "`GL_LIGHTEN_KHR: GLenum = 0x9298`"]
#[cfg(any(feature = "GL_KHR_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_blend_equation_advanced"))))]
pub const GL_LIGHTEN_KHR: GLenum = 0x9298;
#[doc = "`GL_LIGHTEN_NV: GLenum = 0x9298`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_LIGHTEN_NV: GLenum = 0x9298;
#[doc = "`GL_LINEAR: GLenum = 0x2601`"]
#[doc = "* **Groups:** BlitFramebufferFilter, FogMode, TextureMagFilter, TextureMinFilter"]
pub const GL_LINEAR: GLenum = 0x2601;
#[doc = "`GL_LINEARBURN_NV: GLenum = 0x92A5`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_LINEARBURN_NV: GLenum = 0x92A5;
#[doc = "`GL_LINEARDODGE_NV: GLenum = 0x92A4`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_LINEARDODGE_NV: GLenum = 0x92A4;
#[doc = "`GL_LINEARLIGHT_NV: GLenum = 0x92A7`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_LINEARLIGHT_NV: GLenum = 0x92A7;
#[doc = "`GL_LINEAR_MIPMAP_LINEAR: GLenum = 0x2703`"]
#[doc = "* **Groups:** TextureWrapMode, TextureMinFilter"]
pub const GL_LINEAR_MIPMAP_LINEAR: GLenum = 0x2703;
#[doc = "`GL_LINEAR_MIPMAP_NEAREST: GLenum = 0x2701`"]
#[doc = "* **Group:** TextureMinFilter"]
pub const GL_LINEAR_MIPMAP_NEAREST: GLenum = 0x2701;
#[doc = "`GL_LINEAR_TILING_EXT: GLenum = 0x9585`"]
#[cfg(any(feature = "GL_EXT_memory_object"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_memory_object"))))]
pub const GL_LINEAR_TILING_EXT: GLenum = 0x9585;
#[doc = "`GL_LINES: GLenum = 0x0001`"]
#[doc = "* **Group:** PrimitiveType"]
pub const GL_LINES: GLenum = 0x0001;
#[doc = "`GL_LINES_ADJACENCY: GLenum = 0x000A`"]
#[doc = "* **Group:** PrimitiveType"]
pub const GL_LINES_ADJACENCY: GLenum = 0x000A;
#[doc = "`GL_LINES_ADJACENCY_EXT: GLenum = 0x000A`"]
#[doc = "* **Group:** PrimitiveType"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_LINES_ADJACENCY_EXT: GLenum = 0x000A;
#[doc = "`GL_LINES_ADJACENCY_OES: GLenum = 0x000A`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_LINES_ADJACENCY_OES: GLenum = 0x000A;
#[doc = "`GL_LINE_LOOP: GLenum = 0x0002`"]
#[doc = "* **Group:** PrimitiveType"]
pub const GL_LINE_LOOP: GLenum = 0x0002;
#[doc = "`GL_LINE_NV: GLenum = 0x1B01`"]
#[cfg(any(feature = "GL_NV_polygon_mode"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_polygon_mode"))))]
pub const GL_LINE_NV: GLenum = 0x1B01;
#[doc = "`GL_LINE_STRIP: GLenum = 0x0003`"]
#[doc = "* **Group:** PrimitiveType"]
pub const GL_LINE_STRIP: GLenum = 0x0003;
#[doc = "`GL_LINE_STRIP_ADJACENCY: GLenum = 0x000B`"]
#[doc = "* **Group:** PrimitiveType"]
pub const GL_LINE_STRIP_ADJACENCY: GLenum = 0x000B;
#[doc = "`GL_LINE_STRIP_ADJACENCY_EXT: GLenum = 0x000B`"]
#[doc = "* **Group:** PrimitiveType"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_LINE_STRIP_ADJACENCY_EXT: GLenum = 0x000B;
#[doc = "`GL_LINE_STRIP_ADJACENCY_OES: GLenum = 0x000B`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_LINE_STRIP_ADJACENCY_OES: GLenum = 0x000B;
#[doc = "`GL_LINE_TO_NV: GLenum = 0x04`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_LINE_TO_NV: GLenum = 0x04;
#[doc = "`GL_LINE_WIDTH: GLenum = 0x0B21`"]
#[doc = "* **Group:** GetPName"]
pub const GL_LINE_WIDTH: GLenum = 0x0B21;
#[doc = "`GL_LINK_STATUS: GLenum = 0x8B82`"]
#[doc = "* **Group:** ProgramPropertyARB"]
pub const GL_LINK_STATUS: GLenum = 0x8B82;
#[doc = "`GL_LOCATION: GLenum = 0x930E`"]
#[doc = "* **Group:** ProgramResourceProperty"]
pub const GL_LOCATION: GLenum = 0x930E;
#[doc = "`GL_LOCATION_INDEX_EXT: GLenum = 0x930F`"]
#[cfg(any(feature = "GL_EXT_blend_func_extended"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_blend_func_extended"))))]
pub const GL_LOCATION_INDEX_EXT: GLenum = 0x930F;
#[doc = "`GL_LOSE_CONTEXT_ON_RESET: GLenum = 0x8252`"]
pub const GL_LOSE_CONTEXT_ON_RESET: GLenum = 0x8252;
#[doc = "`GL_LOSE_CONTEXT_ON_RESET_EXT: GLenum = 0x8252`"]
#[cfg(any(feature = "GL_EXT_robustness"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_robustness"))))]
pub const GL_LOSE_CONTEXT_ON_RESET_EXT: GLenum = 0x8252;
#[doc = "`GL_LOSE_CONTEXT_ON_RESET_KHR: GLenum = 0x8252`"]
#[cfg(any(feature = "GL_KHR_robustness"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_robustness"))))]
pub const GL_LOSE_CONTEXT_ON_RESET_KHR: GLenum = 0x8252;
#[doc = "`GL_LOWER_LEFT_EXT: GLenum = 0x8CA1`"]
#[doc = "* **Alias Of:** `GL_LOWER_LEFT`"]
#[cfg(any(feature = "GL_EXT_clip_control"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_clip_control"))))]
pub const GL_LOWER_LEFT_EXT: GLenum = 0x8CA1;
#[doc = "`GL_LOW_FLOAT: GLenum = 0x8DF0`"]
#[doc = "* **Group:** PrecisionType"]
pub const GL_LOW_FLOAT: GLenum = 0x8DF0;
#[doc = "`GL_LOW_INT: GLenum = 0x8DF3`"]
#[doc = "* **Group:** PrecisionType"]
pub const GL_LOW_INT: GLenum = 0x8DF3;
#[doc = "`GL_LUID_SIZE_EXT: GLenum = 8`"]
#[cfg(any(
    feature = "GL_EXT_memory_object_win32",
    feature = "GL_EXT_semaphore_win32"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_memory_object_win32",
        feature = "GL_EXT_semaphore_win32"
    )))
)]
pub const GL_LUID_SIZE_EXT: GLenum = 8;
#[doc = "`GL_LUMINANCE: GLenum = 0x1909`"]
#[doc = "* **Groups:** PixelTexGenMode, PathColorFormat, PixelFormat"]
pub const GL_LUMINANCE: GLenum = 0x1909;
#[doc = "`GL_LUMINANCE16F_EXT: GLenum = 0x881E`"]
#[cfg(any(feature = "GL_EXT_texture_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_storage"))))]
pub const GL_LUMINANCE16F_EXT: GLenum = 0x881E;
#[doc = "`GL_LUMINANCE32F_EXT: GLenum = 0x8818`"]
#[cfg(any(feature = "GL_EXT_texture_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_storage"))))]
pub const GL_LUMINANCE32F_EXT: GLenum = 0x8818;
#[doc = "`GL_LUMINANCE4_ALPHA4_OES: GLenum = 0x8043`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_required_internalformat"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_required_internalformat"))))]
pub const GL_LUMINANCE4_ALPHA4_OES: GLenum = 0x8043;
#[doc = "`GL_LUMINANCE8_ALPHA8_EXT: GLenum = 0x8045`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_storage"))))]
pub const GL_LUMINANCE8_ALPHA8_EXT: GLenum = 0x8045;
#[doc = "`GL_LUMINANCE8_ALPHA8_OES: GLenum = 0x8045`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_required_internalformat"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_required_internalformat"))))]
pub const GL_LUMINANCE8_ALPHA8_OES: GLenum = 0x8045;
#[doc = "`GL_LUMINANCE8_EXT: GLenum = 0x8040`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_storage"))))]
pub const GL_LUMINANCE8_EXT: GLenum = 0x8040;
#[doc = "`GL_LUMINANCE8_OES: GLenum = 0x8040`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_required_internalformat"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_required_internalformat"))))]
pub const GL_LUMINANCE8_OES: GLenum = 0x8040;
#[doc = "`GL_LUMINANCE_ALPHA: GLenum = 0x190A`"]
#[doc = "* **Groups:** PixelTexGenMode, PathColorFormat, PixelFormat"]
pub const GL_LUMINANCE_ALPHA: GLenum = 0x190A;
#[doc = "`GL_LUMINANCE_ALPHA16F_EXT: GLenum = 0x881F`"]
#[cfg(any(feature = "GL_EXT_texture_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_storage"))))]
pub const GL_LUMINANCE_ALPHA16F_EXT: GLenum = 0x881F;
#[doc = "`GL_LUMINANCE_ALPHA32F_EXT: GLenum = 0x8819`"]
#[cfg(any(feature = "GL_EXT_texture_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_storage"))))]
pub const GL_LUMINANCE_ALPHA32F_EXT: GLenum = 0x8819;
#[doc = "`GL_MAJOR_VERSION: GLenum = 0x821B`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAJOR_VERSION: GLenum = 0x821B;
#[doc = "`GL_MALI_PROGRAM_BINARY_ARM: GLenum = 0x8F61`"]
#[cfg(any(feature = "GL_ARM_mali_program_binary"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_ARM_mali_program_binary"))))]
pub const GL_MALI_PROGRAM_BINARY_ARM: GLenum = 0x8F61;
#[doc = "`GL_MALI_SHADER_BINARY_ARM: GLenum = 0x8F60`"]
#[doc = "* **Group:** ShaderBinaryFormat"]
#[cfg(any(feature = "GL_ARM_mali_shader_binary"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_ARM_mali_shader_binary"))))]
pub const GL_MALI_SHADER_BINARY_ARM: GLenum = 0x8F60;
#[doc = "`GL_MAP_COHERENT_BIT_EXT: GLbitfield = 0x0080`"]
#[doc = "* **Groups:** MapBufferAccessMask, BufferStorageMask"]
#[cfg(any(feature = "GL_EXT_buffer_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_buffer_storage"))))]
pub const GL_MAP_COHERENT_BIT_EXT: GLbitfield = 0x0080;
#[doc = "`GL_MAP_FLUSH_EXPLICIT_BIT: GLbitfield = 0x0010`"]
#[doc = "* **Group:** MapBufferAccessMask"]
pub const GL_MAP_FLUSH_EXPLICIT_BIT: GLbitfield = 0x0010;
#[doc = "`GL_MAP_FLUSH_EXPLICIT_BIT_EXT: GLbitfield = 0x0010`"]
#[doc = "* **Group:** MapBufferAccessMask"]
#[cfg(any(feature = "GL_EXT_map_buffer_range"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_map_buffer_range"))))]
pub const GL_MAP_FLUSH_EXPLICIT_BIT_EXT: GLbitfield = 0x0010;
#[doc = "`GL_MAP_INVALIDATE_BUFFER_BIT: GLbitfield = 0x0008`"]
#[doc = "* **Group:** MapBufferAccessMask"]
pub const GL_MAP_INVALIDATE_BUFFER_BIT: GLbitfield = 0x0008;
#[doc = "`GL_MAP_INVALIDATE_BUFFER_BIT_EXT: GLbitfield = 0x0008`"]
#[doc = "* **Group:** MapBufferAccessMask"]
#[cfg(any(feature = "GL_EXT_map_buffer_range"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_map_buffer_range"))))]
pub const GL_MAP_INVALIDATE_BUFFER_BIT_EXT: GLbitfield = 0x0008;
#[doc = "`GL_MAP_INVALIDATE_RANGE_BIT: GLbitfield = 0x0004`"]
#[doc = "* **Group:** MapBufferAccessMask"]
pub const GL_MAP_INVALIDATE_RANGE_BIT: GLbitfield = 0x0004;
#[doc = "`GL_MAP_INVALIDATE_RANGE_BIT_EXT: GLbitfield = 0x0004`"]
#[doc = "* **Group:** MapBufferAccessMask"]
#[cfg(any(feature = "GL_EXT_map_buffer_range"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_map_buffer_range"))))]
pub const GL_MAP_INVALIDATE_RANGE_BIT_EXT: GLbitfield = 0x0004;
#[doc = "`GL_MAP_PERSISTENT_BIT_EXT: GLbitfield = 0x0040`"]
#[doc = "* **Groups:** MapBufferAccessMask, BufferStorageMask"]
#[cfg(any(feature = "GL_EXT_buffer_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_buffer_storage"))))]
pub const GL_MAP_PERSISTENT_BIT_EXT: GLbitfield = 0x0040;
#[doc = "`GL_MAP_READ_BIT: GLbitfield = 0x0001`"]
#[doc = "* **Groups:** MapBufferAccessMask, BufferStorageMask"]
pub const GL_MAP_READ_BIT: GLbitfield = 0x0001;
#[doc = "`GL_MAP_READ_BIT_EXT: GLbitfield = 0x0001`"]
#[doc = "* **Groups:** MapBufferAccessMask, BufferStorageMask"]
#[cfg(any(feature = "GL_EXT_map_buffer_range"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_map_buffer_range"))))]
pub const GL_MAP_READ_BIT_EXT: GLbitfield = 0x0001;
#[doc = "`GL_MAP_UNSYNCHRONIZED_BIT: GLbitfield = 0x0020`"]
#[doc = "* **Group:** MapBufferAccessMask"]
pub const GL_MAP_UNSYNCHRONIZED_BIT: GLbitfield = 0x0020;
#[doc = "`GL_MAP_UNSYNCHRONIZED_BIT_EXT: GLbitfield = 0x0020`"]
#[doc = "* **Group:** MapBufferAccessMask"]
#[cfg(any(feature = "GL_EXT_map_buffer_range"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_map_buffer_range"))))]
pub const GL_MAP_UNSYNCHRONIZED_BIT_EXT: GLbitfield = 0x0020;
#[doc = "`GL_MAP_WRITE_BIT: GLbitfield = 0x0002`"]
#[doc = "* **Groups:** MapBufferAccessMask, BufferStorageMask"]
pub const GL_MAP_WRITE_BIT: GLbitfield = 0x0002;
#[doc = "`GL_MAP_WRITE_BIT_EXT: GLbitfield = 0x0002`"]
#[doc = "* **Groups:** MapBufferAccessMask, BufferStorageMask"]
#[cfg(any(feature = "GL_EXT_map_buffer_range"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_map_buffer_range"))))]
pub const GL_MAP_WRITE_BIT_EXT: GLbitfield = 0x0002;
#[doc = "`GL_MATRIX_STRIDE: GLenum = 0x92FF`"]
#[doc = "* **Group:** ProgramResourceProperty"]
pub const GL_MATRIX_STRIDE: GLenum = 0x92FF;
#[doc = "`GL_MAX: GLenum = 0x8008`"]
#[doc = "* **Group:** BlendEquationModeEXT"]
pub const GL_MAX: GLenum = 0x8008;
#[doc = "`GL_MAX_3D_TEXTURE_SIZE: GLenum = 0x8073`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_3D_TEXTURE_SIZE: GLenum = 0x8073;
#[doc = "`GL_MAX_3D_TEXTURE_SIZE_OES: GLenum = 0x8073`"]
#[cfg(any(feature = "GL_OES_texture_3D"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_3D"))))]
pub const GL_MAX_3D_TEXTURE_SIZE_OES: GLenum = 0x8073;
#[doc = "`GL_MAX_ARRAY_TEXTURE_LAYERS: GLenum = 0x88FF`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_ARRAY_TEXTURE_LAYERS: GLenum = 0x88FF;
#[doc = "`GL_MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: GLenum = 0x92DC`"]
pub const GL_MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: GLenum = 0x92DC;
#[doc = "`GL_MAX_ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92D8`"]
pub const GL_MAX_ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92D8;
#[doc = "`GL_MAX_CLIP_DISTANCES_APPLE: GLenum = 0x0D32`"]
#[cfg(any(feature = "GL_APPLE_clip_distance"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_clip_distance"))))]
pub const GL_MAX_CLIP_DISTANCES_APPLE: GLenum = 0x0D32;
#[doc = "`GL_MAX_CLIP_DISTANCES_EXT: GLenum = 0x0D32`"]
#[doc = "* **Alias Of:** `GL_MAX_CLIP_PLANES`"]
#[cfg(any(feature = "GL_EXT_clip_cull_distance"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_clip_cull_distance"))))]
pub const GL_MAX_CLIP_DISTANCES_EXT: GLenum = 0x0D32;
#[doc = "`GL_MAX_COARSE_FRAGMENT_SAMPLES_NV: GLenum = 0x955F`"]
#[cfg(any(feature = "GL_NV_shading_rate_image"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shading_rate_image"))))]
pub const GL_MAX_COARSE_FRAGMENT_SAMPLES_NV: GLenum = 0x955F;
#[doc = "`GL_MAX_COLOR_ATTACHMENTS: GLenum = 0x8CDF`"]
pub const GL_MAX_COLOR_ATTACHMENTS: GLenum = 0x8CDF;
#[doc = "`GL_MAX_COLOR_ATTACHMENTS_EXT: GLenum = 0x8CDF`"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_MAX_COLOR_ATTACHMENTS_EXT: GLenum = 0x8CDF;
#[doc = "`GL_MAX_COLOR_ATTACHMENTS_NV: GLenum = 0x8CDF`"]
#[cfg(any(feature = "GL_NV_fbo_color_attachments"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_fbo_color_attachments"))))]
pub const GL_MAX_COLOR_ATTACHMENTS_NV: GLenum = 0x8CDF;
#[doc = "`GL_MAX_COLOR_FRAMEBUFFER_SAMPLES_AMD: GLenum = 0x91B3`"]
#[cfg(any(feature = "GL_AMD_framebuffer_multisample_advanced"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_AMD_framebuffer_multisample_advanced")))
)]
pub const GL_MAX_COLOR_FRAMEBUFFER_SAMPLES_AMD: GLenum = 0x91B3;
#[doc = "`GL_MAX_COLOR_FRAMEBUFFER_STORAGE_SAMPLES_AMD: GLenum = 0x91B4`"]
#[cfg(any(feature = "GL_AMD_framebuffer_multisample_advanced"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_AMD_framebuffer_multisample_advanced")))
)]
pub const GL_MAX_COLOR_FRAMEBUFFER_STORAGE_SAMPLES_AMD: GLenum = 0x91B4;
#[doc = "`GL_MAX_COLOR_TEXTURE_SAMPLES: GLenum = 0x910E`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_COLOR_TEXTURE_SAMPLES: GLenum = 0x910E;
#[doc = "`GL_MAX_COMBINED_ATOMIC_COUNTERS: GLenum = 0x92D7`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_COMBINED_ATOMIC_COUNTERS: GLenum = 0x92D7;
#[doc = "`GL_MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D1`"]
pub const GL_MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D1;
#[doc = "`GL_MAX_COMBINED_CLIP_AND_CULL_DISTANCES_EXT: GLenum = 0x82FA`"]
#[doc = "* **Alias Of:** `GL_MAX_COMBINED_CLIP_AND_CULL_DISTANCES`"]
#[cfg(any(feature = "GL_EXT_clip_cull_distance"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_clip_cull_distance"))))]
pub const GL_MAX_COMBINED_CLIP_AND_CULL_DISTANCES_EXT: GLenum = 0x82FA;
#[doc = "`GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8266`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8266;
#[doc = "`GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8A33`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8A33;
#[doc = "`GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8A32`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8A32;
#[doc = "`GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS_EXT: GLenum = 0x8A32`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS_EXT: GLenum = 0x8A32;
#[doc = "`GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS_OES: GLenum = 0x8A32`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS_OES: GLenum = 0x8A32;
#[doc = "`GL_MAX_COMBINED_IMAGE_UNIFORMS: GLenum = 0x90CF`"]
pub const GL_MAX_COMBINED_IMAGE_UNIFORMS: GLenum = 0x90CF;
#[doc = "`GL_MAX_COMBINED_MESH_UNIFORM_COMPONENTS_NV: GLenum = 0x8E67`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_COMBINED_MESH_UNIFORM_COMPONENTS_NV: GLenum = 0x8E67;
#[doc = "`GL_MAX_COMBINED_SHADER_OUTPUT_RESOURCES: GLenum = 0x8F39`"]
#[doc = "* **Alias Of:** `GL_MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS`"]
pub const GL_MAX_COMBINED_SHADER_OUTPUT_RESOURCES: GLenum = 0x8F39;
#[doc = "`GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS: GLenum = 0x90DC`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS: GLenum = 0x90DC;
#[doc = "`GL_MAX_COMBINED_TASK_UNIFORM_COMPONENTS_NV: GLenum = 0x8E6F`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_COMBINED_TASK_UNIFORM_COMPONENTS_NV: GLenum = 0x8E6F;
#[doc = "`GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E1E`"]
pub const GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E1E;
#[doc = "`GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS_EXT: GLenum = 0x8E1E`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS_EXT: GLenum = 0x8E1E;
#[doc = "`GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS_OES: GLenum = 0x8E1E`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS_OES: GLenum = 0x8E1E;
#[doc = "`GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E1F`"]
pub const GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E1F;
#[doc = "`GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS_EXT: GLenum = 0x8E1F`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS_EXT: GLenum = 0x8E1F;
#[doc = "`GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS_OES: GLenum = 0x8E1F`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS_OES: GLenum = 0x8E1F;
#[doc = "`GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4D`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4D;
#[doc = "`GL_MAX_COMBINED_UNIFORM_BLOCKS: GLenum = 0x8A2E`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_COMBINED_UNIFORM_BLOCKS: GLenum = 0x8A2E;
#[doc = "`GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8A31`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8A31;
#[doc = "`GL_MAX_COMPUTE_ATOMIC_COUNTERS: GLenum = 0x8265`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_COMPUTE_ATOMIC_COUNTERS: GLenum = 0x8265;
#[doc = "`GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x8264`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x8264;
#[doc = "`GL_MAX_COMPUTE_IMAGE_UNIFORMS: GLenum = 0x91BD`"]
pub const GL_MAX_COMPUTE_IMAGE_UNIFORMS: GLenum = 0x91BD;
#[doc = "`GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS: GLenum = 0x90DB`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS: GLenum = 0x90DB;
#[doc = "`GL_MAX_COMPUTE_SHARED_MEMORY_SIZE: GLenum = 0x8262`"]
pub const GL_MAX_COMPUTE_SHARED_MEMORY_SIZE: GLenum = 0x8262;
#[doc = "`GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS: GLenum = 0x91BC`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS: GLenum = 0x91BC;
#[doc = "`GL_MAX_COMPUTE_UNIFORM_BLOCKS: GLenum = 0x91BB`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_COMPUTE_UNIFORM_BLOCKS: GLenum = 0x91BB;
#[doc = "`GL_MAX_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8263`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8263;
#[doc = "`GL_MAX_COMPUTE_WORK_GROUP_COUNT: GLenum = 0x91BE`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_COMPUTE_WORK_GROUP_COUNT: GLenum = 0x91BE;
#[doc = "`GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS: GLenum = 0x90EB`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS: GLenum = 0x90EB;
#[doc = "`GL_MAX_COMPUTE_WORK_GROUP_SIZE: GLenum = 0x91BF`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_COMPUTE_WORK_GROUP_SIZE: GLenum = 0x91BF;
#[doc = "`GL_MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 0x851C`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 0x851C;
#[doc = "`GL_MAX_CULL_DISTANCES_EXT: GLenum = 0x82F9`"]
#[doc = "* **Alias Of:** `GL_MAX_CULL_DISTANCES`"]
#[cfg(any(feature = "GL_EXT_clip_cull_distance"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_clip_cull_distance"))))]
pub const GL_MAX_CULL_DISTANCES_EXT: GLenum = 0x82F9;
#[doc = "`GL_MAX_DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826C`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826C;
#[doc = "`GL_MAX_DEBUG_GROUP_STACK_DEPTH_KHR: GLenum = 0x826C`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_MAX_DEBUG_GROUP_STACK_DEPTH_KHR: GLenum = 0x826C;
#[doc = "`GL_MAX_DEBUG_LOGGED_MESSAGES: GLenum = 0x9144`"]
pub const GL_MAX_DEBUG_LOGGED_MESSAGES: GLenum = 0x9144;
#[doc = "`GL_MAX_DEBUG_LOGGED_MESSAGES_KHR: GLenum = 0x9144`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_MAX_DEBUG_LOGGED_MESSAGES_KHR: GLenum = 0x9144;
#[doc = "`GL_MAX_DEBUG_MESSAGE_LENGTH: GLenum = 0x9143`"]
pub const GL_MAX_DEBUG_MESSAGE_LENGTH: GLenum = 0x9143;
#[doc = "`GL_MAX_DEBUG_MESSAGE_LENGTH_KHR: GLenum = 0x9143`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_MAX_DEBUG_MESSAGE_LENGTH_KHR: GLenum = 0x9143;
#[doc = "`GL_MAX_DEPTH_STENCIL_FRAMEBUFFER_SAMPLES_AMD: GLenum = 0x91B5`"]
#[cfg(any(feature = "GL_AMD_framebuffer_multisample_advanced"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_AMD_framebuffer_multisample_advanced")))
)]
pub const GL_MAX_DEPTH_STENCIL_FRAMEBUFFER_SAMPLES_AMD: GLenum = 0x91B5;
#[doc = "`GL_MAX_DEPTH_TEXTURE_SAMPLES: GLenum = 0x910F`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_DEPTH_TEXTURE_SAMPLES: GLenum = 0x910F;
#[doc = "`GL_MAX_DETACHED_BUFFERS_NV: GLenum = 0x95AD`"]
#[cfg(any(feature = "GL_NV_memory_attachment"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_memory_attachment"))))]
pub const GL_MAX_DETACHED_BUFFERS_NV: GLenum = 0x95AD;
#[doc = "`GL_MAX_DETACHED_TEXTURES_NV: GLenum = 0x95AC`"]
#[cfg(any(feature = "GL_NV_memory_attachment"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_memory_attachment"))))]
pub const GL_MAX_DETACHED_TEXTURES_NV: GLenum = 0x95AC;
#[doc = "`GL_MAX_DRAW_BUFFERS: GLenum = 0x8824`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_DRAW_BUFFERS: GLenum = 0x8824;
#[doc = "`GL_MAX_DRAW_BUFFERS_EXT: GLenum = 0x8824`"]
#[cfg(any(feature = "GL_EXT_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_draw_buffers"))))]
pub const GL_MAX_DRAW_BUFFERS_EXT: GLenum = 0x8824;
#[doc = "`GL_MAX_DRAW_BUFFERS_NV: GLenum = 0x8824`"]
#[cfg(any(feature = "GL_NV_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_draw_buffers"))))]
pub const GL_MAX_DRAW_BUFFERS_NV: GLenum = 0x8824;
#[doc = "`GL_MAX_DRAW_MESH_TASKS_COUNT_NV: GLenum = 0x953D`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_DRAW_MESH_TASKS_COUNT_NV: GLenum = 0x953D;
#[doc = "`GL_MAX_DUAL_SOURCE_DRAW_BUFFERS_EXT: GLenum = 0x88FC`"]
#[cfg(any(feature = "GL_EXT_blend_func_extended"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_blend_func_extended"))))]
pub const GL_MAX_DUAL_SOURCE_DRAW_BUFFERS_EXT: GLenum = 0x88FC;
#[doc = "`GL_MAX_ELEMENTS_INDICES: GLenum = 0x80E9`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_ELEMENTS_INDICES: GLenum = 0x80E9;
#[doc = "`GL_MAX_ELEMENTS_VERTICES: GLenum = 0x80E8`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_ELEMENTS_VERTICES: GLenum = 0x80E8;
#[doc = "`GL_MAX_ELEMENT_INDEX: GLenum = 0x8D6B`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_ELEMENT_INDEX: GLenum = 0x8D6B;
#[doc = "`GL_MAX_EXT: GLenum = 0x8008`"]
#[doc = "* **Group:** BlendEquationModeEXT"]
#[cfg(any(feature = "GL_EXT_blend_minmax"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_blend_minmax"))))]
pub const GL_MAX_EXT: GLenum = 0x8008;
#[doc = "`GL_MAX_FRAGMENT_ATOMIC_COUNTERS: GLenum = 0x92D6`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_FRAGMENT_ATOMIC_COUNTERS: GLenum = 0x92D6;
#[doc = "`GL_MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D0`"]
pub const GL_MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D0;
#[doc = "`GL_MAX_FRAGMENT_IMAGE_UNIFORMS: GLenum = 0x90CE`"]
pub const GL_MAX_FRAGMENT_IMAGE_UNIFORMS: GLenum = 0x90CE;
#[doc = "`GL_MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = 0x9125`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = 0x9125;
#[doc = "`GL_MAX_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 0x8E5C`"]
pub const GL_MAX_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 0x8E5C;
#[doc = "`GL_MAX_FRAGMENT_INTERPOLATION_OFFSET_OES: GLenum = 0x8E5C`"]
#[cfg(any(feature = "GL_OES_shader_multisample_interpolation"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_OES_shader_multisample_interpolation")))
)]
pub const GL_MAX_FRAGMENT_INTERPOLATION_OFFSET_OES: GLenum = 0x8E5C;
#[doc = "`GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: GLenum = 0x90DA`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: GLenum = 0x90DA;
#[doc = "`GL_MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = 0x8A2D`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = 0x8A2D;
#[doc = "`GL_MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8B49`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8B49;
#[doc = "`GL_MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = 0x8DFD`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = 0x8DFD;
#[doc = "`GL_MAX_FRAMEBUFFER_HEIGHT: GLenum = 0x9316`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_FRAMEBUFFER_HEIGHT: GLenum = 0x9316;
#[doc = "`GL_MAX_FRAMEBUFFER_LAYERS: GLenum = 0x9317`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_FRAMEBUFFER_LAYERS: GLenum = 0x9317;
#[doc = "`GL_MAX_FRAMEBUFFER_LAYERS_EXT: GLenum = 0x9317`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_MAX_FRAMEBUFFER_LAYERS_EXT: GLenum = 0x9317;
#[doc = "`GL_MAX_FRAMEBUFFER_LAYERS_OES: GLenum = 0x9317`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_MAX_FRAMEBUFFER_LAYERS_OES: GLenum = 0x9317;
#[doc = "`GL_MAX_FRAMEBUFFER_SAMPLES: GLenum = 0x9318`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_FRAMEBUFFER_SAMPLES: GLenum = 0x9318;
#[doc = "`GL_MAX_FRAMEBUFFER_WIDTH: GLenum = 0x9315`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_FRAMEBUFFER_WIDTH: GLenum = 0x9315;
#[doc = "`GL_MAX_GEOMETRY_ATOMIC_COUNTERS: GLenum = 0x92D5`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTERS: GLenum = 0x92D5;
#[doc = "`GL_MAX_GEOMETRY_ATOMIC_COUNTERS_EXT: GLenum = 0x92D5`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTERS_EXT: GLenum = 0x92D5;
#[doc = "`GL_MAX_GEOMETRY_ATOMIC_COUNTERS_OES: GLenum = 0x92D5`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTERS_OES: GLenum = 0x92D5;
#[doc = "`GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CF`"]
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CF;
#[doc = "`GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS_EXT: GLenum = 0x92CF`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS_EXT: GLenum = 0x92CF;
#[doc = "`GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS_OES: GLenum = 0x92CF`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS_OES: GLenum = 0x92CF;
#[doc = "`GL_MAX_GEOMETRY_IMAGE_UNIFORMS: GLenum = 0x90CD`"]
pub const GL_MAX_GEOMETRY_IMAGE_UNIFORMS: GLenum = 0x90CD;
#[doc = "`GL_MAX_GEOMETRY_IMAGE_UNIFORMS_EXT: GLenum = 0x90CD`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_IMAGE_UNIFORMS_EXT: GLenum = 0x90CD;
#[doc = "`GL_MAX_GEOMETRY_IMAGE_UNIFORMS_OES: GLenum = 0x90CD`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_IMAGE_UNIFORMS_OES: GLenum = 0x90CD;
#[doc = "`GL_MAX_GEOMETRY_INPUT_COMPONENTS: GLenum = 0x9123`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS: GLenum = 0x9123;
#[doc = "`GL_MAX_GEOMETRY_INPUT_COMPONENTS_EXT: GLenum = 0x9123`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS_EXT: GLenum = 0x9123;
#[doc = "`GL_MAX_GEOMETRY_INPUT_COMPONENTS_OES: GLenum = 0x9123`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS_OES: GLenum = 0x9123;
#[doc = "`GL_MAX_GEOMETRY_OUTPUT_COMPONENTS: GLenum = 0x9124`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS: GLenum = 0x9124;
#[doc = "`GL_MAX_GEOMETRY_OUTPUT_COMPONENTS_EXT: GLenum = 0x9124`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS_EXT: GLenum = 0x9124;
#[doc = "`GL_MAX_GEOMETRY_OUTPUT_COMPONENTS_OES: GLenum = 0x9124`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS_OES: GLenum = 0x9124;
#[doc = "`GL_MAX_GEOMETRY_OUTPUT_VERTICES: GLenum = 0x8DE0`"]
pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES: GLenum = 0x8DE0;
#[doc = "`GL_MAX_GEOMETRY_OUTPUT_VERTICES_EXT: GLenum = 0x8DE0`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES_EXT: GLenum = 0x8DE0;
#[doc = "`GL_MAX_GEOMETRY_OUTPUT_VERTICES_OES: GLenum = 0x8DE0`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES_OES: GLenum = 0x8DE0;
#[doc = "`GL_MAX_GEOMETRY_SHADER_INVOCATIONS: GLenum = 0x8E5A`"]
pub const GL_MAX_GEOMETRY_SHADER_INVOCATIONS: GLenum = 0x8E5A;
#[doc = "`GL_MAX_GEOMETRY_SHADER_INVOCATIONS_EXT: GLenum = 0x8E5A`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_SHADER_INVOCATIONS_EXT: GLenum = 0x8E5A;
#[doc = "`GL_MAX_GEOMETRY_SHADER_INVOCATIONS_OES: GLenum = 0x8E5A`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_SHADER_INVOCATIONS_OES: GLenum = 0x8E5A;
#[doc = "`GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: GLenum = 0x90D7`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: GLenum = 0x90D7;
#[doc = "`GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS_EXT: GLenum = 0x90D7`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS_EXT: GLenum = 0x90D7;
#[doc = "`GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS_OES: GLenum = 0x90D7`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS_OES: GLenum = 0x90D7;
#[doc = "`GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GLenum = 0x8C29`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GLenum = 0x8C29;
#[doc = "`GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS_EXT: GLenum = 0x8C29`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS_EXT: GLenum = 0x8C29;
#[doc = "`GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS_OES: GLenum = 0x8C29`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS_OES: GLenum = 0x8C29;
#[doc = "`GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8DE1`"]
pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8DE1;
#[doc = "`GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS_EXT: GLenum = 0x8DE1`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS_EXT: GLenum = 0x8DE1;
#[doc = "`GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS_OES: GLenum = 0x8DE1`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS_OES: GLenum = 0x8DE1;
#[doc = "`GL_MAX_GEOMETRY_UNIFORM_BLOCKS: GLenum = 0x8A2C`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS: GLenum = 0x8A2C;
#[doc = "`GL_MAX_GEOMETRY_UNIFORM_BLOCKS_EXT: GLenum = 0x8A2C`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS_EXT: GLenum = 0x8A2C;
#[doc = "`GL_MAX_GEOMETRY_UNIFORM_BLOCKS_OES: GLenum = 0x8A2C`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS_OES: GLenum = 0x8A2C;
#[doc = "`GL_MAX_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8DDF`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8DDF;
#[doc = "`GL_MAX_GEOMETRY_UNIFORM_COMPONENTS_EXT: GLenum = 0x8DDF`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS_EXT: GLenum = 0x8DDF;
#[doc = "`GL_MAX_GEOMETRY_UNIFORM_COMPONENTS_OES: GLenum = 0x8DDF`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS_OES: GLenum = 0x8DDF;
#[doc = "`GL_MAX_IMAGE_UNITS: GLenum = 0x8F38`"]
pub const GL_MAX_IMAGE_UNITS: GLenum = 0x8F38;
#[doc = "`GL_MAX_INTEGER_SAMPLES: GLenum = 0x9110`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_INTEGER_SAMPLES: GLenum = 0x9110;
#[doc = "`GL_MAX_LABEL_LENGTH: GLenum = 0x82E8`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_LABEL_LENGTH: GLenum = 0x82E8;
#[doc = "`GL_MAX_LABEL_LENGTH_KHR: GLenum = 0x82E8`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_MAX_LABEL_LENGTH_KHR: GLenum = 0x82E8;
#[doc = "`GL_MAX_MESH_ATOMIC_COUNTERS_NV: GLenum = 0x8E65`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_MESH_ATOMIC_COUNTERS_NV: GLenum = 0x8E65;
#[doc = "`GL_MAX_MESH_ATOMIC_COUNTER_BUFFERS_NV: GLenum = 0x8E64`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_MESH_ATOMIC_COUNTER_BUFFERS_NV: GLenum = 0x8E64;
#[doc = "`GL_MAX_MESH_IMAGE_UNIFORMS_NV: GLenum = 0x8E62`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_MESH_IMAGE_UNIFORMS_NV: GLenum = 0x8E62;
#[doc = "`GL_MAX_MESH_OUTPUT_PRIMITIVES_NV: GLenum = 0x9539`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_MESH_OUTPUT_PRIMITIVES_NV: GLenum = 0x9539;
#[doc = "`GL_MAX_MESH_OUTPUT_VERTICES_NV: GLenum = 0x9538`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_MESH_OUTPUT_VERTICES_NV: GLenum = 0x9538;
#[doc = "`GL_MAX_MESH_SHADER_STORAGE_BLOCKS_NV: GLenum = 0x8E66`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_MESH_SHADER_STORAGE_BLOCKS_NV: GLenum = 0x8E66;
#[doc = "`GL_MAX_MESH_TEXTURE_IMAGE_UNITS_NV: GLenum = 0x8E61`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_MESH_TEXTURE_IMAGE_UNITS_NV: GLenum = 0x8E61;
#[doc = "`GL_MAX_MESH_TOTAL_MEMORY_SIZE_NV: GLenum = 0x9536`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_MESH_TOTAL_MEMORY_SIZE_NV: GLenum = 0x9536;
#[doc = "`GL_MAX_MESH_UNIFORM_BLOCKS_NV: GLenum = 0x8E60`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_MESH_UNIFORM_BLOCKS_NV: GLenum = 0x8E60;
#[doc = "`GL_MAX_MESH_UNIFORM_COMPONENTS_NV: GLenum = 0x8E63`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_MESH_UNIFORM_COMPONENTS_NV: GLenum = 0x8E63;
#[doc = "`GL_MAX_MESH_VIEWS_NV: GLenum = 0x9557`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_MESH_VIEWS_NV: GLenum = 0x9557;
#[doc = "`GL_MAX_MESH_WORK_GROUP_INVOCATIONS_NV: GLenum = 0x95A2`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_MESH_WORK_GROUP_INVOCATIONS_NV: GLenum = 0x95A2;
#[doc = "`GL_MAX_MESH_WORK_GROUP_SIZE_NV: GLenum = 0x953B`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_MESH_WORK_GROUP_SIZE_NV: GLenum = 0x953B;
#[doc = "`GL_MAX_MULTIVIEW_BUFFERS_EXT: GLenum = 0x90F2`"]
#[cfg(any(feature = "GL_EXT_multiview_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_multiview_draw_buffers"))))]
pub const GL_MAX_MULTIVIEW_BUFFERS_EXT: GLenum = 0x90F2;
#[doc = "`GL_MAX_NAME_LENGTH: GLenum = 0x92F6`"]
#[doc = "* **Group:** ProgramInterfacePName"]
pub const GL_MAX_NAME_LENGTH: GLenum = 0x92F6;
#[doc = "`GL_MAX_NUM_ACTIVE_VARIABLES: GLenum = 0x92F7`"]
#[doc = "* **Group:** ProgramInterfacePName"]
pub const GL_MAX_NUM_ACTIVE_VARIABLES: GLenum = 0x92F7;
#[doc = "`GL_MAX_PATCH_VERTICES: GLenum = 0x8E7D`"]
pub const GL_MAX_PATCH_VERTICES: GLenum = 0x8E7D;
#[doc = "`GL_MAX_PATCH_VERTICES_EXT: GLenum = 0x8E7D`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_PATCH_VERTICES_EXT: GLenum = 0x8E7D;
#[doc = "`GL_MAX_PATCH_VERTICES_OES: GLenum = 0x8E7D`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_PATCH_VERTICES_OES: GLenum = 0x8E7D;
#[doc = "`GL_MAX_PROGRAM_TEXEL_OFFSET: GLenum = 0x8905`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_PROGRAM_TEXEL_OFFSET: GLenum = 0x8905;
#[doc = "`GL_MAX_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5F`"]
pub const GL_MAX_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5F;
#[doc = "`GL_MAX_RASTER_SAMPLES_EXT: GLenum = 0x9329`"]
#[cfg(any(
    feature = "GL_EXT_raster_multisample",
    feature = "GL_NV_framebuffer_mixed_samples"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_raster_multisample",
        feature = "GL_NV_framebuffer_mixed_samples"
    )))
)]
pub const GL_MAX_RASTER_SAMPLES_EXT: GLenum = 0x9329;
#[doc = "`GL_MAX_RENDERBUFFER_SIZE: GLenum = 0x84E8`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_RENDERBUFFER_SIZE: GLenum = 0x84E8;
#[doc = "`GL_MAX_SAMPLES: GLenum = 0x8D57`"]
pub const GL_MAX_SAMPLES: GLenum = 0x8D57;
#[doc = "`GL_MAX_SAMPLES_ANGLE: GLenum = 0x8D57`"]
#[cfg(any(feature = "GL_ANGLE_framebuffer_multisample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_ANGLE_framebuffer_multisample"))))]
pub const GL_MAX_SAMPLES_ANGLE: GLenum = 0x8D57;
#[doc = "`GL_MAX_SAMPLES_APPLE: GLenum = 0x8D57`"]
#[cfg(any(feature = "GL_APPLE_framebuffer_multisample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_framebuffer_multisample"))))]
pub const GL_MAX_SAMPLES_APPLE: GLenum = 0x8D57;
#[doc = "`GL_MAX_SAMPLES_EXT: GLenum = 0x8D57`"]
#[cfg(any(feature = "GL_EXT_multisampled_render_to_texture"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_multisampled_render_to_texture")))
)]
pub const GL_MAX_SAMPLES_EXT: GLenum = 0x8D57;
#[doc = "`GL_MAX_SAMPLES_IMG: GLenum = 0x9135`"]
#[cfg(any(feature = "GL_IMG_multisampled_render_to_texture"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_IMG_multisampled_render_to_texture")))
)]
pub const GL_MAX_SAMPLES_IMG: GLenum = 0x9135;
#[doc = "`GL_MAX_SAMPLES_NV: GLenum = 0x8D57`"]
#[cfg(any(feature = "GL_NV_framebuffer_multisample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_framebuffer_multisample"))))]
pub const GL_MAX_SAMPLES_NV: GLenum = 0x8D57;
#[doc = "`GL_MAX_SAMPLE_MASK_WORDS: GLenum = 0x8E59`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_SAMPLE_MASK_WORDS: GLenum = 0x8E59;
#[doc = "`GL_MAX_SERVER_WAIT_TIMEOUT: GLenum = 0x9111`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_SERVER_WAIT_TIMEOUT: GLenum = 0x9111;
#[doc = "`GL_MAX_SERVER_WAIT_TIMEOUT_APPLE: GLenum = 0x9111`"]
#[cfg(any(feature = "GL_APPLE_sync"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_sync"))))]
pub const GL_MAX_SERVER_WAIT_TIMEOUT_APPLE: GLenum = 0x9111;
#[doc = "`GL_MAX_SHADER_COMBINED_LOCAL_STORAGE_FAST_SIZE_EXT: GLenum = 0x9650`"]
#[cfg(any(feature = "GL_EXT_shader_pixel_local_storage2"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_shader_pixel_local_storage2"))))]
pub const GL_MAX_SHADER_COMBINED_LOCAL_STORAGE_FAST_SIZE_EXT: GLenum = 0x9650;
#[doc = "`GL_MAX_SHADER_COMBINED_LOCAL_STORAGE_SIZE_EXT: GLenum = 0x9651`"]
#[cfg(any(feature = "GL_EXT_shader_pixel_local_storage2"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_shader_pixel_local_storage2"))))]
pub const GL_MAX_SHADER_COMBINED_LOCAL_STORAGE_SIZE_EXT: GLenum = 0x9651;
#[doc = "`GL_MAX_SHADER_COMPILER_THREADS_KHR: GLenum = 0x91B0`"]
#[cfg(any(feature = "GL_KHR_parallel_shader_compile"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_parallel_shader_compile"))))]
pub const GL_MAX_SHADER_COMPILER_THREADS_KHR: GLenum = 0x91B0;
#[doc = "`GL_MAX_SHADER_PIXEL_LOCAL_STORAGE_FAST_SIZE_EXT: GLenum = 0x8F63`"]
#[cfg(any(feature = "GL_EXT_shader_pixel_local_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_shader_pixel_local_storage"))))]
pub const GL_MAX_SHADER_PIXEL_LOCAL_STORAGE_FAST_SIZE_EXT: GLenum = 0x8F63;
#[doc = "`GL_MAX_SHADER_PIXEL_LOCAL_STORAGE_SIZE_EXT: GLenum = 0x8F67`"]
#[cfg(any(feature = "GL_EXT_shader_pixel_local_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_shader_pixel_local_storage"))))]
pub const GL_MAX_SHADER_PIXEL_LOCAL_STORAGE_SIZE_EXT: GLenum = 0x8F67;
#[doc = "`GL_MAX_SHADER_STORAGE_BLOCK_SIZE: GLenum = 0x90DE`"]
pub const GL_MAX_SHADER_STORAGE_BLOCK_SIZE: GLenum = 0x90DE;
#[doc = "`GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS: GLenum = 0x90DD`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS: GLenum = 0x90DD;
#[doc = "`GL_MAX_SHADER_SUBSAMPLED_IMAGE_UNITS_QCOM: GLenum = 0x8FA1`"]
#[cfg(any(feature = "GL_QCOM_texture_foveated_subsampled_layout"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_QCOM_texture_foveated_subsampled_layout")))
)]
pub const GL_MAX_SHADER_SUBSAMPLED_IMAGE_UNITS_QCOM: GLenum = 0x8FA1;
#[doc = "`GL_MAX_SPARSE_3D_TEXTURE_SIZE_EXT: GLenum = 0x9199`"]
#[cfg(any(feature = "GL_EXT_sparse_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_sparse_texture"))))]
pub const GL_MAX_SPARSE_3D_TEXTURE_SIZE_EXT: GLenum = 0x9199;
#[doc = "`GL_MAX_SPARSE_ARRAY_TEXTURE_LAYERS_EXT: GLenum = 0x919A`"]
#[cfg(any(feature = "GL_EXT_sparse_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_sparse_texture"))))]
pub const GL_MAX_SPARSE_ARRAY_TEXTURE_LAYERS_EXT: GLenum = 0x919A;
#[doc = "`GL_MAX_SPARSE_TEXTURE_SIZE_EXT: GLenum = 0x9198`"]
#[cfg(any(feature = "GL_EXT_sparse_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_sparse_texture"))))]
pub const GL_MAX_SPARSE_TEXTURE_SIZE_EXT: GLenum = 0x9198;
#[doc = "`GL_MAX_SUBPIXEL_PRECISION_BIAS_BITS_NV: GLenum = 0x9349`"]
#[cfg(any(feature = "GL_NV_conservative_raster"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_conservative_raster"))))]
pub const GL_MAX_SUBPIXEL_PRECISION_BIAS_BITS_NV: GLenum = 0x9349;
#[doc = "`GL_MAX_TASK_ATOMIC_COUNTERS_NV: GLenum = 0x8E6D`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_TASK_ATOMIC_COUNTERS_NV: GLenum = 0x8E6D;
#[doc = "`GL_MAX_TASK_ATOMIC_COUNTER_BUFFERS_NV: GLenum = 0x8E6C`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_TASK_ATOMIC_COUNTER_BUFFERS_NV: GLenum = 0x8E6C;
#[doc = "`GL_MAX_TASK_IMAGE_UNIFORMS_NV: GLenum = 0x8E6A`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_TASK_IMAGE_UNIFORMS_NV: GLenum = 0x8E6A;
#[doc = "`GL_MAX_TASK_OUTPUT_COUNT_NV: GLenum = 0x953A`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_TASK_OUTPUT_COUNT_NV: GLenum = 0x953A;
#[doc = "`GL_MAX_TASK_SHADER_STORAGE_BLOCKS_NV: GLenum = 0x8E6E`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_TASK_SHADER_STORAGE_BLOCKS_NV: GLenum = 0x8E6E;
#[doc = "`GL_MAX_TASK_TEXTURE_IMAGE_UNITS_NV: GLenum = 0x8E69`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_TASK_TEXTURE_IMAGE_UNITS_NV: GLenum = 0x8E69;
#[doc = "`GL_MAX_TASK_TOTAL_MEMORY_SIZE_NV: GLenum = 0x9537`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_TASK_TOTAL_MEMORY_SIZE_NV: GLenum = 0x9537;
#[doc = "`GL_MAX_TASK_UNIFORM_BLOCKS_NV: GLenum = 0x8E68`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_TASK_UNIFORM_BLOCKS_NV: GLenum = 0x8E68;
#[doc = "`GL_MAX_TASK_UNIFORM_COMPONENTS_NV: GLenum = 0x8E6B`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_TASK_UNIFORM_COMPONENTS_NV: GLenum = 0x8E6B;
#[doc = "`GL_MAX_TASK_WORK_GROUP_INVOCATIONS_NV: GLenum = 0x95A3`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_TASK_WORK_GROUP_INVOCATIONS_NV: GLenum = 0x95A3;
#[doc = "`GL_MAX_TASK_WORK_GROUP_SIZE_NV: GLenum = 0x953C`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MAX_TASK_WORK_GROUP_SIZE_NV: GLenum = 0x953C;
#[doc = "`GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS: GLenum = 0x92D3`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS: GLenum = 0x92D3;
#[doc = "`GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS_EXT: GLenum = 0x92D3`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS_EXT: GLenum = 0x92D3;
#[doc = "`GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS_OES: GLenum = 0x92D3`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS_OES: GLenum = 0x92D3;
#[doc = "`GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CD`"]
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CD;
#[doc = "`GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS_EXT: GLenum = 0x92CD`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS_EXT: GLenum = 0x92CD;
#[doc = "`GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS_OES: GLenum = 0x92CD`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS_OES: GLenum = 0x92CD;
#[doc = "`GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS: GLenum = 0x90CB`"]
pub const GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS: GLenum = 0x90CB;
#[doc = "`GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS_EXT: GLenum = 0x90CB`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS_EXT: GLenum = 0x90CB;
#[doc = "`GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS_OES: GLenum = 0x90CB`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS_OES: GLenum = 0x90CB;
#[doc = "`GL_MAX_TESS_CONTROL_INPUT_COMPONENTS: GLenum = 0x886C`"]
pub const GL_MAX_TESS_CONTROL_INPUT_COMPONENTS: GLenum = 0x886C;
#[doc = "`GL_MAX_TESS_CONTROL_INPUT_COMPONENTS_EXT: GLenum = 0x886C`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_TESS_CONTROL_INPUT_COMPONENTS_EXT: GLenum = 0x886C;
#[doc = "`GL_MAX_TESS_CONTROL_INPUT_COMPONENTS_OES: GLenum = 0x886C`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_TESS_CONTROL_INPUT_COMPONENTS_OES: GLenum = 0x886C;
#[doc = "`GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS: GLenum = 0x8E83`"]
pub const GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS: GLenum = 0x8E83;
#[doc = "`GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS_EXT: GLenum = 0x8E83`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS_EXT: GLenum = 0x8E83;
#[doc = "`GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS_OES: GLenum = 0x8E83`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS_OES: GLenum = 0x8E83;
#[doc = "`GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: GLenum = 0x90D8`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: GLenum = 0x90D8;
#[doc = "`GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS_EXT: GLenum = 0x90D8`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS_EXT: GLenum = 0x90D8;
#[doc = "`GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS_OES: GLenum = 0x90D8`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS_OES: GLenum = 0x90D8;
#[doc = "`GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: GLenum = 0x8E81`"]
pub const GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: GLenum = 0x8E81;
#[doc = "`GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS_EXT: GLenum = 0x8E81`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS_EXT: GLenum = 0x8E81;
#[doc = "`GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS_OES: GLenum = 0x8E81`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS_OES: GLenum = 0x8E81;
#[doc = "`GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8E85`"]
pub const GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8E85;
#[doc = "`GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS_EXT: GLenum = 0x8E85`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS_EXT: GLenum = 0x8E85;
#[doc = "`GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS_OES: GLenum = 0x8E85`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS_OES: GLenum = 0x8E85;
#[doc = "`GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS: GLenum = 0x8E89`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS: GLenum = 0x8E89;
#[doc = "`GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS_EXT: GLenum = 0x8E89`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS_EXT: GLenum = 0x8E89;
#[doc = "`GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS_OES: GLenum = 0x8E89`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS_OES: GLenum = 0x8E89;
#[doc = "`GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E7F`"]
pub const GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E7F;
#[doc = "`GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS_EXT: GLenum = 0x8E7F`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS_EXT: GLenum = 0x8E7F;
#[doc = "`GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS_OES: GLenum = 0x8E7F`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS_OES: GLenum = 0x8E7F;
#[doc = "`GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS: GLenum = 0x92D4`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS: GLenum = 0x92D4;
#[doc = "`GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS_EXT: GLenum = 0x92D4`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS_EXT: GLenum = 0x92D4;
#[doc = "`GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS_OES: GLenum = 0x92D4`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS_OES: GLenum = 0x92D4;
#[doc = "`GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CE`"]
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CE;
#[doc = "`GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS_EXT: GLenum = 0x92CE`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS_EXT: GLenum = 0x92CE;
#[doc = "`GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS_OES: GLenum = 0x92CE`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS_OES: GLenum = 0x92CE;
#[doc = "`GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS: GLenum = 0x90CC`"]
pub const GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS: GLenum = 0x90CC;
#[doc = "`GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS_EXT: GLenum = 0x90CC`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS_EXT: GLenum = 0x90CC;
#[doc = "`GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS_OES: GLenum = 0x90CC`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS_OES: GLenum = 0x90CC;
#[doc = "`GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS: GLenum = 0x886D`"]
pub const GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS: GLenum = 0x886D;
#[doc = "`GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS_EXT: GLenum = 0x886D`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS_EXT: GLenum = 0x886D;
#[doc = "`GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS_OES: GLenum = 0x886D`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS_OES: GLenum = 0x886D;
#[doc = "`GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: GLenum = 0x8E86`"]
pub const GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: GLenum = 0x8E86;
#[doc = "`GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS_EXT: GLenum = 0x8E86`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS_EXT: GLenum = 0x8E86;
#[doc = "`GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS_OES: GLenum = 0x8E86`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS_OES: GLenum = 0x8E86;
#[doc = "`GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: GLenum = 0x90D9`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: GLenum = 0x90D9;
#[doc = "`GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS_EXT: GLenum = 0x90D9`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS_EXT: GLenum = 0x90D9;
#[doc = "`GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS_OES: GLenum = 0x90D9`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS_OES: GLenum = 0x90D9;
#[doc = "`GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: GLenum = 0x8E82`"]
pub const GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: GLenum = 0x8E82;
#[doc = "`GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS_EXT: GLenum = 0x8E82`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS_EXT: GLenum = 0x8E82;
#[doc = "`GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS_OES: GLenum = 0x8E82`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS_OES: GLenum = 0x8E82;
#[doc = "`GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS: GLenum = 0x8E8A`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS: GLenum = 0x8E8A;
#[doc = "`GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS_EXT: GLenum = 0x8E8A`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS_EXT: GLenum = 0x8E8A;
#[doc = "`GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS_OES: GLenum = 0x8E8A`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS_OES: GLenum = 0x8E8A;
#[doc = "`GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E80`"]
pub const GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E80;
#[doc = "`GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS_EXT: GLenum = 0x8E80`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS_EXT: GLenum = 0x8E80;
#[doc = "`GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS_OES: GLenum = 0x8E80`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS_OES: GLenum = 0x8E80;
#[doc = "`GL_MAX_TESS_GEN_LEVEL: GLenum = 0x8E7E`"]
pub const GL_MAX_TESS_GEN_LEVEL: GLenum = 0x8E7E;
#[doc = "`GL_MAX_TESS_GEN_LEVEL_EXT: GLenum = 0x8E7E`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_TESS_GEN_LEVEL_EXT: GLenum = 0x8E7E;
#[doc = "`GL_MAX_TESS_GEN_LEVEL_OES: GLenum = 0x8E7E`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_TESS_GEN_LEVEL_OES: GLenum = 0x8E7E;
#[doc = "`GL_MAX_TESS_PATCH_COMPONENTS: GLenum = 0x8E84`"]
pub const GL_MAX_TESS_PATCH_COMPONENTS: GLenum = 0x8E84;
#[doc = "`GL_MAX_TESS_PATCH_COMPONENTS_EXT: GLenum = 0x8E84`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_MAX_TESS_PATCH_COMPONENTS_EXT: GLenum = 0x8E84;
#[doc = "`GL_MAX_TESS_PATCH_COMPONENTS_OES: GLenum = 0x8E84`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_MAX_TESS_PATCH_COMPONENTS_OES: GLenum = 0x8E84;
#[doc = "`GL_MAX_TEXTURE_BUFFER_SIZE: GLenum = 0x8C2B`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_TEXTURE_BUFFER_SIZE: GLenum = 0x8C2B;
#[doc = "`GL_MAX_TEXTURE_BUFFER_SIZE_EXT: GLenum = 0x8C2B`"]
#[cfg(any(feature = "GL_EXT_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_buffer"))))]
pub const GL_MAX_TEXTURE_BUFFER_SIZE_EXT: GLenum = 0x8C2B;
#[doc = "`GL_MAX_TEXTURE_BUFFER_SIZE_OES: GLenum = 0x8C2B`"]
#[cfg(any(feature = "GL_OES_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_buffer"))))]
pub const GL_MAX_TEXTURE_BUFFER_SIZE_OES: GLenum = 0x8C2B;
#[doc = "`GL_MAX_TEXTURE_IMAGE_UNITS: GLenum = 0x8872`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_TEXTURE_IMAGE_UNITS: GLenum = 0x8872;
#[doc = "`GL_MAX_TEXTURE_LOD_BIAS: GLenum = 0x84FD`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_TEXTURE_LOD_BIAS: GLenum = 0x84FD;
#[doc = "`GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT: GLenum = 0x84FF`"]
#[doc = "* **Alias Of:** `GL_MAX_TEXTURE_MAX_ANISOTROPY`"]
#[cfg(any(feature = "GL_EXT_texture_filter_anisotropic"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_filter_anisotropic"))))]
pub const GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT: GLenum = 0x84FF;
#[doc = "`GL_MAX_TEXTURE_SIZE: GLenum = 0x0D33`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_TEXTURE_SIZE: GLenum = 0x0D33;
#[doc = "`GL_MAX_TIMELINE_SEMAPHORE_VALUE_DIFFERENCE_NV: GLenum = 0x95B6`"]
#[doc = "* **Group:** GetPName"]
#[cfg(any(feature = "GL_NV_timeline_semaphore"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_timeline_semaphore"))))]
pub const GL_MAX_TIMELINE_SEMAPHORE_VALUE_DIFFERENCE_NV: GLenum = 0x95B6;
#[doc = "`GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = 0x8C8A`"]
pub const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = 0x8C8A;
#[doc = "`GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = 0x8C8B`"]
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = 0x8C8B;
#[doc = "`GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = 0x8C80`"]
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = 0x8C80;
#[doc = "`GL_MAX_UNIFORM_BLOCK_SIZE: GLenum = 0x8A30`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_UNIFORM_BLOCK_SIZE: GLenum = 0x8A30;
#[doc = "`GL_MAX_UNIFORM_BUFFER_BINDINGS: GLenum = 0x8A2F`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_UNIFORM_BUFFER_BINDINGS: GLenum = 0x8A2F;
#[doc = "`GL_MAX_UNIFORM_LOCATIONS: GLenum = 0x826E`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_UNIFORM_LOCATIONS: GLenum = 0x826E;
#[doc = "`GL_MAX_VARYING_COMPONENTS: GLenum = 0x8B4B`"]
#[doc = "* **Group:** GetPName"]
#[doc = "* **Alias Of:** `GL_MAX_VARYING_FLOATS`"]
pub const GL_MAX_VARYING_COMPONENTS: GLenum = 0x8B4B;
#[doc = "`GL_MAX_VARYING_VECTORS: GLenum = 0x8DFC`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_VARYING_VECTORS: GLenum = 0x8DFC;
#[doc = "`GL_MAX_VERTEX_ATOMIC_COUNTERS: GLenum = 0x92D2`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_VERTEX_ATOMIC_COUNTERS: GLenum = 0x92D2;
#[doc = "`GL_MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CC`"]
pub const GL_MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CC;
#[doc = "`GL_MAX_VERTEX_ATTRIBS: GLenum = 0x8869`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_VERTEX_ATTRIBS: GLenum = 0x8869;
#[doc = "`GL_MAX_VERTEX_ATTRIB_BINDINGS: GLenum = 0x82DA`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_VERTEX_ATTRIB_BINDINGS: GLenum = 0x82DA;
#[doc = "`GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D9`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D9;
#[doc = "`GL_MAX_VERTEX_ATTRIB_STRIDE: GLenum = 0x82E5`"]
pub const GL_MAX_VERTEX_ATTRIB_STRIDE: GLenum = 0x82E5;
#[doc = "`GL_MAX_VERTEX_IMAGE_UNIFORMS: GLenum = 0x90CA`"]
pub const GL_MAX_VERTEX_IMAGE_UNIFORMS: GLenum = 0x90CA;
#[doc = "`GL_MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = 0x9122`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = 0x9122;
#[doc = "`GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS: GLenum = 0x90D6`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS: GLenum = 0x90D6;
#[doc = "`GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4C`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4C;
#[doc = "`GL_MAX_VERTEX_UNIFORM_BLOCKS: GLenum = 0x8A2B`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_VERTEX_UNIFORM_BLOCKS: GLenum = 0x8A2B;
#[doc = "`GL_MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8B4A`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8B4A;
#[doc = "`GL_MAX_VERTEX_UNIFORM_VECTORS: GLenum = 0x8DFB`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_VERTEX_UNIFORM_VECTORS: GLenum = 0x8DFB;
#[doc = "`GL_MAX_VIEWPORTS_NV: GLenum = 0x825B`"]
#[cfg(any(feature = "GL_NV_viewport_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_viewport_array"))))]
pub const GL_MAX_VIEWPORTS_NV: GLenum = 0x825B;
#[doc = "`GL_MAX_VIEWPORTS_OES: GLenum = 0x825B`"]
#[cfg(any(feature = "GL_OES_viewport_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_viewport_array"))))]
pub const GL_MAX_VIEWPORTS_OES: GLenum = 0x825B;
#[doc = "`GL_MAX_VIEWPORT_DIMS: GLenum = 0x0D3A`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MAX_VIEWPORT_DIMS: GLenum = 0x0D3A;
#[doc = "`GL_MAX_VIEWS_OVR: GLenum = 0x9631`"]
#[cfg(any(feature = "GL_OVR_multiview"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OVR_multiview"))))]
pub const GL_MAX_VIEWS_OVR: GLenum = 0x9631;
#[doc = "`GL_MAX_WINDOW_RECTANGLES_EXT: GLenum = 0x8F14`"]
#[cfg(any(feature = "GL_EXT_window_rectangles"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_window_rectangles"))))]
pub const GL_MAX_WINDOW_RECTANGLES_EXT: GLenum = 0x8F14;
#[doc = "`GL_MEDIUM_FLOAT: GLenum = 0x8DF1`"]
#[doc = "* **Group:** PrecisionType"]
pub const GL_MEDIUM_FLOAT: GLenum = 0x8DF1;
#[doc = "`GL_MEDIUM_INT: GLenum = 0x8DF4`"]
#[doc = "* **Group:** PrecisionType"]
pub const GL_MEDIUM_INT: GLenum = 0x8DF4;
#[doc = "`GL_MEMORY_ATTACHABLE_ALIGNMENT_NV: GLenum = 0x95A6`"]
#[cfg(any(feature = "GL_NV_memory_attachment"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_memory_attachment"))))]
pub const GL_MEMORY_ATTACHABLE_ALIGNMENT_NV: GLenum = 0x95A6;
#[doc = "`GL_MEMORY_ATTACHABLE_NV: GLenum = 0x95A8`"]
#[cfg(any(feature = "GL_NV_memory_attachment"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_memory_attachment"))))]
pub const GL_MEMORY_ATTACHABLE_NV: GLenum = 0x95A8;
#[doc = "`GL_MEMORY_ATTACHABLE_SIZE_NV: GLenum = 0x95A7`"]
#[cfg(any(feature = "GL_NV_memory_attachment"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_memory_attachment"))))]
pub const GL_MEMORY_ATTACHABLE_SIZE_NV: GLenum = 0x95A7;
#[doc = "`GL_MESH_OUTPUT_PER_PRIMITIVE_GRANULARITY_NV: GLenum = 0x9543`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MESH_OUTPUT_PER_PRIMITIVE_GRANULARITY_NV: GLenum = 0x9543;
#[doc = "`GL_MESH_OUTPUT_PER_VERTEX_GRANULARITY_NV: GLenum = 0x92DF`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MESH_OUTPUT_PER_VERTEX_GRANULARITY_NV: GLenum = 0x92DF;
#[doc = "`GL_MESH_OUTPUT_TYPE_NV: GLenum = 0x957B`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MESH_OUTPUT_TYPE_NV: GLenum = 0x957B;
#[doc = "`GL_MESH_PRIMITIVES_OUT_NV: GLenum = 0x957A`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MESH_PRIMITIVES_OUT_NV: GLenum = 0x957A;
#[doc = "`GL_MESH_SHADER_BIT_NV: GLbitfield = 0x00000040`"]
#[doc = "* **Group:** UseProgramStageMask"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MESH_SHADER_BIT_NV: GLbitfield = 0x00000040;
#[doc = "`GL_MESH_SHADER_NV: GLenum = 0x9559`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MESH_SHADER_NV: GLenum = 0x9559;
#[doc = "`GL_MESH_SUBROUTINE_NV: GLenum = 0x957C`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MESH_SUBROUTINE_NV: GLenum = 0x957C;
#[doc = "`GL_MESH_SUBROUTINE_UNIFORM_NV: GLenum = 0x957E`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MESH_SUBROUTINE_UNIFORM_NV: GLenum = 0x957E;
#[doc = "`GL_MESH_VERTICES_OUT_NV: GLenum = 0x9579`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MESH_VERTICES_OUT_NV: GLenum = 0x9579;
#[doc = "`GL_MESH_WORK_GROUP_SIZE_NV: GLenum = 0x953E`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_MESH_WORK_GROUP_SIZE_NV: GLenum = 0x953E;
#[doc = "`GL_MIN: GLenum = 0x8007`"]
#[doc = "* **Group:** BlendEquationModeEXT"]
pub const GL_MIN: GLenum = 0x8007;
#[doc = "`GL_MINOR_VERSION: GLenum = 0x821C`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MINOR_VERSION: GLenum = 0x821C;
#[doc = "`GL_MINUS_CLAMPED_NV: GLenum = 0x92B3`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_MINUS_CLAMPED_NV: GLenum = 0x92B3;
#[doc = "`GL_MINUS_NV: GLenum = 0x929F`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_MINUS_NV: GLenum = 0x929F;
#[doc = "`GL_MIN_EXT: GLenum = 0x8007`"]
#[doc = "* **Group:** BlendEquationModeEXT"]
#[cfg(any(feature = "GL_EXT_blend_minmax"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_blend_minmax"))))]
pub const GL_MIN_EXT: GLenum = 0x8007;
#[doc = "`GL_MIN_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 0x8E5B`"]
pub const GL_MIN_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 0x8E5B;
#[doc = "`GL_MIN_FRAGMENT_INTERPOLATION_OFFSET_OES: GLenum = 0x8E5B`"]
#[cfg(any(feature = "GL_OES_shader_multisample_interpolation"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_OES_shader_multisample_interpolation")))
)]
pub const GL_MIN_FRAGMENT_INTERPOLATION_OFFSET_OES: GLenum = 0x8E5B;
#[doc = "`GL_MIN_PROGRAM_TEXEL_OFFSET: GLenum = 0x8904`"]
#[doc = "* **Group:** GetPName"]
pub const GL_MIN_PROGRAM_TEXEL_OFFSET: GLenum = 0x8904;
#[doc = "`GL_MIN_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5E`"]
pub const GL_MIN_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5E;
#[doc = "`GL_MIN_SAMPLE_SHADING_VALUE: GLenum = 0x8C37`"]
pub const GL_MIN_SAMPLE_SHADING_VALUE: GLenum = 0x8C37;
#[doc = "`GL_MIN_SAMPLE_SHADING_VALUE_OES: GLenum = 0x8C37`"]
#[cfg(any(feature = "GL_OES_sample_shading"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_sample_shading"))))]
pub const GL_MIN_SAMPLE_SHADING_VALUE_OES: GLenum = 0x8C37;
#[doc = "`GL_MIRRORED_REPEAT: GLenum = 0x8370`"]
#[doc = "* **Group:** TextureWrapMode"]
pub const GL_MIRRORED_REPEAT: GLenum = 0x8370;
#[doc = "`GL_MIRROR_CLAMP_TO_EDGE_EXT: GLenum = 0x8743`"]
#[cfg(any(feature = "GL_EXT_texture_mirror_clamp_to_edge"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_texture_mirror_clamp_to_edge")))
)]
pub const GL_MIRROR_CLAMP_TO_EDGE_EXT: GLenum = 0x8743;
#[doc = "`GL_MITER_REVERT_NV: GLenum = 0x90A7`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_MITER_REVERT_NV: GLenum = 0x90A7;
#[doc = "`GL_MITER_TRUNCATE_NV: GLenum = 0x90A8`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_MITER_TRUNCATE_NV: GLenum = 0x90A8;
#[doc = "`GL_MIXED_DEPTH_SAMPLES_SUPPORTED_NV: GLenum = 0x932F`"]
#[cfg(any(feature = "GL_NV_framebuffer_mixed_samples"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_framebuffer_mixed_samples"))))]
pub const GL_MIXED_DEPTH_SAMPLES_SUPPORTED_NV: GLenum = 0x932F;
#[doc = "`GL_MIXED_STENCIL_SAMPLES_SUPPORTED_NV: GLenum = 0x9330`"]
#[cfg(any(feature = "GL_NV_framebuffer_mixed_samples"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_framebuffer_mixed_samples"))))]
pub const GL_MIXED_STENCIL_SAMPLES_SUPPORTED_NV: GLenum = 0x9330;
#[doc = "`GL_MOTION_ESTIMATION_SEARCH_BLOCK_X_QCOM: GLenum = 0x8C90`"]
#[doc = "* **Group:** GetPName"]
#[cfg(any(feature = "GL_QCOM_motion_estimation"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_motion_estimation"))))]
pub const GL_MOTION_ESTIMATION_SEARCH_BLOCK_X_QCOM: GLenum = 0x8C90;
#[doc = "`GL_MOTION_ESTIMATION_SEARCH_BLOCK_Y_QCOM: GLenum = 0x8C91`"]
#[doc = "* **Group:** GetPName"]
#[cfg(any(feature = "GL_QCOM_motion_estimation"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_motion_estimation"))))]
pub const GL_MOTION_ESTIMATION_SEARCH_BLOCK_Y_QCOM: GLenum = 0x8C91;
#[doc = "`GL_MOVE_TO_CONTINUES_NV: GLenum = 0x90B6`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_MOVE_TO_CONTINUES_NV: GLenum = 0x90B6;
#[doc = "`GL_MOVE_TO_NV: GLenum = 0x02`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_MOVE_TO_NV: GLenum = 0x02;
#[doc = "`GL_MOVE_TO_RESETS_NV: GLenum = 0x90B5`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_MOVE_TO_RESETS_NV: GLenum = 0x90B5;
#[doc = "`GL_MULTIPLY: GLenum = 0x9294`"]
pub const GL_MULTIPLY: GLenum = 0x9294;
#[doc = "`GL_MULTIPLY_KHR: GLenum = 0x9294`"]
#[cfg(any(feature = "GL_KHR_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_blend_equation_advanced"))))]
pub const GL_MULTIPLY_KHR: GLenum = 0x9294;
#[doc = "`GL_MULTIPLY_NV: GLenum = 0x9294`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_MULTIPLY_NV: GLenum = 0x9294;
#[doc = "`GL_MULTISAMPLES_NV: GLenum = 0x9371`"]
#[cfg(any(feature = "GL_NV_internalformat_sample_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_internalformat_sample_query"))))]
pub const GL_MULTISAMPLES_NV: GLenum = 0x9371;
#[doc = "`GL_MULTISAMPLE_BUFFER_BIT0_QCOM: GLbitfield = 0x01000000`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_MULTISAMPLE_BUFFER_BIT0_QCOM: GLbitfield = 0x01000000;
#[doc = "`GL_MULTISAMPLE_BUFFER_BIT1_QCOM: GLbitfield = 0x02000000`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_MULTISAMPLE_BUFFER_BIT1_QCOM: GLbitfield = 0x02000000;
#[doc = "`GL_MULTISAMPLE_BUFFER_BIT2_QCOM: GLbitfield = 0x04000000`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_MULTISAMPLE_BUFFER_BIT2_QCOM: GLbitfield = 0x04000000;
#[doc = "`GL_MULTISAMPLE_BUFFER_BIT3_QCOM: GLbitfield = 0x08000000`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_MULTISAMPLE_BUFFER_BIT3_QCOM: GLbitfield = 0x08000000;
#[doc = "`GL_MULTISAMPLE_BUFFER_BIT4_QCOM: GLbitfield = 0x10000000`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_MULTISAMPLE_BUFFER_BIT4_QCOM: GLbitfield = 0x10000000;
#[doc = "`GL_MULTISAMPLE_BUFFER_BIT5_QCOM: GLbitfield = 0x20000000`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_MULTISAMPLE_BUFFER_BIT5_QCOM: GLbitfield = 0x20000000;
#[doc = "`GL_MULTISAMPLE_BUFFER_BIT6_QCOM: GLbitfield = 0x40000000`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_MULTISAMPLE_BUFFER_BIT6_QCOM: GLbitfield = 0x40000000;
#[doc = "`GL_MULTISAMPLE_BUFFER_BIT7_QCOM: GLbitfield = 0x80000000`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_MULTISAMPLE_BUFFER_BIT7_QCOM: GLbitfield = 0x80000000;
#[doc = "`GL_MULTISAMPLE_LINE_WIDTH_GRANULARITY: GLenum = 0x9382`"]
pub const GL_MULTISAMPLE_LINE_WIDTH_GRANULARITY: GLenum = 0x9382;
#[doc = "`GL_MULTISAMPLE_LINE_WIDTH_RANGE: GLenum = 0x9381`"]
pub const GL_MULTISAMPLE_LINE_WIDTH_RANGE: GLenum = 0x9381;
#[doc = "`GL_MULTISAMPLE_RASTERIZATION_ALLOWED_EXT: GLenum = 0x932B`"]
#[cfg(any(
    feature = "GL_EXT_raster_multisample",
    feature = "GL_NV_framebuffer_mixed_samples"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_raster_multisample",
        feature = "GL_NV_framebuffer_mixed_samples"
    )))
)]
pub const GL_MULTISAMPLE_RASTERIZATION_ALLOWED_EXT: GLenum = 0x932B;
#[doc = "`GL_MULTIVIEW_EXT: GLenum = 0x90F1`"]
#[cfg(any(feature = "GL_EXT_multiview_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_multiview_draw_buffers"))))]
pub const GL_MULTIVIEW_EXT: GLenum = 0x90F1;
#[doc = "`GL_NAME_LENGTH: GLenum = 0x92F9`"]
#[doc = "* **Group:** ProgramResourceProperty"]
pub const GL_NAME_LENGTH: GLenum = 0x92F9;
#[doc = "`GL_NEAREST: GLenum = 0x2600`"]
#[doc = "* **Groups:** BlitFramebufferFilter, TextureMagFilter, TextureMinFilter"]
pub const GL_NEAREST: GLenum = 0x2600;
#[doc = "`GL_NEAREST_MIPMAP_LINEAR: GLenum = 0x2702`"]
#[doc = "* **Group:** TextureMinFilter"]
pub const GL_NEAREST_MIPMAP_LINEAR: GLenum = 0x2702;
#[doc = "`GL_NEAREST_MIPMAP_NEAREST: GLenum = 0x2700`"]
#[doc = "* **Group:** TextureMinFilter"]
pub const GL_NEAREST_MIPMAP_NEAREST: GLenum = 0x2700;
#[doc = "`GL_NEGATIVE_ONE_TO_ONE_EXT: GLenum = 0x935E`"]
#[doc = "* **Alias Of:** `GL_NEGATIVE_ONE_TO_ONE`"]
#[cfg(any(feature = "GL_EXT_clip_control"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_clip_control"))))]
pub const GL_NEGATIVE_ONE_TO_ONE_EXT: GLenum = 0x935E;
#[doc = "`GL_NEVER: GLenum = 0x0200`"]
#[doc = "* **Groups:** StencilFunction, IndexFunctionEXT, AlphaFunction, DepthFunction"]
pub const GL_NEVER: GLenum = 0x0200;
#[doc = "`GL_NICEST: GLenum = 0x1102`"]
#[doc = "* **Group:** HintMode"]
pub const GL_NICEST: GLenum = 0x1102;
#[doc = "`GL_NONE: GLenum = 0`"]
#[doc = "* **Groups:** SyncBehaviorFlags, TextureCompareMode, PathColorFormat, CombinerBiasNV, CombinerScaleNV, DrawBufferMode, PixelTexGenMode, ReadBufferMode, ColorBuffer, PathGenMode, PathTransformType, PathFontStyle"]
pub const GL_NONE: GLenum = 0;
#[doc = "`GL_NOTEQUAL: GLenum = 0x0205`"]
#[doc = "* **Groups:** StencilFunction, IndexFunctionEXT, AlphaFunction, DepthFunction"]
pub const GL_NOTEQUAL: GLenum = 0x0205;
#[doc = "`GL_NO_ERROR: GLenum = 0`"]
#[doc = "* **Groups:** GraphicsResetStatus, ErrorCode"]
pub const GL_NO_ERROR: GLenum = 0;
#[doc = "`GL_NO_RESET_NOTIFICATION: GLenum = 0x8261`"]
pub const GL_NO_RESET_NOTIFICATION: GLenum = 0x8261;
#[doc = "`GL_NO_RESET_NOTIFICATION_EXT: GLenum = 0x8261`"]
#[cfg(any(feature = "GL_EXT_robustness"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_robustness"))))]
pub const GL_NO_RESET_NOTIFICATION_EXT: GLenum = 0x8261;
#[doc = "`GL_NO_RESET_NOTIFICATION_KHR: GLenum = 0x8261`"]
#[cfg(any(feature = "GL_KHR_robustness"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_robustness"))))]
pub const GL_NO_RESET_NOTIFICATION_KHR: GLenum = 0x8261;
#[doc = "`GL_NUM_ACTIVE_VARIABLES: GLenum = 0x9304`"]
#[doc = "* **Group:** ProgramResourceProperty"]
pub const GL_NUM_ACTIVE_VARIABLES: GLenum = 0x9304;
#[doc = "`GL_NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A2`"]
#[doc = "* **Group:** GetPName"]
pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A2;
#[doc = "`GL_NUM_DEVICE_UUIDS_EXT: GLenum = 0x9596`"]
#[doc = "* **Group:** GetPName"]
#[cfg(any(feature = "GL_EXT_memory_object", feature = "GL_EXT_semaphore"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_memory_object", feature = "GL_EXT_semaphore")))
)]
pub const GL_NUM_DEVICE_UUIDS_EXT: GLenum = 0x9596;
#[doc = "`GL_NUM_DOWNSAMPLE_SCALES_IMG: GLenum = 0x913D`"]
#[cfg(any(feature = "GL_IMG_framebuffer_downsample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_IMG_framebuffer_downsample"))))]
pub const GL_NUM_DOWNSAMPLE_SCALES_IMG: GLenum = 0x913D;
#[doc = "`GL_NUM_EXTENSIONS: GLenum = 0x821D`"]
#[doc = "* **Group:** GetPName"]
pub const GL_NUM_EXTENSIONS: GLenum = 0x821D;
#[doc = "`GL_NUM_PROGRAM_BINARY_FORMATS: GLenum = 0x87FE`"]
#[doc = "* **Group:** GetPName"]
pub const GL_NUM_PROGRAM_BINARY_FORMATS: GLenum = 0x87FE;
#[doc = "`GL_NUM_PROGRAM_BINARY_FORMATS_OES: GLenum = 0x87FE`"]
#[cfg(any(feature = "GL_OES_get_program_binary"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_get_program_binary"))))]
pub const GL_NUM_PROGRAM_BINARY_FORMATS_OES: GLenum = 0x87FE;
#[doc = "`GL_NUM_SAMPLE_COUNTS: GLenum = 0x9380`"]
#[doc = "* **Group:** InternalFormatPName"]
pub const GL_NUM_SAMPLE_COUNTS: GLenum = 0x9380;
#[doc = "`GL_NUM_SHADER_BINARY_FORMATS: GLenum = 0x8DF9`"]
#[doc = "* **Group:** GetPName"]
pub const GL_NUM_SHADER_BINARY_FORMATS: GLenum = 0x8DF9;
#[doc = "`GL_NUM_SPARSE_LEVELS_EXT: GLenum = 0x91AA`"]
#[cfg(any(feature = "GL_EXT_sparse_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_sparse_texture"))))]
pub const GL_NUM_SPARSE_LEVELS_EXT: GLenum = 0x91AA;
#[doc = "`GL_NUM_SUPPORTED_MULTISAMPLE_MODES_AMD: GLenum = 0x91B6`"]
#[cfg(any(feature = "GL_AMD_framebuffer_multisample_advanced"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_AMD_framebuffer_multisample_advanced")))
)]
pub const GL_NUM_SUPPORTED_MULTISAMPLE_MODES_AMD: GLenum = 0x91B6;
#[doc = "`GL_NUM_TILING_TYPES_EXT: GLenum = 0x9582`"]
#[cfg(any(feature = "GL_EXT_memory_object"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_memory_object"))))]
pub const GL_NUM_TILING_TYPES_EXT: GLenum = 0x9582;
#[doc = "`GL_NUM_VIRTUAL_PAGE_SIZES_EXT: GLenum = 0x91A8`"]
#[cfg(any(feature = "GL_EXT_sparse_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_sparse_texture"))))]
pub const GL_NUM_VIRTUAL_PAGE_SIZES_EXT: GLenum = 0x91A8;
#[doc = "`GL_NUM_WINDOW_RECTANGLES_EXT: GLenum = 0x8F15`"]
#[cfg(any(feature = "GL_EXT_window_rectangles"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_window_rectangles"))))]
pub const GL_NUM_WINDOW_RECTANGLES_EXT: GLenum = 0x8F15;
#[doc = "`GL_OBJECT_TYPE: GLenum = 0x9112`"]
#[doc = "* **Group:** SyncParameterName"]
pub const GL_OBJECT_TYPE: GLenum = 0x9112;
#[doc = "`GL_OBJECT_TYPE_APPLE: GLenum = 0x9112`"]
#[cfg(any(feature = "GL_APPLE_sync"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_sync"))))]
pub const GL_OBJECT_TYPE_APPLE: GLenum = 0x9112;
#[doc = "`GL_OFFSET: GLenum = 0x92FC`"]
#[doc = "* **Group:** ProgramResourceProperty"]
pub const GL_OFFSET: GLenum = 0x92FC;
#[doc = "`GL_ONE: GLenum = 1`"]
#[doc = "* **Groups:** TextureSwizzle, BlendingFactor"]
pub const GL_ONE: GLenum = 1;
#[doc = "`GL_ONE_MINUS_CONSTANT_ALPHA: GLenum = 0x8004`"]
#[doc = "* **Group:** BlendingFactor"]
pub const GL_ONE_MINUS_CONSTANT_ALPHA: GLenum = 0x8004;
#[doc = "`GL_ONE_MINUS_CONSTANT_COLOR: GLenum = 0x8002`"]
#[doc = "* **Group:** BlendingFactor"]
pub const GL_ONE_MINUS_CONSTANT_COLOR: GLenum = 0x8002;
#[doc = "`GL_ONE_MINUS_DST_ALPHA: GLenum = 0x0305`"]
#[doc = "* **Group:** BlendingFactor"]
pub const GL_ONE_MINUS_DST_ALPHA: GLenum = 0x0305;
#[doc = "`GL_ONE_MINUS_DST_COLOR: GLenum = 0x0307`"]
#[doc = "* **Group:** BlendingFactor"]
pub const GL_ONE_MINUS_DST_COLOR: GLenum = 0x0307;
#[doc = "`GL_ONE_MINUS_SRC1_ALPHA_EXT: GLenum = 0x88FB`"]
#[cfg(any(feature = "GL_EXT_blend_func_extended"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_blend_func_extended"))))]
pub const GL_ONE_MINUS_SRC1_ALPHA_EXT: GLenum = 0x88FB;
#[doc = "`GL_ONE_MINUS_SRC1_COLOR_EXT: GLenum = 0x88FA`"]
#[cfg(any(feature = "GL_EXT_blend_func_extended"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_blend_func_extended"))))]
pub const GL_ONE_MINUS_SRC1_COLOR_EXT: GLenum = 0x88FA;
#[doc = "`GL_ONE_MINUS_SRC_ALPHA: GLenum = 0x0303`"]
#[doc = "* **Group:** BlendingFactor"]
pub const GL_ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
#[doc = "`GL_ONE_MINUS_SRC_COLOR: GLenum = 0x0301`"]
#[doc = "* **Group:** BlendingFactor"]
pub const GL_ONE_MINUS_SRC_COLOR: GLenum = 0x0301;
#[doc = "`GL_OPTIMAL_TILING_EXT: GLenum = 0x9584`"]
#[cfg(any(feature = "GL_EXT_memory_object"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_memory_object"))))]
pub const GL_OPTIMAL_TILING_EXT: GLenum = 0x9584;
#[doc = "`GL_OUT_OF_MEMORY: GLenum = 0x0505`"]
#[doc = "* **Group:** ErrorCode"]
pub const GL_OUT_OF_MEMORY: GLenum = 0x0505;
#[doc = "`GL_OVERLAY: GLenum = 0x9296`"]
pub const GL_OVERLAY: GLenum = 0x9296;
#[doc = "`GL_OVERLAY_KHR: GLenum = 0x9296`"]
#[cfg(any(feature = "GL_KHR_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_blend_equation_advanced"))))]
pub const GL_OVERLAY_KHR: GLenum = 0x9296;
#[doc = "`GL_OVERLAY_NV: GLenum = 0x9296`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_OVERLAY_NV: GLenum = 0x9296;
#[doc = "`GL_PACK_ALIGNMENT: GLenum = 0x0D05`"]
#[doc = "* **Groups:** PixelStoreParameter, GetPName"]
pub const GL_PACK_ALIGNMENT: GLenum = 0x0D05;
#[doc = "`GL_PACK_REVERSE_ROW_ORDER_ANGLE: GLenum = 0x93A4`"]
#[cfg(any(feature = "GL_ANGLE_pack_reverse_row_order"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_ANGLE_pack_reverse_row_order"))))]
pub const GL_PACK_REVERSE_ROW_ORDER_ANGLE: GLenum = 0x93A4;
#[doc = "`GL_PACK_ROW_LENGTH: GLenum = 0x0D02`"]
#[doc = "* **Groups:** PixelStoreParameter, GetPName"]
pub const GL_PACK_ROW_LENGTH: GLenum = 0x0D02;
#[doc = "`GL_PACK_SKIP_PIXELS: GLenum = 0x0D04`"]
#[doc = "* **Groups:** PixelStoreParameter, GetPName"]
pub const GL_PACK_SKIP_PIXELS: GLenum = 0x0D04;
#[doc = "`GL_PACK_SKIP_ROWS: GLenum = 0x0D03`"]
#[doc = "* **Groups:** PixelStoreParameter, GetPName"]
pub const GL_PACK_SKIP_ROWS: GLenum = 0x0D03;
#[doc = "`GL_PALETTE4_R5_G6_B5_OES: GLenum = 0x8B92`"]
#[cfg(any(feature = "GL_OES_compressed_paletted_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_compressed_paletted_texture"))))]
pub const GL_PALETTE4_R5_G6_B5_OES: GLenum = 0x8B92;
#[doc = "`GL_PALETTE4_RGB5_A1_OES: GLenum = 0x8B94`"]
#[cfg(any(feature = "GL_OES_compressed_paletted_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_compressed_paletted_texture"))))]
pub const GL_PALETTE4_RGB5_A1_OES: GLenum = 0x8B94;
#[doc = "`GL_PALETTE4_RGB8_OES: GLenum = 0x8B90`"]
#[cfg(any(feature = "GL_OES_compressed_paletted_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_compressed_paletted_texture"))))]
pub const GL_PALETTE4_RGB8_OES: GLenum = 0x8B90;
#[doc = "`GL_PALETTE4_RGBA4_OES: GLenum = 0x8B93`"]
#[cfg(any(feature = "GL_OES_compressed_paletted_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_compressed_paletted_texture"))))]
pub const GL_PALETTE4_RGBA4_OES: GLenum = 0x8B93;
#[doc = "`GL_PALETTE4_RGBA8_OES: GLenum = 0x8B91`"]
#[cfg(any(feature = "GL_OES_compressed_paletted_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_compressed_paletted_texture"))))]
pub const GL_PALETTE4_RGBA8_OES: GLenum = 0x8B91;
#[doc = "`GL_PALETTE8_R5_G6_B5_OES: GLenum = 0x8B97`"]
#[cfg(any(feature = "GL_OES_compressed_paletted_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_compressed_paletted_texture"))))]
pub const GL_PALETTE8_R5_G6_B5_OES: GLenum = 0x8B97;
#[doc = "`GL_PALETTE8_RGB5_A1_OES: GLenum = 0x8B99`"]
#[cfg(any(feature = "GL_OES_compressed_paletted_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_compressed_paletted_texture"))))]
pub const GL_PALETTE8_RGB5_A1_OES: GLenum = 0x8B99;
#[doc = "`GL_PALETTE8_RGB8_OES: GLenum = 0x8B95`"]
#[cfg(any(feature = "GL_OES_compressed_paletted_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_compressed_paletted_texture"))))]
pub const GL_PALETTE8_RGB8_OES: GLenum = 0x8B95;
#[doc = "`GL_PALETTE8_RGBA4_OES: GLenum = 0x8B98`"]
#[cfg(any(feature = "GL_OES_compressed_paletted_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_compressed_paletted_texture"))))]
pub const GL_PALETTE8_RGBA4_OES: GLenum = 0x8B98;
#[doc = "`GL_PALETTE8_RGBA8_OES: GLenum = 0x8B96`"]
#[cfg(any(feature = "GL_OES_compressed_paletted_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_compressed_paletted_texture"))))]
pub const GL_PALETTE8_RGBA8_OES: GLenum = 0x8B96;
#[doc = "`GL_PATCHES: GLenum = 0x000E`"]
#[doc = "* **Group:** PrimitiveType"]
pub const GL_PATCHES: GLenum = 0x000E;
#[doc = "`GL_PATCHES_EXT: GLenum = 0x000E`"]
#[doc = "* **Group:** PrimitiveType"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_PATCHES_EXT: GLenum = 0x000E;
#[doc = "`GL_PATCHES_OES: GLenum = 0x000E`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_PATCHES_OES: GLenum = 0x000E;
#[doc = "`GL_PATCH_VERTICES: GLenum = 0x8E72`"]
#[doc = "* **Group:** PatchParameterName"]
pub const GL_PATCH_VERTICES: GLenum = 0x8E72;
#[doc = "`GL_PATCH_VERTICES_EXT: GLenum = 0x8E72`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_PATCH_VERTICES_EXT: GLenum = 0x8E72;
#[doc = "`GL_PATCH_VERTICES_OES: GLenum = 0x8E72`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_PATCH_VERTICES_OES: GLenum = 0x8E72;
#[doc = "`GL_PATH_CLIENT_LENGTH_NV: GLenum = 0x907F`"]
#[doc = "* **Group:** PathParameter"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_CLIENT_LENGTH_NV: GLenum = 0x907F;
#[doc = "`GL_PATH_COMMAND_COUNT_NV: GLenum = 0x909D`"]
#[doc = "* **Group:** PathParameter"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_COMMAND_COUNT_NV: GLenum = 0x909D;
#[doc = "`GL_PATH_COMPUTED_LENGTH_NV: GLenum = 0x90A0`"]
#[doc = "* **Group:** PathParameter"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_COMPUTED_LENGTH_NV: GLenum = 0x90A0;
#[doc = "`GL_PATH_COORD_COUNT_NV: GLenum = 0x909E`"]
#[doc = "* **Group:** PathParameter"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_COORD_COUNT_NV: GLenum = 0x909E;
#[doc = "`GL_PATH_COVER_DEPTH_FUNC_NV: GLenum = 0x90BF`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_COVER_DEPTH_FUNC_NV: GLenum = 0x90BF;
#[doc = "`GL_PATH_DASH_ARRAY_COUNT_NV: GLenum = 0x909F`"]
#[doc = "* **Group:** PathParameter"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_DASH_ARRAY_COUNT_NV: GLenum = 0x909F;
#[doc = "`GL_PATH_DASH_CAPS_NV: GLenum = 0x907B`"]
#[doc = "* **Group:** PathParameter"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_DASH_CAPS_NV: GLenum = 0x907B;
#[doc = "`GL_PATH_DASH_OFFSET_NV: GLenum = 0x907E`"]
#[doc = "* **Group:** PathParameter"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_DASH_OFFSET_NV: GLenum = 0x907E;
#[doc = "`GL_PATH_DASH_OFFSET_RESET_NV: GLenum = 0x90B4`"]
#[doc = "* **Group:** PathParameter"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_DASH_OFFSET_RESET_NV: GLenum = 0x90B4;
#[doc = "`GL_PATH_END_CAPS_NV: GLenum = 0x9076`"]
#[doc = "* **Group:** PathParameter"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_END_CAPS_NV: GLenum = 0x9076;
#[doc = "`GL_PATH_ERROR_POSITION_NV: GLenum = 0x90AB`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_ERROR_POSITION_NV: GLenum = 0x90AB;
#[doc = "`GL_PATH_FILL_BOUNDING_BOX_NV: GLenum = 0x90A1`"]
#[doc = "* **Group:** PathParameter"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_FILL_BOUNDING_BOX_NV: GLenum = 0x90A1;
#[doc = "`GL_PATH_FILL_COVER_MODE_NV: GLenum = 0x9082`"]
#[doc = "* **Groups:** PathCoverMode, PathParameter"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_FILL_COVER_MODE_NV: GLenum = 0x9082;
#[doc = "`GL_PATH_FILL_MASK_NV: GLenum = 0x9081`"]
#[doc = "* **Group:** PathParameter"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_FILL_MASK_NV: GLenum = 0x9081;
#[doc = "`GL_PATH_FILL_MODE_NV: GLenum = 0x9080`"]
#[doc = "* **Groups:** PathParameter, PathFillMode"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_FILL_MODE_NV: GLenum = 0x9080;
#[doc = "`GL_PATH_FORMAT_PS_NV: GLenum = 0x9071`"]
#[doc = "* **Group:** PathStringFormat"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_FORMAT_PS_NV: GLenum = 0x9071;
#[doc = "`GL_PATH_FORMAT_SVG_NV: GLenum = 0x9070`"]
#[doc = "* **Group:** PathStringFormat"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_FORMAT_SVG_NV: GLenum = 0x9070;
#[doc = "`GL_PATH_GEN_COEFF_NV: GLenum = 0x90B1`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_GEN_COEFF_NV: GLenum = 0x90B1;
#[doc = "`GL_PATH_GEN_COMPONENTS_NV: GLenum = 0x90B3`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_GEN_COMPONENTS_NV: GLenum = 0x90B3;
#[doc = "`GL_PATH_GEN_MODE_NV: GLenum = 0x90B0`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_GEN_MODE_NV: GLenum = 0x90B0;
#[doc = "`GL_PATH_INITIAL_DASH_CAP_NV: GLenum = 0x907C`"]
#[doc = "* **Group:** PathParameter"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_INITIAL_DASH_CAP_NV: GLenum = 0x907C;
#[doc = "`GL_PATH_INITIAL_END_CAP_NV: GLenum = 0x9077`"]
#[doc = "* **Group:** PathParameter"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_INITIAL_END_CAP_NV: GLenum = 0x9077;
#[doc = "`GL_PATH_JOIN_STYLE_NV: GLenum = 0x9079`"]
#[doc = "* **Group:** PathParameter"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_JOIN_STYLE_NV: GLenum = 0x9079;
#[doc = "`GL_PATH_MAX_MODELVIEW_STACK_DEPTH_NV: GLenum = 0x0D36`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_MAX_MODELVIEW_STACK_DEPTH_NV: GLenum = 0x0D36;
#[doc = "`GL_PATH_MAX_PROJECTION_STACK_DEPTH_NV: GLenum = 0x0D38`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_MAX_PROJECTION_STACK_DEPTH_NV: GLenum = 0x0D38;
#[doc = "`GL_PATH_MITER_LIMIT_NV: GLenum = 0x907A`"]
#[doc = "* **Group:** PathParameter"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_MITER_LIMIT_NV: GLenum = 0x907A;
#[doc = "`GL_PATH_MODELVIEW_MATRIX_NV: GLenum = 0x0BA6`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_MODELVIEW_MATRIX_NV: GLenum = 0x0BA6;
#[doc = "`GL_PATH_MODELVIEW_NV: GLenum = 0x1700`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_MODELVIEW_NV: GLenum = 0x1700;
#[doc = "`GL_PATH_MODELVIEW_STACK_DEPTH_NV: GLenum = 0x0BA3`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_MODELVIEW_STACK_DEPTH_NV: GLenum = 0x0BA3;
#[doc = "`GL_PATH_OBJECT_BOUNDING_BOX_NV: GLenum = 0x908A`"]
#[doc = "* **Groups:** PathGenMode, PathParameter"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_OBJECT_BOUNDING_BOX_NV: GLenum = 0x908A;
#[doc = "`GL_PATH_PROJECTION_MATRIX_NV: GLenum = 0x0BA7`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_PROJECTION_MATRIX_NV: GLenum = 0x0BA7;
#[doc = "`GL_PATH_PROJECTION_NV: GLenum = 0x1701`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_PROJECTION_NV: GLenum = 0x1701;
#[doc = "`GL_PATH_PROJECTION_STACK_DEPTH_NV: GLenum = 0x0BA4`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_PROJECTION_STACK_DEPTH_NV: GLenum = 0x0BA4;
#[doc = "`GL_PATH_STENCIL_DEPTH_OFFSET_FACTOR_NV: GLenum = 0x90BD`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_STENCIL_DEPTH_OFFSET_FACTOR_NV: GLenum = 0x90BD;
#[doc = "`GL_PATH_STENCIL_DEPTH_OFFSET_UNITS_NV: GLenum = 0x90BE`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_STENCIL_DEPTH_OFFSET_UNITS_NV: GLenum = 0x90BE;
#[doc = "`GL_PATH_STENCIL_FUNC_NV: GLenum = 0x90B7`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_STENCIL_FUNC_NV: GLenum = 0x90B7;
#[doc = "`GL_PATH_STENCIL_REF_NV: GLenum = 0x90B8`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_STENCIL_REF_NV: GLenum = 0x90B8;
#[doc = "`GL_PATH_STENCIL_VALUE_MASK_NV: GLenum = 0x90B9`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_STENCIL_VALUE_MASK_NV: GLenum = 0x90B9;
#[doc = "`GL_PATH_STROKE_BOUNDING_BOX_NV: GLenum = 0x90A2`"]
#[doc = "* **Group:** PathParameter"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_STROKE_BOUNDING_BOX_NV: GLenum = 0x90A2;
#[doc = "`GL_PATH_STROKE_COVER_MODE_NV: GLenum = 0x9083`"]
#[doc = "* **Group:** PathParameter"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_STROKE_COVER_MODE_NV: GLenum = 0x9083;
#[doc = "`GL_PATH_STROKE_MASK_NV: GLenum = 0x9084`"]
#[doc = "* **Group:** PathParameter"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_STROKE_MASK_NV: GLenum = 0x9084;
#[doc = "`GL_PATH_STROKE_WIDTH_NV: GLenum = 0x9075`"]
#[doc = "* **Group:** PathParameter"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_STROKE_WIDTH_NV: GLenum = 0x9075;
#[doc = "`GL_PATH_TERMINAL_DASH_CAP_NV: GLenum = 0x907D`"]
#[doc = "* **Group:** PathParameter"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_TERMINAL_DASH_CAP_NV: GLenum = 0x907D;
#[doc = "`GL_PATH_TERMINAL_END_CAP_NV: GLenum = 0x9078`"]
#[doc = "* **Group:** PathParameter"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_TERMINAL_END_CAP_NV: GLenum = 0x9078;
#[doc = "`GL_PATH_TRANSPOSE_MODELVIEW_MATRIX_NV: GLenum = 0x84E3`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_TRANSPOSE_MODELVIEW_MATRIX_NV: GLenum = 0x84E3;
#[doc = "`GL_PATH_TRANSPOSE_PROJECTION_MATRIX_NV: GLenum = 0x84E4`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_PATH_TRANSPOSE_PROJECTION_MATRIX_NV: GLenum = 0x84E4;
#[doc = "`GL_PERCENTAGE_AMD: GLenum = 0x8BC3`"]
#[cfg(any(feature = "GL_AMD_performance_monitor"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_AMD_performance_monitor"))))]
pub const GL_PERCENTAGE_AMD: GLenum = 0x8BC3;
#[doc = "`GL_PERFMON_RESULT_AMD: GLenum = 0x8BC6`"]
#[cfg(any(feature = "GL_AMD_performance_monitor"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_AMD_performance_monitor"))))]
pub const GL_PERFMON_RESULT_AMD: GLenum = 0x8BC6;
#[doc = "`GL_PERFMON_RESULT_AVAILABLE_AMD: GLenum = 0x8BC4`"]
#[cfg(any(feature = "GL_AMD_performance_monitor"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_AMD_performance_monitor"))))]
pub const GL_PERFMON_RESULT_AVAILABLE_AMD: GLenum = 0x8BC4;
#[doc = "`GL_PERFMON_RESULT_SIZE_AMD: GLenum = 0x8BC5`"]
#[cfg(any(feature = "GL_AMD_performance_monitor"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_AMD_performance_monitor"))))]
pub const GL_PERFMON_RESULT_SIZE_AMD: GLenum = 0x8BC5;
#[doc = "`GL_PERFQUERY_COUNTER_DATA_BOOL32_INTEL: GLenum = 0x94FC`"]
#[cfg(any(feature = "GL_INTEL_performance_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_INTEL_performance_query"))))]
pub const GL_PERFQUERY_COUNTER_DATA_BOOL32_INTEL: GLenum = 0x94FC;
#[doc = "`GL_PERFQUERY_COUNTER_DATA_DOUBLE_INTEL: GLenum = 0x94FB`"]
#[cfg(any(feature = "GL_INTEL_performance_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_INTEL_performance_query"))))]
pub const GL_PERFQUERY_COUNTER_DATA_DOUBLE_INTEL: GLenum = 0x94FB;
#[doc = "`GL_PERFQUERY_COUNTER_DATA_FLOAT_INTEL: GLenum = 0x94FA`"]
#[cfg(any(feature = "GL_INTEL_performance_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_INTEL_performance_query"))))]
pub const GL_PERFQUERY_COUNTER_DATA_FLOAT_INTEL: GLenum = 0x94FA;
#[doc = "`GL_PERFQUERY_COUNTER_DATA_UINT32_INTEL: GLenum = 0x94F8`"]
#[cfg(any(feature = "GL_INTEL_performance_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_INTEL_performance_query"))))]
pub const GL_PERFQUERY_COUNTER_DATA_UINT32_INTEL: GLenum = 0x94F8;
#[doc = "`GL_PERFQUERY_COUNTER_DATA_UINT64_INTEL: GLenum = 0x94F9`"]
#[cfg(any(feature = "GL_INTEL_performance_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_INTEL_performance_query"))))]
pub const GL_PERFQUERY_COUNTER_DATA_UINT64_INTEL: GLenum = 0x94F9;
#[doc = "`GL_PERFQUERY_COUNTER_DESC_LENGTH_MAX_INTEL: GLenum = 0x94FF`"]
#[cfg(any(feature = "GL_INTEL_performance_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_INTEL_performance_query"))))]
pub const GL_PERFQUERY_COUNTER_DESC_LENGTH_MAX_INTEL: GLenum = 0x94FF;
#[doc = "`GL_PERFQUERY_COUNTER_DURATION_NORM_INTEL: GLenum = 0x94F1`"]
#[cfg(any(feature = "GL_INTEL_performance_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_INTEL_performance_query"))))]
pub const GL_PERFQUERY_COUNTER_DURATION_NORM_INTEL: GLenum = 0x94F1;
#[doc = "`GL_PERFQUERY_COUNTER_DURATION_RAW_INTEL: GLenum = 0x94F2`"]
#[cfg(any(feature = "GL_INTEL_performance_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_INTEL_performance_query"))))]
pub const GL_PERFQUERY_COUNTER_DURATION_RAW_INTEL: GLenum = 0x94F2;
#[doc = "`GL_PERFQUERY_COUNTER_EVENT_INTEL: GLenum = 0x94F0`"]
#[cfg(any(feature = "GL_INTEL_performance_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_INTEL_performance_query"))))]
pub const GL_PERFQUERY_COUNTER_EVENT_INTEL: GLenum = 0x94F0;
#[doc = "`GL_PERFQUERY_COUNTER_NAME_LENGTH_MAX_INTEL: GLenum = 0x94FE`"]
#[cfg(any(feature = "GL_INTEL_performance_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_INTEL_performance_query"))))]
pub const GL_PERFQUERY_COUNTER_NAME_LENGTH_MAX_INTEL: GLenum = 0x94FE;
#[doc = "`GL_PERFQUERY_COUNTER_RAW_INTEL: GLenum = 0x94F4`"]
#[cfg(any(feature = "GL_INTEL_performance_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_INTEL_performance_query"))))]
pub const GL_PERFQUERY_COUNTER_RAW_INTEL: GLenum = 0x94F4;
#[doc = "`GL_PERFQUERY_COUNTER_THROUGHPUT_INTEL: GLenum = 0x94F3`"]
#[cfg(any(feature = "GL_INTEL_performance_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_INTEL_performance_query"))))]
pub const GL_PERFQUERY_COUNTER_THROUGHPUT_INTEL: GLenum = 0x94F3;
#[doc = "`GL_PERFQUERY_COUNTER_TIMESTAMP_INTEL: GLenum = 0x94F5`"]
#[cfg(any(feature = "GL_INTEL_performance_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_INTEL_performance_query"))))]
pub const GL_PERFQUERY_COUNTER_TIMESTAMP_INTEL: GLenum = 0x94F5;
#[doc = "`GL_PERFQUERY_DONOT_FLUSH_INTEL: GLenum = 0x83F9`"]
#[cfg(any(feature = "GL_INTEL_performance_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_INTEL_performance_query"))))]
pub const GL_PERFQUERY_DONOT_FLUSH_INTEL: GLenum = 0x83F9;
#[doc = "`GL_PERFQUERY_FLUSH_INTEL: GLenum = 0x83FA`"]
#[cfg(any(feature = "GL_INTEL_performance_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_INTEL_performance_query"))))]
pub const GL_PERFQUERY_FLUSH_INTEL: GLenum = 0x83FA;
#[doc = "`GL_PERFQUERY_GLOBAL_CONTEXT_INTEL: GLbitfield = 0x00000001`"]
#[doc = "* **Group:** PerformanceQueryCapsMaskINTEL"]
#[cfg(any(feature = "GL_INTEL_performance_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_INTEL_performance_query"))))]
pub const GL_PERFQUERY_GLOBAL_CONTEXT_INTEL: GLbitfield = 0x00000001;
#[doc = "`GL_PERFQUERY_GPA_EXTENDED_COUNTERS_INTEL: GLenum = 0x9500`"]
#[cfg(any(feature = "GL_INTEL_performance_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_INTEL_performance_query"))))]
pub const GL_PERFQUERY_GPA_EXTENDED_COUNTERS_INTEL: GLenum = 0x9500;
#[doc = "`GL_PERFQUERY_QUERY_NAME_LENGTH_MAX_INTEL: GLenum = 0x94FD`"]
#[cfg(any(feature = "GL_INTEL_performance_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_INTEL_performance_query"))))]
pub const GL_PERFQUERY_QUERY_NAME_LENGTH_MAX_INTEL: GLenum = 0x94FD;
#[doc = "`GL_PERFQUERY_SINGLE_CONTEXT_INTEL: GLbitfield = 0x00000000`"]
#[doc = "* **Group:** PerformanceQueryCapsMaskINTEL"]
#[cfg(any(feature = "GL_INTEL_performance_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_INTEL_performance_query"))))]
pub const GL_PERFQUERY_SINGLE_CONTEXT_INTEL: GLbitfield = 0x00000000;
#[doc = "`GL_PERFQUERY_WAIT_INTEL: GLenum = 0x83FB`"]
#[cfg(any(feature = "GL_INTEL_performance_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_INTEL_performance_query"))))]
pub const GL_PERFQUERY_WAIT_INTEL: GLenum = 0x83FB;
#[doc = "`GL_PINLIGHT_NV: GLenum = 0x92A8`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_PINLIGHT_NV: GLenum = 0x92A8;
#[doc = "`GL_PIXEL_BUFFER_BARRIER_BIT: GLbitfield = 0x00000080`"]
#[doc = "* **Group:** MemoryBarrierMask"]
pub const GL_PIXEL_BUFFER_BARRIER_BIT: GLbitfield = 0x00000080;
#[doc = "`GL_PIXEL_PACK_BUFFER: GLenum = 0x88EB`"]
#[doc = "* **Groups:** CopyBufferSubDataTarget, BufferTargetARB, BufferStorageTarget"]
pub const GL_PIXEL_PACK_BUFFER: GLenum = 0x88EB;
#[doc = "`GL_PIXEL_PACK_BUFFER_BINDING: GLenum = 0x88ED`"]
#[doc = "* **Group:** GetPName"]
pub const GL_PIXEL_PACK_BUFFER_BINDING: GLenum = 0x88ED;
#[doc = "`GL_PIXEL_PACK_BUFFER_BINDING_NV: GLenum = 0x88ED`"]
#[cfg(any(feature = "GL_NV_pixel_buffer_object"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_pixel_buffer_object"))))]
pub const GL_PIXEL_PACK_BUFFER_BINDING_NV: GLenum = 0x88ED;
#[doc = "`GL_PIXEL_PACK_BUFFER_NV: GLenum = 0x88EB`"]
#[cfg(any(feature = "GL_NV_pixel_buffer_object"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_pixel_buffer_object"))))]
pub const GL_PIXEL_PACK_BUFFER_NV: GLenum = 0x88EB;
#[doc = "`GL_PIXEL_UNPACK_BUFFER: GLenum = 0x88EC`"]
#[doc = "* **Groups:** CopyBufferSubDataTarget, BufferTargetARB, BufferStorageTarget"]
pub const GL_PIXEL_UNPACK_BUFFER: GLenum = 0x88EC;
#[doc = "`GL_PIXEL_UNPACK_BUFFER_BINDING: GLenum = 0x88EF`"]
#[doc = "* **Group:** GetPName"]
pub const GL_PIXEL_UNPACK_BUFFER_BINDING: GLenum = 0x88EF;
#[doc = "`GL_PIXEL_UNPACK_BUFFER_BINDING_NV: GLenum = 0x88EF`"]
#[cfg(any(feature = "GL_NV_pixel_buffer_object"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_pixel_buffer_object"))))]
pub const GL_PIXEL_UNPACK_BUFFER_BINDING_NV: GLenum = 0x88EF;
#[doc = "`GL_PIXEL_UNPACK_BUFFER_NV: GLenum = 0x88EC`"]
#[cfg(any(feature = "GL_NV_pixel_buffer_object"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_pixel_buffer_object"))))]
pub const GL_PIXEL_UNPACK_BUFFER_NV: GLenum = 0x88EC;
#[doc = "`GL_PLUS_CLAMPED_ALPHA_NV: GLenum = 0x92B2`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_PLUS_CLAMPED_ALPHA_NV: GLenum = 0x92B2;
#[doc = "`GL_PLUS_CLAMPED_NV: GLenum = 0x92B1`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_PLUS_CLAMPED_NV: GLenum = 0x92B1;
#[doc = "`GL_PLUS_DARKER_NV: GLenum = 0x9292`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_PLUS_DARKER_NV: GLenum = 0x9292;
#[doc = "`GL_PLUS_NV: GLenum = 0x9291`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_PLUS_NV: GLenum = 0x9291;
#[doc = "`GL_POINTS: GLenum = 0x0000`"]
#[doc = "* **Group:** PrimitiveType"]
pub const GL_POINTS: GLenum = 0x0000;
#[doc = "`GL_POINT_NV: GLenum = 0x1B00`"]
#[cfg(any(feature = "GL_NV_polygon_mode"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_polygon_mode"))))]
pub const GL_POINT_NV: GLenum = 0x1B00;
#[doc = "`GL_POLYGON_MODE_NV: GLenum = 0x0B40`"]
#[cfg(any(feature = "GL_NV_polygon_mode"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_polygon_mode"))))]
pub const GL_POLYGON_MODE_NV: GLenum = 0x0B40;
#[doc = "`GL_POLYGON_OFFSET_CLAMP_EXT: GLenum = 0x8E1B`"]
#[doc = "* **Alias Of:** `GL_POLYGON_OFFSET_CLAMP`"]
#[cfg(any(feature = "GL_EXT_polygon_offset_clamp"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_polygon_offset_clamp"))))]
pub const GL_POLYGON_OFFSET_CLAMP_EXT: GLenum = 0x8E1B;
#[doc = "`GL_POLYGON_OFFSET_FACTOR: GLenum = 0x8038`"]
#[doc = "* **Group:** GetPName"]
pub const GL_POLYGON_OFFSET_FACTOR: GLenum = 0x8038;
#[doc = "`GL_POLYGON_OFFSET_FILL: GLenum = 0x8037`"]
#[doc = "* **Groups:** GetPName, EnableCap"]
pub const GL_POLYGON_OFFSET_FILL: GLenum = 0x8037;
#[doc = "`GL_POLYGON_OFFSET_LINE_NV: GLenum = 0x2A02`"]
#[cfg(any(feature = "GL_NV_polygon_mode"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_polygon_mode"))))]
pub const GL_POLYGON_OFFSET_LINE_NV: GLenum = 0x2A02;
#[doc = "`GL_POLYGON_OFFSET_POINT_NV: GLenum = 0x2A01`"]
#[cfg(any(feature = "GL_NV_polygon_mode"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_polygon_mode"))))]
pub const GL_POLYGON_OFFSET_POINT_NV: GLenum = 0x2A01;
#[doc = "`GL_POLYGON_OFFSET_UNITS: GLenum = 0x2A00`"]
#[doc = "* **Group:** GetPName"]
pub const GL_POLYGON_OFFSET_UNITS: GLenum = 0x2A00;
#[doc = "`GL_PRIMITIVES_GENERATED: GLenum = 0x8C87`"]
#[doc = "* **Group:** QueryTarget"]
pub const GL_PRIMITIVES_GENERATED: GLenum = 0x8C87;
#[doc = "`GL_PRIMITIVES_GENERATED_EXT: GLenum = 0x8C87`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_PRIMITIVES_GENERATED_EXT: GLenum = 0x8C87;
#[doc = "`GL_PRIMITIVES_GENERATED_OES: GLenum = 0x8C87`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_PRIMITIVES_GENERATED_OES: GLenum = 0x8C87;
#[doc = "`GL_PRIMITIVE_BOUNDING_BOX: GLenum = 0x92BE`"]
pub const GL_PRIMITIVE_BOUNDING_BOX: GLenum = 0x92BE;
#[doc = "`GL_PRIMITIVE_BOUNDING_BOX_EXT: GLenum = 0x92BE`"]
#[cfg(any(feature = "GL_EXT_primitive_bounding_box"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_primitive_bounding_box"))))]
pub const GL_PRIMITIVE_BOUNDING_BOX_EXT: GLenum = 0x92BE;
#[doc = "`GL_PRIMITIVE_BOUNDING_BOX_OES: GLenum = 0x92BE`"]
#[cfg(any(feature = "GL_OES_primitive_bounding_box"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_primitive_bounding_box"))))]
pub const GL_PRIMITIVE_BOUNDING_BOX_OES: GLenum = 0x92BE;
#[doc = "`GL_PRIMITIVE_RESTART_FIXED_INDEX: GLenum = 0x8D69`"]
#[doc = "* **Group:** EnableCap"]
pub const GL_PRIMITIVE_RESTART_FIXED_INDEX: GLenum = 0x8D69;
#[doc = "`GL_PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: GLenum = 0x8221`"]
pub const GL_PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: GLenum = 0x8221;
#[doc = "`GL_PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED_OES: GLenum = 0x8221`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED_OES: GLenum = 0x8221;
#[doc = "`GL_PROGRAM: GLenum = 0x82E2`"]
#[doc = "* **Group:** ObjectIdentifier"]
pub const GL_PROGRAM: GLenum = 0x82E2;
#[doc = "`GL_PROGRAMMABLE_SAMPLE_LOCATION_NV: GLenum = 0x9341`"]
#[cfg(any(feature = "GL_NV_sample_locations"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_sample_locations"))))]
pub const GL_PROGRAMMABLE_SAMPLE_LOCATION_NV: GLenum = 0x9341;
#[doc = "`GL_PROGRAMMABLE_SAMPLE_LOCATION_TABLE_SIZE_NV: GLenum = 0x9340`"]
#[cfg(any(feature = "GL_NV_sample_locations"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_sample_locations"))))]
pub const GL_PROGRAMMABLE_SAMPLE_LOCATION_TABLE_SIZE_NV: GLenum = 0x9340;
#[doc = "`GL_PROGRAM_BINARY_ANGLE: GLenum = 0x93A6`"]
#[cfg(any(feature = "GL_ANGLE_program_binary"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_ANGLE_program_binary"))))]
pub const GL_PROGRAM_BINARY_ANGLE: GLenum = 0x93A6;
#[doc = "`GL_PROGRAM_BINARY_FORMATS: GLenum = 0x87FF`"]
#[doc = "* **Group:** GetPName"]
pub const GL_PROGRAM_BINARY_FORMATS: GLenum = 0x87FF;
#[doc = "`GL_PROGRAM_BINARY_FORMATS_OES: GLenum = 0x87FF`"]
#[cfg(any(feature = "GL_OES_get_program_binary"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_get_program_binary"))))]
pub const GL_PROGRAM_BINARY_FORMATS_OES: GLenum = 0x87FF;
#[doc = "`GL_PROGRAM_BINARY_FORMAT_MESA: GLenum = 0x875F`"]
#[cfg(any(feature = "GL_MESA_program_binary_formats"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_MESA_program_binary_formats"))))]
pub const GL_PROGRAM_BINARY_FORMAT_MESA: GLenum = 0x875F;
#[doc = "`GL_PROGRAM_BINARY_LENGTH: GLenum = 0x8741`"]
#[doc = "* **Group:** ProgramPropertyARB"]
pub const GL_PROGRAM_BINARY_LENGTH: GLenum = 0x8741;
#[doc = "`GL_PROGRAM_BINARY_LENGTH_OES: GLenum = 0x8741`"]
#[cfg(any(feature = "GL_OES_get_program_binary"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_get_program_binary"))))]
pub const GL_PROGRAM_BINARY_LENGTH_OES: GLenum = 0x8741;
#[doc = "`GL_PROGRAM_BINARY_RETRIEVABLE_HINT: GLenum = 0x8257`"]
#[doc = "* **Groups:** ProgramParameterPName, HintTarget"]
pub const GL_PROGRAM_BINARY_RETRIEVABLE_HINT: GLenum = 0x8257;
#[doc = "`GL_PROGRAM_INPUT: GLenum = 0x92E3`"]
#[doc = "* **Group:** ProgramInterface"]
pub const GL_PROGRAM_INPUT: GLenum = 0x92E3;
#[doc = "`GL_PROGRAM_KHR: GLenum = 0x82E2`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_PROGRAM_KHR: GLenum = 0x82E2;
#[doc = "`GL_PROGRAM_OBJECT_EXT: GLenum = 0x8B40`"]
#[cfg(any(feature = "GL_EXT_debug_label"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_debug_label"))))]
pub const GL_PROGRAM_OBJECT_EXT: GLenum = 0x8B40;
#[doc = "`GL_PROGRAM_OUTPUT: GLenum = 0x92E4`"]
#[doc = "* **Group:** ProgramInterface"]
pub const GL_PROGRAM_OUTPUT: GLenum = 0x92E4;
#[doc = "`GL_PROGRAM_PIPELINE: GLenum = 0x82E4`"]
#[doc = "* **Group:** ObjectIdentifier"]
pub const GL_PROGRAM_PIPELINE: GLenum = 0x82E4;
#[doc = "`GL_PROGRAM_PIPELINE_BINDING: GLenum = 0x825A`"]
#[doc = "* **Group:** GetPName"]
pub const GL_PROGRAM_PIPELINE_BINDING: GLenum = 0x825A;
#[doc = "`GL_PROGRAM_PIPELINE_BINDING_EXT: GLenum = 0x825A`"]
#[cfg(any(feature = "GL_EXT_separate_shader_objects"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_separate_shader_objects"))))]
pub const GL_PROGRAM_PIPELINE_BINDING_EXT: GLenum = 0x825A;
#[doc = "`GL_PROGRAM_PIPELINE_KHR: GLenum = 0x82E4`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_PROGRAM_PIPELINE_KHR: GLenum = 0x82E4;
#[doc = "`GL_PROGRAM_PIPELINE_OBJECT_EXT: GLenum = 0x8A4F`"]
#[cfg(any(feature = "GL_EXT_debug_label"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_debug_label"))))]
pub const GL_PROGRAM_PIPELINE_OBJECT_EXT: GLenum = 0x8A4F;
#[doc = "`GL_PROGRAM_SEPARABLE: GLenum = 0x8258`"]
#[doc = "* **Group:** ProgramParameterPName"]
pub const GL_PROGRAM_SEPARABLE: GLenum = 0x8258;
#[doc = "`GL_PROGRAM_SEPARABLE_EXT: GLenum = 0x8258`"]
#[cfg(any(feature = "GL_EXT_separate_shader_objects"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_separate_shader_objects"))))]
pub const GL_PROGRAM_SEPARABLE_EXT: GLenum = 0x8258;
#[doc = "`GL_PROTECTED_MEMORY_OBJECT_EXT: GLenum = 0x959B`"]
#[doc = "* **Group:** MemoryObjectParameterName"]
#[cfg(any(feature = "GL_EXT_memory_object"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_memory_object"))))]
pub const GL_PROTECTED_MEMORY_OBJECT_EXT: GLenum = 0x959B;
#[doc = "`GL_QUADRATIC_CURVE_TO_NV: GLenum = 0x0A`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_QUADRATIC_CURVE_TO_NV: GLenum = 0x0A;
#[doc = "`GL_QUADS: GLenum = 0x0007`"]
#[doc = "* **Group:** PrimitiveType"]
pub const GL_QUADS: GLenum = 0x0007;
#[doc = "`GL_QUADS_EXT: GLenum = 0x0007`"]
#[doc = "* **Group:** PrimitiveType"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_QUADS_EXT: GLenum = 0x0007;
#[doc = "`GL_QUADS_OES: GLenum = 0x0007`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_QUADS_OES: GLenum = 0x0007;
#[doc = "`GL_QUERY: GLenum = 0x82E3`"]
#[doc = "* **Group:** ObjectIdentifier"]
pub const GL_QUERY: GLenum = 0x82E3;
#[doc = "`GL_QUERY_BY_REGION_NO_WAIT_NV: GLenum = 0x8E16`"]
#[cfg(any(feature = "GL_NV_conditional_render"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_conditional_render"))))]
pub const GL_QUERY_BY_REGION_NO_WAIT_NV: GLenum = 0x8E16;
#[doc = "`GL_QUERY_BY_REGION_WAIT_NV: GLenum = 0x8E15`"]
#[cfg(any(feature = "GL_NV_conditional_render"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_conditional_render"))))]
pub const GL_QUERY_BY_REGION_WAIT_NV: GLenum = 0x8E15;
#[doc = "`GL_QUERY_COUNTER_BITS_EXT: GLenum = 0x8864`"]
#[cfg(any(feature = "GL_EXT_disjoint_timer_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_disjoint_timer_query"))))]
pub const GL_QUERY_COUNTER_BITS_EXT: GLenum = 0x8864;
#[doc = "`GL_QUERY_KHR: GLenum = 0x82E3`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_QUERY_KHR: GLenum = 0x82E3;
#[doc = "`GL_QUERY_NO_WAIT_NV: GLenum = 0x8E14`"]
#[cfg(any(feature = "GL_NV_conditional_render"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_conditional_render"))))]
pub const GL_QUERY_NO_WAIT_NV: GLenum = 0x8E14;
#[doc = "`GL_QUERY_OBJECT_EXT: GLenum = 0x9153`"]
#[cfg(any(feature = "GL_EXT_debug_label"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_debug_label"))))]
pub const GL_QUERY_OBJECT_EXT: GLenum = 0x9153;
#[doc = "`GL_QUERY_RESULT: GLenum = 0x8866`"]
#[doc = "* **Group:** QueryObjectParameterName"]
pub const GL_QUERY_RESULT: GLenum = 0x8866;
#[doc = "`GL_QUERY_RESULT_AVAILABLE: GLenum = 0x8867`"]
#[doc = "* **Group:** QueryObjectParameterName"]
pub const GL_QUERY_RESULT_AVAILABLE: GLenum = 0x8867;
#[doc = "`GL_QUERY_RESULT_AVAILABLE_EXT: GLenum = 0x8867`"]
#[cfg(any(
    feature = "GL_EXT_disjoint_timer_query",
    feature = "GL_EXT_occlusion_query_boolean"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_disjoint_timer_query",
        feature = "GL_EXT_occlusion_query_boolean"
    )))
)]
pub const GL_QUERY_RESULT_AVAILABLE_EXT: GLenum = 0x8867;
#[doc = "`GL_QUERY_RESULT_EXT: GLenum = 0x8866`"]
#[cfg(any(
    feature = "GL_EXT_disjoint_timer_query",
    feature = "GL_EXT_occlusion_query_boolean"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_disjoint_timer_query",
        feature = "GL_EXT_occlusion_query_boolean"
    )))
)]
pub const GL_QUERY_RESULT_EXT: GLenum = 0x8866;
#[doc = "`GL_QUERY_WAIT_NV: GLenum = 0x8E13`"]
#[cfg(any(feature = "GL_NV_conditional_render"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_conditional_render"))))]
pub const GL_QUERY_WAIT_NV: GLenum = 0x8E13;
#[doc = "`GL_R11F_G11F_B10F: GLenum = 0x8C3A`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_R11F_G11F_B10F: GLenum = 0x8C3A;
#[doc = "`GL_R11F_G11F_B10F_APPLE: GLenum = 0x8C3A`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_APPLE_texture_packed_float"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_texture_packed_float"))))]
pub const GL_R11F_G11F_B10F_APPLE: GLenum = 0x8C3A;
#[doc = "`GL_R16F: GLenum = 0x822D`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_R16F: GLenum = 0x822D;
#[doc = "`GL_R16F_EXT: GLenum = 0x822D`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_EXT_color_buffer_half_float",
    feature = "GL_EXT_texture_storage"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_color_buffer_half_float",
        feature = "GL_EXT_texture_storage"
    )))
)]
pub const GL_R16F_EXT: GLenum = 0x822D;
#[doc = "`GL_R16I: GLenum = 0x8233`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_R16I: GLenum = 0x8233;
#[doc = "`GL_R16UI: GLenum = 0x8234`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_R16UI: GLenum = 0x8234;
#[doc = "`GL_R16_EXT: GLenum = 0x822A`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_norm16"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_norm16"))))]
pub const GL_R16_EXT: GLenum = 0x822A;
#[doc = "`GL_R16_SNORM_EXT: GLenum = 0x8F98`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_render_snorm", feature = "GL_EXT_texture_norm16"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_render_snorm", feature = "GL_EXT_texture_norm16")))
)]
pub const GL_R16_SNORM_EXT: GLenum = 0x8F98;
#[doc = "`GL_R32F: GLenum = 0x822E`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_R32F: GLenum = 0x822E;
#[doc = "`GL_R32F_EXT: GLenum = 0x822E`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_storage"))))]
pub const GL_R32F_EXT: GLenum = 0x822E;
#[doc = "`GL_R32I: GLenum = 0x8235`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_R32I: GLenum = 0x8235;
#[doc = "`GL_R32UI: GLenum = 0x8236`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_R32UI: GLenum = 0x8236;
#[doc = "`GL_R8: GLenum = 0x8229`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_R8: GLenum = 0x8229;
#[doc = "`GL_R8I: GLenum = 0x8231`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_R8I: GLenum = 0x8231;
#[doc = "`GL_R8UI: GLenum = 0x8232`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_R8UI: GLenum = 0x8232;
#[doc = "`GL_R8_EXT: GLenum = 0x8229`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_rg", feature = "GL_EXT_texture_storage"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_texture_rg", feature = "GL_EXT_texture_storage")))
)]
pub const GL_R8_EXT: GLenum = 0x8229;
#[doc = "`GL_R8_SNORM: GLenum = 0x8F94`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_R8_SNORM: GLenum = 0x8F94;
#[doc = "`GL_RASTERIZER_DISCARD: GLenum = 0x8C89`"]
#[doc = "* **Group:** EnableCap"]
pub const GL_RASTERIZER_DISCARD: GLenum = 0x8C89;
#[doc = "`GL_RASTER_FIXED_SAMPLE_LOCATIONS_EXT: GLenum = 0x932A`"]
#[cfg(any(
    feature = "GL_EXT_raster_multisample",
    feature = "GL_NV_framebuffer_mixed_samples"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_raster_multisample",
        feature = "GL_NV_framebuffer_mixed_samples"
    )))
)]
pub const GL_RASTER_FIXED_SAMPLE_LOCATIONS_EXT: GLenum = 0x932A;
#[doc = "`GL_RASTER_MULTISAMPLE_EXT: GLenum = 0x9327`"]
#[cfg(any(
    feature = "GL_EXT_raster_multisample",
    feature = "GL_NV_framebuffer_mixed_samples"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_raster_multisample",
        feature = "GL_NV_framebuffer_mixed_samples"
    )))
)]
pub const GL_RASTER_MULTISAMPLE_EXT: GLenum = 0x9327;
#[doc = "`GL_RASTER_SAMPLES_EXT: GLenum = 0x9328`"]
#[cfg(any(
    feature = "GL_EXT_raster_multisample",
    feature = "GL_NV_framebuffer_mixed_samples"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_raster_multisample",
        feature = "GL_NV_framebuffer_mixed_samples"
    )))
)]
pub const GL_RASTER_SAMPLES_EXT: GLenum = 0x9328;
#[doc = "`GL_READ_BUFFER: GLenum = 0x0C02`"]
#[doc = "* **Group:** GetPName"]
pub const GL_READ_BUFFER: GLenum = 0x0C02;
#[doc = "`GL_READ_BUFFER_EXT: GLenum = 0x0C02`"]
#[doc = "* **Group:** GetPName"]
#[cfg(any(feature = "GL_EXT_multiview_draw_buffers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_multiview_draw_buffers"))))]
pub const GL_READ_BUFFER_EXT: GLenum = 0x0C02;
#[doc = "`GL_READ_BUFFER_NV: GLenum = 0x0C02`"]
#[doc = "* **Group:** GetPName"]
#[cfg(any(feature = "GL_NV_read_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_read_buffer"))))]
pub const GL_READ_BUFFER_NV: GLenum = 0x0C02;
#[doc = "`GL_READ_FRAMEBUFFER: GLenum = 0x8CA8`"]
#[doc = "* **Groups:** CheckFramebufferStatusTarget, FramebufferTarget"]
pub const GL_READ_FRAMEBUFFER: GLenum = 0x8CA8;
#[doc = "`GL_READ_FRAMEBUFFER_ANGLE: GLenum = 0x8CA8`"]
#[cfg(any(feature = "GL_ANGLE_framebuffer_blit"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_ANGLE_framebuffer_blit"))))]
pub const GL_READ_FRAMEBUFFER_ANGLE: GLenum = 0x8CA8;
#[doc = "`GL_READ_FRAMEBUFFER_APPLE: GLenum = 0x8CA8`"]
#[cfg(any(feature = "GL_APPLE_framebuffer_multisample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_framebuffer_multisample"))))]
pub const GL_READ_FRAMEBUFFER_APPLE: GLenum = 0x8CA8;
#[doc = "`GL_READ_FRAMEBUFFER_BINDING: GLenum = 0x8CAA`"]
#[doc = "* **Group:** GetPName"]
pub const GL_READ_FRAMEBUFFER_BINDING: GLenum = 0x8CAA;
#[doc = "`GL_READ_FRAMEBUFFER_BINDING_ANGLE: GLenum = 0x8CAA`"]
#[cfg(any(feature = "GL_ANGLE_framebuffer_blit"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_ANGLE_framebuffer_blit"))))]
pub const GL_READ_FRAMEBUFFER_BINDING_ANGLE: GLenum = 0x8CAA;
#[doc = "`GL_READ_FRAMEBUFFER_BINDING_APPLE: GLenum = 0x8CAA`"]
#[cfg(any(feature = "GL_APPLE_framebuffer_multisample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_framebuffer_multisample"))))]
pub const GL_READ_FRAMEBUFFER_BINDING_APPLE: GLenum = 0x8CAA;
#[doc = "`GL_READ_FRAMEBUFFER_BINDING_NV: GLenum = 0x8CAA`"]
#[cfg(any(feature = "GL_NV_framebuffer_blit"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_framebuffer_blit"))))]
pub const GL_READ_FRAMEBUFFER_BINDING_NV: GLenum = 0x8CAA;
#[doc = "`GL_READ_FRAMEBUFFER_NV: GLenum = 0x8CA8`"]
#[cfg(any(feature = "GL_NV_framebuffer_blit"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_framebuffer_blit"))))]
pub const GL_READ_FRAMEBUFFER_NV: GLenum = 0x8CA8;
#[doc = "`GL_READ_ONLY: GLenum = 0x88B8`"]
#[doc = "* **Group:** BufferAccessARB"]
pub const GL_READ_ONLY: GLenum = 0x88B8;
#[doc = "`GL_READ_WRITE: GLenum = 0x88BA`"]
#[doc = "* **Group:** BufferAccessARB"]
pub const GL_READ_WRITE: GLenum = 0x88BA;
#[doc = "`GL_RECT_NV: GLenum = 0xF6`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_RECT_NV: GLenum = 0xF6;
#[doc = "`GL_RED: GLenum = 0x1903`"]
#[doc = "* **Groups:** TextureSwizzle, PixelFormat, InternalFormat"]
pub const GL_RED: GLenum = 0x1903;
#[doc = "`GL_RED_BITS: GLenum = 0x0D52`"]
#[doc = "* **Group:** GetPName"]
pub const GL_RED_BITS: GLenum = 0x0D52;
#[doc = "`GL_RED_EXT: GLenum = 0x1903`"]
#[doc = "* **Groups:** InternalFormat, PixelFormat"]
#[cfg(any(feature = "GL_EXT_texture_rg"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_rg"))))]
pub const GL_RED_EXT: GLenum = 0x1903;
#[doc = "`GL_RED_INTEGER: GLenum = 0x8D94`"]
#[doc = "* **Group:** PixelFormat"]
pub const GL_RED_INTEGER: GLenum = 0x8D94;
#[doc = "`GL_RED_NV: GLenum = 0x1903`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_RED_NV: GLenum = 0x1903;
#[doc = "`GL_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x930B`"]
#[doc = "* **Group:** ProgramResourceProperty"]
pub const GL_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x930B;
#[doc = "`GL_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x930A`"]
#[doc = "* **Group:** ProgramResourceProperty"]
pub const GL_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x930A;
#[doc = "`GL_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x9309`"]
#[doc = "* **Group:** ProgramResourceProperty"]
pub const GL_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x9309;
#[doc = "`GL_REFERENCED_BY_GEOMETRY_SHADER_EXT: GLenum = 0x9309`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_REFERENCED_BY_GEOMETRY_SHADER_EXT: GLenum = 0x9309;
#[doc = "`GL_REFERENCED_BY_GEOMETRY_SHADER_OES: GLenum = 0x9309`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_REFERENCED_BY_GEOMETRY_SHADER_OES: GLenum = 0x9309;
#[doc = "`GL_REFERENCED_BY_MESH_SHADER_NV: GLenum = 0x95A0`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_REFERENCED_BY_MESH_SHADER_NV: GLenum = 0x95A0;
#[doc = "`GL_REFERENCED_BY_TASK_SHADER_NV: GLenum = 0x95A1`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_REFERENCED_BY_TASK_SHADER_NV: GLenum = 0x95A1;
#[doc = "`GL_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x9307`"]
#[doc = "* **Group:** ProgramResourceProperty"]
pub const GL_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x9307;
#[doc = "`GL_REFERENCED_BY_TESS_CONTROL_SHADER_EXT: GLenum = 0x9307`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_REFERENCED_BY_TESS_CONTROL_SHADER_EXT: GLenum = 0x9307;
#[doc = "`GL_REFERENCED_BY_TESS_CONTROL_SHADER_OES: GLenum = 0x9307`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_REFERENCED_BY_TESS_CONTROL_SHADER_OES: GLenum = 0x9307;
#[doc = "`GL_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x9308`"]
#[doc = "* **Group:** ProgramResourceProperty"]
pub const GL_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x9308;
#[doc = "`GL_REFERENCED_BY_TESS_EVALUATION_SHADER_EXT: GLenum = 0x9308`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_REFERENCED_BY_TESS_EVALUATION_SHADER_EXT: GLenum = 0x9308;
#[doc = "`GL_REFERENCED_BY_TESS_EVALUATION_SHADER_OES: GLenum = 0x9308`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_REFERENCED_BY_TESS_EVALUATION_SHADER_OES: GLenum = 0x9308;
#[doc = "`GL_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x9306`"]
#[doc = "* **Group:** ProgramResourceProperty"]
pub const GL_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x9306;
#[doc = "`GL_RELATIVE_ARC_TO_NV: GLenum = 0xFF`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_RELATIVE_ARC_TO_NV: GLenum = 0xFF;
#[doc = "`GL_RELATIVE_CONIC_CURVE_TO_NV: GLenum = 0x1B`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_RELATIVE_CONIC_CURVE_TO_NV: GLenum = 0x1B;
#[doc = "`GL_RELATIVE_CUBIC_CURVE_TO_NV: GLenum = 0x0D`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_RELATIVE_CUBIC_CURVE_TO_NV: GLenum = 0x0D;
#[doc = "`GL_RELATIVE_HORIZONTAL_LINE_TO_NV: GLenum = 0x07`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_RELATIVE_HORIZONTAL_LINE_TO_NV: GLenum = 0x07;
#[doc = "`GL_RELATIVE_LARGE_CCW_ARC_TO_NV: GLenum = 0x17`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_RELATIVE_LARGE_CCW_ARC_TO_NV: GLenum = 0x17;
#[doc = "`GL_RELATIVE_LARGE_CW_ARC_TO_NV: GLenum = 0x19`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_RELATIVE_LARGE_CW_ARC_TO_NV: GLenum = 0x19;
#[doc = "`GL_RELATIVE_LINE_TO_NV: GLenum = 0x05`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_RELATIVE_LINE_TO_NV: GLenum = 0x05;
#[doc = "`GL_RELATIVE_MOVE_TO_NV: GLenum = 0x03`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_RELATIVE_MOVE_TO_NV: GLenum = 0x03;
#[doc = "`GL_RELATIVE_QUADRATIC_CURVE_TO_NV: GLenum = 0x0B`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_RELATIVE_QUADRATIC_CURVE_TO_NV: GLenum = 0x0B;
#[doc = "`GL_RELATIVE_RECT_NV: GLenum = 0xF7`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_RELATIVE_RECT_NV: GLenum = 0xF7;
#[doc = "`GL_RELATIVE_ROUNDED_RECT2_NV: GLenum = 0xEB`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_RELATIVE_ROUNDED_RECT2_NV: GLenum = 0xEB;
#[doc = "`GL_RELATIVE_ROUNDED_RECT4_NV: GLenum = 0xED`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_RELATIVE_ROUNDED_RECT4_NV: GLenum = 0xED;
#[doc = "`GL_RELATIVE_ROUNDED_RECT8_NV: GLenum = 0xEF`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_RELATIVE_ROUNDED_RECT8_NV: GLenum = 0xEF;
#[doc = "`GL_RELATIVE_ROUNDED_RECT_NV: GLenum = 0xE9`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_RELATIVE_ROUNDED_RECT_NV: GLenum = 0xE9;
#[doc = "`GL_RELATIVE_SMALL_CCW_ARC_TO_NV: GLenum = 0x13`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_RELATIVE_SMALL_CCW_ARC_TO_NV: GLenum = 0x13;
#[doc = "`GL_RELATIVE_SMALL_CW_ARC_TO_NV: GLenum = 0x15`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_RELATIVE_SMALL_CW_ARC_TO_NV: GLenum = 0x15;
#[doc = "`GL_RELATIVE_SMOOTH_CUBIC_CURVE_TO_NV: GLenum = 0x11`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_RELATIVE_SMOOTH_CUBIC_CURVE_TO_NV: GLenum = 0x11;
#[doc = "`GL_RELATIVE_SMOOTH_QUADRATIC_CURVE_TO_NV: GLenum = 0x0F`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_RELATIVE_SMOOTH_QUADRATIC_CURVE_TO_NV: GLenum = 0x0F;
#[doc = "`GL_RELATIVE_VERTICAL_LINE_TO_NV: GLenum = 0x09`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_RELATIVE_VERTICAL_LINE_TO_NV: GLenum = 0x09;
#[doc = "`GL_RENDERBUFFER: GLenum = 0x8D41`"]
#[doc = "* **Groups:** ObjectIdentifier, RenderbufferTarget, CopyImageSubDataTarget"]
pub const GL_RENDERBUFFER: GLenum = 0x8D41;
#[doc = "`GL_RENDERBUFFER_ALPHA_SIZE: GLenum = 0x8D53`"]
#[doc = "* **Group:** RenderbufferParameterName"]
pub const GL_RENDERBUFFER_ALPHA_SIZE: GLenum = 0x8D53;
#[doc = "`GL_RENDERBUFFER_BINDING: GLenum = 0x8CA7`"]
#[doc = "* **Group:** GetPName"]
pub const GL_RENDERBUFFER_BINDING: GLenum = 0x8CA7;
#[doc = "`GL_RENDERBUFFER_BLUE_SIZE: GLenum = 0x8D52`"]
#[doc = "* **Group:** RenderbufferParameterName"]
pub const GL_RENDERBUFFER_BLUE_SIZE: GLenum = 0x8D52;
#[doc = "`GL_RENDERBUFFER_DEPTH_SIZE: GLenum = 0x8D54`"]
#[doc = "* **Group:** RenderbufferParameterName"]
pub const GL_RENDERBUFFER_DEPTH_SIZE: GLenum = 0x8D54;
#[doc = "`GL_RENDERBUFFER_GREEN_SIZE: GLenum = 0x8D51`"]
#[doc = "* **Group:** RenderbufferParameterName"]
pub const GL_RENDERBUFFER_GREEN_SIZE: GLenum = 0x8D51;
#[doc = "`GL_RENDERBUFFER_HEIGHT: GLenum = 0x8D43`"]
#[doc = "* **Group:** RenderbufferParameterName"]
pub const GL_RENDERBUFFER_HEIGHT: GLenum = 0x8D43;
#[doc = "`GL_RENDERBUFFER_INTERNAL_FORMAT: GLenum = 0x8D44`"]
#[doc = "* **Group:** RenderbufferParameterName"]
pub const GL_RENDERBUFFER_INTERNAL_FORMAT: GLenum = 0x8D44;
#[doc = "`GL_RENDERBUFFER_RED_SIZE: GLenum = 0x8D50`"]
#[doc = "* **Group:** RenderbufferParameterName"]
pub const GL_RENDERBUFFER_RED_SIZE: GLenum = 0x8D50;
#[doc = "`GL_RENDERBUFFER_SAMPLES: GLenum = 0x8CAB`"]
#[doc = "* **Group:** RenderbufferParameterName"]
pub const GL_RENDERBUFFER_SAMPLES: GLenum = 0x8CAB;
#[doc = "`GL_RENDERBUFFER_SAMPLES_ANGLE: GLenum = 0x8CAB`"]
#[doc = "* **Group:** RenderbufferParameterName"]
#[cfg(any(feature = "GL_ANGLE_framebuffer_multisample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_ANGLE_framebuffer_multisample"))))]
pub const GL_RENDERBUFFER_SAMPLES_ANGLE: GLenum = 0x8CAB;
#[doc = "`GL_RENDERBUFFER_SAMPLES_APPLE: GLenum = 0x8CAB`"]
#[doc = "* **Group:** RenderbufferParameterName"]
#[cfg(any(feature = "GL_APPLE_framebuffer_multisample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_framebuffer_multisample"))))]
pub const GL_RENDERBUFFER_SAMPLES_APPLE: GLenum = 0x8CAB;
#[doc = "`GL_RENDERBUFFER_SAMPLES_EXT: GLenum = 0x8CAB`"]
#[doc = "* **Group:** RenderbufferParameterName"]
#[cfg(any(feature = "GL_EXT_multisampled_render_to_texture"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_multisampled_render_to_texture")))
)]
pub const GL_RENDERBUFFER_SAMPLES_EXT: GLenum = 0x8CAB;
#[doc = "`GL_RENDERBUFFER_SAMPLES_IMG: GLenum = 0x9133`"]
#[doc = "* **Group:** RenderbufferParameterName"]
#[cfg(any(feature = "GL_IMG_multisampled_render_to_texture"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_IMG_multisampled_render_to_texture")))
)]
pub const GL_RENDERBUFFER_SAMPLES_IMG: GLenum = 0x9133;
#[doc = "`GL_RENDERBUFFER_SAMPLES_NV: GLenum = 0x8CAB`"]
#[doc = "* **Group:** RenderbufferParameterName"]
#[cfg(any(feature = "GL_NV_framebuffer_multisample"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_framebuffer_multisample"))))]
pub const GL_RENDERBUFFER_SAMPLES_NV: GLenum = 0x8CAB;
#[doc = "`GL_RENDERBUFFER_STENCIL_SIZE: GLenum = 0x8D55`"]
#[doc = "* **Group:** RenderbufferParameterName"]
pub const GL_RENDERBUFFER_STENCIL_SIZE: GLenum = 0x8D55;
#[doc = "`GL_RENDERBUFFER_STORAGE_SAMPLES_AMD: GLenum = 0x91B2`"]
#[doc = "* **Group:** RenderbufferParameterName"]
#[cfg(any(feature = "GL_AMD_framebuffer_multisample_advanced"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_AMD_framebuffer_multisample_advanced")))
)]
pub const GL_RENDERBUFFER_STORAGE_SAMPLES_AMD: GLenum = 0x91B2;
#[doc = "`GL_RENDERBUFFER_WIDTH: GLenum = 0x8D42`"]
#[doc = "* **Group:** RenderbufferParameterName"]
pub const GL_RENDERBUFFER_WIDTH: GLenum = 0x8D42;
#[doc = "`GL_RENDERER: GLenum = 0x1F01`"]
#[doc = "* **Group:** StringName"]
pub const GL_RENDERER: GLenum = 0x1F01;
#[doc = "`GL_RENDER_DIRECT_TO_FRAMEBUFFER_QCOM: GLenum = 0x8FB3`"]
#[cfg(any(feature = "GL_QCOM_binning_control"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_binning_control"))))]
pub const GL_RENDER_DIRECT_TO_FRAMEBUFFER_QCOM: GLenum = 0x8FB3;
#[doc = "`GL_REPEAT: GLenum = 0x2901`"]
#[doc = "* **Group:** TextureWrapMode"]
pub const GL_REPEAT: GLenum = 0x2901;
#[doc = "`GL_REPLACE: GLenum = 0x1E01`"]
#[doc = "* **Groups:** StencilOp, LightEnvModeSGIX"]
pub const GL_REPLACE: GLenum = 0x1E01;
#[doc = "`GL_REPRESENTATIVE_FRAGMENT_TEST_NV: GLenum = 0x937F`"]
#[cfg(any(feature = "GL_NV_representative_fragment_test"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_representative_fragment_test"))))]
pub const GL_REPRESENTATIVE_FRAGMENT_TEST_NV: GLenum = 0x937F;
#[doc = "`GL_REQUIRED_TEXTURE_IMAGE_UNITS_OES: GLenum = 0x8D68`"]
#[cfg(any(feature = "GL_EXT_YUV_target", feature = "GL_OES_EGL_image_external"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_YUV_target", feature = "GL_OES_EGL_image_external")))
)]
pub const GL_REQUIRED_TEXTURE_IMAGE_UNITS_OES: GLenum = 0x8D68;
#[doc = "`GL_RESET_NOTIFICATION_STRATEGY: GLenum = 0x8256`"]
pub const GL_RESET_NOTIFICATION_STRATEGY: GLenum = 0x8256;
#[doc = "`GL_RESET_NOTIFICATION_STRATEGY_EXT: GLenum = 0x8256`"]
#[cfg(any(feature = "GL_EXT_robustness"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_robustness"))))]
pub const GL_RESET_NOTIFICATION_STRATEGY_EXT: GLenum = 0x8256;
#[doc = "`GL_RESET_NOTIFICATION_STRATEGY_KHR: GLenum = 0x8256`"]
#[cfg(any(feature = "GL_KHR_robustness"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_robustness"))))]
pub const GL_RESET_NOTIFICATION_STRATEGY_KHR: GLenum = 0x8256;
#[doc = "`GL_RESTART_PATH_NV: GLenum = 0xF0`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_RESTART_PATH_NV: GLenum = 0xF0;
#[doc = "`GL_RG: GLenum = 0x8227`"]
#[doc = "* **Groups:** InternalFormat, PixelFormat"]
pub const GL_RG: GLenum = 0x8227;
#[doc = "`GL_RG16F: GLenum = 0x822F`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RG16F: GLenum = 0x822F;
#[doc = "`GL_RG16F_EXT: GLenum = 0x822F`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_EXT_color_buffer_half_float",
    feature = "GL_EXT_texture_storage"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_color_buffer_half_float",
        feature = "GL_EXT_texture_storage"
    )))
)]
pub const GL_RG16F_EXT: GLenum = 0x822F;
#[doc = "`GL_RG16I: GLenum = 0x8239`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RG16I: GLenum = 0x8239;
#[doc = "`GL_RG16UI: GLenum = 0x823A`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RG16UI: GLenum = 0x823A;
#[doc = "`GL_RG16_EXT: GLenum = 0x822C`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_norm16"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_norm16"))))]
pub const GL_RG16_EXT: GLenum = 0x822C;
#[doc = "`GL_RG16_SNORM_EXT: GLenum = 0x8F99`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_render_snorm", feature = "GL_EXT_texture_norm16"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_render_snorm", feature = "GL_EXT_texture_norm16")))
)]
pub const GL_RG16_SNORM_EXT: GLenum = 0x8F99;
#[doc = "`GL_RG32F: GLenum = 0x8230`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RG32F: GLenum = 0x8230;
#[doc = "`GL_RG32F_EXT: GLenum = 0x8230`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_storage"))))]
pub const GL_RG32F_EXT: GLenum = 0x8230;
#[doc = "`GL_RG32I: GLenum = 0x823B`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RG32I: GLenum = 0x823B;
#[doc = "`GL_RG32UI: GLenum = 0x823C`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RG32UI: GLenum = 0x823C;
#[doc = "`GL_RG8: GLenum = 0x822B`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RG8: GLenum = 0x822B;
#[doc = "`GL_RG8I: GLenum = 0x8237`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RG8I: GLenum = 0x8237;
#[doc = "`GL_RG8UI: GLenum = 0x8238`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RG8UI: GLenum = 0x8238;
#[doc = "`GL_RG8_EXT: GLenum = 0x822B`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_rg", feature = "GL_EXT_texture_storage"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_texture_rg", feature = "GL_EXT_texture_storage")))
)]
pub const GL_RG8_EXT: GLenum = 0x822B;
#[doc = "`GL_RG8_SNORM: GLenum = 0x8F95`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RG8_SNORM: GLenum = 0x8F95;
#[doc = "`GL_RGB: GLenum = 0x1907`"]
#[doc = "* **Groups:** PixelTexGenMode, CombinerPortionNV, PathColorFormat, CombinerComponentUsageNV, PixelFormat, InternalFormat"]
pub const GL_RGB: GLenum = 0x1907;
#[doc = "`GL_RGB10_A2: GLenum = 0x8059`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGB10_A2: GLenum = 0x8059;
#[doc = "`GL_RGB10_A2UI: GLenum = 0x906F`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGB10_A2UI: GLenum = 0x906F;
#[doc = "`GL_RGB10_A2_EXT: GLenum = 0x8059`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_EXT_texture_storage",
    feature = "GL_OES_required_internalformat"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_texture_storage",
        feature = "GL_OES_required_internalformat"
    )))
)]
pub const GL_RGB10_A2_EXT: GLenum = 0x8059;
#[doc = "`GL_RGB10_EXT: GLenum = 0x8052`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_EXT_texture_storage",
    feature = "GL_OES_required_internalformat"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_texture_storage",
        feature = "GL_OES_required_internalformat"
    )))
)]
pub const GL_RGB10_EXT: GLenum = 0x8052;
#[doc = "`GL_RGB16F: GLenum = 0x881B`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGB16F: GLenum = 0x881B;
#[doc = "`GL_RGB16F_EXT: GLenum = 0x881B`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_EXT_color_buffer_half_float",
    feature = "GL_EXT_texture_storage"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_color_buffer_half_float",
        feature = "GL_EXT_texture_storage"
    )))
)]
pub const GL_RGB16F_EXT: GLenum = 0x881B;
#[doc = "`GL_RGB16I: GLenum = 0x8D89`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGB16I: GLenum = 0x8D89;
#[doc = "`GL_RGB16UI: GLenum = 0x8D77`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGB16UI: GLenum = 0x8D77;
#[doc = "`GL_RGB16_EXT: GLenum = 0x8054`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_norm16"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_norm16"))))]
pub const GL_RGB16_EXT: GLenum = 0x8054;
#[doc = "`GL_RGB16_SNORM_EXT: GLenum = 0x8F9A`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_norm16"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_norm16"))))]
pub const GL_RGB16_SNORM_EXT: GLenum = 0x8F9A;
#[doc = "`GL_RGB32F: GLenum = 0x8815`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGB32F: GLenum = 0x8815;
#[doc = "`GL_RGB32F_EXT: GLenum = 0x8815`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_storage"))))]
pub const GL_RGB32F_EXT: GLenum = 0x8815;
#[doc = "`GL_RGB32I: GLenum = 0x8D83`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGB32I: GLenum = 0x8D83;
#[doc = "`GL_RGB32UI: GLenum = 0x8D71`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGB32UI: GLenum = 0x8D71;
#[doc = "`GL_RGB565: GLenum = 0x8D62`"]
pub const GL_RGB565: GLenum = 0x8D62;
#[doc = "`GL_RGB565_OES: GLenum = 0x8D62`"]
#[cfg(any(feature = "GL_OES_required_internalformat"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_required_internalformat"))))]
pub const GL_RGB565_OES: GLenum = 0x8D62;
#[doc = "`GL_RGB5_A1: GLenum = 0x8057`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGB5_A1: GLenum = 0x8057;
#[doc = "`GL_RGB5_A1_OES: GLenum = 0x8057`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_required_internalformat"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_required_internalformat"))))]
pub const GL_RGB5_A1_OES: GLenum = 0x8057;
#[doc = "`GL_RGB8: GLenum = 0x8051`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGB8: GLenum = 0x8051;
#[doc = "`GL_RGB8I: GLenum = 0x8D8F`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGB8I: GLenum = 0x8D8F;
#[doc = "`GL_RGB8UI: GLenum = 0x8D7D`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGB8UI: GLenum = 0x8D7D;
#[doc = "`GL_RGB8_OES: GLenum = 0x8051`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_OES_required_internalformat",
    feature = "GL_OES_rgb8_rgba8"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_OES_required_internalformat",
        feature = "GL_OES_rgb8_rgba8"
    )))
)]
pub const GL_RGB8_OES: GLenum = 0x8051;
#[doc = "`GL_RGB8_SNORM: GLenum = 0x8F96`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGB8_SNORM: GLenum = 0x8F96;
#[doc = "`GL_RGB9_E5: GLenum = 0x8C3D`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGB9_E5: GLenum = 0x8C3D;
#[doc = "`GL_RGB9_E5_APPLE: GLenum = 0x8C3D`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_APPLE_texture_packed_float"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_texture_packed_float"))))]
pub const GL_RGB9_E5_APPLE: GLenum = 0x8C3D;
#[doc = "`GL_RGBA: GLenum = 0x1908`"]
#[doc = "* **Groups:** PixelTexGenMode, PathColorFormat, PixelFormat, InternalFormat"]
pub const GL_RGBA: GLenum = 0x1908;
#[doc = "`GL_RGBA16F: GLenum = 0x881A`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGBA16F: GLenum = 0x881A;
#[doc = "`GL_RGBA16F_EXT: GLenum = 0x881A`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_EXT_color_buffer_half_float",
    feature = "GL_EXT_texture_storage"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_color_buffer_half_float",
        feature = "GL_EXT_texture_storage"
    )))
)]
pub const GL_RGBA16F_EXT: GLenum = 0x881A;
#[doc = "`GL_RGBA16I: GLenum = 0x8D88`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGBA16I: GLenum = 0x8D88;
#[doc = "`GL_RGBA16UI: GLenum = 0x8D76`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGBA16UI: GLenum = 0x8D76;
#[doc = "`GL_RGBA16_EXT: GLenum = 0x805B`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_norm16"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_norm16"))))]
pub const GL_RGBA16_EXT: GLenum = 0x805B;
#[doc = "`GL_RGBA16_SNORM_EXT: GLenum = 0x8F9B`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_render_snorm", feature = "GL_EXT_texture_norm16"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_render_snorm", feature = "GL_EXT_texture_norm16")))
)]
pub const GL_RGBA16_SNORM_EXT: GLenum = 0x8F9B;
#[doc = "`GL_RGBA32F: GLenum = 0x8814`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGBA32F: GLenum = 0x8814;
#[doc = "`GL_RGBA32F_EXT: GLenum = 0x8814`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_storage"))))]
pub const GL_RGBA32F_EXT: GLenum = 0x8814;
#[doc = "`GL_RGBA32I: GLenum = 0x8D82`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGBA32I: GLenum = 0x8D82;
#[doc = "`GL_RGBA32UI: GLenum = 0x8D70`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGBA32UI: GLenum = 0x8D70;
#[doc = "`GL_RGBA4: GLenum = 0x8056`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGBA4: GLenum = 0x8056;
#[doc = "`GL_RGBA4_OES: GLenum = 0x8056`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_required_internalformat"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_required_internalformat"))))]
pub const GL_RGBA4_OES: GLenum = 0x8056;
#[doc = "`GL_RGBA8: GLenum = 0x8058`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGBA8: GLenum = 0x8058;
#[doc = "`GL_RGBA8I: GLenum = 0x8D8E`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGBA8I: GLenum = 0x8D8E;
#[doc = "`GL_RGBA8UI: GLenum = 0x8D7C`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGBA8UI: GLenum = 0x8D7C;
#[doc = "`GL_RGBA8_OES: GLenum = 0x8058`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(
    feature = "GL_OES_required_internalformat",
    feature = "GL_OES_rgb8_rgba8"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_OES_required_internalformat",
        feature = "GL_OES_rgb8_rgba8"
    )))
)]
pub const GL_RGBA8_OES: GLenum = 0x8058;
#[doc = "`GL_RGBA8_SNORM: GLenum = 0x8F97`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_RGBA8_SNORM: GLenum = 0x8F97;
#[doc = "`GL_RGBA_INTEGER: GLenum = 0x8D99`"]
#[doc = "* **Group:** PixelFormat"]
pub const GL_RGBA_INTEGER: GLenum = 0x8D99;
#[doc = "`GL_RGB_422_APPLE: GLenum = 0x8A1F`"]
#[cfg(any(feature = "GL_APPLE_rgb_422"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_rgb_422"))))]
pub const GL_RGB_422_APPLE: GLenum = 0x8A1F;
#[doc = "`GL_RGB_INTEGER: GLenum = 0x8D98`"]
#[doc = "* **Group:** PixelFormat"]
pub const GL_RGB_INTEGER: GLenum = 0x8D98;
#[doc = "`GL_RGB_RAW_422_APPLE: GLenum = 0x8A51`"]
#[cfg(any(feature = "GL_APPLE_rgb_422"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_rgb_422"))))]
pub const GL_RGB_RAW_422_APPLE: GLenum = 0x8A51;
#[doc = "`GL_RG_EXT: GLenum = 0x8227`"]
#[cfg(any(feature = "GL_EXT_texture_rg"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_rg"))))]
pub const GL_RG_EXT: GLenum = 0x8227;
#[doc = "`GL_RG_INTEGER: GLenum = 0x8228`"]
#[doc = "* **Group:** PixelFormat"]
pub const GL_RG_INTEGER: GLenum = 0x8228;
#[doc = "`GL_ROUNDED_RECT2_NV: GLenum = 0xEA`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_ROUNDED_RECT2_NV: GLenum = 0xEA;
#[doc = "`GL_ROUNDED_RECT4_NV: GLenum = 0xEC`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_ROUNDED_RECT4_NV: GLenum = 0xEC;
#[doc = "`GL_ROUNDED_RECT8_NV: GLenum = 0xEE`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_ROUNDED_RECT8_NV: GLenum = 0xEE;
#[doc = "`GL_ROUNDED_RECT_NV: GLenum = 0xE8`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_ROUNDED_RECT_NV: GLenum = 0xE8;
#[doc = "`GL_ROUND_NV: GLenum = 0x90A4`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_ROUND_NV: GLenum = 0x90A4;
#[doc = "`GL_SAMPLER: GLenum = 0x82E6`"]
#[doc = "* **Group:** ObjectIdentifier"]
pub const GL_SAMPLER: GLenum = 0x82E6;
#[doc = "`GL_SAMPLER_2D: GLenum = 0x8B5E`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_SAMPLER_2D: GLenum = 0x8B5E;
#[doc = "`GL_SAMPLER_2D_ARRAY: GLenum = 0x8DC1`"]
#[doc = "* **Groups:** GlslTypeToken, UniformType"]
pub const GL_SAMPLER_2D_ARRAY: GLenum = 0x8DC1;
#[doc = "`GL_SAMPLER_2D_ARRAY_SHADOW: GLenum = 0x8DC4`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_SAMPLER_2D_ARRAY_SHADOW: GLenum = 0x8DC4;
#[doc = "`GL_SAMPLER_2D_ARRAY_SHADOW_NV: GLenum = 0x8DC4`"]
#[cfg(any(feature = "GL_NV_shadow_samplers_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shadow_samplers_array"))))]
pub const GL_SAMPLER_2D_ARRAY_SHADOW_NV: GLenum = 0x8DC4;
#[doc = "`GL_SAMPLER_2D_MULTISAMPLE: GLenum = 0x9108`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_SAMPLER_2D_MULTISAMPLE: GLenum = 0x9108;
#[doc = "`GL_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910B`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910B;
#[doc = "`GL_SAMPLER_2D_MULTISAMPLE_ARRAY_OES: GLenum = 0x910B`"]
#[cfg(any(feature = "GL_OES_texture_storage_multisample_2d_array"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_OES_texture_storage_multisample_2d_array")))
)]
pub const GL_SAMPLER_2D_MULTISAMPLE_ARRAY_OES: GLenum = 0x910B;
#[doc = "`GL_SAMPLER_2D_SHADOW: GLenum = 0x8B62`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_SAMPLER_2D_SHADOW: GLenum = 0x8B62;
#[doc = "`GL_SAMPLER_2D_SHADOW_EXT: GLenum = 0x8B62`"]
#[doc = "* **Group:** AttributeType"]
#[cfg(any(feature = "GL_EXT_shadow_samplers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_shadow_samplers"))))]
pub const GL_SAMPLER_2D_SHADOW_EXT: GLenum = 0x8B62;
#[doc = "`GL_SAMPLER_3D: GLenum = 0x8B5F`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_SAMPLER_3D: GLenum = 0x8B5F;
#[doc = "`GL_SAMPLER_3D_OES: GLenum = 0x8B5F`"]
#[doc = "* **Group:** AttributeType"]
#[cfg(any(feature = "GL_OES_texture_3D"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_3D"))))]
pub const GL_SAMPLER_3D_OES: GLenum = 0x8B5F;
#[doc = "`GL_SAMPLER_BINDING: GLenum = 0x8919`"]
#[doc = "* **Group:** GetPName"]
pub const GL_SAMPLER_BINDING: GLenum = 0x8919;
#[doc = "`GL_SAMPLER_BUFFER: GLenum = 0x8DC2`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_SAMPLER_BUFFER: GLenum = 0x8DC2;
#[doc = "`GL_SAMPLER_BUFFER_EXT: GLenum = 0x8DC2`"]
#[cfg(any(feature = "GL_EXT_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_buffer"))))]
pub const GL_SAMPLER_BUFFER_EXT: GLenum = 0x8DC2;
#[doc = "`GL_SAMPLER_BUFFER_OES: GLenum = 0x8DC2`"]
#[cfg(any(feature = "GL_OES_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_buffer"))))]
pub const GL_SAMPLER_BUFFER_OES: GLenum = 0x8DC2;
#[doc = "`GL_SAMPLER_CUBE: GLenum = 0x8B60`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_SAMPLER_CUBE: GLenum = 0x8B60;
#[doc = "`GL_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900C`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900C;
#[doc = "`GL_SAMPLER_CUBE_MAP_ARRAY_EXT: GLenum = 0x900C`"]
#[cfg(any(feature = "GL_EXT_texture_cube_map_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_cube_map_array"))))]
pub const GL_SAMPLER_CUBE_MAP_ARRAY_EXT: GLenum = 0x900C;
#[doc = "`GL_SAMPLER_CUBE_MAP_ARRAY_OES: GLenum = 0x900C`"]
#[cfg(any(feature = "GL_OES_texture_cube_map_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_cube_map_array"))))]
pub const GL_SAMPLER_CUBE_MAP_ARRAY_OES: GLenum = 0x900C;
#[doc = "`GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW: GLenum = 0x900D`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW: GLenum = 0x900D;
#[doc = "`GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW_EXT: GLenum = 0x900D`"]
#[cfg(any(feature = "GL_EXT_texture_cube_map_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_cube_map_array"))))]
pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW_EXT: GLenum = 0x900D;
#[doc = "`GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW_OES: GLenum = 0x900D`"]
#[cfg(any(feature = "GL_OES_texture_cube_map_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_cube_map_array"))))]
pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW_OES: GLenum = 0x900D;
#[doc = "`GL_SAMPLER_CUBE_SHADOW: GLenum = 0x8DC5`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_SAMPLER_CUBE_SHADOW: GLenum = 0x8DC5;
#[doc = "`GL_SAMPLER_CUBE_SHADOW_NV: GLenum = 0x8DC5`"]
#[cfg(any(feature = "GL_NV_shadow_samplers_cube"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shadow_samplers_cube"))))]
pub const GL_SAMPLER_CUBE_SHADOW_NV: GLenum = 0x8DC5;
#[doc = "`GL_SAMPLER_EXTERNAL_2D_Y2Y_EXT: GLenum = 0x8BE7`"]
#[cfg(any(feature = "GL_EXT_YUV_target"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_YUV_target"))))]
pub const GL_SAMPLER_EXTERNAL_2D_Y2Y_EXT: GLenum = 0x8BE7;
#[doc = "`GL_SAMPLER_EXTERNAL_OES: GLenum = 0x8D66`"]
#[cfg(any(feature = "GL_OES_EGL_image_external"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_EGL_image_external"))))]
pub const GL_SAMPLER_EXTERNAL_OES: GLenum = 0x8D66;
#[doc = "`GL_SAMPLER_KHR: GLenum = 0x82E6`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_SAMPLER_KHR: GLenum = 0x82E6;
#[doc = "`GL_SAMPLES: GLenum = 0x80A9`"]
#[doc = "* **Groups:** GetFramebufferParameter, GetPName, InternalFormatPName"]
pub const GL_SAMPLES: GLenum = 0x80A9;
#[doc = "`GL_SAMPLE_ALPHA_TO_COVERAGE: GLenum = 0x809E`"]
#[doc = "* **Group:** EnableCap"]
pub const GL_SAMPLE_ALPHA_TO_COVERAGE: GLenum = 0x809E;
#[doc = "`GL_SAMPLE_BUFFERS: GLenum = 0x80A8`"]
#[doc = "* **Groups:** GetFramebufferParameter, GetPName"]
pub const GL_SAMPLE_BUFFERS: GLenum = 0x80A8;
#[doc = "`GL_SAMPLE_COVERAGE: GLenum = 0x80A0`"]
#[doc = "* **Group:** EnableCap"]
pub const GL_SAMPLE_COVERAGE: GLenum = 0x80A0;
#[doc = "`GL_SAMPLE_COVERAGE_INVERT: GLenum = 0x80AB`"]
#[doc = "* **Group:** GetPName"]
pub const GL_SAMPLE_COVERAGE_INVERT: GLenum = 0x80AB;
#[doc = "`GL_SAMPLE_COVERAGE_VALUE: GLenum = 0x80AA`"]
#[doc = "* **Group:** GetPName"]
pub const GL_SAMPLE_COVERAGE_VALUE: GLenum = 0x80AA;
#[doc = "`GL_SAMPLE_LOCATION_NV: GLenum = 0x8E50`"]
#[doc = "* **Alias Of:** `GL_SAMPLE_POSITION_NV`"]
#[cfg(any(feature = "GL_NV_sample_locations"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_sample_locations"))))]
pub const GL_SAMPLE_LOCATION_NV: GLenum = 0x8E50;
#[doc = "`GL_SAMPLE_LOCATION_PIXEL_GRID_HEIGHT_NV: GLenum = 0x933F`"]
#[cfg(any(feature = "GL_NV_sample_locations"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_sample_locations"))))]
pub const GL_SAMPLE_LOCATION_PIXEL_GRID_HEIGHT_NV: GLenum = 0x933F;
#[doc = "`GL_SAMPLE_LOCATION_PIXEL_GRID_WIDTH_NV: GLenum = 0x933E`"]
#[cfg(any(feature = "GL_NV_sample_locations"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_sample_locations"))))]
pub const GL_SAMPLE_LOCATION_PIXEL_GRID_WIDTH_NV: GLenum = 0x933E;
#[doc = "`GL_SAMPLE_LOCATION_SUBPIXEL_BITS_NV: GLenum = 0x933D`"]
#[cfg(any(feature = "GL_NV_sample_locations"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_sample_locations"))))]
pub const GL_SAMPLE_LOCATION_SUBPIXEL_BITS_NV: GLenum = 0x933D;
#[doc = "`GL_SAMPLE_MASK: GLenum = 0x8E51`"]
#[doc = "* **Group:** EnableCap"]
pub const GL_SAMPLE_MASK: GLenum = 0x8E51;
#[doc = "`GL_SAMPLE_MASK_VALUE: GLenum = 0x8E52`"]
pub const GL_SAMPLE_MASK_VALUE: GLenum = 0x8E52;
#[doc = "`GL_SAMPLE_POSITION: GLenum = 0x8E50`"]
#[doc = "* **Group:** GetMultisamplePNameNV"]
pub const GL_SAMPLE_POSITION: GLenum = 0x8E50;
#[doc = "`GL_SAMPLE_SHADING: GLenum = 0x8C36`"]
#[doc = "* **Group:** EnableCap"]
pub const GL_SAMPLE_SHADING: GLenum = 0x8C36;
#[doc = "`GL_SAMPLE_SHADING_OES: GLenum = 0x8C36`"]
#[cfg(any(feature = "GL_OES_sample_shading"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_sample_shading"))))]
pub const GL_SAMPLE_SHADING_OES: GLenum = 0x8C36;
#[doc = "`GL_SCISSOR_BOX: GLenum = 0x0C10`"]
#[doc = "* **Group:** GetPName"]
pub const GL_SCISSOR_BOX: GLenum = 0x0C10;
#[doc = "`GL_SCISSOR_BOX_EXCLUSIVE_NV: GLenum = 0x9556`"]
#[cfg(any(feature = "GL_NV_scissor_exclusive"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_scissor_exclusive"))))]
pub const GL_SCISSOR_BOX_EXCLUSIVE_NV: GLenum = 0x9556;
#[doc = "`GL_SCISSOR_TEST: GLenum = 0x0C11`"]
#[doc = "* **Groups:** GetPName, EnableCap"]
pub const GL_SCISSOR_TEST: GLenum = 0x0C11;
#[doc = "`GL_SCISSOR_TEST_EXCLUSIVE_NV: GLenum = 0x9555`"]
#[cfg(any(feature = "GL_NV_scissor_exclusive"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_scissor_exclusive"))))]
pub const GL_SCISSOR_TEST_EXCLUSIVE_NV: GLenum = 0x9555;
#[doc = "`GL_SCREEN: GLenum = 0x9295`"]
pub const GL_SCREEN: GLenum = 0x9295;
#[doc = "`GL_SCREEN_KHR: GLenum = 0x9295`"]
#[cfg(any(feature = "GL_KHR_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_blend_equation_advanced"))))]
pub const GL_SCREEN_KHR: GLenum = 0x9295;
#[doc = "`GL_SCREEN_NV: GLenum = 0x9295`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_SCREEN_NV: GLenum = 0x9295;
#[doc = "`GL_SEMAPHORE_TYPE_BINARY_NV: GLenum = 0x95B4`"]
#[doc = "* **Group:** SemaphoreParameterName"]
#[cfg(any(feature = "GL_NV_timeline_semaphore"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_timeline_semaphore"))))]
pub const GL_SEMAPHORE_TYPE_BINARY_NV: GLenum = 0x95B4;
#[doc = "`GL_SEMAPHORE_TYPE_NV: GLenum = 0x95B3`"]
#[doc = "* **Group:** SemaphoreParameterName"]
#[cfg(any(feature = "GL_NV_timeline_semaphore"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_timeline_semaphore"))))]
pub const GL_SEMAPHORE_TYPE_NV: GLenum = 0x95B3;
#[doc = "`GL_SEMAPHORE_TYPE_TIMELINE_NV: GLenum = 0x95B5`"]
#[doc = "* **Group:** SemaphoreParameterName"]
#[cfg(any(feature = "GL_NV_timeline_semaphore"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_timeline_semaphore"))))]
pub const GL_SEMAPHORE_TYPE_TIMELINE_NV: GLenum = 0x95B5;
#[doc = "`GL_SEPARATE_ATTRIBS: GLenum = 0x8C8D`"]
#[doc = "* **Group:** TransformFeedbackBufferMode"]
pub const GL_SEPARATE_ATTRIBS: GLenum = 0x8C8D;
#[doc = "`GL_SGX_BINARY_IMG: GLenum = 0x8C0A`"]
#[doc = "* **Group:** ShaderBinaryFormat"]
#[cfg(any(feature = "GL_IMG_shader_binary"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_IMG_shader_binary"))))]
pub const GL_SGX_BINARY_IMG: GLenum = 0x8C0A;
#[doc = "`GL_SGX_PROGRAM_BINARY_IMG: GLenum = 0x9130`"]
#[cfg(any(feature = "GL_IMG_program_binary"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_IMG_program_binary"))))]
pub const GL_SGX_PROGRAM_BINARY_IMG: GLenum = 0x9130;
#[doc = "`GL_SHADER: GLenum = 0x82E1`"]
#[doc = "* **Group:** ObjectIdentifier"]
pub const GL_SHADER: GLenum = 0x82E1;
#[doc = "`GL_SHADER_BINARY_DMP: GLenum = 0x9250`"]
#[doc = "* **Group:** ShaderBinaryFormat"]
#[cfg(any(feature = "GL_DMP_shader_binary"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_DMP_shader_binary"))))]
pub const GL_SHADER_BINARY_DMP: GLenum = 0x9250;
#[doc = "`GL_SHADER_BINARY_FORMATS: GLenum = 0x8DF8`"]
#[doc = "* **Group:** GetPName"]
pub const GL_SHADER_BINARY_FORMATS: GLenum = 0x8DF8;
#[doc = "`GL_SHADER_BINARY_VIV: GLenum = 0x8FC4`"]
#[doc = "* **Group:** ShaderBinaryFormat"]
#[cfg(any(feature = "GL_VIV_shader_binary"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_VIV_shader_binary"))))]
pub const GL_SHADER_BINARY_VIV: GLenum = 0x8FC4;
#[doc = "`GL_SHADER_COMPILER: GLenum = 0x8DFA`"]
#[doc = "* **Group:** GetPName"]
pub const GL_SHADER_COMPILER: GLenum = 0x8DFA;
#[doc = "`GL_SHADER_IMAGE_ACCESS_BARRIER_BIT: GLbitfield = 0x00000020`"]
#[doc = "* **Group:** MemoryBarrierMask"]
pub const GL_SHADER_IMAGE_ACCESS_BARRIER_BIT: GLbitfield = 0x00000020;
#[doc = "`GL_SHADER_KHR: GLenum = 0x82E1`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_SHADER_KHR: GLenum = 0x82E1;
#[doc = "`GL_SHADER_OBJECT_EXT: GLenum = 0x8B48`"]
#[cfg(any(feature = "GL_EXT_debug_label"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_debug_label"))))]
pub const GL_SHADER_OBJECT_EXT: GLenum = 0x8B48;
#[doc = "`GL_SHADER_PIXEL_LOCAL_STORAGE_EXT: GLenum = 0x8F64`"]
#[cfg(any(feature = "GL_EXT_shader_pixel_local_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_shader_pixel_local_storage"))))]
pub const GL_SHADER_PIXEL_LOCAL_STORAGE_EXT: GLenum = 0x8F64;
#[doc = "`GL_SHADER_SOURCE_LENGTH: GLenum = 0x8B88`"]
#[doc = "* **Group:** ShaderParameterName"]
pub const GL_SHADER_SOURCE_LENGTH: GLenum = 0x8B88;
#[doc = "`GL_SHADER_STORAGE_BARRIER_BIT: GLbitfield = 0x00002000`"]
#[doc = "* **Group:** MemoryBarrierMask"]
pub const GL_SHADER_STORAGE_BARRIER_BIT: GLbitfield = 0x00002000;
#[doc = "`GL_SHADER_STORAGE_BLOCK: GLenum = 0x92E6`"]
#[doc = "* **Group:** ProgramInterface"]
pub const GL_SHADER_STORAGE_BLOCK: GLenum = 0x92E6;
#[doc = "`GL_SHADER_STORAGE_BUFFER: GLenum = 0x90D2`"]
#[doc = "* **Groups:** CopyBufferSubDataTarget, BufferTargetARB, BufferStorageTarget"]
pub const GL_SHADER_STORAGE_BUFFER: GLenum = 0x90D2;
#[doc = "`GL_SHADER_STORAGE_BUFFER_BINDING: GLenum = 0x90D3`"]
#[doc = "* **Group:** GetPName"]
pub const GL_SHADER_STORAGE_BUFFER_BINDING: GLenum = 0x90D3;
#[doc = "`GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x90DF`"]
#[doc = "* **Group:** GetPName"]
pub const GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x90DF;
#[doc = "`GL_SHADER_STORAGE_BUFFER_SIZE: GLenum = 0x90D5`"]
#[doc = "* **Group:** GetPName"]
pub const GL_SHADER_STORAGE_BUFFER_SIZE: GLenum = 0x90D5;
#[doc = "`GL_SHADER_STORAGE_BUFFER_START: GLenum = 0x90D4`"]
#[doc = "* **Group:** GetPName"]
pub const GL_SHADER_STORAGE_BUFFER_START: GLenum = 0x90D4;
#[doc = "`GL_SHADER_TYPE: GLenum = 0x8B4F`"]
#[doc = "* **Group:** ShaderParameterName"]
pub const GL_SHADER_TYPE: GLenum = 0x8B4F;
#[doc = "`GL_SHADING_LANGUAGE_VERSION: GLenum = 0x8B8C`"]
#[doc = "* **Group:** StringName"]
pub const GL_SHADING_LANGUAGE_VERSION: GLenum = 0x8B8C;
#[doc = "`GL_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV: GLenum = 0x956F`"]
#[cfg(any(feature = "GL_NV_shading_rate_image"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shading_rate_image"))))]
pub const GL_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV: GLenum = 0x956F;
#[doc = "`GL_SHADING_RATE_1X1_PIXELS_QCOM: GLenum = 0x96A6`"]
#[doc = "* **Group:** ShadingRateQCOM"]
#[cfg(any(feature = "GL_QCOM_shading_rate"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_shading_rate"))))]
pub const GL_SHADING_RATE_1X1_PIXELS_QCOM: GLenum = 0x96A6;
#[doc = "`GL_SHADING_RATE_1X2_PIXELS_QCOM: GLenum = 0x96A7`"]
#[doc = "* **Group:** ShadingRateQCOM"]
#[cfg(any(feature = "GL_QCOM_shading_rate"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_shading_rate"))))]
pub const GL_SHADING_RATE_1X2_PIXELS_QCOM: GLenum = 0x96A7;
#[doc = "`GL_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS_NV: GLenum = 0x9566`"]
#[cfg(any(feature = "GL_NV_shading_rate_image"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shading_rate_image"))))]
pub const GL_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS_NV: GLenum = 0x9566;
#[doc = "`GL_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS_NV: GLenum = 0x9567`"]
#[cfg(any(feature = "GL_NV_shading_rate_image"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shading_rate_image"))))]
pub const GL_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS_NV: GLenum = 0x9567;
#[doc = "`GL_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS_NV: GLenum = 0x9568`"]
#[cfg(any(feature = "GL_NV_shading_rate_image"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shading_rate_image"))))]
pub const GL_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS_NV: GLenum = 0x9568;
#[doc = "`GL_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS_NV: GLenum = 0x9569`"]
#[cfg(any(feature = "GL_NV_shading_rate_image"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shading_rate_image"))))]
pub const GL_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS_NV: GLenum = 0x9569;
#[doc = "`GL_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS_NV: GLenum = 0x956A`"]
#[cfg(any(feature = "GL_NV_shading_rate_image"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shading_rate_image"))))]
pub const GL_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS_NV: GLenum = 0x956A;
#[doc = "`GL_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS_NV: GLenum = 0x956B`"]
#[cfg(any(feature = "GL_NV_shading_rate_image"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shading_rate_image"))))]
pub const GL_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS_NV: GLenum = 0x956B;
#[doc = "`GL_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV: GLenum = 0x9565`"]
#[cfg(any(feature = "GL_NV_shading_rate_image"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shading_rate_image"))))]
pub const GL_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV: GLenum = 0x9565;
#[doc = "`GL_SHADING_RATE_2X1_PIXELS_QCOM: GLenum = 0x96A8`"]
#[doc = "* **Group:** ShadingRateQCOM"]
#[cfg(any(feature = "GL_QCOM_shading_rate"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_shading_rate"))))]
pub const GL_SHADING_RATE_2X1_PIXELS_QCOM: GLenum = 0x96A8;
#[doc = "`GL_SHADING_RATE_2X2_PIXELS_QCOM: GLenum = 0x96A9`"]
#[doc = "* **Group:** ShadingRateQCOM"]
#[cfg(any(feature = "GL_QCOM_shading_rate"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_shading_rate"))))]
pub const GL_SHADING_RATE_2X2_PIXELS_QCOM: GLenum = 0x96A9;
#[doc = "`GL_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV: GLenum = 0x956C`"]
#[cfg(any(feature = "GL_NV_shading_rate_image"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shading_rate_image"))))]
pub const GL_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV: GLenum = 0x956C;
#[doc = "`GL_SHADING_RATE_4X2_PIXELS_QCOM: GLenum = 0x96AC`"]
#[doc = "* **Group:** ShadingRateQCOM"]
#[cfg(any(feature = "GL_QCOM_shading_rate"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_shading_rate"))))]
pub const GL_SHADING_RATE_4X2_PIXELS_QCOM: GLenum = 0x96AC;
#[doc = "`GL_SHADING_RATE_4X4_PIXELS_QCOM: GLenum = 0x96AE`"]
#[doc = "* **Group:** ShadingRateQCOM"]
#[cfg(any(feature = "GL_QCOM_shading_rate"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_shading_rate"))))]
pub const GL_SHADING_RATE_4X4_PIXELS_QCOM: GLenum = 0x96AE;
#[doc = "`GL_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV: GLenum = 0x956D`"]
#[cfg(any(feature = "GL_NV_shading_rate_image"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shading_rate_image"))))]
pub const GL_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV: GLenum = 0x956D;
#[doc = "`GL_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV: GLenum = 0x956E`"]
#[cfg(any(feature = "GL_NV_shading_rate_image"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shading_rate_image"))))]
pub const GL_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV: GLenum = 0x956E;
#[doc = "`GL_SHADING_RATE_IMAGE_BINDING_NV: GLenum = 0x955B`"]
#[cfg(any(feature = "GL_NV_shading_rate_image"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shading_rate_image"))))]
pub const GL_SHADING_RATE_IMAGE_BINDING_NV: GLenum = 0x955B;
#[doc = "`GL_SHADING_RATE_IMAGE_NV: GLenum = 0x9563`"]
#[cfg(any(feature = "GL_NV_shading_rate_image"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shading_rate_image"))))]
pub const GL_SHADING_RATE_IMAGE_NV: GLenum = 0x9563;
#[doc = "`GL_SHADING_RATE_IMAGE_PALETTE_COUNT_NV: GLenum = 0x95B2`"]
#[doc = "* **Group:** GetPName"]
#[cfg(any(feature = "GL_NV_primitive_shading_rate"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_primitive_shading_rate"))))]
pub const GL_SHADING_RATE_IMAGE_PALETTE_COUNT_NV: GLenum = 0x95B2;
#[doc = "`GL_SHADING_RATE_IMAGE_PALETTE_SIZE_NV: GLenum = 0x955E`"]
#[cfg(any(feature = "GL_NV_shading_rate_image"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shading_rate_image"))))]
pub const GL_SHADING_RATE_IMAGE_PALETTE_SIZE_NV: GLenum = 0x955E;
#[doc = "`GL_SHADING_RATE_IMAGE_PER_PRIMITIVE_NV: GLenum = 0x95B1`"]
#[doc = "* **Groups:** EnableCap, GetPName"]
#[cfg(any(feature = "GL_NV_primitive_shading_rate"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_primitive_shading_rate"))))]
pub const GL_SHADING_RATE_IMAGE_PER_PRIMITIVE_NV: GLenum = 0x95B1;
#[doc = "`GL_SHADING_RATE_IMAGE_TEXEL_HEIGHT_NV: GLenum = 0x955D`"]
#[cfg(any(feature = "GL_NV_shading_rate_image"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shading_rate_image"))))]
pub const GL_SHADING_RATE_IMAGE_TEXEL_HEIGHT_NV: GLenum = 0x955D;
#[doc = "`GL_SHADING_RATE_IMAGE_TEXEL_WIDTH_NV: GLenum = 0x955C`"]
#[cfg(any(feature = "GL_NV_shading_rate_image"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shading_rate_image"))))]
pub const GL_SHADING_RATE_IMAGE_TEXEL_WIDTH_NV: GLenum = 0x955C;
#[doc = "`GL_SHADING_RATE_NO_INVOCATIONS_NV: GLenum = 0x9564`"]
#[cfg(any(feature = "GL_NV_shading_rate_image"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shading_rate_image"))))]
pub const GL_SHADING_RATE_NO_INVOCATIONS_NV: GLenum = 0x9564;
#[doc = "`GL_SHADING_RATE_PRESERVE_ASPECT_RATIO_QCOM: GLenum = 0x96A5`"]
#[doc = "* **Group:** EnableCap"]
#[cfg(any(feature = "GL_QCOM_shading_rate"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_shading_rate"))))]
pub const GL_SHADING_RATE_PRESERVE_ASPECT_RATIO_QCOM: GLenum = 0x96A5;
#[doc = "`GL_SHADING_RATE_QCOM: GLenum = 0x96A4`"]
#[doc = "* **Group:** GetPName"]
#[cfg(any(feature = "GL_QCOM_shading_rate"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_shading_rate"))))]
pub const GL_SHADING_RATE_QCOM: GLenum = 0x96A4;
#[doc = "`GL_SHADING_RATE_SAMPLE_ORDER_DEFAULT_NV: GLenum = 0x95AE`"]
#[cfg(any(feature = "GL_NV_shading_rate_image"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shading_rate_image"))))]
pub const GL_SHADING_RATE_SAMPLE_ORDER_DEFAULT_NV: GLenum = 0x95AE;
#[doc = "`GL_SHADING_RATE_SAMPLE_ORDER_PIXEL_MAJOR_NV: GLenum = 0x95AF`"]
#[cfg(any(feature = "GL_NV_shading_rate_image"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shading_rate_image"))))]
pub const GL_SHADING_RATE_SAMPLE_ORDER_PIXEL_MAJOR_NV: GLenum = 0x95AF;
#[doc = "`GL_SHADING_RATE_SAMPLE_ORDER_SAMPLE_MAJOR_NV: GLenum = 0x95B0`"]
#[cfg(any(feature = "GL_NV_shading_rate_image"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shading_rate_image"))))]
pub const GL_SHADING_RATE_SAMPLE_ORDER_SAMPLE_MAJOR_NV: GLenum = 0x95B0;
#[doc = "`GL_SHARED_EDGE_NV: GLenum = 0xC0`"]
#[cfg(any(feature = "GL_NV_path_rendering_shared_edge"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering_shared_edge"))))]
pub const GL_SHARED_EDGE_NV: GLenum = 0xC0;
#[doc = "`GL_SHORT: GLenum = 0x1402`"]
#[doc = "* **Groups:** VertexAttribIType, SecondaryColorPointerTypeIBM, WeightPointerTypeARB, TangentPointerTypeEXT, BinormalPointerTypeEXT, IndexPointerType, ListNameType, NormalPointerType, PixelType, TexCoordPointerType, VertexPointerType, VertexAttribType, VertexAttribPointerType"]
pub const GL_SHORT: GLenum = 0x1402;
#[doc = "`GL_SIGNALED: GLenum = 0x9119`"]
pub const GL_SIGNALED: GLenum = 0x9119;
#[doc = "`GL_SIGNALED_APPLE: GLenum = 0x9119`"]
#[cfg(any(feature = "GL_APPLE_sync"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_sync"))))]
pub const GL_SIGNALED_APPLE: GLenum = 0x9119;
#[doc = "`GL_SIGNED_NORMALIZED: GLenum = 0x8F9C`"]
pub const GL_SIGNED_NORMALIZED: GLenum = 0x8F9C;
#[doc = "`GL_SKIP_DECODE_EXT: GLenum = 0x8A4A`"]
#[cfg(any(feature = "GL_EXT_texture_sRGB_decode"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_sRGB_decode"))))]
pub const GL_SKIP_DECODE_EXT: GLenum = 0x8A4A;
#[doc = "`GL_SKIP_MISSING_GLYPH_NV: GLenum = 0x90A9`"]
#[doc = "* **Group:** PathHandleMissingGlyphs"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_SKIP_MISSING_GLYPH_NV: GLenum = 0x90A9;
#[doc = "`GL_SLUMINANCE8_ALPHA8_NV: GLenum = 0x8C45`"]
#[cfg(any(feature = "GL_NV_sRGB_formats"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_sRGB_formats"))))]
pub const GL_SLUMINANCE8_ALPHA8_NV: GLenum = 0x8C45;
#[doc = "`GL_SLUMINANCE8_NV: GLenum = 0x8C47`"]
#[cfg(any(feature = "GL_NV_sRGB_formats"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_sRGB_formats"))))]
pub const GL_SLUMINANCE8_NV: GLenum = 0x8C47;
#[doc = "`GL_SLUMINANCE_ALPHA_NV: GLenum = 0x8C44`"]
#[cfg(any(feature = "GL_NV_sRGB_formats"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_sRGB_formats"))))]
pub const GL_SLUMINANCE_ALPHA_NV: GLenum = 0x8C44;
#[doc = "`GL_SLUMINANCE_NV: GLenum = 0x8C46`"]
#[cfg(any(feature = "GL_NV_sRGB_formats"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_sRGB_formats"))))]
pub const GL_SLUMINANCE_NV: GLenum = 0x8C46;
#[doc = "`GL_SMALL_CCW_ARC_TO_NV: GLenum = 0x12`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_SMALL_CCW_ARC_TO_NV: GLenum = 0x12;
#[doc = "`GL_SMALL_CW_ARC_TO_NV: GLenum = 0x14`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_SMALL_CW_ARC_TO_NV: GLenum = 0x14;
#[doc = "`GL_SMAPHS30_PROGRAM_BINARY_DMP: GLenum = 0x9251`"]
#[cfg(any(feature = "GL_DMP_program_binary"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_DMP_program_binary"))))]
pub const GL_SMAPHS30_PROGRAM_BINARY_DMP: GLenum = 0x9251;
#[doc = "`GL_SMAPHS_PROGRAM_BINARY_DMP: GLenum = 0x9252`"]
#[cfg(any(feature = "GL_DMP_program_binary"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_DMP_program_binary"))))]
pub const GL_SMAPHS_PROGRAM_BINARY_DMP: GLenum = 0x9252;
#[doc = "`GL_SMOOTH_CUBIC_CURVE_TO_NV: GLenum = 0x10`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_SMOOTH_CUBIC_CURVE_TO_NV: GLenum = 0x10;
#[doc = "`GL_SMOOTH_QUADRATIC_CURVE_TO_NV: GLenum = 0x0E`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_SMOOTH_QUADRATIC_CURVE_TO_NV: GLenum = 0x0E;
#[doc = "`GL_SOFTLIGHT: GLenum = 0x929C`"]
pub const GL_SOFTLIGHT: GLenum = 0x929C;
#[doc = "`GL_SOFTLIGHT_KHR: GLenum = 0x929C`"]
#[cfg(any(feature = "GL_KHR_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_blend_equation_advanced"))))]
pub const GL_SOFTLIGHT_KHR: GLenum = 0x929C;
#[doc = "`GL_SOFTLIGHT_NV: GLenum = 0x929C`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_SOFTLIGHT_NV: GLenum = 0x929C;
#[doc = "`GL_SPARSE_TEXTURE_FULL_ARRAY_CUBE_MIPMAPS_EXT: GLenum = 0x91A9`"]
#[cfg(any(feature = "GL_EXT_sparse_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_sparse_texture"))))]
pub const GL_SPARSE_TEXTURE_FULL_ARRAY_CUBE_MIPMAPS_EXT: GLenum = 0x91A9;
#[doc = "`GL_SQUARE_NV: GLenum = 0x90A3`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_SQUARE_NV: GLenum = 0x90A3;
#[doc = "`GL_SR8_EXT: GLenum = 0x8FBD`"]
#[doc = "* **Group:** InternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_sRGB_R8"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_sRGB_R8"))))]
pub const GL_SR8_EXT: GLenum = 0x8FBD;
#[doc = "`GL_SRC1_ALPHA_EXT: GLenum = 0x8589`"]
#[cfg(any(feature = "GL_EXT_blend_func_extended"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_blend_func_extended"))))]
pub const GL_SRC1_ALPHA_EXT: GLenum = 0x8589;
#[doc = "`GL_SRC1_COLOR_EXT: GLenum = 0x88F9`"]
#[cfg(any(feature = "GL_EXT_blend_func_extended"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_blend_func_extended"))))]
pub const GL_SRC1_COLOR_EXT: GLenum = 0x88F9;
#[doc = "`GL_SRC_ALPHA: GLenum = 0x0302`"]
#[doc = "* **Group:** BlendingFactor"]
pub const GL_SRC_ALPHA: GLenum = 0x0302;
#[doc = "`GL_SRC_ALPHA_SATURATE: GLenum = 0x0308`"]
#[doc = "* **Group:** BlendingFactor"]
pub const GL_SRC_ALPHA_SATURATE: GLenum = 0x0308;
#[doc = "`GL_SRC_ALPHA_SATURATE_EXT: GLenum = 0x0308`"]
#[cfg(any(feature = "GL_EXT_blend_func_extended"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_blend_func_extended"))))]
pub const GL_SRC_ALPHA_SATURATE_EXT: GLenum = 0x0308;
#[doc = "`GL_SRC_ATOP_NV: GLenum = 0x928E`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_SRC_ATOP_NV: GLenum = 0x928E;
#[doc = "`GL_SRC_COLOR: GLenum = 0x0300`"]
#[doc = "* **Group:** BlendingFactor"]
pub const GL_SRC_COLOR: GLenum = 0x0300;
#[doc = "`GL_SRC_IN_NV: GLenum = 0x928A`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_SRC_IN_NV: GLenum = 0x928A;
#[doc = "`GL_SRC_NV: GLenum = 0x9286`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_SRC_NV: GLenum = 0x9286;
#[doc = "`GL_SRC_OUT_NV: GLenum = 0x928C`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_SRC_OUT_NV: GLenum = 0x928C;
#[doc = "`GL_SRC_OVER_NV: GLenum = 0x9288`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_SRC_OVER_NV: GLenum = 0x9288;
#[doc = "`GL_SRG8_EXT: GLenum = 0x8FBE`"]
#[doc = "* **Group:** InternalFormat"]
#[cfg(any(feature = "GL_EXT_texture_sRGB_RG8"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_sRGB_RG8"))))]
pub const GL_SRG8_EXT: GLenum = 0x8FBE;
#[doc = "`GL_SRGB: GLenum = 0x8C40`"]
#[doc = "* **Group:** InternalFormat"]
pub const GL_SRGB: GLenum = 0x8C40;
#[doc = "`GL_SRGB8: GLenum = 0x8C41`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_SRGB8: GLenum = 0x8C41;
#[doc = "`GL_SRGB8_ALPHA8: GLenum = 0x8C43`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_SRGB8_ALPHA8: GLenum = 0x8C43;
#[doc = "`GL_SRGB8_ALPHA8_EXT: GLenum = 0x8C43`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_EXT_sRGB"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_sRGB"))))]
pub const GL_SRGB8_ALPHA8_EXT: GLenum = 0x8C43;
#[doc = "`GL_SRGB8_NV: GLenum = 0x8C41`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_NV_sRGB_formats"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_sRGB_formats"))))]
pub const GL_SRGB8_NV: GLenum = 0x8C41;
#[doc = "`GL_SRGB_ALPHA_EXT: GLenum = 0x8C42`"]
#[doc = "* **Group:** InternalFormat"]
#[cfg(any(feature = "GL_EXT_sRGB"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_sRGB"))))]
pub const GL_SRGB_ALPHA_EXT: GLenum = 0x8C42;
#[doc = "`GL_SRGB_EXT: GLenum = 0x8C40`"]
#[doc = "* **Group:** InternalFormat"]
#[cfg(any(feature = "GL_EXT_sRGB"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_sRGB"))))]
pub const GL_SRGB_EXT: GLenum = 0x8C40;
#[doc = "`GL_STACK_OVERFLOW: GLenum = 0x0503`"]
#[doc = "* **Group:** ErrorCode"]
pub const GL_STACK_OVERFLOW: GLenum = 0x0503;
#[doc = "`GL_STACK_OVERFLOW_KHR: GLenum = 0x0503`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_STACK_OVERFLOW_KHR: GLenum = 0x0503;
#[doc = "`GL_STACK_UNDERFLOW: GLenum = 0x0504`"]
#[doc = "* **Group:** ErrorCode"]
pub const GL_STACK_UNDERFLOW: GLenum = 0x0504;
#[doc = "`GL_STACK_UNDERFLOW_KHR: GLenum = 0x0504`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_STACK_UNDERFLOW_KHR: GLenum = 0x0504;
#[doc = "`GL_STANDARD_FONT_FORMAT_NV: GLenum = 0x936C`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_STANDARD_FONT_FORMAT_NV: GLenum = 0x936C;
#[doc = "`GL_STANDARD_FONT_NAME_NV: GLenum = 0x9072`"]
#[doc = "* **Group:** PathFontTarget"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_STANDARD_FONT_NAME_NV: GLenum = 0x9072;
#[doc = "`GL_STATE_RESTORE: GLenum = 0x8BDC`"]
#[cfg(any(feature = "GL_QCOM_extended_get"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_extended_get"))))]
pub const GL_STATE_RESTORE: GLenum = 0x8BDC;
#[doc = "`GL_STATIC_COPY: GLenum = 0x88E6`"]
#[doc = "* **Groups:** VertexBufferObjectUsage, BufferUsageARB"]
pub const GL_STATIC_COPY: GLenum = 0x88E6;
#[doc = "`GL_STATIC_DRAW: GLenum = 0x88E4`"]
#[doc = "* **Groups:** VertexBufferObjectUsage, BufferUsageARB"]
pub const GL_STATIC_DRAW: GLenum = 0x88E4;
#[doc = "`GL_STATIC_READ: GLenum = 0x88E5`"]
#[doc = "* **Groups:** VertexBufferObjectUsage, BufferUsageARB"]
pub const GL_STATIC_READ: GLenum = 0x88E5;
#[doc = "`GL_STENCIL: GLenum = 0x1802`"]
#[doc = "* **Groups:** Buffer, PixelCopyType, InvalidateFramebufferAttachment"]
pub const GL_STENCIL: GLenum = 0x1802;
#[doc = "`GL_STENCIL_ATTACHMENT: GLenum = 0x8D20`"]
#[doc = "* **Group:** FramebufferAttachment"]
pub const GL_STENCIL_ATTACHMENT: GLenum = 0x8D20;
#[doc = "`GL_STENCIL_BACK_FAIL: GLenum = 0x8801`"]
#[doc = "* **Group:** GetPName"]
pub const GL_STENCIL_BACK_FAIL: GLenum = 0x8801;
#[doc = "`GL_STENCIL_BACK_FUNC: GLenum = 0x8800`"]
#[doc = "* **Group:** GetPName"]
pub const GL_STENCIL_BACK_FUNC: GLenum = 0x8800;
#[doc = "`GL_STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 0x8802`"]
#[doc = "* **Group:** GetPName"]
pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 0x8802;
#[doc = "`GL_STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 0x8803`"]
#[doc = "* **Group:** GetPName"]
pub const GL_STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 0x8803;
#[doc = "`GL_STENCIL_BACK_REF: GLenum = 0x8CA3`"]
#[doc = "* **Group:** GetPName"]
pub const GL_STENCIL_BACK_REF: GLenum = 0x8CA3;
#[doc = "`GL_STENCIL_BACK_VALUE_MASK: GLenum = 0x8CA4`"]
#[doc = "* **Group:** GetPName"]
pub const GL_STENCIL_BACK_VALUE_MASK: GLenum = 0x8CA4;
#[doc = "`GL_STENCIL_BACK_WRITEMASK: GLenum = 0x8CA5`"]
#[doc = "* **Group:** GetPName"]
pub const GL_STENCIL_BACK_WRITEMASK: GLenum = 0x8CA5;
#[doc = "`GL_STENCIL_BITS: GLenum = 0x0D57`"]
#[doc = "* **Group:** GetPName"]
pub const GL_STENCIL_BITS: GLenum = 0x0D57;
#[doc = "`GL_STENCIL_BUFFER_BIT: GLbitfield = 0x00000400`"]
#[doc = "* **Groups:** ClearBufferMask, AttribMask"]
pub const GL_STENCIL_BUFFER_BIT: GLbitfield = 0x00000400;
#[doc = "`GL_STENCIL_BUFFER_BIT0_QCOM: GLbitfield = 0x00010000`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_STENCIL_BUFFER_BIT0_QCOM: GLbitfield = 0x00010000;
#[doc = "`GL_STENCIL_BUFFER_BIT1_QCOM: GLbitfield = 0x00020000`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_STENCIL_BUFFER_BIT1_QCOM: GLbitfield = 0x00020000;
#[doc = "`GL_STENCIL_BUFFER_BIT2_QCOM: GLbitfield = 0x00040000`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_STENCIL_BUFFER_BIT2_QCOM: GLbitfield = 0x00040000;
#[doc = "`GL_STENCIL_BUFFER_BIT3_QCOM: GLbitfield = 0x00080000`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_STENCIL_BUFFER_BIT3_QCOM: GLbitfield = 0x00080000;
#[doc = "`GL_STENCIL_BUFFER_BIT4_QCOM: GLbitfield = 0x00100000`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_STENCIL_BUFFER_BIT4_QCOM: GLbitfield = 0x00100000;
#[doc = "`GL_STENCIL_BUFFER_BIT5_QCOM: GLbitfield = 0x00200000`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_STENCIL_BUFFER_BIT5_QCOM: GLbitfield = 0x00200000;
#[doc = "`GL_STENCIL_BUFFER_BIT6_QCOM: GLbitfield = 0x00400000`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_STENCIL_BUFFER_BIT6_QCOM: GLbitfield = 0x00400000;
#[doc = "`GL_STENCIL_BUFFER_BIT7_QCOM: GLbitfield = 0x00800000`"]
#[doc = "* **Group:** BufferBitQCOM"]
#[cfg(any(feature = "GL_QCOM_tiled_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_tiled_rendering"))))]
pub const GL_STENCIL_BUFFER_BIT7_QCOM: GLbitfield = 0x00800000;
#[doc = "`GL_STENCIL_CLEAR_VALUE: GLenum = 0x0B91`"]
#[doc = "* **Group:** GetPName"]
pub const GL_STENCIL_CLEAR_VALUE: GLenum = 0x0B91;
#[doc = "`GL_STENCIL_EXT: GLenum = 0x1802`"]
#[doc = "* **Group:** PixelCopyType"]
#[cfg(any(feature = "GL_EXT_discard_framebuffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_discard_framebuffer"))))]
pub const GL_STENCIL_EXT: GLenum = 0x1802;
#[doc = "`GL_STENCIL_FAIL: GLenum = 0x0B94`"]
#[doc = "* **Group:** GetPName"]
pub const GL_STENCIL_FAIL: GLenum = 0x0B94;
#[doc = "`GL_STENCIL_FUNC: GLenum = 0x0B92`"]
#[doc = "* **Group:** GetPName"]
pub const GL_STENCIL_FUNC: GLenum = 0x0B92;
#[doc = "`GL_STENCIL_INDEX: GLenum = 0x1901`"]
#[doc = "* **Groups:** InternalFormat, PixelFormat"]
pub const GL_STENCIL_INDEX: GLenum = 0x1901;
#[doc = "`GL_STENCIL_INDEX1_OES: GLenum = 0x8D46`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_stencil1"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_stencil1"))))]
pub const GL_STENCIL_INDEX1_OES: GLenum = 0x8D46;
#[doc = "`GL_STENCIL_INDEX4_OES: GLenum = 0x8D47`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_stencil4"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_stencil4"))))]
pub const GL_STENCIL_INDEX4_OES: GLenum = 0x8D47;
#[doc = "`GL_STENCIL_INDEX8: GLenum = 0x8D48`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
pub const GL_STENCIL_INDEX8: GLenum = 0x8D48;
#[doc = "`GL_STENCIL_INDEX8_OES: GLenum = 0x8D48`"]
#[doc = "* **Groups:** InternalFormat, SizedInternalFormat"]
#[cfg(any(feature = "GL_OES_texture_stencil8"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_stencil8"))))]
pub const GL_STENCIL_INDEX8_OES: GLenum = 0x8D48;
#[doc = "`GL_STENCIL_INDEX_OES: GLenum = 0x1901`"]
#[doc = "* **Group:** InternalFormat"]
#[cfg(any(feature = "GL_OES_texture_stencil8"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_stencil8"))))]
pub const GL_STENCIL_INDEX_OES: GLenum = 0x1901;
#[doc = "`GL_STENCIL_PASS_DEPTH_FAIL: GLenum = 0x0B95`"]
#[doc = "* **Group:** GetPName"]
pub const GL_STENCIL_PASS_DEPTH_FAIL: GLenum = 0x0B95;
#[doc = "`GL_STENCIL_PASS_DEPTH_PASS: GLenum = 0x0B96`"]
#[doc = "* **Group:** GetPName"]
pub const GL_STENCIL_PASS_DEPTH_PASS: GLenum = 0x0B96;
#[doc = "`GL_STENCIL_REF: GLenum = 0x0B97`"]
#[doc = "* **Group:** GetPName"]
pub const GL_STENCIL_REF: GLenum = 0x0B97;
#[doc = "`GL_STENCIL_SAMPLES_NV: GLenum = 0x932E`"]
#[cfg(any(feature = "GL_NV_framebuffer_mixed_samples"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_framebuffer_mixed_samples"))))]
pub const GL_STENCIL_SAMPLES_NV: GLenum = 0x932E;
#[doc = "`GL_STENCIL_TEST: GLenum = 0x0B90`"]
#[doc = "* **Groups:** GetPName, EnableCap"]
pub const GL_STENCIL_TEST: GLenum = 0x0B90;
#[doc = "`GL_STENCIL_VALUE_MASK: GLenum = 0x0B93`"]
#[doc = "* **Group:** GetPName"]
pub const GL_STENCIL_VALUE_MASK: GLenum = 0x0B93;
#[doc = "`GL_STENCIL_WRITEMASK: GLenum = 0x0B98`"]
#[doc = "* **Group:** GetPName"]
pub const GL_STENCIL_WRITEMASK: GLenum = 0x0B98;
#[doc = "`GL_STREAM_COPY: GLenum = 0x88E2`"]
#[doc = "* **Groups:** VertexBufferObjectUsage, BufferUsageARB"]
pub const GL_STREAM_COPY: GLenum = 0x88E2;
#[doc = "`GL_STREAM_DRAW: GLenum = 0x88E0`"]
#[doc = "* **Groups:** VertexBufferObjectUsage, BufferUsageARB"]
pub const GL_STREAM_DRAW: GLenum = 0x88E0;
#[doc = "`GL_STREAM_READ: GLenum = 0x88E1`"]
#[doc = "* **Groups:** VertexBufferObjectUsage, BufferUsageARB"]
pub const GL_STREAM_READ: GLenum = 0x88E1;
#[doc = "`GL_SUBGROUP_FEATURE_ARITHMETIC_BIT_KHR: GLbitfield = 0x00000004`"]
#[doc = "* **Group:** SubgroupSupportedFeatures"]
#[cfg(any(feature = "GL_KHR_shader_subgroup"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_shader_subgroup"))))]
pub const GL_SUBGROUP_FEATURE_ARITHMETIC_BIT_KHR: GLbitfield = 0x00000004;
#[doc = "`GL_SUBGROUP_FEATURE_BALLOT_BIT_KHR: GLbitfield = 0x00000008`"]
#[doc = "* **Group:** SubgroupSupportedFeatures"]
#[cfg(any(feature = "GL_KHR_shader_subgroup"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_shader_subgroup"))))]
pub const GL_SUBGROUP_FEATURE_BALLOT_BIT_KHR: GLbitfield = 0x00000008;
#[doc = "`GL_SUBGROUP_FEATURE_BASIC_BIT_KHR: GLbitfield = 0x00000001`"]
#[doc = "* **Group:** SubgroupSupportedFeatures"]
#[cfg(any(feature = "GL_KHR_shader_subgroup"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_shader_subgroup"))))]
pub const GL_SUBGROUP_FEATURE_BASIC_BIT_KHR: GLbitfield = 0x00000001;
#[doc = "`GL_SUBGROUP_FEATURE_CLUSTERED_BIT_KHR: GLbitfield = 0x00000040`"]
#[doc = "* **Group:** SubgroupSupportedFeatures"]
#[cfg(any(feature = "GL_KHR_shader_subgroup"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_shader_subgroup"))))]
pub const GL_SUBGROUP_FEATURE_CLUSTERED_BIT_KHR: GLbitfield = 0x00000040;
#[doc = "`GL_SUBGROUP_FEATURE_PARTITIONED_BIT_NV: GLbitfield = 0x00000100`"]
#[doc = "* **Group:** SubgroupSupportedFeatures"]
#[cfg(any(feature = "GL_NV_shader_subgroup_partitioned"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_shader_subgroup_partitioned"))))]
pub const GL_SUBGROUP_FEATURE_PARTITIONED_BIT_NV: GLbitfield = 0x00000100;
#[doc = "`GL_SUBGROUP_FEATURE_QUAD_BIT_KHR: GLbitfield = 0x00000080`"]
#[doc = "* **Group:** SubgroupSupportedFeatures"]
#[cfg(any(feature = "GL_KHR_shader_subgroup"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_shader_subgroup"))))]
pub const GL_SUBGROUP_FEATURE_QUAD_BIT_KHR: GLbitfield = 0x00000080;
#[doc = "`GL_SUBGROUP_FEATURE_SHUFFLE_BIT_KHR: GLbitfield = 0x00000010`"]
#[doc = "* **Group:** SubgroupSupportedFeatures"]
#[cfg(any(feature = "GL_KHR_shader_subgroup"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_shader_subgroup"))))]
pub const GL_SUBGROUP_FEATURE_SHUFFLE_BIT_KHR: GLbitfield = 0x00000010;
#[doc = "`GL_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT_KHR: GLbitfield = 0x00000020`"]
#[doc = "* **Group:** SubgroupSupportedFeatures"]
#[cfg(any(feature = "GL_KHR_shader_subgroup"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_shader_subgroup"))))]
pub const GL_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT_KHR: GLbitfield = 0x00000020;
#[doc = "`GL_SUBGROUP_FEATURE_VOTE_BIT_KHR: GLbitfield = 0x00000002`"]
#[doc = "* **Group:** SubgroupSupportedFeatures"]
#[cfg(any(feature = "GL_KHR_shader_subgroup"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_shader_subgroup"))))]
pub const GL_SUBGROUP_FEATURE_VOTE_BIT_KHR: GLbitfield = 0x00000002;
#[doc = "`GL_SUBGROUP_QUAD_ALL_STAGES_KHR: GLenum = 0x9535`"]
#[cfg(any(feature = "GL_KHR_shader_subgroup"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_shader_subgroup"))))]
pub const GL_SUBGROUP_QUAD_ALL_STAGES_KHR: GLenum = 0x9535;
#[doc = "`GL_SUBGROUP_SIZE_KHR: GLenum = 0x9532`"]
#[cfg(any(feature = "GL_KHR_shader_subgroup"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_shader_subgroup"))))]
pub const GL_SUBGROUP_SIZE_KHR: GLenum = 0x9532;
#[doc = "`GL_SUBGROUP_SUPPORTED_FEATURES_KHR: GLenum = 0x9534`"]
#[cfg(any(feature = "GL_KHR_shader_subgroup"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_shader_subgroup"))))]
pub const GL_SUBGROUP_SUPPORTED_FEATURES_KHR: GLenum = 0x9534;
#[doc = "`GL_SUBGROUP_SUPPORTED_STAGES_KHR: GLenum = 0x9533`"]
#[cfg(any(feature = "GL_KHR_shader_subgroup"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_shader_subgroup"))))]
pub const GL_SUBGROUP_SUPPORTED_STAGES_KHR: GLenum = 0x9533;
#[doc = "`GL_SUBPIXEL_BITS: GLenum = 0x0D50`"]
#[doc = "* **Group:** GetPName"]
pub const GL_SUBPIXEL_BITS: GLenum = 0x0D50;
#[doc = "`GL_SUBPIXEL_PRECISION_BIAS_X_BITS_NV: GLenum = 0x9347`"]
#[cfg(any(feature = "GL_NV_conservative_raster"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_conservative_raster"))))]
pub const GL_SUBPIXEL_PRECISION_BIAS_X_BITS_NV: GLenum = 0x9347;
#[doc = "`GL_SUBPIXEL_PRECISION_BIAS_Y_BITS_NV: GLenum = 0x9348`"]
#[cfg(any(feature = "GL_NV_conservative_raster"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_conservative_raster"))))]
pub const GL_SUBPIXEL_PRECISION_BIAS_Y_BITS_NV: GLenum = 0x9348;
#[doc = "`GL_SUPERSAMPLE_SCALE_X_NV: GLenum = 0x9372`"]
#[cfg(any(feature = "GL_NV_internalformat_sample_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_internalformat_sample_query"))))]
pub const GL_SUPERSAMPLE_SCALE_X_NV: GLenum = 0x9372;
#[doc = "`GL_SUPERSAMPLE_SCALE_Y_NV: GLenum = 0x9373`"]
#[cfg(any(feature = "GL_NV_internalformat_sample_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_internalformat_sample_query"))))]
pub const GL_SUPERSAMPLE_SCALE_Y_NV: GLenum = 0x9373;
#[doc = "`GL_SUPPORTED_MULTISAMPLE_MODES_AMD: GLenum = 0x91B7`"]
#[cfg(any(feature = "GL_AMD_framebuffer_multisample_advanced"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_AMD_framebuffer_multisample_advanced")))
)]
pub const GL_SUPPORTED_MULTISAMPLE_MODES_AMD: GLenum = 0x91B7;
#[doc = "`GL_SYNC_CONDITION: GLenum = 0x9113`"]
#[doc = "* **Group:** SyncParameterName"]
pub const GL_SYNC_CONDITION: GLenum = 0x9113;
#[doc = "`GL_SYNC_CONDITION_APPLE: GLenum = 0x9113`"]
#[cfg(any(feature = "GL_APPLE_sync"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_sync"))))]
pub const GL_SYNC_CONDITION_APPLE: GLenum = 0x9113;
#[doc = "`GL_SYNC_FENCE: GLenum = 0x9116`"]
pub const GL_SYNC_FENCE: GLenum = 0x9116;
#[doc = "`GL_SYNC_FENCE_APPLE: GLenum = 0x9116`"]
#[cfg(any(feature = "GL_APPLE_sync"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_sync"))))]
pub const GL_SYNC_FENCE_APPLE: GLenum = 0x9116;
#[doc = "`GL_SYNC_FLAGS: GLenum = 0x9115`"]
#[doc = "* **Group:** SyncParameterName"]
pub const GL_SYNC_FLAGS: GLenum = 0x9115;
#[doc = "`GL_SYNC_FLAGS_APPLE: GLenum = 0x9115`"]
#[cfg(any(feature = "GL_APPLE_sync"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_sync"))))]
pub const GL_SYNC_FLAGS_APPLE: GLenum = 0x9115;
#[doc = "`GL_SYNC_FLUSH_COMMANDS_BIT: GLbitfield = 0x00000001`"]
#[doc = "* **Group:** SyncObjectMask"]
pub const GL_SYNC_FLUSH_COMMANDS_BIT: GLbitfield = 0x00000001;
#[doc = "`GL_SYNC_FLUSH_COMMANDS_BIT_APPLE: GLbitfield = 0x00000001`"]
#[doc = "* **Group:** SyncObjectMask"]
#[cfg(any(feature = "GL_APPLE_sync"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_sync"))))]
pub const GL_SYNC_FLUSH_COMMANDS_BIT_APPLE: GLbitfield = 0x00000001;
#[doc = "`GL_SYNC_GPU_COMMANDS_COMPLETE: GLenum = 0x9117`"]
#[doc = "* **Group:** SyncCondition"]
pub const GL_SYNC_GPU_COMMANDS_COMPLETE: GLenum = 0x9117;
#[doc = "`GL_SYNC_GPU_COMMANDS_COMPLETE_APPLE: GLenum = 0x9117`"]
#[cfg(any(feature = "GL_APPLE_sync"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_sync"))))]
pub const GL_SYNC_GPU_COMMANDS_COMPLETE_APPLE: GLenum = 0x9117;
#[doc = "`GL_SYNC_OBJECT_APPLE: GLenum = 0x8A53`"]
#[cfg(any(feature = "GL_APPLE_sync"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_sync"))))]
pub const GL_SYNC_OBJECT_APPLE: GLenum = 0x8A53;
#[doc = "`GL_SYNC_STATUS: GLenum = 0x9114`"]
#[doc = "* **Group:** SyncParameterName"]
pub const GL_SYNC_STATUS: GLenum = 0x9114;
#[doc = "`GL_SYNC_STATUS_APPLE: GLenum = 0x9114`"]
#[cfg(any(feature = "GL_APPLE_sync"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_sync"))))]
pub const GL_SYNC_STATUS_APPLE: GLenum = 0x9114;
#[doc = "`GL_SYSTEM_FONT_NAME_NV: GLenum = 0x9073`"]
#[doc = "* **Group:** PathFontTarget"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_SYSTEM_FONT_NAME_NV: GLenum = 0x9073;
#[doc = "`GL_TASK_SHADER_BIT_NV: GLbitfield = 0x00000080`"]
#[doc = "* **Group:** UseProgramStageMask"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_TASK_SHADER_BIT_NV: GLbitfield = 0x00000080;
#[doc = "`GL_TASK_SHADER_NV: GLenum = 0x955A`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_TASK_SHADER_NV: GLenum = 0x955A;
#[doc = "`GL_TASK_SUBROUTINE_NV: GLenum = 0x957D`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_TASK_SUBROUTINE_NV: GLenum = 0x957D;
#[doc = "`GL_TASK_SUBROUTINE_UNIFORM_NV: GLenum = 0x957F`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_TASK_SUBROUTINE_UNIFORM_NV: GLenum = 0x957F;
#[doc = "`GL_TASK_WORK_GROUP_SIZE_NV: GLenum = 0x953F`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_TASK_WORK_GROUP_SIZE_NV: GLenum = 0x953F;
#[doc = "`GL_TESS_CONTROL_OUTPUT_VERTICES: GLenum = 0x8E75`"]
pub const GL_TESS_CONTROL_OUTPUT_VERTICES: GLenum = 0x8E75;
#[doc = "`GL_TESS_CONTROL_OUTPUT_VERTICES_EXT: GLenum = 0x8E75`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_TESS_CONTROL_OUTPUT_VERTICES_EXT: GLenum = 0x8E75;
#[doc = "`GL_TESS_CONTROL_OUTPUT_VERTICES_OES: GLenum = 0x8E75`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_TESS_CONTROL_OUTPUT_VERTICES_OES: GLenum = 0x8E75;
#[doc = "`GL_TESS_CONTROL_SHADER: GLenum = 0x8E88`"]
#[doc = "* **Groups:** PipelineParameterName, ShaderType"]
pub const GL_TESS_CONTROL_SHADER: GLenum = 0x8E88;
#[doc = "`GL_TESS_CONTROL_SHADER_BIT: GLbitfield = 0x00000008`"]
#[doc = "* **Group:** UseProgramStageMask"]
pub const GL_TESS_CONTROL_SHADER_BIT: GLbitfield = 0x00000008;
#[doc = "`GL_TESS_CONTROL_SHADER_BIT_EXT: GLbitfield = 0x00000008`"]
#[doc = "* **Group:** UseProgramStageMask"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_TESS_CONTROL_SHADER_BIT_EXT: GLbitfield = 0x00000008;
#[doc = "`GL_TESS_CONTROL_SHADER_BIT_OES: GLbitfield = 0x00000008`"]
#[doc = "* **Group:** UseProgramStageMask"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_TESS_CONTROL_SHADER_BIT_OES: GLbitfield = 0x00000008;
#[doc = "`GL_TESS_CONTROL_SHADER_EXT: GLenum = 0x8E88`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_TESS_CONTROL_SHADER_EXT: GLenum = 0x8E88;
#[doc = "`GL_TESS_CONTROL_SHADER_OES: GLenum = 0x8E88`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_TESS_CONTROL_SHADER_OES: GLenum = 0x8E88;
#[doc = "`GL_TESS_EVALUATION_SHADER: GLenum = 0x8E87`"]
#[doc = "* **Groups:** PipelineParameterName, ShaderType"]
pub const GL_TESS_EVALUATION_SHADER: GLenum = 0x8E87;
#[doc = "`GL_TESS_EVALUATION_SHADER_BIT: GLbitfield = 0x00000010`"]
#[doc = "* **Group:** UseProgramStageMask"]
pub const GL_TESS_EVALUATION_SHADER_BIT: GLbitfield = 0x00000010;
#[doc = "`GL_TESS_EVALUATION_SHADER_BIT_EXT: GLbitfield = 0x00000010`"]
#[doc = "* **Group:** UseProgramStageMask"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_TESS_EVALUATION_SHADER_BIT_EXT: GLbitfield = 0x00000010;
#[doc = "`GL_TESS_EVALUATION_SHADER_BIT_OES: GLbitfield = 0x00000010`"]
#[doc = "* **Group:** UseProgramStageMask"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_TESS_EVALUATION_SHADER_BIT_OES: GLbitfield = 0x00000010;
#[doc = "`GL_TESS_EVALUATION_SHADER_EXT: GLenum = 0x8E87`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_TESS_EVALUATION_SHADER_EXT: GLenum = 0x8E87;
#[doc = "`GL_TESS_EVALUATION_SHADER_OES: GLenum = 0x8E87`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_TESS_EVALUATION_SHADER_OES: GLenum = 0x8E87;
#[doc = "`GL_TESS_GEN_MODE: GLenum = 0x8E76`"]
pub const GL_TESS_GEN_MODE: GLenum = 0x8E76;
#[doc = "`GL_TESS_GEN_MODE_EXT: GLenum = 0x8E76`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_TESS_GEN_MODE_EXT: GLenum = 0x8E76;
#[doc = "`GL_TESS_GEN_MODE_OES: GLenum = 0x8E76`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_TESS_GEN_MODE_OES: GLenum = 0x8E76;
#[doc = "`GL_TESS_GEN_POINT_MODE: GLenum = 0x8E79`"]
pub const GL_TESS_GEN_POINT_MODE: GLenum = 0x8E79;
#[doc = "`GL_TESS_GEN_POINT_MODE_EXT: GLenum = 0x8E79`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_TESS_GEN_POINT_MODE_EXT: GLenum = 0x8E79;
#[doc = "`GL_TESS_GEN_POINT_MODE_OES: GLenum = 0x8E79`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_TESS_GEN_POINT_MODE_OES: GLenum = 0x8E79;
#[doc = "`GL_TESS_GEN_SPACING: GLenum = 0x8E77`"]
pub const GL_TESS_GEN_SPACING: GLenum = 0x8E77;
#[doc = "`GL_TESS_GEN_SPACING_EXT: GLenum = 0x8E77`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_TESS_GEN_SPACING_EXT: GLenum = 0x8E77;
#[doc = "`GL_TESS_GEN_SPACING_OES: GLenum = 0x8E77`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_TESS_GEN_SPACING_OES: GLenum = 0x8E77;
#[doc = "`GL_TESS_GEN_VERTEX_ORDER: GLenum = 0x8E78`"]
pub const GL_TESS_GEN_VERTEX_ORDER: GLenum = 0x8E78;
#[doc = "`GL_TESS_GEN_VERTEX_ORDER_EXT: GLenum = 0x8E78`"]
#[cfg(any(feature = "GL_EXT_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_tessellation_shader"))))]
pub const GL_TESS_GEN_VERTEX_ORDER_EXT: GLenum = 0x8E78;
#[doc = "`GL_TESS_GEN_VERTEX_ORDER_OES: GLenum = 0x8E78`"]
#[cfg(any(feature = "GL_OES_tessellation_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_tessellation_shader"))))]
pub const GL_TESS_GEN_VERTEX_ORDER_OES: GLenum = 0x8E78;
#[doc = "`GL_TEXTURE: GLenum = 0x1702`"]
#[doc = "* **Groups:** ObjectIdentifier, MatrixMode"]
pub const GL_TEXTURE: GLenum = 0x1702;
#[doc = "`GL_TEXTURE0: GLenum = 0x84C0`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE0: GLenum = 0x84C0;
#[doc = "`GL_TEXTURE1: GLenum = 0x84C1`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE1: GLenum = 0x84C1;
#[doc = "`GL_TEXTURE10: GLenum = 0x84CA`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE10: GLenum = 0x84CA;
#[doc = "`GL_TEXTURE11: GLenum = 0x84CB`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE11: GLenum = 0x84CB;
#[doc = "`GL_TEXTURE12: GLenum = 0x84CC`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE12: GLenum = 0x84CC;
#[doc = "`GL_TEXTURE13: GLenum = 0x84CD`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE13: GLenum = 0x84CD;
#[doc = "`GL_TEXTURE14: GLenum = 0x84CE`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE14: GLenum = 0x84CE;
#[doc = "`GL_TEXTURE15: GLenum = 0x84CF`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE15: GLenum = 0x84CF;
#[doc = "`GL_TEXTURE16: GLenum = 0x84D0`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE16: GLenum = 0x84D0;
#[doc = "`GL_TEXTURE17: GLenum = 0x84D1`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE17: GLenum = 0x84D1;
#[doc = "`GL_TEXTURE18: GLenum = 0x84D2`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE18: GLenum = 0x84D2;
#[doc = "`GL_TEXTURE19: GLenum = 0x84D3`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE19: GLenum = 0x84D3;
#[doc = "`GL_TEXTURE2: GLenum = 0x84C2`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE2: GLenum = 0x84C2;
#[doc = "`GL_TEXTURE20: GLenum = 0x84D4`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE20: GLenum = 0x84D4;
#[doc = "`GL_TEXTURE21: GLenum = 0x84D5`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE21: GLenum = 0x84D5;
#[doc = "`GL_TEXTURE22: GLenum = 0x84D6`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE22: GLenum = 0x84D6;
#[doc = "`GL_TEXTURE23: GLenum = 0x84D7`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE23: GLenum = 0x84D7;
#[doc = "`GL_TEXTURE24: GLenum = 0x84D8`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE24: GLenum = 0x84D8;
#[doc = "`GL_TEXTURE25: GLenum = 0x84D9`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE25: GLenum = 0x84D9;
#[doc = "`GL_TEXTURE26: GLenum = 0x84DA`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE26: GLenum = 0x84DA;
#[doc = "`GL_TEXTURE27: GLenum = 0x84DB`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE27: GLenum = 0x84DB;
#[doc = "`GL_TEXTURE28: GLenum = 0x84DC`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE28: GLenum = 0x84DC;
#[doc = "`GL_TEXTURE29: GLenum = 0x84DD`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE29: GLenum = 0x84DD;
#[doc = "`GL_TEXTURE3: GLenum = 0x84C3`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE3: GLenum = 0x84C3;
#[doc = "`GL_TEXTURE30: GLenum = 0x84DE`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE30: GLenum = 0x84DE;
#[doc = "`GL_TEXTURE31: GLenum = 0x84DF`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE31: GLenum = 0x84DF;
#[doc = "`GL_TEXTURE4: GLenum = 0x84C4`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE4: GLenum = 0x84C4;
#[doc = "`GL_TEXTURE5: GLenum = 0x84C5`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE5: GLenum = 0x84C5;
#[doc = "`GL_TEXTURE6: GLenum = 0x84C6`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE6: GLenum = 0x84C6;
#[doc = "`GL_TEXTURE7: GLenum = 0x84C7`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE7: GLenum = 0x84C7;
#[doc = "`GL_TEXTURE8: GLenum = 0x84C8`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE8: GLenum = 0x84C8;
#[doc = "`GL_TEXTURE9: GLenum = 0x84C9`"]
#[doc = "* **Group:** TextureUnit"]
pub const GL_TEXTURE9: GLenum = 0x84C9;
#[doc = "`GL_TEXTURE_2D: GLenum = 0x0DE1`"]
#[doc = "* **Groups:** CopyImageSubDataTarget, EnableCap, GetPName, TextureTarget"]
pub const GL_TEXTURE_2D: GLenum = 0x0DE1;
#[doc = "`GL_TEXTURE_2D_ARRAY: GLenum = 0x8C1A`"]
#[doc = "* **Groups:** CopyImageSubDataTarget, TextureTarget"]
pub const GL_TEXTURE_2D_ARRAY: GLenum = 0x8C1A;
#[doc = "`GL_TEXTURE_2D_MULTISAMPLE: GLenum = 0x9100`"]
#[doc = "* **Groups:** CopyImageSubDataTarget, TextureTarget"]
pub const GL_TEXTURE_2D_MULTISAMPLE: GLenum = 0x9100;
#[doc = "`GL_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9102`"]
#[doc = "* **Groups:** CopyImageSubDataTarget, TextureTarget"]
pub const GL_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9102;
#[doc = "`GL_TEXTURE_2D_MULTISAMPLE_ARRAY_OES: GLenum = 0x9102`"]
#[cfg(any(feature = "GL_OES_texture_storage_multisample_2d_array"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_OES_texture_storage_multisample_2d_array")))
)]
pub const GL_TEXTURE_2D_MULTISAMPLE_ARRAY_OES: GLenum = 0x9102;
#[doc = "`GL_TEXTURE_3D: GLenum = 0x806F`"]
#[doc = "* **Groups:** CopyImageSubDataTarget, TextureTarget"]
pub const GL_TEXTURE_3D: GLenum = 0x806F;
#[doc = "`GL_TEXTURE_3D_OES: GLenum = 0x806F`"]
#[doc = "* **Group:** TextureTarget"]
#[cfg(any(feature = "GL_OES_texture_3D"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_3D"))))]
pub const GL_TEXTURE_3D_OES: GLenum = 0x806F;
#[doc = "`GL_TEXTURE_ALPHA_SIZE: GLenum = 0x805F`"]
#[doc = "* **Groups:** TextureParameterName, GetTextureParameter"]
pub const GL_TEXTURE_ALPHA_SIZE: GLenum = 0x805F;
#[doc = "`GL_TEXTURE_ALPHA_TYPE: GLenum = 0x8C13`"]
pub const GL_TEXTURE_ALPHA_TYPE: GLenum = 0x8C13;
#[doc = "`GL_TEXTURE_ASTC_DECODE_PRECISION_EXT: GLenum = 0x8F69`"]
#[cfg(any(feature = "GL_EXT_texture_compression_astc_decode_mode"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_texture_compression_astc_decode_mode")))
)]
pub const GL_TEXTURE_ASTC_DECODE_PRECISION_EXT: GLenum = 0x8F69;
#[doc = "`GL_TEXTURE_BASE_LEVEL: GLenum = 0x813C`"]
#[doc = "* **Group:** TextureParameterName"]
pub const GL_TEXTURE_BASE_LEVEL: GLenum = 0x813C;
#[doc = "`GL_TEXTURE_BINDING_2D: GLenum = 0x8069`"]
#[doc = "* **Group:** GetPName"]
pub const GL_TEXTURE_BINDING_2D: GLenum = 0x8069;
#[doc = "`GL_TEXTURE_BINDING_2D_ARRAY: GLenum = 0x8C1D`"]
#[doc = "* **Group:** GetPName"]
pub const GL_TEXTURE_BINDING_2D_ARRAY: GLenum = 0x8C1D;
#[doc = "`GL_TEXTURE_BINDING_2D_MULTISAMPLE: GLenum = 0x9104`"]
#[doc = "* **Group:** GetPName"]
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE: GLenum = 0x9104;
#[doc = "`GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: GLenum = 0x9105`"]
#[doc = "* **Group:** GetPName"]
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: GLenum = 0x9105;
#[doc = "`GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY_OES: GLenum = 0x9105`"]
#[cfg(any(feature = "GL_OES_texture_storage_multisample_2d_array"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_OES_texture_storage_multisample_2d_array")))
)]
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY_OES: GLenum = 0x9105;
#[doc = "`GL_TEXTURE_BINDING_3D: GLenum = 0x806A`"]
#[doc = "* **Group:** GetPName"]
pub const GL_TEXTURE_BINDING_3D: GLenum = 0x806A;
#[doc = "`GL_TEXTURE_BINDING_3D_OES: GLenum = 0x806A`"]
#[cfg(any(feature = "GL_OES_texture_3D"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_3D"))))]
pub const GL_TEXTURE_BINDING_3D_OES: GLenum = 0x806A;
#[doc = "`GL_TEXTURE_BINDING_BUFFER: GLenum = 0x8C2C`"]
#[doc = "* **Group:** GetPName"]
pub const GL_TEXTURE_BINDING_BUFFER: GLenum = 0x8C2C;
#[doc = "`GL_TEXTURE_BINDING_BUFFER_EXT: GLenum = 0x8C2C`"]
#[cfg(any(feature = "GL_EXT_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_buffer"))))]
pub const GL_TEXTURE_BINDING_BUFFER_EXT: GLenum = 0x8C2C;
#[doc = "`GL_TEXTURE_BINDING_BUFFER_OES: GLenum = 0x8C2C`"]
#[cfg(any(feature = "GL_OES_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_buffer"))))]
pub const GL_TEXTURE_BINDING_BUFFER_OES: GLenum = 0x8C2C;
#[doc = "`GL_TEXTURE_BINDING_CUBE_MAP: GLenum = 0x8514`"]
#[doc = "* **Group:** GetPName"]
pub const GL_TEXTURE_BINDING_CUBE_MAP: GLenum = 0x8514;
#[doc = "`GL_TEXTURE_BINDING_CUBE_MAP_ARRAY: GLenum = 0x900A`"]
pub const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY: GLenum = 0x900A;
#[doc = "`GL_TEXTURE_BINDING_CUBE_MAP_ARRAY_EXT: GLenum = 0x900A`"]
#[cfg(any(feature = "GL_EXT_texture_cube_map_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_cube_map_array"))))]
pub const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY_EXT: GLenum = 0x900A;
#[doc = "`GL_TEXTURE_BINDING_CUBE_MAP_ARRAY_OES: GLenum = 0x900A`"]
#[cfg(any(feature = "GL_OES_texture_cube_map_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_cube_map_array"))))]
pub const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY_OES: GLenum = 0x900A;
#[doc = "`GL_TEXTURE_BINDING_EXTERNAL_OES: GLenum = 0x8D67`"]
#[cfg(any(feature = "GL_EXT_YUV_target", feature = "GL_OES_EGL_image_external"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_YUV_target", feature = "GL_OES_EGL_image_external")))
)]
pub const GL_TEXTURE_BINDING_EXTERNAL_OES: GLenum = 0x8D67;
#[doc = "`GL_TEXTURE_BLUE_SIZE: GLenum = 0x805E`"]
#[doc = "* **Groups:** TextureParameterName, GetTextureParameter"]
pub const GL_TEXTURE_BLUE_SIZE: GLenum = 0x805E;
#[doc = "`GL_TEXTURE_BLUE_TYPE: GLenum = 0x8C12`"]
pub const GL_TEXTURE_BLUE_TYPE: GLenum = 0x8C12;
#[doc = "`GL_TEXTURE_BORDER_COLOR: GLenum = 0x1004`"]
#[doc = "* **Groups:** SamplerParameterF, GetTextureParameter, TextureParameterName"]
pub const GL_TEXTURE_BORDER_COLOR: GLenum = 0x1004;
#[doc = "`GL_TEXTURE_BORDER_COLOR_EXT: GLenum = 0x1004`"]
#[cfg(any(feature = "GL_EXT_texture_border_clamp"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_border_clamp"))))]
pub const GL_TEXTURE_BORDER_COLOR_EXT: GLenum = 0x1004;
#[doc = "`GL_TEXTURE_BORDER_COLOR_NV: GLenum = 0x1004`"]
#[doc = "* **Groups:** TextureParameterName, GetTextureParameter"]
#[cfg(any(feature = "GL_NV_texture_border_clamp"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_texture_border_clamp"))))]
pub const GL_TEXTURE_BORDER_COLOR_NV: GLenum = 0x1004;
#[doc = "`GL_TEXTURE_BORDER_COLOR_OES: GLenum = 0x1004`"]
#[cfg(any(feature = "GL_OES_texture_border_clamp"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_border_clamp"))))]
pub const GL_TEXTURE_BORDER_COLOR_OES: GLenum = 0x1004;
#[doc = "`GL_TEXTURE_BUFFER: GLenum = 0x8C2A`"]
#[doc = "* **Groups:** TextureTarget, CopyBufferSubDataTarget, BufferTargetARB, BufferStorageTarget"]
pub const GL_TEXTURE_BUFFER: GLenum = 0x8C2A;
#[doc = "`GL_TEXTURE_BUFFER_BINDING: GLenum = 0x8C2A`"]
pub const GL_TEXTURE_BUFFER_BINDING: GLenum = 0x8C2A;
#[doc = "`GL_TEXTURE_BUFFER_BINDING_EXT: GLenum = 0x8C2A`"]
#[cfg(any(feature = "GL_EXT_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_buffer"))))]
pub const GL_TEXTURE_BUFFER_BINDING_EXT: GLenum = 0x8C2A;
#[doc = "`GL_TEXTURE_BUFFER_BINDING_OES: GLenum = 0x8C2A`"]
#[cfg(any(feature = "GL_OES_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_buffer"))))]
pub const GL_TEXTURE_BUFFER_BINDING_OES: GLenum = 0x8C2A;
#[doc = "`GL_TEXTURE_BUFFER_DATA_STORE_BINDING: GLenum = 0x8C2D`"]
pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING: GLenum = 0x8C2D;
#[doc = "`GL_TEXTURE_BUFFER_DATA_STORE_BINDING_EXT: GLenum = 0x8C2D`"]
#[cfg(any(feature = "GL_EXT_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_buffer"))))]
pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING_EXT: GLenum = 0x8C2D;
#[doc = "`GL_TEXTURE_BUFFER_DATA_STORE_BINDING_OES: GLenum = 0x8C2D`"]
#[cfg(any(feature = "GL_OES_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_buffer"))))]
pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING_OES: GLenum = 0x8C2D;
#[doc = "`GL_TEXTURE_BUFFER_EXT: GLenum = 0x8C2A`"]
#[cfg(any(feature = "GL_EXT_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_buffer"))))]
pub const GL_TEXTURE_BUFFER_EXT: GLenum = 0x8C2A;
#[doc = "`GL_TEXTURE_BUFFER_OES: GLenum = 0x8C2A`"]
#[cfg(any(feature = "GL_OES_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_buffer"))))]
pub const GL_TEXTURE_BUFFER_OES: GLenum = 0x8C2A;
#[doc = "`GL_TEXTURE_BUFFER_OFFSET: GLenum = 0x919D`"]
pub const GL_TEXTURE_BUFFER_OFFSET: GLenum = 0x919D;
#[doc = "`GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x919F`"]
#[doc = "* **Group:** GetPName"]
pub const GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x919F;
#[doc = "`GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT_EXT: GLenum = 0x919F`"]
#[cfg(any(feature = "GL_EXT_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_buffer"))))]
pub const GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT_EXT: GLenum = 0x919F;
#[doc = "`GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT_OES: GLenum = 0x919F`"]
#[cfg(any(feature = "GL_OES_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_buffer"))))]
pub const GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT_OES: GLenum = 0x919F;
#[doc = "`GL_TEXTURE_BUFFER_OFFSET_EXT: GLenum = 0x919D`"]
#[cfg(any(feature = "GL_EXT_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_buffer"))))]
pub const GL_TEXTURE_BUFFER_OFFSET_EXT: GLenum = 0x919D;
#[doc = "`GL_TEXTURE_BUFFER_OFFSET_OES: GLenum = 0x919D`"]
#[cfg(any(feature = "GL_OES_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_buffer"))))]
pub const GL_TEXTURE_BUFFER_OFFSET_OES: GLenum = 0x919D;
#[doc = "`GL_TEXTURE_BUFFER_SIZE: GLenum = 0x919E`"]
pub const GL_TEXTURE_BUFFER_SIZE: GLenum = 0x919E;
#[doc = "`GL_TEXTURE_BUFFER_SIZE_EXT: GLenum = 0x919E`"]
#[cfg(any(feature = "GL_EXT_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_buffer"))))]
pub const GL_TEXTURE_BUFFER_SIZE_EXT: GLenum = 0x919E;
#[doc = "`GL_TEXTURE_BUFFER_SIZE_OES: GLenum = 0x919E`"]
#[cfg(any(feature = "GL_OES_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_buffer"))))]
pub const GL_TEXTURE_BUFFER_SIZE_OES: GLenum = 0x919E;
#[doc = "`GL_TEXTURE_COMPARE_FUNC: GLenum = 0x884D`"]
#[doc = "* **Groups:** SamplerParameterI, TextureParameterName"]
pub const GL_TEXTURE_COMPARE_FUNC: GLenum = 0x884D;
#[doc = "`GL_TEXTURE_COMPARE_FUNC_EXT: GLenum = 0x884D`"]
#[cfg(any(feature = "GL_EXT_shadow_samplers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_shadow_samplers"))))]
pub const GL_TEXTURE_COMPARE_FUNC_EXT: GLenum = 0x884D;
#[doc = "`GL_TEXTURE_COMPARE_MODE: GLenum = 0x884C`"]
#[doc = "* **Groups:** SamplerParameterI, TextureParameterName"]
pub const GL_TEXTURE_COMPARE_MODE: GLenum = 0x884C;
#[doc = "`GL_TEXTURE_COMPARE_MODE_EXT: GLenum = 0x884C`"]
#[cfg(any(feature = "GL_EXT_shadow_samplers"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_shadow_samplers"))))]
pub const GL_TEXTURE_COMPARE_MODE_EXT: GLenum = 0x884C;
#[doc = "`GL_TEXTURE_COMPRESSED: GLenum = 0x86A1`"]
#[doc = "* **Group:** InternalFormatPName"]
pub const GL_TEXTURE_COMPRESSED: GLenum = 0x86A1;
#[doc = "`GL_TEXTURE_CUBE_MAP: GLenum = 0x8513`"]
#[doc = "* **Groups:** CopyImageSubDataTarget, TextureTarget"]
pub const GL_TEXTURE_CUBE_MAP: GLenum = 0x8513;
#[doc = "`GL_TEXTURE_CUBE_MAP_ARRAY: GLenum = 0x9009`"]
#[doc = "* **Groups:** CopyImageSubDataTarget, TextureTarget"]
pub const GL_TEXTURE_CUBE_MAP_ARRAY: GLenum = 0x9009;
#[doc = "`GL_TEXTURE_CUBE_MAP_ARRAY_EXT: GLenum = 0x9009`"]
#[doc = "* **Group:** TextureTarget"]
#[cfg(any(feature = "GL_EXT_texture_cube_map_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_cube_map_array"))))]
pub const GL_TEXTURE_CUBE_MAP_ARRAY_EXT: GLenum = 0x9009;
#[doc = "`GL_TEXTURE_CUBE_MAP_ARRAY_OES: GLenum = 0x9009`"]
#[doc = "* **Group:** TextureTarget"]
#[cfg(any(
    feature = "GL_EXT_sparse_texture",
    feature = "GL_OES_texture_cube_map_array"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_EXT_sparse_texture",
        feature = "GL_OES_texture_cube_map_array"
    )))
)]
pub const GL_TEXTURE_CUBE_MAP_ARRAY_OES: GLenum = 0x9009;
#[doc = "`GL_TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 0x8516`"]
#[doc = "* **Group:** TextureTarget"]
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 0x8516;
#[doc = "`GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 0x8518`"]
#[doc = "* **Group:** TextureTarget"]
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 0x8518;
#[doc = "`GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 0x851A`"]
#[doc = "* **Group:** TextureTarget"]
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 0x851A;
#[doc = "`GL_TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 0x8515`"]
#[doc = "* **Group:** TextureTarget"]
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 0x8515;
#[doc = "`GL_TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 0x8517`"]
#[doc = "* **Group:** TextureTarget"]
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 0x8517;
#[doc = "`GL_TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 0x8519`"]
#[doc = "* **Group:** TextureTarget"]
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 0x8519;
#[doc = "`GL_TEXTURE_DEPTH: GLenum = 0x8071`"]
pub const GL_TEXTURE_DEPTH: GLenum = 0x8071;
#[doc = "`GL_TEXTURE_DEPTH_QCOM: GLenum = 0x8BD4`"]
#[cfg(any(feature = "GL_QCOM_extended_get"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_extended_get"))))]
pub const GL_TEXTURE_DEPTH_QCOM: GLenum = 0x8BD4;
#[doc = "`GL_TEXTURE_DEPTH_SIZE: GLenum = 0x884A`"]
pub const GL_TEXTURE_DEPTH_SIZE: GLenum = 0x884A;
#[doc = "`GL_TEXTURE_DEPTH_TYPE: GLenum = 0x8C16`"]
pub const GL_TEXTURE_DEPTH_TYPE: GLenum = 0x8C16;
#[doc = "`GL_TEXTURE_EXTERNAL_OES: GLenum = 0x8D65`"]
#[cfg(any(feature = "GL_EXT_YUV_target", feature = "GL_OES_EGL_image_external"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_YUV_target", feature = "GL_OES_EGL_image_external")))
)]
pub const GL_TEXTURE_EXTERNAL_OES: GLenum = 0x8D65;
#[doc = "`GL_TEXTURE_FETCH_BARRIER_BIT: GLbitfield = 0x00000008`"]
#[doc = "* **Group:** MemoryBarrierMask"]
pub const GL_TEXTURE_FETCH_BARRIER_BIT: GLbitfield = 0x00000008;
#[doc = "`GL_TEXTURE_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9107`"]
pub const GL_TEXTURE_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9107;
#[doc = "`GL_TEXTURE_FORMAT_QCOM: GLenum = 0x8BD6`"]
#[cfg(any(feature = "GL_QCOM_extended_get"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_extended_get"))))]
pub const GL_TEXTURE_FORMAT_QCOM: GLenum = 0x8BD6;
#[doc = "`GL_TEXTURE_FORMAT_SRGB_OVERRIDE_EXT: GLenum = 0x8FBF`"]
#[cfg(any(feature = "GL_EXT_texture_format_sRGB_override"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_texture_format_sRGB_override")))
)]
pub const GL_TEXTURE_FORMAT_SRGB_OVERRIDE_EXT: GLenum = 0x8FBF;
#[doc = "`GL_TEXTURE_FOVEATED_CUTOFF_DENSITY_QCOM: GLenum = 0x96A0`"]
#[doc = "* **Group:** TextureParameterName"]
#[cfg(any(feature = "GL_QCOM_texture_foveated2"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_texture_foveated2"))))]
pub const GL_TEXTURE_FOVEATED_CUTOFF_DENSITY_QCOM: GLenum = 0x96A0;
#[doc = "`GL_TEXTURE_FOVEATED_FEATURE_BITS_QCOM: GLenum = 0x8BFB`"]
#[cfg(any(feature = "GL_QCOM_texture_foveated"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_texture_foveated"))))]
pub const GL_TEXTURE_FOVEATED_FEATURE_BITS_QCOM: GLenum = 0x8BFB;
#[doc = "`GL_TEXTURE_FOVEATED_FEATURE_QUERY_QCOM: GLenum = 0x8BFD`"]
#[cfg(any(feature = "GL_QCOM_texture_foveated"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_texture_foveated"))))]
pub const GL_TEXTURE_FOVEATED_FEATURE_QUERY_QCOM: GLenum = 0x8BFD;
#[doc = "`GL_TEXTURE_FOVEATED_MIN_PIXEL_DENSITY_QCOM: GLenum = 0x8BFC`"]
#[cfg(any(feature = "GL_QCOM_texture_foveated"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_texture_foveated"))))]
pub const GL_TEXTURE_FOVEATED_MIN_PIXEL_DENSITY_QCOM: GLenum = 0x8BFC;
#[doc = "`GL_TEXTURE_FOVEATED_NUM_FOCAL_POINTS_QUERY_QCOM: GLenum = 0x8BFE`"]
#[cfg(any(feature = "GL_QCOM_texture_foveated"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_texture_foveated"))))]
pub const GL_TEXTURE_FOVEATED_NUM_FOCAL_POINTS_QUERY_QCOM: GLenum = 0x8BFE;
#[doc = "`GL_TEXTURE_GREEN_SIZE: GLenum = 0x805D`"]
#[doc = "* **Groups:** TextureParameterName, GetTextureParameter"]
pub const GL_TEXTURE_GREEN_SIZE: GLenum = 0x805D;
#[doc = "`GL_TEXTURE_GREEN_TYPE: GLenum = 0x8C11`"]
pub const GL_TEXTURE_GREEN_TYPE: GLenum = 0x8C11;
#[doc = "`GL_TEXTURE_HEIGHT: GLenum = 0x1001`"]
#[doc = "* **Groups:** TextureParameterName, GetTextureParameter"]
pub const GL_TEXTURE_HEIGHT: GLenum = 0x1001;
#[doc = "`GL_TEXTURE_HEIGHT_QCOM: GLenum = 0x8BD3`"]
#[cfg(any(feature = "GL_QCOM_extended_get"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_extended_get"))))]
pub const GL_TEXTURE_HEIGHT_QCOM: GLenum = 0x8BD3;
#[doc = "`GL_TEXTURE_IMAGE_VALID_QCOM: GLenum = 0x8BD8`"]
#[cfg(any(feature = "GL_QCOM_extended_get"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_extended_get"))))]
pub const GL_TEXTURE_IMAGE_VALID_QCOM: GLenum = 0x8BD8;
#[doc = "`GL_TEXTURE_IMMUTABLE_FORMAT: GLenum = 0x912F`"]
pub const GL_TEXTURE_IMMUTABLE_FORMAT: GLenum = 0x912F;
#[doc = "`GL_TEXTURE_IMMUTABLE_FORMAT_EXT: GLenum = 0x912F`"]
#[cfg(any(feature = "GL_EXT_texture_storage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_storage"))))]
pub const GL_TEXTURE_IMMUTABLE_FORMAT_EXT: GLenum = 0x912F;
#[doc = "`GL_TEXTURE_IMMUTABLE_LEVELS: GLenum = 0x82DF`"]
pub const GL_TEXTURE_IMMUTABLE_LEVELS: GLenum = 0x82DF;
#[doc = "`GL_TEXTURE_INTERNAL_FORMAT: GLenum = 0x1003`"]
#[doc = "* **Groups:** TextureParameterName, GetTextureParameter"]
pub const GL_TEXTURE_INTERNAL_FORMAT: GLenum = 0x1003;
#[doc = "`GL_TEXTURE_INTERNAL_FORMAT_QCOM: GLenum = 0x8BD5`"]
#[cfg(any(feature = "GL_QCOM_extended_get"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_extended_get"))))]
pub const GL_TEXTURE_INTERNAL_FORMAT_QCOM: GLenum = 0x8BD5;
#[doc = "`GL_TEXTURE_MAG_FILTER: GLenum = 0x2800`"]
#[doc = "* **Groups:** SamplerParameterI, GetTextureParameter, TextureParameterName"]
pub const GL_TEXTURE_MAG_FILTER: GLenum = 0x2800;
#[doc = "`GL_TEXTURE_MAX_ANISOTROPY_EXT: GLenum = 0x84FE`"]
#[doc = "* **Alias Of:** `GL_TEXTURE_MAX_ANISOTROPY`"]
#[cfg(any(feature = "GL_EXT_texture_filter_anisotropic"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_filter_anisotropic"))))]
pub const GL_TEXTURE_MAX_ANISOTROPY_EXT: GLenum = 0x84FE;
#[doc = "`GL_TEXTURE_MAX_LEVEL: GLenum = 0x813D`"]
#[doc = "* **Group:** TextureParameterName"]
pub const GL_TEXTURE_MAX_LEVEL: GLenum = 0x813D;
#[doc = "`GL_TEXTURE_MAX_LEVEL_APPLE: GLenum = 0x813D`"]
#[cfg(any(feature = "GL_APPLE_texture_max_level"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_texture_max_level"))))]
pub const GL_TEXTURE_MAX_LEVEL_APPLE: GLenum = 0x813D;
#[doc = "`GL_TEXTURE_MAX_LOD: GLenum = 0x813B`"]
#[doc = "* **Groups:** SamplerParameterF, TextureParameterName"]
pub const GL_TEXTURE_MAX_LOD: GLenum = 0x813B;
#[doc = "`GL_TEXTURE_MIN_FILTER: GLenum = 0x2801`"]
#[doc = "* **Groups:** SamplerParameterI, GetTextureParameter, TextureParameterName"]
pub const GL_TEXTURE_MIN_FILTER: GLenum = 0x2801;
#[doc = "`GL_TEXTURE_MIN_LOD: GLenum = 0x813A`"]
#[doc = "* **Groups:** SamplerParameterF, TextureParameterName"]
pub const GL_TEXTURE_MIN_LOD: GLenum = 0x813A;
#[doc = "`GL_TEXTURE_NUM_LEVELS_QCOM: GLenum = 0x8BD9`"]
#[cfg(any(feature = "GL_QCOM_extended_get"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_extended_get"))))]
pub const GL_TEXTURE_NUM_LEVELS_QCOM: GLenum = 0x8BD9;
#[doc = "`GL_TEXTURE_OBJECT_VALID_QCOM: GLenum = 0x8BDB`"]
#[cfg(any(feature = "GL_QCOM_extended_get"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_extended_get"))))]
pub const GL_TEXTURE_OBJECT_VALID_QCOM: GLenum = 0x8BDB;
#[doc = "`GL_TEXTURE_PROTECTED_EXT: GLenum = 0x8BFA`"]
#[cfg(any(feature = "GL_EXT_protected_textures"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_protected_textures"))))]
pub const GL_TEXTURE_PROTECTED_EXT: GLenum = 0x8BFA;
#[doc = "`GL_TEXTURE_REDUCTION_MODE_EXT: GLenum = 0x9366`"]
#[doc = "* **Alias Of:** `GL_TEXTURE_REDUCTION_MODE_ARB`"]
#[cfg(any(feature = "GL_EXT_texture_filter_minmax"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_filter_minmax"))))]
pub const GL_TEXTURE_REDUCTION_MODE_EXT: GLenum = 0x9366;
#[doc = "`GL_TEXTURE_RED_SIZE: GLenum = 0x805C`"]
#[doc = "* **Groups:** TextureParameterName, GetTextureParameter"]
pub const GL_TEXTURE_RED_SIZE: GLenum = 0x805C;
#[doc = "`GL_TEXTURE_RED_TYPE: GLenum = 0x8C10`"]
pub const GL_TEXTURE_RED_TYPE: GLenum = 0x8C10;
#[doc = "`GL_TEXTURE_SAMPLES: GLenum = 0x9106`"]
pub const GL_TEXTURE_SAMPLES: GLenum = 0x9106;
#[doc = "`GL_TEXTURE_SAMPLES_IMG: GLenum = 0x9136`"]
#[cfg(any(feature = "GL_IMG_multisampled_render_to_texture"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_IMG_multisampled_render_to_texture")))
)]
pub const GL_TEXTURE_SAMPLES_IMG: GLenum = 0x9136;
#[doc = "`GL_TEXTURE_SHARED_SIZE: GLenum = 0x8C3F`"]
pub const GL_TEXTURE_SHARED_SIZE: GLenum = 0x8C3F;
#[doc = "`GL_TEXTURE_SPARSE_EXT: GLenum = 0x91A6`"]
#[cfg(any(feature = "GL_EXT_sparse_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_sparse_texture"))))]
pub const GL_TEXTURE_SPARSE_EXT: GLenum = 0x91A6;
#[doc = "`GL_TEXTURE_SRGB_DECODE_EXT: GLenum = 0x8A48`"]
#[cfg(any(feature = "GL_EXT_texture_sRGB_decode"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_sRGB_decode"))))]
pub const GL_TEXTURE_SRGB_DECODE_EXT: GLenum = 0x8A48;
#[doc = "`GL_TEXTURE_STENCIL_SIZE: GLenum = 0x88F1`"]
pub const GL_TEXTURE_STENCIL_SIZE: GLenum = 0x88F1;
#[doc = "`GL_TEXTURE_SWIZZLE_A: GLenum = 0x8E45`"]
#[doc = "* **Group:** TextureParameterName"]
pub const GL_TEXTURE_SWIZZLE_A: GLenum = 0x8E45;
#[doc = "`GL_TEXTURE_SWIZZLE_B: GLenum = 0x8E44`"]
#[doc = "* **Group:** TextureParameterName"]
pub const GL_TEXTURE_SWIZZLE_B: GLenum = 0x8E44;
#[doc = "`GL_TEXTURE_SWIZZLE_G: GLenum = 0x8E43`"]
#[doc = "* **Group:** TextureParameterName"]
pub const GL_TEXTURE_SWIZZLE_G: GLenum = 0x8E43;
#[doc = "`GL_TEXTURE_SWIZZLE_R: GLenum = 0x8E42`"]
#[doc = "* **Group:** TextureParameterName"]
pub const GL_TEXTURE_SWIZZLE_R: GLenum = 0x8E42;
#[doc = "`GL_TEXTURE_TARGET_QCOM: GLenum = 0x8BDA`"]
#[cfg(any(feature = "GL_QCOM_extended_get"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_extended_get"))))]
pub const GL_TEXTURE_TARGET_QCOM: GLenum = 0x8BDA;
#[doc = "`GL_TEXTURE_TILING_EXT: GLenum = 0x9580`"]
#[doc = "* **Group:** TextureParameterName"]
#[cfg(any(feature = "GL_EXT_memory_object"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_memory_object"))))]
pub const GL_TEXTURE_TILING_EXT: GLenum = 0x9580;
#[doc = "`GL_TEXTURE_TYPE_QCOM: GLenum = 0x8BD7`"]
#[cfg(any(feature = "GL_QCOM_extended_get"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_extended_get"))))]
pub const GL_TEXTURE_TYPE_QCOM: GLenum = 0x8BD7;
#[doc = "`GL_TEXTURE_UNNORMALIZED_COORDINATES_ARM: GLenum = 0x8F6A`"]
#[doc = "* **Groups:** SamplerParameterF, SamplerParameterI, GetTextureParameter, TextureParameterName"]
#[cfg(any(feature = "GL_ARM_texture_unnormalized_coordinates"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_ARM_texture_unnormalized_coordinates")))
)]
pub const GL_TEXTURE_UNNORMALIZED_COORDINATES_ARM: GLenum = 0x8F6A;
#[doc = "`GL_TEXTURE_UPDATE_BARRIER_BIT: GLbitfield = 0x00000100`"]
#[doc = "* **Group:** MemoryBarrierMask"]
pub const GL_TEXTURE_UPDATE_BARRIER_BIT: GLbitfield = 0x00000100;
#[doc = "`GL_TEXTURE_USAGE_ANGLE: GLenum = 0x93A2`"]
#[cfg(any(feature = "GL_ANGLE_texture_usage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_ANGLE_texture_usage"))))]
pub const GL_TEXTURE_USAGE_ANGLE: GLenum = 0x93A2;
#[doc = "`GL_TEXTURE_VIEW_MIN_LAYER_EXT: GLenum = 0x82DD`"]
#[cfg(any(feature = "GL_EXT_texture_view"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_view"))))]
pub const GL_TEXTURE_VIEW_MIN_LAYER_EXT: GLenum = 0x82DD;
#[doc = "`GL_TEXTURE_VIEW_MIN_LAYER_OES: GLenum = 0x82DD`"]
#[cfg(any(feature = "GL_OES_texture_view"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_view"))))]
pub const GL_TEXTURE_VIEW_MIN_LAYER_OES: GLenum = 0x82DD;
#[doc = "`GL_TEXTURE_VIEW_MIN_LEVEL_EXT: GLenum = 0x82DB`"]
#[cfg(any(feature = "GL_EXT_texture_view"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_view"))))]
pub const GL_TEXTURE_VIEW_MIN_LEVEL_EXT: GLenum = 0x82DB;
#[doc = "`GL_TEXTURE_VIEW_MIN_LEVEL_OES: GLenum = 0x82DB`"]
#[cfg(any(feature = "GL_OES_texture_view"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_view"))))]
pub const GL_TEXTURE_VIEW_MIN_LEVEL_OES: GLenum = 0x82DB;
#[doc = "`GL_TEXTURE_VIEW_NUM_LAYERS_EXT: GLenum = 0x82DE`"]
#[cfg(any(feature = "GL_EXT_texture_view"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_view"))))]
pub const GL_TEXTURE_VIEW_NUM_LAYERS_EXT: GLenum = 0x82DE;
#[doc = "`GL_TEXTURE_VIEW_NUM_LAYERS_OES: GLenum = 0x82DE`"]
#[cfg(any(feature = "GL_OES_texture_view"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_view"))))]
pub const GL_TEXTURE_VIEW_NUM_LAYERS_OES: GLenum = 0x82DE;
#[doc = "`GL_TEXTURE_VIEW_NUM_LEVELS_EXT: GLenum = 0x82DC`"]
#[cfg(any(feature = "GL_EXT_texture_view"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_view"))))]
pub const GL_TEXTURE_VIEW_NUM_LEVELS_EXT: GLenum = 0x82DC;
#[doc = "`GL_TEXTURE_VIEW_NUM_LEVELS_OES: GLenum = 0x82DC`"]
#[cfg(any(feature = "GL_OES_texture_view"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_view"))))]
pub const GL_TEXTURE_VIEW_NUM_LEVELS_OES: GLenum = 0x82DC;
#[doc = "`GL_TEXTURE_WIDTH: GLenum = 0x1000`"]
#[doc = "* **Groups:** TextureParameterName, GetTextureParameter"]
pub const GL_TEXTURE_WIDTH: GLenum = 0x1000;
#[doc = "`GL_TEXTURE_WIDTH_QCOM: GLenum = 0x8BD2`"]
#[cfg(any(feature = "GL_QCOM_extended_get"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_extended_get"))))]
pub const GL_TEXTURE_WIDTH_QCOM: GLenum = 0x8BD2;
#[doc = "`GL_TEXTURE_WRAP_R: GLenum = 0x8072`"]
#[doc = "* **Groups:** SamplerParameterI, TextureParameterName"]
pub const GL_TEXTURE_WRAP_R: GLenum = 0x8072;
#[doc = "`GL_TEXTURE_WRAP_R_OES: GLenum = 0x8072`"]
#[doc = "* **Group:** TextureParameterName"]
#[cfg(any(feature = "GL_OES_texture_3D"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_3D"))))]
pub const GL_TEXTURE_WRAP_R_OES: GLenum = 0x8072;
#[doc = "`GL_TEXTURE_WRAP_S: GLenum = 0x2802`"]
#[doc = "* **Groups:** SamplerParameterI, GetTextureParameter, TextureParameterName"]
pub const GL_TEXTURE_WRAP_S: GLenum = 0x2802;
#[doc = "`GL_TEXTURE_WRAP_T: GLenum = 0x2803`"]
#[doc = "* **Groups:** SamplerParameterI, GetTextureParameter, TextureParameterName"]
pub const GL_TEXTURE_WRAP_T: GLenum = 0x2803;
#[doc = "`GL_TILING_TYPES_EXT: GLenum = 0x9583`"]
#[cfg(any(feature = "GL_EXT_memory_object"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_memory_object"))))]
pub const GL_TILING_TYPES_EXT: GLenum = 0x9583;
#[doc = "`GL_TIMELINE_SEMAPHORE_VALUE_NV: GLenum = 0x9595`"]
#[doc = "* **Group:** SemaphoreParameterName"]
#[cfg(any(feature = "GL_NV_timeline_semaphore"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_timeline_semaphore"))))]
pub const GL_TIMELINE_SEMAPHORE_VALUE_NV: GLenum = 0x9595;
#[doc = "`GL_TIMEOUT_EXPIRED: GLenum = 0x911B`"]
#[doc = "* **Group:** SyncStatus"]
pub const GL_TIMEOUT_EXPIRED: GLenum = 0x911B;
#[doc = "`GL_TIMEOUT_EXPIRED_APPLE: GLenum = 0x911B`"]
#[cfg(any(feature = "GL_APPLE_sync"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_sync"))))]
pub const GL_TIMEOUT_EXPIRED_APPLE: GLenum = 0x911B;
#[doc = "`GL_TIMEOUT_IGNORED: u64 = 0xFFFFFFFFFFFFFFFF`"]
pub const GL_TIMEOUT_IGNORED: u64 = 0xFFFFFFFFFFFFFFFF;
#[doc = "`GL_TIMEOUT_IGNORED_APPLE: u64 = 0xFFFFFFFFFFFFFFFF`"]
#[cfg(any(feature = "GL_APPLE_sync"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_sync"))))]
pub const GL_TIMEOUT_IGNORED_APPLE: u64 = 0xFFFFFFFFFFFFFFFF;
#[doc = "`GL_TIMESTAMP_EXT: GLenum = 0x8E28`"]
#[cfg(any(feature = "GL_EXT_disjoint_timer_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_disjoint_timer_query"))))]
pub const GL_TIMESTAMP_EXT: GLenum = 0x8E28;
#[doc = "`GL_TIME_ELAPSED_EXT: GLenum = 0x88BF`"]
#[cfg(any(feature = "GL_EXT_disjoint_timer_query"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_disjoint_timer_query"))))]
pub const GL_TIME_ELAPSED_EXT: GLenum = 0x88BF;
#[doc = "`GL_TOP_LEVEL_ARRAY_SIZE: GLenum = 0x930C`"]
#[doc = "* **Group:** ProgramResourceProperty"]
pub const GL_TOP_LEVEL_ARRAY_SIZE: GLenum = 0x930C;
#[doc = "`GL_TOP_LEVEL_ARRAY_STRIDE: GLenum = 0x930D`"]
#[doc = "* **Group:** ProgramResourceProperty"]
pub const GL_TOP_LEVEL_ARRAY_STRIDE: GLenum = 0x930D;
#[doc = "`GL_TRANSFORM_FEEDBACK: GLenum = 0x8E22`"]
#[doc = "* **Groups:** ObjectIdentifier, BindTransformFeedbackTarget"]
pub const GL_TRANSFORM_FEEDBACK: GLenum = 0x8E22;
#[doc = "`GL_TRANSFORM_FEEDBACK_ACTIVE: GLenum = 0x8E24`"]
#[doc = "* **Group:** TransformFeedbackPName"]
#[doc = "* **Alias Of:** `GL_TRANSFORM_FEEDBACK_BUFFER_ACTIVE`"]
pub const GL_TRANSFORM_FEEDBACK_ACTIVE: GLenum = 0x8E24;
#[doc = "`GL_TRANSFORM_FEEDBACK_BARRIER_BIT: GLbitfield = 0x00000800`"]
#[doc = "* **Group:** MemoryBarrierMask"]
pub const GL_TRANSFORM_FEEDBACK_BARRIER_BIT: GLbitfield = 0x00000800;
#[doc = "`GL_TRANSFORM_FEEDBACK_BINDING: GLenum = 0x8E25`"]
pub const GL_TRANSFORM_FEEDBACK_BINDING: GLenum = 0x8E25;
#[doc = "`GL_TRANSFORM_FEEDBACK_BUFFER: GLenum = 0x8C8E`"]
#[doc = "* **Groups:** ProgramInterface, BufferTargetARB, BufferStorageTarget, CopyBufferSubDataTarget"]
pub const GL_TRANSFORM_FEEDBACK_BUFFER: GLenum = 0x8C8E;
#[doc = "`GL_TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = 0x8C8F`"]
#[doc = "* **Groups:** TransformFeedbackPName, GetPName"]
pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = 0x8C8F;
#[doc = "`GL_TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = 0x8C7F`"]
#[doc = "* **Group:** ProgramPropertyARB"]
pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = 0x8C7F;
#[doc = "`GL_TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = 0x8C85`"]
#[doc = "* **Groups:** TransformFeedbackPName, GetPName"]
pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = 0x8C85;
#[doc = "`GL_TRANSFORM_FEEDBACK_BUFFER_START: GLenum = 0x8C84`"]
#[doc = "* **Groups:** TransformFeedbackPName, GetPName"]
pub const GL_TRANSFORM_FEEDBACK_BUFFER_START: GLenum = 0x8C84;
#[doc = "`GL_TRANSFORM_FEEDBACK_PAUSED: GLenum = 0x8E23`"]
#[doc = "* **Group:** TransformFeedbackPName"]
#[doc = "* **Alias Of:** `GL_TRANSFORM_FEEDBACK_BUFFER_PAUSED`"]
pub const GL_TRANSFORM_FEEDBACK_PAUSED: GLenum = 0x8E23;
#[doc = "`GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = 0x8C88`"]
#[doc = "* **Group:** QueryTarget"]
pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = 0x8C88;
#[doc = "`GL_TRANSFORM_FEEDBACK_VARYING: GLenum = 0x92F4`"]
#[doc = "* **Group:** ProgramInterface"]
pub const GL_TRANSFORM_FEEDBACK_VARYING: GLenum = 0x92F4;
#[doc = "`GL_TRANSFORM_FEEDBACK_VARYINGS: GLenum = 0x8C83`"]
#[doc = "* **Group:** ProgramPropertyARB"]
pub const GL_TRANSFORM_FEEDBACK_VARYINGS: GLenum = 0x8C83;
#[doc = "`GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GLenum = 0x8C76`"]
#[doc = "* **Group:** ProgramPropertyARB"]
pub const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GLenum = 0x8C76;
#[doc = "`GL_TRANSLATED_SHADER_SOURCE_LENGTH_ANGLE: GLenum = 0x93A0`"]
#[cfg(any(feature = "GL_ANGLE_translated_shader_source"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_ANGLE_translated_shader_source"))))]
pub const GL_TRANSLATED_SHADER_SOURCE_LENGTH_ANGLE: GLenum = 0x93A0;
#[doc = "`GL_TRANSLATE_2D_NV: GLenum = 0x9090`"]
#[doc = "* **Group:** PathTransformType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_TRANSLATE_2D_NV: GLenum = 0x9090;
#[doc = "`GL_TRANSLATE_3D_NV: GLenum = 0x9091`"]
#[doc = "* **Group:** PathTransformType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_TRANSLATE_3D_NV: GLenum = 0x9091;
#[doc = "`GL_TRANSLATE_X_NV: GLenum = 0x908E`"]
#[doc = "* **Group:** PathTransformType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_TRANSLATE_X_NV: GLenum = 0x908E;
#[doc = "`GL_TRANSLATE_Y_NV: GLenum = 0x908F`"]
#[doc = "* **Group:** PathTransformType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_TRANSLATE_Y_NV: GLenum = 0x908F;
#[doc = "`GL_TRANSPOSE_AFFINE_2D_NV: GLenum = 0x9096`"]
#[doc = "* **Group:** PathTransformType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_TRANSPOSE_AFFINE_2D_NV: GLenum = 0x9096;
#[doc = "`GL_TRANSPOSE_AFFINE_3D_NV: GLenum = 0x9098`"]
#[doc = "* **Group:** PathTransformType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_TRANSPOSE_AFFINE_3D_NV: GLenum = 0x9098;
#[doc = "`GL_TRIANGLES: GLenum = 0x0004`"]
#[doc = "* **Group:** PrimitiveType"]
pub const GL_TRIANGLES: GLenum = 0x0004;
#[doc = "`GL_TRIANGLES_ADJACENCY: GLenum = 0x000C`"]
#[doc = "* **Group:** PrimitiveType"]
pub const GL_TRIANGLES_ADJACENCY: GLenum = 0x000C;
#[doc = "`GL_TRIANGLES_ADJACENCY_EXT: GLenum = 0x000C`"]
#[doc = "* **Group:** PrimitiveType"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_TRIANGLES_ADJACENCY_EXT: GLenum = 0x000C;
#[doc = "`GL_TRIANGLES_ADJACENCY_OES: GLenum = 0x000C`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_TRIANGLES_ADJACENCY_OES: GLenum = 0x000C;
#[doc = "`GL_TRIANGLE_FAN: GLenum = 0x0006`"]
#[doc = "* **Group:** PrimitiveType"]
pub const GL_TRIANGLE_FAN: GLenum = 0x0006;
#[doc = "`GL_TRIANGLE_STRIP: GLenum = 0x0005`"]
#[doc = "* **Group:** PrimitiveType"]
pub const GL_TRIANGLE_STRIP: GLenum = 0x0005;
#[doc = "`GL_TRIANGLE_STRIP_ADJACENCY: GLenum = 0x000D`"]
#[doc = "* **Group:** PrimitiveType"]
pub const GL_TRIANGLE_STRIP_ADJACENCY: GLenum = 0x000D;
#[doc = "`GL_TRIANGLE_STRIP_ADJACENCY_EXT: GLenum = 0x000D`"]
#[doc = "* **Group:** PrimitiveType"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_TRIANGLE_STRIP_ADJACENCY_EXT: GLenum = 0x000D;
#[doc = "`GL_TRIANGLE_STRIP_ADJACENCY_OES: GLenum = 0x000D`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_TRIANGLE_STRIP_ADJACENCY_OES: GLenum = 0x000D;
#[doc = "`GL_TRIANGULAR_NV: GLenum = 0x90A5`"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_TRIANGULAR_NV: GLenum = 0x90A5;
#[doc = "`GL_TRUE: GLenum = 1`"]
#[doc = "* **Groups:** Boolean, VertexShaderWriteMaskEXT, ClampColorModeARB"]
pub const GL_TRUE: GLboolean = 1;
#[doc = "`GL_TYPE: GLenum = 0x92FA`"]
#[doc = "* **Group:** ProgramResourceProperty"]
pub const GL_TYPE: GLenum = 0x92FA;
#[doc = "`GL_UNCORRELATED_NV: GLenum = 0x9282`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_UNCORRELATED_NV: GLenum = 0x9282;
#[doc = "`GL_UNDEFINED_VERTEX: GLenum = 0x8260`"]
pub const GL_UNDEFINED_VERTEX: GLenum = 0x8260;
#[doc = "`GL_UNDEFINED_VERTEX_EXT: GLenum = 0x8260`"]
#[cfg(any(feature = "GL_EXT_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_geometry_shader"))))]
pub const GL_UNDEFINED_VERTEX_EXT: GLenum = 0x8260;
#[doc = "`GL_UNDEFINED_VERTEX_OES: GLenum = 0x8260`"]
#[cfg(any(feature = "GL_OES_geometry_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_geometry_shader"))))]
pub const GL_UNDEFINED_VERTEX_OES: GLenum = 0x8260;
#[doc = "`GL_UNIFORM: GLenum = 0x92E1`"]
#[doc = "* **Groups:** ProgramResourceProperty, ProgramInterface"]
pub const GL_UNIFORM: GLenum = 0x92E1;
#[doc = "`GL_UNIFORM_ARRAY_STRIDE: GLenum = 0x8A3C`"]
#[doc = "* **Group:** UniformPName"]
pub const GL_UNIFORM_ARRAY_STRIDE: GLenum = 0x8A3C;
#[doc = "`GL_UNIFORM_BARRIER_BIT: GLbitfield = 0x00000004`"]
#[doc = "* **Group:** MemoryBarrierMask"]
pub const GL_UNIFORM_BARRIER_BIT: GLbitfield = 0x00000004;
#[doc = "`GL_UNIFORM_BLOCK: GLenum = 0x92E2`"]
#[doc = "* **Group:** ProgramInterface"]
pub const GL_UNIFORM_BLOCK: GLenum = 0x92E2;
#[doc = "`GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = 0x8A42`"]
#[doc = "* **Group:** UniformBlockPName"]
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = 0x8A42;
#[doc = "`GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = 0x8A43`"]
#[doc = "* **Group:** UniformBlockPName"]
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = 0x8A43;
#[doc = "`GL_UNIFORM_BLOCK_BINDING: GLenum = 0x8A3F`"]
#[doc = "* **Group:** UniformBlockPName"]
pub const GL_UNIFORM_BLOCK_BINDING: GLenum = 0x8A3F;
#[doc = "`GL_UNIFORM_BLOCK_DATA_SIZE: GLenum = 0x8A40`"]
#[doc = "* **Group:** UniformBlockPName"]
pub const GL_UNIFORM_BLOCK_DATA_SIZE: GLenum = 0x8A40;
#[doc = "`GL_UNIFORM_BLOCK_INDEX: GLenum = 0x8A3A`"]
#[doc = "* **Group:** UniformPName"]
pub const GL_UNIFORM_BLOCK_INDEX: GLenum = 0x8A3A;
#[doc = "`GL_UNIFORM_BLOCK_NAME_LENGTH: GLenum = 0x8A41`"]
#[doc = "* **Group:** UniformBlockPName"]
pub const GL_UNIFORM_BLOCK_NAME_LENGTH: GLenum = 0x8A41;
#[doc = "`GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x8A46`"]
#[doc = "* **Group:** UniformBlockPName"]
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x8A46;
#[doc = "`GL_UNIFORM_BLOCK_REFERENCED_BY_MESH_SHADER_NV: GLenum = 0x959C`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_MESH_SHADER_NV: GLenum = 0x959C;
#[doc = "`GL_UNIFORM_BLOCK_REFERENCED_BY_TASK_SHADER_NV: GLenum = 0x959D`"]
#[cfg(any(feature = "GL_NV_mesh_shader"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_mesh_shader"))))]
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TASK_SHADER_NV: GLenum = 0x959D;
#[doc = "`GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x8A44`"]
#[doc = "* **Group:** UniformBlockPName"]
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x8A44;
#[doc = "`GL_UNIFORM_BUFFER: GLenum = 0x8A11`"]
#[doc = "* **Groups:** CopyBufferSubDataTarget, BufferTargetARB, BufferStorageTarget"]
pub const GL_UNIFORM_BUFFER: GLenum = 0x8A11;
#[doc = "`GL_UNIFORM_BUFFER_BINDING: GLenum = 0x8A28`"]
#[doc = "* **Group:** GetPName"]
pub const GL_UNIFORM_BUFFER_BINDING: GLenum = 0x8A28;
#[doc = "`GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x8A34`"]
#[doc = "* **Group:** GetPName"]
pub const GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x8A34;
#[doc = "`GL_UNIFORM_BUFFER_SIZE: GLenum = 0x8A2A`"]
#[doc = "* **Group:** GetPName"]
pub const GL_UNIFORM_BUFFER_SIZE: GLenum = 0x8A2A;
#[doc = "`GL_UNIFORM_BUFFER_START: GLenum = 0x8A29`"]
#[doc = "* **Group:** GetPName"]
pub const GL_UNIFORM_BUFFER_START: GLenum = 0x8A29;
#[doc = "`GL_UNIFORM_IS_ROW_MAJOR: GLenum = 0x8A3E`"]
#[doc = "* **Group:** UniformPName"]
pub const GL_UNIFORM_IS_ROW_MAJOR: GLenum = 0x8A3E;
#[doc = "`GL_UNIFORM_MATRIX_STRIDE: GLenum = 0x8A3D`"]
#[doc = "* **Group:** UniformPName"]
pub const GL_UNIFORM_MATRIX_STRIDE: GLenum = 0x8A3D;
#[doc = "`GL_UNIFORM_NAME_LENGTH: GLenum = 0x8A39`"]
#[doc = "* **Groups:** SubroutineParameterName, UniformPName"]
pub const GL_UNIFORM_NAME_LENGTH: GLenum = 0x8A39;
#[doc = "`GL_UNIFORM_OFFSET: GLenum = 0x8A3B`"]
#[doc = "* **Group:** UniformPName"]
pub const GL_UNIFORM_OFFSET: GLenum = 0x8A3B;
#[doc = "`GL_UNIFORM_SIZE: GLenum = 0x8A38`"]
#[doc = "* **Groups:** SubroutineParameterName, UniformPName"]
pub const GL_UNIFORM_SIZE: GLenum = 0x8A38;
#[doc = "`GL_UNIFORM_TYPE: GLenum = 0x8A37`"]
#[doc = "* **Group:** UniformPName"]
pub const GL_UNIFORM_TYPE: GLenum = 0x8A37;
#[doc = "`GL_UNKNOWN_CONTEXT_RESET: GLenum = 0x8255`"]
#[doc = "* **Group:** GraphicsResetStatus"]
pub const GL_UNKNOWN_CONTEXT_RESET: GLenum = 0x8255;
#[doc = "`GL_UNKNOWN_CONTEXT_RESET_EXT: GLenum = 0x8255`"]
#[cfg(any(feature = "GL_EXT_robustness"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_robustness"))))]
pub const GL_UNKNOWN_CONTEXT_RESET_EXT: GLenum = 0x8255;
#[doc = "`GL_UNKNOWN_CONTEXT_RESET_KHR: GLenum = 0x8255`"]
#[cfg(any(feature = "GL_KHR_robustness"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_robustness"))))]
pub const GL_UNKNOWN_CONTEXT_RESET_KHR: GLenum = 0x8255;
#[doc = "`GL_UNPACK_ALIGNMENT: GLenum = 0x0CF5`"]
#[doc = "* **Groups:** PixelStoreParameter, GetPName"]
pub const GL_UNPACK_ALIGNMENT: GLenum = 0x0CF5;
#[doc = "`GL_UNPACK_IMAGE_HEIGHT: GLenum = 0x806E`"]
#[doc = "* **Groups:** PixelStoreParameter, GetPName"]
pub const GL_UNPACK_IMAGE_HEIGHT: GLenum = 0x806E;
#[doc = "`GL_UNPACK_ROW_LENGTH: GLenum = 0x0CF2`"]
#[doc = "* **Groups:** PixelStoreParameter, GetPName"]
pub const GL_UNPACK_ROW_LENGTH: GLenum = 0x0CF2;
#[doc = "`GL_UNPACK_ROW_LENGTH_EXT: GLenum = 0x0CF2`"]
#[doc = "* **Group:** PixelStoreParameter"]
#[cfg(any(feature = "GL_EXT_unpack_subimage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_unpack_subimage"))))]
pub const GL_UNPACK_ROW_LENGTH_EXT: GLenum = 0x0CF2;
#[doc = "`GL_UNPACK_SKIP_IMAGES: GLenum = 0x806D`"]
#[doc = "* **Groups:** PixelStoreParameter, GetPName"]
pub const GL_UNPACK_SKIP_IMAGES: GLenum = 0x806D;
#[doc = "`GL_UNPACK_SKIP_PIXELS: GLenum = 0x0CF4`"]
#[doc = "* **Groups:** PixelStoreParameter, GetPName"]
pub const GL_UNPACK_SKIP_PIXELS: GLenum = 0x0CF4;
#[doc = "`GL_UNPACK_SKIP_PIXELS_EXT: GLenum = 0x0CF4`"]
#[doc = "* **Group:** PixelStoreParameter"]
#[cfg(any(feature = "GL_EXT_unpack_subimage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_unpack_subimage"))))]
pub const GL_UNPACK_SKIP_PIXELS_EXT: GLenum = 0x0CF4;
#[doc = "`GL_UNPACK_SKIP_ROWS: GLenum = 0x0CF3`"]
#[doc = "* **Groups:** PixelStoreParameter, GetPName"]
pub const GL_UNPACK_SKIP_ROWS: GLenum = 0x0CF3;
#[doc = "`GL_UNPACK_SKIP_ROWS_EXT: GLenum = 0x0CF3`"]
#[doc = "* **Group:** PixelStoreParameter"]
#[cfg(any(feature = "GL_EXT_unpack_subimage"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_unpack_subimage"))))]
pub const GL_UNPACK_SKIP_ROWS_EXT: GLenum = 0x0CF3;
#[doc = "`GL_UNSIGNALED: GLenum = 0x9118`"]
pub const GL_UNSIGNALED: GLenum = 0x9118;
#[doc = "`GL_UNSIGNALED_APPLE: GLenum = 0x9118`"]
#[cfg(any(feature = "GL_APPLE_sync"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_sync"))))]
pub const GL_UNSIGNALED_APPLE: GLenum = 0x9118;
#[doc = "`GL_UNSIGNED_BYTE: GLenum = 0x1401`"]
#[doc = "* **Groups:** VertexAttribIType, ScalarType, ReplacementCodeTypeSUN, ElementPointerTypeATI, MatrixIndexPointerTypeARB, WeightPointerTypeARB, ColorPointerType, DrawElementsType, ListNameType, PixelType, VertexAttribType, VertexAttribPointerType"]
pub const GL_UNSIGNED_BYTE: GLenum = 0x1401;
#[doc = "`GL_UNSIGNED_INT: GLenum = 0x1405`"]
#[doc = "* **Groups:** VertexAttribIType, ScalarType, ReplacementCodeTypeSUN, ElementPointerTypeATI, MatrixIndexPointerTypeARB, WeightPointerTypeARB, ColorPointerType, DrawElementsType, ListNameType, PixelFormat, PixelType, VertexAttribType, AttributeType, UniformType, VertexAttribPointerType, GlslTypeToken"]
pub const GL_UNSIGNED_INT: GLenum = 0x1405;
#[doc = "`GL_UNSIGNED_INT16_NV: GLenum = 0x8FF0`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_UNSIGNED_INT16_NV: GLenum = 0x8FF0;
#[doc = "`GL_UNSIGNED_INT16_VEC2_NV: GLenum = 0x8FF1`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_UNSIGNED_INT16_VEC2_NV: GLenum = 0x8FF1;
#[doc = "`GL_UNSIGNED_INT16_VEC3_NV: GLenum = 0x8FF2`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_UNSIGNED_INT16_VEC3_NV: GLenum = 0x8FF2;
#[doc = "`GL_UNSIGNED_INT16_VEC4_NV: GLenum = 0x8FF3`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_UNSIGNED_INT16_VEC4_NV: GLenum = 0x8FF3;
#[doc = "`GL_UNSIGNED_INT64_AMD: GLenum = 0x8BC2`"]
#[cfg(any(feature = "GL_AMD_performance_monitor"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_AMD_performance_monitor"))))]
pub const GL_UNSIGNED_INT64_AMD: GLenum = 0x8BC2;
#[doc = "`GL_UNSIGNED_INT64_NV: GLenum = 0x140F`"]
#[doc = "* **Groups:** VertexAttribPointerType, AttributeType"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_UNSIGNED_INT64_NV: GLenum = 0x140F;
#[doc = "`GL_UNSIGNED_INT64_VEC2_NV: GLenum = 0x8FF5`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_UNSIGNED_INT64_VEC2_NV: GLenum = 0x8FF5;
#[doc = "`GL_UNSIGNED_INT64_VEC3_NV: GLenum = 0x8FF6`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_UNSIGNED_INT64_VEC3_NV: GLenum = 0x8FF6;
#[doc = "`GL_UNSIGNED_INT64_VEC4_NV: GLenum = 0x8FF7`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_UNSIGNED_INT64_VEC4_NV: GLenum = 0x8FF7;
#[doc = "`GL_UNSIGNED_INT8_NV: GLenum = 0x8FEC`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_UNSIGNED_INT8_NV: GLenum = 0x8FEC;
#[doc = "`GL_UNSIGNED_INT8_VEC2_NV: GLenum = 0x8FED`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_UNSIGNED_INT8_VEC2_NV: GLenum = 0x8FED;
#[doc = "`GL_UNSIGNED_INT8_VEC3_NV: GLenum = 0x8FEE`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_UNSIGNED_INT8_VEC3_NV: GLenum = 0x8FEE;
#[doc = "`GL_UNSIGNED_INT8_VEC4_NV: GLenum = 0x8FEF`"]
#[cfg(any(feature = "GL_NV_gpu_shader5"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_gpu_shader5"))))]
pub const GL_UNSIGNED_INT8_VEC4_NV: GLenum = 0x8FEF;
#[doc = "`GL_UNSIGNED_INT_10F_11F_11F_REV: GLenum = 0x8C3B`"]
#[doc = "* **Groups:** VertexAttribPointerType, VertexAttribType"]
pub const GL_UNSIGNED_INT_10F_11F_11F_REV: GLenum = 0x8C3B;
#[doc = "`GL_UNSIGNED_INT_10F_11F_11F_REV_APPLE: GLenum = 0x8C3B`"]
#[cfg(any(feature = "GL_APPLE_texture_packed_float"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_texture_packed_float"))))]
pub const GL_UNSIGNED_INT_10F_11F_11F_REV_APPLE: GLenum = 0x8C3B;
#[doc = "`GL_UNSIGNED_INT_10_10_10_2_OES: GLenum = 0x8DF6`"]
#[cfg(any(feature = "GL_OES_vertex_type_10_10_10_2"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_vertex_type_10_10_10_2"))))]
pub const GL_UNSIGNED_INT_10_10_10_2_OES: GLenum = 0x8DF6;
#[doc = "`GL_UNSIGNED_INT_24_8: GLenum = 0x84FA`"]
pub const GL_UNSIGNED_INT_24_8: GLenum = 0x84FA;
#[doc = "`GL_UNSIGNED_INT_24_8_OES: GLenum = 0x84FA`"]
#[cfg(any(
    feature = "GL_ANGLE_depth_texture",
    feature = "GL_OES_packed_depth_stencil"
))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(
        feature = "GL_ANGLE_depth_texture",
        feature = "GL_OES_packed_depth_stencil"
    )))
)]
pub const GL_UNSIGNED_INT_24_8_OES: GLenum = 0x84FA;
#[doc = "`GL_UNSIGNED_INT_2_10_10_10_REV: GLenum = 0x8368`"]
#[doc = "* **Groups:** VertexAttribPointerType, VertexAttribType"]
pub const GL_UNSIGNED_INT_2_10_10_10_REV: GLenum = 0x8368;
#[doc = "`GL_UNSIGNED_INT_2_10_10_10_REV_EXT: GLenum = 0x8368`"]
#[cfg(any(feature = "GL_EXT_texture_type_2_10_10_10_REV"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_type_2_10_10_10_REV"))))]
pub const GL_UNSIGNED_INT_2_10_10_10_REV_EXT: GLenum = 0x8368;
#[doc = "`GL_UNSIGNED_INT_5_9_9_9_REV: GLenum = 0x8C3E`"]
pub const GL_UNSIGNED_INT_5_9_9_9_REV: GLenum = 0x8C3E;
#[doc = "`GL_UNSIGNED_INT_5_9_9_9_REV_APPLE: GLenum = 0x8C3E`"]
#[cfg(any(feature = "GL_APPLE_texture_packed_float"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_texture_packed_float"))))]
pub const GL_UNSIGNED_INT_5_9_9_9_REV_APPLE: GLenum = 0x8C3E;
#[doc = "`GL_UNSIGNED_INT_ATOMIC_COUNTER: GLenum = 0x92DB`"]
#[doc = "* **Group:** GlslTypeToken"]
pub const GL_UNSIGNED_INT_ATOMIC_COUNTER: GLenum = 0x92DB;
#[doc = "`GL_UNSIGNED_INT_IMAGE_2D: GLenum = 0x9063`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType"]
pub const GL_UNSIGNED_INT_IMAGE_2D: GLenum = 0x9063;
#[doc = "`GL_UNSIGNED_INT_IMAGE_2D_ARRAY: GLenum = 0x9069`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType"]
pub const GL_UNSIGNED_INT_IMAGE_2D_ARRAY: GLenum = 0x9069;
#[doc = "`GL_UNSIGNED_INT_IMAGE_3D: GLenum = 0x9064`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType"]
pub const GL_UNSIGNED_INT_IMAGE_3D: GLenum = 0x9064;
#[doc = "`GL_UNSIGNED_INT_IMAGE_BUFFER: GLenum = 0x9067`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType"]
pub const GL_UNSIGNED_INT_IMAGE_BUFFER: GLenum = 0x9067;
#[doc = "`GL_UNSIGNED_INT_IMAGE_BUFFER_EXT: GLenum = 0x9067`"]
#[cfg(any(feature = "GL_EXT_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_buffer"))))]
pub const GL_UNSIGNED_INT_IMAGE_BUFFER_EXT: GLenum = 0x9067;
#[doc = "`GL_UNSIGNED_INT_IMAGE_BUFFER_OES: GLenum = 0x9067`"]
#[cfg(any(feature = "GL_OES_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_buffer"))))]
pub const GL_UNSIGNED_INT_IMAGE_BUFFER_OES: GLenum = 0x9067;
#[doc = "`GL_UNSIGNED_INT_IMAGE_CUBE: GLenum = 0x9066`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType"]
pub const GL_UNSIGNED_INT_IMAGE_CUBE: GLenum = 0x9066;
#[doc = "`GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x906A`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType"]
pub const GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x906A;
#[doc = "`GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY_EXT: GLenum = 0x906A`"]
#[cfg(any(feature = "GL_EXT_texture_cube_map_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_cube_map_array"))))]
pub const GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY_EXT: GLenum = 0x906A;
#[doc = "`GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY_OES: GLenum = 0x906A`"]
#[cfg(any(feature = "GL_OES_texture_cube_map_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_cube_map_array"))))]
pub const GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY_OES: GLenum = 0x906A;
#[doc = "`GL_UNSIGNED_INT_SAMPLER_2D: GLenum = 0x8DD2`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_UNSIGNED_INT_SAMPLER_2D: GLenum = 0x8DD2;
#[doc = "`GL_UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = 0x8DD7`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = 0x8DD7;
#[doc = "`GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x910A`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x910A;
#[doc = "`GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910D`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910D;
#[doc = "`GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY_OES: GLenum = 0x910D`"]
#[cfg(any(feature = "GL_OES_texture_storage_multisample_2d_array"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_OES_texture_storage_multisample_2d_array")))
)]
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY_OES: GLenum = 0x910D;
#[doc = "`GL_UNSIGNED_INT_SAMPLER_3D: GLenum = 0x8DD3`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_UNSIGNED_INT_SAMPLER_3D: GLenum = 0x8DD3;
#[doc = "`GL_UNSIGNED_INT_SAMPLER_BUFFER: GLenum = 0x8DD8`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_UNSIGNED_INT_SAMPLER_BUFFER: GLenum = 0x8DD8;
#[doc = "`GL_UNSIGNED_INT_SAMPLER_BUFFER_EXT: GLenum = 0x8DD8`"]
#[cfg(any(feature = "GL_EXT_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_buffer"))))]
pub const GL_UNSIGNED_INT_SAMPLER_BUFFER_EXT: GLenum = 0x8DD8;
#[doc = "`GL_UNSIGNED_INT_SAMPLER_BUFFER_OES: GLenum = 0x8DD8`"]
#[cfg(any(feature = "GL_OES_texture_buffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_buffer"))))]
pub const GL_UNSIGNED_INT_SAMPLER_BUFFER_OES: GLenum = 0x8DD8;
#[doc = "`GL_UNSIGNED_INT_SAMPLER_CUBE: GLenum = 0x8DD4`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_UNSIGNED_INT_SAMPLER_CUBE: GLenum = 0x8DD4;
#[doc = "`GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900F`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900F;
#[doc = "`GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY_EXT: GLenum = 0x900F`"]
#[cfg(any(feature = "GL_EXT_texture_cube_map_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_cube_map_array"))))]
pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY_EXT: GLenum = 0x900F;
#[doc = "`GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY_OES: GLenum = 0x900F`"]
#[cfg(any(feature = "GL_OES_texture_cube_map_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_texture_cube_map_array"))))]
pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY_OES: GLenum = 0x900F;
#[doc = "`GL_UNSIGNED_INT_VEC2: GLenum = 0x8DC6`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_UNSIGNED_INT_VEC2: GLenum = 0x8DC6;
#[doc = "`GL_UNSIGNED_INT_VEC3: GLenum = 0x8DC7`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_UNSIGNED_INT_VEC3: GLenum = 0x8DC7;
#[doc = "`GL_UNSIGNED_INT_VEC4: GLenum = 0x8DC8`"]
#[doc = "* **Groups:** GlslTypeToken, AttributeType, UniformType"]
pub const GL_UNSIGNED_INT_VEC4: GLenum = 0x8DC8;
#[doc = "`GL_UNSIGNED_NORMALIZED: GLenum = 0x8C17`"]
pub const GL_UNSIGNED_NORMALIZED: GLenum = 0x8C17;
#[doc = "`GL_UNSIGNED_NORMALIZED_EXT: GLenum = 0x8C17`"]
#[cfg(any(feature = "GL_EXT_color_buffer_half_float"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_color_buffer_half_float"))))]
pub const GL_UNSIGNED_NORMALIZED_EXT: GLenum = 0x8C17;
#[doc = "`GL_UNSIGNED_SHORT: GLenum = 0x1403`"]
#[doc = "* **Groups:** VertexAttribIType, ScalarType, ReplacementCodeTypeSUN, ElementPointerTypeATI, MatrixIndexPointerTypeARB, WeightPointerTypeARB, ColorPointerType, DrawElementsType, ListNameType, PixelFormat, PixelType, VertexAttribType, VertexAttribPointerType"]
pub const GL_UNSIGNED_SHORT: GLenum = 0x1403;
#[doc = "`GL_UNSIGNED_SHORT_1_5_5_5_REV_EXT: GLenum = 0x8366`"]
#[cfg(any(feature = "GL_EXT_read_format_bgra"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_read_format_bgra"))))]
pub const GL_UNSIGNED_SHORT_1_5_5_5_REV_EXT: GLenum = 0x8366;
#[doc = "`GL_UNSIGNED_SHORT_4_4_4_4: GLenum = 0x8033`"]
#[doc = "* **Group:** PixelType"]
pub const GL_UNSIGNED_SHORT_4_4_4_4: GLenum = 0x8033;
#[doc = "`GL_UNSIGNED_SHORT_4_4_4_4_REV_EXT: GLenum = 0x8365`"]
#[cfg(any(feature = "GL_EXT_read_format_bgra"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_read_format_bgra"))))]
pub const GL_UNSIGNED_SHORT_4_4_4_4_REV_EXT: GLenum = 0x8365;
#[doc = "`GL_UNSIGNED_SHORT_4_4_4_4_REV_IMG: GLenum = 0x8365`"]
#[cfg(any(feature = "GL_IMG_read_format"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_IMG_read_format"))))]
pub const GL_UNSIGNED_SHORT_4_4_4_4_REV_IMG: GLenum = 0x8365;
#[doc = "`GL_UNSIGNED_SHORT_5_5_5_1: GLenum = 0x8034`"]
#[doc = "* **Group:** PixelType"]
pub const GL_UNSIGNED_SHORT_5_5_5_1: GLenum = 0x8034;
#[doc = "`GL_UNSIGNED_SHORT_5_6_5: GLenum = 0x8363`"]
pub const GL_UNSIGNED_SHORT_5_6_5: GLenum = 0x8363;
#[doc = "`GL_UNSIGNED_SHORT_8_8_APPLE: GLenum = 0x85BA`"]
#[cfg(any(feature = "GL_APPLE_rgb_422"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_rgb_422"))))]
pub const GL_UNSIGNED_SHORT_8_8_APPLE: GLenum = 0x85BA;
#[doc = "`GL_UNSIGNED_SHORT_8_8_REV_APPLE: GLenum = 0x85BB`"]
#[cfg(any(feature = "GL_APPLE_rgb_422"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_rgb_422"))))]
pub const GL_UNSIGNED_SHORT_8_8_REV_APPLE: GLenum = 0x85BB;
#[doc = "`GL_UPPER_LEFT_EXT: GLenum = 0x8CA2`"]
#[doc = "* **Alias Of:** `GL_UPPER_LEFT`"]
#[cfg(any(feature = "GL_EXT_clip_control"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_clip_control"))))]
pub const GL_UPPER_LEFT_EXT: GLenum = 0x8CA2;
#[doc = "`GL_USE_MISSING_GLYPH_NV: GLenum = 0x90AA`"]
#[doc = "* **Group:** PathHandleMissingGlyphs"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_USE_MISSING_GLYPH_NV: GLenum = 0x90AA;
#[doc = "`GL_UTF16_NV: GLenum = 0x909B`"]
#[doc = "* **Group:** PathElementType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_UTF16_NV: GLenum = 0x909B;
#[doc = "`GL_UTF8_NV: GLenum = 0x909A`"]
#[doc = "* **Group:** PathElementType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_UTF8_NV: GLenum = 0x909A;
#[doc = "`GL_UUID_SIZE_EXT: GLenum = 16`"]
#[cfg(any(feature = "GL_EXT_memory_object", feature = "GL_EXT_semaphore"))]
#[cfg_attr(
    docs_rs,
    doc(cfg(any(feature = "GL_EXT_memory_object", feature = "GL_EXT_semaphore")))
)]
pub const GL_UUID_SIZE_EXT: GLenum = 16;
#[doc = "`GL_VALIDATE_STATUS: GLenum = 0x8B83`"]
#[doc = "* **Group:** ProgramPropertyARB"]
pub const GL_VALIDATE_STATUS: GLenum = 0x8B83;
#[doc = "`GL_VENDOR: GLenum = 0x1F00`"]
#[doc = "* **Group:** StringName"]
pub const GL_VENDOR: GLenum = 0x1F00;
#[doc = "`GL_VERSION: GLenum = 0x1F02`"]
#[doc = "* **Group:** StringName"]
pub const GL_VERSION: GLenum = 0x1F02;
#[doc = "`GL_VERTEX_ARRAY: GLenum = 0x8074`"]
#[doc = "* **Groups:** ObjectIdentifier, EnableCap, GetPName"]
pub const GL_VERTEX_ARRAY: GLenum = 0x8074;
#[doc = "`GL_VERTEX_ARRAY_BINDING: GLenum = 0x85B5`"]
#[doc = "* **Group:** GetPName"]
pub const GL_VERTEX_ARRAY_BINDING: GLenum = 0x85B5;
#[doc = "`GL_VERTEX_ARRAY_BINDING_OES: GLenum = 0x85B5`"]
#[cfg(any(feature = "GL_OES_vertex_array_object"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_vertex_array_object"))))]
pub const GL_VERTEX_ARRAY_BINDING_OES: GLenum = 0x85B5;
#[doc = "`GL_VERTEX_ARRAY_KHR: GLenum = 0x8074`"]
#[cfg(any(feature = "GL_KHR_debug"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_KHR_debug"))))]
pub const GL_VERTEX_ARRAY_KHR: GLenum = 0x8074;
#[doc = "`GL_VERTEX_ARRAY_OBJECT_EXT: GLenum = 0x9154`"]
#[cfg(any(feature = "GL_EXT_debug_label"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_debug_label"))))]
pub const GL_VERTEX_ARRAY_OBJECT_EXT: GLenum = 0x9154;
#[doc = "`GL_VERTEX_ATTRIB_ARRAY_BARRIER_BIT: GLbitfield = 0x00000001`"]
#[doc = "* **Group:** MemoryBarrierMask"]
pub const GL_VERTEX_ATTRIB_ARRAY_BARRIER_BIT: GLbitfield = 0x00000001;
#[doc = "`GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 0x889F`"]
#[doc = "* **Groups:** VertexAttribEnum, VertexAttribPropertyARB"]
pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 0x889F;
#[doc = "`GL_VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = 0x88FE`"]
#[doc = "* **Groups:** VertexAttribEnum, VertexAttribPropertyARB, VertexArrayPName"]
pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = 0x88FE;
#[doc = "`GL_VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE: GLenum = 0x88FE`"]
#[cfg(any(feature = "GL_ANGLE_instanced_arrays"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_ANGLE_instanced_arrays"))))]
pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE: GLenum = 0x88FE;
#[doc = "`GL_VERTEX_ATTRIB_ARRAY_DIVISOR_EXT: GLenum = 0x88FE`"]
#[cfg(any(feature = "GL_EXT_instanced_arrays"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_instanced_arrays"))))]
pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR_EXT: GLenum = 0x88FE;
#[doc = "`GL_VERTEX_ATTRIB_ARRAY_DIVISOR_NV: GLenum = 0x88FE`"]
#[cfg(any(feature = "GL_NV_instanced_arrays"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_instanced_arrays"))))]
pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR_NV: GLenum = 0x88FE;
#[doc = "`GL_VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 0x8622`"]
#[doc = "* **Groups:** VertexAttribEnum, VertexAttribPropertyARB, VertexArrayPName"]
pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 0x8622;
#[doc = "`GL_VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = 0x88FD`"]
#[doc = "* **Groups:** VertexAttribEnum, VertexAttribPropertyARB, VertexArrayPName"]
pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = 0x88FD;
#[doc = "`GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 0x886A`"]
#[doc = "* **Groups:** VertexAttribEnum, VertexAttribPropertyARB, VertexArrayPName"]
pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 0x886A;
#[doc = "`GL_VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 0x8645`"]
#[doc = "* **Group:** VertexAttribPointerPropertyARB"]
pub const GL_VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 0x8645;
#[doc = "`GL_VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 0x8623`"]
#[doc = "* **Groups:** VertexAttribEnum, VertexAttribPropertyARB, VertexArrayPName"]
pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 0x8623;
#[doc = "`GL_VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 0x8624`"]
#[doc = "* **Groups:** VertexAttribEnum, VertexAttribPropertyARB, VertexArrayPName"]
pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 0x8624;
#[doc = "`GL_VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 0x8625`"]
#[doc = "* **Groups:** VertexAttribEnum, VertexAttribPropertyARB, VertexArrayPName"]
pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 0x8625;
#[doc = "`GL_VERTEX_ATTRIB_BINDING: GLenum = 0x82D4`"]
#[doc = "* **Group:** VertexAttribPropertyARB"]
pub const GL_VERTEX_ATTRIB_BINDING: GLenum = 0x82D4;
#[doc = "`GL_VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D5`"]
#[doc = "* **Groups:** VertexArrayPName, VertexAttribPropertyARB"]
pub const GL_VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D5;
#[doc = "`GL_VERTEX_BINDING_BUFFER: GLenum = 0x8F4F`"]
pub const GL_VERTEX_BINDING_BUFFER: GLenum = 0x8F4F;
#[doc = "`GL_VERTEX_BINDING_DIVISOR: GLenum = 0x82D6`"]
#[doc = "* **Group:** GetPName"]
pub const GL_VERTEX_BINDING_DIVISOR: GLenum = 0x82D6;
#[doc = "`GL_VERTEX_BINDING_OFFSET: GLenum = 0x82D7`"]
#[doc = "* **Group:** GetPName"]
pub const GL_VERTEX_BINDING_OFFSET: GLenum = 0x82D7;
#[doc = "`GL_VERTEX_BINDING_STRIDE: GLenum = 0x82D8`"]
#[doc = "* **Group:** GetPName"]
pub const GL_VERTEX_BINDING_STRIDE: GLenum = 0x82D8;
#[doc = "`GL_VERTEX_SHADER: GLenum = 0x8B31`"]
#[doc = "* **Groups:** PipelineParameterName, ShaderType"]
pub const GL_VERTEX_SHADER: GLenum = 0x8B31;
#[doc = "`GL_VERTEX_SHADER_BIT: GLbitfield = 0x00000001`"]
#[doc = "* **Group:** UseProgramStageMask"]
pub const GL_VERTEX_SHADER_BIT: GLbitfield = 0x00000001;
#[doc = "`GL_VERTEX_SHADER_BIT_EXT: GLbitfield = 0x00000001`"]
#[doc = "* **Group:** UseProgramStageMask"]
#[cfg(any(feature = "GL_EXT_separate_shader_objects"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_separate_shader_objects"))))]
pub const GL_VERTEX_SHADER_BIT_EXT: GLbitfield = 0x00000001;
#[doc = "`GL_VERTICAL_LINE_TO_NV: GLenum = 0x08`"]
#[doc = "* **Group:** PathCoordType"]
#[cfg(any(feature = "GL_NV_path_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_path_rendering"))))]
pub const GL_VERTICAL_LINE_TO_NV: GLenum = 0x08;
#[doc = "`GL_VIEWPORT: GLenum = 0x0BA2`"]
#[doc = "* **Group:** GetPName"]
pub const GL_VIEWPORT: GLenum = 0x0BA2;
#[doc = "`GL_VIEWPORT_BOUNDS_RANGE_NV: GLenum = 0x825D`"]
#[cfg(any(feature = "GL_NV_viewport_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_viewport_array"))))]
pub const GL_VIEWPORT_BOUNDS_RANGE_NV: GLenum = 0x825D;
#[doc = "`GL_VIEWPORT_BOUNDS_RANGE_OES: GLenum = 0x825D`"]
#[cfg(any(feature = "GL_OES_viewport_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_viewport_array"))))]
pub const GL_VIEWPORT_BOUNDS_RANGE_OES: GLenum = 0x825D;
#[doc = "`GL_VIEWPORT_INDEX_PROVOKING_VERTEX_NV: GLenum = 0x825F`"]
#[cfg(any(feature = "GL_NV_viewport_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_viewport_array"))))]
pub const GL_VIEWPORT_INDEX_PROVOKING_VERTEX_NV: GLenum = 0x825F;
#[doc = "`GL_VIEWPORT_INDEX_PROVOKING_VERTEX_OES: GLenum = 0x825F`"]
#[cfg(any(feature = "GL_OES_viewport_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_viewport_array"))))]
pub const GL_VIEWPORT_INDEX_PROVOKING_VERTEX_OES: GLenum = 0x825F;
#[doc = "`GL_VIEWPORT_POSITION_W_SCALE_NV: GLenum = 0x937C`"]
#[cfg(any(feature = "GL_NV_clip_space_w_scaling"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_clip_space_w_scaling"))))]
pub const GL_VIEWPORT_POSITION_W_SCALE_NV: GLenum = 0x937C;
#[doc = "`GL_VIEWPORT_POSITION_W_SCALE_X_COEFF_NV: GLenum = 0x937D`"]
#[cfg(any(feature = "GL_NV_clip_space_w_scaling"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_clip_space_w_scaling"))))]
pub const GL_VIEWPORT_POSITION_W_SCALE_X_COEFF_NV: GLenum = 0x937D;
#[doc = "`GL_VIEWPORT_POSITION_W_SCALE_Y_COEFF_NV: GLenum = 0x937E`"]
#[cfg(any(feature = "GL_NV_clip_space_w_scaling"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_clip_space_w_scaling"))))]
pub const GL_VIEWPORT_POSITION_W_SCALE_Y_COEFF_NV: GLenum = 0x937E;
#[doc = "`GL_VIEWPORT_SUBPIXEL_BITS_NV: GLenum = 0x825C`"]
#[cfg(any(feature = "GL_NV_viewport_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_viewport_array"))))]
pub const GL_VIEWPORT_SUBPIXEL_BITS_NV: GLenum = 0x825C;
#[doc = "`GL_VIEWPORT_SUBPIXEL_BITS_OES: GLenum = 0x825C`"]
#[cfg(any(feature = "GL_OES_viewport_array"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_viewport_array"))))]
pub const GL_VIEWPORT_SUBPIXEL_BITS_OES: GLenum = 0x825C;
#[doc = "`GL_VIEWPORT_SWIZZLE_NEGATIVE_W_NV: GLenum = 0x9357`"]
#[cfg(any(feature = "GL_NV_viewport_swizzle"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_viewport_swizzle"))))]
pub const GL_VIEWPORT_SWIZZLE_NEGATIVE_W_NV: GLenum = 0x9357;
#[doc = "`GL_VIEWPORT_SWIZZLE_NEGATIVE_X_NV: GLenum = 0x9351`"]
#[cfg(any(feature = "GL_NV_viewport_swizzle"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_viewport_swizzle"))))]
pub const GL_VIEWPORT_SWIZZLE_NEGATIVE_X_NV: GLenum = 0x9351;
#[doc = "`GL_VIEWPORT_SWIZZLE_NEGATIVE_Y_NV: GLenum = 0x9353`"]
#[cfg(any(feature = "GL_NV_viewport_swizzle"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_viewport_swizzle"))))]
pub const GL_VIEWPORT_SWIZZLE_NEGATIVE_Y_NV: GLenum = 0x9353;
#[doc = "`GL_VIEWPORT_SWIZZLE_NEGATIVE_Z_NV: GLenum = 0x9355`"]
#[cfg(any(feature = "GL_NV_viewport_swizzle"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_viewport_swizzle"))))]
pub const GL_VIEWPORT_SWIZZLE_NEGATIVE_Z_NV: GLenum = 0x9355;
#[doc = "`GL_VIEWPORT_SWIZZLE_POSITIVE_W_NV: GLenum = 0x9356`"]
#[cfg(any(feature = "GL_NV_viewport_swizzle"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_viewport_swizzle"))))]
pub const GL_VIEWPORT_SWIZZLE_POSITIVE_W_NV: GLenum = 0x9356;
#[doc = "`GL_VIEWPORT_SWIZZLE_POSITIVE_X_NV: GLenum = 0x9350`"]
#[cfg(any(feature = "GL_NV_viewport_swizzle"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_viewport_swizzle"))))]
pub const GL_VIEWPORT_SWIZZLE_POSITIVE_X_NV: GLenum = 0x9350;
#[doc = "`GL_VIEWPORT_SWIZZLE_POSITIVE_Y_NV: GLenum = 0x9352`"]
#[cfg(any(feature = "GL_NV_viewport_swizzle"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_viewport_swizzle"))))]
pub const GL_VIEWPORT_SWIZZLE_POSITIVE_Y_NV: GLenum = 0x9352;
#[doc = "`GL_VIEWPORT_SWIZZLE_POSITIVE_Z_NV: GLenum = 0x9354`"]
#[cfg(any(feature = "GL_NV_viewport_swizzle"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_viewport_swizzle"))))]
pub const GL_VIEWPORT_SWIZZLE_POSITIVE_Z_NV: GLenum = 0x9354;
#[doc = "`GL_VIEWPORT_SWIZZLE_W_NV: GLenum = 0x935B`"]
#[cfg(any(feature = "GL_NV_viewport_swizzle"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_viewport_swizzle"))))]
pub const GL_VIEWPORT_SWIZZLE_W_NV: GLenum = 0x935B;
#[doc = "`GL_VIEWPORT_SWIZZLE_X_NV: GLenum = 0x9358`"]
#[cfg(any(feature = "GL_NV_viewport_swizzle"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_viewport_swizzle"))))]
pub const GL_VIEWPORT_SWIZZLE_X_NV: GLenum = 0x9358;
#[doc = "`GL_VIEWPORT_SWIZZLE_Y_NV: GLenum = 0x9359`"]
#[cfg(any(feature = "GL_NV_viewport_swizzle"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_viewport_swizzle"))))]
pub const GL_VIEWPORT_SWIZZLE_Y_NV: GLenum = 0x9359;
#[doc = "`GL_VIEWPORT_SWIZZLE_Z_NV: GLenum = 0x935A`"]
#[cfg(any(feature = "GL_NV_viewport_swizzle"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_viewport_swizzle"))))]
pub const GL_VIEWPORT_SWIZZLE_Z_NV: GLenum = 0x935A;
#[doc = "`GL_VIRTUAL_PAGE_SIZE_INDEX_EXT: GLenum = 0x91A7`"]
#[cfg(any(feature = "GL_EXT_sparse_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_sparse_texture"))))]
pub const GL_VIRTUAL_PAGE_SIZE_INDEX_EXT: GLenum = 0x91A7;
#[doc = "`GL_VIRTUAL_PAGE_SIZE_X_EXT: GLenum = 0x9195`"]
#[cfg(any(feature = "GL_EXT_sparse_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_sparse_texture"))))]
pub const GL_VIRTUAL_PAGE_SIZE_X_EXT: GLenum = 0x9195;
#[doc = "`GL_VIRTUAL_PAGE_SIZE_Y_EXT: GLenum = 0x9196`"]
#[cfg(any(feature = "GL_EXT_sparse_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_sparse_texture"))))]
pub const GL_VIRTUAL_PAGE_SIZE_Y_EXT: GLenum = 0x9196;
#[doc = "`GL_VIRTUAL_PAGE_SIZE_Z_EXT: GLenum = 0x9197`"]
#[cfg(any(feature = "GL_EXT_sparse_texture"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_sparse_texture"))))]
pub const GL_VIRTUAL_PAGE_SIZE_Z_EXT: GLenum = 0x9197;
#[doc = "`GL_VIVIDLIGHT_NV: GLenum = 0x92A6`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_VIVIDLIGHT_NV: GLenum = 0x92A6;
#[doc = "`GL_WAIT_FAILED: GLenum = 0x911D`"]
#[doc = "* **Group:** SyncStatus"]
pub const GL_WAIT_FAILED: GLenum = 0x911D;
#[doc = "`GL_WAIT_FAILED_APPLE: GLenum = 0x911D`"]
#[cfg(any(feature = "GL_APPLE_sync"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_APPLE_sync"))))]
pub const GL_WAIT_FAILED_APPLE: GLenum = 0x911D;
#[doc = "`GL_WEIGHTED_AVERAGE_EXT: GLenum = 0x9367`"]
#[doc = "* **Alias Of:** `GL_WEIGHTED_AVERAGE_ARB`"]
#[cfg(any(feature = "GL_EXT_texture_filter_minmax"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_texture_filter_minmax"))))]
pub const GL_WEIGHTED_AVERAGE_EXT: GLenum = 0x9367;
#[doc = "`GL_WINDOW_RECTANGLE_EXT: GLenum = 0x8F12`"]
#[cfg(any(feature = "GL_EXT_window_rectangles"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_window_rectangles"))))]
pub const GL_WINDOW_RECTANGLE_EXT: GLenum = 0x8F12;
#[doc = "`GL_WINDOW_RECTANGLE_MODE_EXT: GLenum = 0x8F13`"]
#[cfg(any(feature = "GL_EXT_window_rectangles"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_window_rectangles"))))]
pub const GL_WINDOW_RECTANGLE_MODE_EXT: GLenum = 0x8F13;
#[doc = "`GL_WRITEONLY_RENDERING_QCOM: GLenum = 0x8823`"]
#[cfg(any(feature = "GL_QCOM_writeonly_rendering"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_QCOM_writeonly_rendering"))))]
pub const GL_WRITEONLY_RENDERING_QCOM: GLenum = 0x8823;
#[doc = "`GL_WRITE_ONLY: GLenum = 0x88B9`"]
#[doc = "* **Group:** BufferAccessARB"]
pub const GL_WRITE_ONLY: GLenum = 0x88B9;
#[doc = "`GL_WRITE_ONLY_OES: GLenum = 0x88B9`"]
#[cfg(any(feature = "GL_OES_mapbuffer"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_OES_mapbuffer"))))]
pub const GL_WRITE_ONLY_OES: GLenum = 0x88B9;
#[doc = "`GL_XOR_NV: GLenum = 0x1506`"]
#[cfg(any(feature = "GL_NV_blend_equation_advanced"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_NV_blend_equation_advanced"))))]
pub const GL_XOR_NV: GLenum = 0x1506;
#[doc = "`GL_Z400_BINARY_AMD: GLenum = 0x8740`"]
#[cfg(any(feature = "GL_AMD_program_binary_Z400"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_AMD_program_binary_Z400"))))]
pub const GL_Z400_BINARY_AMD: GLenum = 0x8740;
#[doc = "`GL_ZERO: GLenum = 0`"]
#[doc = "* **Groups:** TextureSwizzle, StencilOp, BlendingFactor"]
pub const GL_ZERO: GLenum = 0;
#[doc = "`GL_ZERO_TO_ONE_EXT: GLenum = 0x935F`"]
#[doc = "* **Alias Of:** `GL_ZERO_TO_ONE`"]
#[cfg(any(feature = "GL_EXT_clip_control"))]
#[cfg_attr(docs_rs, doc(cfg(any(feature = "GL_EXT_clip_control"))))]
pub const GL_ZERO_TO_ONE_EXT: GLenum = 0x935F;
