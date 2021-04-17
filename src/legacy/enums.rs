#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into,
    clippy::upper_case_acronyms
)]

use glib::translate::*;
use glib::{
    error::ErrorDomain,
    value::{FromValue, FromValueOptional, SetValue, Value},
    Quark, StaticType, Type,
};

use std::fmt;

/// Data types for the components of a vertex attribute.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum AttributeType {
    /// Data is the same size of a byte
    Byte,
    /// Data is the same size of an
    ///  unsigned byte
    UnsignedByte,
    /// Data is the same size of a short integer
    Short,
    /// Data is the same size of
    ///  an unsigned short integer
    UnsignedShort,
    /// Data is the same size of a float
    Float,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for AttributeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AttributeType::{}",
            match *self {
                AttributeType::Byte => "Byte",
                AttributeType::UnsignedByte => "UnsignedByte",
                AttributeType::Short => "Short",
                AttributeType::UnsignedShort => "UnsignedShort",
                AttributeType::Float => "Float",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for AttributeType {
    type GlibType = ffi::CoglAttributeType;

    fn to_glib(&self) -> ffi::CoglAttributeType {
        match *self {
            AttributeType::Byte => ffi::COGL_ATTRIBUTE_TYPE_BYTE,
            AttributeType::UnsignedByte => ffi::COGL_ATTRIBUTE_TYPE_UNSIGNED_BYTE,
            AttributeType::Short => ffi::COGL_ATTRIBUTE_TYPE_SHORT,
            AttributeType::UnsignedShort => ffi::COGL_ATTRIBUTE_TYPE_UNSIGNED_SHORT,
            AttributeType::Float => ffi::COGL_ATTRIBUTE_TYPE_FLOAT,
            AttributeType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglAttributeType> for AttributeType {
    fn from_glib(value: ffi::CoglAttributeType) -> Self {
        match value {
            5120 => AttributeType::Byte,
            5121 => AttributeType::UnsignedByte,
            5122 => AttributeType::Short,
            5123 => AttributeType::UnsignedShort,
            5126 => AttributeType::Float,
            value => AttributeType::__Unknown(value),
        }
    }
}

impl StaticType for AttributeType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_attribute_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for AttributeType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for AttributeType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for AttributeType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Error codes that can be thrown when performing bitmap
/// operations. Note that `gdk_pixbuf_new_from_file` can also throw
/// errors directly from the underlying image loading library. For
/// example, if `GdkPixbuf` is used then errors ``GdkPixbufError``<!-- -->s
/// will be used directly.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum BitmapError {
    /// Generic failure code, something went
    ///  wrong.
    Failed,
    /// Unknown image type.
    UnknownType,
    /// An image file was broken somehow.
    CorruptImage,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for BitmapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BitmapError::{}",
            match *self {
                BitmapError::Failed => "Failed",
                BitmapError::UnknownType => "UnknownType",
                BitmapError::CorruptImage => "CorruptImage",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for BitmapError {
    type GlibType = ffi::CoglBitmapError;

    fn to_glib(&self) -> ffi::CoglBitmapError {
        match *self {
            BitmapError::Failed => ffi::COGL_BITMAP_ERROR_FAILED,
            BitmapError::UnknownType => ffi::COGL_BITMAP_ERROR_UNKNOWN_TYPE,
            BitmapError::CorruptImage => ffi::COGL_BITMAP_ERROR_CORRUPT_IMAGE,
            BitmapError::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglBitmapError> for BitmapError {
    fn from_glib(value: ffi::CoglBitmapError) -> Self {
        match value {
            0 => BitmapError::Failed,
            1 => BitmapError::UnknownType,
            2 => BitmapError::CorruptImage,
            value => BitmapError::__Unknown(value),
        }
    }
}

impl ErrorDomain for BitmapError {
    fn domain() -> Quark {
        unsafe { from_glib(ffi::cogl_bitmap_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            0 => Some(BitmapError::Failed),
            1 => Some(BitmapError::UnknownType),
            2 => Some(BitmapError::CorruptImage),
            _ => Some(BitmapError::Failed),
        }
    }
}

impl StaticType for BitmapError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_bitmap_error_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for BitmapError {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for BitmapError {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for BitmapError {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Error enumeration for the blend strings parser
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum BlendStringError {
    /// Generic parse error
    ParseError,
    /// Argument parse error
    ArgumentParseError,
    /// Internal parser error
    InvalidError,
    /// Blend string not
    ///  supported by the GPU
    GpuUnsupportedError,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for BlendStringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BlendStringError::{}",
            match *self {
                BlendStringError::ParseError => "ParseError",
                BlendStringError::ArgumentParseError => "ArgumentParseError",
                BlendStringError::InvalidError => "InvalidError",
                BlendStringError::GpuUnsupportedError => "GpuUnsupportedError",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for BlendStringError {
    type GlibType = ffi::CoglBlendStringError;

    fn to_glib(&self) -> ffi::CoglBlendStringError {
        match *self {
            BlendStringError::ParseError => ffi::COGL_BLEND_STRING_ERROR_PARSE_ERROR,
            BlendStringError::ArgumentParseError => {
                ffi::COGL_BLEND_STRING_ERROR_ARGUMENT_PARSE_ERROR
            }
            BlendStringError::InvalidError => ffi::COGL_BLEND_STRING_ERROR_INVALID_ERROR,
            BlendStringError::GpuUnsupportedError => {
                ffi::COGL_BLEND_STRING_ERROR_GPU_UNSUPPORTED_ERROR
            }
            BlendStringError::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglBlendStringError> for BlendStringError {
    fn from_glib(value: ffi::CoglBlendStringError) -> Self {
        match value {
            0 => BlendStringError::ParseError,
            1 => BlendStringError::ArgumentParseError,
            2 => BlendStringError::InvalidError,
            3 => BlendStringError::GpuUnsupportedError,
            value => BlendStringError::__Unknown(value),
        }
    }
}

impl ErrorDomain for BlendStringError {
    fn domain() -> Quark {
        unsafe { from_glib(ffi::cogl_blend_string_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            0 => Some(BlendStringError::ParseError),
            1 => Some(BlendStringError::ArgumentParseError),
            2 => Some(BlendStringError::InvalidError),
            3 => Some(BlendStringError::GpuUnsupportedError),
            value => Some(BlendStringError::__Unknown(value)),
        }
    }
}

impl StaticType for BlendStringError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_blend_string_error_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for BlendStringError {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for BlendStringError {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for BlendStringError {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Error enumeration for `Buffer`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum BufferError {
    /// A buffer could not be mapped either
    ///  because the feature isn't supported or because a system
    ///  limitation was hit.
    BufferErrorMap,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for BufferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BufferError::{}",
            match *self {
                BufferError::BufferErrorMap => "BufferErrorMap",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for BufferError {
    type GlibType = ffi::CoglBufferError;

    fn to_glib(&self) -> ffi::CoglBufferError {
        match *self {
            BufferError::BufferErrorMap => ffi::COGL_BUFFER_ERROR_MAP,
            BufferError::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglBufferError> for BufferError {
    fn from_glib(value: ffi::CoglBufferError) -> Self {
        match value {
            0 => BufferError::BufferErrorMap,
            value => BufferError::__Unknown(value),
        }
    }
}

/// The update hint on a buffer allows the user to give some detail on how often
/// the buffer data is going to be updated.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum BufferUpdateHint {
    /// the buffer will not change over time
    Static,
    /// the buffer will change from time to time
    Dynamic,
    /// the buffer will be used once or a couple of
    ///  times
    Stream,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for BufferUpdateHint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BufferUpdateHint::{}",
            match *self {
                BufferUpdateHint::Static => "Static",
                BufferUpdateHint::Dynamic => "Dynamic",
                BufferUpdateHint::Stream => "Stream",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for BufferUpdateHint {
    type GlibType = ffi::CoglBufferUpdateHint;

    fn to_glib(&self) -> ffi::CoglBufferUpdateHint {
        match *self {
            BufferUpdateHint::Static => ffi::COGL_BUFFER_UPDATE_HINT_STATIC,
            BufferUpdateHint::Dynamic => ffi::COGL_BUFFER_UPDATE_HINT_DYNAMIC,
            BufferUpdateHint::Stream => ffi::COGL_BUFFER_UPDATE_HINT_STREAM,
            BufferUpdateHint::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglBufferUpdateHint> for BufferUpdateHint {
    fn from_glib(value: ffi::CoglBufferUpdateHint) -> Self {
        match value {
            0 => BufferUpdateHint::Static,
            1 => BufferUpdateHint::Dynamic,
            2 => BufferUpdateHint::Stream,
            value => BufferUpdateHint::__Unknown(value),
        }
    }
}

/// When using depth testing one of these functions is used to compare
/// the depth of an incoming fragment against the depth value currently
/// stored in the depth buffer. The function is changed using
/// `DepthState::set_test_function`.
///
/// The test is only done when depth testing is explicitly enabled. (See
/// `DepthState::set_test_enabled`)
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum DepthTestFunction {
    /// Never passes.
    Never,
    /// Passes if the fragment's depth
    /// value is less than the value currently in the depth buffer.
    Less,
    /// Passes if the fragment's depth
    /// value is equal to the value currently in the depth buffer.
    Equal,
    /// Passes if the fragment's depth
    /// value is less or equal to the value currently in the depth buffer.
    Lequal,
    /// Passes if the fragment's depth
    /// value is greater than the value currently in the depth buffer.
    Greater,
    /// Passes if the fragment's depth
    /// value is not equal to the value currently in the depth buffer.
    Notequal,
    /// Passes if the fragment's depth
    /// value greater than or equal to the value currently in the depth buffer.
    Gequal,
    /// Always passes.
    Always,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for DepthTestFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DepthTestFunction::{}",
            match *self {
                DepthTestFunction::Never => "Never",
                DepthTestFunction::Less => "Less",
                DepthTestFunction::Equal => "Equal",
                DepthTestFunction::Lequal => "Lequal",
                DepthTestFunction::Greater => "Greater",
                DepthTestFunction::Notequal => "Notequal",
                DepthTestFunction::Gequal => "Gequal",
                DepthTestFunction::Always => "Always",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for DepthTestFunction {
    type GlibType = ffi::CoglDepthTestFunction;

    fn to_glib(&self) -> ffi::CoglDepthTestFunction {
        match *self {
            DepthTestFunction::Never => ffi::COGL_DEPTH_TEST_FUNCTION_NEVER,
            DepthTestFunction::Less => ffi::COGL_DEPTH_TEST_FUNCTION_LESS,
            DepthTestFunction::Equal => ffi::COGL_DEPTH_TEST_FUNCTION_EQUAL,
            DepthTestFunction::Lequal => ffi::COGL_DEPTH_TEST_FUNCTION_LEQUAL,
            DepthTestFunction::Greater => ffi::COGL_DEPTH_TEST_FUNCTION_GREATER,
            DepthTestFunction::Notequal => ffi::COGL_DEPTH_TEST_FUNCTION_NOTEQUAL,
            DepthTestFunction::Gequal => ffi::COGL_DEPTH_TEST_FUNCTION_GEQUAL,
            DepthTestFunction::Always => ffi::COGL_DEPTH_TEST_FUNCTION_ALWAYS,
            DepthTestFunction::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglDepthTestFunction> for DepthTestFunction {
    fn from_glib(value: ffi::CoglDepthTestFunction) -> Self {
        match value {
            512 => DepthTestFunction::Never,
            513 => DepthTestFunction::Less,
            514 => DepthTestFunction::Equal,
            515 => DepthTestFunction::Lequal,
            516 => DepthTestFunction::Greater,
            517 => DepthTestFunction::Notequal,
            518 => DepthTestFunction::Gequal,
            519 => DepthTestFunction::Always,
            value => DepthTestFunction::__Unknown(value),
        }
    }
}

impl StaticType for DepthTestFunction {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_depth_test_function_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for DepthTestFunction {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for DepthTestFunction {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for DepthTestFunction {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Identifiers for underlying hardware drivers that may be used by
/// Cogl for rendering.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum Driver {
    /// Implies no preference for which driver is used
    Any,
    /// A No-Op driver.
    Nop,
    /// An OpenGL driver.
    Gl,
    /// An OpenGL driver using the core GL 3.1 profile
    Gl3,
    /// An OpenGL ES 1.1 driver.
    Gles1,
    /// An OpenGL ES 2.0 driver.
    Gles2,
    /// A WebGL driver.
    Webgl,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for Driver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Driver::{}",
            match *self {
                Driver::Any => "Any",
                Driver::Nop => "Nop",
                Driver::Gl => "Gl",
                Driver::Gl3 => "Gl3",
                Driver::Gles1 => "Gles1",
                Driver::Gles2 => "Gles2",
                Driver::Webgl => "Webgl",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for Driver {
    type GlibType = ffi::CoglDriver;

    fn to_glib(&self) -> ffi::CoglDriver {
        match *self {
            Driver::Any => ffi::COGL_DRIVER_ANY,
            Driver::Nop => ffi::COGL_DRIVER_NOP,
            Driver::Gl => ffi::COGL_DRIVER_GL,
            Driver::Gl3 => ffi::COGL_DRIVER_GL3,
            Driver::Gles1 => ffi::COGL_DRIVER_GLES1,
            Driver::Gles2 => ffi::COGL_DRIVER_GLES2,
            Driver::Webgl => ffi::COGL_DRIVER_WEBGL,
            Driver::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglDriver> for Driver {
    fn from_glib(value: ffi::CoglDriver) -> Self {
        match value {
            0 => Driver::Any,
            1 => Driver::Nop,
            2 => Driver::Gl,
            3 => Driver::Gl3,
            4 => Driver::Gles1,
            5 => Driver::Gles2,
            6 => Driver::Webgl,
            value => Driver::__Unknown(value),
        }
    }
}

/// All the capabilities that can vary between different GPUs supported
/// by Cogl. Applications that depend on any of these features should explicitly
/// check for them using `has_feature` or `has_features`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum FeatureID {
    /// The hardware supports non power
    ///  of two textures, but you also need to check the
    ///  `FeatureID::OglFeatureIdTextureNpotMipmap` and `FeatureID::OglFeatureIdTextureNpotRepeat`
    ///  features to know if the hardware supports npot texture mipmaps
    ///  or repeat modes other than
    ///  `PipelineWrapMode::ClampToEdge` respectively.
    OglFeatureIdTextureNpotBasic,
    /// Mipmapping is supported in
    ///  conjuntion with non power of two textures.
    OglFeatureIdTextureNpotMipmap,
    /// Repeat modes other than
    ///  `PipelineWrapMode::ClampToEdge` are supported by the
    ///  hardware.
    OglFeatureIdTextureNpotRepeat,
    /// Non power of two textures are supported
    ///  by the hardware. This is a equivalent to the
    ///  `FeatureID::OglFeatureIdTextureNpotBasic`, `FeatureID::OglFeatureIdTextureNpotMipmap`
    ///  and `FeatureID::OglFeatureIdTextureNpotRepeat` features combined.
    OglFeatureIdTextureNpot,
    /// Support for rectangular
    ///  textures with non-normalized texture coordinates.
    OglFeatureIdTextureRectangle,
    /// 3D texture support
    OglFeatureIdTexture3d,
    /// GLSL support
    OglFeatureIdGlsl,
    /// ARBFP support
    OglFeatureIdArbfp,
    /// Offscreen rendering support
    OglFeatureIdOffscreen,
    /// Multisample support for
    ///  offscreen framebuffers
    OglFeatureIdOffscreenMultisample,
    /// Multiple onscreen framebuffers
    ///  supported.
    OglFeatureIdOnscreenMultiple,
    /// Set if
    ///  `IndicesType::Int` is supported in
    ///  `Indices::new`.
    OglFeatureIdUnsignedIntIndices,
    /// `pipeline_set_depth_range` support
    OglFeatureIdDepthRange,
    /// Whether
    ///  `Pipeline::set_layer_point_sprite_coords_enabled` is supported.
    OglFeatureIdPointSprite,
    /// Whether `buffer_map` is
    ///  supported with CoglBufferAccess including read support.
    OglFeatureIdMapBufferForRead,
    /// Whether `buffer_map` is
    ///  supported with CoglBufferAccess including write support.
    OglFeatureIdMapBufferForWrite,
    /// Whether
    ///  `PipelineWrapMode::MirroredRepeat` is supported.
    OglFeatureIdMirroredRepeat,
    /// Available if the window system supports reporting an event
    ///  for swap buffer completions.
    OglFeatureIdSwapBuffersEvent,
    /// Whether creating new GLES2 contexts is
    ///  suported.
    OglFeatureIdGles2Context,
    /// Whether `Framebuffer` support rendering
    ///  the depth buffer to a texture.
    OglFeatureIdDepthTexture,
    /// Whether frame presentation
    ///  time stamps will be recorded in `FrameInfo` objects.
    OglFeatureIdPresentationTime,
    OglFeatureIdFence,
    /// Whether point_size_in
    ///  can be used as an attribute to set a per-vertex point size.
    OglFeatureIdPerVertexPointSize,
    /// Support for
    ///  `TextureComponents::Rg` as the internal components of a
    ///  texture.
    OglFeatureIdTextureRg,
    /// Available if the age of `Onscreen` back
    ///  buffers are tracked and so `Onscreen::get_buffer_age` can be
    ///  expected to return age values other than 0.
    OglFeatureIdBufferAge,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for FeatureID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FeatureID::{}",
            match *self {
                FeatureID::OglFeatureIdTextureNpotBasic => "OglFeatureIdTextureNpotBasic",
                FeatureID::OglFeatureIdTextureNpotMipmap => "OglFeatureIdTextureNpotMipmap",
                FeatureID::OglFeatureIdTextureNpotRepeat => "OglFeatureIdTextureNpotRepeat",
                FeatureID::OglFeatureIdTextureNpot => "OglFeatureIdTextureNpot",
                FeatureID::OglFeatureIdTextureRectangle => "OglFeatureIdTextureRectangle",
                FeatureID::OglFeatureIdTexture3d => "OglFeatureIdTexture3d",
                FeatureID::OglFeatureIdGlsl => "OglFeatureIdGlsl",
                FeatureID::OglFeatureIdArbfp => "OglFeatureIdArbfp",
                FeatureID::OglFeatureIdOffscreen => "OglFeatureIdOffscreen",
                FeatureID::OglFeatureIdOffscreenMultisample => "OglFeatureIdOffscreenMultisample",
                FeatureID::OglFeatureIdOnscreenMultiple => "OglFeatureIdOnscreenMultiple",
                FeatureID::OglFeatureIdUnsignedIntIndices => "OglFeatureIdUnsignedIntIndices",
                FeatureID::OglFeatureIdDepthRange => "OglFeatureIdDepthRange",
                FeatureID::OglFeatureIdPointSprite => "OglFeatureIdPointSprite",
                FeatureID::OglFeatureIdMapBufferForRead => "OglFeatureIdMapBufferForRead",
                FeatureID::OglFeatureIdMapBufferForWrite => "OglFeatureIdMapBufferForWrite",
                FeatureID::OglFeatureIdMirroredRepeat => "OglFeatureIdMirroredRepeat",
                FeatureID::OglFeatureIdSwapBuffersEvent => "OglFeatureIdSwapBuffersEvent",
                FeatureID::OglFeatureIdGles2Context => "OglFeatureIdGles2Context",
                FeatureID::OglFeatureIdDepthTexture => "OglFeatureIdDepthTexture",
                FeatureID::OglFeatureIdPresentationTime => "OglFeatureIdPresentationTime",
                FeatureID::OglFeatureIdFence => "OglFeatureIdFence",
                FeatureID::OglFeatureIdPerVertexPointSize => "OglFeatureIdPerVertexPointSize",
                FeatureID::OglFeatureIdTextureRg => "OglFeatureIdTextureRg",
                FeatureID::OglFeatureIdBufferAge => "OglFeatureIdBufferAge",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for FeatureID {
    type GlibType = ffi::CoglFeatureID;

    fn to_glib(&self) -> ffi::CoglFeatureID {
        match *self {
            FeatureID::OglFeatureIdTextureNpotBasic => ffi::COGL_FEATURE_ID_TEXTURE_NPOT_BASIC,
            FeatureID::OglFeatureIdTextureNpotMipmap => ffi::COGL_FEATURE_ID_TEXTURE_NPOT_MIPMAP,
            FeatureID::OglFeatureIdTextureNpotRepeat => ffi::COGL_FEATURE_ID_TEXTURE_NPOT_REPEAT,
            FeatureID::OglFeatureIdTextureNpot => ffi::COGL_FEATURE_ID_TEXTURE_NPOT,
            FeatureID::OglFeatureIdTextureRectangle => ffi::COGL_FEATURE_ID_TEXTURE_RECTANGLE,
            FeatureID::OglFeatureIdTexture3d => ffi::COGL_FEATURE_ID_TEXTURE_3D,
            FeatureID::OglFeatureIdGlsl => ffi::COGL_FEATURE_ID_GLSL,
            FeatureID::OglFeatureIdArbfp => ffi::COGL_FEATURE_ID_ARBFP,
            FeatureID::OglFeatureIdOffscreen => ffi::COGL_FEATURE_ID_OFFSCREEN,
            FeatureID::OglFeatureIdOffscreenMultisample => {
                ffi::COGL_FEATURE_ID_OFFSCREEN_MULTISAMPLE
            }
            FeatureID::OglFeatureIdOnscreenMultiple => ffi::COGL_FEATURE_ID_ONSCREEN_MULTIPLE,
            FeatureID::OglFeatureIdUnsignedIntIndices => ffi::COGL_FEATURE_ID_UNSIGNED_INT_INDICES,
            FeatureID::OglFeatureIdDepthRange => ffi::COGL_FEATURE_ID_DEPTH_RANGE,
            FeatureID::OglFeatureIdPointSprite => ffi::COGL_FEATURE_ID_POINT_SPRITE,
            FeatureID::OglFeatureIdMapBufferForRead => ffi::COGL_FEATURE_ID_MAP_BUFFER_FOR_READ,
            FeatureID::OglFeatureIdMapBufferForWrite => ffi::COGL_FEATURE_ID_MAP_BUFFER_FOR_WRITE,
            FeatureID::OglFeatureIdMirroredRepeat => ffi::COGL_FEATURE_ID_MIRRORED_REPEAT,
            FeatureID::OglFeatureIdSwapBuffersEvent => ffi::COGL_FEATURE_ID_SWAP_BUFFERS_EVENT,
            FeatureID::OglFeatureIdGles2Context => ffi::COGL_FEATURE_ID_GLES2_CONTEXT,
            FeatureID::OglFeatureIdDepthTexture => ffi::COGL_FEATURE_ID_DEPTH_TEXTURE,
            FeatureID::OglFeatureIdPresentationTime => ffi::COGL_FEATURE_ID_PRESENTATION_TIME,
            FeatureID::OglFeatureIdFence => ffi::COGL_FEATURE_ID_FENCE,
            FeatureID::OglFeatureIdPerVertexPointSize => ffi::COGL_FEATURE_ID_PER_VERTEX_POINT_SIZE,
            FeatureID::OglFeatureIdTextureRg => ffi::COGL_FEATURE_ID_TEXTURE_RG,
            FeatureID::OglFeatureIdBufferAge => ffi::COGL_FEATURE_ID_BUFFER_AGE,
            FeatureID::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglFeatureID> for FeatureID {
    fn from_glib(value: ffi::CoglFeatureID) -> Self {
        match value {
            1 => FeatureID::OglFeatureIdTextureNpotBasic,
            2 => FeatureID::OglFeatureIdTextureNpotMipmap,
            3 => FeatureID::OglFeatureIdTextureNpotRepeat,
            4 => FeatureID::OglFeatureIdTextureNpot,
            5 => FeatureID::OglFeatureIdTextureRectangle,
            6 => FeatureID::OglFeatureIdTexture3d,
            7 => FeatureID::OglFeatureIdGlsl,
            8 => FeatureID::OglFeatureIdArbfp,
            9 => FeatureID::OglFeatureIdOffscreen,
            10 => FeatureID::OglFeatureIdOffscreenMultisample,
            11 => FeatureID::OglFeatureIdOnscreenMultiple,
            12 => FeatureID::OglFeatureIdUnsignedIntIndices,
            13 => FeatureID::OglFeatureIdDepthRange,
            14 => FeatureID::OglFeatureIdPointSprite,
            15 => FeatureID::OglFeatureIdMapBufferForRead,
            16 => FeatureID::OglFeatureIdMapBufferForWrite,
            17 => FeatureID::OglFeatureIdMirroredRepeat,
            18 => FeatureID::OglFeatureIdSwapBuffersEvent,
            19 => FeatureID::OglFeatureIdGles2Context,
            20 => FeatureID::OglFeatureIdDepthTexture,
            21 => FeatureID::OglFeatureIdPresentationTime,
            22 => FeatureID::OglFeatureIdFence,
            23 => FeatureID::OglFeatureIdPerVertexPointSize,
            24 => FeatureID::OglFeatureIdTextureRg,
            25 => FeatureID::OglFeatureIdBufferAge,
            value => FeatureID::__Unknown(value),
        }
    }
}

/// Return values for the `CoglXlibFilterFunc` and `CoglWin32FilterFunc` functions.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum FilterReturn {
    /// The event was not handled, continues the
    ///  processing
    Continue,
    /// Remove the event, stops the processing
    Remove,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for FilterReturn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FilterReturn::{}",
            match *self {
                FilterReturn::Continue => "Continue",
                FilterReturn::Remove => "Remove",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for FilterReturn {
    type GlibType = ffi::CoglFilterReturn;

    fn to_glib(&self) -> ffi::CoglFilterReturn {
        match *self {
            FilterReturn::Continue => ffi::COGL_FILTER_CONTINUE,
            FilterReturn::Remove => ffi::COGL_FILTER_REMOVE,
            FilterReturn::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglFilterReturn> for FilterReturn {
    fn from_glib(value: ffi::CoglFilterReturn) -> Self {
        match value {
            0 => FilterReturn::Continue,
            1 => FilterReturn::Remove,
            value => FilterReturn::__Unknown(value),
        }
    }
}

impl StaticType for FilterReturn {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_filter_return_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for FilterReturn {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for FilterReturn {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for FilterReturn {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// The fog mode determines the equation used to calculate the fogging blend
/// factor while fogging is enabled. The simplest `FogMode::Linear` mode
/// determines f as:
///
///
/// ```text
///   f = end - eye_distance / end - start
/// ```
///
/// Where eye_distance is the distance of the current fragment in eye
/// coordinates from the origin.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum FogMode {
    /// Calculates the fog blend factor as:
    ///
    /// ```text
    ///   f = end - eye_distance / end - start
    /// ```
    Linear,
    /// Calculates the fog blend factor as:
    ///
    /// ```text
    ///   f = e ^ -(density * eye_distance)
    /// ```
    Exponential,
    /// Calculates the fog blend factor as:
    ///
    /// ```text
    ///   f = e ^ -(density * eye_distance)^2
    /// ```
    ExponentialSquared,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for FogMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FogMode::{}",
            match *self {
                FogMode::Linear => "Linear",
                FogMode::Exponential => "Exponential",
                FogMode::ExponentialSquared => "ExponentialSquared",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for FogMode {
    type GlibType = ffi::CoglFogMode;

    fn to_glib(&self) -> ffi::CoglFogMode {
        match *self {
            FogMode::Linear => ffi::COGL_FOG_MODE_LINEAR,
            FogMode::Exponential => ffi::COGL_FOG_MODE_EXPONENTIAL,
            FogMode::ExponentialSquared => ffi::COGL_FOG_MODE_EXPONENTIAL_SQUARED,
            FogMode::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglFogMode> for FogMode {
    fn from_glib(value: ffi::CoglFogMode) -> Self {
        match value {
            0 => FogMode::Linear,
            1 => FogMode::Exponential,
            2 => FogMode::ExponentialSquared,
            value => FogMode::__Unknown(value),
        }
    }
}

impl StaticType for FogMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_fog_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for FogMode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for FogMode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for FogMode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Identifiers that are passed to `CoglFrameCallback` functions
/// (registered using `Onscreen::add_frame_callback`) that
/// mark the progression of a frame in some way which usually
/// means that new information will have been accumulated in the
/// frame's corresponding `FrameInfo` object.
///
/// The last event that will be sent for a frame will be a
/// `FrameEvent::Complete` event and so these are a good
/// opportunity to collect statistics about a frame since the
/// `FrameInfo` should hold the most data at this point.
///
/// `<note>`A frame may not be completed before the next frame can start
/// so applications should avoid needing to collect all statistics for
/// a particular frame before they can start a new frame.`</note>`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum FrameEvent {
    /// Notifies that the system compositor has
    ///  acknowledged a frame and is ready for a
    ///  new frame to be created.
    Sync,
    /// Notifies that a frame has ended. This
    ///  is a good time for applications to
    ///  collect statistics about the frame
    ///  since the `FrameInfo` should hold
    ///  the most data at this point. No other
    ///  events should be expected after a
    ///  `FrameEvent::Complete` event.
    Complete,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for FrameEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FrameEvent::{}",
            match *self {
                FrameEvent::Sync => "Sync",
                FrameEvent::Complete => "Complete",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for FrameEvent {
    type GlibType = ffi::CoglFrameEvent;

    fn to_glib(&self) -> ffi::CoglFrameEvent {
        match *self {
            FrameEvent::Sync => ffi::COGL_FRAME_EVENT_SYNC,
            FrameEvent::Complete => ffi::COGL_FRAME_EVENT_COMPLETE,
            FrameEvent::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglFrameEvent> for FrameEvent {
    fn from_glib(value: ffi::CoglFrameEvent) -> Self {
        match value {
            1 => FrameEvent::Sync,
            2 => FrameEvent::Complete,
            value => FrameEvent::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum FramebufferError {
    FramebufferErrorAllocate,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for FramebufferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FramebufferError::{}",
            match *self {
                FramebufferError::FramebufferErrorAllocate => "FramebufferErrorAllocate",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for FramebufferError {
    type GlibType = ffi::CoglFramebufferError;

    fn to_glib(&self) -> ffi::CoglFramebufferError {
        match *self {
            FramebufferError::FramebufferErrorAllocate => ffi::COGL_FRAMEBUFFER_ERROR_ALLOCATE,
            FramebufferError::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglFramebufferError> for FramebufferError {
    fn from_glib(value: ffi::CoglFramebufferError) -> Self {
        match value {
            0 => FramebufferError::FramebufferErrorAllocate,
            value => FramebufferError::__Unknown(value),
        }
    }
}

/// Error codes that relate to the gles2_context api.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum GLES2ContextError {
    /// Creating GLES2 contexts
    ///  isn't supported. Applications should use `has_feature` to
    ///  check for the `FeatureID::OglFeatureIdGles2Context`.
    Unsupported,
    /// An underlying driver error
    ///  occured.
    Driver,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for GLES2ContextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "GLES2ContextError::{}",
            match *self {
                GLES2ContextError::Unsupported => "Unsupported",
                GLES2ContextError::Driver => "Driver",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for GLES2ContextError {
    type GlibType = ffi::CoglGLES2ContextError;

    fn to_glib(&self) -> ffi::CoglGLES2ContextError {
        match *self {
            GLES2ContextError::Unsupported => ffi::COGL_GLES2_CONTEXT_ERROR_UNSUPPORTED,
            GLES2ContextError::Driver => ffi::COGL_GLES2_CONTEXT_ERROR_DRIVER,
            GLES2ContextError::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglGLES2ContextError> for GLES2ContextError {
    fn from_glib(value: ffi::CoglGLES2ContextError) -> Self {
        match value {
            0 => GLES2ContextError::Unsupported,
            1 => GLES2ContextError::Driver,
            value => GLES2ContextError::__Unknown(value),
        }
    }
}

/// You should aim to use the smallest data type that gives you enough
/// range, since it reduces the size of your index array and can help
/// reduce the demand on memory bandwidth.
///
/// Note that `IndicesType::Int` is only supported if the
/// `FeatureID::OglFeatureIdUnsignedIntIndices` feature is available. This
/// should always be available on OpenGL but on OpenGL ES it will only
/// be available if the GL_OES_element_index_uint extension is
/// advertized.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum IndicesType {
    /// Your indices are unsigned bytes
    Byte,
    /// Your indices are unsigned shorts
    Short,
    /// Your indices are unsigned ints
    Int,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for IndicesType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IndicesType::{}",
            match *self {
                IndicesType::Byte => "Byte",
                IndicesType::Short => "Short",
                IndicesType::Int => "Int",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for IndicesType {
    type GlibType = ffi::CoglIndicesType;

    fn to_glib(&self) -> ffi::CoglIndicesType {
        match *self {
            IndicesType::Byte => ffi::COGL_INDICES_TYPE_UNSIGNED_BYTE,
            IndicesType::Short => ffi::COGL_INDICES_TYPE_UNSIGNED_SHORT,
            IndicesType::Int => ffi::COGL_INDICES_TYPE_UNSIGNED_INT,
            IndicesType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglIndicesType> for IndicesType {
    fn from_glib(value: ffi::CoglIndicesType) -> Self {
        match value {
            0 => IndicesType::Byte,
            1 => IndicesType::Short,
            2 => IndicesType::Int,
            value => IndicesType::__Unknown(value),
        }
    }
}

impl StaticType for IndicesType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_indices_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for IndicesType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for IndicesType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for IndicesType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Alpha testing happens before blending primitives with the framebuffer and
/// gives an opportunity to discard fragments based on a comparison with the
/// incoming alpha value and a reference alpha value. The `MaterialAlphaFunc`
/// determines how the comparison is done.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum MaterialAlphaFunc {
    /// Never let the fragment through.
    Never,
    /// Let the fragment through if the incoming
    ///  alpha value is less than the reference alpha value
    Less,
    /// Let the fragment through if the incoming
    ///  alpha value equals the reference alpha value
    Equal,
    /// Let the fragment through if the incoming
    ///  alpha value is less than or equal to the reference alpha value
    Lequal,
    /// Let the fragment through if the incoming
    ///  alpha value is greater than the reference alpha value
    Greater,
    /// Let the fragment through if the incoming
    ///  alpha value does not equal the reference alpha value
    Notequal,
    /// Let the fragment through if the incoming
    ///  alpha value is greater than or equal to the reference alpha value.
    Gequal,
    /// Always let the fragment through.
    Always,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for MaterialAlphaFunc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MaterialAlphaFunc::{}",
            match *self {
                MaterialAlphaFunc::Never => "Never",
                MaterialAlphaFunc::Less => "Less",
                MaterialAlphaFunc::Equal => "Equal",
                MaterialAlphaFunc::Lequal => "Lequal",
                MaterialAlphaFunc::Greater => "Greater",
                MaterialAlphaFunc::Notequal => "Notequal",
                MaterialAlphaFunc::Gequal => "Gequal",
                MaterialAlphaFunc::Always => "Always",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for MaterialAlphaFunc {
    type GlibType = ffi::CoglMaterialAlphaFunc;

    fn to_glib(&self) -> ffi::CoglMaterialAlphaFunc {
        match *self {
            MaterialAlphaFunc::Never => ffi::COGL_MATERIAL_ALPHA_FUNC_NEVER,
            MaterialAlphaFunc::Less => ffi::COGL_MATERIAL_ALPHA_FUNC_LESS,
            MaterialAlphaFunc::Equal => ffi::COGL_MATERIAL_ALPHA_FUNC_EQUAL,
            MaterialAlphaFunc::Lequal => ffi::COGL_MATERIAL_ALPHA_FUNC_LEQUAL,
            MaterialAlphaFunc::Greater => ffi::COGL_MATERIAL_ALPHA_FUNC_GREATER,
            MaterialAlphaFunc::Notequal => ffi::COGL_MATERIAL_ALPHA_FUNC_NOTEQUAL,
            MaterialAlphaFunc::Gequal => ffi::COGL_MATERIAL_ALPHA_FUNC_GEQUAL,
            MaterialAlphaFunc::Always => ffi::COGL_MATERIAL_ALPHA_FUNC_ALWAYS,
            MaterialAlphaFunc::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglMaterialAlphaFunc> for MaterialAlphaFunc {
    fn from_glib(value: ffi::CoglMaterialAlphaFunc) -> Self {
        match value {
            512 => MaterialAlphaFunc::Never,
            513 => MaterialAlphaFunc::Less,
            514 => MaterialAlphaFunc::Equal,
            515 => MaterialAlphaFunc::Lequal,
            516 => MaterialAlphaFunc::Greater,
            517 => MaterialAlphaFunc::Notequal,
            518 => MaterialAlphaFunc::Gequal,
            519 => MaterialAlphaFunc::Always,
            value => MaterialAlphaFunc::__Unknown(value),
        }
    }
}

impl StaticType for MaterialAlphaFunc {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_material_alpha_func_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for MaterialAlphaFunc {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for MaterialAlphaFunc {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for MaterialAlphaFunc {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Texture filtering is used whenever the current pixel maps either to more
/// than one texture element (texel) or less than one. These filter enums
/// correspond to different strategies used to come up with a pixel color, by
/// possibly referring to multiple neighbouring texels and taking a weighted
/// average or simply using the nearest texel.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum MaterialFilter {
    /// Measuring in manhatten distance from the,
    ///  current pixel center, use the nearest texture texel
    Nearest,
    /// Use the weighted average of the 4 texels
    ///  nearest the current pixel center
    Linear,
    /// Select the mimap level whose
    ///  texel size most closely matches the current pixel, and use the
    ///  `MaterialFilter::Nearest` criterion
    NearestMipmapNearest,
    /// Select the mimap level whose
    ///  texel size most closely matches the current pixel, and use the
    ///  `MaterialFilter::Linear` criterion
    LinearMipmapNearest,
    /// Select the two mimap levels
    ///  whose texel size most closely matches the current pixel, use
    ///  the `MaterialFilter::Nearest` criterion on each one and take
    ///  their weighted average
    NearestMipmapLinear,
    /// Select the two mimap levels
    ///  whose texel size most closely matches the current pixel, use
    ///  the `MaterialFilter::Linear` criterion on each one and take
    ///  their weighted average
    LinearMipmapLinear,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for MaterialFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MaterialFilter::{}",
            match *self {
                MaterialFilter::Nearest => "Nearest",
                MaterialFilter::Linear => "Linear",
                MaterialFilter::NearestMipmapNearest => "NearestMipmapNearest",
                MaterialFilter::LinearMipmapNearest => "LinearMipmapNearest",
                MaterialFilter::NearestMipmapLinear => "NearestMipmapLinear",
                MaterialFilter::LinearMipmapLinear => "LinearMipmapLinear",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for MaterialFilter {
    type GlibType = ffi::CoglMaterialFilter;

    fn to_glib(&self) -> ffi::CoglMaterialFilter {
        match *self {
            MaterialFilter::Nearest => ffi::COGL_MATERIAL_FILTER_NEAREST,
            MaterialFilter::Linear => ffi::COGL_MATERIAL_FILTER_LINEAR,
            MaterialFilter::NearestMipmapNearest => {
                ffi::COGL_MATERIAL_FILTER_NEAREST_MIPMAP_NEAREST
            }
            MaterialFilter::LinearMipmapNearest => ffi::COGL_MATERIAL_FILTER_LINEAR_MIPMAP_NEAREST,
            MaterialFilter::NearestMipmapLinear => ffi::COGL_MATERIAL_FILTER_NEAREST_MIPMAP_LINEAR,
            MaterialFilter::LinearMipmapLinear => ffi::COGL_MATERIAL_FILTER_LINEAR_MIPMAP_LINEAR,
            MaterialFilter::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglMaterialFilter> for MaterialFilter {
    fn from_glib(value: ffi::CoglMaterialFilter) -> Self {
        match value {
            9728 => MaterialFilter::Nearest,
            9729 => MaterialFilter::Linear,
            9984 => MaterialFilter::NearestMipmapNearest,
            9985 => MaterialFilter::LinearMipmapNearest,
            9986 => MaterialFilter::NearestMipmapLinear,
            9987 => MaterialFilter::LinearMipmapLinear,
            value => MaterialFilter::__Unknown(value),
        }
    }
}

impl StaticType for MaterialFilter {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_material_filter_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for MaterialFilter {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for MaterialFilter {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for MaterialFilter {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Available types of layers for a `Material`. This enumeration
/// might be expanded in later versions.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum MaterialLayerType {
    /// The layer represents a
    ///  <link linkend="cogl-Textures">texture`</link>`
    Texture,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for MaterialLayerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MaterialLayerType::{}",
            match *self {
                MaterialLayerType::Texture => "Texture",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for MaterialLayerType {
    type GlibType = ffi::CoglMaterialLayerType;

    fn to_glib(&self) -> ffi::CoglMaterialLayerType {
        match *self {
            MaterialLayerType::Texture => ffi::COGL_MATERIAL_LAYER_TYPE_TEXTURE,
            MaterialLayerType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglMaterialLayerType> for MaterialLayerType {
    fn from_glib(value: ffi::CoglMaterialLayerType) -> Self {
        match value {
            0 => MaterialLayerType::Texture,
            value => MaterialLayerType::__Unknown(value),
        }
    }
}

impl StaticType for MaterialLayerType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_material_layer_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for MaterialLayerType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for MaterialLayerType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for MaterialLayerType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// The wrap mode specifies what happens when texture coordinates
/// outside the range 0→1 are used. Note that if the filter mode is
/// anything but `MaterialFilter::Nearest` then texels outside the
/// range 0→1 might be used even when the coordinate is exactly 0 or 1
/// because OpenGL will try to sample neighbouring pixels. For example
/// if you are trying to render the full texture then you may get
/// artifacts around the edges when the pixels from the other side are
/// merged in if the wrap mode is set to repeat.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum MaterialWrapMode {
    /// The texture will be repeated. This
    ///  is useful for example to draw a tiled background.
    Repeat,
    /// The coordinates outside the
    ///  range 0→1 will sample copies of the edge pixels of the
    ///  texture. This is useful to avoid artifacts if only one copy of
    ///  the texture is being rendered.
    ClampToEdge,
    /// Cogl will try to automatically
    ///  decide which of the above two to use. For `rectangle`, it
    ///  will use repeat mode if any of the texture coordinates are
    ///  outside the range 0→1, otherwise it will use clamp to edge. For
    ///  `polygon` it will always use repeat mode. For
    ///  `vertex_buffer_draw` it will use repeat mode except for
    ///  layers that have point sprite coordinate generation enabled. This
    ///  is the default value.
    Automatic,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for MaterialWrapMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MaterialWrapMode::{}",
            match *self {
                MaterialWrapMode::Repeat => "Repeat",
                MaterialWrapMode::ClampToEdge => "ClampToEdge",
                MaterialWrapMode::Automatic => "Automatic",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for MaterialWrapMode {
    type GlibType = ffi::CoglMaterialWrapMode;

    fn to_glib(&self) -> ffi::CoglMaterialWrapMode {
        match *self {
            MaterialWrapMode::Repeat => ffi::COGL_MATERIAL_WRAP_MODE_REPEAT,
            MaterialWrapMode::ClampToEdge => ffi::COGL_MATERIAL_WRAP_MODE_CLAMP_TO_EDGE,
            MaterialWrapMode::Automatic => ffi::COGL_MATERIAL_WRAP_MODE_AUTOMATIC,
            MaterialWrapMode::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglMaterialWrapMode> for MaterialWrapMode {
    fn from_glib(value: ffi::CoglMaterialWrapMode) -> Self {
        match value {
            10497 => MaterialWrapMode::Repeat,
            33071 => MaterialWrapMode::ClampToEdge,
            519 => MaterialWrapMode::Automatic,
            value => MaterialWrapMode::__Unknown(value),
        }
    }
}

impl StaticType for MaterialWrapMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_material_wrap_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for MaterialWrapMode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for MaterialWrapMode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for MaterialWrapMode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Alpha testing happens before blending primitives with the framebuffer and
/// gives an opportunity to discard fragments based on a comparison with the
/// incoming alpha value and a reference alpha value. The `PipelineAlphaFunc`
/// determines how the comparison is done.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum PipelineAlphaFunc {
    /// Never let the fragment through.
    Never,
    /// Let the fragment through if the incoming
    ///  alpha value is less than the reference alpha value
    Less,
    /// Let the fragment through if the incoming
    ///  alpha value equals the reference alpha value
    Equal,
    /// Let the fragment through if the incoming
    ///  alpha value is less than or equal to the reference alpha value
    Lequal,
    /// Let the fragment through if the incoming
    ///  alpha value is greater than the reference alpha value
    Greater,
    /// Let the fragment through if the incoming
    ///  alpha value does not equal the reference alpha value
    Notequal,
    /// Let the fragment through if the incoming
    ///  alpha value is greater than or equal to the reference alpha value.
    Gequal,
    /// Always let the fragment through.
    Always,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for PipelineAlphaFunc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PipelineAlphaFunc::{}",
            match *self {
                PipelineAlphaFunc::Never => "Never",
                PipelineAlphaFunc::Less => "Less",
                PipelineAlphaFunc::Equal => "Equal",
                PipelineAlphaFunc::Lequal => "Lequal",
                PipelineAlphaFunc::Greater => "Greater",
                PipelineAlphaFunc::Notequal => "Notequal",
                PipelineAlphaFunc::Gequal => "Gequal",
                PipelineAlphaFunc::Always => "Always",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for PipelineAlphaFunc {
    type GlibType = ffi::CoglPipelineAlphaFunc;

    fn to_glib(&self) -> ffi::CoglPipelineAlphaFunc {
        match *self {
            PipelineAlphaFunc::Never => ffi::COGL_PIPELINE_ALPHA_FUNC_NEVER,
            PipelineAlphaFunc::Less => ffi::COGL_PIPELINE_ALPHA_FUNC_LESS,
            PipelineAlphaFunc::Equal => ffi::COGL_PIPELINE_ALPHA_FUNC_EQUAL,
            PipelineAlphaFunc::Lequal => ffi::COGL_PIPELINE_ALPHA_FUNC_LEQUAL,
            PipelineAlphaFunc::Greater => ffi::COGL_PIPELINE_ALPHA_FUNC_GREATER,
            PipelineAlphaFunc::Notequal => ffi::COGL_PIPELINE_ALPHA_FUNC_NOTEQUAL,
            PipelineAlphaFunc::Gequal => ffi::COGL_PIPELINE_ALPHA_FUNC_GEQUAL,
            PipelineAlphaFunc::Always => ffi::COGL_PIPELINE_ALPHA_FUNC_ALWAYS,
            PipelineAlphaFunc::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglPipelineAlphaFunc> for PipelineAlphaFunc {
    fn from_glib(value: ffi::CoglPipelineAlphaFunc) -> Self {
        match value {
            512 => PipelineAlphaFunc::Never,
            513 => PipelineAlphaFunc::Less,
            514 => PipelineAlphaFunc::Equal,
            515 => PipelineAlphaFunc::Lequal,
            516 => PipelineAlphaFunc::Greater,
            517 => PipelineAlphaFunc::Notequal,
            518 => PipelineAlphaFunc::Gequal,
            519 => PipelineAlphaFunc::Always,
            value => PipelineAlphaFunc::__Unknown(value),
        }
    }
}

/// Specifies which faces should be culled. This can be set on a
/// pipeline using `Pipeline::set_cull_face_mode`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum PipelineCullFaceMode {
    /// Neither face will be
    ///  culled. This is the default.
    None,
    /// Front faces will be culled.
    Front,
    /// Back faces will be culled.
    Back,
    /// All faces will be culled.
    Both,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for PipelineCullFaceMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PipelineCullFaceMode::{}",
            match *self {
                PipelineCullFaceMode::None => "None",
                PipelineCullFaceMode::Front => "Front",
                PipelineCullFaceMode::Back => "Back",
                PipelineCullFaceMode::Both => "Both",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for PipelineCullFaceMode {
    type GlibType = ffi::CoglPipelineCullFaceMode;

    fn to_glib(&self) -> ffi::CoglPipelineCullFaceMode {
        match *self {
            PipelineCullFaceMode::None => ffi::COGL_PIPELINE_CULL_FACE_MODE_NONE,
            PipelineCullFaceMode::Front => ffi::COGL_PIPELINE_CULL_FACE_MODE_FRONT,
            PipelineCullFaceMode::Back => ffi::COGL_PIPELINE_CULL_FACE_MODE_BACK,
            PipelineCullFaceMode::Both => ffi::COGL_PIPELINE_CULL_FACE_MODE_BOTH,
            PipelineCullFaceMode::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglPipelineCullFaceMode> for PipelineCullFaceMode {
    fn from_glib(value: ffi::CoglPipelineCullFaceMode) -> Self {
        match value {
            0 => PipelineCullFaceMode::None,
            1 => PipelineCullFaceMode::Front,
            2 => PipelineCullFaceMode::Back,
            3 => PipelineCullFaceMode::Both,
            value => PipelineCullFaceMode::__Unknown(value),
        }
    }
}

/// Texture filtering is used whenever the current pixel maps either to more
/// than one texture element (texel) or less than one. These filter enums
/// correspond to different strategies used to come up with a pixel color, by
/// possibly referring to multiple neighbouring texels and taking a weighted
/// average or simply using the nearest texel.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum PipelineFilter {
    /// Measuring in manhatten distance from the,
    ///  current pixel center, use the nearest texture texel
    Nearest,
    /// Use the weighted average of the 4 texels
    ///  nearest the current pixel center
    Linear,
    /// Select the mimap level whose
    ///  texel size most closely matches the current pixel, and use the
    ///  `PipelineFilter::Nearest` criterion
    NearestMipmapNearest,
    /// Select the mimap level whose
    ///  texel size most closely matches the current pixel, and use the
    ///  `PipelineFilter::Linear` criterion
    LinearMipmapNearest,
    /// Select the two mimap levels
    ///  whose texel size most closely matches the current pixel, use
    ///  the `PipelineFilter::Nearest` criterion on each one and take
    ///  their weighted average
    NearestMipmapLinear,
    /// Select the two mimap levels
    ///  whose texel size most closely matches the current pixel, use
    ///  the `PipelineFilter::Linear` criterion on each one and take
    ///  their weighted average
    LinearMipmapLinear,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for PipelineFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PipelineFilter::{}",
            match *self {
                PipelineFilter::Nearest => "Nearest",
                PipelineFilter::Linear => "Linear",
                PipelineFilter::NearestMipmapNearest => "NearestMipmapNearest",
                PipelineFilter::LinearMipmapNearest => "LinearMipmapNearest",
                PipelineFilter::NearestMipmapLinear => "NearestMipmapLinear",
                PipelineFilter::LinearMipmapLinear => "LinearMipmapLinear",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for PipelineFilter {
    type GlibType = ffi::CoglPipelineFilter;

    fn to_glib(&self) -> ffi::CoglPipelineFilter {
        match *self {
            PipelineFilter::Nearest => ffi::COGL_PIPELINE_FILTER_NEAREST,
            PipelineFilter::Linear => ffi::COGL_PIPELINE_FILTER_LINEAR,
            PipelineFilter::NearestMipmapNearest => {
                ffi::COGL_PIPELINE_FILTER_NEAREST_MIPMAP_NEAREST
            }
            PipelineFilter::LinearMipmapNearest => ffi::COGL_PIPELINE_FILTER_LINEAR_MIPMAP_NEAREST,
            PipelineFilter::NearestMipmapLinear => ffi::COGL_PIPELINE_FILTER_NEAREST_MIPMAP_LINEAR,
            PipelineFilter::LinearMipmapLinear => ffi::COGL_PIPELINE_FILTER_LINEAR_MIPMAP_LINEAR,
            PipelineFilter::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglPipelineFilter> for PipelineFilter {
    fn from_glib(value: ffi::CoglPipelineFilter) -> Self {
        match value {
            9728 => PipelineFilter::Nearest,
            9729 => PipelineFilter::Linear,
            9984 => PipelineFilter::NearestMipmapNearest,
            9985 => PipelineFilter::LinearMipmapNearest,
            9986 => PipelineFilter::NearestMipmapLinear,
            9987 => PipelineFilter::LinearMipmapLinear,
            value => PipelineFilter::__Unknown(value),
        }
    }
}

/// The wrap mode specifies what happens when texture coordinates
/// outside the range 0→1 are used. Note that if the filter mode is
/// anything but `PipelineFilter::Nearest` then texels outside the
/// range 0→1 might be used even when the coordinate is exactly 0 or 1
/// because OpenGL will try to sample neighbouring pixels. For example
/// if you are trying to render the full texture then you may get
/// artifacts around the edges when the pixels from the other side are
/// merged in if the wrap mode is set to repeat.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum PipelineWrapMode {
    /// The texture will be repeated. This
    ///  is useful for example to draw a tiled background.
    Repeat,
    MirroredRepeat,
    /// The coordinates outside the
    ///  range 0→1 will sample copies of the edge pixels of the
    ///  texture. This is useful to avoid artifacts if only one copy of
    ///  the texture is being rendered.
    ClampToEdge,
    /// Cogl will try to automatically
    ///  decide which of the above two to use. For `rectangle`, it
    ///  will use repeat mode if any of the texture coordinates are
    ///  outside the range 0→1, otherwise it will use clamp to edge. For
    ///  `polygon` it will always use repeat mode. For
    ///  `vertex_buffer_draw` it will use repeat mode except for
    ///  layers that have point sprite coordinate generation enabled. This
    ///  is the default value.
    ///
    Automatic,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for PipelineWrapMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PipelineWrapMode::{}",
            match *self {
                PipelineWrapMode::Repeat => "Repeat",
                PipelineWrapMode::MirroredRepeat => "MirroredRepeat",
                PipelineWrapMode::ClampToEdge => "ClampToEdge",
                PipelineWrapMode::Automatic => "Automatic",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for PipelineWrapMode {
    type GlibType = ffi::CoglPipelineWrapMode;

    fn to_glib(&self) -> ffi::CoglPipelineWrapMode {
        match *self {
            PipelineWrapMode::Repeat => ffi::COGL_PIPELINE_WRAP_MODE_REPEAT,
            PipelineWrapMode::MirroredRepeat => ffi::COGL_PIPELINE_WRAP_MODE_MIRRORED_REPEAT,
            PipelineWrapMode::ClampToEdge => ffi::COGL_PIPELINE_WRAP_MODE_CLAMP_TO_EDGE,
            PipelineWrapMode::Automatic => ffi::COGL_PIPELINE_WRAP_MODE_AUTOMATIC,
            PipelineWrapMode::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglPipelineWrapMode> for PipelineWrapMode {
    fn from_glib(value: ffi::CoglPipelineWrapMode) -> Self {
        match value {
            10497 => PipelineWrapMode::Repeat,
            33648 => PipelineWrapMode::MirroredRepeat,
            33071 => PipelineWrapMode::ClampToEdge,
            519 => PipelineWrapMode::Automatic,
            value => PipelineWrapMode::__Unknown(value),
        }
    }
}

/// Pixel formats used by Cogl. For the formats with a byte per
/// component, the order of the components specify the order in
/// increasing memory addresses. So for example
/// `PixelFormat::Rgb888` would have the red component in the
/// lowest address, green in the next address and blue after that
/// regardless of the endianness of the system.
///
/// For the formats with non byte aligned components the component
/// order specifies the order within a 16-bit or 32-bit number from
/// most significant bit to least significant. So for
/// `PixelFormat::Rgb565`, the red component would be in bits
/// 11-15, the green component would be in 6-11 and the blue component
/// would be in 1-5. Therefore the order in memory depends on the
/// endianness of the system.
///
/// When uploading a texture `PixelFormat::Any` can be used as the
/// internal format. Cogl will try to pick the best format to use
/// internally and convert the texture data if necessary.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum PixelFormat {
    /// Any format
    Any,
    /// 8 bits alpha mask
    A8,
    /// RGB, 16 bits
    Rgb565,
    /// RGBA, 16 bits
    Rgba4444,
    /// RGBA, 16 bits
    Rgba5551,
    /// Not currently supported
    Yuv,
    /// Single luminance component
    G8,
    /// RG, 16 bits. Note that red-green textures
    ///  are only available if `FeatureID::OglFeatureIdTextureRg` is advertised.
    ///  See `Texture::set_components` for details.
    Rg88,
    /// RGB, 24 bits
    Rgb888,
    /// BGR, 24 bits
    Bgr888,
    /// RGBA, 32 bits
    Rgba8888,
    /// BGRA, 32 bits
    Bgra8888,
    /// ARGB, 32 bits
    Argb8888,
    /// ABGR, 32 bits
    Abgr8888,
    /// RGBA, 32 bits, 10 bpc
    Rgba1010102,
    /// BGRA, 32 bits, 10 bpc
    Bgra1010102,
    /// ARGB, 32 bits, 10 bpc
    Argb2101010,
    /// ABGR, 32 bits, 10 bpc
    Abgr2101010,
    /// Premultiplied RGBA, 32 bits
    Rgba8888Pre,
    /// Premultiplied BGRA, 32 bits
    Bgra8888Pre,
    /// Premultiplied ARGB, 32 bits
    Argb8888Pre,
    /// Premultiplied ABGR, 32 bits
    Abgr8888Pre,
    /// Premultiplied RGBA, 16 bits
    Rgba4444Pre,
    /// Premultiplied RGBA, 16 bits
    Rgba5551Pre,
    /// Premultiplied RGBA, 32 bits, 10 bpc
    Rgba1010102Pre,
    /// Premultiplied BGRA, 32 bits, 10 bpc
    Bgra1010102Pre,
    /// Premultiplied ARGB, 32 bits, 10 bpc
    Argb2101010Pre,
    /// Premultiplied ABGR, 32 bits, 10 bpc
    Abgr2101010Pre,
    Depth16,
    Depth32,
    Depth24Stencil8,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for PixelFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PixelFormat::{}",
            match *self {
                PixelFormat::Any => "Any",
                PixelFormat::A8 => "A8",
                PixelFormat::Rgb565 => "Rgb565",
                PixelFormat::Rgba4444 => "Rgba4444",
                PixelFormat::Rgba5551 => "Rgba5551",
                PixelFormat::Yuv => "Yuv",
                PixelFormat::G8 => "G8",
                PixelFormat::Rg88 => "Rg88",
                PixelFormat::Rgb888 => "Rgb888",
                PixelFormat::Bgr888 => "Bgr888",
                PixelFormat::Rgba8888 => "Rgba8888",
                PixelFormat::Bgra8888 => "Bgra8888",
                PixelFormat::Argb8888 => "Argb8888",
                PixelFormat::Abgr8888 => "Abgr8888",
                PixelFormat::Rgba1010102 => "Rgba1010102",
                PixelFormat::Bgra1010102 => "Bgra1010102",
                PixelFormat::Argb2101010 => "Argb2101010",
                PixelFormat::Abgr2101010 => "Abgr2101010",
                PixelFormat::Rgba8888Pre => "Rgba8888Pre",
                PixelFormat::Bgra8888Pre => "Bgra8888Pre",
                PixelFormat::Argb8888Pre => "Argb8888Pre",
                PixelFormat::Abgr8888Pre => "Abgr8888Pre",
                PixelFormat::Rgba4444Pre => "Rgba4444Pre",
                PixelFormat::Rgba5551Pre => "Rgba5551Pre",
                PixelFormat::Rgba1010102Pre => "Rgba1010102Pre",
                PixelFormat::Bgra1010102Pre => "Bgra1010102Pre",
                PixelFormat::Argb2101010Pre => "Argb2101010Pre",
                PixelFormat::Abgr2101010Pre => "Abgr2101010Pre",
                PixelFormat::Depth16 => "Depth16",
                PixelFormat::Depth32 => "Depth32",
                PixelFormat::Depth24Stencil8 => "Depth24Stencil8",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for PixelFormat {
    type GlibType = ffi::CoglPixelFormat;

    fn to_glib(&self) -> ffi::CoglPixelFormat {
        match *self {
            PixelFormat::Any => ffi::COGL_PIXEL_FORMAT_ANY,
            PixelFormat::A8 => ffi::COGL_PIXEL_FORMAT_A_8,
            PixelFormat::Rgb565 => ffi::COGL_PIXEL_FORMAT_RGB_565,
            PixelFormat::Rgba4444 => ffi::COGL_PIXEL_FORMAT_RGBA_4444,
            PixelFormat::Rgba5551 => ffi::COGL_PIXEL_FORMAT_RGBA_5551,
            PixelFormat::Yuv => ffi::COGL_PIXEL_FORMAT_YUV,
            PixelFormat::G8 => ffi::COGL_PIXEL_FORMAT_G_8,
            PixelFormat::Rg88 => ffi::COGL_PIXEL_FORMAT_RG_88,
            PixelFormat::Rgb888 => ffi::COGL_PIXEL_FORMAT_RGB_888,
            PixelFormat::Bgr888 => ffi::COGL_PIXEL_FORMAT_BGR_888,
            PixelFormat::Rgba8888 => ffi::COGL_PIXEL_FORMAT_RGBA_8888,
            PixelFormat::Bgra8888 => ffi::COGL_PIXEL_FORMAT_BGRA_8888,
            PixelFormat::Argb8888 => ffi::COGL_PIXEL_FORMAT_ARGB_8888,
            PixelFormat::Abgr8888 => ffi::COGL_PIXEL_FORMAT_ABGR_8888,
            PixelFormat::Rgba1010102 => ffi::COGL_PIXEL_FORMAT_RGBA_1010102,
            PixelFormat::Bgra1010102 => ffi::COGL_PIXEL_FORMAT_BGRA_1010102,
            PixelFormat::Argb2101010 => ffi::COGL_PIXEL_FORMAT_ARGB_2101010,
            PixelFormat::Abgr2101010 => ffi::COGL_PIXEL_FORMAT_ABGR_2101010,
            PixelFormat::Rgba8888Pre => ffi::COGL_PIXEL_FORMAT_RGBA_8888_PRE,
            PixelFormat::Bgra8888Pre => ffi::COGL_PIXEL_FORMAT_BGRA_8888_PRE,
            PixelFormat::Argb8888Pre => ffi::COGL_PIXEL_FORMAT_ARGB_8888_PRE,
            PixelFormat::Abgr8888Pre => ffi::COGL_PIXEL_FORMAT_ABGR_8888_PRE,
            PixelFormat::Rgba4444Pre => ffi::COGL_PIXEL_FORMAT_RGBA_4444_PRE,
            PixelFormat::Rgba5551Pre => ffi::COGL_PIXEL_FORMAT_RGBA_5551_PRE,
            PixelFormat::Rgba1010102Pre => ffi::COGL_PIXEL_FORMAT_RGBA_1010102_PRE,
            PixelFormat::Bgra1010102Pre => ffi::COGL_PIXEL_FORMAT_BGRA_1010102_PRE,
            PixelFormat::Argb2101010Pre => ffi::COGL_PIXEL_FORMAT_ARGB_2101010_PRE,
            PixelFormat::Abgr2101010Pre => ffi::COGL_PIXEL_FORMAT_ABGR_2101010_PRE,
            PixelFormat::Depth16 => ffi::COGL_PIXEL_FORMAT_DEPTH_16,
            PixelFormat::Depth32 => ffi::COGL_PIXEL_FORMAT_DEPTH_32,
            PixelFormat::Depth24Stencil8 => ffi::COGL_PIXEL_FORMAT_DEPTH_24_STENCIL_8,
            PixelFormat::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglPixelFormat> for PixelFormat {
    fn from_glib(value: ffi::CoglPixelFormat) -> Self {
        match value {
            0 => PixelFormat::Any,
            17 => PixelFormat::A8,
            4 => PixelFormat::Rgb565,
            21 => PixelFormat::Rgba4444,
            22 => PixelFormat::Rgba5551,
            7 => PixelFormat::Yuv,
            8 => PixelFormat::G8,
            9 => PixelFormat::Rg88,
            2 => PixelFormat::Rgb888,
            34 => PixelFormat::Bgr888,
            19 => PixelFormat::Rgba8888,
            51 => PixelFormat::Bgra8888,
            83 => PixelFormat::Argb8888,
            115 => PixelFormat::Abgr8888,
            29 => PixelFormat::Rgba1010102,
            61 => PixelFormat::Bgra1010102,
            93 => PixelFormat::Argb2101010,
            125 => PixelFormat::Abgr2101010,
            147 => PixelFormat::Rgba8888Pre,
            179 => PixelFormat::Bgra8888Pre,
            211 => PixelFormat::Argb8888Pre,
            243 => PixelFormat::Abgr8888Pre,
            149 => PixelFormat::Rgba4444Pre,
            150 => PixelFormat::Rgba5551Pre,
            157 => PixelFormat::Rgba1010102Pre,
            189 => PixelFormat::Bgra1010102Pre,
            221 => PixelFormat::Argb2101010Pre,
            253 => PixelFormat::Abgr2101010Pre,
            265 => PixelFormat::Depth16,
            259 => PixelFormat::Depth32,
            771 => PixelFormat::Depth24Stencil8,
            value => PixelFormat::__Unknown(value),
        }
    }
}

impl StaticType for PixelFormat {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_pixel_format_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PixelFormat {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PixelFormat {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for PixelFormat {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// A bitmask of events that Cogl may need to wake on for a file
/// descriptor. Note that these all have the same values as the
/// corresponding defines for the poll function call on Unix so they
/// may be directly passed to poll.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum PollFDEvent {
    /// there is data to read
    In,
    /// data can be written (without blocking)
    Pri,
    /// there is urgent data to read.
    Out,
    /// error condition
    Err,
    /// hung up (the connection has been broken, usually
    ///  for pipes and sockets).
    Hup,
    /// invalid request. The file descriptor is not open.
    Nval,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for PollFDEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PollFDEvent::{}",
            match *self {
                PollFDEvent::In => "In",
                PollFDEvent::Pri => "Pri",
                PollFDEvent::Out => "Out",
                PollFDEvent::Err => "Err",
                PollFDEvent::Hup => "Hup",
                PollFDEvent::Nval => "Nval",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for PollFDEvent {
    type GlibType = ffi::CoglPollFDEvent;

    fn to_glib(&self) -> ffi::CoglPollFDEvent {
        match *self {
            PollFDEvent::In => ffi::COGL_POLL_FD_EVENT_IN,
            PollFDEvent::Pri => ffi::COGL_POLL_FD_EVENT_PRI,
            PollFDEvent::Out => ffi::COGL_POLL_FD_EVENT_OUT,
            PollFDEvent::Err => ffi::COGL_POLL_FD_EVENT_ERR,
            PollFDEvent::Hup => ffi::COGL_POLL_FD_EVENT_HUP,
            PollFDEvent::Nval => ffi::COGL_POLL_FD_EVENT_NVAL,
            PollFDEvent::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglPollFDEvent> for PollFDEvent {
    fn from_glib(value: ffi::CoglPollFDEvent) -> Self {
        match value {
            1 => PollFDEvent::In,
            2 => PollFDEvent::Pri,
            4 => PollFDEvent::Out,
            8 => PollFDEvent::Err,
            16 => PollFDEvent::Hup,
            32 => PollFDEvent::Nval,
            value => PollFDEvent::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum RendererError {
    XlibDisplayOpen,
    BadConstraint,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for RendererError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RendererError::{}",
            match *self {
                RendererError::XlibDisplayOpen => "XlibDisplayOpen",
                RendererError::BadConstraint => "BadConstraint",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for RendererError {
    type GlibType = ffi::CoglRendererError;

    fn to_glib(&self) -> ffi::CoglRendererError {
        match *self {
            RendererError::XlibDisplayOpen => ffi::COGL_RENDERER_ERROR_XLIB_DISPLAY_OPEN,
            RendererError::BadConstraint => ffi::COGL_RENDERER_ERROR_BAD_CONSTRAINT,
            RendererError::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglRendererError> for RendererError {
    fn from_glib(value: ffi::CoglRendererError) -> Self {
        match value {
            0 => RendererError::XlibDisplayOpen,
            1 => RendererError::BadConstraint,
            value => RendererError::__Unknown(value),
        }
    }
}

impl ErrorDomain for RendererError {
    fn domain() -> Quark {
        unsafe { from_glib(ffi::cogl_renderer_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            0 => Some(RendererError::XlibDisplayOpen),
            1 => Some(RendererError::BadConstraint),
            value => Some(RendererError::__Unknown(value)),
        }
    }
}

impl StaticType for RendererError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_renderer_error_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for RendererError {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for RendererError {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for RendererError {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Types of shaders
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum ShaderType {
    /// A program for proccessing vertices
    Vertex,
    /// A program for processing fragments
    Fragment,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ShaderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ShaderType::{}",
            match *self {
                ShaderType::Vertex => "Vertex",
                ShaderType::Fragment => "Fragment",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for ShaderType {
    type GlibType = ffi::CoglShaderType;

    fn to_glib(&self) -> ffi::CoglShaderType {
        match *self {
            ShaderType::Vertex => ffi::COGL_SHADER_TYPE_VERTEX,
            ShaderType::Fragment => ffi::COGL_SHADER_TYPE_FRAGMENT,
            ShaderType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglShaderType> for ShaderType {
    fn from_glib(value: ffi::CoglShaderType) -> Self {
        match value {
            0 => ShaderType::Vertex,
            1 => ShaderType::Fragment,
            value => ShaderType::__Unknown(value),
        }
    }
}

impl StaticType for ShaderType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_shader_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ShaderType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ShaderType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for ShaderType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// `SnippetHook` is used to specify a location within a
/// `Pipeline` where the code of the snippet should be used when it
/// is attached to a pipeline.
///
/// `<glosslist>`
///  `<glossentry>`
///  `<glossterm>``SnippetHook::VertexGlobals``</glossterm>`
///  `<glossdef>`
/// `<para>`
/// Adds a shader snippet at the beginning of the global section of the
/// shader for the vertex processing. Any declarations here can be
/// shared with all other snippets that are attached to a vertex hook.
/// Only the ‘declarations’ string is used and the other strings are
/// ignored.
/// `</para>`
///  `</glossdef>`
///  `</glossentry>`
///  `<glossentry>`
///  `<glossterm>``SnippetHook::FragmentGlobals``</glossterm>`
///  `<glossdef>`
/// `<para>`
/// Adds a shader snippet at the beginning of the global section of the
/// shader for the fragment processing. Any declarations here can be
/// shared with all other snippets that are attached to a fragment
/// hook. Only the ‘declarations’ string is used and the other strings
/// are ignored.
/// `</para>`
///  `</glossdef>`
///  `</glossentry>`
///  `<glossentry>`
///  `<glossterm>``SnippetHook::Vertex``</glossterm>`
///  `<glossdef>`
/// `<para>`
/// Adds a shader snippet that will hook on to the vertex processing
/// stage of the pipeline. This gives a chance for the application to
/// modify the vertex attributes generated by the shader. Typically the
/// snippet will modify color_out or position_out builtins.
/// `</para>`
/// `<para>`
/// The ‘declarations’ string in `snippet` will be inserted in the
/// global scope of the shader. Use this to declare any uniforms,
/// attributes or functions that the snippet requires.
/// `</para>`
/// `<para>`
/// The ‘pre’ string in `snippet` will be inserted at the top of the
/// `main` function before any vertex processing is done.
/// `</para>`
/// `<para>`
/// The ‘replace’ string in `snippet` will be used instead of the
/// generated vertex processing if it is present. This can be used if
/// the application wants to provide a complete vertex shader and
/// doesn't need the generated output from Cogl.
/// `</para>`
/// `<para>`
/// The ‘post’ string in `snippet` will be inserted after all of the
/// standard vertex processing is done. This can be used to modify the
/// outputs.
/// `</para>`
///  `</glossdef>`
///  `</glossentry>`
///  `<glossentry>`
///  `<glossterm>``SnippetHook::VertexTransform``</glossterm>`
///  `<glossdef>`
/// `<para>`
/// Adds a shader snippet that will hook on to the vertex transform stage.
/// Typically the snippet will use the modelview_matrix,
/// projection_matrix and modelview_projection_matrix matrices and the
/// position_in attribute. The hook must write to position_out.
/// The default processing for this hook will multiply position_in by
/// the combined modelview-projection matrix and store it on position_out.
/// `</para>`
/// `<para>`
/// The ‘declarations’ string in `snippet` will be inserted in the
/// global scope of the shader. Use this to declare any uniforms,
/// attributes or functions that the snippet requires.
/// `</para>`
/// `<para>`
/// The ‘pre’ string in `snippet` will be inserted at the top of the
/// `main` function before the vertex transform is done.
/// `</para>`
/// `<para>`
/// The ‘replace’ string in `snippet` will be used instead of the
/// generated vertex transform if it is present.
/// `</para>`
/// `<para>`
/// The ‘post’ string in `snippet` will be inserted after all of the
/// standard vertex transformation is done. This can be used to modify the
/// position_out in addition to the default processing.
/// `</para>`
///  `</glossdef>`
///  `</glossentry>`
///  `<glossentry>`
///  `<glossterm>``SnippetHook::PointSize``</glossterm>`
///  `<glossdef>`
/// `<para>`
/// Adds a shader snippet that will hook on to the point size
/// calculation step within the vertex shader stage. The snippet should
/// write to the builtin point_size_out with the new point size.
/// The snippet can either read point_size_in directly and write a
/// new value or first read an existing value in point_size_out
/// that would be set by a previous snippet. Note that this hook is
/// only used if `Pipeline::set_per_vertex_point_size` is enabled
/// on the pipeline.
/// `</para>`
/// `<para>`
/// The ‘declarations’ string in `snippet` will be inserted in the
/// global scope of the shader. Use this to declare any uniforms,
/// attributes or functions that the snippet requires.
/// `</para>`
/// `<para>`
/// The ‘pre’ string in `snippet` will be inserted just before
/// calculating the point size.
/// `</para>`
/// `<para>`
/// The ‘replace’ string in `snippet` will be used instead of the
/// generated point size calculation if it is present.
/// `</para>`
/// `<para>`
/// The ‘post’ string in `snippet` will be inserted after the
/// standard point size calculation is done. This can be used to modify
/// point_size_out in addition to the default processing.
/// `</para>`
///  `</glossdef>`
///  `</glossentry>`
///  `<glossentry>`
///  `<glossterm>``SnippetHook::Fragment``</glossterm>`
///  `<glossdef>`
/// `<para>`
/// Adds a shader snippet that will hook on to the fragment processing
/// stage of the pipeline. This gives a chance for the application to
/// modify the fragment color generated by the shader. Typically the
/// snippet will modify color_out.
/// `</para>`
/// `<para>`
/// The ‘declarations’ string in `snippet` will be inserted in the
/// global scope of the shader. Use this to declare any uniforms,
/// attributes or functions that the snippet requires.
/// `</para>`
/// `<para>`
/// The ‘pre’ string in `snippet` will be inserted at the top of the
/// `main` function before any fragment processing is done.
/// `</para>`
/// `<para>`
/// The ‘replace’ string in `snippet` will be used instead of the
/// generated fragment processing if it is present. This can be used if
/// the application wants to provide a complete fragment shader and
/// doesn't need the generated output from Cogl.
/// `</para>`
/// `<para>`
/// The ‘post’ string in `snippet` will be inserted after all of the
/// standard fragment processing is done. At this point the generated
/// value for the rest of the pipeline state will already be in
/// color_out so the application can modify the result by altering
/// this variable.
/// `</para>`
///  `</glossdef>`
///  `</glossentry>`
///  `<glossentry>`
///  `<glossterm>``SnippetHook::TextureCoordTransform``</glossterm>`
///  `<glossdef>`
/// `<para>`
/// Adds a shader snippet that will hook on to the texture coordinate
/// transformation of a particular layer. This can be used to replace
/// the processing for a layer or to modify the results.
/// `</para>`
/// `<para>`
/// Within the snippet code for this hook there are two extra
/// variables. The first is a mat4 called matrix which represents
/// the user matrix for this layer. The second is called tex_coord
/// and represents the incoming and outgoing texture coordinate. On
/// entry to the hook, tex_coord contains the value of the
/// corresponding texture coordinate attribute for this layer. The hook
/// is expected to modify this variable. The output will be passed as a
/// varying to the fragment processing stage. The default code will
/// just multiply matrix by tex_coord and store the result in
/// tex_coord.
/// `</para>`
/// `<para>`
/// The ‘declarations’ string in `snippet` will be inserted in the
/// global scope of the shader. Use this to declare any uniforms,
/// attributes or functions that the snippet requires.
/// `</para>`
/// `<para>`
/// The ‘pre’ string in `snippet` will be inserted just before the
/// fragment processing for this layer. At this point tex_coord
/// still contains the value of the texture coordinate attribute.
/// `</para>`
/// `<para>`
/// If a ‘replace’ string is given then this will be used instead of
/// the default fragment processing for this layer. The snippet can
/// modify tex_coord or leave it as is to apply no transformation.
/// `</para>`
/// `<para>`
/// The ‘post’ string in `snippet` will be inserted just after the
/// transformation. At this point tex_coord will contain the
/// results of the transformation but it can be further modified by the
/// snippet.
/// `</para>`
///  `</glossdef>`
///  `</glossentry>`
///  `<glossentry>`
///  `<glossterm>``SnippetHook::LayerFragment``</glossterm>`
///  `<glossdef>`
/// `<para>`
/// Adds a shader snippet that will hook on to the fragment processing
/// of a particular layer. This can be used to replace the processing
/// for a layer or to modify the results.
/// `</para>`
/// `<para>`
/// Within the snippet code for this hook there is an extra vec4
/// variable called ‘layer’. This contains the resulting color
/// that will be used for the layer. This can be modified in the ‘post’
/// section or it the default processing can be replaced entirely using
/// the ‘replace’ section.
/// `</para>`
/// `<para>`
/// The ‘declarations’ string in `snippet` will be inserted in the
/// global scope of the shader. Use this to declare any uniforms,
/// attributes or functions that the snippet requires.
/// `</para>`
/// `<para>`
/// The ‘pre’ string in `snippet` will be inserted just before the
/// fragment processing for this layer.
/// `</para>`
/// `<para>`
/// If a ‘replace’ string is given then this will be used instead of
/// the default fragment processing for this layer. The snippet must write to
/// the ‘layer’ variable in that case.
/// `</para>`
/// `<para>`
/// The ‘post’ string in `snippet` will be inserted just after the
/// fragment processing for the layer. The results can be modified by changing
/// the value of the ‘layer’ variable.
/// `</para>`
///  `</glossdef>`
///  `</glossentry>`
///  `<glossentry>`
///  `<glossterm>``SnippetHook::TextureLookup``</glossterm>`
///  `<glossdef>`
/// `<para>`
/// Adds a shader snippet that will hook on to the texture lookup part
/// of a given layer. This gives a chance for the application to modify
/// the coordinates that will be used for the texture lookup or to
/// alter the returned texel.
/// `</para>`
/// `<para>`
/// Within the snippet code for this hook there are three extra
/// variables available. ‘sampler’ is a sampler object
/// representing the sampler for the layer where the snippet is
/// attached. ‘tex_coord’ is a vec4 which contains the texture
/// coordinates that will be used for the texture lookup. This can be
/// modified. ‘texel’ will contain the result of the texture
/// lookup. This can also be modified.
/// `</para>`
/// `<para>`
/// The ‘declarations’ string in `snippet` will be inserted in the
/// global scope of the shader. Use this to declare any uniforms,
/// attributes or functions that the snippet requires.
/// `</para>`
/// `<para>`
/// The ‘pre’ string in `snippet` will be inserted at the top of the
/// `main` function before any fragment processing is done. This is a
/// good place to modify the tex_coord variable.
/// `</para>`
/// `<para>`
/// If a ‘replace’ string is given then this will be used instead of a
/// the default texture lookup. The snippet would typically use its own
/// sampler in this case.
/// `</para>`
/// `<para>`
/// The ‘post’ string in `snippet` will be inserted after texture lookup
/// has been preformed. Here the snippet can modify the texel
/// variable to alter the returned texel.
/// `</para>`
///  `</glossdef>`
///  `</glossentry>`
/// `</glosslist>`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum SnippetHook {
    /// A hook for the entire vertex processing
    ///  stage of the pipeline.
    Vertex,
    /// A hook for the vertex transformation.
    VertexTransform,
    /// A hook for declaring global data
    ///  that can be shared with all other snippets that are on a vertex
    ///  hook.
    VertexGlobals,
    /// A hook for manipulating the point
    ///  size of a vertex. This is only used if
    ///  `Pipeline::set_per_vertex_point_size` is enabled on the
    ///  pipeline.
    PointSize,
    /// A hook for the entire fragment
    ///  processing stage of the pipeline.
    Fragment,
    /// A hook for declaring global
    ///  data wthat can be shared with all other snippets that are on a
    ///  fragment hook.
    FragmentGlobals,
    /// A hook for applying the
    ///  layer matrix to a texture coordinate for a layer.
    TextureCoordTransform,
    /// A hook for the fragment
    ///  processing of a particular layer.
    LayerFragment,
    /// A hook for the texture lookup
    ///  stage of a given layer in a pipeline.
    TextureLookup,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for SnippetHook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SnippetHook::{}",
            match *self {
                SnippetHook::Vertex => "Vertex",
                SnippetHook::VertexTransform => "VertexTransform",
                SnippetHook::VertexGlobals => "VertexGlobals",
                SnippetHook::PointSize => "PointSize",
                SnippetHook::Fragment => "Fragment",
                SnippetHook::FragmentGlobals => "FragmentGlobals",
                SnippetHook::TextureCoordTransform => "TextureCoordTransform",
                SnippetHook::LayerFragment => "LayerFragment",
                SnippetHook::TextureLookup => "TextureLookup",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for SnippetHook {
    type GlibType = ffi::CoglSnippetHook;

    fn to_glib(&self) -> ffi::CoglSnippetHook {
        match *self {
            SnippetHook::Vertex => ffi::COGL_SNIPPET_HOOK_VERTEX,
            SnippetHook::VertexTransform => ffi::COGL_SNIPPET_HOOK_VERTEX_TRANSFORM,
            SnippetHook::VertexGlobals => ffi::COGL_SNIPPET_HOOK_VERTEX_GLOBALS,
            SnippetHook::PointSize => ffi::COGL_SNIPPET_HOOK_POINT_SIZE,
            SnippetHook::Fragment => ffi::COGL_SNIPPET_HOOK_FRAGMENT,
            SnippetHook::FragmentGlobals => ffi::COGL_SNIPPET_HOOK_FRAGMENT_GLOBALS,
            SnippetHook::TextureCoordTransform => ffi::COGL_SNIPPET_HOOK_TEXTURE_COORD_TRANSFORM,
            SnippetHook::LayerFragment => ffi::COGL_SNIPPET_HOOK_LAYER_FRAGMENT,
            SnippetHook::TextureLookup => ffi::COGL_SNIPPET_HOOK_TEXTURE_LOOKUP,
            SnippetHook::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglSnippetHook> for SnippetHook {
    fn from_glib(value: ffi::CoglSnippetHook) -> Self {
        match value {
            0 => SnippetHook::Vertex,
            1 => SnippetHook::VertexTransform,
            2 => SnippetHook::VertexGlobals,
            3 => SnippetHook::PointSize,
            2048 => SnippetHook::Fragment,
            2049 => SnippetHook::FragmentGlobals,
            4096 => SnippetHook::TextureCoordTransform,
            6144 => SnippetHook::LayerFragment,
            6145 => SnippetHook::TextureLookup,
            value => SnippetHook::__Unknown(value),
        }
    }
}

/// Represents how draw should affect the two buffers
/// of a stereo framebuffer. See `Framebuffer::set_stereo_mode`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum StereoMode {
    /// draw to both stereo buffers
    Both,
    /// draw only to the left stereo buffer
    Left,
    /// draw only to the left stereo buffer
    Right,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for StereoMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "StereoMode::{}",
            match *self {
                StereoMode::Both => "Both",
                StereoMode::Left => "Left",
                StereoMode::Right => "Right",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for StereoMode {
    type GlibType = ffi::CoglStereoMode;

    fn to_glib(&self) -> ffi::CoglStereoMode {
        match *self {
            StereoMode::Both => ffi::COGL_STEREO_BOTH,
            StereoMode::Left => ffi::COGL_STEREO_LEFT,
            StereoMode::Right => ffi::COGL_STEREO_RIGHT,
            StereoMode::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglStereoMode> for StereoMode {
    fn from_glib(value: ffi::CoglStereoMode) -> Self {
        match value {
            0 => StereoMode::Both,
            1 => StereoMode::Left,
            2 => StereoMode::Right,
            value => StereoMode::__Unknown(value),
        }
    }
}

impl StaticType for StereoMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_stereo_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for StereoMode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for StereoMode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for StereoMode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Some output devices (such as LCD panels) display colors
/// by making each pixel consist of smaller "subpixels"
/// that each have a particular color. By using knowledge
/// of the layout of this subpixel components, it is possible
/// to create image content with higher resolution than the
/// pixel grid.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum SubpixelOrder {
    /// the layout of subpixel
    ///  components for the device is unknown.
    Unknown,
    /// the device displays colors
    ///  without geometrically-separated subpixel components,
    ///  or the positioning or colors of the components do not
    ///  match any of the values in the enumeration.
    None,
    /// the device has
    ///  horizontally arranged components in the order
    ///  red-green-blue from left to right.
    HorizontalRgb,
    /// the device has
    ///  horizontally arranged components in the order
    ///  blue-green-red from left to right.
    HorizontalBgr,
    /// the device has
    ///  vertically arranged components in the order
    ///  red-green-blue from top to bottom.
    VerticalRgb,
    /// the device has
    ///  vertically arranged components in the order
    ///  blue-green-red from top to bottom.
    VerticalBgr,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for SubpixelOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SubpixelOrder::{}",
            match *self {
                SubpixelOrder::Unknown => "Unknown",
                SubpixelOrder::None => "None",
                SubpixelOrder::HorizontalRgb => "HorizontalRgb",
                SubpixelOrder::HorizontalBgr => "HorizontalBgr",
                SubpixelOrder::VerticalRgb => "VerticalRgb",
                SubpixelOrder::VerticalBgr => "VerticalBgr",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for SubpixelOrder {
    type GlibType = ffi::CoglSubpixelOrder;

    fn to_glib(&self) -> ffi::CoglSubpixelOrder {
        match *self {
            SubpixelOrder::Unknown => ffi::COGL_SUBPIXEL_ORDER_UNKNOWN,
            SubpixelOrder::None => ffi::COGL_SUBPIXEL_ORDER_NONE,
            SubpixelOrder::HorizontalRgb => ffi::COGL_SUBPIXEL_ORDER_HORIZONTAL_RGB,
            SubpixelOrder::HorizontalBgr => ffi::COGL_SUBPIXEL_ORDER_HORIZONTAL_BGR,
            SubpixelOrder::VerticalRgb => ffi::COGL_SUBPIXEL_ORDER_VERTICAL_RGB,
            SubpixelOrder::VerticalBgr => ffi::COGL_SUBPIXEL_ORDER_VERTICAL_BGR,
            SubpixelOrder::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglSubpixelOrder> for SubpixelOrder {
    fn from_glib(value: ffi::CoglSubpixelOrder) -> Self {
        match value {
            0 => SubpixelOrder::Unknown,
            1 => SubpixelOrder::None,
            2 => SubpixelOrder::HorizontalRgb,
            3 => SubpixelOrder::HorizontalBgr,
            4 => SubpixelOrder::VerticalRgb,
            5 => SubpixelOrder::VerticalBgr,
            value => SubpixelOrder::__Unknown(value),
        }
    }
}

/// Error enumeration for Cogl
///
/// The `SystemError::CoglSystemErrorUnsupported` error can be thrown for a
/// variety of reasons. For example:
///
/// `<itemizedlist>`
///  `<listitem>``<para>`You've tried to use a feature that is not
///  advertised by `has_feature`. This could happen if you create
///  a 2d texture with a non-power-of-two size when
///  `FeatureID::OglFeatureIdTextureNpot` is not advertised.`</para>``</listitem>`
///  `<listitem>``<para>`The GPU can not handle the configuration you have
///  requested. An example might be if you try to use too many texture
///  layers in a single `Pipeline``</para>``</listitem>`
///  `<listitem>``<para>`The driver does not support some
///  configuration.`</para>``</listiem>`
/// `</itemizedlist>`
///
/// Currently this is only used by Cogl API marked as experimental so
/// this enum should also be considered experimental.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum SystemError {
    /// You tried to use a feature or
    ///  configuration not currently available.
    CoglSystemErrorUnsupported,
    /// You tried to allocate a resource
    ///  such as a texture and there wasn't enough memory.
    CoglSystemErrorNoMemory,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SystemError::{}",
            match *self {
                SystemError::CoglSystemErrorUnsupported => "CoglSystemErrorUnsupported",
                SystemError::CoglSystemErrorNoMemory => "CoglSystemErrorNoMemory",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for SystemError {
    type GlibType = ffi::CoglSystemError;

    fn to_glib(&self) -> ffi::CoglSystemError {
        match *self {
            SystemError::CoglSystemErrorUnsupported => ffi::COGL_SYSTEM_ERROR_UNSUPPORTED,
            SystemError::CoglSystemErrorNoMemory => ffi::COGL_SYSTEM_ERROR_NO_MEMORY,
            SystemError::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglSystemError> for SystemError {
    fn from_glib(value: ffi::CoglSystemError) -> Self {
        match value {
            0 => SystemError::CoglSystemErrorUnsupported,
            1 => SystemError::CoglSystemErrorNoMemory,
            value => SystemError::__Unknown(value),
        }
    }
}

impl StaticType for SystemError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_system_error_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SystemError {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SystemError {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for SystemError {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// See `Texture::set_components`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum TextureComponents {
    /// Only the alpha component
    A,
    /// Red and green components. Note that
    ///  this can only be used if the `FeatureID::OglFeatureIdTextureRg` feature
    ///  is advertised.
    Rg,
    /// Red, green and blue components
    Rgb,
    /// Red, green, blue and alpha components
    Rgba,
    /// Only a depth component
    Depth,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for TextureComponents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TextureComponents::{}",
            match *self {
                TextureComponents::A => "A",
                TextureComponents::Rg => "Rg",
                TextureComponents::Rgb => "Rgb",
                TextureComponents::Rgba => "Rgba",
                TextureComponents::Depth => "Depth",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for TextureComponents {
    type GlibType = ffi::CoglTextureComponents;

    fn to_glib(&self) -> ffi::CoglTextureComponents {
        match *self {
            TextureComponents::A => ffi::COGL_TEXTURE_COMPONENTS_A,
            TextureComponents::Rg => ffi::COGL_TEXTURE_COMPONENTS_RG,
            TextureComponents::Rgb => ffi::COGL_TEXTURE_COMPONENTS_RGB,
            TextureComponents::Rgba => ffi::COGL_TEXTURE_COMPONENTS_RGBA,
            TextureComponents::Depth => ffi::COGL_TEXTURE_COMPONENTS_DEPTH,
            TextureComponents::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglTextureComponents> for TextureComponents {
    fn from_glib(value: ffi::CoglTextureComponents) -> Self {
        match value {
            1 => TextureComponents::A,
            2 => TextureComponents::Rg,
            3 => TextureComponents::Rgb,
            4 => TextureComponents::Rgba,
            5 => TextureComponents::Depth,
            value => TextureComponents::__Unknown(value),
        }
    }
}

impl StaticType for TextureComponents {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_texture_components_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for TextureComponents {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for TextureComponents {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for TextureComponents {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Error codes that can be thrown when allocating textures.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum TextureError {
    /// Unsupported size
    Size,
    /// Unsupported format
    Format,
    BadParameter,
    /// A primitive texture type that is
    ///  unsupported by the driver was used
    Type,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for TextureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TextureError::{}",
            match *self {
                TextureError::Size => "Size",
                TextureError::Format => "Format",
                TextureError::BadParameter => "BadParameter",
                TextureError::Type => "Type",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for TextureError {
    type GlibType = ffi::CoglTextureError;

    fn to_glib(&self) -> ffi::CoglTextureError {
        match *self {
            TextureError::Size => ffi::COGL_TEXTURE_ERROR_SIZE,
            TextureError::Format => ffi::COGL_TEXTURE_ERROR_FORMAT,
            TextureError::BadParameter => ffi::COGL_TEXTURE_ERROR_BAD_PARAMETER,
            TextureError::Type => ffi::COGL_TEXTURE_ERROR_TYPE,
            TextureError::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglTextureError> for TextureError {
    fn from_glib(value: ffi::CoglTextureError) -> Self {
        match value {
            0 => TextureError::Size,
            1 => TextureError::Format,
            2 => TextureError::BadParameter,
            3 => TextureError::Type,
            value => TextureError::__Unknown(value),
        }
    }
}

impl ErrorDomain for TextureError {
    fn domain() -> Quark {
        unsafe { from_glib(ffi::cogl_texture_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            0 => Some(TextureError::Size),
            1 => Some(TextureError::Format),
            2 => Some(TextureError::BadParameter),
            3 => Some(TextureError::Type),
            value => Some(TextureError::__Unknown(value)),
        }
    }
}

impl StaticType for TextureError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_texture_error_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for TextureError {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for TextureError {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for TextureError {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Error codes that can be thrown when performing texture-pixmap-x11
/// operations.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum TexturePixmapX11Error {
    /// An X11 protocol error
    TexturePixmapX11ErrorX11,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for TexturePixmapX11Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TexturePixmapX11Error::{}",
            match *self {
                TexturePixmapX11Error::TexturePixmapX11ErrorX11 => "TexturePixmapX11ErrorX11",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for TexturePixmapX11Error {
    type GlibType = ffi::CoglTexturePixmapX11Error;

    fn to_glib(&self) -> ffi::CoglTexturePixmapX11Error {
        match *self {
            TexturePixmapX11Error::TexturePixmapX11ErrorX11 => {
                ffi::COGL_TEXTURE_PIXMAP_X11_ERROR_X11
            }
            TexturePixmapX11Error::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglTexturePixmapX11Error> for TexturePixmapX11Error {
    fn from_glib(value: ffi::CoglTexturePixmapX11Error) -> Self {
        match value {
            0 => TexturePixmapX11Error::TexturePixmapX11ErrorX11,
            value => TexturePixmapX11Error::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum TexturePixmapX11ReportLevel {
    RawRectangles,
    DeltaRectangles,
    BoundingBox,
    NonEmpty,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for TexturePixmapX11ReportLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TexturePixmapX11ReportLevel::{}",
            match *self {
                TexturePixmapX11ReportLevel::RawRectangles => "RawRectangles",
                TexturePixmapX11ReportLevel::DeltaRectangles => "DeltaRectangles",
                TexturePixmapX11ReportLevel::BoundingBox => "BoundingBox",
                TexturePixmapX11ReportLevel::NonEmpty => "NonEmpty",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for TexturePixmapX11ReportLevel {
    type GlibType = ffi::CoglTexturePixmapX11ReportLevel;

    fn to_glib(&self) -> ffi::CoglTexturePixmapX11ReportLevel {
        match *self {
            TexturePixmapX11ReportLevel::RawRectangles => {
                ffi::COGL_TEXTURE_PIXMAP_X11_DAMAGE_RAW_RECTANGLES
            }
            TexturePixmapX11ReportLevel::DeltaRectangles => {
                ffi::COGL_TEXTURE_PIXMAP_X11_DAMAGE_DELTA_RECTANGLES
            }
            TexturePixmapX11ReportLevel::BoundingBox => {
                ffi::COGL_TEXTURE_PIXMAP_X11_DAMAGE_BOUNDING_BOX
            }
            TexturePixmapX11ReportLevel::NonEmpty => ffi::COGL_TEXTURE_PIXMAP_X11_DAMAGE_NON_EMPTY,
            TexturePixmapX11ReportLevel::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglTexturePixmapX11ReportLevel> for TexturePixmapX11ReportLevel {
    fn from_glib(value: ffi::CoglTexturePixmapX11ReportLevel) -> Self {
        match value {
            0 => TexturePixmapX11ReportLevel::RawRectangles,
            1 => TexturePixmapX11ReportLevel::DeltaRectangles,
            2 => TexturePixmapX11ReportLevel::BoundingBox,
            3 => TexturePixmapX11ReportLevel::NonEmpty,
            value => TexturePixmapX11ReportLevel::__Unknown(value),
        }
    }
}

/// Constants representing the underlying hardware texture type of a
/// `Texture`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum TextureType {
    _2d,
    _3d,
    /// A `TextureRectangle`
    Rectangle,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for TextureType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TextureType::{}",
            match *self {
                TextureType::_2d => "_2d",
                TextureType::_3d => "_3d",
                TextureType::Rectangle => "Rectangle",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for TextureType {
    type GlibType = ffi::CoglTextureType;

    fn to_glib(&self) -> ffi::CoglTextureType {
        match *self {
            TextureType::_2d => ffi::COGL_TEXTURE_TYPE_2D,
            TextureType::_3d => ffi::COGL_TEXTURE_TYPE_3D,
            TextureType::Rectangle => ffi::COGL_TEXTURE_TYPE_RECTANGLE,
            TextureType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglTextureType> for TextureType {
    fn from_glib(value: ffi::CoglTextureType) -> Self {
        match value {
            0 => TextureType::_2d,
            1 => TextureType::_3d,
            2 => TextureType::Rectangle,
            value => TextureType::__Unknown(value),
        }
    }
}

impl StaticType for TextureType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_texture_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for TextureType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for TextureType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for TextureType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Different ways of interpreting vertices when drawing.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum VerticesMode {
    /// FIXME, equivalent to
    /// `<constant>`GL_POINTS`</constant>`
    Points,
    /// FIXME, equivalent to `<constant>`GL_LINES`</constant>`
    Lines,
    /// FIXME, equivalent to
    /// `<constant>`GL_LINE_LOOP`</constant>`
    LineLoop,
    /// FIXME, equivalent to
    /// `<constant>`GL_LINE_STRIP`</constant>`
    LineStrip,
    /// FIXME, equivalent to
    /// `<constant>`GL_TRIANGLES`</constant>`
    Triangles,
    /// FIXME, equivalent to
    /// `<constant>`GL_TRIANGLE_STRIP`</constant>`
    TriangleStrip,
    /// FIXME, equivalent to `<constant>`GL_TRIANGLE_FAN`</constant>`
    TriangleFan,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for VerticesMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "VerticesMode::{}",
            match *self {
                VerticesMode::Points => "Points",
                VerticesMode::Lines => "Lines",
                VerticesMode::LineLoop => "LineLoop",
                VerticesMode::LineStrip => "LineStrip",
                VerticesMode::Triangles => "Triangles",
                VerticesMode::TriangleStrip => "TriangleStrip",
                VerticesMode::TriangleFan => "TriangleFan",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for VerticesMode {
    type GlibType = ffi::CoglVerticesMode;

    fn to_glib(&self) -> ffi::CoglVerticesMode {
        match *self {
            VerticesMode::Points => ffi::COGL_VERTICES_MODE_POINTS,
            VerticesMode::Lines => ffi::COGL_VERTICES_MODE_LINES,
            VerticesMode::LineLoop => ffi::COGL_VERTICES_MODE_LINE_LOOP,
            VerticesMode::LineStrip => ffi::COGL_VERTICES_MODE_LINE_STRIP,
            VerticesMode::Triangles => ffi::COGL_VERTICES_MODE_TRIANGLES,
            VerticesMode::TriangleStrip => ffi::COGL_VERTICES_MODE_TRIANGLE_STRIP,
            VerticesMode::TriangleFan => ffi::COGL_VERTICES_MODE_TRIANGLE_FAN,
            VerticesMode::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglVerticesMode> for VerticesMode {
    fn from_glib(value: ffi::CoglVerticesMode) -> Self {
        match value {
            0 => VerticesMode::Points,
            1 => VerticesMode::Lines,
            2 => VerticesMode::LineLoop,
            3 => VerticesMode::LineStrip,
            4 => VerticesMode::Triangles,
            5 => VerticesMode::TriangleStrip,
            6 => VerticesMode::TriangleFan,
            value => VerticesMode::__Unknown(value),
        }
    }
}

impl StaticType for VerticesMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_vertices_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for VerticesMode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for VerticesMode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for VerticesMode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Enum used to represent the two directions of rotation. This can be
/// used to set the front face for culling by calling
/// `Pipeline::set_front_face_winding`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum Winding {
    /// Vertices are in a clockwise order
    Clockwise,
    /// Vertices are in a counter-clockwise order
    CounterClockwise,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for Winding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Winding::{}",
            match *self {
                Winding::Clockwise => "Clockwise",
                Winding::CounterClockwise => "CounterClockwise",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for Winding {
    type GlibType = ffi::CoglWinding;

    fn to_glib(&self) -> ffi::CoglWinding {
        match *self {
            Winding::Clockwise => ffi::COGL_WINDING_CLOCKWISE,
            Winding::CounterClockwise => ffi::COGL_WINDING_COUNTER_CLOCKWISE,
            Winding::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglWinding> for Winding {
    fn from_glib(value: ffi::CoglWinding) -> Self {
        match value {
            0 => Winding::Clockwise,
            1 => Winding::CounterClockwise,
            value => Winding::__Unknown(value),
        }
    }
}

impl StaticType for Winding {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_winding_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for Winding {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for Winding {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for Winding {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum WinsysFeature {
    MultipleOnscreen,
    SwapThrottle,
    VblankCounter,
    VblankWait,
    TextureFromPixmap,
    SwapBuffersEvent,
    SwapRegion,
    SwapRegionThrottle,
    SwapRegionSynchronized,
    BufferAge,
    SyncAndCompleteEvent,
    NFeatures,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for WinsysFeature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "WinsysFeature::{}",
            match *self {
                WinsysFeature::MultipleOnscreen => "MultipleOnscreen",
                WinsysFeature::SwapThrottle => "SwapThrottle",
                WinsysFeature::VblankCounter => "VblankCounter",
                WinsysFeature::VblankWait => "VblankWait",
                WinsysFeature::TextureFromPixmap => "TextureFromPixmap",
                WinsysFeature::SwapBuffersEvent => "SwapBuffersEvent",
                WinsysFeature::SwapRegion => "SwapRegion",
                WinsysFeature::SwapRegionThrottle => "SwapRegionThrottle",
                WinsysFeature::SwapRegionSynchronized => "SwapRegionSynchronized",
                WinsysFeature::BufferAge => "BufferAge",
                WinsysFeature::SyncAndCompleteEvent => "SyncAndCompleteEvent",
                WinsysFeature::NFeatures => "NFeatures",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for WinsysFeature {
    type GlibType = ffi::CoglWinsysFeature;

    fn to_glib(&self) -> ffi::CoglWinsysFeature {
        match *self {
            WinsysFeature::MultipleOnscreen => ffi::COGL_WINSYS_FEATURE_MULTIPLE_ONSCREEN,
            WinsysFeature::SwapThrottle => ffi::COGL_WINSYS_FEATURE_SWAP_THROTTLE,
            WinsysFeature::VblankCounter => ffi::COGL_WINSYS_FEATURE_VBLANK_COUNTER,
            WinsysFeature::VblankWait => ffi::COGL_WINSYS_FEATURE_VBLANK_WAIT,
            WinsysFeature::TextureFromPixmap => ffi::COGL_WINSYS_FEATURE_TEXTURE_FROM_PIXMAP,
            WinsysFeature::SwapBuffersEvent => ffi::COGL_WINSYS_FEATURE_SWAP_BUFFERS_EVENT,
            WinsysFeature::SwapRegion => ffi::COGL_WINSYS_FEATURE_SWAP_REGION,
            WinsysFeature::SwapRegionThrottle => ffi::COGL_WINSYS_FEATURE_SWAP_REGION_THROTTLE,
            WinsysFeature::SwapRegionSynchronized => {
                ffi::COGL_WINSYS_FEATURE_SWAP_REGION_SYNCHRONIZED
            }
            WinsysFeature::BufferAge => ffi::COGL_WINSYS_FEATURE_BUFFER_AGE,
            WinsysFeature::SyncAndCompleteEvent => ffi::COGL_WINSYS_FEATURE_SYNC_AND_COMPLETE_EVENT,
            WinsysFeature::NFeatures => ffi::COGL_WINSYS_FEATURE_N_FEATURES,
            WinsysFeature::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglWinsysFeature> for WinsysFeature {
    fn from_glib(value: ffi::CoglWinsysFeature) -> Self {
        match value {
            0 => WinsysFeature::MultipleOnscreen,
            1 => WinsysFeature::SwapThrottle,
            2 => WinsysFeature::VblankCounter,
            3 => WinsysFeature::VblankWait,
            4 => WinsysFeature::TextureFromPixmap,
            5 => WinsysFeature::SwapBuffersEvent,
            6 => WinsysFeature::SwapRegion,
            7 => WinsysFeature::SwapRegionThrottle,
            8 => WinsysFeature::SwapRegionSynchronized,
            9 => WinsysFeature::BufferAge,
            10 => WinsysFeature::SyncAndCompleteEvent,
            11 => WinsysFeature::NFeatures,
            value => WinsysFeature::__Unknown(value),
        }
    }
}

impl StaticType for WinsysFeature {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_winsys_feature_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for WinsysFeature {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for WinsysFeature {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for WinsysFeature {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Identifies specific window system backends that Cogl supports.
///
/// These can be used to query what backend Cogl is using or to try and
/// explicitly select a backend to use.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum WinsysID {
    /// Implies no preference for which backend is used
    Any,
    /// Use the no-op stub backend
    Stub,
    /// Use the GLX window system binding API
    Glx,
    /// Use EGL with the X window system via XLib
    EglXlib,
    /// Use EGL with the PowerVR NULL window system
    EglNull,
    /// Use EGL with the GDL platform
    EglGdl,
    /// Use EGL with the Wayland window system
    EglWayland,
    /// Use EGL with the KMS platform
    EglKms,
    /// Use EGL with the Android platform
    EglAndroid,
    /// Use EGL with the Mir server
    EglMir,
    /// Use the Microsoft Windows WGL binding API
    Wgl,
    /// Use the SDL window system
    Sdl,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for WinsysID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "WinsysID::{}",
            match *self {
                WinsysID::Any => "Any",
                WinsysID::Stub => "Stub",
                WinsysID::Glx => "Glx",
                WinsysID::EglXlib => "EglXlib",
                WinsysID::EglNull => "EglNull",
                WinsysID::EglGdl => "EglGdl",
                WinsysID::EglWayland => "EglWayland",
                WinsysID::EglKms => "EglKms",
                WinsysID::EglAndroid => "EglAndroid",
                WinsysID::EglMir => "EglMir",
                WinsysID::Wgl => "Wgl",
                WinsysID::Sdl => "Sdl",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for WinsysID {
    type GlibType = ffi::CoglWinsysID;

    fn to_glib(&self) -> ffi::CoglWinsysID {
        match *self {
            WinsysID::Any => ffi::COGL_WINSYS_ID_ANY,
            WinsysID::Stub => ffi::COGL_WINSYS_ID_STUB,
            WinsysID::Glx => ffi::COGL_WINSYS_ID_GLX,
            WinsysID::EglXlib => ffi::COGL_WINSYS_ID_EGL_XLIB,
            WinsysID::EglNull => ffi::COGL_WINSYS_ID_EGL_NULL,
            WinsysID::EglGdl => ffi::COGL_WINSYS_ID_EGL_GDL,
            WinsysID::EglWayland => ffi::COGL_WINSYS_ID_EGL_WAYLAND,
            WinsysID::EglKms => ffi::COGL_WINSYS_ID_EGL_KMS,
            WinsysID::EglAndroid => ffi::COGL_WINSYS_ID_EGL_ANDROID,
            WinsysID::EglMir => ffi::COGL_WINSYS_ID_EGL_MIR,
            WinsysID::Wgl => ffi::COGL_WINSYS_ID_WGL,
            WinsysID::Sdl => ffi::COGL_WINSYS_ID_SDL,
            WinsysID::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglWinsysID> for WinsysID {
    fn from_glib(value: ffi::CoglWinsysID) -> Self {
        match value {
            0 => WinsysID::Any,
            1 => WinsysID::Stub,
            2 => WinsysID::Glx,
            3 => WinsysID::EglXlib,
            4 => WinsysID::EglNull,
            5 => WinsysID::EglGdl,
            6 => WinsysID::EglWayland,
            7 => WinsysID::EglKms,
            8 => WinsysID::EglAndroid,
            9 => WinsysID::EglMir,
            10 => WinsysID::Wgl,
            11 => WinsysID::Sdl,
            value => WinsysID::__Unknown(value),
        }
    }
}
