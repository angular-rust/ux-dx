#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into,
    clippy::upper_case_acronyms
)]

use super::{Bitmap, Offscreen, Onscreen};
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
            }
        )
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
            }
        )
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
            }
        )
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
}

impl fmt::Display for BufferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BufferError::{}",
            match *self {
                BufferError::BufferErrorMap => "BufferErrorMap",
            }
        )
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
}

impl Default for BufferUpdateHint {
    fn default() -> Self {
        Self::Static
    }
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
            }
        )
    }
}

/// When using depth testing one of these functions is used to compare
/// the depth of an incoming fragment against the depth value currently
/// stored in the depth buffer. The fn is changed using
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
            }
        )
    }
}

/// Identifiers for underlying hardware drivers that may be used by
///  for rendering.
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
}

impl Default for Driver {
    fn default() -> Self {
        Self::Any
    }
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
            }
        )
    }
}

/// All the capabilities that can vary between different GPUs supported
/// by . Applications that depend on any of these features should explicitly
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
    ///  supported with BufferAccess including read support.
    OglFeatureIdMapBufferForRead,
    /// Whether `buffer_map` is
    ///  supported with BufferAccess including write support.
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
            }
        )
    }
}

/// Return values for the `XlibFilterFunc` and `Win32FilterFunc` functions.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum FilterReturn {
    /// The event was not handled, continues the
    ///  processing
    Continue,
    /// Remove the event, stops the processing
    Remove,
}

impl fmt::Display for FilterReturn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FilterReturn::{}",
            match *self {
                FilterReturn::Continue => "Continue",
                FilterReturn::Remove => "Remove",
            }
        )
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
            }
        )
    }
}

/// Identifiers that are passed to `FrameCallback` functions
/// (registered using `Onscreen::add_frame_callback`) that
/// mark the progression of a frame in some way which usually
/// means that new information will have been accumulated in the
/// frame's corresponding `FrameInfo` object::
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
}

impl fmt::Display for FrameEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FrameEvent::{}",
            match *self {
                FrameEvent::Sync => "Sync",
                FrameEvent::Complete => "Complete",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum FramebufferError {
    FramebufferErrorAllocate,
}

impl fmt::Display for FramebufferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FramebufferError::{}",
            match *self {
                FramebufferError::FramebufferErrorAllocate => "FramebufferErrorAllocate",
            }
        )
    }
}

// XXX: The order of these indices determines the order they are
// flushed.
//
// Flushing clip state may trash the modelview and projection matrices
// so we must do it before flushing the matrices.
//
pub enum FramebufferStateIndex {
    Bind = 0,
    ViewPort = 1,
    Clip = 2,
    Dither = 3,
    ModelView = 4,
    Projection = 5,
    ColorMask = 6,
    FrontFaceWinding = 7,
    DepthWrite = 8,
    StereoMode = 9,
    Max = 10,
}

pub enum FramebufferState {
    Bind = 1 << 0,
    ViewPort = 1 << 1,
    Clip = 1 << 2,
    Dither = 1 << 3,
    ModelView = 1 << 4,
    Projection = 1 << 5,
    ColorMask = 1 << 6,
    FrontFaceWinding = 1 << 7,
    DepthWrite = 1 << 8,
    StereoMode = 1 << 9,
}

#[derive(Debug)]
pub enum FramebufferType {
    OnScreen(Onscreen),
    OffScreen(Offscreen),
}

impl Default for FramebufferType {
    fn default() -> Self {
        Self::OnScreen(Default::default())
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
}

impl fmt::Display for GLES2ContextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "GLES2ContextError::{}",
            match *self {
                GLES2ContextError::Unsupported => "Unsupported",
                GLES2ContextError::Driver => "Driver",
            }
        )
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
            }
        )
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
            }
        )
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
            }
        )
    }
}

/// Available types of layers for a `Material`. This enumeration
/// might be expanded in later versions.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum MaterialLayerType {
    /// The layer represents a
    ///  <link linkend="Textures">texture`</link>`
    Texture,
}

impl fmt::Display for MaterialLayerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MaterialLayerType::{}",
            match *self {
                MaterialLayerType::Texture => "Texture",
            }
        )
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
    ///  will try to automatically
    ///  decide which of the above two to use. For `rectangle`, it
    ///  will use repeat mode if any of the texture coordinates are
    ///  outside the range 0→1, otherwise it will use clamp to edge. For
    ///  `polygon` it will always use repeat mode. For
    ///  `vertex_buffer_draw` it will use repeat mode except for
    ///  layers that have point sprite coordinate generation enabled. This
    ///  is the default value.
    Automatic,
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
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OffscreenAllocateFlags {
    None = 0,
    DepthStencil = 1 << 0,
    Depth = 1 << 1,
    Stencil = 1 << 2,
}

// Flags to pass to _offscreen_new_with_texture_full
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OffscreenFlags {
    None = 0,
    DisableDepthAndStencil = 1,
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
            }
        )
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
            }
        )
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
            }
        )
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
    ///  will try to automatically
    ///  decide which of the above two to use. For `rectangle`, it
    ///  will use repeat mode if any of the texture coordinates are
    ///  outside the range 0→1, otherwise it will use clamp to edge. For
    ///  `polygon` it will always use repeat mode. For
    ///  `vertex_buffer_draw` it will use repeat mode except for
    ///  layers that have point sprite coordinate generation enabled. This
    ///  is the default value.
    ///
    Automatic,
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
            }
        )
    }
}

const BIT_A: u32 = 1 << 4;
const BIT_BGR: u32 = 1 << 5;
const BIT_AFIRST: u32 = 1 << 6;
const BIT_PREMULT: u32 = 1 << 7;
const BIT_DEPTH: u32 = 1 << 8;
const BIT_STENCIL: u32 = 1 << 9;

/// Pixel formats used by . For the formats with a byte per
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
/// internal format.  will try to pick the best format to use
/// internally and convert the texture data if necessary.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[repr(u32)]
pub enum PixelFormat {
    /// Any format
    Any = 0,
    /// 8 bits alpha mask
    A8 = 1 | BIT_A,
    /// RGB, 16 bits
    Rgb565 = 4,
    /// RGBA, 16 bits
    Rgba4444 = 5 | BIT_A,
    /// RGBA, 16 bits
    Rgba5551 = 6 | BIT_A,
    /// Not currently supported
    Yuv = 7,
    /// Single luminance component
    G8 = 8,
    /// RG, 16 bits. Note that red-green textures
    ///  are only available if `FeatureID::OglFeatureIdTextureRg` is advertised.
    ///  See `Texture::set_components` for details.
    Rg88 = 9,
    /// RGB, 24 bits
    Rgb888 = 2,
    /// BGR, 24 bits
    Bgr888 = 2 | BIT_BGR,
    /// RGBA, 32 bits
    Rgba8888 = 3 | BIT_A,
    /// BGRA, 32 bits
    Bgra8888 = 3 | BIT_A | BIT_BGR,
    /// ARGB, 32 bits
    Argb8888 = 3 | BIT_A | BIT_AFIRST,
    /// ABGR, 32 bits
    Abgr8888 = 3 | BIT_A | BIT_BGR | BIT_AFIRST,
    /// RGBA, 32 bits, 10 bpc
    Rgba1010102 = 13 | BIT_A,
    /// BGRA, 32 bits, 10 bpc
    Bgra1010102 = 13 | BIT_A | BIT_BGR,
    /// ARGB, 32 bits, 10 bpc
    Argb2101010 = 13 | BIT_A | BIT_AFIRST,
    /// ABGR, 32 bits, 10 bpc
    Abgr2101010 = 13 | BIT_A | BIT_BGR | BIT_AFIRST,
    /// Premultiplied RGBA, 32 bits
    Rgba8888Pre = 3 | BIT_A | BIT_PREMULT,
    /// Premultiplied BGRA, 32 bits
    Bgra8888Pre = 3 | BIT_A | BIT_PREMULT | BIT_BGR,
    /// Premultiplied ARGB, 32 bits
    Argb8888Pre = 3 | BIT_A | BIT_PREMULT | BIT_AFIRST,
    /// Premultiplied ABGR, 32 bits
    Abgr8888Pre = 3 | BIT_A | BIT_PREMULT | BIT_BGR | BIT_AFIRST,
    /// Premultiplied RGBA, 16 bits
    Rgba4444Pre = (5 | BIT_A) | BIT_A | BIT_PREMULT,
    /// Premultiplied RGBA, 16 bits
    Rgba5551Pre = (6 | BIT_A) | BIT_A | BIT_PREMULT,
    /// Premultiplied RGBA, 32 bits, 10 bpc
    Rgba1010102Pre = (13 | BIT_A) | BIT_PREMULT,
    /// Premultiplied BGRA, 32 bits, 10 bpc
    Bgra1010102Pre = (13 | BIT_A | BIT_BGR) | BIT_PREMULT,
    /// Premultiplied ARGB, 32 bits, 10 bpc
    Argb2101010Pre = (13 | BIT_A | BIT_AFIRST) | BIT_PREMULT,
    /// Premultiplied ABGR, 32 bits, 10 bpc
    Abgr2101010Pre = (13 | BIT_A | BIT_BGR | BIT_AFIRST) | BIT_PREMULT,
    Depth16 = (9 | BIT_DEPTH),
    Depth32 = (3 | BIT_DEPTH),
    Depth24Stencil8 = (3 | BIT_DEPTH | BIT_STENCIL),
}

impl PixelFormat {
    // Returns the number of bytes-per-pixel of a given format. The bpp
    // can be extracted from the least significant nibble of the pixel
    // format (see PixelFormat).
    //
    // The mapping is the following (see discussion on bug #660188):
    //
    // 0     = undefined
    // 1, 8  = 1 bpp (e.g. A_8, G_8)
    // 2     = 3 bpp, aligned (e.g. 888)
    // 3     = 4 bpp, aligned (e.g. 8888)
    // 4-6   = 2 bpp, not aligned (e.g. 565, 4444, 5551)
    // 7     = undefined yuv
    // 9     = 2 bpp, aligned
    // 10     = undefined
    // 11     = undefined
    // 12    = 3 bpp, not aligned
    // 13    = 4 bpp, not aligned (e.g. 2101010)
    // 14-15 = undefined
    pub fn bytes_per_pixel(&self) -> u32 {
        let bpp: [u32; 16] = [0, 1, 3, 4, 2, 2, 2, 0, 1, 2, 0, 0, 3, 4, 0, 0];
        let idx = *self as usize & 0xf;

        bpp[idx]
    }
}
impl Default for PixelFormat {
    fn default() -> Self {
        Self::Rgba8888
    }
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
            }
        )
    }
}

/// A bitmask of events that  may need to wake on for a file
/// descriptor. Note that these all have the same values as the
/// corresponding defines for the poll fn call on Unix so they
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
            }
        )
    }
}

// Private flags that can internally be added to ReadPixelsFlags
pub enum PrivateReadPixelsFlags {
    // If this is set then the data will not be flipped to compensate
    // for GL's upside-down coordinate system but instead will be left
    // in whatever order GL gives us (which will depend on whether the
    // framebuffer is offscreen or not)
    NoFlip = 1 << 30,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum RendererError {
    XlibDisplayOpen,
    BadConstraint,
}

impl fmt::Display for RendererError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RendererError::{}",
            match *self {
                RendererError::XlibDisplayOpen => "XlibDisplayOpen",
                RendererError::BadConstraint => "BadConstraint",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum TextureLoader {
    Sized {
        width: u32,
        height: u32,
        depth: u32, // for 3d textures
    },
    Bitmap {
        bitmap: Bitmap,
        height: u32, // for 3d textures
        depth: u32,  // for 3d textures
        can_convert_in_place: bool,
    },
    #[cfg(feature = "egl")]
    EglImage {
        image: EGLImageKHR,
        width: u32,
        height: u32,
        format: PixelFormat,
    },
    GlForeign {
        width: u32,
        height: u32,
        format: PixelFormat,
        gl_handle: u32,
    },
}

impl Default for TextureLoader {
    fn default() -> Self {
        Self::Sized {
            width: 0,
            height: 0,
            depth: 0,
        }
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
}

impl fmt::Display for ShaderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ShaderType::{}",
            match *self {
                ShaderType::Vertex => "Vertex",
                ShaderType::Fragment => "Fragment",
            }
        )
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
/// `main` fn before any vertex processing is done.
/// `</para>`
/// `<para>`
/// The ‘replace’ string in `snippet` will be used instead of the
/// generated vertex processing if it is present. This can be used if
/// the application wants to provide a complete vertex shader and
/// doesn't need the generated output from .
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
/// `main` fn before the vertex transform is done.
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
/// `main` fn before any fragment processing is done.
/// `</para>`
/// `<para>`
/// The ‘replace’ string in `snippet` will be used instead of the
/// generated fragment processing if it is present. This can be used if
/// the application wants to provide a complete fragment shader and
/// doesn't need the generated output from .
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
/// `main` fn before any fragment processing is done. This is a
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
            }
        )
    }
}

/// Represents how draw should affect the two buffers
/// of a stereo framebuffer. See `Framebuffer::set_stereo_mode`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum StereoMode {
    None,
    /// draw to both stereo buffers
    Both,
    /// draw only to the left stereo buffer
    Left,
    /// draw only to the left stereo buffer
    Right,
}

impl Default for StereoMode {
    fn default() -> Self {
        Self::None
    }
}

impl fmt::Display for StereoMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "StereoMode::{}",
            match *self {
                StereoMode::None => "None",
                StereoMode::Both => "Both",
                StereoMode::Left => "Left",
                StereoMode::Right => "Right",
            }
        )
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
}

impl Default for SubpixelOrder {
    fn default() -> Self {
        Self::Unknown
    }
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
            }
        )
    }
}

/// Error enumeration for
///
/// The `SystemError::SystemErrorUnsupported` error can be thrown for a
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
/// Currently this is only used by  API marked as experimental so
/// this enum should also be considered experimental.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum SystemError {
    /// You tried to use a feature or
    ///  configuration not currently available.
    SystemErrorUnsupported,
    /// You tried to allocate a resource
    ///  such as a texture and there wasn't enough memory.
    SystemErrorNoMemory,
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SystemError::{}",
            match *self {
                SystemError::SystemErrorUnsupported => "SystemErrorUnsupported",
                SystemError::SystemErrorNoMemory => "SystemErrorNoMemory",
            }
        )
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
            }
        )
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
            }
        )
    }
}

/// Error codes that can be thrown when performing texture-pixmap-x11
/// operations.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum TexturePixmapX11Error {
    /// An X11 protocol error
    TexturePixmapX11ErrorX11,
}

impl fmt::Display for TexturePixmapX11Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TexturePixmapX11Error::{}",
            match *self {
                TexturePixmapX11Error::TexturePixmapX11ErrorX11 => "TexturePixmapX11ErrorX11",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum TexturePixmapX11ReportLevel {
    RawRectangles,
    DeltaRectangles,
    BoundingBox,
    NonEmpty,
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
            }
        )
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
            }
        )
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
            }
        )
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
}

impl fmt::Display for Winding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Winding::{}",
            match *self {
                Winding::Clockwise => "Clockwise",
                Winding::CounterClockwise => "CounterClockwise",
            }
        )
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
            }
        )
    }
}

/// Identifies specific window system backends that  supports.
///
/// These can be used to query what backend  is using or to try and
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
            }
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Usage {
    Dynamic,
    Static,
}

impl Default for Usage {
    fn default() -> Self {
        Self::Static
    }
}
